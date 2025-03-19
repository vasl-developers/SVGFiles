//
// Execution control defines.
//
pub const INCLUDE_NAME: bool =		false;
pub const INCLUDE_FONTS: bool =		false;
pub const EMBED_FONTS: bool =		false;
pub const LINK_FONTS: bool =		false;
pub const CREATE_BEVEL: bool =		true;
pub const INCLUDE_IMAGES: bool =	true;
pub const CREATE_WRECKS: bool =		false;
pub const CREATE_MALF_SIDE: bool =	true;

pub const GUN_COLUMN_X_POSITION: f64 =	 		 3.0;
pub const GUN_COLUMN_Y_POSITION: f64 =			57.0;
pub const GUN_CALIBER_BASELINE: f64 =			60.0; // Adjusted to allow for descenders like '[', ')', etc.
pub const GUN_COLUMN_BREAKDOWN_HEIGHT: f64 =	 9.6;
pub const GUN_COLUMN_BREAKDOWN_WIDTH: f64 =		18.0;
pub const ALT_BREAKDOWN_X_POSITION: f64 =		42.0;
pub const IFE_HEIGHT: f64 =		 				 8.4;
pub const RANGE_LINE_Y_POSITION: f64 =			57.0;
//
// For armed vehicles the T# is displayed on the right side of the counter.
// For unarmed vehicles the T# is displayed with the PP# where the MA would normally be.
//
pub const MGS_LINE_X_POSITION: f64 =		21.0;
pub const MGS_LINE_Y_POSITION: f64 =		57.0;
pub const MGS_LINE_2_Y_POSITION: f64 =		48.0;
pub const MGS_WIDTH: f64 =					36.0;
pub const TOWING_X_POSITION: f64 =			33.0;
pub const TOWING_Y_POSITION: f64 =			48.0;
pub const TOWING_PP_X_POSITION: f64 =		57.0;
pub const BEFORE_TOWING_X_POSITION: f64 =	49.4;
