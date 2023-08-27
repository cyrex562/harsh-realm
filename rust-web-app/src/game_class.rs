// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.GameClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Threading;
// usingSystem.Windows.Forms;
// usingWindowsApplication1.My;

pub const REMOVEWINDOW: i32 =  1;
    pub const OPENWINDOW: i32 =  2;
    pub const REFRESHWINDOW: i32 =  4;
    pub const NEWSCREEN: i32 =  3;
    pub const NEWPOPUPSCREEN: i32 =  5;
    pub const CLOSEPOPUPSCREEN: i32 =  6;
    pub const REPAINTWINDOW: i32 =  7;
    pub const THEWINDOW: i32 =  1;
    pub const LEFTWINDOW: i32 =  2;
    pub const RIGHTWINDOW: i32 =  3;
    pub const MIDDLEWINDOW: i32 =  4;
    pub const DOWNWINDOW: i32 =  5;
    pub const EDITMENUWINDOW: i32 =  6;
    pub const MIDDOWNWINDOW: i32 =  7;
    pub const RESWINDOW: i32 =  8;
    pub const TABSWINDOW: i32 =  9;
    pub const LANDSCAPETYPEWINDOW: i32 =  11;
    pub const MAPWINDOW: i32 =  12;
    pub const EDITOPTIONSWINDOW: i32 =  13;
    pub const ROADTYPEWINDOW: i32 =  14;
    pub const REGIMEWINDOW: i32 =  15;
    pub const SFTYPEWINDOW: i32 =  16;
    pub const EDITUNITWINDOW: i32 =  17;
    pub const PLAYMAINWINDOW: i32 =  18;
    pub const MAINLOOPWINDOW: i32 =  19;
    pub const PLAYEXTRAWINDOW: i32 =  20;
    pub const LOCTYPEWINDOW: i32 =  21;
    pub const PEOPLEWINDOW: i32 =  22;
    pub const GENERALWINDOW: i32 =  23;
    pub const ITEMTYPEWINDOW: i32 =  24;
    pub const PRODWINDOW: i32 =  25;
    pub const LOCWINDOW: i32 =  26;
    pub const NEWSFWINDOW: i32 =  27;
    pub const NEWUNITWINDOW: i32 =  28;
    pub const INFOWINDOW: i32 =  29;
    pub const TRANSFERWINDOW: i32 =  30;
    pub const COMBATSTATUSWINDOW: i32 =  31;
    pub const COMBATRESULTWINDOW: i32 =  32;
    pub const RIVERTYPEWINDOW: i32 =  33;
    pub const BRIDGEWINDOW: i32 =  34;
    pub const STRATEGICWINDOW: i32 =  35;
    pub const RESEARCHWINDOW: i32 =  37;
    pub const PLAYRESEARCHWINDOW: i32 =  38;
    pub const DIPWINDOW: i32 =  39;
    pub const CONSTRUCTWINDOW: i32 =  40;
    pub const HISTORYWINDOW: i32 =  41;
    pub const EVENTWINDOW: i32 =  42;
    pub const SFWINDOW: i32 =  43;
    pub const ORDERWINDOW: i32 =  44;
    pub const PREFSWINDOW: i32 =  45;
    pub const STATSWINDOW: i32 =  46;
    pub const PEOPLETRANSFERWINDOW: i32 =  47;
    pub const BUILDWINDOW: i32 =  48;
    pub const INTROWINDOW: i32 =  49;
    pub const RANDOMWINDOW: i32 =  50;
    pub const AIRSUPPLYWINDOW: i32 =  51;
    pub const PRODFLAPWINDOW: i32 =  52;
    pub const BIGMINIMAPWINDOW: i32 =  53;
    pub const WELCOMEWINDOW: i32 =  54;
    pub const INITIALMENUWINDOW: i32 =  55;
    pub const HISTORICWINDOW: i32 =  56;
    pub const ACTIONCARDWINDOW: i32 =  57;
    pub const CONNECTIONWINDOW: i32 =  58;
    pub const NEWUNIT2WINDOW: i32 =  59;
    pub const STRINGLISTWINDOW: i32 =  60;
    pub const OFFICERPOOLWINDOW: i32 =  61;
    pub const OFFICERINFOWINDOW: i32 =  62;
    pub const CHANGEMODELWINDOW: i32 =  63;
    pub const MODELDESIGNERWINDOW: i32 =  64;
    pub const DESIGNSFWINDOW: i32 =  65;
    pub const RESOURCEWINDOW: i32 =  66;
    pub const RESOURCEWINDOW2: i32 =  67;
    pub const ORDERWINDOW2: i32 =  68;
    pub const PLAYEXTRAWINDOW2: i32 =  69;
    pub const TABBRIEFING2: i32 =  70;
    pub const TABSTATS2: i32 =  71;
    pub const TABOOB2: i32 =  72;
    pub const TABREPORTS2: i32 =  73;
    pub const TABCARDS2: i32 =  74;
    pub const TABSMAP2: i32 =  75;
    pub const TABSMINI: i32 =  76;
    pub const STRATEGICWINDOW2: i32 =  76;
    pub const TABSPREFS: i32 =  77;
    pub const SYSTEMMESSAGEWINDOW: i32 =  78;
    pub const INTROWINDOW2: i32 =  49;
    pub const HISTORYWINDOW2: i32 =  80;
    pub const HISTORYORDERWINDOW: i32 =  81;
    pub const COMBATDETAILWINDOW: i32 =  82;
    pub const CREDITSWINDOW: i32 =  83;
    pub const COMBATWINDOW: i32 =  84;
    pub const WELCOMEWINDOW2: i32 =  85;
    pub const EDITHISWINDOW: i32 =  86;
    pub const OFFICERPOOLWINDOW2: i32 =  87;
    pub const NEWUNITWINDOW2: i32 =  88;
    pub const NEWUNIT2WINDOW2: i32 =  89;
    pub const CHANGEMODELWINDOW2: i32 =  90;
    pub const COREPBEMWINDOW: i32 =  91;
    pub const PBEMINITIALIZEWINDOW: i32 =  92;
    pub const LIBRARYWINDOW: i32 =  93;
    pub const SIMPLEEDITOPTIONSWINDOW: i32 =  94;
    pub const SIMPLEEDITDASHBOARDWINDOW: i32 =  95;
    pub const SIMPLEEDITLIBRARYWINDOW: i32 =  96;
    pub const SIMPLELIBIMPORTPOPUP: i32 =  97;
    pub const SIMPLEEDITMAPWINDOW: i32 =  98;
    pub const SIMPLEEDITUNITWINDOW: i32 =  99;
    pub const SIMPLEDITREGIMEWINDOW: i32 =  100;
    pub const SIMPLEDITTABLEWINDOW: i32 =  101;
    pub const SIMPLEMAPWINDOW: i32 =  102;
    pub const SIMPLETROOPTYPEWINDOW: i32 =  103;
    pub const SIMPLEHISWINDOW: i32 =  104;
    pub const SIMPLEOFFICERWINDOW: i32 =  105;
    pub const BIGMESSAGEWINDOW: i32 =  106;
    pub const SIMPLEMAPDASHBOARDWINDOW: i32 =  107;
    pub const SIMPLEMAPOPTIONSWINDOW: i32 =  108;
    pub const SIMPLEDEBUGWINDOW: i32 =  109;
    pub const TABHELP: i32 =  110;
    pub const SIMPLEPREFSWINDOW: i32 =  111;
    pub const SIMPLEMAPREPLACEMENTWINDOW: i32 =  112;
    pub const TABMANAGEMENT: i32 =  113;
    pub const RANDOMTOP: i32 =  114;
    pub const RANDOMMIDDLE: i32 =  115;
    pub const RANDOMBOTTOM: i32 =  116;
    pub const COMBATSELECTWINDOW: i32 =  117;
    pub const UDSUNITOPSWINDOW: i32 =  118;
    pub const ADVICEWINDOW: i32 =  119;
    pub const LISTRAFFICWINDOW: i32 =  120;
    pub const SUPPLYLAYERWINDOW: i32 =  121;
    pub const SSEVENTPICS: i32 =  122;
    pub const SSSMALLGFX: i32 =  123;
    pub const FIRSTSCREEN: i32 =  1;
    pub const MAINEDITSCREEN: i32 =  2;
    pub const PLAYSCREEN: i32 =  3;
    pub const MAINLOOPSCREEN: i32 =  4;
    pub const BATTLESCREEN: i32 =  5;
    pub const HISTORYSCREEN: i32 =  6;
    pub const EVENTSCREEN: i32 =  7;
    pub const SFSCREEN: i32 =  8;
    pub const STATSSCREEN: i32 =  9;
    pub const MESSAGEPOPUPSCREEN: i32 =  10;
    pub const PLAYSCREEN2: i32 =  11;
    pub const FIRSTSCREEN2: i32 =  12;
    pub const MAINLOOPSCREEN2: i32 =  13;
    pub const MESSAGEPOPUPSCREEN2: i32 =  14;
    pub const SFSCREEN2: i32 =  15;
    pub const HISTORYSCREEN2: i32 =  16;
    pub const SIMPLEEDITSCREEN: i32 =  17;
    pub const SIMPLETROOPTYPESCREEN: i32 =  18;
    pub const SIMPLEHISSCREEN: i32 =  19;
    pub const SIMPLEOFFICERSCREEN: i32 =  20;
    pub const SIMPLEMAPSCREEN: i32 =  21;
    pub const DYNAMICCINEMATICSSCREEN: i32 =  22;
    pub const RANDOMSCREEN2: i32 =  23;
    pub const MANAGEMENTSCREEN: i32 =  24;
    pub const SIMPLESSSCREEN: i32 =  25;
    pub const MAXROUND: i32 =  10;
    pub const GAMENAME: String = String::from("Shadow Empire : Planetary Conquest");
    
    pub const dlc_SE_Astro: i32 =  1;
    pub const dlc_SE_Navy: i32 =  2;
    

// namespace WindowsApplication1
// {
#[derive(Default,Debug,Copy,Clone)]
pub struct GameClass
{
    pub AppPath: String,
    pub AppPath_SAVEGAMES: String,
    pub UseSlimDX: bool,
    pub ScreenWidth: i32,
    pub ScreenHeight: i32,
    pub RealScreenWidth: i32,
    pub RealScreenHeight: i32,
    pub StartupScreenWidth: i32,
    pub StartupScreenHeight: i32,
    pub SuperAdminRights: bool,
    pub UserTestMode: bool,
    pub UserDebugger: bool,
    pub noDrawNoFocus: bool,
    pub EditorBlock: bool,
    pub EditorBlockSimple: bool,
    pub MetricsURL: String,
    pub Metrics2URL: String,
    pub EditorName: String,
    pub AlternativeGraphics: String,
    pub dlc_Counter: i32,
    pub dlc_ID: Vec<i32>,
    pub modlib_Counter: i32,
    pub modlib_Name: Vec<String>,
    pub modlib_Designer: Vec<String>,
    pub modlib_Description: Vec<String>,
    pub modlib_Version: Vec<i32>,
    pub modlib_Filename: Vec<String>,
    pub modlib_Flagged: Vec<bool>,
    pub IsWin10: bool,
    pub theaterThreadBlock: bool,
    pub AllowHeightMap: bool,
    pub EmpireStyle: bool,
    pub HighGraphicsSpeedPossible: bool,
    pub useModLib: bool,
    pub BUTTON1: i32,
    pub TUTARROW: i32,
    pub BUTTON2: i32,
    pub BUTTON3: i32,
    pub BUTTONUP: i32,
    pub BUTTONDOWN: i32,
    pub BUTTON4: i32,
    pub BUTTON5: i32,
    pub BUTTONBLOCK: i32,
    pub BUTTONCHANGEMODEL: i32,
    pub BUTTONMODELDESIGNER: i32,
    pub BUTTONFLAGGED: i32,
    pub BUTTONUNFLAGGED: i32,
    pub BUTTONPLUS: i32,
    pub BUTTONKILL: i32,
    pub BUTTONDRAW: i32,
    pub BUTTONLONG: i32,
    pub BUTTONLONGHIGH: i32,
    pub TEMP0: i32,
    pub TEMP1: i32,
    pub NOHEX: i32,
    pub SELECTEDHEX: i32,
    pub WHITEHEX: i32,
    pub DEFAULTCOUNTER: i32,
    pub BUTTONMOVE: i32,
    pub BUTTONNEXT: i32,
    pub BUTTONHQPROD: i32,
    pub BUTTONHQUNIT: i32,
    pub BUTTONPROD: i32,
    pub BUTTONRECRUIT: i32,
    pub BUTTONSF: i32,
    pub BUTTONITEMS: i32,
    pub BUTTONNEWSF: i32,
    pub MARCBUTBARFRAME: i32,
    pub BACKGROUND1MARC: i32,
    pub BACKGROUND1MARC2: i32,
    pub BACKTUT: i32,
    pub BUTTONSTATS: i32,
    pub BUTTONGIVEUNIT: i32,
    pub BUTTONGIVEHEX: i32,
    pub BUTTONREPORT: i32,
    pub BUTTONNEWUNIT: i32,
    pub NEUTRAL: i32,
    pub NEUTRALHIGH: i32,
    pub BUTTONTRANSFER: i32,
    pub BUTTONATTACK: i32,
    pub BUTTONARTATTACK: i32,
    pub BUTTONSEAATTACK: i32,
    pub BUTTONSEAARTATTACK: i32,
    pub BUTTONMOVE2: i32,
    pub BUTTONSTRATEGIC2: i32,
    pub BUTTONAIRATTACK: i32,
    pub BUTTONINTERDICT: i32,
    pub BUTTON25: i32,
    pub BUTTON50: i32,
    pub BUTTON75: i32,
    pub BUTTON100: i32,
    pub BUTTONLEFT: i32,
    pub BUTTONRIGHT: i32,
    pub BUTTONLANDCAP: i32,
    pub BUTTONSEACAP: i32,
    pub BUTTONAIRCAP: i32,
    pub BUTTONSTRATEGIC: i32,
    pub BUTTONMAKEHQ: i32,
    pub MAINFRAME: i32,
    pub MAINBACKGROUND: i32,
    pub BUTTONPARADROP: i32,
    pub BUTTONLOAD: i32,
    pub BUTTONHEXUNIT2: i32,
    pub BUTTONUNLOAD: i32,
    pub BUTTONPOOL: i32,
    pub BUTTONNEWUNIT2: i32,
    pub BUTTONREMOVEOFFICER: i32,
    pub BUTTONASSIGNOFFICER: i32,
    pub BUTTONBUYOFFICER: i32,
    pub BUTTONDIP: i32,
    pub BUTTONRESEARCH: i32,
    pub BUTTONCONSTRUCT: i32,
    pub BUTTONBUILDROAD: i32,
    pub BUTTONBLOWBRIDGE: i32,
    pub BUTTONBUILDLOCATION: i32,
    pub BUTTONUPGRADELOCATION: i32,
    pub BUTTONREPAIRLOCATION: i32,
    pub BUTTONHISTORY: i32,
    pub ATTACKART2: i32,
    pub ATTACK0: i32,
    pub ATTACK1: i32,
    pub ATTACK2: i32,
    pub ATTACK3: i32,
    pub ATTACK4: i32,
    pub ATTACK5: i32,
    pub ATTACKART: i32,
    pub ATTACKAMPH: i32,
    pub ATTACKPARADROP: i32,
    pub ATTACKAIR: i32,
    pub SUPPLIESSYMBOL: i32,
    pub BUTTONDISBAND: i32,
    pub BUTTONSAVE: i32,
    pub BUTTONQUIT: i32,
    pub QUICKINFO: i32,
    pub OFFICERINFO: i32,
    pub BUTTONNONO: i32,
    pub BUTTONLANDCAP2: i32,
    pub BUTTONSEACAP2: i32,
    pub PAPER1: i32,
    pub BUTTONAIRCAP2: i32,
    pub BUTTONMARC1: i32,
    pub BUTTONMARC1b: i32,
    pub EXTRABACKGROUND: i32,
    pub MINIMAPBACKGROUND: i32,
    pub HEXINFOBACKGROUND: i32,
    pub OKBALL: i32,
    pub CANCELBALL: i32,
    pub BACKGROUND2MARC: i32,
    pub LISTUP: i32,
    pub LISTDOWN: i32,
    pub LISTBLOCK: i32,
    pub QUESTIONBALL: i32,
    pub BACKGROUND3MARC: i32,
    pub BACKGROUND5MARC: i32,
    pub MARCINTRO1: i32,
    pub MARCINTRO2: i32,
    pub TARGETHEX: i32,
    pub BUTTONPREFS: i32,
    pub WHITEFLAG: i32,
    pub BUTTONGUIUP: i32,
    pub BUTTONGUIDOWN: i32,
    pub PAPERBACK1: i32,
    pub PAPERBACK2: i32,
    pub PAPERBACK3: i32,
    pub SLITHERINE: i32,
    pub BUTTONCOPY: i32,
    pub BUTTONPASTE: i32,
    pub BUTTONSTATISTICS: i32,
    pub BUTTONDISBANDTROOPS: i32,
    pub BUTTONDISBANDITEMS: i32,
    pub BUTTONAIRRECON: i32,
    pub BUTTONPEOPLETRANSFER: i32,
    pub BACKBUTTON: i32,
    pub BUTTONQUITSAVE: i32,
    pub BUTTONSUPPLYON: i32,
    pub BUTTONSUPPLYOFF: i32,
    pub HEXRASTER: i32,
    pub BUYBUTTON: i32,
    pub BLOWLOCATIONBUTTON: i32,
    pub DEFAULTCOUNTERSMALL: i32,
    pub PERCENT: Vec<i32>,
    pub PRODSUPPLY: i32,
    pub PRODPP: i32,
    pub PERCENTX: i32,
    pub SHADEDHEX: i32,
    pub BUTTONAIRSUPPLY: i32,
    pub BORDER: Vec<i32>,
    pub MAPBORDER: Vec<i32>,
    pub ZONEBORDER: Vec<i32>,
    pub LIGHTZONEBORDER: Vec<i32>,
    pub LOGIN: i32,
    pub NOBRIDGE: Vec<i32>,
    pub BUTTONLEFT2: i32,
    pub BUTTONRIGHT2: i32,
    pub BLACKHEX: i32,
    pub FIRSTHEX: i32,
    pub PIN1: i32,
    pub PIN1SHADE: i32,
    pub PIN2: i32,
    pub PIN2SHADE: i32,
    pub PIN3: i32,
    pub PIN3SHADE: i32,
    pub PIN4: i32,
    pub PIN4SHADE: i32,
    pub PIN5: i32,
    pub PIN5SHADE: i32,
    pub PIN6: i32,
    pub PIN6SHADE: i32,
    pub PIN7: i32,
    pub PIN7shade: i32,
    pub BOTTOMRED: i32,
    pub BACKGROUND4: i32,
    pub BOTTOMGREEN: i32,
    pub BOTTOMBLACK: i32,
    pub BUTTONSURRENDER: i32,
    pub REDOVAL: i32,
    pub BLUEOVAL: i32,
    pub BROWNOVAL: i32,
    pub EDITDRAW: i32,
    pub EDITPOINTER: i32,
    pub EDITPAINT: i32,
    pub ADV1: i32,
    pub ADV2: i32,
    pub ADV3: i32,
    pub BUTTONHEXUNIT: i32,
    pub BUTTONHEX: i32,
    pub BUTTONSTEVE1: i32,
    pub BUTTONSTEVE1b: i32,
    pub LONGBUTTON: i32,
    pub LONGBUTTONHIGH: i32,
    pub NONEBUTTON: i32,
    pub ALLBUTTON: i32,
    pub SYSTEM1: i32,
    pub SYSTEM1B: i32,
    pub SYSTEM2: i32,
    pub SYSTEM2B: i32,
    pub EMPTYSLOT: i32,
    pub VSLIDER: i32,
    pub VSLIDERB: i32,
    pub BUTTONHEXINFO: i32,
    pub BUTTONHEXINFO2: i32,
    pub ACTIONCARD: i32,
    pub ACTIONCARD2: i32,
    pub ACTIONFRAME: i32,
    pub BUTTONLEFTB: i32,
    pub BUTTONLEFT2B: i32,
    pub BUTTONRIGHTB: i32,
    pub BUTTONRIGHT2B: i32,
    pub STATSGRADIENT: i32,
    pub REDBUTTON: i32,
    pub COMBATGRADIENT: i32,
    pub SLIDER1: i32,
    pub SLIDER2: i32,
    pub MGSPLASH: i32,
    pub VRSPLASH: i32,
    pub BLACKOVAL: i32,
    pub BLACKBOX: i32,
    pub BUTTONOFFICER: i32,
    pub TEXTCLOUD: i32,
    pub BUTTONZOOMIN: i32,
    pub MINICARD: i32,
    pub MINICARDBIG: i32,
    pub DEFAULTCOUNTERBIG: i32,
    pub BUTTONZOOMOUT: i32,
    pub BUTTONSTACKEDUNIT: i32,
    pub BUTTONSPREADUNIT: i32,
    pub BUTTONDESIGNSF: i32,
    pub FRAME1: i32,
    pub RADIO1: i32,
    pub RADIO2: i32,
    pub RADIO1A: i32,
    pub RADIO2A: i32,
    pub LISTBAR: i32,
    pub LISTBACK: i32,
    pub LOGOFLATTINY: i32,
    pub EXPLOSION: i32,
    pub TUTHEX: i32,
    pub SMALLCHAR1: i32,
    pub SMALLCHAR2: i32,
    pub ICONSUP2: i32,
    pub ICONSTR: i32,
    pub ICONFLAG: i32,
    pub ICONEN1: i32,
    pub ICONEN2: i32,
    pub ICONMO1: i32,
    pub ICONMO2: i32,
    pub ICONEX1: i32,
    pub ICONEX1_VARIED: i32,
    pub ICONEX2: i32,
    pub ICONRD1: i32,
    pub ICONRD2: i32,
    pub ICONSU1: i32,
    pub ICONAP1: i32,
    pub ICONIN1: i32,
    pub FOGSHEET: i32,
    pub SHROWDSHEET: i32,
    pub NEUTRALREGIMEBACK: Bitmap,
    pub CARD1: Bitmap,
    pub CARD2: Bitmap,
    pub CARD3: Bitmap,
    pub CARD4: Bitmap,
    pub CARD5: Bitmap,
    pub CARD1B: Bitmap,
    pub CARD2B: Bitmap,
    pub CARD3B: Bitmap,
    pub MARCOPTSLOTS: i32,
    pub LEATHER: i32,
    pub MARCTAB: i32,
    pub BUTTONSTEVE2: i32,
    pub ORNAMENT1: i32,
    pub BUTTONBLUE: i32,
    pub BUTTONYELLOW: i32,
    pub ORNAMENT2: i32,
    pub ORNAMENT3A: i32,
    pub ORNAMENT3B: i32,
    pub ORNAMENT4: i32,
    pub BUTTONSTEVE2B: i32,
    pub MARCTABBUTTON: i32,
    pub MARCTABBUTTONHIGH: i32,
    pub MARCARROW: i32,
    pub COMBATART1: i32,
    pub COMBATART2: i32,
    pub MARCMESFRAME: i32,
    pub MARCTOPBAR: i32,
    pub MARCOFFICER: i32,
    pub MARCSCOPE: i32,
    pub MARCBUTBAR: i32,
    pub MARCBUTBARHISTORY: i32,
    pub MARCBOTBAR: i32,
    pub MARCLARGETAB: i32,
    pub MARCBIGBOTTOMTAB: i32,
    pub ATTACKREINFORCEMENTS: i32,
    pub MARCBLOCK: i32,
    pub MARCBLOCKHIGH: i32,
    pub MARCBLOCKPRESSED: i32,
    pub MARCBLOCK2: i32,
    pub MARCBLOCKHIGH2: i32,
    pub MARCBLOCKPRESSED2: i32,
    pub BUTTONLEFTBIG: i32,
    pub BUTTONRIGHTBIG: i32,
    pub BUTTONRIGHT2BIG: i32,
    pub BUTTONLEFT2BIG: i32,
    pub VSLIDERBIG: i32,
    pub WARNINGTRIANGLE: i32,
    pub ICONSUPATT: i32,
    pub ICONSUPDEF: i32,
    pub ICONHQPOW: i32,
    pub ICONEP: i32,
    pub ICONVIGOR: i32,
    pub ICON: i32,
    pub HEXMASKBIG: i32,
    pub HEXMASKMEDIUM: i32,
    pub HEXMASKSMALL: i32,
    pub MARCCARD1A: i32,
    pub MARCCARD1B: i32,
    pub MARCCARD1C: i32,
    pub MARCCARD2A: i32,
    pub MARCCARD2B: i32,
    pub MARCCARD2C: i32,
    pub MARCCARD3A: i32,
    pub MARCCARD3B: i32,
    pub MARCCARD3C: i32,
    pub MARCCARD4A: i32,
    pub MARCCARD4B: i32,
    pub MARCCARD4C: i32,
    pub MARCCARD6A: i32,
    pub MARCCARD6B: i32,
    pub MARCCARD6C: i32,
    pub SECARDOUTLINE: i32,
    pub BACKGROUND4MARC: i32,
    pub MARCCARD5A: i32,
    pub MARCCARD5B: i32,
    pub MARCCARD5C: i32,
    pub MARCBACK1: i32,
    pub MARCBACK1B: i32,
    pub MARCBACK2: i32,
    pub MARCBACK2B: i32,
    pub MARCBACK3: i32,
    pub MARCBACK3B: i32,
    pub MARCBACK4: i32,
    pub MARCBACK4B: i32,
    pub SUPPLYBLOCK: i32,
    pub TRAFFIC1: i32,
    pub TRAFFIC2: i32,
    pub TRAFFIC3: i32,
    pub TRAFFIC4: i32,
    pub EYE1: i32,
    pub EYE2: i32,
    pub EYE3: i32,
    pub EYE4: i32,
    pub FIRELISTICONS: i32,
    pub COMBATICONS: i32,
    pub DC4COUNTER: i32,
    pub FRONTLINETILESET: i32,
    pub FOG: Vec<i32>,
    pub SHROWD: Vec<i32>,
    pub ARROWSHEET: i32,
    pub WHITEHEXTRANS: i32,
    pub WHITEHEXTRANS2: i32,
    pub WHITEHEXTRANS3: i32,
    pub LINES: i32,
    pub LINESFRAME: i32,
    pub DOWN20: i32,
    pub UP20: i32,
    pub ROUNDBALL: i32,
    pub RESEARCHOVERPRINT: i32,
    pub SPRITE64: Vec<i32>,
    pub ARROW64: Vec<i32>,
    pub SHEETX: Vec<i32>,
    pub SHEETY: Vec<i32>,
    pub NATO: Vec<i32>,
    pub HEIGHTMAP_BEACH: i32,
    pub HEIGHTMAP_SHADOW1: i32,
    pub HEIGHTMAP_SHADOW2: i32,
    pub HEIGHTMAP_SHADOW3: i32,
    pub HEIGHTMAP_LINE1: i32,
    pub HEIGHTMAP_LINE2: i32,
    pub HEIGHTMAP_LINE3: i32,
    pub HEIGHTMAP_TRANS_SHADOW: i32,
    pub HEIGHTMAP_TRANS_LINE: i32,
    pub SE1_FLAGPANEL: i32,
    pub SE1_RESOURCEBAR_BOTTOM: i32,
    pub SE1_RESOURCEBAR_LEFT: i32,
    pub SE1_RESOURCEBAR_RIGHT: i32,
    pub SE1_RESOURCEBAR_TAB: i32,
    pub SE1_RESOURCEBAR_VARBOX: i32,
    pub SE1_ICONS: i32,
    pub SE1_ARROW1: i32,
    pub SE1_ARROW2: i32,
    pub SE1_ARROW3: i32,
    pub SE1_ARROW4: i32,
    pub SE1_ARROWBUTTON: i32,
    pub SE1_ARROWBUTTONHIGH: i32,
    pub SE1_ICONHIGHLIGHT: i32,
    pub SE1_MAINFRAME_LEFT: i32,
    pub SE1_MAINFRAME_LEFT2: i32,
    pub SE1_MAINFRAME_RIGHT: i32,
    pub SE1_MAINFRAME_RIGHT2: i32,
    pub SE1_MAINFRAME_MIDDLE: i32,
    pub SE1_ORDERBAR_TAB1LOW: i32,
    pub SE1_ORDERBAR_TAB1HIGH: i32,
    pub SE1_ORDERBAR_TAB2LOW: i32,
    pub SE1_ORDERBAR_TAB2HIGH: i32,
    pub SE1_SIDEBAR_TEXTURE: i32,
    pub SE1_SIDEBAR_TABLEFT: i32,
    pub SE1_SIDEBAR_TABRIGHT: i32,
    pub SE1_SIDEBAR_EXITLEFT: i32,
    pub SE1_SIDEBAR_EXITRIGHT: i32,
    pub SE1_BLACKGRADIENT: i32,
    pub SE1_ZONEFRAME: i32,
    pub SE1_ZONEBUTTONSMALL: i32,
    pub SE1_ZONEBUTTONSMALLHIGH: i32,
    pub SE1_ZONEBUTTON: i32,
    pub SE1_ZONEBUTTONHIGH: i32,
    pub SE1_ZONEPAPERFRAME1: i32,
    pub SE1_ZONEPAPERFRAME1air: i32,
    pub SE1_ZONEPAPERFRAME2: i32,
    pub SE1_ZONEPAPERFRAME3: i32,
    pub SE1_ZONEPAPERFRAME4: i32,
    pub SE1_QUICKHEXFRAME: i32,
    pub SE1_QUICKREGIMEFRAME: i32,
    pub SE1_BOTTOMORNAMENTALLEFT: i32,
    pub SE1_BOTTOMORNAMENTALRIGHT: i32,
    pub SE1_VARBOX2: i32,
    pub SE1_VARBOX3: i32,
    pub SE1_VARBOX4: i32,
    pub SE1_QUICKREGIMEHEADER: i32,
    pub SE1_QUICKREGIMEPAPERFRAME: i32,
    pub SE1_SIDEBAR_TOPSHADOWLEFT: i32,
    pub SE1_SIDEBAR_TOPSHADOWRIGHT: i32,
    pub SE1_UNITFRAME: i32,
    pub SE1_UNITBED: i32,
    pub SE1_UNITBEDHIGH: i32,
    pub SE1_TROOPFRAME: i32,
    pub SE1_BOTTOMPAGEBUTTON: i32,
    pub SE1_BOTTOMPAGEBUTTONHIGH: i32,
    pub SE1_VARBOX5: i32,
    pub SE1_VARBOX5HIGH: i32,
    pub SE1_QUICKUNITFRAME: i32,
    pub SE1_UNITPAPERFRAME1: i32,
    pub SE1_ASSETBACKGROUND: i32,
    pub SE1_ASSETBORDER: i32,
    pub SE1_ASSETBORDERCORNER: i32,
    pub SE1_ASSETBORDERHIGH: i32,
    pub SE1_ASSETFRAME: i32,
    pub SE1_ITEMFRAME: i32,
    pub SE1_ITEMBOX: i32,
    pub SE1_ITEMBOXCLOSED: i32,
    pub SE1_ITEMBACKGROUND: i32,
    pub SE1_ITEMBOXPROBLEM: i32,
    pub SE1_SIDEBARHEADER: i32,
    pub SE1_ORDERBUTTON: i32,
    pub SE1_ORDERBUTTONPRESSED: i32,
    pub SE1_SIDEBAR_VARBOX: i32,
    pub SE1_SIDEBAR_VARBOX_LONG: i32,
    pub SE1_PAPER1: i32,
    pub SE1_PAPER2: i32,
    pub SE1_PAPER3: i32,
    pub SE1_REGIMEFRAME: i32,
    pub SE1_CLOSEDPANEL: i32,
    pub SE1_PORTRAITBACKGROUND: i32,
    pub SE1_PORTRAITPAPER: i32,
    pub SE1_PAPERCLIP: i32,
    pub SE1_PORTRAITPAPER2: i32,
    pub SE1_PORTRAITBACKGROUNDFACTIONLEADER: i32,
    pub SE1_MULTICARD: i32,
    pub SE1_BACKGROUNDLOOP: i32,
    pub SE1_SUPERIMPOSEBACKGROUND: i32,
    pub SE1_COMPLEXFRAME: i32,
    pub SE1_COMBATBAR1: i32,
    pub SE1_COMBATBAR2: i32,
    pub SE1_COMBATBLOCK1: i32,
    pub SE1_COMBATBLOCK2: i32,
    pub SE1_COMBATBLOCK1B: i32,
    pub SE1_COMBATBLOCK2B: i32,
    pub SE1_COMBATBLOCK3: i32,
    pub SE1_COMBAT_DEAD: i32,
    pub SE1_COMBAT_SURRENDER: i32,
    pub SE1_COMBAT_RETREATED: i32,
    pub SE1_COMBAT_RETREATING: i32,
    pub seColWhite: Color,
    pub seColYellow: Color,
    pub seColRed: Color,
    pub seColGreen: Color,
    pub seColGray: Color,
    pub seColDelegated: Color,
    pub seColBrown: Color,
    pub seColBlue: Color,
    pub seColTW: Color,
    pub BUTTONSMALL: i32,
    pub BUTTONSMALLHIGH: i32,
    pub MARCLEFTBAR: i32,
    pub MARCRIGHTBAR: i32,
    pub MARCTOPBARDIGITAL: i32,
    pub UDSBUTLONG: i32,
    pub UDSBUTLONGHIGH: i32,
    pub UDSRADIO: i32,
    pub UDSRADIO2: i32,
    pub UDSRADIOHIGH: i32,
    pub UDSRADIO2HIGH: i32,
    pub UDSSMALLRADIO: i32,
    pub UDSSMALLRADIO2: i32,
    pub UDSSMALLRADIOHIGH: i32,
    pub UDSSMALLRADIO2HIGH: i32,
    pub UDSBUT2LONG: i32,
    pub UDSBUT2LONGHIGH: i32,
    pub Data: DataClass,
    pub CustomBitmapObj: CustomBitmapClass,
    pub HandyFunctionsObj: HandyFunctionsclass,
    pub TempCombat: CombatClass,
    pub ProcessingObj: ProcessingClass,
    pub AIObj: AIClass,
    pub NewAIObj: NewAIClass,
    pub DC2AIObj: DC2AIClass,
    pub EventRelatedObj: EventRelatedClass,
    pub EditObj: EditClass,
    pub ModIntroType: i32,
    pub ModNatoCounters: String,
    pub ModScenarioDir: String,
    pub ModSoundDir: String,
    pub ModBackgroundScreenName: String,
    pub ModBackgroundScreenBmp: i32,
    pub ModSystemGraphicsDirectory: String,
    pub ModOpeningSoundtrack: String,
    pub ModExtraSound: String,
    pub ModScenario: String,
    pub ModCounter: i32,
    pub ModButSize: Vec<i32>,
    pub ModButX: Vec<i32>,
    pub ModButY: Vec<i32>,
    pub ModButText: Vec<String>,
    pub ModButActive: Vec<i32>,
    pub ModButType: Vec<i32>,
    pub ModButDatastring: Vec<String>,
    pub ModButDatastring2: Vec<String>,
    pub ModGfxReplaceCounter: i32,
    pub ModGfxReplaceS1: Vec<String>,
    pub ModGfxReplaceS2: Vec<String>,
    pub CornerX: i32,
    pub CornerY: i32,
    pub SelectX: i32,
    pub SelectY: i32,
    pub MapFrame1: Bitmap,
    pub MapFrame2: Bitmap,
    pub MapFrame3: Bitmap,
    pub MapFrame4: Bitmap,
    pub LeftFrame1: Bitmap,
    pub LeftFrame2: Bitmap,
    pub LeftFrame3: Bitmap,
    pub LeftFrame4: Bitmap,
    pub GameCol1: Color,
    pub GameCol2: Color,
    pub GameCol3: Color,
    pub GameCol4: Color,
    pub GameCol5: Color,
    pub GameCol6: Color,
    pub GameCol7: Color,
    pub GameCol8: Color,
    pub GameCol9: Color,
    pub GameColPen1: Color,
    pub GameFont1: Font,
    pub GameFont2: Font,
    pub GameFont3: Font,
    pub gamefont1b: Font,
    pub gamefont2b: Font,
    pub MarcFont1: Font,
    pub MarcFont2: Font,
    pub MarcFont3: Font,
    pub MarcFont4: Font,
    pub MarcFont4b: Font,
    pub marcFont18: Font,
    pub marcFont17: Font,
    pub MarcFont5: Font,
    pub MarcFont6: Font,
    pub MarcFont16: Font,
    pub seFont1: Font,
    pub VicFont1: Font,
    pub VicFont2: Font,
    pub VicFont3: Font,
    pub VicFont4: Font,
    pub VicFont5: Font,
    pub VicFont6: Font,
    pub VicFont7: Font,
    pub VicFont8: Font,
    pub MarcFont7: Font,
    pub MarcFont8: Font,
    pub MarcFont8b: Font,
    pub MarcFont8c: Font,
    pub MarcFont9: Font,
    pub MarcFont10: Font,
    pub MarcFont11: Font,
    pub MarcFont11b: Font,
    pub MarcFont12: Font,
    pub MarcFont13: Font,
    pub MarcFont14: Font,
    pub MarcFont14b: Font,
    pub MarcFont15: Font,
    pub MarcCol1: Color,
    pub MarcCol2: Color,
    pub MarcCol3: Color,
    pub MarcCol4: Color,
    pub MarcCol5: Color,
    pub MarcCol6: Color,
    pub MarcCol7: Color,
    pub VicColor1: Color,
    pub VicColor2: Color,
    pub VicColor3: Color,
    pub VicColor6: Color,
    pub VicColor4: Color,
    pub VicColor5: Color,
    pub VicColor8: Color,
    pub viccolor7: Color,
    pub VicColor5Shade: Color,
    pub VicColor1Shade: Color,
    pub VicColor2Shade: Color,
    pub VicColor3Shade: Color,
    pub VicColor4Shade: Color,
    pub introFont1: Font,
    pub introFont2: Font,
    pub introfont2b: Font,
    pub introfont1b: Font,
    pub introfont3: Font,
    pub shadowFontConsole: Font,
    pub shadowFontConsole2: Font,
    pub shadowFontConsole3: Font,
    pub shadowFontConsole4: Font,
    pub seFont2: Font,
    pub se1TypeWriterBig: Font,
    pub se1TypeWriterBig2: Font,
    pub se1TypeWriterBig3: Font,
    pub se1TypeWriterMedium: Font,
    pub se1TypeWriterSmall: Font,
    pub FontCol: PrivateFontCollection,
    pub FontCol2: PrivateFontCollection,
    pub DynFontCol: Vec<PrivateFontCollection>,
    pub DynFont: Vec<Font>,
    pub DynFontSize: Vec<i32>,
    pub DynFontStyle: Vec<i32>,
    pub DynFontFileName: Vec<String>,
    pub DynFontWorld: Vec<bool>,
    pub DynFontCount: i32,
    pub FormRef: Form1,
    pub AIThread: Thread,
    pub AIThreadRunning: bool,
    pub AIRunning: bool,
    pub se1Thread: Thread,
    pub se1ThreadRunning: bool,
    pub se1Running: bool,
    pub se1GameLoop: GameLoopClass2 ,
    pub se1ThreadBlock: bool,
    pub ModFile: String,
    pub HelpCounter: i32,
    pub HelpDir: Vec<String>,
    pub HelpFile: Vec<String>,
    pub HelpText: Vec<String>,
}

  impl CheckDLCpresent {
    pub CheckDLCpresent: bool(dlcType: i32)
    {
      let mut dlcCounter: i32 =  this.dlc_Counter;
      for (let mut index: i32 =  0; index <= dlcCounter; index += 1)
      {
        if (this.dlc_ID[index] == dlcType)
          return true;
      }
      return false;
    }

    pub fn AddDLC(dlcType: i32)
    {
      this += 1.dlc_Counter;
      this.dlc_ID = (int[]) Utils.CopyArray((Array) this.dlc_ID, (Array) new int[this.dlc_Counter + 1]);
      this.dlc_ID[this.dlc_Counter] = dlcType;
    }

    pub fn AddDynFont(name: String, size: i32, style: i32, bool world = false) -> i32
    {
      bool flag = false;
      let mut num1: i32 =  -1;
      if (size < 1)
        size = 16;
      if (Information.IsNothing( name))
        name = "georgia.ttf";
      if (Operators.CompareString(name, "", false) == 0)
        name = "georgia.ttf";
      let mut dynFontCount: i32 =  this.DynFontCount;
      for (let mut index: i32 =  0; index <= dynFontCount; index += 1)
      {
        if (Operators.CompareString(this.DynFontFileName[index], name, false) == 0 & this.DynFontSize[index] == size & this.DynFontStyle[index] == style & this.DynFontWorld[index] == world)
        {
          flag = true;
          num1 = index;
          break;
        }
      }
      if (this.DynFontCount >= 99)
      {
        let mut num2: i32 =   Interaction.MsgBox( "Max Dynamic use: Font (99 slots) exceeded! Creation of aborted: Font.", Title: ( "Shadow Empire : Planetary Conquest"));
        return 0;
      }
      if (!flag)
      {
        this += 1.DynFontCount;
        if (Information.IsNothing( this.DynFontCol[this.DynFontCount]))
          this.DynFontCol[this.DynFontCount] = PrivateFontCollection::new();
        try
        {
          this.DynFontCol[this.DynFontCount].AddFontFile(this.AppPath + "fonts/" + name);
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.DynFontCol[this.DynFontCount].AddFontFile(this.AppPath + "fonts/georgia.ttf");
          ProjectData.ClearProjectError();
        }
        this.DynFontFileName[this.DynFontCount] = name;
        this.DynFontSize[this.DynFontCount] = size;
        this.DynFontStyle[this.DynFontCount] = style;
        this.DynFontWorld[this.DynFontCount] = world;
        num3: i32;
        try
        {
          if (world)
          {
            switch (style)
            {
              case 1:
                this.DynFont[this.DynFontCount] = Font::new(this.DynFontCol[this.DynFontCount].Families[0],  size, FontStyle.Bold, GraphicsUnit.World);
                break;
              case 2:
                this.DynFont[this.DynFontCount] = Font::new(this.DynFontCol[this.DynFontCount].Families[0],  size, FontStyle.Italic, GraphicsUnit.World);
                break;
              default:
                this.DynFont[this.DynFontCount] = Font::new(this.DynFontCol[this.DynFontCount].Families[0],  size, FontStyle.Regular, GraphicsUnit.World);
                break;
            }
          }
          else
          {
            switch (style)
            {
              case 1:
                this.DynFont[this.DynFontCount] = Font::new(this.DynFontCol[this.DynFontCount].Families[0],  size, FontStyle.Bold, GraphicsUnit.Pixel);
                break;
              case 2:
                this.DynFont[this.DynFontCount] = Font::new(this.DynFontCol[this.DynFontCount].Families[0],  size, FontStyle.Italic, GraphicsUnit.Pixel);
                break;
              default:
                this.DynFont[this.DynFontCount] = Font::new(this.DynFontCol[this.DynFontCount].Families[0],  size, FontStyle.Regular, GraphicsUnit.Pixel);
                break;
            }
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut num4: i32 =   Interaction.MsgBox( ("Error creating font: " + ex.Message), Title: ( "Shadow Empire : Planetary Conquest"));
          num3 = -1;
          ProjectData.ClearProjectError();
          goto label_31;
        }
        num1 = this.DynFontCount;
        goto label_30;
label_31:
        return num3;
      }
label_30:
      return num1;
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub GameClass(tformref: Form1)
    {
      this.UserTestMode = false;
      this.UserDebugger = false;
      this.noDrawNoFocus = false;
      this.MetricsURL = "";
      this.Metrics2URL = "";
      this.AlternativeGraphics = "";
      this.dlc_Counter = -1;
      this.dlc_ID = new int[1];
      this.modlib_Counter = -1;
      this.modlib_Name = new string[1];
      this.modlib_Designer = new string[1];
      this.modlib_Description = new string[1];
      this.modlib_Version = new int[1];
      this.modlib_Filename = new string[1];
      this.modlib_Flagged = new bool[1];
      this.theaterThreadBlock = false;
      this.AllowHeightMap = false;
      this.EmpireStyle = true;
      this.HighGraphicsSpeedPossible = true;
      this.useModLib = true;
      this.PERCENT = new int[11];
      this.BORDER = new int[6];
      this.MAPBORDER = new int[11];
      this.ZONEBORDER = new int[6];
      this.LIGHTZONEBORDER = new int[6];
      this.NOBRIDGE = new int[6];
      this.FOG = new int[65];
      this.SHROWD = new int[65];
      this.SPRITE64 = new int[2, 2, 2, 2, 2, 2];
      this.ARROW64 = new int[3, 3, 3, 3, 3, 3];
      this.SHEETX = new int[65];
      this.SHEETY = new int[65];
      this.NATO = new int[2];
      this.seColWhite = Color::new();
      this.seColYellow = Color::new();
      this.seColRed = Color::new();
      this.seColGreen = Color::new();
      this.seColGray = Color::new();
      this.seColDelegated = Color::new();
      this.seColBrown = Color::new();
      this.seColBlue = Color::new();
      this.seColTW = Color::new();
      this.ModButSize = new int[2];
      this.ModButX = new int[2];
      this.ModButY = new int[2];
      this.ModButText = new string[2];
      this.ModButActive = new int[2];
      this.ModButType = new int[2];
      this.ModButDatastring = new string[2];
      this.ModButDatastring2 = new string[2];
      this.ModGfxReplaceS1 = new string[2];
      this.ModGfxReplaceS2 = new string[2];
      this.DynFontCol = new PrivateFontCollection[100];
      this.DynFont = new Font[100];
      this.DynFontSize = new int[100];
      this.DynFontStyle = new int[100];
      this.DynFontFileName = new string[100];
      this.DynFontWorld = new bool[100];
      this.DynFontCount = -1;
      this.se1ThreadBlock = false;
      this.HelpDir = new string[1];
      this.HelpFile = new string[1];
      this.HelpText = new string[1];
      this.AppPath = AppDomain.CurrentDomain.BaseDirectory;
      this.AppPath_SAVEGAMES = this.AppPath + "savedgames\\";
      this.AppPath_SAVEGAMES = MyProject.Computer.FileSystem.SpecialDirectories.MyDocuments + "\\My Games\\Shadow Empire\\";
      if (Directory.Exists(this.AppPath_SAVEGAMES))
      {
        this.AppPath_SAVEGAMES = this.AppPath_SAVEGAMES;
        path: String = this.AppPath_SAVEGAMES + "system_test_5001234x.se1";
        FileStream Expression;
        try
        {
          Expression = new FileStream(path, FileMode.Create, FileAccess.Write, FileShare.ReadWrite);
          Expression.Close();
          Expression.Dispose();
          File.Delete(path);
        }
        catch (Exception ex1)
        {
          ProjectData.SetProjectError(ex1);
          if (!Information.IsNothing( Expression))
          {
            try
            {
              Expression.Close();
            }
            catch (Exception ex2)
            {
              ProjectData.SetProjectError(ex2);
              ProjectData.ClearProjectError();
            }
            Expression.Dispose();
          }
          this.AppPath_SAVEGAMES = this.AppPath + "savedgames\\";
          ProjectData.ClearProjectError();
        }
      }
      else
        this.AppPath_SAVEGAMES = this.AppPath + "savedgames\\";
      if (Information.IsNothing( tformref))
        return;
      str1: String;
      try
      {
        this.ModFile = DrawMod.ModFile;
        StreamReader streamReader = File.OpenText(this.AppPath + this.ModFile);
        str1 = streamReader.ReadLine();
        this.ModScenario = streamReader.ReadLine();
        this.ModIntroType = Conversions.ToInteger(streamReader.ReadLine());
        if (!(this.ModIntroType == 1 | this.ModIntroType == 2))
        {
          streamReader.Close();
          let mut num: i32 =   Interaction.MsgBox( "Faulty Mod File");
          ProjectData.EndApp();
        }
        this.ModSystemGraphicsDirectory = streamReader.ReadLine();
        this.ModScenarioDir = streamReader.ReadLine();
        this.ModSoundDir = streamReader.ReadLine();
        this.ModNatoCounters = streamReader.ReadLine();
        this.ModNatoCounters = "natocounters".to_owned();
        this.ModOpeningSoundtrack = streamReader.ReadLine();
        this.ModExtraSound = streamReader.ReadLine();
        this.ModCounter = Conversions.ToInteger(streamReader.ReadLine());
        let mut modCounter: i32 =  this.ModCounter;
        for (let mut index: i32 =  1; index <= modCounter; index += 1)
        {
          this.ModButActive = (int[]) Utils.CopyArray((Array) this.ModButActive, (Array) new int[index + 1]);
          this.ModButSize = (int[]) Utils.CopyArray((Array) this.ModButSize, (Array) new int[index + 1]);
          this.ModButX = (int[]) Utils.CopyArray((Array) this.ModButX, (Array) new int[index + 1]);
          this.ModButY = (int[]) Utils.CopyArray((Array) this.ModButY, (Array) new int[index + 1]);
          this.ModButText = (string[]) Utils.CopyArray((Array) this.ModButText, (Array) new string[index + 1]);
          this.ModButType = (int[]) Utils.CopyArray((Array) this.ModButType, (Array) new int[index + 1]);
          this.ModButDatastring = (string[]) Utils.CopyArray((Array) this.ModButDatastring, (Array) new string[index + 1]);
          this.ModButDatastring2 = (string[]) Utils.CopyArray((Array) this.ModButDatastring2, (Array) new string[index + 1]);
          strArray: Vec<String> = Strings.Split(streamReader.ReadLine(), ",");
          this.ModButActive[index] = Conversions.ToInteger(strArray[0]);
          this.ModButSize[index] = Conversions.ToInteger(strArray[1]);
          this.ModButX[index] = Conversions.ToInteger(strArray[2]);
          this.ModButY[index] = Conversions.ToInteger(strArray[3]);
          this.ModButText[index] = strArray[4];
          this.ModButType[index] = Conversions.ToInteger(strArray[5]);
          this.ModButDatastring[index] = strArray[6];
          if (strArray.GetUpperBound(0) > 6)
          {
            try
            {
              this.ModButDatastring2[index] = strArray[7];
            }
            catch (Exception ex)
            {
              ProjectData.SetProjectError(ex);
              this.ModButDatastring2[index] = "";
              ProjectData.ClearProjectError();
            }
          }
        }
        this.ModGfxReplaceCounter = 0;
        if (!streamReader.EndOfStream)
        {
          try
          {
            this.ModGfxReplaceCounter =  Math.Round(Conversion.Val(streamReader.ReadLine()));
            let mut gfxReplaceCounter: i32 =  this.ModGfxReplaceCounter;
            for (let mut index: i32 =  1; index <= gfxReplaceCounter; index += 1)
            {
              this.ModGfxReplaceS1 = (string[]) Utils.CopyArray((Array) this.ModGfxReplaceS1, (Array) new string[index + 1]);
              this.ModGfxReplaceS2 = (string[]) Utils.CopyArray((Array) this.ModGfxReplaceS2, (Array) new string[index + 1]);
              strArray: Vec<String> = Strings.Split(streamReader.ReadLine(), ",");
              this.ModGfxReplaceS1[index] = strArray[0];
              this.ModGfxReplaceS2[index] = strArray[1];
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            ProjectData.ClearProjectError();
          }
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut num: i32 =   Interaction.MsgBox( "MOD FILE CAUSED AN ERROR", Title: ( "Shadow Empire : Planetary Conquest"));
        ProjectData.EndApp();
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "editorblock.txt");
        try
        {
          this.EditorBlock = false;
          this.EditorBlock = Conversions.ToBoolean(streamReader.ReadLine());
          try
          {
            this.EditorBlockSimple = false;
            this.EditorBlockSimple = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            ProjectData.ClearProjectError();
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "metrics.txt");
        try
        {
          this.MetricsURL = streamReader.ReadLine();
          try
          {
            this.Metrics2URL = streamReader.ReadLine();
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.Metrics2URL = "";
            ProjectData.ClearProjectError();
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.MetricsURL = "";
          this.Metrics2URL = "";
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.MetricsURL = "";
        this.Metrics2URL = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "supercameron.txt");
        try
        {
          this.SuperAdminRights = Conversions.ToBoolean(streamReader.ReadLine());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.SuperAdminRights = false;
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SuperAdminRights = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "usertestmode.txt");
        try
        {
          this.UserTestMode = Conversions.ToBoolean(streamReader.ReadLine());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.UserTestMode = false;
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.UserTestMode = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "userdebugger.txt");
        try
        {
          this.UserDebugger = Conversions.ToBoolean(streamReader.ReadLine());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.UserDebugger = false;
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.UserDebugger = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "nodrawnofocus.txt");
        try
        {
          this.noDrawNoFocus = Conversions.ToBoolean(streamReader.ReadLine());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.noDrawNoFocus = false;
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.noDrawNoFocus = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "se1dlc1.txt");
        this.AddDLC(1);
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      try
      {
        StreamReader streamReader = File.OpenText(this.AppPath + "se1dlc2.txt");
        this.AddDLC(2);
        streamReader.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      this.AlternativeGraphics = "graphicsAlt".to_owned();
      this.AllowHeightMap = true;
      if (!this.SuperAdminRights)
      {
        this.EditorBlock = true;
        this.EditorBlockSimple = true;
      }
      this.HandyFunctionsObj = new HandyFunctionsclass(this);
      DrawMod.TGame = this;
      this.UseSlimDX = true;
      Application.DoEvents();
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading Fonts";
      this.GameCol1 = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      this.GameCol2 = Color.FromArgb( byte.MaxValue, 165, 165, 165);
      this.GameCol3 = Color.FromArgb( byte.MaxValue, 225, 215, 205);
      this.GameCol4 = Color.FromArgb( byte.MaxValue, 175, 165, 155);
      this.GameCol4 = Color.FromArgb( byte.MaxValue, 175, 165, 155);
      this.GameCol4 = Color.FromArgb( byte.MaxValue, 145, 135, 125);
      this.GameCol7 = Color.FromArgb( byte.MaxValue, 0, 0, 0);
      this.GameCol8 = Color.FromArgb( byte.MaxValue, 50, 100, 150);
      this.GameCol9 = Color.FromArgb( byte.MaxValue, 30, 60, 100);
      this.GameColPen1 = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      this.VicColor1 = Color.FromArgb( byte.MaxValue, 200, 200, 200);
      this.VicColor1Shade = Color.FromArgb(128, 0, 0, 0);
      this.VicColor2 = Color.White;
      this.VicColor2Shade = Color.DarkGray;
      this.VicColor3 = Color.FromArgb(200, 100, 100, 100);
      this.VicColor3Shade = Color.FromArgb(192, 0, 0, 0);
      this.VicColor4 = Color.FromArgb(128, 0, 0, 0);
      this.VicColor5 = Color.FromArgb(192, 0, 0, 0);
      this.VicColor6 = Color.FromArgb( byte.MaxValue, 150, 150, 150);
      this.viccolor7 = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 0, 0);
      this.VicColor8 = Color.FromArgb( byte.MaxValue, 0, 0, 0);
      this.MarcCol1 = Color.FromArgb(100, 0, 0, 0);
      this.MarcCol2 = Color.FromArgb(100, 120, 140, 150);
      this.MarcCol3 = Color.FromArgb( byte.MaxValue, 120, 140, 150);
      this.MarcCol4 = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 0, 0);
      this.MarcCol5 = Color.FromArgb( byte.MaxValue, 212, 70, 60);
      this.IsWin10 = Environment.OSVersion.Version.Major > 6 | Environment.OSVersion.Version.Major == 6 & Environment.OSVersion.Version.Minor >= 3;
      fullName: String = Directory.GetParent(Environment.GetFolderPath(Environment.SpecialFolder.System)).FullName;
      this.FontCol = PrivateFontCollection::new();
      this.FontCol.AddFontFile(this.AppPath + "fonts/ARIALN.TTF");
      this.FontCol.AddFontFile(this.AppPath + "fonts/ARIALNB.TTF");
      try
      {
        this.FontCol.AddFontFile(fullName + "/fonts/georgia.ttf");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.FontCol.AddFontFile(this.AppPath + "fonts/georgia.TTF");
        ProjectData.ClearProjectError();
      }
      this.FontCol.AddFontFile(this.AppPath + "fonts/small5.ttf");
      this.FontCol.AddFontFile(this.AppPath + "fonts/04B_03B_.TTF");
      try
      {
        this.FontCol.AddFontFile(fullName + "/fonts/arial.ttf");
        this.FontCol.AddFontFile(fullName + "/fonts/arialbd.ttf");
        this.FontCol.AddFontFile(fullName + "/fonts/arialbi.ttf");
        this.FontCol.AddFontFile(fullName + "/fonts/ariali.ttf");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.FontCol.AddFontFile(this.AppPath + "fonts/ARIAL.TTF");
        this.FontCol.AddFontFile(this.AppPath + "fonts/ARIALBD.TTF");
        this.FontCol.AddFontFile(this.AppPath + "fonts/ARIALBI.TTF");
        this.FontCol.AddFontFile(this.AppPath + "fonts/ARIALI.TTF");
        ProjectData.ClearProjectError();
      }
      this.FontCol.AddFontFile(this.AppPath + "fonts/bnt.TTF");
      this.FontCol.AddFontFile(this.AppPath + "fonts/bnto.TTF");
      this.MarcFont1 = Font::new(this.FontCol.Families[2], 28f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.MarcFont2 = Font::new(this.FontCol.Families[2], 28f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont3 = Font::new(this.FontCol.Families[2], 20f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont4 = Font::new(this.FontCol.Families[2], 16f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont4b = Font::new("Courier New", 15f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont5 = Font::new(this.FontCol.Families[2], 11f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.MarcFont6 = Font::new(this.FontCol.Families[4], 16f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont7 = Font::new(this.FontCol.Families[2], 14f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.MarcFont8 = Font::new(this.FontCol.Families[4], 14f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont8c = Font::new(this.FontCol.Families[4], 13f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont9 = Font::new(this.FontCol.Families[2], 8f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont10 = Font::new(this.FontCol.Families[2], 10f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont11 = Font::new(this.FontCol.Families[2], 9f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont11b = Font::new(this.FontCol.Families[2], 11f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.MarcFont12 = Font::new(this.FontCol.Families[4], 28f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont13 = Font::new(this.FontCol.Families[4], 11f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont14 = Font::new(this.FontCol.Families[5], 9f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont14b = Font::new(this.FontCol.Families[5], 14f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont15 = Font::new(this.FontCol.Families[5], 8f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.MarcFont16 = Font::new(this.FontCol.Families[2], 16f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.MarcFont8b = Font::new(this.FontCol.Families[4], 14f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.marcFont17 = Font::new(this.FontCol.Families[2], 14f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.marcFont18 = Font::new(this.FontCol.Families[2], 12f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.introFont1 = Font::new(this.FontCol.Families[3], 36f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.introFont2 = Font::new(this.FontCol.Families[3], 48f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.GameFont1 = this.MarcFont4;
      this.gamefont1b = this.MarcFont4;
      this.GameFont2 = this.MarcFont4;
      this.gamefont2b = this.MarcFont4;
      this.GameFont3 = this.MarcFont4;
      this.FontCol.AddFontFile(this.AppPath + "fonts/LMmono-Regular.ttf");
      this.FontCol.AddFontFile(this.AppPath + "fonts/LMmonoCaps-Regular.ttf");
      this.shadowFontConsole = Font::new(this.FontCol.Families[6], 13f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.shadowFontConsole2 = Font::new(this.FontCol.Families[6], 20f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.shadowFontConsole3 = Font::new(this.FontCol.Families[6], 10f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.shadowFontConsole4 = Font::new(this.FontCol.Families[6], 8f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.se1TypeWriterBig = Font::new(this.FontCol.Families[7], 20f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.se1TypeWriterBig2 = Font::new(this.FontCol.Families[7], 16f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.se1TypeWriterBig3 = Font::new(this.FontCol.Families[7], 14f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.se1TypeWriterMedium = Font::new(this.FontCol.Families[6], 15f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.se1TypeWriterSmall = Font::new(this.FontCol.Families[6], 13f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.seFont1 = Font::new(this.FontCol.Families[7], 32f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.seFont2 = Font::new(this.FontCol.Families[7], 44f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.VicFont1 = this.MarcFont16;
      this.VicFont2 = this.MarcFont4;
      this.VicFont3 = this.MarcFont8c;
      this.VicFont4 = this.MarcFont10;
      this.VicFont5 = this.MarcFont10;
      this.VicFont6 = Font::new(this.FontCol.Families[3], 8f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.VicFont7 = this.MarcFont8;
      this.VicFont8 = this.MarcFont12;
      this.GameFont1 = this.MarcFont4;
      this.gamefont1b = this.MarcFont4;
      this.GameFont2 = this.MarcFont4;
      this.gamefont2b = this.MarcFont4;
      this.GameFont3 = this.MarcFont4;
      BitmapStore.GiveGraphicsPath(this.AppPath + "graphics/");
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading System gfx";
      Application.DoEvents();
      this.REDBUTTON = BitmapStore.AddFile("systemgraphics/wide red normal.png", true);
      this.RESEARCHOVERPRINT = BitmapStore.AddFile("systemgraphics/cat1/cardpanel.png", true);
      this.ZONEBORDER[0] = BitmapStore.AddFile("systemgraphics/border/ZoneBorder1.png", true, true);
      this.ZONEBORDER[1] = BitmapStore.AddFile("systemgraphics/border/ZoneBorder2.png", true, true);
      this.ZONEBORDER[2] = BitmapStore.AddFile("systemgraphics/border/ZoneBorder3.png", true, true);
      this.ZONEBORDER[3] = BitmapStore.AddFile("systemgraphics/border/ZoneBorder4.png", true, true);
      this.ZONEBORDER[4] = BitmapStore.AddFile("systemgraphics/border/ZoneBorder5.png", true, true);
      this.ZONEBORDER[5] = BitmapStore.AddFile("systemgraphics/border/ZoneBorder6.png", true, true);
      this.LIGHTZONEBORDER[0] = BitmapStore.AddFile("systemgraphics/border/LightZoneBorder1.png", true, true);
      this.LIGHTZONEBORDER[1] = BitmapStore.AddFile("systemgraphics/border/LightZoneBorder2.png", true, true);
      this.LIGHTZONEBORDER[2] = BitmapStore.AddFile("systemgraphics/border/LightZoneBorder3.png", true, true);
      this.LIGHTZONEBORDER[3] = BitmapStore.AddFile("systemgraphics/border/LightZoneBorder4.png", true, true);
      this.LIGHTZONEBORDER[4] = BitmapStore.AddFile("systemgraphics/border/LightZoneBorder5.png", true, true);
      this.LIGHTZONEBORDER[5] = BitmapStore.AddFile("systemgraphics/border/LightZoneBorder6.png", true, true);
      this.WHITEHEXTRANS = BitmapStore.AddFile("systemgraphics/whitehextrans.png", true, true);
      this.WHITEHEXTRANS2 = BitmapStore.AddFile("systemgraphics/whitehextrans2.png", true, true);
      this.WHITEHEXTRANS3 = BitmapStore.AddFile("systemgraphics/whitehextrans3.png", true, true);
      this.LINES = BitmapStore.AddFile("systemgraphics/cat1/lines.png", true);
      this.LINESFRAME = BitmapStore.AddFile("systemgraphics/cat1/linesframe.png", true);
      this.MARCBUTBAR = BitmapStore.AddFile("systemgraphics/cat1/ButBarSe1.png", true);
      this.MARCBUTBARHISTORY = BitmapStore.AddFile("systemgraphics/cat1/ButBar.png", true);
      this.UP20 = BitmapStore.AddFile("systemgraphics/up20.png", true);
      this.DOWN20 = BitmapStore.AddFile("systemgraphics/down20.png", true);
      this.SECARDOUTLINE = BitmapStore.AddFile("systemgraphics/cat1/CardMediumOutline.png", true);
      this.ARROWSHEET = BitmapStore.AddFile("systemgraphics/cat4/arrows.png", true, true);
      this.BACKGROUND1MARC2 = BitmapStore.AddFile("systemgraphics/se1/backgroundmarc2.png", true);
      this.BUTTONSMALL = BitmapStore.AddFile("systemgraphics/cat1/Button Small.png", true);
      this.BUTTONSMALLHIGH = BitmapStore.AddFile("systemgraphics/cat1/Button Small Over.png", true);
      this.MARCLEFTBAR = BitmapStore.AddFile("systemgraphics/cat1/sidebarleft.png", true);
      this.MARCTOPBARDIGITAL = BitmapStore.AddFile("systemgraphics/cat1/topbardigital.png", true);
      this.MARCRIGHTBAR = BitmapStore.AddFile("systemgraphics/cat1/sidebarright.png", true);
      this.SE1_SUPERIMPOSEBACKGROUND = BitmapStore.AddFile("systemgraphics/se1/superimposebackground.png", true);
      this.SE1_COMPLEXFRAME = BitmapStore.AddFile("systemgraphics/se1/complexframe.png", true);
      this.SE1_BACKGROUNDLOOP = BitmapStore.AddFile("systemgraphics/se1/backgroundloop.png", true);
      this.SE1_RESOURCEBAR_BOTTOM = BitmapStore.AddFile("systemgraphics/se1/resourcebar_bottom.png", true);
      this.SE1_RESOURCEBAR_LEFT = BitmapStore.AddFile("systemgraphics/se1/resourcebar_left.png", true);
      this.SE1_RESOURCEBAR_RIGHT = BitmapStore.AddFile("systemgraphics/se1/resourcebar_right.png", true);
      this.SE1_RESOURCEBAR_TAB = BitmapStore.AddFile("systemgraphics/se1/resourcebar_tab.png", true);
      this.SE1_RESOURCEBAR_VARBOX = BitmapStore.AddFile("systemgraphics/se1/resourcebar_varbox.png", true);
      this.SE1_ICONS = BitmapStore.AddFile("systemgraphics/se1/icons.png", true);
      this.SE1_ARROW1 = BitmapStore.AddFile("systemgraphics/se1/arrow1.png", true);
      this.SE1_ARROW2 = BitmapStore.AddFile("systemgraphics/se1/arrow2.png", true);
      this.SE1_ARROW3 = BitmapStore.AddFile("systemgraphics/se1/arrow3.png", true);
      this.SE1_ARROW4 = BitmapStore.AddFile("systemgraphics/se1/arrow4.png", true);
      this.SE1_ARROWBUTTON = BitmapStore.AddFile("systemgraphics/se1/arrowbutton.png", true);
      this.SE1_ARROWBUTTONHIGH = BitmapStore.AddFile("systemgraphics/se1/arrowbuttonhigh.png", true);
      this.SE1_ICONHIGHLIGHT = BitmapStore.AddFile("systemgraphics/se1/iconhighlight.png", true);
      this.SE1_MAINFRAME_LEFT = BitmapStore.AddFile("systemgraphics/se1/mainframe_left.png", true);
      this.SE1_MAINFRAME_LEFT2 = BitmapStore.AddFile("systemgraphics/se1/mainframe_left2.png", true);
      this.SE1_MAINFRAME_RIGHT = BitmapStore.AddFile("systemgraphics/se1/mainframe_right.png", true);
      this.SE1_MAINFRAME_RIGHT2 = BitmapStore.AddFile("systemgraphics/se1/mainframe_right2.png", true);
      this.SE1_MAINFRAME_MIDDLE = BitmapStore.AddFile("systemgraphics/se1/mainframe_middle.png", true);
      this.SE1_ORDERBAR_TAB1LOW = BitmapStore.AddFile("systemgraphics/se1/orderbar_tab1low.png", true);
      this.SE1_ORDERBAR_TAB1HIGH = BitmapStore.AddFile("systemgraphics/se1/orderbar_tab1high.png", true);
      this.SE1_ORDERBAR_TAB2LOW = BitmapStore.AddFile("systemgraphics/se1/orderbar_tab2low.png", true);
      this.SE1_ORDERBAR_TAB2HIGH = BitmapStore.AddFile("systemgraphics/se1/orderbar_tab2high.png", true);
      this.SE1_SIDEBAR_TEXTURE = BitmapStore.AddFile("systemgraphics/se1/sidebar_texture.png", true);
      this.SE1_SIDEBAR_TABLEFT = BitmapStore.AddFile("systemgraphics/se1/sidebar_tableft.png", true);
      this.SE1_SIDEBAR_TABRIGHT = BitmapStore.AddFile("systemgraphics/se1/sidebar_tabright.png", true);
      this.SE1_SIDEBAR_EXITLEFT = BitmapStore.AddFile("systemgraphics/se1/sidebar_exitleft.png", true);
      this.SE1_SIDEBAR_EXITRIGHT = BitmapStore.AddFile("systemgraphics/se1/sidebar_exitright.png", true);
      this.SE1_BLACKGRADIENT = BitmapStore.AddFile("systemgraphics/se1/blackgradient.png", true);
      this.SE1_ZONEFRAME = BitmapStore.AddFile("systemgraphics/se1/zoneframe.png", true);
      this.SE1_ZONEBUTTON = BitmapStore.AddFile("systemgraphics/se1/zonebutton.png", true);
      this.SE1_ZONEBUTTONHIGH = BitmapStore.AddFile("systemgraphics/se1/zonebuttonhigh.png", true);
      this.SE1_ZONEBUTTONSMALL = BitmapStore.AddFile("systemgraphics/se1/zonebuttonsmall.png", true);
      this.SE1_ZONEBUTTONSMALLHIGH = BitmapStore.AddFile("systemgraphics/se1/zonebuttonsmallhigh.png", true);
      this.SE1_ZONEPAPERFRAME1air = BitmapStore.AddFile("systemgraphics/se1/zonepaperframe1air.png", true);
      this.SE1_ZONEPAPERFRAME1 = BitmapStore.AddFile("systemgraphics/se1/zonepaperframe1.png", true);
      this.SE1_ZONEPAPERFRAME2 = BitmapStore.AddFile("systemgraphics/se1/zonepaperframe2.png", true);
      this.SE1_ZONEPAPERFRAME3 = BitmapStore.AddFile("systemgraphics/se1/zonepaperframe3.png", true);
      this.SE1_ZONEPAPERFRAME4 = BitmapStore.AddFile("systemgraphics/se1/zonepaperframe4.png", true);
      this.SE1_QUICKHEXFRAME = BitmapStore.AddFile("systemgraphics/se1/quickhexframe.png", true);
      this.SE1_QUICKREGIMEFRAME = BitmapStore.AddFile("systemgraphics/se1/quickregimeframe.png", true);
      this.SE1_BOTTOMORNAMENTALLEFT = BitmapStore.AddFile("systemgraphics/se1/bottomornamentalleft.png", true);
      this.SE1_BOTTOMORNAMENTALRIGHT = BitmapStore.AddFile("systemgraphics/se1/bottomornamentalright.png", true);
      this.SE1_VARBOX2 = BitmapStore.AddFile("systemgraphics/se1/varbox2.png", true);
      this.SE1_VARBOX3 = BitmapStore.AddFile("systemgraphics/se1/varbox3.png", true);
      this.SE1_VARBOX4 = BitmapStore.AddFile("systemgraphics/se1/varbox4.png", true);
      this.SE1_QUICKREGIMEHEADER = BitmapStore.AddFile("systemgraphics/se1/quickregimeheader.png", true);
      this.SE1_QUICKREGIMEPAPERFRAME = BitmapStore.AddFile("systemgraphics/se1/quickregimepaperframe.png", true);
      this.SE1_SIDEBAR_TOPSHADOWLEFT = BitmapStore.AddFile("systemgraphics/se1/sidebar_topshadowleft.png", true);
      this.SE1_SIDEBAR_TOPSHADOWRIGHT = BitmapStore.AddFile("systemgraphics/se1/sidebar_topshadowright.png", true);
      this.SE1_UNITFRAME = BitmapStore.AddFile("systemgraphics/se1/unitframe.png", true);
      this.SE1_UNITBED = BitmapStore.AddFile("systemgraphics/se1/unitbed.png", true);
      this.SE1_UNITBEDHIGH = BitmapStore.AddFile("systemgraphics/se1/unitbedhigh.png", true);
      this.SE1_TROOPFRAME = BitmapStore.AddFile("systemgraphics/se1/troopframe.png", true);
      this.SE1_BOTTOMPAGEBUTTON = BitmapStore.AddFile("systemgraphics/se1/bottompagebutton.png", true);
      this.SE1_BOTTOMPAGEBUTTONHIGH = BitmapStore.AddFile("systemgraphics/se1/bottompagebuttonhigh.png", true);
      this.SE1_VARBOX5 = BitmapStore.AddFile("systemgraphics/se1/varbox5.png", true);
      this.SE1_VARBOX5HIGH = BitmapStore.AddFile("systemgraphics/se1/varbox5high.png", true);
      this.SE1_QUICKUNITFRAME = BitmapStore.AddFile("systemgraphics/se1/quickunitframe.png", true);
      this.SE1_UNITPAPERFRAME1 = BitmapStore.AddFile("systemgraphics/se1/unitpaperframe1.png", true);
      this.SE1_ASSETBACKGROUND = BitmapStore.AddFile("systemgraphics/se1/assetbackground.png", true);
      this.SE1_ASSETBORDER = BitmapStore.AddFile("systemgraphics/se1/assetborder.png", true);
      this.SE1_ASSETBORDERCORNER = BitmapStore.AddFile("systemgraphics/se1/assetbordercorner.png", true);
      this.SE1_ASSETBORDERHIGH = BitmapStore.AddFile("systemgraphics/se1/assetborderhigh.png", true);
      this.SE1_ASSETFRAME = BitmapStore.AddFile("systemgraphics/se1/assetframe.png", true);
      this.SE1_ITEMFRAME = BitmapStore.AddFile("systemgraphics/se1/itemframe.png", true);
      this.SE1_ITEMBOX = BitmapStore.AddFile("systemgraphics/se1/itembox.png", true);
      this.SE1_ITEMBOXPROBLEM = BitmapStore.AddFile("systemgraphics/se1/itemboxproblem.png", true);
      this.SE1_ITEMBOXCLOSED = BitmapStore.AddFile("systemgraphics/se1/itemboxclosed.png", true);
      this.SE1_ITEMBACKGROUND = BitmapStore.AddFile("systemgraphics/se1/itembackground.png", true);
      this.SE1_SIDEBARHEADER = BitmapStore.AddFile("systemgraphics/se1/sidebarheader.png", true);
      this.SE1_ORDERBUTTON = BitmapStore.AddFile("systemgraphics/se1/orderbutton.png", true);
      this.SE1_ORDERBUTTONPRESSED = BitmapStore.AddFile("systemgraphics/se1/orderbuttonpressed.png", true);
      this.SE1_SIDEBAR_VARBOX = BitmapStore.AddFile("systemgraphics/se1/sidebar_varbox.png", true);
      this.SE1_SIDEBAR_VARBOX_LONG = BitmapStore.AddFile("systemgraphics/se1/sidebar_varbox_long.png", true);
      this.SE1_PAPER1 = BitmapStore.AddFile("systemgraphics/se1/paper1.png", true);
      this.SE1_PAPER2 = BitmapStore.AddFile("systemgraphics/se1/paper2.png", true);
      this.SE1_PAPER3 = BitmapStore.AddFile("systemgraphics/se1/paper3.png", true);
      this.SE1_REGIMEFRAME = BitmapStore.AddFile("systemgraphics/se1/regimeframe.png", true);
      this.SE1_CLOSEDPANEL = BitmapStore.AddFile("systemgraphics/se1/closedpanel.png", true);
      this.SE1_PORTRAITBACKGROUND = BitmapStore.AddFile("systemgraphics/se1/portraitbackground.png", true);
      this.SE1_PORTRAITPAPER = BitmapStore.AddFile("systemgraphics/se1/portraitpaper.png", true);
      this.SE1_PORTRAITPAPER2 = BitmapStore.AddFile("systemgraphics/se1/portraitpaper2.png", true);
      this.SE1_PAPERCLIP = BitmapStore.AddFile("systemgraphics/se1/paperclip.png", true);
      this.SE1_PORTRAITBACKGROUNDFACTIONLEADER = BitmapStore.AddFile("systemgraphics/se1/portraitbackgroundfactionleader.png", true);
      this.SE1_MULTICARD = BitmapStore.AddFile("systemgraphics/se1/multicard.png", true);
      this.SE1_COMBATBAR1 = BitmapStore.AddFile("systemgraphics/se1/combatbar1.png", true);
      this.SE1_COMBATBAR2 = BitmapStore.AddFile("systemgraphics/se1/combatbar2.png", true);
      this.SE1_COMBATBLOCK1 = BitmapStore.AddFile("systemgraphics/se1/combatblock1.png", true, true);
      this.SE1_COMBATBLOCK2 = BitmapStore.AddFile("systemgraphics/se1/combatblock2.png", true, true);
      this.SE1_COMBATBLOCK3 = BitmapStore.AddFile("systemgraphics/se1/combatblock3.png", true, true);
      this.SE1_COMBATBLOCK1B = BitmapStore.AddFile("systemgraphics/se1/combatblock1b.png", true, true);
      this.SE1_COMBATBLOCK2B = BitmapStore.AddFile("systemgraphics/se1/combatblock2b.png", true, true);
      this.SE1_COMBAT_DEAD = BitmapStore.AddFile("systemgraphics/se1/combat_dead.png", true, true);
      this.SE1_COMBAT_SURRENDER = BitmapStore.AddFile("systemgraphics/se1/combat_surrender.png", true, true);
      this.SE1_COMBAT_RETREATED = BitmapStore.AddFile("systemgraphics/se1/combat_retreated.png", true, true);
      this.SE1_COMBAT_RETREATING = BitmapStore.AddFile("systemgraphics/se1/combat_retreating.png", true, true);
      this.SE1_FLAGPANEL = BitmapStore.AddFile("systemgraphics/se1/flagpanel.png", true);
      this.seColWhite = Color.White;
      this.seColGray = Color.FromArgb( byte.MaxValue, 225, 225, 225);
      this.seColRed = Color.FromArgb( byte.MaxValue, 225, 0, 0);
      this.seColGreen = Color.FromArgb( byte.MaxValue, 0, 225, 0);
      this.seColBlue = Color.FromArgb( byte.MaxValue, 0, 225, 225);
      this.seColYellow = Color.FromArgb( byte.MaxValue, 225, 225, 0);
      this.seColBrown = Color.FromArgb( byte.MaxValue, 235, 185, 135);
      this.seColDelegated = Color.FromArgb( byte.MaxValue, 165, 0, 165);
      this.seColTW = Color.Black;
      try
      {
        this.UDSBUTLONG = BitmapStore.AddFile("systemgraphics/cat1/ButtonUds.png", true);
        this.UDSBUTLONGHIGH = BitmapStore.AddFile("systemgraphics/cat1/ButtonUdsOver.png", true);
        this.UDSBUT2LONG = BitmapStore.AddFile("systemgraphics/cat1/ButtonUds2.png", true);
        this.UDSBUT2LONGHIGH = BitmapStore.AddFile("systemgraphics/cat1/ButtonUds2Over.png", true);
        this.UDSRADIO = BitmapStore.AddFile("systemgraphics/cat1/RadioUds.png", true);
        this.UDSRADIOHIGH = BitmapStore.AddFile("systemgraphics/cat1/RadioUdsOver.png", true);
        this.UDSRADIO2 = BitmapStore.AddFile("systemgraphics/cat1/RadioUds2.png", true);
        this.UDSRADIO2HIGH = BitmapStore.AddFile("systemgraphics/cat1/RadioUds2Over.png", true);
        this.UDSSMALLRADIO = BitmapStore.AddFile("systemgraphics/cat1/RadioSmallUds.png", true);
        this.UDSSMALLRADIOHIGH = BitmapStore.AddFile("systemgraphics/cat1/RadioSmallUdsOver.png", true);
        this.UDSSMALLRADIO2 = BitmapStore.AddFile("systemgraphics/cat1/RadioSmallUds2.png", true);
        this.UDSSMALLRADIO2HIGH = BitmapStore.AddFile("systemgraphics/cat1/RadioSmallUds2Over.png", true);
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.UDSBUTLONG = 0;
        this.UDSBUTLONGHIGH = 0;
        this.UDSRADIO = 0;
        this.UDSRADIOHIGH = 0;
        this.UDSRADIO2 = 0;
        this.UDSRADIO2HIGH = 0;
        this.UDSBUT2LONG = 0;
        this.UDSBUT2LONGHIGH = 0;
        ProjectData.ClearProjectError();
      }
      this.REDOVAL = BitmapStore.AddFile("systemgraphics/redoval.bmp", true);
      this.BLUEOVAL = BitmapStore.AddFile("systemgraphics/blueoval.bmp", true);
      this.BROWNOVAL = BitmapStore.AddFile("systemgraphics/brownoval.bmp", true);
      this.SYSTEM1 = BitmapStore.AddFile("systemgraphics/Minimize Button Normal.png", true);
      this.SYSTEM1B = BitmapStore.AddFile("systemgraphics/Minimize Button Mouse Over.png", true);
      this.SYSTEM2 = BitmapStore.AddFile("systemgraphics/Quit Button Normal.png", true);
      this.SYSTEM2B = BitmapStore.AddFile("systemgraphics/Quit Button Mouse Over.png", true);
      this.NONEBUTTON = BitmapStore.AddFile("systemgraphics/buttonnone.png", true);
      this.ALLBUTTON = BitmapStore.AddFile("systemgraphics/buttonall.png", true);
      this.BUTTONHEXINFO = BitmapStore.AddFile("systemgraphics/buttonhexinfo.png", true);
      this.BUTTONHEXINFO2 = BitmapStore.AddFile("systemgraphics/buttonhexinfo2.png", true);
      this.NOBRIDGE[0] = BitmapStore.AddFile("systemgraphics/nobridge/01.png", true, true);
      this.NOBRIDGE[1] = BitmapStore.AddFile("systemgraphics/nobridge/02.png", true, true);
      this.NOBRIDGE[2] = BitmapStore.AddFile("systemgraphics/nobridge/03.png", true, true);
      this.NOBRIDGE[3] = BitmapStore.AddFile("systemgraphics/nobridge/04.png", true, true);
      this.NOBRIDGE[4] = BitmapStore.AddFile("systemgraphics/nobridge/05.png", true, true);
      this.NOBRIDGE[5] = BitmapStore.AddFile("systemgraphics/nobridge/06.png", true, true);
      this.SHADEDHEX = BitmapStore.AddFile("systemgraphics/shadedhex.png", true, true);
      this.BUTTONAIRRECON = BitmapStore.AddFile("systemgraphics/buttonairrecon.png", true);
      this.BACKBUTTON = BitmapStore.AddFile("systemgraphics/buttonback.png", true);
      this.BLACKOVAL = BitmapStore.AddFile("systemgraphics/blackoval.bmp", true);
      this.BLACKBOX = BitmapStore.AddFile("systemgraphics/blackbox.bmp", true);
      this.BUTTONSUPPLYON = BitmapStore.AddFile("systemgraphics/buttonsupplyon.png", true);
      this.BUTTONGIVEUNIT = BitmapStore.AddFile("systemgraphics/buttongiveunit.png", true);
      this.BUTTONMOVE2 = BitmapStore.AddFile("systemgraphics/buttonmove2.png", true);
      this.BUTTONSTRATEGIC2 = BitmapStore.AddFile("systemgraphics/buttonstrategic2.png", true);
      this.BLACKHEX = BitmapStore.AddFile("systemgraphics/blackhex.bmp", true, true);
      this.BORDER[0] = BitmapStore.AddFile("systemgraphics/border/border1.png", true, true);
      this.BORDER[1] = BitmapStore.AddFile("systemgraphics/border/border2.png", true, true);
      this.BORDER[2] = BitmapStore.AddFile("systemgraphics/border/border3.png", true, true);
      this.BORDER[3] = BitmapStore.AddFile("systemgraphics/border/border4.png", true, true);
      this.BORDER[4] = BitmapStore.AddFile("systemgraphics/border/border5.png", true, true);
      this.BORDER[5] = BitmapStore.AddFile("systemgraphics/border/border6.png", true, true);
      this.MAPBORDER[0] = BitmapStore.AddFile("systemgraphics/border/mapborder1.png", true, true);
      this.MAPBORDER[1] = BitmapStore.AddFile("systemgraphics/border/mapborder2.png", true, true);
      this.MAPBORDER[2] = BitmapStore.AddFile("systemgraphics/border/mapborder3.png", true, true);
      this.MAPBORDER[3] = BitmapStore.AddFile("systemgraphics/border/mapborder4.png", true, true);
      this.MAPBORDER[4] = BitmapStore.AddFile("systemgraphics/border/mapborder5.png", true, true);
      this.MAPBORDER[5] = BitmapStore.AddFile("systemgraphics/border/mapborder6.png", true, true);
      this.MAPBORDER[6] = BitmapStore.AddFile("systemgraphics/border/mapborder7.png", true, true);
      this.MAPBORDER[7] = BitmapStore.AddFile("systemgraphics/border/mapborder8.png", true, true);
      this.MAPBORDER[8] = BitmapStore.AddFile("systemgraphics/border/mapborder9.png", true, true);
      this.MAPBORDER[9] = BitmapStore.AddFile("systemgraphics/border/mapborder10.png", true, true);
      this.MAPBORDER[10] = BitmapStore.AddFile("systemgraphics/border/mapborder11.png", true, true);
      this.BUTTONSTRATEGIC = BitmapStore.AddFile("systemgraphics/buttonstrategic.png", true);
      this.BUTTONLEFT = BitmapStore.AddFile("systemgraphics/buttonleft normal.png", true);
      this.BUTTONRIGHT = BitmapStore.AddFile("systemgraphics/buttonright normal.png", true);
      this.BUTTONLEFT2 = BitmapStore.AddFile("systemgraphics/buttonleft2 normal.png", true);
      this.BUTTONRIGHT2 = BitmapStore.AddFile("systemgraphics/buttonright2 normal.png", true);
      this.BUTTONLEFTB = BitmapStore.AddFile("systemgraphics/buttonleft mouseover.png", true);
      this.BUTTONRIGHTB = BitmapStore.AddFile("systemgraphics/buttonright mouseover.png", true);
      this.BUTTONLEFT2B = BitmapStore.AddFile("systemgraphics/buttonleft2 mouseover.png", true);
      this.BUTTONRIGHT2B = BitmapStore.AddFile("systemgraphics/buttonright2 mouseover.png", true);
      this.VSLIDER = BitmapStore.AddFile("systemgraphics/vslider normal.png", true);
      this.VSLIDERB = BitmapStore.AddFile("systemgraphics/vslider mouse over.png", true);
      this.TARGETHEX = BitmapStore.AddFile("systemgraphics/target.png", true, true);
      this.BUTTONHQUNIT = BitmapStore.AddFile("systemgraphics/buttonhqunit.png", true);
      this.BUTTONATTACK = BitmapStore.AddFile("systemgraphics/buttonattack.png", true);
      this.ROUNDBALL = BitmapStore.AddFile("systemgraphics/roundball.bmp", true, true);
      this.BUTTONAIRATTACK = BitmapStore.AddFile("systemgraphics/buttonairattack.png", true);
      this.BUTTONARTATTACK = BitmapStore.AddFile("systemgraphics/buttonartattack.png", true);
      this.BUTTONMOVE = BitmapStore.AddFile("systemgraphics/buttonmove.png", true);
      this.BUTTONNEXT = BitmapStore.AddFile("systemgraphics/buttonnext.png", true);
      this.WHITEHEX = BitmapStore.AddFile("systemgraphics/whitehex.bmp", true, true);
      this.NOHEX = BitmapStore.AddFile("systemgraphics/nohex.bmp", true, true);
      this.SELECTEDHEX = BitmapStore.AddFile("systemgraphics/selected hex.png", true, true);
      this.LISTUP = BitmapStore.AddFile("systemgraphics/listup.png", true);
      this.LISTBLOCK = BitmapStore.AddFile("systemgraphics/ListBlock.png", true);
      this.LISTDOWN = BitmapStore.AddFile("systemgraphics/listdown.png", true);
      this.LISTBACK = BitmapStore.AddFile("systemgraphics/ListBackground.png", true);
      this.OKBALL = BitmapStore.AddFile("systemgraphics/buttonokball.png", true);
      this.CANCELBALL = BitmapStore.AddFile("systemgraphics/buttoncancelball.png", true);
      this.SUPPLIESSYMBOL = BitmapStore.AddFile("systemgraphics/Supply Icon.png", true);
      this.BUTTONSAVE = BitmapStore.AddFile("systemgraphics/buttonsave.png", true);
      this.ATTACK0 = BitmapStore.AddFile("systemgraphics/attack 0.png", true, true);
      this.ATTACK1 = BitmapStore.AddFile("systemgraphics/attack 1.png", true, true);
      this.ATTACK2 = BitmapStore.AddFile("systemgraphics/attack 2.png", true, true);
      this.ATTACK3 = BitmapStore.AddFile("systemgraphics/attack 3.png", true, true);
      this.ATTACK4 = BitmapStore.AddFile("systemgraphics/attack 4.png", true, true);
      this.ATTACK5 = BitmapStore.AddFile("systemgraphics/attack 5.png", true, true);
      this.ATTACKART = BitmapStore.AddFile("systemgraphics/attack art.png", true);
      this.ATTACKAMPH = BitmapStore.AddFile("systemgraphics/attack amphib.png", true);
      this.ATTACKPARADROP = BitmapStore.AddFile("systemgraphics/attack para.png", true);
      this.ATTACKAIR = BitmapStore.AddFile("systemgraphics/attack air.png", true, true);
      this.BUTTONHISTORY = BitmapStore.AddFile("systemgraphics/buttonhistory.png", true);
      this.BUTTONBUILDROAD = BitmapStore.AddFile("systemgraphics/buttonbuildroad.png", true);
      this.BUTTONBLOWBRIDGE = BitmapStore.AddFile("systemgraphics/buttonblowbridge.png", true);
      this.BUTTONHEX = BitmapStore.AddFile("systemgraphics/buttonhex.png", true);
      this.BUTTONHEXUNIT = BitmapStore.AddFile("systemgraphics/buttonhexunit.png", true);
      this.BUTTONHEXUNIT2 = BitmapStore.AddFile("systemgraphics/buttonhexunit2.png", true);
      this.HEXRASTER = BitmapStore.AddFile("systemgraphics/hexraster.png", true, true);
      this.DEFAULTCOUNTER = BitmapStore.AddFile("systemgraphics/defaultcounter.png", true);
      this.DEFAULTCOUNTERBIG = BitmapStore.AddFile("systemgraphics/defaultcounterbig.png", true);
      this.DEFAULTCOUNTERSMALL = BitmapStore.AddFile("systemgraphics/defaultcountersmall.png", true);
      this.MGSPLASH = BitmapStore.AddFile("systemgraphics/MG-splash.png", true);
      this.SLITHERINE = BitmapStore.AddFile("systemgraphics/slitherine.png", true);
      this.VRSPLASH = BitmapStore.AddFile("systemgraphics/VR-splash.png", true);
      this.TUTARROW = BitmapStore.AddFile("systemgraphics/TutArrow.png", true);
      this.TUTHEX = BitmapStore.AddFile("systemgraphics/TutHex.png", true);
      this.BUTTONZOOMIN = BitmapStore.AddFile("systemgraphics/ButtonZoomIn.png", true);
      this.BUTTONZOOMOUT = BitmapStore.AddFile("systemgraphics/ButtonZoomOut.png", true);
      this.BUTTONSTACKEDUNIT = BitmapStore.AddFile("systemgraphics/ButtonStackedUnit.png", true);
      this.BUTTONSPREADUNIT = BitmapStore.AddFile("systemgraphics/ButtonSpreadUnit.png", true);
      if (this.AllowHeightMap)
      {
        this.HEIGHTMAP_BEACH = BitmapStore.AddFile("systemgraphics/Heights/beachShadow.png", true, true);
        this.HEIGHTMAP_SHADOW1 = BitmapStore.AddFile("systemgraphics/Heights/shadowLevel1.png", true, true);
        this.HEIGHTMAP_SHADOW2 = BitmapStore.AddFile("systemgraphics/Heights/shadowLevel2.png", true, true);
        this.HEIGHTMAP_SHADOW3 = BitmapStore.AddFile("systemgraphics/Heights/shadowLevel3.png", true, true);
        this.HEIGHTMAP_LINE1 = BitmapStore.AddFile("systemgraphics/Heights/lineLevel1.png", true, true);
        this.HEIGHTMAP_LINE2 = BitmapStore.AddFile("systemgraphics/Heights/lineLevel2.png", true, true);
        this.HEIGHTMAP_LINE3 = BitmapStore.AddFile("systemgraphics/Heights/lineLevel3.png", true, true);
        this.HEIGHTMAP_TRANS_SHADOW = BitmapStore.AddFile("systemgraphics/Heights/shadowTransitions.png", true, true);
        this.HEIGHTMAP_TRANS_LINE = BitmapStore.AddFile("systemgraphics/Heights/lineTransitions.png", true, true);
      }
      this.ATTACKART2 = BitmapStore.AddFile("systemgraphics/attackart.png", true, true);
      this.SUPPLYBLOCK = BitmapStore.AddFile("systemgraphics/blocksupply.png", true, true);
      this.TRAFFIC1 = BitmapStore.AddFile("systemgraphics/traffic1.png", true, true);
      this.TRAFFIC2 = BitmapStore.AddFile("systemgraphics/traffic2.png", true, true);
      this.TRAFFIC3 = BitmapStore.AddFile("systemgraphics/traffic3.png", true, true);
      this.TRAFFIC4 = BitmapStore.AddFile("systemgraphics/traffic4.png", true, true);
      this.FIRELISTICONS = BitmapStore.AddFile("systemgraphics/firelisticons.png", true, true);
      this.EYE1 = BitmapStore.AddFile("systemgraphics/eye1.png", true, true);
      this.EYE2 = BitmapStore.AddFile("systemgraphics/eye2.png", true, true);
      this.EYE3 = BitmapStore.AddFile("systemgraphics/eye3.png", true, true);
      this.EYE4 = BitmapStore.AddFile("systemgraphics/eye4.png", true, true);
      this.ICONSUP2 = BitmapStore.AddFile("systemgraphics/cat1/SUP2-Icon.png", true);
      this.ICONSTR = BitmapStore.AddFile("systemgraphics/cat1/STR-Icon.png", true);
      this.ICONFLAG = BitmapStore.AddFile("systemgraphics/cat1/FLAG-Icon.png", true);
      this.COMBATICONS = BitmapStore.AddFile("systemgraphics/cat1/combatIcons.png", true);
      this.ICONEX1_VARIED = BitmapStore.AddFile("systemgraphics/cat1/EX-Icon_varied.png", true);
      this.ICONEP = BitmapStore.AddFile("systemgraphics/cat1/EP-Icon.png", true);
      this.ICONVIGOR = BitmapStore.AddFile("systemgraphics/cat1/VIGOR-Icon.png", true);
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading System cat1";
      Application.DoEvents();
      this.SMALLCHAR1 = BitmapStore.AddFile("systemgraphics/cat1/small1.png", true);
      this.SMALLCHAR2 = BitmapStore.AddFile("systemgraphics/cat1/small2.png", true);
      this.PAPERBACK1 = BitmapStore.AddFile("systemgraphics/cat1/paperBackground1.png", true);
      this.PAPERBACK2 = BitmapStore.AddFile("systemgraphics/cat1/paperBackground2.png", true);
      this.PAPERBACK3 = BitmapStore.AddFile("systemgraphics/cat1/paperBackground3.png", true);
      this.MARCCARD1A = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Attack-big.png", true);
      this.MARCCARD1B = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Attack-med.png", true);
      this.MARCCARD1C = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Attack-small.png", true);
      this.MARCCARD5A = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Defend-big.png", true);
      this.MARCCARD5B = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Defend-med.png", true);
      this.MARCCARD5C = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Defend-small.png", true);
      this.MARCCARD6A = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Org-big.png", true);
      this.MARCCARD6B = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Org-med.png", true);
      this.MARCCARD6C = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Org-small.png", true);
      this.MARCCARD2A = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Move-big.png", true);
      this.MARCCARD2B = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Move-med.png", true);
      this.MARCCARD2C = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Move-small.png", true);
      this.MARCCARD4A = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-HCommand-big.png", true);
      this.MARCCARD4B = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-HCommand-med.png", true);
      this.MARCCARD4C = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-HCommand-small.png", true);
      this.MARCCARD3A = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Regime-big.png", true);
      this.MARCCARD3B = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Regime-med.png", true);
      this.MARCCARD3C = BitmapStore.AddFile("systemgraphics/cat1/ActionCard-Regime-small.png", true);
      this.MARCOFFICER = BitmapStore.AddFile("systemgraphics/cat1/OfficerVic.png", true);
      this.MARCLARGETAB = BitmapStore.AddFile("systemgraphics/cat1/LargeTabButton.png", true);
      this.MARCOPTSLOTS = BitmapStore.AddFile("systemgraphics/cat1/OptSlots.png", true);
      this.MARCTAB = BitmapStore.AddFile("systemgraphics/cat1/Tab.png", true);
      this.MARCARROW = BitmapStore.AddFile("systemgraphics/cat1/MarcArrow.png", true);
      this.MARCTABBUTTON = BitmapStore.AddFile("systemgraphics/cat1/TabButton.png", true);
      this.MARCTABBUTTONHIGH = BitmapStore.AddFile("systemgraphics/cat1/TabButtonHigh.png", true);
      this.MARCTOPBAR = BitmapStore.AddFile("systemgraphics/cat1/TopBar.png", true);
      this.MARCSCOPE = BitmapStore.AddFile("systemgraphics/cat1/Scope.png", true);
      this.MARCBOTBAR = BitmapStore.AddFile("systemgraphics/cat1/BotBar.png", true);
      this.MARCBUTBARFRAME = BitmapStore.AddFile("systemgraphics/cat1/ButBarFrame.png", true);
      this.LEATHER = BitmapStore.AddFile("systemgraphics/cat1/LeatherTile.png", true);
      this.COMBATART1 = BitmapStore.AddFile("systemgraphics/cat1/CombatArt1.png", true);
      this.COMBATART2 = BitmapStore.AddFile("systemgraphics/cat1/CombatArt2.png", true);
      this.MARCMESFRAME = BitmapStore.AddFile("systemgraphics/cat1/MessFrame.png", true);
      this.EXPLOSION = BitmapStore.AddFile("systemgraphics/cat1/explosion.png", true, true);
      this.BUTTONMARC1 = BitmapStore.AddFile("systemgraphics/cat1/Button Marc.png", true);
      this.BUTTONMARC1b = BitmapStore.AddFile("systemgraphics/cat1/Button Marc Over.png", true);
      this.ICONAP1 = BitmapStore.AddFile("systemgraphics/cat1/AP-Icon.png", true);
      this.ICONIN1 = BitmapStore.AddFile("systemgraphics/cat1/INT-Icon.png", true);
      this.ICONSU1 = BitmapStore.AddFile("systemgraphics/cat1/SUP-Icon.png", true);
      this.ICONRD1 = BitmapStore.AddFile("systemgraphics/cat1/RND-Icon.png", true);
      this.ICONEN1 = BitmapStore.AddFile("systemgraphics/cat1/EN-Icon.png", true);
      this.ICONEX1 = BitmapStore.AddFile("systemgraphics/cat1/EX-Icon.png", true);
      this.ICONMO1 = BitmapStore.AddFile("systemgraphics/cat1/MOR-Icon.png", true);
      this.ICONMO2 = BitmapStore.AddFile("systemgraphics/cat1/MO-MiniIcon.png", true);
      this.ICONRD2 = BitmapStore.AddFile("systemgraphics/cat1/RD-MiniIcon.png", true);
      this.ICONEN2 = BitmapStore.AddFile("systemgraphics/cat1/EN-MiniIcon.png", true);
      this.ICONEX2 = BitmapStore.AddFile("systemgraphics/cat1/EX-MiniIcon.png", true);
      this.MARCINTRO2 = BitmapStore.AddFile("systemgraphics/cat1/marcintro2.png", true);
      this.MINICARD = BitmapStore.AddFile("systemgraphics/cat1/minicard.png", true);
      this.MINICARDBIG = BitmapStore.AddFile("systemgraphics/cat1/minicardbig.png", true);
      this.FRAME1 = BitmapStore.AddFile("systemgraphics/cat1/framework.png", true);
      this.MARCBACK1 = BitmapStore.AddFile("systemgraphics/cat1/Button-LightRed-normal.png", true);
      this.MARCBACK2 = BitmapStore.AddFile("systemgraphics/cat1/Button-Red-normal.png", true);
      this.MARCBACK3 = BitmapStore.AddFile("systemgraphics/cat1/Button-Blue-normal.png", true);
      this.MARCBACK4 = BitmapStore.AddFile("systemgraphics/cat1/Button-Yellow-normal.png", true);
      this.MARCBACK1B = BitmapStore.AddFile("systemgraphics/cat1/Button-LightRed-over.png", true);
      this.MARCBACK2B = BitmapStore.AddFile("systemgraphics/cat1/Button-Red-over.png", true);
      this.MARCBACK3B = BitmapStore.AddFile("systemgraphics/cat1/Button-Blue-over.png", true);
      this.MARCBACK4B = BitmapStore.AddFile("systemgraphics/cat1/Button-Yellow-over.png", true);
      this.LOGOFLATTINY = BitmapStore.AddFile("systemgraphics/cat1/logo-flat-tiny.png", true);
      this.RADIO1 = BitmapStore.AddFile("systemgraphics/cat1/RadioButton1.png", true);
      this.RADIO1A = BitmapStore.AddFile("systemgraphics/cat1/RadioButton1a.png", true);
      this.RADIO2 = BitmapStore.AddFile("systemgraphics/cat1/RadioButton2.png", true);
      this.RADIO2A = BitmapStore.AddFile("systemgraphics/cat1/RadioButton2a.png", true);
      this.BACKGROUND1MARC = BitmapStore.AddFile("systemgraphics/cat1/backgroundmarc.png", true);
      this.BACKGROUND3MARC = BitmapStore.AddFile("systemgraphics/cat1/background3marc.png", true);
      this.BACKGROUND5MARC = BitmapStore.AddFile("systemgraphics/cat1/background4marc.png", true);
      this.BACKTUT = BitmapStore.AddFile("systemgraphics/cat1/background3tut.png", true);
      this.BUTTONSTEVE1 = this.BUTTONMARC1;
      this.BUTTONSTEVE1b = this.BUTTONMARC1b;
      this.BUTTONLONG = BitmapStore.AddFile("systemgraphics/cat1/marclong.png", true);
      this.BUTTONLONGHIGH = BitmapStore.AddFile("systemgraphics/cat1/marclongb.png", true);
      this.BUTTONGUIUP = BitmapStore.AddFile("systemgraphics/cat1/buttonup.png", true);
      this.BUTTONGUIDOWN = BitmapStore.AddFile("systemgraphics/cat1/buttondown.png", true);
      this.BUTTONSTEVE2 = this.BUTTONMARC1;
      this.BUTTONSTEVE2B = this.BUTTONMARC1b;
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading System gfx cat3";
      Application.DoEvents();
      this.BUTTONSEAATTACK = BitmapStore.AddFile("systemgraphics/cat3/buttonseaattack.png", true);
      this.BUTTONINTERDICT = BitmapStore.AddFile("systemgraphics/cat3/buttonbomber.png", true);
      this.BUTTONSEAARTATTACK = BitmapStore.AddFile("systemgraphics/cat3/buttonseaartattack.png", true);
      this.BUTTONTRANSFER = BitmapStore.AddFile("systemgraphics/cat3/buttontransfer.png", true);
      this.BUTTONNEWUNIT = BitmapStore.AddFile("systemgraphics/cat3/buttonnewunit.png", true);
      this.BUTTONHQPROD = BitmapStore.AddFile("systemgraphics/cat3/buttonhqprod.png", true);
      this.BUTTONPROD = BitmapStore.AddFile("systemgraphics/cat3/buttonprod.png", true);
      this.BUTTONDRAW = BitmapStore.AddFile("systemgraphics/cat3/buttondraw.bmp", true);
      this.BUTTONUP = BitmapStore.AddFile("systemgraphics/cat3/buttonup.bmp", true);
      this.BUTTONDOWN = BitmapStore.AddFile("systemgraphics/cat3/buttondown.bmp", true);
      this.BUTTONBLOCK = BitmapStore.AddFile("systemgraphics/cat3/buttonblock.bmp", true);
      this.BUTTONBLUE = BitmapStore.AddFile("systemgraphics/cat3/buttonblue.bmp", true);
      this.BUTTONYELLOW = BitmapStore.AddFile("systemgraphics/cat3/buttonyellow.bmp", true);
      this.BUTTONFLAGGED = BitmapStore.AddFile("systemgraphics/cat3/buttonflagged.bmp", true);
      this.BUTTONUNFLAGGED = BitmapStore.AddFile("systemgraphics/cat3/buttonunflagged.bmp", true);
      this.BUTTONPLUS = BitmapStore.AddFile("systemgraphics/cat3/buttonplus.bmp", true);
      this.BUTTONKILL = BitmapStore.AddFile("systemgraphics/cat3/buttonkill.bmp", true);
      this.TEXTCLOUD = BitmapStore.AddFile("systemgraphics/cat3/textcloud.png", true);
      this.EDITDRAW = BitmapStore.AddFile("systemgraphics/cat3/editdraw.bmp", true);
      this.EDITPAINT = BitmapStore.AddFile("systemgraphics/cat3/editpaint.bmp", true);
      this.EDITPOINTER = BitmapStore.AddFile("systemgraphics/cat3/editpointer.bmp", true);
      this.BUTTONDESIGNSF = BitmapStore.AddFile("systemgraphics/cat3/buttondesignsf.png", true);
      this.BUTTONOFFICER = BitmapStore.AddFile("systemgraphics/cat3/buttonofficer.png", true);
      this.SLIDER1 = BitmapStore.AddFile("systemgraphics/cat3/slider normal.png", true);
      this.SLIDER2 = BitmapStore.AddFile("systemgraphics/cat3/slider mouse over.png", true);
      this.BUTTONSUPPLYOFF = BitmapStore.AddFile("systemgraphics/cat3/buttonsupplyoff.png", true);
      this.BUTTONGIVEHEX = BitmapStore.AddFile("systemgraphics/cat3/buttongivehex.png", true);
      this.BLOWLOCATIONBUTTON = BitmapStore.AddFile("systemgraphics/cat3/buttonblowlocation.png", true);
      this.BUTTONAIRSUPPLY = BitmapStore.AddFile("systemgraphics/cat3/buttonairsupply.png", true);
      this.BUTTONSTATISTICS = BitmapStore.AddFile("systemgraphics/cat3/buttonstatistics.png", true);
      this.BUTTONSURRENDER = BitmapStore.AddFile("systemgraphics/cat3/buttonsurrender.png", true);
      this.BUTTONDISBAND = BitmapStore.AddFile("systemgraphics/cat3/buttondisband.png", true);
      this.BUTTONNEWUNIT2 = BitmapStore.AddFile("systemgraphics/cat3/buttonnewunit2.png", true);
      this.BUTTONPASTE = BitmapStore.AddFile("systemgraphics/cat3/buttonpaste.bmp", true);
      this.BUTTONCOPY = BitmapStore.AddFile("systemgraphics/cat3/buttoncopy.bmp", true);
      this.WHITEFLAG = BitmapStore.AddFile("systemgraphics/cat3/whiteflag.bmp", true);
      this.BUTTONPREFS = BitmapStore.AddFile("systemgraphics/cat3/buttonprefs.png", true);
      this.QUESTIONBALL = BitmapStore.AddFile("systemgraphics/cat3/questionball.bmp", true);
      this.BUTTONCHANGEMODEL = BitmapStore.AddFile("systemgraphics/cat3/orderchangemodel.png", true);
      this.BUTTONMODELDESIGNER = BitmapStore.AddFile("systemgraphics/cat3/ordermodeldesigner.png", true);
      this.BUTTONBUILDLOCATION = BitmapStore.AddFile("systemgraphics/cat3/buttonbuildlocation.png", true);
      this.BUTTONQUIT = BitmapStore.AddFile("systemgraphics/cat3/buttonquit.png", true);
      this.BUTTONRESEARCH = BitmapStore.AddFile("systemgraphics/cat3/buttonresearch.png", true);
      this.BUTTONPARADROP = BitmapStore.AddFile("systemgraphics/cat3/buttonparadrop.png", true);
      this.BUTTONLOAD = BitmapStore.AddFile("systemgraphics/cat3/buttonload.png", true);
      this.BUTTONUNLOAD = BitmapStore.AddFile("systemgraphics/cat3/buttonunload.png", true);
      this.BUTTONDIP = BitmapStore.AddFile("systemgraphics/cat3/buttondip.png", true);
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading Cards";
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading Fow Gfx";
      Application.DoEvents();
      this.FOGSHEET = BitmapStore.AddFile("systemgraphics/fog/fogofwar.png", true, true);
      this.SHROWDSHEET = BitmapStore.AddFile("systemgraphics/shrowd/shrowd.png", true, true);
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading 'Nato' Gfx";
      Application.DoEvents();
      str1 = "";
      num1: i32;
      do
      {
        num1 = 0;
        Number: i32;
        Number += 1;
        if (File.Exists(this.AppPath + "graphics/systemgraphics/" + this.ModNatoCounters + "/" + Strings.Trim(Conversion.Str( Number)) + ".png"))
        {
          this.NATO = (int[]) Utils.CopyArray((Array) this.NATO, (Array) new int[Number + 1]);
          num1 = 1;
          this.NATO[Number] = BitmapStore.AddFile("systemgraphics/" + this.ModNatoCounters + "/" + Strings.Trim(Conversion.Str( Number)) + ".png", true, true);
        }
        if (num1 == 0)
        {
          if (File.Exists(this.AppPath + "graphics/" + this.ModSystemGraphicsDirectory + "/" + this.ModNatoCounters + "/" + Strings.Trim(Conversion.Str( Number)) + ".png"))
          {
            this.NATO = (int[]) Utils.CopyArray((Array) this.NATO, (Array) new int[Number + 1]);
            num1 = 1;
            this.NATO[Number] = BitmapStore.AddFile("systemgraphics/" + this.ModNatoCounters + "/" + Strings.Trim(Conversion.Str( Number)) + ".png", true, true);
          }
        }
      }
      while (num1 == 1);
      this.SPRITE64[0, 0, 0, 0, 0, 0] = 1;
      this.SPRITE64[1, 0, 0, 0, 0, 0] = 2;
      this.SPRITE64[0, 1, 0, 0, 0, 0] = 3;
      this.SPRITE64[0, 0, 1, 0, 0, 0] = 4;
      this.SPRITE64[0, 0, 0, 1, 0, 0] = 5;
      this.SPRITE64[0, 0, 0, 0, 1, 0] = 6;
      this.SPRITE64[0, 0, 0, 0, 0, 1] = 7;
      this.SPRITE64[1, 1, 0, 0, 0, 0] = 8;
      this.SPRITE64[0, 1, 1, 0, 0, 0] = 9;
      this.SPRITE64[0, 0, 1, 1, 0, 0] = 10;
      this.SPRITE64[0, 0, 0, 1, 1, 0] = 11;
      this.SPRITE64[0, 0, 0, 0, 1, 1] = 12;
      this.SPRITE64[1, 0, 0, 0, 0, 1] = 13;
      this.SPRITE64[1, 0, 1, 0, 0, 0] = 14;
      this.SPRITE64[0, 1, 0, 1, 0, 0] = 15;
      this.SPRITE64[0, 0, 1, 0, 1, 0] = 16;
      this.SPRITE64[0, 0, 0, 1, 0, 1] = 17;
      this.SPRITE64[1, 0, 0, 0, 1, 0] = 18;
      this.SPRITE64[0, 1, 0, 0, 0, 1] = 19;
      this.SPRITE64[1, 0, 0, 1, 0, 0] = 20;
      this.SPRITE64[0, 1, 0, 0, 1, 0] = 21;
      this.SPRITE64[0, 0, 1, 0, 0, 1] = 22;
      this.SPRITE64[1, 1, 1, 0, 0, 0] = 23;
      this.SPRITE64[0, 1, 1, 1, 0, 0] = 24;
      this.SPRITE64[0, 0, 1, 1, 1, 0] = 25;
      this.SPRITE64[0, 0, 0, 1, 1, 1] = 26;
      this.SPRITE64[1, 0, 0, 0, 1, 1] = 27;
      this.SPRITE64[1, 1, 0, 0, 0, 1] = 28;
      this.SPRITE64[1, 1, 0, 1, 0, 0] = 29;
      this.SPRITE64[0, 1, 1, 0, 1, 0] = 30;
      this.SPRITE64[0, 0, 1, 1, 0, 1] = 31;
      this.SPRITE64[1, 0, 0, 1, 1, 0] = 32;
      this.SPRITE64[0, 1, 0, 0, 1, 1] = 33;
      this.SPRITE64[1, 0, 1, 0, 0, 1] = 34;
      this.SPRITE64[1, 1, 0, 0, 1, 0] = 35;
      this.SPRITE64[0, 1, 1, 0, 0, 1] = 36;
      this.SPRITE64[1, 0, 1, 1, 0, 0] = 37;
      this.SPRITE64[0, 1, 0, 1, 1, 0] = 38;
      this.SPRITE64[0, 0, 1, 0, 1, 1] = 39;
      this.SPRITE64[1, 0, 0, 1, 0, 1] = 40;
      this.SPRITE64[1, 0, 1, 0, 1, 0] = 41;
      this.SPRITE64[0, 1, 0, 1, 0, 1] = 42;
      this.SPRITE64[0, 0, 1, 1, 1, 1] = 43;
      this.SPRITE64[1, 0, 0, 1, 1, 1] = 44;
      this.SPRITE64[1, 1, 0, 0, 1, 1] = 45;
      this.SPRITE64[1, 1, 1, 0, 0, 1] = 46;
      this.SPRITE64[1, 1, 1, 1, 0, 0] = 47;
      this.SPRITE64[0, 1, 1, 1, 1, 0] = 48;
      this.SPRITE64[0, 1, 0, 1, 1, 1] = 49;
      this.SPRITE64[1, 0, 1, 0, 1, 1] = 50;
      this.SPRITE64[1, 1, 0, 1, 0, 1] = 51;
      this.SPRITE64[1, 1, 1, 0, 1, 0] = 52;
      this.SPRITE64[0, 1, 1, 1, 0, 1] = 53;
      this.SPRITE64[1, 0, 1, 1, 1, 0] = 54;
      this.SPRITE64[0, 1, 1, 0, 1, 1] = 55;
      this.SPRITE64[1, 0, 1, 1, 0, 1] = 56;
      this.SPRITE64[1, 1, 0, 1, 1, 0] = 57;
      this.SPRITE64[0, 1, 1, 1, 1, 1] = 58;
      this.SPRITE64[1, 0, 1, 1, 1, 1] = 59;
      this.SPRITE64[1, 1, 0, 1, 1, 1] = 60;
      this.SPRITE64[1, 1, 1, 0, 1, 1] = 61;
      this.SPRITE64[1, 1, 1, 1, 0, 1] = 62;
      this.SPRITE64[1, 1, 1, 1, 1, 0] = 63;
      this.SPRITE64[1, 1, 1, 1, 1, 1] = 64;
      this.SHEETX[1] = 5;
      this.SHEETY[1] = 10;
      this.SHEETX[2] = 0;
      this.SHEETY[2] = 0;
      this.SHEETX[3] = 1;
      this.SHEETY[3] = 0;
      this.SHEETX[4] = 2;
      this.SHEETY[4] = 0;
      this.SHEETX[5] = 3;
      this.SHEETY[5] = 0;
      this.SHEETX[6] = 4;
      this.SHEETY[6] = 0;
      this.SHEETX[7] = 5;
      this.SHEETY[7] = 0;
      this.SHEETX[8] = 0;
      this.SHEETY[8] = 1;
      this.SHEETX[9] = 5;
      this.SHEETY[9] = 1;
      this.SHEETX[10] = 3;
      this.SHEETY[10] = 2;
      this.SHEETX[11] = 0;
      this.SHEETY[11] = 3;
      this.SHEETX[12] = 2;
      this.SHEETY[12] = 3;
      this.SHEETX[13] = 4;
      this.SHEETY[13] = 1;
      this.SHEETX[14] = 1;
      this.SHEETY[14] = 1;
      this.SHEETX[15] = 0;
      this.SHEETY[15] = 2;
      this.SHEETX[16] = 4;
      this.SHEETY[16] = 2;
      this.SHEETX[17] = 1;
      this.SHEETY[17] = 3;
      this.SHEETX[18] = 3;
      this.SHEETY[18] = 1;
      this.SHEETX[19] = 2;
      this.SHEETY[19] = 2;
      this.SHEETX[20] = 2;
      this.SHEETY[20] = 1;
      this.SHEETX[21] = 1;
      this.SHEETY[21] = 2;
      this.SHEETX[22] = 5;
      this.SHEETY[22] = 2;
      this.SHEETX[23] = 3;
      this.SHEETY[23] = 3;
      this.SHEETX[24] = 1;
      this.SHEETY[24] = 5;
      this.SHEETX[25] = 1;
      this.SHEETY[25] = 6;
      this.SHEETX[26] = 4;
      this.SHEETY[26] = 6;
      this.SHEETX[27] = 0;
      this.SHEETY[27] = 5;
      this.SHEETX[28] = 0;
      this.SHEETY[28] = 4;
      this.SHEETX[29] = 4;
      this.SHEETY[29] = 3;
      this.SHEETX[30] = 2;
      this.SHEETY[30] = 5;
      this.SHEETX[31] = 2;
      this.SHEETY[31] = 6;
      this.SHEETX[32] = 4;
      this.SHEETY[32] = 4;
      this.SHEETX[33] = 0;
      this.SHEETY[33] = 6;
      this.SHEETX[34] = 3;
      this.SHEETY[34] = 4;
      this.SHEETX[35] = 5;
      this.SHEETY[35] = 3;
      this.SHEETX[36] = 3;
      this.SHEETY[36] = 5;
      this.SHEETX[37] = 1;
      this.SHEETY[37] = 4;
      this.SHEETX[38] = 4;
      this.SHEETY[38] = 5;
      this.SHEETX[39] = 3;
      this.SHEETY[39] = 6;
      this.SHEETX[40] = 5;
      this.SHEETY[40] = 4;
      this.SHEETX[41] = 2;
      this.SHEETY[41] = 4;
      this.SHEETX[42] = 5;
      this.SHEETY[42] = 5;
      this.SHEETX[43] = 4;
      this.SHEETY[43] = 8;
      this.SHEETX[44] = 1;
      this.SHEETY[44] = 9;
      this.SHEETX[45] = 4;
      this.SHEETY[45] = 7;
      this.SHEETX[46] = 1;
      this.SHEETY[46] = 7;
      this.SHEETX[47] = 5;
      this.SHEETY[47] = 6;
      this.SHEETX[48] = 5;
      this.SHEETY[48] = 7;
      this.SHEETX[49] = 2;
      this.SHEETY[49] = 8;
      this.SHEETX[50] = 0;
      this.SHEETY[50] = 9;
      this.SHEETX[51] = 3;
      this.SHEETY[51] = 7;
      this.SHEETX[52] = 0;
      this.SHEETY[52] = 7;
      this.SHEETX[53] = 0;
      this.SHEETY[53] = 8;
      this.SHEETX[54] = 3;
      this.SHEETY[54] = 8;
      this.SHEETX[55] = 1;
      this.SHEETY[55] = 8;
      this.SHEETX[56] = 5;
      this.SHEETY[56] = 8;
      this.SHEETX[57] = 2;
      this.SHEETY[57] = 7;
      this.SHEETX[58] = 1;
      this.SHEETY[58] = 10;
      this.SHEETX[59] = 0;
      this.SHEETY[59] = 10;
      this.SHEETX[60] = 5;
      this.SHEETY[60] = 9;
      this.SHEETX[61] = 4;
      this.SHEETY[61] = 9;
      this.SHEETX[62] = 3;
      this.SHEETY[62] = 9;
      this.SHEETX[63] = 2;
      this.SHEETY[63] = 9;
      this.SHEETX[64] = 2;
      this.SHEETY[64] = 10;
      this.ARROW64[1, 0, 0, 0, 0, 0] = 1;
      this.ARROW64[0, 1, 0, 0, 0, 0] = 2;
      this.ARROW64[0, 0, 1, 0, 0, 0] = 3;
      this.ARROW64[0, 0, 0, 1, 0, 0] = 4;
      this.ARROW64[0, 0, 0, 0, 1, 0] = 5;
      this.ARROW64[0, 0, 0, 0, 0, 1] = 6;
      this.ARROW64[1, 2, 0, 0, 0, 0] = 7;
      this.ARROW64[1, 0, 2, 0, 0, 0] = 8;
      this.ARROW64[1, 0, 0, 2, 0, 0] = 9;
      this.ARROW64[1, 0, 0, 0, 2, 0] = 10;
      this.ARROW64[1, 0, 0, 0, 0, 2] = 11;
      this.ARROW64[0, 1, 2, 0, 0, 0] = 12;
      this.ARROW64[0, 1, 0, 2, 0, 0] = 13;
      this.ARROW64[0, 1, 0, 0, 2, 0] = 14;
      this.ARROW64[0, 1, 0, 0, 0, 2] = 15;
      this.ARROW64[0, 0, 1, 2, 0, 0] = 16;
      this.ARROW64[0, 0, 1, 0, 2, 0] = 17;
      this.ARROW64[0, 0, 1, 0, 0, 2] = 18;
      this.ARROW64[0, 0, 0, 1, 2, 0] = 19;
      this.ARROW64[0, 0, 0, 1, 0, 2] = 20;
      this.ARROW64[0, 0, 0, 0, 1, 2] = 21;
      this.ARROW64[2, 1, 0, 0, 0, 0] = 7;
      this.ARROW64[2, 0, 1, 0, 0, 0] = 8;
      this.ARROW64[2, 0, 0, 1, 0, 0] = 9;
      this.ARROW64[2, 0, 0, 0, 1, 0] = 10;
      this.ARROW64[2, 0, 0, 0, 0, 1] = 11;
      this.ARROW64[0, 2, 1, 0, 0, 0] = 12;
      this.ARROW64[0, 2, 0, 1, 0, 0] = 13;
      this.ARROW64[0, 2, 0, 0, 1, 0] = 14;
      this.ARROW64[0, 2, 0, 0, 0, 1] = 15;
      this.ARROW64[0, 0, 2, 1, 0, 0] = 16;
      this.ARROW64[0, 0, 2, 0, 1, 0] = 17;
      this.ARROW64[0, 0, 2, 0, 0, 1] = 18;
      this.ARROW64[0, 0, 0, 2, 1, 0] = 19;
      this.ARROW64[0, 0, 0, 2, 0, 1] = 20;
      this.ARROW64[0, 0, 0, 0, 2, 1] = 21;
      this.ARROW64[2, 0, 0, 0, 0, 0] = 25;
      this.ARROW64[0, 2, 0, 0, 0, 0] = 26;
      this.ARROW64[0, 0, 2, 0, 0, 0] = 27;
      this.ARROW64[0, 0, 0, 2, 0, 0] = 28;
      this.ARROW64[0, 0, 0, 0, 2, 0] = 29;
      this.ARROW64[0, 0, 0, 0, 0, 2] = 30;
      Application.DoEvents();
      this.Data = DataClass::new();
      Application.DoEvents();
      if (Operators.CompareString(this.ModSystemGraphicsDirectory, "systemgraphics", false) != 0)
        BitmapStore.ReloadSystemGraphics(this.ModSystemGraphicsDirectory);
      else
        BitmapStore.lastreloadsystemgfx = "";
      Application.DoEvents();
      this.CornerX = 0;
      this.CornerY = 0;
      this.SelectX = -1;
      this.SelectY = -1;
      if (this.ModScenario.Length > 1)
      {
        str2: String = this.AppPath + this.ModScenario;
        if (!Information.IsNothing( tformref))
          tformref.Label1.Text = "Loading Scn Gfx 1/6";
        Application.DoEvents();
        this.HandyFunctionsObj.Unzip(str2);
        if (!Information.IsNothing( tformref))
          tformref.Label1.Text = "Loading Scn Gfx 2/6";
        Application.DoEvents();
        this.Data = DataClass.deserialize(str2);
        if (!Information.IsNothing( tformref))
          tformref.Label1.Text = "Loading Scn Gfx 3/6";
        Application.DoEvents();
        this.HandyFunctionsObj.ZipFile(str2);
        this.Data.LoadGraphics(tformref);
      }
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Finishing Up";
      Application.DoEvents();
      this.CustomBitmapObj = new CustomBitmapClass(this);
      this.HelpCounter = -1;
      this.ProcessingObj = new ProcessingClass(this);
      this.EventRelatedObj = new EventRelatedClass(this);
      this.EditObj = new EditClass(this.AppPath + "editobj.txt");
      if (this.useModLib)
      {
        this.modlib_Initialize();
        this.modlib_loadPrefs();
      }
      Application.DoEvents();
    }

    pub fn LoadHelpFiles()
    {
      FileInfo[] files;
      bool flag;
      try
      {
        files = new DirectoryInfo(AppDomain.CurrentDomain.BaseDirectory + "help/").GetFiles("*.txt", SearchOption.AllDirectories);
        flag = false;
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        flag = true;
        ProjectData.ClearProjectError();
      }
      this.HelpCounter = -1;
      if (flag || files.Length <= 0)
        return;
      foreach (FileInfo fileInfo in files)
      {
        this += 1.HelpCounter;
        StreamReader streamReader = File.OpenText(fileInfo.DirectoryName + "\\" + fileInfo.Name);
        end: String = streamReader.ReadToEnd();
        streamReader.Close();
        this.HelpFile = (string[]) Utils.CopyArray((Array) this.HelpFile, (Array) new string[this.HelpCounter + 1]);
        this.HelpText = (string[]) Utils.CopyArray((Array) this.HelpText, (Array) new string[this.HelpCounter + 1]);
        this.HelpDir = (string[]) Utils.CopyArray((Array) this.HelpDir, (Array) new string[this.HelpCounter + 1]);
        this.HelpText[this.HelpCounter] = end;
        this.HelpFile[this.HelpCounter] = fileInfo.Name.Replace(".txt", "");
        str: String = fileInfo.DirectoryName.Replace("/", "\\").Replace(AppDomain.CurrentDomain.BaseDirectory + "help\\", "").Replace(AppDomain.CurrentDomain.BaseDirectory + "help", "");
        str.Replace("\\", ":");
        this.HelpDir[this.HelpCounter] = str;
      }
    }

    pub fn modlib_Initialize()
    {
      FileInfo[] files = new DirectoryInfo(this.AppPath + this.ModScenarioDir + "/").GetFiles();
      this.modlib_Counter = -1;
      foreach (FileInfo fileInfo in files)
      {
        if (Strings.InStr(fileInfo.Extension.ToLower(), ".se1evlib") > 0 && Strings.InStr(fileInfo.Name.ToLower(), "_v") < 1 & Strings.InStr(fileInfo.Name.ToLower(), "default") < 1)
        {
          bool flag = false;
          this.EditObj.TempFileName = fileInfo.FullName.ToString();
          tData: DataClass;
          this.HandyFunctionsObj.LoadLibrary( tData);
          this.EditObj.TempFileName = "";
          if (tData.LibraryCounter > -1)
            flag = true;
          if (flag)
          {
            this += 1.modlib_Counter;
            this.modlib_Description = (string[]) Utils.CopyArray((Array) this.modlib_Description, (Array) new string[this.modlib_Counter + 1]);
            this.modlib_Designer = (string[]) Utils.CopyArray((Array) this.modlib_Designer, (Array) new string[this.modlib_Counter + 1]);
            this.modlib_Filename = (string[]) Utils.CopyArray((Array) this.modlib_Filename, (Array) new string[this.modlib_Counter + 1]);
            this.modlib_Flagged = (bool[]) Utils.CopyArray((Array) this.modlib_Flagged, (Array) new bool[this.modlib_Counter + 1]);
            this.modlib_Name = (string[]) Utils.CopyArray((Array) this.modlib_Name, (Array) new string[this.modlib_Counter + 1]);
            this.modlib_Version = (int[]) Utils.CopyArray((Array) this.modlib_Version, (Array) new int[this.modlib_Counter + 1]);
            this.modlib_Filename[this.modlib_Counter] = fileInfo.Name;
            this.modlib_Description[this.modlib_Counter] = tData.LibraryObj[0].information;
            this.modlib_Designer[this.modlib_Counter] = tData.LibraryObj[0].creator;
            this.modlib_Flagged[this.modlib_Counter] = false;
            this.modlib_Name[this.modlib_Counter] = tData.LibraryObj[0].name;
            this.modlib_Version[this.modlib_Counter] = tData.LibraryObj[0].version;
          }
          tData = (DataClass) null;
        }
      }
      let mut num: i32 =  num;
    }

    pub fn modlib_loadPrefs()
    {
      FileInfo fileInfo = new FileInfo(this.AppPath + "modlib.txt");
      if (fileInfo.Exists)
      {
        StreamReader streamReader = fileInfo.OpenText();
        while (!streamReader.EndOfStream)
        {
          try
          {
            Expression1: String = streamReader.ReadLine();
            Expression2: String = streamReader.ReadLine();
            if (!Information.IsNothing( Expression1) & !Information.IsNothing( Expression2))
            {
              bool boolean = Conversions.ToBoolean(Expression2);
              let mut modlibCounter: i32 =  this.modlib_Counter;
              for (let mut index: i32 =  0; index <= modlibCounter; index += 1)
              {
                if (Operators.CompareString(this.modlib_Filename[index].ToLower(), Expression1.ToLower(), false) == 0)
                  this.modlib_Flagged[index] = boolean;
              }
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            streamReader.Close();
            ProjectData.ClearProjectError();
            break;
          }
        }
        streamReader.Close();
      }
      this.modlib_savePrefs();
    }

    pub fn modlib_savePrefs()
    {
      StreamWriter text = File.CreateText(this.AppPath + "modlib.txt");
      let mut modlibCounter: i32 =  this.modlib_Counter;
      for (let mut index: i32 =  0; index <= modlibCounter; index += 1)
      {
        text.WriteLine(this.modlib_Filename[index]);
        text.WriteLine(this.modlib_Flagged[index]);
      }
      text.Close();
    }
  }
// }
