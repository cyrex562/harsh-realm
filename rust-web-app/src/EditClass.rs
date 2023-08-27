// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EditClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;

namespace WindowsApplication1
{
  pub class EditClass
  {
    pub TutMode: bool;
    pub ButtonLoadMode: bool;
    pub TutOrder: i32;
    pub object TutX;
    pub object TutY;
    pub object TutHis;
    pub object TutSubpart;
    pub object TutCard;
    pub donequest: i32;
    pub SubformationListMode: bool;
    pub inSimpleEditor: bool;
    pub inSimpleMapEditor: bool;
    pub TutStep: i32;
    pub ranraw: i32;
    pub ranpop: i32;
    pub OfficerHisOverrule: i32;
    pub CombatTextual: bool;
    pub SkippedPreSelectPopup: bool;
    pub GuiDown: bool;
    pub RightDown: bool;
    pub LeftDown: bool;
    pub TempRegimeSlot: i32;
    pub TempHisModelUnit: i32;
    pub TipText: String;
    pub TipTitle: String;
    pub TipColor: i32;
    pub TipButton: bool;
    pub warningshown: bool;
    pub FindModFile: bool;
    pub ZipFileText: String;
    pub QuestionText: String;
    pub QuestionCard: i32;
    pub AnswerCount: i32;
    pub AnswerText: Vec<String>;
    pub AnswerTextMouseOver: Vec<String>;
    pub AnswerChosen: i32;
    pub nextEventSlot: i32;
    pub RandomUseMaster: String;
    pub EditorUseMaster: String;
    pub AIProgressNow: i32;
    pub AIProgressMax: i32;
    pub SFCompare: i32;
    pub BattleTimerActive: bool;
    pub DateTime BattleTimer;
    pub BattleAnimNr: i32;
    pub battleTimerInterceptFire: bool;
    pub battleTimerPopupRefreshDoesntStartIt: bool;
    pub Layout: i32;
    pub TownInfo: bool;
    pub TextInputString: String;
    pub MouseOverVisible: bool;
    pub MouseWheel: i32;
    pub MouseWheelWait: i32;
    pub MapPopupMode: bool;
    pub PencilType: i32;
    pub PencilData1: i32;
    pub PencilData2: i32;
    pub PencilMode: i32;
    pub RightClickX: i32;
    pub RightCLickY: i32;
    pub RightClickMap: i32;
    pub TempAIString: String;
    pub LocTypeSelected: i32;
    pub PeopleSelected: i32;
    pub TempRandom: i32;
    pub TempAIWatch: bool;
    pub AutoCombat: bool;
    pub Test: i32;
    pub TestEarlyCinematics: i32;
    pub MapMatrix2[] TempValue;
    pub MapMatrix2[] TempValueSpecial;
    pub MapMatrix2[] TempValueSpecial2;
    pub MapMatrix2[] TempValue2;
    pub MapMatrix2[] TempValue3;
    pub MapMatrix2[] TempValue4;
    pub MapMatrix2Plus6[] TempAttack;
    pub TempAI: Vec<i32>;
    pub TempAI2: Vec<i32>;
    pub MapMatrix2[] TempLos;
    pub MapMatrix2[] TempObstruct;
    pub MapMatrix2[] FuzzyPresumed;
    pub MapMatrix2[] FuzzyReal;
    pub int[,,] PossiblePull;
    pub int[,,] origPossiblePull;
    pub tempGroupMoveCounter: i32;
    pub tempGroupMoveUnr: Vec<i32>;
    pub CoordList[] tempGroupMoveCameFrom;
    pub CoordList[] tempGroupMovePath;
    pub MapMatrix2[] TempSup;
    pub MapMatrix2Coordinate[] TempSupCameFrom;
    pub LayerSupplyOn: bool;
    pub LayerSupplyHQ: i32;
    pub LayerSupplyAP: i32;
    pub HexRasterOn: bool;
    pub InsideSlotty: bool;
    pub HideDetail: bool;
    pub HideAS: bool;
    pub CombatSim: bool;
    pub DoQuit: bool;
    pub RemoveReinforce: Vec<i32>;
    pub LoadGame: String;
    pub ShownWelcome: bool;
    pub ShowInitialMenu: bool;
    pub MapMatrix2TempString: Vec<String>;
    pub MapMatrix2TempString2: Vec<String>;
    pub ApTempString: String;
    pub MapMatrix2Coordinate[] TempCameFrom;
    pub CoordList SupplyPath;
    pub AirSupplyPts: i32;
    pub AirSupplyCarry: i32;
    pub AirSupplyHq: i32;
    pub CoordList TempCoordList;
    pub CoordList TempCoordListLastMove;
    pub CoordList TempMovePathList;
    pub mouseOverActive: bool;
    pub UnitList TempUnitList;
    pub UnitList TempPossibleUnitList;
    pub CurrentDescript: String;
    pub MiddleWindow: bool;
    pub PaintShortcut1: i32;
    pub PaintShortcut2: i32;
    pub PaintShortcut3: i32;
    pub EventMap: i32;
    pub OrderType: i32;
    pub OrderSubType: i32;
    pub OrderUnit: i32;
    pub OrderTarget: i32;
    pub OrderData: i32;
    pub OrderX: i32;
    pub OrderY: i32;
    pub OrderMap: i32;
    pub OrderLoc: i32;
    pub orderInt: i32;
    pub TargetX: i32;
    pub TargetY: i32;
    pub TargetMap: i32;
    pub Phase: i32;
    pub MainSub: i32;
    pub OrderBombMode: i32;
    pub ShortRandomScreen: bool;
    pub maximumInterfaceSpace: bool;
    pub AlternateMusic: bool;
    pub firstroundcheck: bool;
    pub MapMatrix2[] HisOwner;
    pub MapMatrix2[] HisForce;
    pub MapMatrix2[] HisSFType;
    pub MapMatrix2[] HisHis;
    pub MapMatrix2[] HisDepth;
    pub HisStep: i32;
    pub Neighbours HisNeighbour;
    pub HisAttackType: i32;
    pub HisHotX: i32;
    pub HisHotY: i32;
    pub HisLossCounter: i32;
    pub HisRegimeWon: i32;
    pub HisLossAttReg: i32;
    pub HisLossDefReg: i32;
    pub HisLossSFType: Vec<i32>;
    pub HisLossAttacker: Vec<i32>;
    pub HisLossOK: Vec<i32>;
    pub HisLossDEAD: Vec<i32>;
    pub HisLossCounter2: i32;
    pub HisRegimeWon2: i32;
    pub HisLossAttReg2: i32;
    pub HisLossDefReg2: i32;
    pub HisLossSFType2: Vec<i32>;
    pub HisLossAttacker2: Vec<i32>;
    pub HisLossOK2: Vec<i32>;
    pub HisLossDEAD2: Vec<i32>;
    pub HisInfoString: String;
    pub TempPeopleSlot: i32;
    pub TempItemSlot: i32;
    pub TempSFType: i32;
    pub TempSelected: i32;
    pub TempCopy: i32;
    pub UnitSelected: i32;
    pub SFSelected: i32;
    pub MouseOverX: i32;
    pub MouseOverY: i32;
    pub TransferLostQty: i32;
    pub TransferLostType: i32;
    pub TransferLostTransports: i32;
    pub FeedBackString: String;
    pub CommandClass TempCommand;
    pub EventClass TempEvent;
    pub CombatOneSentence: String;
    pub CombatOneSentenceCustom: String;
    pub CombatOneSentence2: String;
    pub PrefShowFOW: bool;
    pub PrefCombatLog: bool;
    pub PrefMinimalistCounter: bool;
    pub CameFrom: i32;
    pub SFTypeSelected: i32;
    pub TempProdList1: i32;
    pub TempProdList2: i32;
    pub TempProdList3: i32;
    pub SetViewMode: i32;
    pub SetViewMode2: i32;
    pub SetSubViewMode: i32;
    pub SetViewMode3: bool;
    pub SetViewMode4: i32;
    pub SetViewModeExtraNr: i32;
    pub ShowTransfer: bool;
    pub HideUnit: i32;
    pub SoundOn: bool;
    pub ShowLabel: bool;
    pub ShowBase: bool;
    pub IntroSoundOn: bool;
    pub LastHistoryStep: i32;
    pub MapMatrix2 airRangeTempLos;
    pub MapMatrix2 airRangeMaxObstruct;
    pub MessageStyle: i32;
    pub MessageNote: String;
    pub MessageNote2: String;
    pub MessageName: String;
    pub MessageGroup: i32;
    pub MessageHideFromTab: bool;
    pub MessageHideFromStart: bool;
    pub CurrentMiniX: i32;
    pub CurrentMiniY: i32;
    pub ranmem: i32;
    pub ranr1: i32;
    pub ranr2: i32;
    pub ranr3: i32;
    pub ranr4: i32;
    pub ranr5: i32;
    pub ranr6: i32;
    pub ranr7: i32;
    pub ranr8: i32;
    pub ran1: i32;
    pub ran2: i32;
    pub ran3: i32;
    pub ran4: i32;
    pub ran5: i32;
    pub ran6: i32;
    pub ran7: i32;
    pub ran8: i32;
    pub ran9: i32;
    pub ran10: i32;
    pub ran11: i32;
    pub ran12: i32;
    pub ranoldkingdom: i32;
    pub randomaploop: i32;
    pub randoallied: i32;
    pub randoshrowd: i32;
    pub randofirsttech: i32;
    pub randomirror: i32;
    pub randoblockcenter: i32;
    pub ranoptimize: i32;
    pub ranmasterfile: String;
    pub ransinglestart: i32;
    pub ransworldsize: i32;
    pub ransplayer: i32;
    pub ranrawuse: i32;
    pub ranswater: i32;
    pub ransclimate: i32;
    pub ranscrate: i32;
    pub ranstats: i32;
    pub RandomSettingsFromMod: String;
    pub float PassengerDammage;
    pub AutoSave: bool;
    pub BlockAdvice: bool;
    pub TempBlockAdvice: bool;
    pub LastStatWindow: i32;
    pub StartWithHistory: bool;
    pub FromMessage: i32;
    pub AreaSlot: i32;
    pub AreaValue: i32;
    pub AreaX: i32;
    pub AreaY: i32;
    pub AreaMap: i32;
    pub PopupValue: i32;
    pub DoCardSlot: i32;
    pub EditClass.AfterPopUpRefresh MyDelegate;
    pub EditClass.AfterPopUpRefresh MyDelegateMap;
    pub TempEventWav: String;
    pub CampaignRoomBitmap: i32;
    pub CampaignRoomTitle: String;
    pub HumanPlayer: i32;
    pub Volume: i32;
    pub Volume2: i32;
    pub AIMoving: bool;
    pub RegimeColoring: bool;
    pub TempHisUnit: i32;
    pub HandCard: i32;
    pub TempAIInitialized: bool;
    pub TempAIRegime: i32;
    pub MapSelected: i32;
    pub HisHotMap: i32;
    pub SpreadUnit: bool;
    pub PbemUserName: String;
    pub PbemPassword: String;
    pub PbemSerial: String;
    pub PbemEmail: String;
    pub PbemTitle: String;
    pub PbemChallengeID: i32;
    pub PbemGameInstanceID: i32;
    pub PbemMiscString: String;
    pub PbemPrivatePassword: String;
    pub PbemChallengeMiscData: String;
    pub PbemSteam: bool;
    pub PbemAlreadyAccount: bool;
    pub ServerCommandType ServerCommand;
    pub ServerCommandStep: i32;
    pub ServerCommandMaxStep: i32;
    pub ServerCommandMaxStepOrig: i32;
    pub ServerStatusType ServerStatus;
    pub MessageList ServerMessages;
    pub DateTime ServerStatusStartTime;
    pub ServerLostConnect: bool;
    pub ServerRegisterReplyType ServerRegisterReply;
    pub ServerAuthReplyType ServerAuthReply;
    pub ServerSerialReplyType ServerSerialReply;
    pub ServerGenericReplyType ServerGenericReply;
    pub ServerRegisterUserNameReply: String;
    pub ServerStatusInitializeTried: bool;
    pub ServerCookieValue: String;
    pub byte[] ServerUploadFile;
    pub ServerCrc32: String;
    pub long ServerUploadSize;
    pub long ServerUploadDone;
    pub long ServerDownloadSize;
    pub long ServerDownloadDone;
    pub byte[] ServerDownloadFile;
    pub ChallengeClass[] ServerChallengeList;
    pub RunningGameClass[] ServerRunningGameList;
    pub ServerExactErrorCode: i32;
    pub ServerCommandType OrigServerCommand;
    pub ServerTextBuffer: String;
    pub PbemGameSetupPhase PbemGameSetup;
    pub PbemInspectReturnFromID: i32;
    pub PbemGameID: i32;
    pub PbemPlayer1: String;
    pub PbemPlayer2: String;
    pub PbemCheckPlayer: String;
    pub PbemGameOver: i32;
    pub ShowUnderHQ: bool;
    pub ShowSameHistorical: bool;
    pub ShowMouseOver: bool;
    pub ShowHQPower: bool;
    pub ShowAirRange: bool;
    pub ShowLISRange: bool;
    pub OldUnit: i32;
    pub ServerOrderCancel: bool;
    pub DoubleSize: bool;
    pub DoubleSizePercentage: i32;
    pub PurelyOrderRedrawRefresh: bool;
    pub pdfGenerated: String;
    pub helpAlreadyOpened: bool;
    pub systemPopup: bool;
    pub highGfxSpeedOn: bool;
    pub rightSideBarMode: i32;
    pub leftSideBarMode: i32;
    pub leftSideBarModePage: i32;
    pub debugAiOnlyTillRound: i32;
    pub attackTypeOption: i32;
    pub udsUnitOrderMode: i32;
    pub udsReturnFromPopup: bool;
    pub udsLastCalledPopupEventNr: i32;
    pub udsOrderBarFeedbackString: String;
    pub udsOrderBarFeedbackColor: i32;
    pub udsOrderPossible: bool;
    pub udsManagementTabOverrideId: i32;
    pub udsViewMode2Override: i32;
    pub CoordList tempDidInterceptList;
    pub useLeftRightClickMode: bool;
    pub se1_SelectZoneButton: i32;
    pub se1_SelectUnitButton: i32;
    pub se1_SelectAssetButton: i32;
    pub se1_AssetPage: i32;
    pub se1_UnitPage: i32;
    pub se1_AssetCategory1: i32;
    pub se1_AssetCategory2: i32;
    pub se1_AssetCategory3: i32;
    pub se1_AssetCategory4: i32;
    pub se1_AssetCategory5: i32;
    pub se1_assetMode: i32;
    pub se1_assetSHQ: i32;
    pub se1_assetZone: i32;
    pub se1_assetItem: i32;
    pub se1_assetItemMode1: i32;
    pub se1_assetItemMode2: i32;
    pub se1_assetRightPanel: i32;
    pub se1_assetPage2: i32;
    pub se1_modelSHQ: i32;
    pub se1_modelReinf: i32;
    pub se1_modelSelected: i32;
    pub se1_modelPage: i32;
    pub se1_modelPage2: i32;
    pub se1_modelView: i32;
    pub se1_modelObsoleteHidden: i32;
    pub se1_ExecReturnValue: i32;
    pub se1_CardsCategory: i32;
    pub se1_CardsTarget: i32;
    pub se1_CardsCard: i32;
    pub se1_CardsPage: i32;
    pub se1_CardsViewMode: i32;
    pub se1_CardsSmallCards: i32;
    pub se1_CardsSelectX: i32;
    pub se1_CardsSelectY: i32;
    pub se1_StrategyTab: i32;
    pub se1_StrategySpecial1: i32;
    pub se1_StrategySpecial2: i32;
    pub se1_adviceWindowState: i32;
    pub se1_adviceWindowPage: i32;
    pub se1_ManagementMode: bool;
    pub se1_ManagementTab: i32;
    pub se1_leaderGroup: i32;
    pub se1_leaderSelected: i32;
    pub SimpleList se1_leaderColumns;
    pub se1_leaderPage: i32;
    pub tempRenameString: String;
    pub tempRenameString2: String;
    pub tempRenameString3: String;
    pub statsTab_tab: i32;
    pub statsTab_item: i32;
    pub uds_subtab: Vec<i32>;
    pub uds_page: Vec<i32>;
    pub RealTurn: i32;
    pub RealRound: i32;
    pub overruleScreenResWidth: i32;
    pub overruleScreenResHeight: i32;
    pub mouseScreenLock: bool;
    pub EarlyCinematicsLoggedIn: bool;
    pub showAdvice: bool;
    pub dontShowAImoves: bool;
    pub usePullAssets: bool;
    pub usePullCities: bool;
    pub usePullUnits: bool;
    pub matrixSubPart_BlockMouseUpScroller1time: bool;
    pub const ORDERMOVE: i32 =  1;
    pub const ORDERATTACK: i32 =  2;
    pub const ORDERUNITHQ: i32 =  3;
    pub const ORDERPRODHQ: i32 =  4;
    pub const ORDERRECRUIT: i32 =  5;
    pub const ORDERPROD: i32 =  6;
    pub const ORDERNEWUNIT: i32 =  7;
    pub const ORDERNEWSF: i32 =  8;
    pub const ORDERTRANSFER: i32 =  9;
    pub const ORDERARTATTACK: i32 =  11;
    pub const ORDERSEAATTACK: i32 =  12;
    pub const ORDERSEAARTATTACK: i32 =  13;
    pub const ORDERAIRSTRIKE: i32 =  14;
    pub const ORDERBOMB: i32 =  15;
    pub const ORDERINTERNALAA: i32 =  16;
    pub const ORDERINTERNALDOGFIGHT: i32 =  17;
    pub const ORDERSTRATEGIC: i32 =  18;
    pub const ORDERPARADROP: i32 =  19;
    pub const ORDERLOAD: i32 =  20;
    pub const ORDERUNLOAD: i32 =  21;
    pub const ORDERPOOL: i32 =  22;
    pub const ORDERRESEARCH: i32 =  23;
    pub const ORDERDIP: i32 =  24;
    pub const ORDERCONSTRUCT: i32 =  25;
    pub const ORDERHISTORY: i32 =  26;
    pub const ORDERHISTORYACAP: i32 =  27;
    pub const ORDERHISTORYACAP2: i32 =  28;
    pub const ORDERHISTORYACAP3: i32 =  29;
    pub const ORDERPREFS: i32 =  30;
    pub const ORDERINTERNALREBEL: i32 =  31;
    pub const ORDERSTATS: i32 =  32;
    pub const ORDERAIRRECON: i32 =  33;
    pub const ORDERPEOPLETRANSFER: i32 =  34;
    pub const ORDERBLOW: i32 =  35;
    pub const ORDERINFRA: i32 =  36;
    pub const ORDERBUILD: i32 =  37;
    pub const ORDERVIEWSUPPLY: i32 =  38;
    pub const ORDERACAP: i32 =  39;
    pub const ORDERAIRSUPPLY: i32 =  40;
    pub const ORDERSURRENDER: i32 =  41;
    pub const ORDERAIRLIFT: i32 =  42;
    pub const ORDERAITEST: i32 =  43;
    pub const ORDERNEWUNIT2: i32 =  44;
    pub const ORDEROFFICER: i32 =  45;
    pub const ORDERCHANGEMODEL: i32 =  46;
    pub const ORDERMODELDESIGNER: i32 =  47;
    pub const ORDERGROUPMOVE: i32 =  48;
    pub const ORDERGROUPSTRATEGIC: i32 =  49;
    pub const ORDERNEXT: i32 =  50;
    pub const ORDERSUPPLYLAYER: i32 =  51;
    pub const ORDERSFDESIGN: i32 =  52;
    pub const ORDERTRAFIC: i32 =  53;
    pub const ORDERZONEBORDER: i32 =  54;
    pub const ORDERAIRBRIDGE: i32 =  55;
    pub CounterAlpha: i32;
    pub CounterAlpha2: i32;
    pub StratMap: Bitmap;
    pub MiniMap: Bitmap;
    pub MiniMapOffset: i32;
    pub ProdFlap: bool;
    pub InEditor: bool;
    pub ScrollDir: i32;
    pub Zoom: i32;
    pub CombatNumbers: bool;
    pub Screenshoton: bool;
    pub StartRdn: Vec<i32>;
    pub StartXp: Vec<i32>;
    pub StartMor: Vec<i32>;
    pub StartEntr: Vec<i32>;
    pub OverlayMode: i32;
    pub overlayOffsetX: i32;
    pub overlayOffsetY: i32;
    pub LoadFileName: String;
    pub LoadingFinished: bool;
    pub LoadType LoadingResult;
    pub LoadString: String;
    pub LoadCheckSum: String;
    pub LoadNoNewEdit: bool;
    pub NextTurnButtonPress: bool;
    pub OddsCalcFinished: bool;
    pub OddsCalcStarted: bool;
    pub ShowHQ: bool;
    pub SimpleEditWindow: i32;
    pub TempFileName: String;
    pub NewEnums.LibFileType TempFileType;
    pub allowMetrics: bool;
    pub askedMetricsPermission: bool;
    pub UDStabText: String;
    pub UDSbottomText: String;
    pub UDSpopupText: String;
    pub UDSpopupMode: i32;
    pub UDSinputCounter: i32;
    pub UDSinputKey: Vec<String>;
    pub UDSinputValue: Vec<String>;
    pub TempUDSinputCounter: i32;
    pub TempUDSinputKey: Vec<String>;
    pub TempUDSinputValue: Vec<String>;
    pub interfaceCue: i32;
    pub dssLastDominant: i32;
    pub dssLastEngineAction: i32;
    pub dssLastTrackId: i32;
    pub dssLastHighlightId: i32;
    pub layerLisAvailable: bool;
    pub layerLisUsed: bool;
    pub layerLisTotal: bool;
    pub layerLisBottlenecks: bool;
    pub layerLisPreview: bool;
    pub layerLisOnlyAssetId_isSupplyBase: bool;
    pub layerLisOnlyAssetId: i32;
    pub layerLisPreview_LogMode: i32;
    pub layerLis_LogType: i32;
    pub layerLis_TraficWindowOpen: i32;
    pub tempZoneTest: Vec<i32>;
    pub tempOtherTest: Vec<i32>;
    pub tempOtherTestText: Vec<String>;
    pub layerUnits: bool;
    pub inRandomScreen: bool;
    pub UDSpushedPopupText: String;
    pub UdsInsideTabOpenMode: bool;
    pub SimpleList udsInsideTabOpenModeList;
    pub TempFireListCacheSet: bool;
    pub resourceVpCacheSet: bool;
    pub WINDOW_DEBUG_MODE: bool;
    pub AutoDpiCheckDone: bool;
    pub Dc4_RightWindow_Expand1: bool;
    pub Dc4_RightWindow_Expand2: bool;
    pub Dc4_RightWindow_Expand3: bool;
    pub Dc4_RightWindow_Expand4: bool;
    pub Dc4_RightWindow_Expand5: bool;
    pub Dc4_RightWindow_Expand6: bool;
    pub Dc4_RightWindow_Expand7: bool;
    pub Dc4_RightWindow_Expand8: bool;
    pub Dc4_RightWindow_Expand9: bool;
    pub Dc4_Card_Select_Feedback: String;
    pub dc4_card_select_feedback_Color: Color;
    pub dc4_card_select_feedback_short: String;
    pub Dc4_temp_cardtab_cache: bool;
    pub dc4_temp_warning1done: bool;
    pub dc4_temp_warning2done: bool;
    pub dc4_last_supply_x: i32;
    pub dc4_last_supply_y: i32;
    pub WindowClass form3windowClass;
    pub quickInterceptCombat: bool;
    pub se1_map_data3cache_set: bool;
    pub skipGfxDetail: bool;

    pub fn UDSClearInput() => this.UDSinputCounter = -1;

    pub fn TempUDSClearInput() => this.TempUDSinputCounter = -1;

    pub fn UDSAddInput(tkey: String, tvalue: String)
    {
      this += 1.UDSinputCounter;
      this.UDSinputKey = (string[]) Utils.CopyArray((Array) this.UDSinputKey, (Array) new string[this.UDSinputCounter + 1 + 1]);
      this.UDSinputValue = (string[]) Utils.CopyArray((Array) this.UDSinputValue, (Array) new string[this.UDSinputCounter + 1 + 1]);
      this.UDSinputKey[this.UDSinputCounter] = tkey;
      this.UDSinputValue[this.UDSinputCounter] = tvalue;
    }

    pub fn TempUDSAddInput(tkey: String, tvalue: String)
    {
      this += 1.TempUDSinputCounter;
      this.TempUDSinputKey = (string[]) Utils.CopyArray((Array) this.TempUDSinputKey, (Array) new string[this.TempUDSinputCounter + 1 + 1]);
      this.TempUDSinputValue = (string[]) Utils.CopyArray((Array) this.TempUDSinputValue, (Array) new string[this.TempUDSinputCounter + 1 + 1]);
      this.TempUDSinputKey[this.TempUDSinputCounter] = tkey;
      this.TempUDSinputValue[this.TempUDSinputCounter] = tvalue;
    }

    pub fn UDSAddInput(tkey: String, tvalue: i32)
    {
      let mut udSinputCounter: i32 =  this.UDSinputCounter;
      for (let mut index: i32 =  0; index <= udSinputCounter; index += 1)
      {
        if (Operators.CompareString(this.UDSinputKey[index].ToLower(), tkey.ToLower(), false) == 0)
        {
          this.UDSinputValue[index] = Conversions.ToString(tvalue);
          return;
        }
      }
      this += 1.UDSinputCounter;
      this.UDSinputKey = (string[]) Utils.CopyArray((Array) this.UDSinputKey, (Array) new string[this.UDSinputCounter + 1 + 1]);
      this.UDSinputValue = (string[]) Utils.CopyArray((Array) this.UDSinputValue, (Array) new string[this.UDSinputCounter + 1 + 1]);
      this.UDSinputKey[this.UDSinputCounter] = tkey;
      this.UDSinputValue[this.UDSinputCounter] = tvalue.ToString();
    }

    pub EditClass()
    {
      this.AnswerText = new string[10];
      this.AnswerTextMouseOver = new string[10];
      this.TempValue = new MapMatrix2[1];
      this.TempValueSpecial = new MapMatrix2[1];
      this.TempValueSpecial2 = new MapMatrix2[1];
      this.TempValue2 = new MapMatrix2[1];
      this.TempValue3 = new MapMatrix2[1];
      this.TempValue4 = new MapMatrix2[1];
      this.TempAttack = new MapMatrix2Plus6[1];
      this.TempAI = new int[1, 1];
      this.TempAI2 = new int[1, 1];
      this.TempLos = new MapMatrix2[1];
      this.TempObstruct = new MapMatrix2[1];
      this.FuzzyPresumed = new MapMatrix2[1];
      this.FuzzyReal = new MapMatrix2[1];
      this.PossiblePull = new int[1, 1, 7];
      this.origPossiblePull = new int[1, 1, 7];
      this.tempGroupMoveUnr = new int[1];
      this.tempGroupMoveCameFrom = new CoordList[1];
      this.tempGroupMovePath = new CoordList[1];
      this.TempSup = new MapMatrix2[1];
      this.TempSupCameFrom = new MapMatrix2Coordinate[1];
      this.RemoveReinforce = new int[100];
      this.TempString = new MapMatrix2String[1];
      this.TempString2 = new MapMatrix2String[1];
      this.TempCameFrom = new MapMatrix2Coordinate[1];
      this.TempCoordList = CoordList::new();
      this.TempCoordListLastMove = CoordList::new();
      this.TempMovePathList = CoordList::new();
      this.TempUnitList = UnitList::new();
      this.HisOwner = new MapMatrix2[1];
      this.HisForce = new MapMatrix2[1];
      this.HisSFType = new MapMatrix2[1];
      this.HisHis = new MapMatrix2[1];
      this.HisDepth = new MapMatrix2[1];
      this.HisLossSFType = new int[1];
      this.HisLossAttacker = new int[1];
      this.HisLossOK = new int[1];
      this.HisLossDEAD = new int[1];
      this.HisLossSFType2 = new int[1];
      this.HisLossAttacker2 = new int[1];
      this.HisLossOK2 = new int[1];
      this.HisLossDEAD2 = new int[1];
      this.ServerCookieValue = "";
      this.PbemGameSetup = PbemGameSetupPhase.None;
      this.helpAlreadyOpened = false;
      this.systemPopup = false;
      this.highGfxSpeedOn = false;
      this.udsUnitOrderMode = 1;
      this.udsReturnFromPopup = false;
      this.udsLastCalledPopupEventNr = -1;
      this.udsManagementTabOverrideId = -1;
      this.udsViewMode2Override = -1;
      this.tempRenameString = "";
      this.tempRenameString2 = "";
      this.tempRenameString3 = "";
      this.uds_subtab = new int[10];
      this.uds_page = new int[10, 20];
      this.MiniMapOffset = 0;
      this.StartRdn = new int[2];
      this.StartXp = new int[2];
      this.StartMor = new int[2];
      this.StartEntr = new int[2];
      this.LoadNoNewEdit = false;
      this.NextTurnButtonPress = false;
      this.UDSinputKey = new string[1];
      this.UDSinputValue = new string[1];
      this.TempUDSinputKey = new string[1];
      this.TempUDSinputValue = new string[1];
      this.tempZoneTest = new int[1, 1];
      this.tempOtherTest = new int[1];
      this.tempOtherTestText = new string[1];
      this.UdsInsideTabOpenMode = false;
      this.TempFireListCacheSet = false;
      this.resourceVpCacheSet = false;
      this.WINDOW_DEBUG_MODE = false;
      this.AutoDpiCheckDone = false;
      this.Dc4_RightWindow_Expand1 = false;
      this.Dc4_RightWindow_Expand2 = false;
      this.Dc4_RightWindow_Expand3 = false;
      this.Dc4_RightWindow_Expand4 = false;
      this.Dc4_RightWindow_Expand5 = false;
      this.Dc4_RightWindow_Expand6 = false;
      this.Dc4_RightWindow_Expand7 = false;
      this.Dc4_RightWindow_Expand8 = false;
      this.Dc4_RightWindow_Expand9 = false;
      this.Dc4_Card_Select_Feedback = "";
      this.Dc4_temp_cardtab_cache = false;
      this.dc4_temp_warning1done = false;
      this.dc4_temp_warning2done = false;
      this.dc4_last_supply_x = -1;
      this.dc4_last_supply_y = -1;
      this.quickInterceptCombat = false;
      this.se1_map_data3cache_set = false;
      this.skipGfxDetail = false;
      this.initialSet();
    }

    pub fn initialSet()
    {
      this.DoCardSlot = -1;
      this.allowMetrics = false;
      this.mouseScreenLock = true;
      this.askedMetricsPermission = false;
      this.AreaSlot = -1;
      this.AreaValue = -1;
      this.TargetX = -1;
      this.TargetY = -1;
      this.AutoCombat = true;
      this.UnitSelected = -1;
      this.HideUnit = 1;
      this.OrderUnit = -1;
      this.TempHisUnit = -1;
      this.Zoom = 0;
      this.HandCard = -1;
      this.MapSelected = 0;
      this.TutOrder = -1;
      this.OldUnit = -1;
      this.TutX =  -1;
      this.TutY =  -1;
      this.overruleScreenResWidth = -1;
      this.overruleScreenResHeight = -1;
      this.TutMode = false;
      this.ButtonLoadMode = false;
      this.TutStep = 0;
      this.Volume = 50;
      this.Volume2 = 50;
      this.Screenshoton = false;
      this.Layout = 0;
      this.ShowLabel = true;
      this.ShowBase = true;
      this.TextInputString = "";
      this.ServerCookieValue = "";
      this.ShowUnderHQ = true;
      this.ShowSameHistorical = true;
      this.ShowMouseOver = false;
      this.PbemUserName = "";
      this.PbemSerial = "";
      this.PbemPassword = "";
      this.DoubleSize = false;
      this.DoubleSizePercentage = 200;
      this.pdfGenerated = "";
      this.AlternateMusic = false;
      this.UDSinputCounter = -1;
      this.dssLastDominant = -1;
      this.dssLastEngineAction = -1;
      this.dssLastHighlightId = -1;
      this.dssLastTrackId = -1;
      this.highGfxSpeedOn = false;
      this.layerUnits = true;
      this.inRandomScreen = false;
      this.ShowLISRange = false;
      this.useLeftRightClickMode = false;
      this.useLeftRightClickMode = true;
      this.dontShowAImoves = false;
      this.showAdvice = true;
      this.usePullAssets = true;
      this.usePullCities = true;
      this.usePullUnits = true;
      this.AutoDpiCheckDone = false;
      this.quickInterceptCombat = false;
      this.skipGfxDetail = false;
    }

    pub EditClass(filename: String)
    {
      this.AnswerText = new string[10];
      this.AnswerTextMouseOver = new string[10];
      this.TempValue = new MapMatrix2[1];
      this.TempValueSpecial = new MapMatrix2[1];
      this.TempValueSpecial2 = new MapMatrix2[1];
      this.TempValue2 = new MapMatrix2[1];
      this.TempValue3 = new MapMatrix2[1];
      this.TempValue4 = new MapMatrix2[1];
      this.TempAttack = new MapMatrix2Plus6[1];
      this.TempAI = new int[1, 1];
      this.TempAI2 = new int[1, 1];
      this.TempLos = new MapMatrix2[1];
      this.TempObstruct = new MapMatrix2[1];
      this.FuzzyPresumed = new MapMatrix2[1];
      this.FuzzyReal = new MapMatrix2[1];
      this.PossiblePull = new int[1, 1, 7];
      this.origPossiblePull = new int[1, 1, 7];
      this.tempGroupMoveUnr = new int[1];
      this.tempGroupMoveCameFrom = new CoordList[1];
      this.tempGroupMovePath = new CoordList[1];
      this.TempSup = new MapMatrix2[1];
      this.TempSupCameFrom = new MapMatrix2Coordinate[1];
      this.RemoveReinforce = new int[100];
      this.TempString = new MapMatrix2String[1];
      this.TempString2 = new MapMatrix2String[1];
      this.TempCameFrom = new MapMatrix2Coordinate[1];
      this.TempCoordList = CoordList::new();
      this.TempCoordListLastMove = CoordList::new();
      this.TempMovePathList = CoordList::new();
      this.TempUnitList = UnitList::new();
      this.HisOwner = new MapMatrix2[1];
      this.HisForce = new MapMatrix2[1];
      this.HisSFType = new MapMatrix2[1];
      this.HisHis = new MapMatrix2[1];
      this.HisDepth = new MapMatrix2[1];
      this.HisLossSFType = new int[1];
      this.HisLossAttacker = new int[1];
      this.HisLossOK = new int[1];
      this.HisLossDEAD = new int[1];
      this.HisLossSFType2 = new int[1];
      this.HisLossAttacker2 = new int[1];
      this.HisLossOK2 = new int[1];
      this.HisLossDEAD2 = new int[1];
      this.ServerCookieValue = "";
      this.PbemGameSetup = PbemGameSetupPhase.None;
      this.helpAlreadyOpened = false;
      this.systemPopup = false;
      this.highGfxSpeedOn = false;
      this.udsUnitOrderMode = 1;
      this.udsReturnFromPopup = false;
      this.udsLastCalledPopupEventNr = -1;
      this.udsManagementTabOverrideId = -1;
      this.udsViewMode2Override = -1;
      this.tempRenameString = "";
      this.tempRenameString2 = "";
      this.tempRenameString3 = "";
      this.uds_subtab = new int[10];
      this.uds_page = new int[10, 20];
      this.MiniMapOffset = 0;
      this.StartRdn = new int[2];
      this.StartXp = new int[2];
      this.StartMor = new int[2];
      this.StartEntr = new int[2];
      this.LoadNoNewEdit = false;
      this.NextTurnButtonPress = false;
      this.UDSinputKey = new string[1];
      this.UDSinputValue = new string[1];
      this.TempUDSinputKey = new string[1];
      this.TempUDSinputValue = new string[1];
      this.tempZoneTest = new int[1, 1];
      this.tempOtherTest = new int[1];
      this.tempOtherTestText = new string[1];
      this.UdsInsideTabOpenMode = false;
      this.TempFireListCacheSet = false;
      this.resourceVpCacheSet = false;
      this.WINDOW_DEBUG_MODE = false;
      this.AutoDpiCheckDone = false;
      this.Dc4_RightWindow_Expand1 = false;
      this.Dc4_RightWindow_Expand2 = false;
      this.Dc4_RightWindow_Expand3 = false;
      this.Dc4_RightWindow_Expand4 = false;
      this.Dc4_RightWindow_Expand5 = false;
      this.Dc4_RightWindow_Expand6 = false;
      this.Dc4_RightWindow_Expand7 = false;
      this.Dc4_RightWindow_Expand8 = false;
      this.Dc4_RightWindow_Expand9 = false;
      this.Dc4_Card_Select_Feedback = "";
      this.Dc4_temp_cardtab_cache = false;
      this.dc4_temp_warning1done = false;
      this.dc4_temp_warning2done = false;
      this.dc4_last_supply_x = -1;
      this.dc4_last_supply_y = -1;
      this.quickInterceptCombat = false;
      this.se1_map_data3cache_set = false;
      this.skipGfxDetail = false;
      this.initialSet();
      this.PencilType = 0;
      this.PrefShowFOW = true;
      this.OrderType = 0;
      this.maximumInterfaceSpace = false;
      this.UnitSelected = -1;
      this.TargetX = -1;
      this.Zoom = 0;
      this.TargetY = -1;
      this.MiniMap = new Bitmap(200, 150);
      this.MiniMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      this.ShownWelcome = true;
      this.PaintShortcut1 = -1;
      this.PaintShortcut2 = -1;
      this.overruleScreenResWidth = -1;
      this.overruleScreenResHeight = -1;
      this.PaintShortcut3 = -1;
      this.SoundOn = true;
      this.AreaSlot = -1;
      this.OldUnit = -1;
      this.HideUnit = 1;
      this.OrderUnit = -1;
      this.AreaValue = -1;
      this.Volume = 50;
      this.Volume2 = 50;
      this.DoCardSlot = -1;
      this.mouseScreenLock = true;
      this.TutOrder = -1;
      this.TutMode = false;
      this.ButtonLoadMode = false;
      this.TutStep = 0;
      this.TextInputString = "";
      this.TutX =  -1;
      this.TutY =  -1;
      this.Layout = 0;
      this.TempHisUnit = -1;
      this.HandCard = -1;
      this.MapSelected = 0;
      this.CampaignRoomBitmap = -1;
      this.ServerCookieValue = "";
      this.PbemUserName = "";
      this.PbemSerial = "";
      this.PbemPassword = "";
      this.PbemEmail = "";
      this.allowMetrics = false;
      this.askedMetricsPermission = false;
      this.pdfGenerated = "";
      this.AlternateMusic = false;
      this.UDSinputCounter = -1;
      this.dssLastDominant = -1;
      this.dssLastEngineAction = -1;
      this.dssLastHighlightId = -1;
      this.dssLastTrackId = -1;
      this.layerUnits = true;
      this.inRandomScreen = false;
      this.ShowLISRange = false;
      this.dontShowAImoves = false;
      this.usePullAssets = true;
      this.usePullCities = true;
      this.usePullUnits = true;
      this.showAdvice = true;
      this.AutoDpiCheckDone = false;
      this.quickInterceptCombat = false;
      this.skipGfxDetail = false;
      if (File.Exists(filename))
      {
        StreamReader streamReader;
        try
        {
          streamReader = File.OpenText(filename);
          this.PrefShowFOW = Conversions.ToBoolean(streamReader.ReadLine());
          this.HexRasterOn = Conversions.ToBoolean(streamReader.ReadLine());
          this.HideAS = Conversions.ToBoolean(streamReader.ReadLine());
          try
          {
            this.SoundOn = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.SoundOn = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.IntroSoundOn = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.IntroSoundOn = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.AutoSave = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.AutoSave = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.StartWithHistory = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.StartWithHistory = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.Zoom = Conversions.ToInteger(streamReader.ReadLine());
            if (this.Zoom == -1)
              this.Zoom = 0;
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.Zoom = 0;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.SpreadUnit = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.SpreadUnit = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.Screenshoton = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.Screenshoton = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.CombatNumbers = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.CombatNumbers = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.Layout = Conversions.ToInteger(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.Layout = 0;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.RegimeColoring = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.RegimeColoring = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.TownInfo = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.TownInfo = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.ShowBase = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.ShowBase = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.ShowLabel = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.ShowLabel = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.PbemUserName = streamReader.ReadLine();
            this.PbemPassword = streamReader.ReadLine();
            this.PbemSerial = streamReader.ReadLine();
            this.PbemEmail = streamReader.ReadLine();
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.PbemUserName = "";
            this.PbemPassword = "";
            this.PbemEmail = "";
            ProjectData.ClearProjectError();
          }
          try
          {
            this.ShowUnderHQ = Conversions.ToBoolean(streamReader.ReadLine());
            this.ShowSameHistorical = Conversions.ToBoolean(streamReader.ReadLine());
            this.ShowMouseOver = Conversions.ToBoolean(streamReader.ReadLine());
            this.ShowHQPower = Conversions.ToBoolean(streamReader.ReadLine());
            this.ShowAirRange = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.ShowUnderHQ = true;
            this.ShowSameHistorical = true;
            this.ShowMouseOver = true;
            this.ShowHQPower = false;
            this.ShowAirRange = false;
            ProjectData.ClearProjectError();
          }
          if (Information.IsNothing( this.PbemUserName))
            this.PbemUserName = "";
          if (Information.IsNothing( this.PbemPassword))
            this.PbemPassword = "";
          if (Information.IsNothing( this.PbemEmail))
            this.PbemEmail = "";
          try
          {
            str: String = streamReader.ReadLine();
            this.Volume = !Information.IsNothing( str) ?  Math.Round(Conversion.Val(str)) : 50;
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.Volume = 50;
            ProjectData.ClearProjectError();
          }
          try
          {
            str: String = streamReader.ReadLine();
            this.Volume2 = !Information.IsNothing( str) ?  Math.Round(Conversion.Val(str)) : 50;
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.Volume2 = 50;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.PrefMinimalistCounter = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.PrefMinimalistCounter = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.DoubleSize = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.DoubleSize = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.allowMetrics = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.allowMetrics = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.HideUnit = Conversions.ToInteger(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.HideUnit = 0;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.AlternateMusic = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.AlternateMusic = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.askedMetricsPermission = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.askedMetricsPermission = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.DoubleSizePercentage = Conversions.ToInteger(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.DoubleSizePercentage = 200;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.PbemSteam = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.PbemSteam = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.highGfxSpeedOn = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.highGfxSpeedOn = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.CombatTextual = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.CombatTextual = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.ShowLISRange = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.ShowLISRange = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.AutoCombat = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.AutoCombat = true;
            ProjectData.ClearProjectError();
          }
          if (this.DoubleSizePercentage < 100)
            this.DoubleSizePercentage = 100;
          if (this.DoubleSize)
          {
            if (this.DoubleSizePercentage < 125)
              this.DoubleSizePercentage = 125;
          }
          try
          {
            this.maximumInterfaceSpace = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.maximumInterfaceSpace = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.useLeftRightClickMode = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.useLeftRightClickMode = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.overruleScreenResWidth = Conversions.ToInteger(streamReader.ReadLine());
            this.overruleScreenResHeight = Conversions.ToInteger(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.overruleScreenResWidth = -1;
            this.overruleScreenResHeight = -1;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.BlockAdvice = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.BlockAdvice = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.mouseScreenLock = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.mouseScreenLock = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.showAdvice = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.showAdvice = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.dontShowAImoves = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.dontShowAImoves = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.usePullAssets = Conversions.ToBoolean(streamReader.ReadLine());
            this.usePullCities = Conversions.ToBoolean(streamReader.ReadLine());
            this.usePullUnits = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.usePullAssets = true;
            this.usePullCities = true;
            this.usePullUnits = true;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.AutoDpiCheckDone = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.AutoDpiCheckDone = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.quickInterceptCombat = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.quickInterceptCombat = false;
            ProjectData.ClearProjectError();
          }
          try
          {
            this.skipGfxDetail = Conversions.ToBoolean(streamReader.ReadLine());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            this.skipGfxDetail = false;
            ProjectData.ClearProjectError();
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          Exception exception = ex;
          this.initialSet();
          let mut num: i32 =   Interaction.MsgBox( ("Prefs loading issue: '" + exception.Message + "'.. you can continue game however"), Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.ClearProjectError();
        }
        streamReader.Close();
      }
      else
      {
        let mut num1: i32 =   Interaction.MsgBox( "Prefs not found and could not be loaded at all. you can continue game however", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      if (this.PbemSerial.Length < 19)
        this.PbemSerial = "0000-0000-0000-0000";
      this.PbemSteam = true;
      this.StartWithHistory = false;
      this.useLeftRightClickMode = true;
    }

    pub fn Save(filename: String)
    {
      StreamWriter text = File.CreateText(filename);
      text.WriteLine(this.PrefShowFOW);
      text.WriteLine(this.HexRasterOn);
      text.WriteLine(this.HideAS);
      text.WriteLine(this.SoundOn);
      text.WriteLine(this.IntroSoundOn);
      text.WriteLine(this.AutoSave);
      text.WriteLine(this.StartWithHistory);
      text.WriteLine(this.Zoom);
      text.WriteLine(this.SpreadUnit);
      text.WriteLine(this.Screenshoton);
      text.WriteLine(this.CombatNumbers);
      text.WriteLine(this.Layout);
      text.WriteLine(this.RegimeColoring);
      text.WriteLine(this.TownInfo);
      text.WriteLine(this.ShowBase);
      text.WriteLine(this.ShowLabel);
      text.WriteLine(this.PbemUserName);
      text.WriteLine(this.PbemPassword);
      text.WriteLine(this.PbemSerial);
      text.WriteLine(this.PbemEmail);
      text.WriteLine(this.ShowUnderHQ);
      text.WriteLine(this.ShowSameHistorical);
      text.WriteLine(this.ShowMouseOver);
      text.WriteLine(this.ShowHQPower);
      text.WriteLine(this.ShowAirRange);
      text.WriteLine(this.Volume);
      text.WriteLine(this.Volume2);
      text.WriteLine(this.PrefMinimalistCounter);
      text.WriteLine(this.DoubleSize);
      text.WriteLine(this.allowMetrics);
      text.WriteLine(this.HideUnit);
      text.WriteLine(this.AlternateMusic);
      text.WriteLine(this.askedMetricsPermission);
      text.WriteLine(this.DoubleSizePercentage);
      text.WriteLine(this.PbemSteam);
      text.WriteLine(this.highGfxSpeedOn);
      text.WriteLine(this.CombatTextual);
      text.WriteLine(this.ShowLISRange);
      text.WriteLine(this.AutoCombat);
      text.WriteLine(this.maximumInterfaceSpace);
      text.WriteLine(this.useLeftRightClickMode);
      text.WriteLine(this.overruleScreenResWidth);
      text.WriteLine(this.overruleScreenResHeight);
      text.WriteLine(this.BlockAdvice);
      text.WriteLine(this.mouseScreenLock);
      text.WriteLine(this.showAdvice);
      text.WriteLine(this.dontShowAImoves);
      text.WriteLine(this.usePullAssets);
      text.WriteLine(this.usePullCities);
      text.WriteLine(this.usePullUnits);
      text.WriteLine(this.AutoDpiCheckDone);
      text.WriteLine(this.quickInterceptCombat);
      text.WriteLine(this.skipGfxDetail);
      text.Close();
    }

    pub delegate void AfterPopUpRefresh();
  }
}
