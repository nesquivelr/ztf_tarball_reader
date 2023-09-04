use serde::Deserialize;

#[derive(Deserialize)]
pub struct Candidate {
    pub jd: f64,
    pub fid: i32,
    pub pid: i64,
    pub diffmaglim: Option<f32>,
    pub pdiffimfilename: Option<String>,
    pub programpi: Option<String>,
    pub programid: i32,
    pub candid: i64,
    pub isdiffpos: String,
    pub tblid: Option<i64>,
    pub nid: Option<i32>,
    pub rcid: Option<i32>,
    pub field: Option<i32>,
    pub xpos: Option<f32>,
    pub ypos: Option<f32>,
    pub ra: f64,
    pub dec: f64,
    pub magpsf: f32,
    pub sigmapsf: f32,
    pub chipsf: Option<f32>,
    pub magap: Option<f32>,
    pub sigmagap: Option<f32>,
    pub distnr: Option<f32>,
    pub magnr: Option<f32>,
    pub sigmagnr: Option<f32>,
    pub chinr: Option<f32>,
    pub sharpnr: Option<f32>,
    pub sky: Option<f32>,
    pub magdiff: Option<f32>,
    pub fwhm: Option<f32>,
    pub classtar: Option<f32>,
    pub mindtoedge: Option<f32>,
    pub magfromlim: Option<f32>,
    pub seeratio: Option<f32>,
    pub aimage: Option<f32>,
    pub bimage: Option<f32>,
    pub aimagerat: Option<f32>,
    pub bimagerat: Option<f32>,
    pub elong: Option<f32>,
    pub nneg: Option<i32>,
    pub nbad: Option<i32>,
    pub rb: Option<f32>,
    pub ssdistnr: Option<f32>,
    pub ssmagnr: Option<f32>,
    pub ssnamenr: Option<String>,
    pub sumrat: Option<f32>,
    pub magapbig: Option<f32>,
    pub sigmagapbig: Option<f32>,
    pub ranr: f64,
    pub decnr: f64,
    pub sgmag1: Option<f32>,
    pub srmag1: Option<f32>,
    pub simag1: Option<f32>,
    pub szmag1: Option<f32>,
    pub sgscore1: Option<f32>,
    pub distpsnr1: Option<f32>,
    pub ndethist: i32,
    pub ncovhist: i32,
    pub jdstarthist: Option<f64>,
    pub jdendhist: Option<f64>,
    pub scorr: Option<f64>,
    pub tooflag: Option<i32>,
    pub objectidps1: Option<i64>,
    pub objectidps2: Option<i64>,
    pub sgmag2: Option<f32>,
    pub srmag2: Option<f32>,
    pub simag2: Option<f32>,
    pub szmag2: Option<f32>,
    pub sgscore2: Option<f32>,
    pub distpsnr2: Option<f32>,
    pub objectidps3: Option<i64>,
    pub sgmag3: Option<f32>,
    pub srmag3: Option<f32>,
    pub simag3: Option<f32>,
    pub szmag3: Option<f32>,
    pub sgscore3: Option<f32>,
    pub distpsnr3: Option<f32>,
    pub nmtchps: i32,
    pub rfid: i64,
    pub jdstartref: f64,
    pub jdendref: f64,
    pub nframesref: i32,
    pub rbversion: String,
    pub dsnrms: Option<f32>,
    pub ssnrms: Option<f32>,
    pub dsdiff: Option<f32>,
    pub magzpsci: Option<f32>,
    pub magzpsciunc: Option<f32>,
    pub magzpscirms: Option<f32>,
    pub nmatches: i32,
    pub clrcoeff: Option<f32>,
    pub clrcounc: Option<f32>,
    pub zpclrcov: Option<f32>,
    pub zpmed: Option<f32>,
    pub clrmed: Option<f32>,
    pub clrrms: Option<f32>,
    pub neargaia: Option<f32>,
    pub neargaiabright: Option<f32>,
    pub maggaia: Option<f32>,
    pub maggaiabright: Option<f32>,
    pub exptime: Option<f32>,
    pub drb: Option<f32>,
    pub drbversion: String,
}

#[derive(Deserialize)]
pub struct PrvCandidate {
    pub jd: f64,
    pub fid: i32,
    pub pid: i64,
    pub diffmaglim: Option<f32>,
    pub pdiffimfilename: Option<String>,
    pub programpi: Option<String>,
    pub programid: i32,
    pub candid: Option<i64>,
    pub isdiffpos: Option<String>,
    pub tblid: Option<i64>,
    pub nid: Option<i32>,
    pub rcid: Option<i32>,
    pub field: Option<i32>,
    pub xpos: Option<f32>,
    pub ypos: Option<f32>,
    pub ra: Option<f64>,
    pub dec: Option<f64>,
    pub magpsf: Option<f32>,
    pub sigmapsf: Option<f32>,
    pub chipsf: Option<f32>,
    pub magap: Option<f32>,
    pub sigmagap: Option<f32>,
    pub distnr: Option<f32>,
    pub magnr: Option<f32>,
    pub sigmagnr: Option<f32>,
    pub chinr: Option<f32>,
    pub sharpnr: Option<f32>,
    pub sky: Option<f32>,
    pub magdiff: Option<f32>,
    pub fwhm: Option<f32>,
    pub classtar: Option<f32>,
    pub mindtoedge: Option<f32>,
    pub magfromlim: Option<f32>,
    pub seeratio: Option<f32>,
    pub aimage: Option<f32>,
    pub bimage: Option<f32>,
    pub aimagerat: Option<f32>,
    pub bimagerat: Option<f32>,
    pub elong: Option<f32>,
    pub nneg: Option<i32>,
    pub nbad: Option<i32>,
    pub rb: Option<f32>,
    pub ssdistnr: Option<f32>,
    pub ssmagnr: Option<f32>,
    pub ssnamenr: Option<String>,
    pub sumrat: Option<f32>,
    pub magapbig: Option<f32>,
    pub sigmagapbig: Option<f32>,
    pub ranr: Option<f64>,
    pub decnr: Option<f64>,
    pub scorr: Option<f64>,
    pub magzpsci: Option<f32>,
    pub magzpsciunc: Option<f32>,
    pub magzpscirms: Option<f32>,
    pub clrcoeff: Option<f32>,
    pub clrcounc: Option<f32>,
    pub rbversion: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct CutOut {
    pub fileName: String,
    #[serde(with = "serde_bytes")]
    pub stampData: Vec<u8>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Alert {
    pub schemavsn: String,
    pub publisher: String,
    pub objectId: String,
    pub candid: i64,
    pub candidate: Candidate,
    pub prv_candidates: Vec<PrvCandidate>,
    pub cutoutScience: Option<CutOut>,
    pub cutoutTemplate: Option<CutOut>,
    pub cutoutDifference: Option<CutOut>,
}