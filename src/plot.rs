//! Plot functions.
//!
//! Taken from <https://github.com/SolarLiner/valib/src/util.rs>.

#![allow(unused)]

use std::{ops::Range, path::Path};

use plotters::coord::{self, ranged1d::ValueFormatter};
use plotters::{chart::SeriesAnno, prelude::*};

const PLOT_SIZE: (u32, u32) = (600, 400);

fn assert_ok(res: Result<(), impl std::fmt::Display>) {
    match res {
        Ok(()) => {}
        Err(value) => panic!("Not OK: {value}"),
    }
}

pub struct Series<'a> {
    pub label: &'a str,
    pub samplerate: f32,
    pub series: &'a [f32],
    pub color: &'a RGBColor,
}

impl<'a> Series<'a> {
    pub fn validate(&self) -> Result<(), String> {
        if self.samplerate <= 0. {
            return Err(format!("Series: {:?}: Samplerate is negative", self.label));
        }
        if self.series.is_empty() {
            return Err(format!("Series: {:?}: No data", self.label));
        }

        Ok(())
    }

    pub fn timescale(&self, bode: bool) -> Range<f32> {
        assert_ok(self.validate());
        if bode {
            0.0..self.samplerate / 2.0
        } else {
            let tmax = self.series.len() as f32 / self.samplerate;
            0.0..tmax
        }
    }

    pub fn y_range(&self) -> Range<f32> {
        assert_ok(self.validate());
        let min = self
            .series
            .iter()
            .copied()
            .min_by(f32::total_cmp)
            .unwrap()
            .clamp(f32::MIN, f32::MAX);
        let max = self
            .series
            .iter()
            .copied()
            .max_by(f32::total_cmp)
            .unwrap()
            .clamp(f32::MIN, f32::MAX);
        min..max
    }

    fn as_series<DB: DrawingBackend>(&self, bode: bool) -> LineSeries<DB, (f32, f32)> {
        LineSeries::new(
            self.series.iter().copied().enumerate().map(|(i, y)| {
                let x = if bode {
                    i as f32
                } else {
                    i as f32 / self.samplerate
                };
                (x, y)
            }),
            self.color,
        )
    }

    fn apply_legend(&self, ann: &mut SeriesAnno<impl DrawingBackend>) {
        let color = *self.color;
        ann.label(self.label);
        ann.legend(move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color));
    }
}

pub enum AxisRange {
    AutoLin,
    AutoLog,
    ManualLin(Range<f32>),
    ManualLog(Range<f32>),
}

pub struct Plot<'a> {
    pub title: &'a str,
    pub bode: bool,
    pub series: &'a [Series<'a>],
    pub y_range: AxisRange,
}

impl<'a> Plot<'a> {
    pub fn validate(&self) -> Result<(), String> {
        if self.series.is_empty() {
            return Err(format!("Plot {:?}: no series", self.title));
        }
        self.series.iter().try_for_each(|s| s.validate())?;
        Ok(())
    }

    pub fn render_into(&self, output: &DrawingArea<impl DrawingBackend, coord::Shift>) {
        use plotters::prelude::*;
        assert_ok(self.validate());

        let timescale = self
            .series
            .iter()
            .map(|s| s.timescale(self.bode))
            .reduce(|l, r| {
                let start = l.start.min(r.start);
                let end = l.end.max(r.end);
                start..end
            })
            .unwrap();
        let timescale = if self.bode {
            timescale.start * 2.0..timescale.end * 2.0
        } else {
            timescale
        };

        let yrange = match &self.y_range {
            AxisRange::AutoLin | AxisRange::AutoLog => self
                .series
                .iter()
                .map(|s| s.y_range())
                .reduce(|l, r| {
                    let start = l.start.min(r.start);
                    let end = l.end.max(r.end);
                    start..end
                })
                .unwrap(),
            AxisRange::ManualLin(range) | AxisRange::ManualLog(range) => range.to_owned(),
        };

        let mut ctx = ChartBuilder::on(output);
        ctx.set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(self.title, ("sans-serif", 40));
        if self.bode {
            match &self.y_range {
                AxisRange::AutoLin | AxisRange::ManualLin(_) => {
                    let ctx = ctx
                        .build_cartesian_2d(timescale.log_scale(), yrange)
                        .unwrap();
                    self.render(ctx);
                }
                AxisRange::AutoLog | AxisRange::ManualLog(_) => {
                    let ctx = ctx
                        .build_cartesian_2d(timescale.log_scale(), yrange.log_scale())
                        .unwrap();
                    self.render(ctx);
                }
            }
        } else {
            let ctx = ctx.build_cartesian_2d(timescale, yrange).unwrap();
            self.render(ctx);
        }
    }

    fn render<
        'ctx,
        T1: 'ctx + Ranged<ValueType = f32> + ValueFormatter<f32>,
        T2: 'ctx + Ranged<ValueType = f32> + ValueFormatter<f32>,
    >(
        &self,
        mut ctx: ChartContext<'ctx, impl 'ctx + DrawingBackend, Cartesian2d<T1, T2>>,
    ) {
        ctx.configure_mesh().draw().unwrap();

        for series in self.series {
            let ann = ctx.draw_series(series.as_series(self.bode)).unwrap();
            series.apply_legend(ann);
        }

        ctx.configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .draw()
            .unwrap();
    }

    pub fn create_svg(&self, filename: impl AsRef<Path>) {
        let path = filename.as_ref();
        let _ = std::fs::create_dir_all(path.parent().expect("Filename is empty"));
        let root = SVGBackend::new(path, PLOT_SIZE).into_drawing_area();
        root.fill(&WHITE).unwrap();
        self.render_into(&root);
    }
}
