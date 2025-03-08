use std::path::PathBuf;

use opencv::{
    core::{Scalar, Size, Vector, CV_8UC1},
    imgproc::{cvt_color, rectangle, COLOR_RGB2GRAY, LINE_8},
    objdetect::{self, CascadeClassifier},
    prelude::*,
    videoio::{self, VideoCapture, VideoCaptureTrait},
    Result,
};

pub struct Camera {
    pub cam: VideoCapture,
    pub frame: Mat,
    pub xml: String,
}

impl Camera {
    pub fn new(index: i32, xml: &PathBuf) -> Result<Self> {
        let cam = videoio::VideoCapture::new(index, videoio::CAP_ANY)?;
        let frame = Mat::default();
        let xml = String::from(xml.to_str().unwrap());
        let result = Camera { cam, frame, xml };
        Ok(result)
    }

    pub(crate) fn read(&mut self) -> Result<()> {
        self.cam.read(&mut self.frame)?;
        Ok(())
    }

    pub fn face_detect(&mut self) -> Result<()> {
        self.read()?;
        let image = &mut self.frame;
        let mut classifier = CascadeClassifier::new(&self.xml)?;
        let mut gray = Mat::new_rows_cols_with_default(
            image.rows(),
            image.cols(),
            CV_8UC1,
            Scalar::default(),
        )?;

        cvt_color(
            image,
            &mut gray,
            COLOR_RGB2GRAY,
            0,
            opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;

        let mut faces = Vector::new();
        classifier.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            Size::new(30, 30),
            Size::default(),
        )?;

        for rec in faces {
            rectangle(image, rec, Scalar::new(0., 255., 0., 255.), 2, LINE_8, 0)?;
        }

        Ok(())
    }
}
