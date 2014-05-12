use libc::*;

// from toplevel.h

pub static STAY_ON_TOP: c_int = 0x8000;
pub static ICONIZE: c_int = 0x4000;
pub static MINIMIZE: c_int = ICONIZE;
pub static MAXIMIZE: c_int = 0x2000;
pub static CLOSE_BOX: c_int = 0x1000;  // == HELP so can't be used with it

pub static SYSTEM_MENU: c_int = 0x0800;
pub static MINIMIZE_BOX: c_int = 0x0400;
pub static MAXIMIZE_BOX: c_int = 0x0200;

pub static TINY_CAPTION: c_int = 0x0080;  // clashes with NO_DEFAULT
pub static RESIZE_BORDER: c_int = 0x0040;  // == CLOSE

pub static DEFAULT_FRAME_STYLE: c_int =
        (SYSTEM_MENU |
         RESIZE_BORDER |
         MINIMIZE_BOX |
         MAXIMIZE_BOX |
         CLOSE_BOX |
         CAPTION |
         CLIP_CHILDREN);

// manually converted from defs.h

pub static VSCROLL: c_int = 0x80000000;
pub static HSCROLL: c_int = 0x40000000;
pub static CAPTION: c_int = 0x20000000;

pub static DOUBLE_BORDER: c_int = BORDER_DOUBLE;
pub static SUNKEN_BORDER: c_int = BORDER_SUNKEN;
pub static RAISED_BORDER: c_int = BORDER_RAISED;
pub static BORDER: c_int = BORDER_SIMPLE;
pub static SIMPLE_BORDER: c_int = BORDER_SIMPLE;
pub static STATIC_BORDER: c_int = BORDER_STATIC;
pub static NO_BORDER: c_int = BORDER_NONE;

pub static ALWAYS_SHOW_SB: c_int = 0x00800000;

pub static CLIP_CHILDREN: c_int = 0x00400000;
pub static CLIP_SIBLINGS: c_int = 0x20000000;

pub static TRANSPARENT_WINDOW: c_int = 0x00100000;

pub static TAB_TRAVERSAL: c_int = 0x00080000;

pub static WANTS_CHARS: c_int = 0x00040000;

pub static RETAINED: c_int = 0x00000000;
pub static BACKINGSTORE: c_int = RETAINED;

pub static POPUP_WINDOW: c_int = 0x00020000;

pub static FULL_REPAINT_ON_RESIZE: c_int = 0x00010000;
pub static NO_FULL_REPAINT_ON_RESIZE: c_int = 0;

pub static WINDOW_STYLE_MASK: c_int =
    (VSCROLL|HSCROLL|BORDER_MASK|ALWAYS_SHOW_SB|CLIP_CHILDREN|
     CLIP_SIBLINGS|TRANSPARENT_WINDOW|TAB_TRAVERSAL|WANTS_CHARS|
     RETAINED|POPUP_WINDOW|FULL_REPAINT_ON_RESIZE);

pub static WS_EX_VALIDATE_RECURSIVELY: c_int = 0x00000001;
pub static WS_EX_BLOCK_EVENTS: c_int = 0x00000002;
pub static WS_EX_TRANSIENT: c_int = 0x00000004;
pub static WS_EX_THEMED_BACKGROUND: c_int = 0x00000008;
pub static WS_EX_PROCESS_IDLE: c_int = 0x00000010;
pub static WS_EX_PROCESS_UI_UPDATES: c_int = 0x00000020;

pub static FRAME_EX_METAL: c_int = 0x00000040;
pub static DIALOG_EX_METAL: c_int = 0x00000040;

pub static WS_EX_CONTEXTHELP: c_int = 0x00000080;

pub static FRAME_EX_CONTEXTHELP: c_int = WS_EX_CONTEXTHELP;
pub static DIALOG_EX_CONTEXTHELP: c_int = WS_EX_CONTEXTHELP;

pub static FRAME_DRAWER: c_int = 0x0020;

pub static FRAME_NO_WINDOW_MENU: c_int = 0x0100;

pub static MB_DOCKABLE: c_int = 0x0001;

pub static MENU_TEAROFF: c_int = 0x0001;

pub static COLOURED: c_int = 0x0800;
pub static FIXED_LENGTH: c_int = 0x0400;

pub static LB_SORT: c_int = 0x0010;
pub static LB_SINGLE: c_int = 0x0020;
pub static LB_MULTIPLE: c_int = 0x0040;
pub static LB_EXTENDED: c_int = 0x0080;
pub static LB_NEEDED_SB: c_int = 0x0000;
pub static LB_OWNERDRAW: c_int = 0x0100;
pub static LB_ALWAYS_SB: c_int = 0x0200;
pub static LB_NO_SB: c_int = 0x0400;
pub static LB_HSCROLL: c_int = HSCROLL;
pub static LB_INT_HEIGHT: c_int = 0x0800;

pub static CB_SIMPLE: c_int = 0x0004;
pub static CB_SORT: c_int = 0x0008;
pub static CB_READONLY: c_int = 0x0010;
pub static CB_DROPDOWN: c_int = 0x0020;

pub static RA_LEFTTORIGHT: c_int = 0x0001;
pub static RA_TOPTOBOTTOM: c_int = 0x0002;
pub static RA_SPECIFY_COLS: c_int = HORIZONTAL;
pub static RA_SPECIFY_ROWS: c_int = VERTICAL;

pub static RA_HORIZONTAL: c_int = HORIZONTAL;
pub static RA_VERTICAL: c_int = VERTICAL;

pub static RB_GROUP: c_int = 0x0004;
pub static RB_SINGLE: c_int = 0x0008;

pub static SB_HORIZONTAL: c_int = HORIZONTAL;
pub static SB_VERTICAL: c_int = VERTICAL;

pub static SP_HORIZONTAL: c_int = HORIZONTAL;
pub static SP_VERTICAL: c_int = VERTICAL;
pub static SP_ARROW_KEYS: c_int = 0x4000;
pub static SP_WRAP: c_int = 0x8000;

pub static TC_RIGHTJUSTIFY: c_int = 0x0010;
pub static TC_FIXEDWIDTH: c_int = 0x0020;
pub static TC_TOP: c_int = 0x0000;
pub static TC_LEFT: c_int = 0x0020;
pub static TC_RIGHT: c_int = 0x0040;
pub static TC_BOTTOM: c_int = 0x0080;
pub static TC_MULTILINE: c_int = 0x0200;
pub static TC_OWNERDRAW: c_int = 0x0400;

pub static BI_EXPAND: c_int = EXPAND;

pub static LI_HORIZONTAL: c_int = HORIZONTAL;
pub static LI_VERTICAL: c_int = VERTICAL;

pub static YES: c_int = 0x00000002;
pub static OK: c_int = 0x00000004;
pub static NO: c_int = 0x00000008;
pub static YES_NO: c_int = (YES | NO);
pub static CANCEL: c_int = 0x00000010;
pub static APPLY: c_int = 0x00000020;
pub static CLOSE: c_int = 0x00000040;

pub static OK_DEFAULT: c_int = 0x00000000;
pub static YES_DEFAULT: c_int = 0x00000000;
pub static NO_DEFAULT: c_int = 0x00000080;
pub static CANCEL_DEFAULT: c_int = 0x80000000;

pub static ICON_EXCLAMATION: c_int = 0x00000100;
pub static ICON_HAND: c_int = 0x00000200;
pub static ICON_WARNING: c_int = ICON_EXCLAMATION;
pub static ICON_ERROR: c_int = ICON_HAND;
pub static ICON_QUESTION: c_int = 0x00000400;
pub static ICON_INFORMATION: c_int = 0x00000800;
pub static ICON_STOP: c_int = ICON_HAND;
pub static ICON_ASTERISK: c_int = ICON_INFORMATION;

pub static HELP: c_int = 0x00001000;
pub static FORWARD: c_int = 0x00002000;
pub static BACKWARD: c_int = 0x00004000;
pub static RESET: c_int = 0x00008000;
pub static MORE: c_int = 0x00010000;
pub static SETUP: c_int = 0x00020000;
pub static ICON_NONE: c_int = 0x00040000;

pub static ICON_MASK: c_int =
    (ICON_EXCLAMATION|ICON_HAND|ICON_QUESTION|ICON_INFORMATION|ICON_NONE);


// generated from /defs.h
pub static P_ALL: c_int = 0;
pub static P_PID: c_int = 1;
pub static P_PGID: c_int = 2;
pub static DefaultCoord: c_int = -1;
pub static CENTRE: c_int = 1;
pub static CENTER: c_int = 1;
pub static HORIZONTAL: c_int = 4;
pub static VERTICAL: c_int = 8;
pub static BOTH: c_int = 12;
pub static ORIENTATION_MASK: c_int = 12;
pub static LEFT: c_int = 16;
pub static RIGHT: c_int = 32;
pub static UP: c_int = 64;
pub static DOWN: c_int = 128;
pub static TOP: c_int = 64;
pub static BOTTOM: c_int = 128;
pub static NORTH: c_int = 64;
pub static SOUTH: c_int = 128;
pub static WEST: c_int = 16;
pub static EAST: c_int = 32;
pub static ALL: c_int = 240;
pub static DIRECTION_MASK: c_int = 240;
pub static ALIGN_INVALID: c_int = -1;
pub static ALIGN_NOT: c_int = 0;
pub static ALIGN_CENTER_HORIZONTAL: c_int = 256;
pub static ALIGN_CENTRE_HORIZONTAL: c_int = 256;
pub static ALIGN_LEFT: c_int = 0;
pub static ALIGN_TOP: c_int = 0;
pub static ALIGN_RIGHT: c_int = 512;
pub static ALIGN_BOTTOM: c_int = 1024;
pub static ALIGN_CENTER_VERTICAL: c_int = 2048;
pub static ALIGN_CENTRE_VERTICAL: c_int = 2048;
pub static ALIGN_CENTER: c_int = 2304;
pub static ALIGN_CENTRE: c_int = 2304;
pub static ALIGN_MASK: c_int = 3840;
pub static ADJUST_MINSIZE: c_int = 0;
pub static FIXED_MINSIZE: c_int = 32768;
pub static RESERVE_SPACE_EVEN_IF_HIDDEN: c_int = 2;
pub static SIZER_FLAG_BITS_MASK: c_int = 32770;
pub static STRETCH_NOT: c_int = 0;
pub static SHRINK: c_int = 4096;
pub static GROW: c_int = 8192;
pub static EXPAND: c_int = 8192;
pub static SHAPED: c_int = 16384;
pub static TILE: c_int = 49152;
pub static STRETCH_MASK: c_int = 28672;
pub static BORDER_DEFAULT: c_int = 0;
pub static BORDER_NONE: c_int = 2097152;
pub static BORDER_STATIC: c_int = 16777216;
pub static BORDER_SIMPLE: c_int = 33554432;
pub static BORDER_RAISED: c_int = 67108864;
pub static BORDER_SUNKEN: c_int = 134217728;
pub static BORDER_DOUBLE: c_int = 268435456;
pub static BORDER_THEME: c_int = 268435456;
pub static BORDER_MASK: c_int = 522190848;
pub static BG_STYLE_ERASE: c_int = 0;
pub static BG_STYLE_SYSTEM: c_int = 1;
pub static BG_STYLE_PAINT: c_int = 2;
pub static BG_STYLE_TRANSPARENT: c_int = 3;
pub static BG_STYLE_COLOUR: c_int = 4;
pub static BG_STYLE_CUSTOM: c_int = 2;
pub static KEY_NONE: c_int = 0;
pub static KEY_INTEGER: c_int = 1;
pub static KEY_STRING: c_int = 2;
pub static ID_AUTO_LOWEST: c_int = -1000000;
pub static ID_AUTO_HIGHEST: c_int = -2000;
pub static ID_NONE: c_int = -3;
pub static ID_SEPARATOR: c_int = -2;
pub static ID_ANY: c_int = -1;
pub static ID_LOWEST: c_int = 4999;
pub static ID_OPEN: c_int = 5000;
pub static ID_CLOSE: c_int = 5001;
pub static ID_NEW: c_int = 5002;
pub static ID_SAVE: c_int = 5003;
pub static ID_SAVEAS: c_int = 5004;
pub static ID_REVERT: c_int = 5005;
pub static ID_EXIT: c_int = 5006;
pub static ID_UNDO: c_int = 5007;
pub static ID_REDO: c_int = 5008;
pub static ID_HELP: c_int = 5009;
pub static ID_PRINT: c_int = 5010;
pub static ID_PRINT_SETUP: c_int = 5011;
pub static ID_PAGE_SETUP: c_int = 5012;
pub static ID_PREVIEW: c_int = 5013;
pub static ID_ABOUT: c_int = 5014;
pub static ID_HELP_CONTENTS: c_int = 5015;
pub static ID_HELP_INDEX: c_int = 5016;
pub static ID_HELP_SEARCH: c_int = 5017;
pub static ID_HELP_COMMANDS: c_int = 5018;
pub static ID_HELP_PROCEDURES: c_int = 5019;
pub static ID_HELP_CONTEXT: c_int = 5020;
pub static ID_CLOSE_ALL: c_int = 5021;
pub static ID_PREFERENCES: c_int = 5022;
pub static ID_EDIT: c_int = 5030;
pub static ID_CUT: c_int = 5031;
pub static ID_COPY: c_int = 5032;
pub static ID_PASTE: c_int = 5033;
pub static ID_CLEAR: c_int = 5034;
pub static ID_FIND: c_int = 5035;
pub static ID_DUPLICATE: c_int = 5036;
pub static ID_SELECTALL: c_int = 5037;
pub static ID_DELETE: c_int = 5038;
pub static ID_REPLACE: c_int = 5039;
pub static ID_REPLACE_ALL: c_int = 5040;
pub static ID_PROPERTIES: c_int = 5041;
pub static ID_VIEW_DETAILS: c_int = 5042;
pub static ID_VIEW_LARGEICONS: c_int = 5043;
pub static ID_VIEW_SMALLICONS: c_int = 5044;
pub static ID_VIEW_LIST: c_int = 5045;
pub static ID_VIEW_SORTDATE: c_int = 5046;
pub static ID_VIEW_SORTNAME: c_int = 5047;
pub static ID_VIEW_SORTSIZE: c_int = 5048;
pub static ID_VIEW_SORTTYPE: c_int = 5049;
pub static ID_FILE: c_int = 5050;
pub static ID_FILE1: c_int = 5051;
pub static ID_FILE2: c_int = 5052;
pub static ID_FILE3: c_int = 5053;
pub static ID_FILE4: c_int = 5054;
pub static ID_FILE5: c_int = 5055;
pub static ID_FILE6: c_int = 5056;
pub static ID_FILE7: c_int = 5057;
pub static ID_FILE8: c_int = 5058;
pub static ID_FILE9: c_int = 5059;
pub static ID_OK: c_int = 5100;
pub static ID_CANCEL: c_int = 5101;
pub static ID_APPLY: c_int = 5102;
pub static ID_YES: c_int = 5103;
pub static ID_NO: c_int = 5104;
pub static ID_STATIC: c_int = 5105;
pub static ID_FORWARD: c_int = 5106;
pub static ID_BACKWARD: c_int = 5107;
pub static ID_DEFAULT: c_int = 5108;
pub static ID_MORE: c_int = 5109;
pub static ID_SETUP: c_int = 5110;
pub static ID_RESET: c_int = 5111;
pub static ID_CONTEXT_HELP: c_int = 5112;
pub static ID_YESTOALL: c_int = 5113;
pub static ID_NOTOALL: c_int = 5114;
pub static ID_ABORT: c_int = 5115;
pub static ID_RETRY: c_int = 5116;
pub static ID_IGNORE: c_int = 5117;
pub static ID_ADD: c_int = 5118;
pub static ID_REMOVE: c_int = 5119;
pub static ID_UP: c_int = 5120;
pub static ID_DOWN: c_int = 5121;
pub static ID_HOME: c_int = 5122;
pub static ID_REFRESH: c_int = 5123;
pub static ID_STOP: c_int = 5124;
pub static ID_INDEX: c_int = 5125;
pub static ID_BOLD: c_int = 5126;
pub static ID_ITALIC: c_int = 5127;
pub static ID_JUSTIFY_CENTER: c_int = 5128;
pub static ID_JUSTIFY_FILL: c_int = 5129;
pub static ID_JUSTIFY_RIGHT: c_int = 5130;
pub static ID_JUSTIFY_LEFT: c_int = 5131;
pub static ID_UNDERLINE: c_int = 5132;
pub static ID_INDENT: c_int = 5133;
pub static ID_UNINDENT: c_int = 5134;
pub static ID_ZOOM_100: c_int = 5135;
pub static ID_ZOOM_FIT: c_int = 5136;
pub static ID_ZOOM_IN: c_int = 5137;
pub static ID_ZOOM_OUT: c_int = 5138;
pub static ID_UNDELETE: c_int = 5139;
pub static ID_REVERT_TO_SAVED: c_int = 5140;
pub static ID_CDROM: c_int = 5141;
pub static ID_CONVERT: c_int = 5142;
pub static ID_EXECUTE: c_int = 5143;
pub static ID_FLOPPY: c_int = 5144;
pub static ID_HARDDISK: c_int = 5145;
pub static ID_BOTTOM: c_int = 5146;
pub static ID_FIRST: c_int = 5147;
pub static ID_LAST: c_int = 5148;
pub static ID_TOP: c_int = 5149;
pub static ID_INFO: c_int = 5150;
pub static ID_JUMP_TO: c_int = 5151;
pub static ID_NETWORK: c_int = 5152;
pub static ID_SELECT_COLOR: c_int = 5153;
pub static ID_SELECT_FONT: c_int = 5154;
pub static ID_SORT_ASCENDING: c_int = 5155;
pub static ID_SORT_DESCENDING: c_int = 5156;
pub static ID_SPELL_CHECK: c_int = 5157;
pub static ID_STRIKETHROUGH: c_int = 5158;
pub static ID_SYSTEM_MENU: c_int = 5200;
pub static ID_CLOSE_FRAME: c_int = 5201;
pub static ID_MOVE_FRAME: c_int = 5202;
pub static ID_RESIZE_FRAME: c_int = 5203;
pub static ID_MAXIMIZE_FRAME: c_int = 5204;
pub static ID_ICONIZE_FRAME: c_int = 5205;
pub static ID_RESTORE_FRAME: c_int = 5206;
pub static ID_MDI_WINDOW_FIRST: c_int = 5230;
pub static ID_MDI_WINDOW_CASCADE: c_int = 5230;
pub static ID_MDI_WINDOW_TILE_HORZ: c_int = 5231;
pub static ID_MDI_WINDOW_TILE_VERT: c_int = 5232;
pub static ID_MDI_WINDOW_ARRANGE_ICONS: c_int = 5233;
pub static ID_MDI_WINDOW_PREV: c_int = 5234;
pub static ID_MDI_WINDOW_NEXT: c_int = 5235;
pub static ID_MDI_WINDOW_LAST: c_int = 5235;
pub static ID_OSX_MENU_FIRST: c_int = 5250;
pub static ID_OSX_HIDE: c_int = 5250;
pub static ID_OSX_HIDEOTHERS: c_int = 5251;
pub static ID_OSX_SHOWALL: c_int = 5252;
pub static ID_OSX_MENU_LAST: c_int = 5252;
pub static ID_FILEDLGG: c_int = 5900;
pub static ID_FILECTRL: c_int = 5950;
pub static ID_HIGHEST: c_int = 5999;
pub static ITEM_SEPARATOR: c_int = -1;
pub static ITEM_NORMAL: c_int = 0;
pub static ITEM_CHECK: c_int = 1;
pub static ITEM_RADIO: c_int = 2;
pub static ITEM_DROPDOWN: c_int = 3;
pub static ITEM_MAX: c_int = 4;
pub static CHK_UNCHECKED: c_int = 0;
pub static CHK_CHECKED: c_int = 1;
pub static CHK_UNDETERMINED: c_int = 2;
pub static HT_NOWHERE: c_int = 0;
pub static HT_SCROLLBAR_FIRST: c_int = 0;
pub static HT_SCROLLBAR_ARROW_LINE_1: c_int = 1;
pub static HT_SCROLLBAR_ARROW_LINE_2: c_int = 2;
pub static HT_SCROLLBAR_ARROW_PAGE_1: c_int = 3;
pub static HT_SCROLLBAR_ARROW_PAGE_2: c_int = 4;
pub static HT_SCROLLBAR_THUMB: c_int = 5;
pub static HT_SCROLLBAR_BAR_1: c_int = 6;
pub static HT_SCROLLBAR_BAR_2: c_int = 7;
pub static HT_SCROLLBAR_LAST: c_int = 8;
pub static HT_WINDOW_OUTSIDE: c_int = 9;
pub static HT_WINDOW_INSIDE: c_int = 10;
pub static HT_WINDOW_VERT_SCROLLBAR: c_int = 11;
pub static HT_WINDOW_HORZ_SCROLLBAR: c_int = 12;
pub static HT_WINDOW_CORNER: c_int = 13;
pub static HT_MAX: c_int = 14;
pub static HATCHSTYLE_INVALID: c_int = -1;
pub static HATCHSTYLE_FIRST: c_int = 111;
pub static HATCHSTYLE_BDIAGONAL: c_int = 111;
pub static HATCHSTYLE_CROSSDIAG: c_int = 112;
pub static HATCHSTYLE_FDIAGONAL: c_int = 113;
pub static HATCHSTYLE_CROSS: c_int = 114;
pub static HATCHSTYLE_HORIZONTAL: c_int = 115;
pub static HATCHSTYLE_VERTICAL: c_int = 116;
pub static HATCHSTYLE_LAST: c_int = 116;
pub static DEFAULT: c_int = 70;
pub static DECORATIVE: c_int = 71;
pub static ROMAN: c_int = 72;
pub static SCRIPT: c_int = 73;
pub static SWISS: c_int = 74;
pub static MODERN: c_int = 75;
pub static TELETYPE: c_int = 76;
pub static VARIABLE: c_int = 80;
pub static FIXED: c_int = 81;
pub static NORMAL: c_int = 90;
pub static LIGHT: c_int = 91;
pub static BOLD: c_int = 92;
pub static ITALIC: c_int = 93;
pub static SLANT: c_int = 94;
pub static SOLID: c_int = 100;
pub static DOT: c_int = 101;
pub static LONG_DASH: c_int = 102;
pub static SHORT_DASH: c_int = 103;
pub static DOT_DASH: c_int = 104;
pub static USER_DASH: c_int = 105;
pub static TRANSPARENT: c_int = 106;
pub static STIPPLE_MASK_OPAQUE: c_int = 107;
pub static STIPPLE_MASK: c_int = 108;
pub static STIPPLE: c_int = 110;
pub static BDIAGONAL_HATCH: c_int = 111;
pub static CROSSDIAG_HATCH: c_int = 112;
pub static FDIAGONAL_HATCH: c_int = 113;
pub static CROSS_HATCH: c_int = 114;
pub static HORIZONTAL_HATCH: c_int = 115;
pub static VERTICAL_HATCH: c_int = 116;
pub static FIRST_HATCH: c_int = 111;
pub static LAST_HATCH: c_int = 116;
pub static TOOL_TOP: c_int = 1;
pub static TOOL_BOTTOM: c_int = 2;
pub static TOOL_LEFT: c_int = 3;
pub static TOOL_RIGHT: c_int = 4;
pub static DF_INVALID: c_int = 0;
pub static DF_TEXT: c_int = 1;
pub static DF_BITMAP: c_int = 2;
pub static DF_METAFILE: c_int = 3;
pub static DF_SYLK: c_int = 4;
pub static DF_DIF: c_int = 5;
pub static DF_TIFF: c_int = 6;
pub static DF_OEMTEXT: c_int = 7;
pub static DF_DIB: c_int = 8;
pub static DF_PALETTE: c_int = 9;
pub static DF_PENDATA: c_int = 10;
pub static DF_RIFF: c_int = 11;
pub static DF_WAVE: c_int = 12;
pub static DF_UNICODETEXT: c_int = 13;
pub static DF_ENHMETAFILE: c_int = 14;
pub static DF_FILENAME: c_int = 15;
pub static DF_LOCALE: c_int = 16;
pub static DF_PRIVATE: c_int = 20;
pub static DF_HTML: c_int = 30;
pub static DF_MAX: c_int = 31;
pub static K_NONE: c_int = 0;
pub static K_CONTROL_A: c_int = 1;
pub static K_CONTROL_B: c_int = 2;
pub static K_CONTROL_C: c_int = 3;
pub static K_CONTROL_D: c_int = 4;
pub static K_CONTROL_E: c_int = 5;
pub static K_CONTROL_F: c_int = 6;
pub static K_CONTROL_G: c_int = 7;
pub static K_CONTROL_H: c_int = 8;
pub static K_CONTROL_I: c_int = 9;
pub static K_CONTROL_J: c_int = 10;
pub static K_CONTROL_K: c_int = 11;
pub static K_CONTROL_L: c_int = 12;
pub static K_CONTROL_M: c_int = 13;
pub static K_CONTROL_N: c_int = 14;
pub static K_CONTROL_O: c_int = 15;
pub static K_CONTROL_P: c_int = 16;
pub static K_CONTROL_Q: c_int = 17;
pub static K_CONTROL_R: c_int = 18;
pub static K_CONTROL_S: c_int = 19;
pub static K_CONTROL_T: c_int = 20;
pub static K_CONTROL_U: c_int = 21;
pub static K_CONTROL_V: c_int = 22;
pub static K_CONTROL_W: c_int = 23;
pub static K_CONTROL_X: c_int = 24;
pub static K_CONTROL_Y: c_int = 25;
pub static K_CONTROL_Z: c_int = 26;
pub static K_BACK: c_int = 8;
pub static K_TAB: c_int = 9;
pub static K_RETURN: c_int = 13;
pub static K_ESCAPE: c_int = 27;
pub static K_SPACE: c_int = 32;
pub static K_DELETE: c_int = 127;
pub static K_START: c_int = 300;
pub static K_LBUTTON: c_int = 301;
pub static K_RBUTTON: c_int = 302;
pub static K_CANCEL: c_int = 303;
pub static K_MBUTTON: c_int = 304;
pub static K_CLEAR: c_int = 305;
pub static K_SHIFT: c_int = 306;
pub static K_ALT: c_int = 307;
pub static K_CONTROL: c_int = 308;
pub static K_MENU: c_int = 309;
pub static K_PAUSE: c_int = 310;
pub static K_CAPITAL: c_int = 311;
pub static K_END: c_int = 312;
pub static K_HOME: c_int = 313;
pub static K_LEFT: c_int = 314;
pub static K_UP: c_int = 315;
pub static K_RIGHT: c_int = 316;
pub static K_DOWN: c_int = 317;
pub static K_SELECT: c_int = 318;
pub static K_PRINT: c_int = 319;
pub static K_EXECUTE: c_int = 320;
pub static K_SNAPSHOT: c_int = 321;
pub static K_INSERT: c_int = 322;
pub static K_HELP: c_int = 323;
pub static K_NUMPAD0: c_int = 324;
pub static K_NUMPAD1: c_int = 325;
pub static K_NUMPAD2: c_int = 326;
pub static K_NUMPAD3: c_int = 327;
pub static K_NUMPAD4: c_int = 328;
pub static K_NUMPAD5: c_int = 329;
pub static K_NUMPAD6: c_int = 330;
pub static K_NUMPAD7: c_int = 331;
pub static K_NUMPAD8: c_int = 332;
pub static K_NUMPAD9: c_int = 333;
pub static K_MULTIPLY: c_int = 334;
pub static K_ADD: c_int = 335;
pub static K_SEPARATOR: c_int = 336;
pub static K_SUBTRACT: c_int = 337;
pub static K_DECIMAL: c_int = 338;
pub static K_DIVIDE: c_int = 339;
pub static K_F1: c_int = 340;
pub static K_F2: c_int = 341;
pub static K_F3: c_int = 342;
pub static K_F4: c_int = 343;
pub static K_F5: c_int = 344;
pub static K_F6: c_int = 345;
pub static K_F7: c_int = 346;
pub static K_F8: c_int = 347;
pub static K_F9: c_int = 348;
pub static K_F10: c_int = 349;
pub static K_F11: c_int = 350;
pub static K_F12: c_int = 351;
pub static K_F13: c_int = 352;
pub static K_F14: c_int = 353;
pub static K_F15: c_int = 354;
pub static K_F16: c_int = 355;
pub static K_F17: c_int = 356;
pub static K_F18: c_int = 357;
pub static K_F19: c_int = 358;
pub static K_F20: c_int = 359;
pub static K_F21: c_int = 360;
pub static K_F22: c_int = 361;
pub static K_F23: c_int = 362;
pub static K_F24: c_int = 363;
pub static K_NUMLOCK: c_int = 364;
pub static K_SCROLL: c_int = 365;
pub static K_PAGEUP: c_int = 366;
pub static K_PAGEDOWN: c_int = 367;
pub static K_NUMPAD_SPACE: c_int = 368;
pub static K_NUMPAD_TAB: c_int = 369;
pub static K_NUMPAD_ENTER: c_int = 370;
pub static K_NUMPAD_F1: c_int = 371;
pub static K_NUMPAD_F2: c_int = 372;
pub static K_NUMPAD_F3: c_int = 373;
pub static K_NUMPAD_F4: c_int = 374;
pub static K_NUMPAD_HOME: c_int = 375;
pub static K_NUMPAD_LEFT: c_int = 376;
pub static K_NUMPAD_UP: c_int = 377;
pub static K_NUMPAD_RIGHT: c_int = 378;
pub static K_NUMPAD_DOWN: c_int = 379;
pub static K_NUMPAD_PAGEUP: c_int = 380;
pub static K_NUMPAD_PAGEDOWN: c_int = 381;
pub static K_NUMPAD_END: c_int = 382;
pub static K_NUMPAD_BEGIN: c_int = 383;
pub static K_NUMPAD_INSERT: c_int = 384;
pub static K_NUMPAD_DELETE: c_int = 385;
pub static K_NUMPAD_EQUAL: c_int = 386;
pub static K_NUMPAD_MULTIPLY: c_int = 387;
pub static K_NUMPAD_ADD: c_int = 388;
pub static K_NUMPAD_SEPARATOR: c_int = 389;
pub static K_NUMPAD_SUBTRACT: c_int = 390;
pub static K_NUMPAD_DECIMAL: c_int = 391;
pub static K_NUMPAD_DIVIDE: c_int = 392;
pub static K_WINDOWS_LEFT: c_int = 393;
pub static K_WINDOWS_RIGHT: c_int = 394;
pub static K_WINDOWS_MENU: c_int = 395;
pub static K_RAW_CONTROL: c_int = 396;
pub static K_COMMAND: c_int = 308;
pub static K_SPECIAL1: c_int = 193;
pub static K_SPECIAL2: c_int = 194;
pub static K_SPECIAL3: c_int = 195;
pub static K_SPECIAL4: c_int = 196;
pub static K_SPECIAL5: c_int = 197;
pub static K_SPECIAL6: c_int = 198;
pub static K_SPECIAL7: c_int = 199;
pub static K_SPECIAL8: c_int = 200;
pub static K_SPECIAL9: c_int = 201;
pub static K_SPECIAL10: c_int = 202;
pub static K_SPECIAL11: c_int = 203;
pub static K_SPECIAL12: c_int = 204;
pub static K_SPECIAL13: c_int = 205;
pub static K_SPECIAL14: c_int = 206;
pub static K_SPECIAL15: c_int = 207;
pub static K_SPECIAL16: c_int = 208;
pub static K_SPECIAL17: c_int = 209;
pub static K_SPECIAL18: c_int = 210;
pub static K_SPECIAL19: c_int = 211;
pub static K_SPECIAL20: c_int = 212;
pub static MOD_NONE: c_int = 0;
pub static MOD_ALT: c_int = 1;
pub static MOD_CONTROL: c_int = 2;
pub static MOD_ALTGR: c_int = 3;
pub static MOD_SHIFT: c_int = 4;
pub static MOD_META: c_int = 8;
pub static MOD_WIN: c_int = 8;
pub static MOD_RAW_CONTROL: c_int = 16;
pub static MOD_CMD: c_int = 2;
pub static MOD_ALL: c_int = 65535;
pub static PAPER_NONE: c_int = 0;
pub static PAPER_LETTER: c_int = 1;
pub static PAPER_LEGAL: c_int = 2;
pub static PAPER_A4: c_int = 3;
pub static PAPER_CSHEET: c_int = 4;
pub static PAPER_DSHEET: c_int = 5;
pub static PAPER_ESHEET: c_int = 6;
pub static PAPER_LETTERSMALL: c_int = 7;
pub static PAPER_TABLOID: c_int = 8;
pub static PAPER_LEDGER: c_int = 9;
pub static PAPER_STATEMENT: c_int = 10;
pub static PAPER_EXECUTIVE: c_int = 11;
pub static PAPER_A3: c_int = 12;
pub static PAPER_A4SMALL: c_int = 13;
pub static PAPER_A5: c_int = 14;
pub static PAPER_B4: c_int = 15;
pub static PAPER_B5: c_int = 16;
pub static PAPER_FOLIO: c_int = 17;
pub static PAPER_QUARTO: c_int = 18;
pub static PAPER_10X14: c_int = 19;
pub static PAPER_11X17: c_int = 20;
pub static PAPER_NOTE: c_int = 21;
pub static PAPER_ENV_9: c_int = 22;
pub static PAPER_ENV_10: c_int = 23;
pub static PAPER_ENV_11: c_int = 24;
pub static PAPER_ENV_12: c_int = 25;
pub static PAPER_ENV_14: c_int = 26;
pub static PAPER_ENV_DL: c_int = 27;
pub static PAPER_ENV_C5: c_int = 28;
pub static PAPER_ENV_C3: c_int = 29;
pub static PAPER_ENV_C4: c_int = 30;
pub static PAPER_ENV_C6: c_int = 31;
pub static PAPER_ENV_C65: c_int = 32;
pub static PAPER_ENV_B4: c_int = 33;
pub static PAPER_ENV_B5: c_int = 34;
pub static PAPER_ENV_B6: c_int = 35;
pub static PAPER_ENV_ITALY: c_int = 36;
pub static PAPER_ENV_MONARCH: c_int = 37;
pub static PAPER_ENV_PERSONAL: c_int = 38;
pub static PAPER_FANFOLD_US: c_int = 39;
pub static PAPER_FANFOLD_STD_GERMAN: c_int = 40;
pub static PAPER_FANFOLD_LGL_GERMAN: c_int = 41;
pub static PAPER_ISO_B4: c_int = 42;
pub static PAPER_JAPANESE_POSTCARD: c_int = 43;
pub static PAPER_9X11: c_int = 44;
pub static PAPER_10X11: c_int = 45;
pub static PAPER_15X11: c_int = 46;
pub static PAPER_ENV_INVITE: c_int = 47;
pub static PAPER_LETTER_EXTRA: c_int = 48;
pub static PAPER_LEGAL_EXTRA: c_int = 49;
pub static PAPER_TABLOID_EXTRA: c_int = 50;
pub static PAPER_A4_EXTRA: c_int = 51;
pub static PAPER_LETTER_TRANSVERSE: c_int = 52;
pub static PAPER_A4_TRANSVERSE: c_int = 53;
pub static PAPER_LETTER_EXTRA_TRANSVERSE: c_int = 54;
pub static PAPER_A_PLUS: c_int = 55;
pub static PAPER_B_PLUS: c_int = 56;
pub static PAPER_LETTER_PLUS: c_int = 57;
pub static PAPER_A4_PLUS: c_int = 58;
pub static PAPER_A5_TRANSVERSE: c_int = 59;
pub static PAPER_B5_TRANSVERSE: c_int = 60;
pub static PAPER_A3_EXTRA: c_int = 61;
pub static PAPER_A5_EXTRA: c_int = 62;
pub static PAPER_B5_EXTRA: c_int = 63;
pub static PAPER_A2: c_int = 64;
pub static PAPER_A3_TRANSVERSE: c_int = 65;
pub static PAPER_A3_EXTRA_TRANSVERSE: c_int = 66;
pub static PAPER_DBL_JAPANESE_POSTCARD: c_int = 67;
pub static PAPER_A6: c_int = 68;
pub static PAPER_JENV_KAKU2: c_int = 69;
pub static PAPER_JENV_KAKU3: c_int = 70;
pub static PAPER_JENV_CHOU3: c_int = 71;
pub static PAPER_JENV_CHOU4: c_int = 72;
pub static PAPER_LETTER_ROTATED: c_int = 73;
pub static PAPER_A3_ROTATED: c_int = 74;
pub static PAPER_A4_ROTATED: c_int = 75;
pub static PAPER_A5_ROTATED: c_int = 76;
pub static PAPER_B4_JIS_ROTATED: c_int = 77;
pub static PAPER_B5_JIS_ROTATED: c_int = 78;
pub static PAPER_JAPANESE_POSTCARD_ROTATED: c_int = 79;
pub static PAPER_DBL_JAPANESE_POSTCARD_ROTATED: c_int = 80;
pub static PAPER_A6_ROTATED: c_int = 81;
pub static PAPER_JENV_KAKU2_ROTATED: c_int = 82;
pub static PAPER_JENV_KAKU3_ROTATED: c_int = 83;
pub static PAPER_JENV_CHOU3_ROTATED: c_int = 84;
pub static PAPER_JENV_CHOU4_ROTATED: c_int = 85;
pub static PAPER_B6_JIS: c_int = 86;
pub static PAPER_B6_JIS_ROTATED: c_int = 87;
pub static PAPER_12X11: c_int = 88;
pub static PAPER_JENV_YOU4: c_int = 89;
pub static PAPER_JENV_YOU4_ROTATED: c_int = 90;
pub static PAPER_P16K: c_int = 91;
pub static PAPER_P32K: c_int = 92;
pub static PAPER_P32KBIG: c_int = 93;
pub static PAPER_PENV_1: c_int = 94;
pub static PAPER_PENV_2: c_int = 95;
pub static PAPER_PENV_3: c_int = 96;
pub static PAPER_PENV_4: c_int = 97;
pub static PAPER_PENV_5: c_int = 98;
pub static PAPER_PENV_6: c_int = 99;
pub static PAPER_PENV_7: c_int = 100;
pub static PAPER_PENV_8: c_int = 101;
pub static PAPER_PENV_9: c_int = 102;
pub static PAPER_PENV_10: c_int = 103;
pub static PAPER_P16K_ROTATED: c_int = 104;
pub static PAPER_P32K_ROTATED: c_int = 105;
pub static PAPER_P32KBIG_ROTATED: c_int = 106;
pub static PAPER_PENV_1_ROTATED: c_int = 107;
pub static PAPER_PENV_2_ROTATED: c_int = 108;
pub static PAPER_PENV_3_ROTATED: c_int = 109;
pub static PAPER_PENV_4_ROTATED: c_int = 110;
pub static PAPER_PENV_5_ROTATED: c_int = 111;
pub static PAPER_PENV_6_ROTATED: c_int = 112;
pub static PAPER_PENV_7_ROTATED: c_int = 113;
pub static PAPER_PENV_8_ROTATED: c_int = 114;
pub static PAPER_PENV_9_ROTATED: c_int = 115;
pub static PAPER_PENV_10_ROTATED: c_int = 116;
pub static PAPER_A0: c_int = 117;
pub static PAPER_A1: c_int = 118;
pub static PORTRAIT: c_int = 1;
pub static LANDSCAPE: c_int = 2;
pub static DUPLEX_SIMPLEX: c_int = 0;
pub static DUPLEX_HORIZONTAL: c_int = 1;
pub static DUPLEX_VERTICAL: c_int = 2;
pub static PRINT_MODE_NONE: c_int = 0;
pub static PRINT_MODE_PREVIEW: c_int = 1;
pub static PRINT_MODE_FILE: c_int = 2;
pub static PRINT_MODE_PRINTER: c_int = 3;
pub static PRINT_MODE_STREAM: c_int = 4;
pub static UPDATE_UI_NONE: c_int = 0;
pub static UPDATE_UI_RECURSE: c_int = 1;
pub static UPDATE_UI_FROMIDLE: c_int = 2;
