use super::{generate_list, GraphMaker};
use std::fmt::Write;

/// Generates a Legend
///
/// [See Matplotlib's documentation](https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.legend.html)
///
/// # Example
///
/// ```
/// use plotpy::{linspace, Curve, Legend, Plot, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // generate (x,y) points
///     let x  = linspace(0.0, 5.0, 6);
///     let y1: Vec<_> = x.iter().map(|v| 0.5 * *v).collect();
///     let y2: Vec<_> = x.iter().map(|v| 1.0 * *v).collect();
///     let y3: Vec<_> = x.iter().map(|v| 1.5 * *v).collect();
///     let y4: Vec<_> = x.iter().map(|v| 2.0 * *v).collect();
///
///     // configure and draw curves
///     let mut curve1 = Curve::new();
///     let mut curve2 = Curve::new();
///     let mut curve3 = Curve::new();
///     let mut curve4 = Curve::new();
///     curve1.set_label("y = 0.5 x");
///     curve2.set_label("y = 1.0 x");
///     curve3.set_label("y = 1.5 x");
///     curve4.set_label("y = 2.0 x");
///     curve1.draw(&x, &y1);
///     curve2.draw(&x, &y2);
///     curve3.draw(&x, &y3);
///     curve4.draw(&x, &y4);
///
///     // configure and draw legends
///     let mut legend1 = Legend::new();
///     legend1.set_fontsize(14.0)
///         .set_handle_len(6.0)
///         .set_num_col(2)
///         .set_outside(true)
///         .set_show_frame(false);
///     legend1.draw();
///     let mut legend2 = Legend::new();
///     legend2.draw();
///
///     // add curves and legends to subplots
///     let mut plot = Plot::new();
///     plot.set_subplot(2, 1, 1)
///         .add(&curve1)
///         .add(&curve2)
///         .add(&curve3)
///         .add(&curve4)
///         .add(&legend1);
///     plot.set_subplot(2, 1, 2)
///         .add(&curve1)
///         .add(&curve2)
///         .add(&curve3)
///         .add(&curve4)
///         .add(&legend2);
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_legend.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_legend.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_legend.svg)
///
/// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
pub struct Legend {
    fontsize: f64,      // Fontsize
    handle_len: f64,    // Length of legend's indicator line
    num_col: usize,     // Number of columns
    location: String,   // Location, e.g., "best", "right", "center left"
    outside: bool,      // Put legend outside plot area
    show_frame: bool,   // Show frame around legend
    x_coords: Vec<f64>, // Normalized coordinates to put legend outside
    extra: String,      // Extra commands (comma separated)
    buffer: String,     // buffer
}

impl Legend {
    /// Creates a new Legend object
    pub fn new() -> Self {
        Legend {
            fontsize: 0.0,
            handle_len: 3.0,
            num_col: 1,
            location: "best".to_string(),
            outside: false,
            show_frame: true,
            x_coords: vec![0.0, 1.02, 1.0, 0.102],
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws legend
    pub fn draw(&mut self) {
        let opt = self.options();
        if self.outside {
            generate_list(&mut self.buffer, "coo", self.x_coords.as_slice());
        }
        write!(&mut self.buffer, "h,l=plt.gca().get_legend_handles_labels()\n").unwrap();
        write!(&mut self.buffer, "if len(h)>0 and len(l)>0:\n").unwrap();
        write!(&mut self.buffer, "    leg=plt.legend({})\n", &opt).unwrap();
        write!(&mut self.buffer, "    add_to_ea(leg)\n").unwrap();
        if !self.show_frame {
            write!(&mut self.buffer, "    leg.get_frame().set_linewidth(0.0)\n").unwrap();
        }
    }

    /// Draws legend (with support for twin y-axis and merged handles/labels)
    pub fn draw_twin(&mut self) {
        let opt = self.options();

        // 如果需要将图例放到外部区域
        if self.outside {
            generate_list(&mut self.buffer, "coo", self.x_coords.as_slice());
        }

        // （注意：你的画图代码需要在这两行之后写入，这里假设你已经在别处生成了 ax.hist / ax_twinx.hist）

        // 3. 从两个轴分别获取 legend handles 和 labels
        write!(&mut self.buffer, "h1, l1 = ax.get_legend_handles_labels()\n").unwrap();
        write!(&mut self.buffer, "h2, l2 = ax_twinx.get_legend_handles_labels()\n").unwrap();

        // 4. 合并两个列表
        write!(&mut self.buffer, "handles = h1 + h2\nlabels = l1 + l2\n").unwrap();

        // 5. 只在有图例时才画
        write!(
            &mut self.buffer,
            "if handles and labels:\n    leg = ax.legend(handles, labels, {})\n",
            &opt
        )
        .unwrap();
        write!(&mut self.buffer, "    add_to_ea(leg)\n").unwrap();

        // 6. 如果不显示边框，则将边框线宽设为 0
        if !self.show_frame {
            write!(&mut self.buffer, "    leg.get_frame().set_linewidth(0.0)\n").unwrap();
        }
    }

    /// Sets the fontsize
    pub fn set_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.fontsize = fontsize;
        self
    }

    /// Sets the length of legend's indicator line
    pub fn set_handle_len(&mut self, length: f64) -> &mut Self {
        self.handle_len = length;
        self
    }

    /// Sets the number of columns
    pub fn set_num_col(&mut self, num_columns: usize) -> &mut Self {
        self.num_col = num_columns;
        self
    }

    /// Sets the location
    ///
    /// Options:
    ///
    /// * "best", "right", "center left"
    /// * Note: Only used if outside == false
    pub fn set_location(&mut self, location: &str) -> &mut Self {
        self.location = String::from(location);
        self
    }

    /// Sets option to put legend outside of plot area
    pub fn set_outside(&mut self, flag: bool) -> &mut Self {
        self.outside = flag;
        self
    }

    /// Sets option to show frame around legend
    pub fn set_show_frame(&mut self, flag: bool) -> &mut Self {
        self.show_frame = flag;
        self
    }

    /// Sets the normalized coordinates when drawing an outside legend
    ///
    /// Example: `[0.0, 1.02, 1.0, 0.102]`
    pub fn set_x_coords(&mut self, coords: &[f64]) -> &mut Self {
        self.x_coords = coords.to_vec();
        self
    }

    /// Sets extra matplotlib commands (comma separated)
    ///
    /// **Important:** The extra commands must be comma separated. For example:
    ///
    /// ```text
    /// param1=123,param2='hello'
    /// ```
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.legend.html)
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns options for legend
    fn options(&self) -> String {
        let mut opt = String::new();
        let mut comma = "";
        if self.handle_len > 0.0 {
            write!(&mut opt, "handlelength={}", self.handle_len).unwrap();
            comma = ",";
        }
        if self.fontsize > 0.0 {
            write!(&mut opt, "{}prop={{'size':{}}}", comma, self.fontsize).unwrap();
            comma = ",";
        }
        if self.num_col > 0 {
            write!(&mut opt, "{}ncol={}", comma, self.num_col).unwrap();
            comma = ",";
        }
        if self.outside {
            write!(
                &mut opt,
                "{}loc=3,bbox_to_anchor=coo,mode='expand',borderaxespad=0.0,columnspacing=1,handletextpad=0.05",
                comma
            )
            .unwrap();
        } else {
            if self.location != "" {
                write!(&mut opt, "{}loc='{}'", comma, self.location).unwrap();
            }
        }
        if self.extra != "" {
            write!(&mut opt, ",{}", self.extra).unwrap();
        }
        opt
    }
}

impl GraphMaker for Legend {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Legend;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let legend = Legend::new();
        assert_eq!(legend.fontsize, 0.0);
        assert_eq!(legend.handle_len, 3.0);
        assert_eq!(legend.num_col, 1);
        assert_eq!(legend.location, "best".to_string());
        assert_eq!(legend.outside, false);
        assert_eq!(legend.show_frame, true);
        assert_eq!(legend.x_coords, vec![0.0, 1.02, 1.0, 0.102]);
        assert_eq!(legend.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut legend = Legend::new();
        legend.set_handle_len(6.0);
        let opt = legend.options();
        assert_eq!(opt, "handlelength=6,ncol=1,loc='best'");
    }

    #[test]
    fn draw_works() {
        let mut legend = Legend::new();
        legend.draw();
        let b: &str = "h,l=plt.gca().get_legend_handles_labels()\n\
                       if len(h)>0 and len(l)>0:\n\
                       \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                       \x20\x20\x20\x20add_to_ea(leg)\n";
        assert_eq!(legend.buffer, b);
        legend.clear_buffer();
        assert_eq!(legend.buffer, "");
    }
}
