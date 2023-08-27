// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RegimeClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class RegimeClass : ISerializable
  {
    pub Name: String;
    pub id: i32;
    pub Red: i32;
    pub Green: i32;
    pub Blue: i32;
    pub Red2: i32;
    pub Green2: i32;
    pub Blue2: i32;
    pub Red3: i32;
    pub Green3: i32;
    pub Blue3: i32;
    pub Red4: i32;
    pub Green4: i32;
    pub Blue4: i32;
    pub People: i32;
    pub RegimeSlot: Vec<i32>;
    pub LastTempRegimeSlotPredict: Vec<i32>;
    pub TempRegimeSlotPredict: Vec<i32>;
    pub TempRegimeSlotIncrease: Vec<i32>;
    pub TempPPIncrease: i32;
    pub HQSpriteOverrule: bool;
    pub BaseMorale: i32;
    pub ResPts: i32;
    pub ResFieldCounter: i32;
    pub ResField: Vec<bool>;
    pub RegimeCounter: i32;
    pub RegimeRel: Vec<i32>;
    pub RegimeOffer: Vec<i32>;
    pub MapCount: i32;
    pub MapMatrix2[] HistoryOwner;
    pub MapMatrix2[] HistoryForce;
    pub MapMatrix2[] HistorySFType;
    pub MapMatrix2[] HistoryHis;
    pub MapMatrix2[] HistoryDepth;
    pub MapMatrix2[] Trafic;
    pub MapMatrix2[] Trafic2;
    pub MapMatrix2[] AIVP;
    pub MapMatrix2[] AIPower;
    pub MapMatrix2[] AIDefense;
    pub AIStance: i32;
    pub MapMatrix2[] OldAINarrow;
    pub AIHelpMove: i32;
    pub AIHelpCombat: i32;
    pub AIHelpStrategic: i32;
    pub HistoryStepClass[] HistoryStep;
    pub HistoryStepCounter: i32;
    pub MessCounter: i32;
    pub MessString: Vec<String>;
    pub MessBackPic: Vec<i32>;
    pub MessFrontPic: Vec<i32>;
    pub MessWav: Vec<String>;
    pub MesStyle: Vec<i32>;
    pub MesNote: Vec<String>;
    pub MesNote2: Vec<String>;
    pub MesName: Vec<String>;
    pub MesGroup: Vec<i32>;
    pub MesHideFromStart: Vec<bool>;
    pub MesHideFromTab: Vec<bool>;
    pub MesChosen: Vec<i32>;
    pub AI: bool;
    pub AIid: i32;
    pub Sleep: bool;
    pub PassWord: String;
    pub UnitName: String;
    pub UnitNumber: i32;
    pub HQName: String;
    pub HQNumber: i32;
    pub SLoss: Vec<i32>;
    pub SKills: Vec<i32>;
    pub SProd: Vec<i32>;
    pub SPresent: Vec<i32>;
    pub int[,,] LisPoints;
    pub HexBack: Bitmap;
    pub PlanCounter: i32;
    pub PlanObj: Vec<AIPlanClass>;
    pub TempCounter: Bitmap;
    pub TempCounterHigh: Bitmap;
    pub TempCounterBig: Bitmap;
    pub TempCounterBigHigh: Bitmap;
    pub TempCountersmall: Bitmap;
    pub TempCountersmallHigh: Bitmap;
    pub TempRegimeColor: Bitmap;
    pub TempRegimeColorSmall: Bitmap;
    pub TempRegimeColorBig: Bitmap;
    pub DipBlock: bool;
    pub SASSupplyLost: i32;
    pub SASSupplyKilled: i32;
    pub SASProdLost: Vec<i32>;
    pub SASKilled: Vec<i32>;
    pub AISavedPP: i32;
    pub float AIConservative;
    pub ProdBonus: i32;
    pub ActionCardCounter: i32;
    pub ActionCard: Vec<i32>;
    pub ActionCardHistoryCounter: i32;
    pub ActionCardHistory: Vec<i32>;
    pub ActionCardHistoryRound: Vec<i32>;
    pub ExtraStat: Vec<i32>;
    pub NationalIconSpriteName: String;
    pub NationalIconSprite: i32;
    pub RoundelSpriteName: String;
    pub RoundelIconSprite: i32;
    pub RoundelSpriteName2: String;
    pub RoundelIconSprite2: i32;
    pub HQSpriteFileName: String;
    pub HQSpriteNr: i32;
    pub HQSpriteFileName2: String;
    pub HQSpriteNr2: i32;
    pub BannerSpriteFileName: String;
    pub BannerSpriteNr: i32;
    pub BannerSpriteFileName2: String;
    pub BannerSpriteNr2: i32;
    pub SymbolSpriteName: String;
    pub SymbolSpriteNr: i32;
    pub ExtraGraphicUse: i32;
    pub FirstRound: i32;
    pub LoadTransferUnit: bool;
    pub LoadTransferHistorical: bool;
    pub LoadAreaSlot: i32;
    pub LoadPerHex: i32;
    pub LoadResearchTransfer: bool;
    pub RandomCode: i32;
    pub LastVersion: i32;
    pub OfficerPool: i32;
    pub AIGroupCounter: i32;
    pub AIGroupName: Vec<String>;
    pub AIGroupType: Vec<i32>;
    pub AIGroupHis: Vec<i32>;
    pub AIGroupLastAttack: Vec<i32>;
    pub AIGroupLastDefend: Vec<i32>;
    pub AIGroupLastFollowUp: Vec<i32>;
    pub AIGroupLastFallBack: Vec<i32>;
    pub AICorpsTopGroup: Vec<i32>;
    pub UberRegime: i32;
    pub Mirror: bool;
    pub Version: i32;
    pub subVersion: String;
    pub PbemPlayer: i32;
    pub UseAlternateActionCardPics: bool;
    pub FerryEffectivity: i32;
    pub hideFromList: bool;
    pub LibIdClass libId;
    pub TempSelectable: bool;
    pub minimumDataUsage: bool;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("Red", self.Red);
      info.AddValue("Green", self.Green);
      info.AddValue("Blue", self.Blue);
      info.AddValue("Red2", self.Red2);
      info.AddValue("Green2", self.Green2);
      info.AddValue("Blue2", self.Blue2);
      info.AddValue("Red3", self.Red3);
      info.AddValue("Green3", self.Green3);
      info.AddValue("Blue3", self.Blue3);
      info.AddValue("Red4", self.Red4);
      info.AddValue("Green4", self.Green4);
      info.AddValue("Blue4", self.Blue4);
      info.AddValue("People", self.People);
      info.AddValue("RegimeSlot",  self.RegimeSlot);
      info.AddValue("HQSpriteFileName",  self.HQSpriteFileName);
      info.AddValue("HQSpriteFileName2",  self.HQSpriteFileName2);
      info.AddValue("BaseMorale", self.BaseMorale);
      info.AddValue("ResPts", self.ResPts);
      info.AddValue("ResFieldCounter", self.ResFieldCounter);
      info.AddValue("ResField",  self.ResField);
      info.AddValue("RegimeCounter", self.RegimeCounter);
      info.AddValue("RegimeRel",  self.RegimeRel);
      info.AddValue("RegimeOffer",  self.RegimeOffer);
      info.AddValue("MapCount", self.MapCount);
      info.AddValue("HistoryOwner",  self.HistoryOwner);
      info.AddValue("HistorySFType",  self.HistorySFType);
      info.AddValue("HistoryForce",  self.HistoryForce);
      info.AddValue("HistoryHis",  self.HistoryHis);
      info.AddValue("HistoryDepth",  self.HistoryDepth);
      info.AddValue("HistoryStep",  self.HistoryStep);
      info.AddValue("HistoryStepCounter", self.HistoryStepCounter);
      info.AddValue("MessCounter", self.MessCounter);
      info.AddValue("MessString",  self.MessString);
      info.AddValue("MessBackPic",  self.MessBackPic);
      info.AddValue("MessFrontPic",  self.MessFrontPic);
      info.AddValue("MessWav",  self.MessWav);
      info.AddValue("MesStyle",  self.MesStyle);
      info.AddValue("MesNote",  self.MesNote);
      info.AddValue("MesNote2",  self.MesNote2);
      info.AddValue("MesName",  self.MesName);
      info.AddValue("MesGroup",  self.MesGroup);
      info.AddValue("MesChosen",  self.MesChosen);
      info.AddValue("MesHideFromStart",  self.MesHideFromStart);
      info.AddValue("MesHideFromTab",  self.MesHideFromTab);
      info.AddValue("AI", self.AI);
      info.AddValue("AIid", self.AIid);
      info.AddValue("Sleep", self.Sleep);
      info.AddValue("PassWord",  self.PassWord);
      info.AddValue("UnitName",  self.UnitName);
      info.AddValue("UnitNumber", self.UnitNumber);
      info.AddValue("HQName",  self.HQName);
      info.AddValue("HQNumber", self.HQNumber);
      info.AddValue("SLoss",  self.SLoss);
      info.AddValue("SKills",  self.SKills);
      info.AddValue("SProd",  self.SProd);
      info.AddValue("SPresent",  self.SPresent);
      info.AddValue("DipBlock", self.DipBlock);
      info.AddValue("SASSupplyLost", self.SASSupplyLost);
      info.AddValue("SASSupplyKilled", self.SASSupplyKilled);
      info.AddValue("SASProdLost",  self.SASProdLost);
      info.AddValue("SASKilled",  self.SASKilled);
      info.AddValue("HQSpriteOverrule", self.HQSpriteOverrule);
      info.AddValue("PlanCounter", self.PlanCounter);
      info.AddValue("PlanObj",  self.PlanObj);
      info.AddValue("ProdBonus", self.ProdBonus);
      info.AddValue("AIVP",  self.AIVP);
      info.AddValue("Trafic",  self.Trafic);
      info.AddValue("Trafic2",  self.Trafic2);
      info.AddValue("AIPower",  self.AIPower);
      info.AddValue("AIDefense",  self.AIDefense);
      info.AddValue("ActionCardCounter", self.ActionCardCounter);
      info.AddValue("ActionCard",  self.ActionCard);
      info.AddValue("ActionCardHistoryCounter", self.ActionCardHistoryCounter);
      info.AddValue("ActionCardHistory",  self.ActionCardHistory);
      info.AddValue("ActionCardHistoryRound",  self.ActionCardHistoryRound);
      info.AddValue("AIConservative", self.AIConservative);
      info.AddValue("ExtraStat",  self.ExtraStat);
      info.AddValue("NationalIconSpriteName",  self.NationalIconSpriteName);
      info.AddValue("RoundelSpriteName",  self.RoundelSpriteName);
      info.AddValue("RoundelSpriteName2",  self.RoundelSpriteName2);
      info.AddValue("BannerSpriteFileName",  self.BannerSpriteFileName);
      info.AddValue("BannerSpriteFileName2",  self.BannerSpriteFileName2);
      info.AddValue("SymbolSpriteName",  self.SymbolSpriteName);
      info.AddValue("ExtraGraphicUse", self.ExtraGraphicUse);
      info.AddValue("FirstRound", self.FirstRound);
      info.AddValue("RandomCode", self.RandomCode);
      info.AddValue("AIGroupCounter", self.AIGroupCounter);
      info.AddValue("AIGroupHis",  self.AIGroupHis);
      info.AddValue("AIGroupType",  self.AIGroupType);
      info.AddValue("AIGroupLastAttack",  self.AIGroupLastAttack);
      info.AddValue("AIGroupLastDefend",  self.AIGroupLastDefend);
      info.AddValue("AIGroupLastFollowUp",  self.AIGroupLastFollowUp);
      info.AddValue("AIGroupLastFallBack",  self.AIGroupLastFallBack);
      info.AddValue("AICorpsTopGroup",  self.AICorpsTopGroup);
      info.AddValue("AIGroupName",  self.AIGroupName);
      info.AddValue("OfficerPool", self.OfficerPool);
      info.AddValue("UberRegime", self.UberRegime);
      info.AddValue("Mirror", self.Mirror);
      info.AddValue("Version", self.Version);
      info.AddValue("subVersion",  self.subVersion);
      info.AddValue("LoadTransferHistorical", self.LoadTransferHistorical);
      info.AddValue("OldAINarrow",  self.OldAINarrow);
      info.AddValue("AIHelpMove", self.AIHelpMove);
      info.AddValue("AIHelpCombat", self.AIHelpCombat);
      info.AddValue("AIHelpStrategic", self.AIHelpStrategic);
      info.AddValue("PBEMPlayer", self.PbemPlayer);
      info.AddValue("UseAlternateActionCardPics", self.UseAlternateActionCardPics);
      info.AddValue("FerryEffectivity", self.FerryEffectivity);
      info.AddValue("LibId",  self.libId);
      info.AddValue("Id", self.id);
      info.AddValue("hideFromList", self.hideFromList);
      info.AddValue("minimumDataUsage", self.minimumDataUsage);
    }

    pub fn AddPlan()
    {
      this += 1.PlanCounter;
      self.PlanObj = (AIPlanClass[]) Utils.CopyArray((Array) self.PlanObj, (Array) new AIPlanClass[self.PlanCounter + 1]);
      self.PlanObj[self.PlanCounter] = AIPlanClass::new();
    }

    pub fn RemovePlan(nr: i32)
    {
      if (nr < self.PlanCounter)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.PlanCounter - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
          self.PlanObj[index] = self.PlanObj[index + 1];
      }
      --self.PlanCounter;
      self.PlanObj = (AIPlanClass[]) Utils.CopyArray((Array) self.PlanObj, (Array) new AIPlanClass[self.PlanCounter + 1]);
    }

    protected RegimeClass(SerializationInfo info, StreamingContext context)
    {
      self.RegimeSlot = new int[500];
      self.LastTempRegimeSlotPredict = new int[500];
      self.TempRegimeSlotPredict = new int[500];
      self.TempRegimeSlotIncrease = new int[500];
      self.ResField = new bool[1];
      self.RegimeRel = new int[1];
      self.RegimeOffer = new int[1];
      self.HistoryOwner = new MapMatrix2[1];
      self.HistoryForce = new MapMatrix2[1];
      self.HistorySFType = new MapMatrix2[1];
      self.HistoryHis = new MapMatrix2[1];
      self.HistoryDepth = new MapMatrix2[1];
      self.Trafic = new MapMatrix2[1];
      self.Trafic2 = new MapMatrix2[1];
      self.AIVP = new MapMatrix2[1];
      self.AIPower = new MapMatrix2[1];
      self.AIDefense = new MapMatrix2[1];
      self.OldAINarrow = new MapMatrix2[1];
      self.HistoryStep = new HistoryStepClass[1];
      self.MessString = new string[1];
      self.MessBackPic = new int[1];
      self.MessFrontPic = new int[1];
      self.MessWav = new string[1];
      self.MesStyle = new int[1];
      self.MesNote = new string[1];
      self.MesNote2 = new string[1];
      self.MesName = new string[1];
      self.MesGroup = new int[1];
      self.MesHideFromStart = new bool[1];
      self.MesHideFromTab = new bool[1];
      self.MesChosen = new int[1];
      self.SLoss = new int[1, 1];
      self.SKills = new int[1, 1];
      self.SProd = new int[1, 1];
      self.SPresent = new int[1, 1];
      self.LisPoints = new int[1, 1, 7];
      self.PlanObj = new AIPlanClass[1];
      self.SASProdLost = new int[1];
      self.SASKilled = new int[1];
      self.ActionCard = new int[1];
      self.ActionCardHistory = new int[1];
      self.ActionCardHistoryRound = new int[1];
      self.ExtraStat = new int[3, 1];
      self.AIGroupName = new string[1];
      self.AIGroupType = new int[1];
      self.AIGroupHis = new int[1];
      self.AIGroupLastAttack = new int[1];
      self.AIGroupLastDefend = new int[1];
      self.AIGroupLastFollowUp = new int[1];
      self.AIGroupLastFallBack = new int[1];
      self.AICorpsTopGroup = new int[1];
      self.Name = info.GetString(nameof (Name));
      self.Red = info.GetInt32(nameof (Red));
      self.Green = info.GetInt32(nameof (Green));
      self.Blue = info.GetInt32(nameof (Blue));
      self.Red2 = info.GetInt32(nameof (Red2));
      self.Green2 = info.GetInt32(nameof (Green2));
      self.Blue2 = info.GetInt32(nameof (Blue2));
      try
      {
        self.Red3 = info.GetInt32(nameof (Red3));
        self.Green3 = info.GetInt32(nameof (Green3));
        self.Blue3 = info.GetInt32(nameof (Blue3));
        self.Red4 = info.GetInt32(nameof (Red4));
        self.Green4 = info.GetInt32(nameof (Green4));
        self.Blue4 = info.GetInt32(nameof (Blue4));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      self.People = info.GetInt32(nameof (People));
      self.RegimeSlot = (int[]) info.GetValue(nameof (RegimeSlot), self.RegimeSlot.GetType());
      self.HQSpriteFileName = info.GetString(nameof (HQSpriteFileName));
      try
      {
        self.HQSpriteFileName2 = info.GetString(nameof (HQSpriteFileName2));
        self.BannerSpriteFileName = info.GetString(nameof (BannerSpriteFileName));
        self.BannerSpriteFileName2 = info.GetString(nameof (BannerSpriteFileName2));
        self.SymbolSpriteName = info.GetString(nameof (SymbolSpriteName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.HQSpriteFileName2 = "systemgraphics/trans.bmp";
        self.BannerSpriteFileName = "systemgraphics/trans.bmp";
        self.BannerSpriteFileName2 = "systemgraphics/trans.bmp";
        self.SymbolSpriteName = "systemgraphics/trans.bmp";
        ProjectData.ClearProjectError();
      }
      self.BaseMorale = info.GetInt32(nameof (BaseMorale));
      self.ResPts = info.GetInt32(nameof (ResPts));
      self.ResFieldCounter = info.GetInt32(nameof (ResFieldCounter));
      self.ResField = self.ResFieldCounter <= -1 ? new bool[1] : new bool[self.ResFieldCounter + 1];
      self.ResField = (bool[]) info.GetValue(nameof (ResField), self.ResField.GetType());
      self.RegimeCounter = info.GetInt32(nameof (RegimeCounter));
      if (self.RegimeCounter > -1)
      {
        self.RegimeRel = new int[self.RegimeCounter + 1];
        self.RegimeOffer = new int[self.RegimeCounter + 1];
      }
      else
      {
        self.RegimeRel = new int[1];
        self.RegimeOffer = new int[1];
      }
      self.RegimeRel = (int[]) info.GetValue(nameof (RegimeRel), self.RegimeRel.GetType());
      try
      {
        self.RegimeOffer = (int[]) info.GetValue(nameof (RegimeOffer), self.RegimeOffer.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      self.AI = info.GetBoolean(nameof (AI));
      self.Sleep = info.GetBoolean(nameof (Sleep));
      self.HistoryStepCounter = info.GetInt32(nameof (HistoryStepCounter));
      self.HistoryStep = self.HistoryStepCounter <= -1 ? new HistoryStepClass[1] : new HistoryStepClass[self.HistoryStepCounter + 1];
      self.HistoryStep = (HistoryStepClass[]) info.GetValue(nameof (HistoryStep), self.HistoryStep.GetType());
      try
      {
        self.UberRegime = info.GetInt32(nameof (UberRegime));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.UberRegime = -1;
        ProjectData.ClearProjectError();
      }
      let mut mapWidth: i32 = DrawMod.TGame.Data.MapWidth;
      let mut mapHeight: i32 = DrawMod.TGame.Data.MapHeight;
      numArray1: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
      try
      {
        self.MapCount = info.GetInt32(nameof (MapCount));
        self.HistoryOwner = new MapMatrix2[self.MapCount + 1];
        self.HistoryForce = new MapMatrix2[self.MapCount + 1];
        self.HistorySFType = new MapMatrix2[self.MapCount + 1];
        self.HistoryHis = new MapMatrix2[self.MapCount + 1];
        self.HistoryDepth = new MapMatrix2[self.MapCount + 1];
        self.AIVP = new MapMatrix2[self.MapCount + 1];
        self.AIPower = new MapMatrix2[self.MapCount + 1];
        self.AIDefense = new MapMatrix2[self.MapCount + 1];
        self.HistoryOwner = (MapMatrix2[]) info.GetValue(nameof (HistoryOwner), self.HistoryOwner.GetType());
        self.HistoryForce = (MapMatrix2[]) info.GetValue(nameof (HistoryForce), self.HistoryForce.GetType());
        self.HistorySFType = (MapMatrix2[]) info.GetValue(nameof (HistorySFType), self.HistorySFType.GetType());
        self.HistoryHis = (MapMatrix2[]) info.GetValue(nameof (HistoryHis), self.HistoryHis.GetType());
        self.HistoryDepth = (MapMatrix2[]) info.GetValue(nameof (HistoryDepth), self.HistoryDepth.GetType());
        self.AIVP = (MapMatrix2[]) info.GetValue(nameof (AIVP), self.AIVP.GetType());
        self.AIPower = (MapMatrix2[]) info.GetValue(nameof (AIPower), self.AIPower.GetType());
        try
        {
          self.AIDefense = (MapMatrix2[]) info.GetValue(nameof (AIDefense), self.AIPower.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        self.Trafic = new MapMatrix2[self.MapCount + 1];
        try
        {
          self.Trafic = (MapMatrix2[]) info.GetValue(nameof (Trafic), self.Trafic.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Trafic[0] = new MapMatrix2(mapWidth, mapHeight);
          ProjectData.ClearProjectError();
        }
        self.Trafic2 = new MapMatrix2[self.MapCount + 1];
        try
        {
          self.Trafic2 = (MapMatrix2[]) info.GetValue(nameof (Trafic2), self.Trafic2.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Trafic2[0] = new MapMatrix2(mapWidth, mapHeight);
          ProjectData.ClearProjectError();
        }
      }
      catch (Exception ex1)
      {
        ProjectData.SetProjectError(ex1);
        self.MapCount = 0;
        self.HistoryOwner = new MapMatrix2[1];
        self.HistoryForce = new MapMatrix2[1];
        self.HistorySFType = new MapMatrix2[1];
        self.HistoryHis = new MapMatrix2[1];
        self.HistoryDepth = new MapMatrix2[1];
        self.AIVP = new MapMatrix2[1];
        self.AIPower = new MapMatrix2[1];
        self.AIDefense = new MapMatrix2[1];
        if (mapWidth > -1 & mapHeight > -1)
        {
          numArray2: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
          numArray3: Vec<i32> = (int[,]) info.GetValue(nameof (HistoryOwner), numArray2.GetType());
          self.HistoryOwner[0] = new MapMatrix2(mapWidth, mapHeight);
          self.HistoryOwner[0].Value = numArray3;
          numArray4: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
          numArray5: Vec<i32> = (int[,]) info.GetValue(nameof (HistoryForce), numArray4.GetType());
          self.HistoryForce[0] = new MapMatrix2(mapWidth, mapHeight);
          self.HistoryForce[0].Value = numArray5;
          numArray6: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
          numArray7: Vec<i32> = (int[,]) info.GetValue(nameof (HistorySFType), numArray6.GetType());
          self.HistorySFType[0] = new MapMatrix2(mapWidth, mapHeight);
          self.HistorySFType[0].Value = numArray7;
          try
          {
            numArray8: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray9: Vec<i32> = (int[,]) info.GetValue(nameof (HistoryHis), numArray8.GetType());
            self.HistoryHis[0] = new MapMatrix2(mapWidth, mapHeight);
            self.HistoryHis[0].Value = numArray9;
            numArray10: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray11: Vec<i32> = (int[,]) info.GetValue(nameof (HistoryDepth), numArray10.GetType());
            self.HistoryDepth[0] = new MapMatrix2(mapWidth, mapHeight);
            self.HistoryDepth[0].Value = numArray11;
          }
          catch (Exception ex2)
          {
            ProjectData.SetProjectError(ex2);
            self.HistoryHis = new MapMatrix2[1];
            self.HistoryDepth = new MapMatrix2[1];
            numArray12: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            self.HistoryHis[0] = new MapMatrix2(mapWidth, mapHeight);
            self.HistoryHis[0].Value = numArray12;
            self.HistoryDepth[0] = new MapMatrix2(mapWidth, mapHeight);
            self.HistoryDepth[0].Value = numArray12;
            ProjectData.ClearProjectError();
          }
          try
          {
            numArray13: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray14: Vec<i32> = (int[,]) info.GetValue(nameof (AIVP), numArray13.GetType());
            self.AIVP[0] = new MapMatrix2(mapWidth, mapHeight);
            self.AIVP[0].Value = numArray14;
          }
          catch (Exception ex3)
          {
            ProjectData.SetProjectError(ex3);
            self.AIVP[0] = new MapMatrix2(mapWidth, mapHeight);
            ProjectData.ClearProjectError();
          }
          try
          {
            numArray15: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray16: Vec<i32> = (int[,]) info.GetValue(nameof (Trafic), numArray15.GetType());
            self.Trafic[0] = new MapMatrix2(mapWidth, mapHeight);
            self.Trafic[0].Value = numArray16;
          }
          catch (Exception ex4)
          {
            ProjectData.SetProjectError(ex4);
            self.Trafic[0] = new MapMatrix2(mapWidth, mapHeight);
            ProjectData.ClearProjectError();
          }
          try
          {
            numArray17: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray18: Vec<i32> = (int[,]) info.GetValue(nameof (Trafic2), numArray17.GetType());
            self.Trafic2[0] = new MapMatrix2(mapWidth, mapHeight);
            self.Trafic2[0].Value = numArray18;
          }
          catch (Exception ex5)
          {
            ProjectData.SetProjectError(ex5);
            self.Trafic2[0] = new MapMatrix2(mapWidth, mapHeight);
            ProjectData.ClearProjectError();
          }
          try
          {
            numArray19: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray20: Vec<i32> = (int[,]) info.GetValue(nameof (AIPower), numArray19.GetType());
            self.AIPower[0] = new MapMatrix2(mapWidth, mapHeight);
            self.AIPower[0].Value = numArray20;
          }
          catch (Exception ex6)
          {
            ProjectData.SetProjectError(ex6);
            self.AIPower[0] = new MapMatrix2(mapWidth, mapHeight);
            ProjectData.ClearProjectError();
          }
          try
          {
            numArray21: Vec<i32> = new int[mapWidth + 1, mapHeight + 1];
            numArray22: Vec<i32> = (int[,]) info.GetValue(nameof (AIDefense), numArray21.GetType());
            self.AIDefense[0] = new MapMatrix2(mapWidth, mapHeight);
            self.AIDefense[0].Value = numArray22;
          }
          catch (Exception ex7)
          {
            ProjectData.SetProjectError(ex7);
            self.AIDefense[0] = new MapMatrix2(mapWidth, mapHeight);
            ProjectData.ClearProjectError();
          }
        }
        ProjectData.ClearProjectError();
      }
      try
      {
        self.LoadTransferHistorical = info.GetBoolean(nameof (LoadTransferHistorical));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LoadTransferHistorical = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.OfficerPool = info.GetInt32(nameof (OfficerPool));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.OfficerPool = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.OldAINarrow[0] = new MapMatrix2(mapWidth, mapHeight);
        self.OldAINarrow = (MapMatrix2[]) info.GetValue(nameof (OldAINarrow), self.OldAINarrow.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.OldAINarrow[0] = new MapMatrix2(mapWidth, mapHeight);
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Mirror = info.GetBoolean(nameof (Mirror));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Mirror = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Version = info.GetInt32(nameof (Version));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Version = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.subVersion = info.GetString(nameof (subVersion));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.subVersion = "";
        ProjectData.ClearProjectError();
      }
      self.MessCounter = info.GetInt32(nameof (MessCounter));
      if (self.MessCounter > -1)
      {
        self.MessString = new string[self.MessCounter + 1];
        self.MessBackPic = new int[self.MessCounter + 1];
        self.MessFrontPic = new int[self.MessCounter + 1];
        self.MessWav = new string[self.MessCounter + 1];
        self.MesStyle = new int[self.MessCounter + 1];
        self.MesNote = new string[self.MessCounter + 1];
        self.MesNote2 = new string[self.MessCounter + 1];
        self.MesGroup = new int[self.MessCounter + 1];
        self.MesName = new string[self.MessCounter + 1];
        self.MesChosen = new int[self.MessCounter + 1];
        self.MesHideFromStart = new bool[self.MessCounter + 1];
        self.MesHideFromTab = new bool[self.MessCounter + 1];
      }
      else
      {
        self.MessString = new string[1];
        self.MessBackPic = new int[1];
        self.MessFrontPic = new int[1];
        self.MessWav = new string[1];
        self.MesStyle = new int[1];
        self.MesNote = new string[1];
        self.MesNote2 = new string[1];
        self.MesGroup = new int[1];
        self.MesName = new string[1];
        self.MesChosen = new int[1];
        self.MesHideFromStart = new bool[1];
        self.MesHideFromTab = new bool[1];
      }
      self.MessString = (string[]) info.GetValue(nameof (MessString), self.MessString.GetType());
      self.MessBackPic = (int[]) info.GetValue(nameof (MessBackPic), self.MessBackPic.GetType());
      self.MessFrontPic = (int[]) info.GetValue(nameof (MessFrontPic), self.MessFrontPic.GetType());
      try
      {
        self.MessWav = (string[]) info.GetValue(nameof (MessWav), self.MessWav.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      try
      {
        self.MesStyle = (int[]) info.GetValue(nameof (MesStyle), self.MesStyle.GetType());
        self.MesNote = (string[]) info.GetValue(nameof (MesNote), self.MesNote.GetType());
        self.MesNote2 = (string[]) info.GetValue(nameof (MesNote2), self.MesNote2.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      try
      {
        self.MesName = (string[]) info.GetValue(nameof (MesName), self.MesName.GetType());
        self.MesGroup = (int[]) info.GetValue(nameof (MesGroup), self.MesGroup.GetType());
        self.MesChosen = (int[]) info.GetValue(nameof (MesChosen), self.MesChosen.GetType());
        self.MesHideFromStart = (bool[]) info.GetValue(nameof (MesHideFromStart), self.MesHideFromStart.GetType());
        self.MesHideFromTab = (bool[]) info.GetValue(nameof (MesHideFromTab), self.MesHideFromTab.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      self.AIid = info.GetInt32(nameof (AIid));
      self.PassWord = info.GetString(nameof (PassWord));
      self.UnitName = info.GetString(nameof (UnitName));
      self.UnitNumber = info.GetInt32(nameof (UnitNumber));
      self.HQName = info.GetString(nameof (HQName));
      self.HQNumber = info.GetInt32(nameof (HQNumber));
      self.SLoss = (int[,]) info.GetValue(nameof (SLoss), self.SLoss.GetType());
      self.SKills = (int[,]) info.GetValue(nameof (SKills), self.SKills.GetType());
      self.SProd = (int[,]) info.GetValue(nameof (SProd), self.SProd.GetType());
      try
      {
        self.SPresent = (int[,]) info.GetValue(nameof (SPresent), self.SPresent.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SPresent = new int[self.SLoss.GetUpperBound(0) + 1, self.SLoss.GetUpperBound(1) + 1];
        ProjectData.ClearProjectError();
      }
      if (self.SProd.GetUpperBound(0) < self.SKills.GetUpperBound(0))
        self.SProd = new int[self.SLoss.GetUpperBound(0) + 1, self.SLoss.GetUpperBound(1) + 1];
      self.DipBlock = info.GetBoolean(nameof (DipBlock));
      self.SASSupplyLost = info.GetInt32(nameof (SASSupplyLost));
      self.SASSupplyKilled = info.GetInt32(nameof (SASSupplyKilled));
      self.SASProdLost = (int[]) info.GetValue(nameof (SASProdLost), self.SASProdLost.GetType());
      self.SASKilled = (int[]) info.GetValue(nameof (SASKilled), self.SASKilled.GetType());
      self.HQSpriteOverrule = info.GetBoolean(nameof (HQSpriteOverrule));
      self.ProdBonus = info.GetInt32(nameof (ProdBonus));
      self.ActionCardCounter = info.GetInt32(nameof (ActionCardCounter));
      self.ActionCardHistoryCounter = info.GetInt32(nameof (ActionCardHistoryCounter));
      if (self.ActionCardCounter > -1)
      {
        self.ActionCard = new int[self.ActionCardCounter + 1];
        self.ActionCard = (int[]) info.GetValue(nameof (ActionCard), self.ActionCard.GetType());
      }
      else
      {
        self.ActionCardCounter = -1;
        self.ActionCard = new int[1];
      }
      if (self.ActionCardHistoryCounter > -1)
      {
        self.ActionCardHistory = new int[self.ActionCardHistoryCounter + 1];
        self.ActionCardHistoryRound = new int[self.ActionCardHistoryCounter + 1];
        self.ActionCardHistory = (int[]) info.GetValue(nameof (ActionCardHistory), self.ActionCardHistory.GetType());
        self.ActionCardHistoryRound = (int[]) info.GetValue(nameof (ActionCardHistoryRound), self.ActionCardHistoryRound.GetType());
      }
      else
      {
        self.ActionCardHistoryCounter = -1;
        self.ActionCardHistory = new int[1];
        self.ActionCardHistoryRound = new int[1];
      }
      self.AISavedPP = 0;
      self.AIConservative = info.GetSingle(nameof (AIConservative));
      if ( self.AIConservative <= 0.0)
        self.AIConservative = 1f;
      self.ExtraStat = (int[,]) info.GetValue(nameof (ExtraStat), self.ExtraStat.GetType());
      self.NationalIconSpriteName = info.GetString(nameof (NationalIconSpriteName));
      self.ExtraGraphicUse = info.GetInt32(nameof (ExtraGraphicUse));
      try
      {
        self.FirstRound = info.GetInt32(nameof (FirstRound));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.FirstRound = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.RandomCode = info.GetInt32(nameof (RandomCode));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.RandomCode = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.AIGroupCounter = info.GetInt32(nameof (AIGroupCounter));
        self.AIGroupHis = (int[]) info.GetValue(nameof (AIGroupHis), self.AIGroupHis.GetType());
        self.AIGroupType = (int[]) info.GetValue(nameof (AIGroupType), self.AIGroupType.GetType());
        self.AIGroupName = (string[]) info.GetValue(nameof (AIGroupName), self.AIGroupName.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.AIGroupCounter = -1;
        ProjectData.ClearProjectError();
      }
      if (self.AIGroupCounter > -1)
      {
        try
        {
          self.AIGroupLastAttack = (int[]) info.GetValue(nameof (AIGroupLastAttack), self.AIGroupLastAttack.GetType());
          self.AIGroupLastDefend = (int[]) info.GetValue(nameof (AIGroupLastDefend), self.AIGroupLastDefend.GetType());
          self.AIGroupLastFallBack = (int[]) info.GetValue(nameof (AIGroupLastFallBack), self.AIGroupLastFallBack.GetType());
          self.AIGroupLastFollowUp = (int[]) info.GetValue(nameof (AIGroupLastFollowUp), self.AIGroupLastFollowUp.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AIGroupLastAttack = new int[self.AIGroupCounter + 1];
          self.AIGroupLastDefend = new int[self.AIGroupCounter + 1];
          self.AIGroupLastFollowUp = new int[self.AIGroupCounter + 1];
          self.AIGroupLastFallBack = new int[self.AIGroupCounter + 1];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.AICorpsTopGroup = (int[]) info.GetValue(nameof (AICorpsTopGroup), self.AICorpsTopGroup.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AICorpsTopGroup = new int[self.AIGroupCounter + 1];
          ProjectData.ClearProjectError();
        }
      }
      try
      {
        self.AIHelpMove = info.GetInt32(nameof (AIHelpMove));
        self.AIHelpCombat = info.GetInt32(nameof (AIHelpCombat));
        self.AIHelpStrategic = info.GetInt32(nameof (AIHelpStrategic));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.AIHelpMove = 0;
        self.AIHelpCombat = 0;
        self.AIHelpStrategic = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.RoundelSpriteName = info.GetString(nameof (RoundelSpriteName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.RoundelSpriteName = "systemgraphics/trans.bmp";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.RoundelSpriteName2 = info.GetString(nameof (RoundelSpriteName2));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.RoundelSpriteName2 = "systemgraphics/trans.bmp";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.PbemPlayer = info.GetInt32("PBEMPlayer");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.PbemPlayer = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.UseAlternateActionCardPics = (uint) info.GetInt32(nameof (UseAlternateActionCardPics)) > 0U;
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.UseAlternateActionCardPics = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.FerryEffectivity = info.GetInt32(nameof (FerryEffectivity));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.FerryEffectivity = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.libId = LibIdClass::new();
        self.libId = (LibIdClass) info.GetValue("LibId", self.libId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.libId = LibIdClass::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        self.id = info.GetInt32("Id");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.id = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.hideFromList = info.GetBoolean(nameof (hideFromList));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.hideFromList = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.minimumDataUsage = info.GetBoolean(nameof (minimumDataUsage));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.minimumDataUsage = false;
        ProjectData.ClearProjectError();
      }
      if (!(self.Sleep | self.AI))
        return;
      self.HistoryStepCounter = -1;
      self.HistoryStep = new HistoryStepClass[1];
    }

    pub RegimeClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (RegimeClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub RegimeClass(
      hardcoded: i32,
      tregimecounter: i32,
      trescounter: i32,
      tData: DataClass,
      bool tminimalDataUsage = false)
    {
      self.RegimeSlot = new int[500];
      self.LastTempRegimeSlotPredict = new int[500];
      self.TempRegimeSlotPredict = new int[500];
      self.TempRegimeSlotIncrease = new int[500];
      self.ResField = new bool[1];
      self.RegimeRel = new int[1];
      self.RegimeOffer = new int[1];
      self.HistoryOwner = new MapMatrix2[1];
      self.HistoryForce = new MapMatrix2[1];
      self.HistorySFType = new MapMatrix2[1];
      self.HistoryHis = new MapMatrix2[1];
      self.HistoryDepth = new MapMatrix2[1];
      self.Trafic = new MapMatrix2[1];
      self.Trafic2 = new MapMatrix2[1];
      self.AIVP = new MapMatrix2[1];
      self.AIPower = new MapMatrix2[1];
      self.AIDefense = new MapMatrix2[1];
      self.OldAINarrow = new MapMatrix2[1];
      self.HistoryStep = new HistoryStepClass[1];
      self.MessString = new string[1];
      self.MessBackPic = new int[1];
      self.MessFrontPic = new int[1];
      self.MessWav = new string[1];
      self.MesStyle = new int[1];
      self.MesNote = new string[1];
      self.MesNote2 = new string[1];
      self.MesName = new string[1];
      self.MesGroup = new int[1];
      self.MesHideFromStart = new bool[1];
      self.MesHideFromTab = new bool[1];
      self.MesChosen = new int[1];
      self.SLoss = new int[1, 1];
      self.SKills = new int[1, 1];
      self.SProd = new int[1, 1];
      self.SPresent = new int[1, 1];
      self.LisPoints = new int[1, 1, 7];
      self.PlanObj = new AIPlanClass[1];
      self.SASProdLost = new int[1];
      self.SASKilled = new int[1];
      self.ActionCard = new int[1];
      self.ActionCardHistory = new int[1];
      self.ActionCardHistoryRound = new int[1];
      self.ExtraStat = new int[3, 1];
      self.AIGroupName = new string[1];
      self.AIGroupType = new int[1];
      self.AIGroupHis = new int[1];
      self.AIGroupLastAttack = new int[1];
      self.AIGroupLastDefend = new int[1];
      self.AIGroupLastFollowUp = new int[1];
      self.AIGroupLastFallBack = new int[1];
      self.AICorpsTopGroup = new int[1];
      self.RoundelSpriteName = "systemgraphics/trans.bmp";
      self.RoundelSpriteName2 = "systemgraphics/trans.bmp";
      self.UberRegime = -1;
      self.Name = "Default Regime";
      self.Red =  byte.MaxValue;
      self.Green = 128;
      self.Blue = 0;
      self.subVersion = "";
      self.Red2 =  byte.MaxValue;
      self.Green2 =  byte.MaxValue;
      self.Blue2 =  byte.MaxValue;
      self.HQSpriteFileName = "systemgraphics/trans.bmp";
      self.HQSpriteFileName2 = "systemgraphics/trans.bmp";
      self.BannerSpriteFileName = "systemgraphics/trans.bmp";
      self.BannerSpriteFileName2 = "systemgraphics/trans.bmp";
      self.SymbolSpriteName = "systemgraphics/trans.bmp";
      let mut index1: i32 = 0;
      do
      {
        self.RegimeSlot[index1] = -1;
        index1 += 1;
      }
      while (index1 <= 499);
      self.BaseMorale = 100;
      self.RegimeCounter = tregimecounter;
      self.RegimeRel = new int[self.RegimeCounter + 1];
      self.RegimeOffer = new int[self.RegimeCounter + 1];
      let mut regimeCounter: i32 = self.RegimeCounter;
      for (let mut index2: i32 = 0; index2 <= regimeCounter; index2 += 1)
        self.RegimeRel[index2] = 1;
      self.ResFieldCounter = trescounter;
      self.ResField = new bool[self.ResFieldCounter + 1];
      let mut resFieldCounter: i32 = self.ResFieldCounter;
      for (let mut index3: i32 = 0; index3 <= resFieldCounter; index3 += 1)
        self.ResField[index3] = false;
      self.HistoryStepCounter = -1;
      self.MapCount = tData.MapCounter;
      self.HistoryOwner = new MapMatrix2[self.MapCount + 1];
      self.HistoryForce = new MapMatrix2[self.MapCount + 1];
      self.HistorySFType = new MapMatrix2[self.MapCount + 1];
      self.HistoryHis = new MapMatrix2[self.MapCount + 1];
      self.HistoryDepth = new MapMatrix2[self.MapCount + 1];
      self.AIVP = new MapMatrix2[self.MapCount + 1];
      self.Trafic = new MapMatrix2[self.MapCount + 1];
      self.Trafic2 = new MapMatrix2[self.MapCount + 1];
      self.AIPower = new MapMatrix2[self.MapCount + 1];
      self.AIDefense = new MapMatrix2[self.MapCount + 1];
      self.minimumDataUsage = tminimalDataUsage;
      if (!self.minimumDataUsage)
      {
        let mut mapCount: i32 = self.MapCount;
        for (let mut index4: i32 = 0; index4 <= mapCount; index4 += 1)
        {
          self.HistoryOwner[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.HistoryForce[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.HistorySFType[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.HistoryHis[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.HistoryDepth[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.AIVP[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.Trafic[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.Trafic2[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.AIPower[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
          self.AIDefense[index4] = new MapMatrix2(tData.MapObj[index4].MapWidth, tData.MapObj[index4].MapHeight);
        }
      }
      self.Sleep = false;
      self.ActionCardCounter = -1;
      self.ActionCardHistoryCounter = -1;
      self.AIConservative = 1f;
      self.NationalIconSpriteName = "systemgraphics/trans.bmp";
      self.ExtraGraphicUse = -1;
      self.OfficerPool = -1;
      self.libId = LibIdClass::new();
    }

    pub fn AddMap(w: i32, h: i32)
    {
      this += 1.MapCount;
      self.HistoryOwner = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryOwner, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryForce = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryForce, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistorySFType = (MapMatrix2[]) Utils.CopyArray((Array) self.HistorySFType, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryHis = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryHis, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryDepth = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryDepth, (Array) new MapMatrix2[self.MapCount + 1]);
      self.AIVP = (MapMatrix2[]) Utils.CopyArray((Array) self.AIVP, (Array) new MapMatrix2[self.MapCount + 1]);
      self.Trafic = (MapMatrix2[]) Utils.CopyArray((Array) self.Trafic, (Array) new MapMatrix2[self.MapCount + 1]);
      self.Trafic2 = (MapMatrix2[]) Utils.CopyArray((Array) self.Trafic2, (Array) new MapMatrix2[self.MapCount + 1]);
      self.AIPower = (MapMatrix2[]) Utils.CopyArray((Array) self.AIPower, (Array) new MapMatrix2[self.MapCount + 1]);
      self.AIDefense = (MapMatrix2[]) Utils.CopyArray((Array) self.AIDefense, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryOwner[self.MapCount] = new MapMatrix2(w, h);
      self.HistoryForce[self.MapCount] = new MapMatrix2(w, h);
      self.HistorySFType[self.MapCount] = new MapMatrix2(w, h);
      self.HistoryHis[self.MapCount] = new MapMatrix2(w, h);
      self.HistoryDepth[self.MapCount] = new MapMatrix2(w, h);
      self.AIVP[self.MapCount] = new MapMatrix2(w, h);
      self.Trafic[self.MapCount] = new MapMatrix2(w, h);
      self.Trafic2[self.MapCount] = new MapMatrix2(w, h);
      self.AIPower[self.MapCount] = new MapMatrix2(w, h);
      self.AIDefense[self.MapCount] = new MapMatrix2(w, h);
    }

    pub fn RemoveMap(nr: i32)
    {
      if (nr < self.MapCount)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.MapCount - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
        {
          self.HistoryOwner[index] = self.HistoryOwner[index + 1];
          self.HistoryForce[index] = self.HistoryForce[index + 1];
          self.HistorySFType[index] = self.HistorySFType[index + 1];
          self.HistoryHis[index] = self.HistoryHis[index + 1];
          self.HistoryDepth[index] = self.HistoryDepth[index + 1];
          self.AIVP[index] = self.AIVP[index + 1];
          self.Trafic[index] = self.Trafic[index + 1];
          self.Trafic2[index] = self.Trafic2[index + 1];
          self.AIPower[index] = self.AIPower[index + 1];
          self.AIDefense[index] = self.AIDefense[index + 1];
        }
      }
      --self.MapCount;
      if (self.MapCount <= -1)
        return;
      self.HistoryOwner = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryOwner, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryForce = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryForce, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistorySFType = (MapMatrix2[]) Utils.CopyArray((Array) self.HistorySFType, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryHis = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryHis, (Array) new MapMatrix2[self.MapCount + 1]);
      self.HistoryDepth = (MapMatrix2[]) Utils.CopyArray((Array) self.HistoryDepth, (Array) new MapMatrix2[self.MapCount + 1]);
      self.AIVP = (MapMatrix2[]) Utils.CopyArray((Array) self.AIVP, (Array) new MapMatrix2[self.MapCount + 1]);
      self.Trafic = (MapMatrix2[]) Utils.CopyArray((Array) self.Trafic, (Array) new MapMatrix2[self.MapCount + 1]);
      self.Trafic2 = (MapMatrix2[]) Utils.CopyArray((Array) self.Trafic2, (Array) new MapMatrix2[self.MapCount + 1]);
      self.AIPower = (MapMatrix2[]) Utils.CopyArray((Array) self.AIPower, (Array) new MapMatrix2[self.MapCount + 1]);
      self.AIDefense = (MapMatrix2[]) Utils.CopyArray((Array) self.AIDefense, (Array) new MapMatrix2[self.MapCount + 1]);
    }

    pub fn AddResField()
    {
      this += 1.ResFieldCounter;
      self.ResField = (bool[]) Utils.CopyArray((Array) self.ResField, (Array) new bool[self.ResFieldCounter + 1]);
      self.ResField[self.ResFieldCounter] = false;
    }

    pub fn RemoveResField(nr: i32)
    {
      if (nr < self.ResFieldCounter)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.ResFieldCounter - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
          self.ResField[index] = self.ResField[index + 1];
      }
      --self.ResFieldCounter;
      if (self.ResFieldCounter <= -1)
        return;
      self.ResField = (bool[]) Utils.CopyArray((Array) self.ResField, (Array) new bool[self.ResFieldCounter + 1]);
    }

    pub fn AddRegime()
    {
      this += 1.RegimeCounter;
      self.RegimeRel = (int[]) Utils.CopyArray((Array) self.RegimeRel, (Array) new int[self.RegimeCounter + 1]);
      self.RegimeOffer = (int[]) Utils.CopyArray((Array) self.RegimeOffer, (Array) new int[self.RegimeCounter + 1]);
      self.RegimeRel[self.RegimeCounter] = 1;
    }

    pub fn Removeregime(nr: i32)
    {
      if (nr < self.RegimeCounter)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.RegimeCounter - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
          self.RegimeRel[index] = self.RegimeRel[index + 1];
      }
      --self.RegimeCounter;
      if (self.RegimeCounter <= -1)
        return;
      self.RegimeRel = (int[]) Utils.CopyArray((Array) self.RegimeRel, (Array) new int[self.RegimeCounter + 1]);
    }

    pub fn AddActionCard(nr: i32)
    {
      this += 1.ActionCardCounter;
      self.ActionCard = (int[]) Utils.CopyArray((Array) self.ActionCard, (Array) new int[self.ActionCardCounter + 1]);
      self.ActionCard[self.ActionCardCounter] = nr;
    }

    pub fn RemoveActionCard(nr: i32)
    {
      if (nr < self.ActionCardCounter)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.ActionCardCounter - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
          self.ActionCard[index] = self.ActionCard[index + 1];
      }
      --self.ActionCardCounter;
      if (self.ActionCardCounter <= -1)
        return;
      self.ActionCard = (int[]) Utils.CopyArray((Array) self.ActionCard, (Array) new int[self.ActionCardCounter + 1]);
    }

    pub fn AddActionCardHistory(nr: i32, round: i32)
    {
      this += 1.ActionCardHistoryCounter;
      self.ActionCardHistory = (int[]) Utils.CopyArray((Array) self.ActionCardHistory, (Array) new int[self.ActionCardHistoryCounter + 1]);
      self.ActionCardHistoryRound = (int[]) Utils.CopyArray((Array) self.ActionCardHistoryRound, (Array) new int[self.ActionCardHistoryCounter + 1]);
      self.ActionCardHistory[self.ActionCardHistoryCounter] = nr;
      self.ActionCardHistoryRound[self.ActionCardHistoryCounter] = round;
    }

    pub fn RemoveActionCardHistory(nr: i32)
    {
      if (nr < self.ActionCardHistoryCounter)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.ActionCardHistoryCounter - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
        {
          self.ActionCardHistory[index] = self.ActionCardHistory[index + 1];
          self.ActionCardHistoryRound[index] = self.ActionCardHistoryRound[index + 1];
        }
      }
      --self.ActionCardHistoryCounter;
      if (self.ActionCardHistoryCounter <= -1)
        return;
      self.ActionCardHistory = (int[]) Utils.CopyArray((Array) self.ActionCardHistory, (Array) new int[self.RegimeCounter + 1]);
      self.ActionCardHistoryRound = (int[]) Utils.CopyArray((Array) self.ActionCardHistoryRound, (Array) new int[self.RegimeCounter + 1]);
    }

    pub fn DoTempCounterBig() => self.DoTempCounterAnySize(1);

    pub fn DoTempCounter() => self.DoTempCounterAnySize(0);

    pub fn DoTempCounterSmall() => self.DoTempCounterAnySize(-1);

    pub fn doTempRegimeHighlight(bool lighter = false)
    {
      float num1 =  ( self.Red / 512.0 - 0.9);
      float num2 =  ( self.Green / 512.0 - 0.9);
      float num3 =  ( self.Blue / 512.0 - 0.9);
      float num4 = 0.25f;
      if (lighter)
        num4 = 0.1f;
      bitmap1: Bitmap = new Bitmap(128, 96);
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics1 = Graphics.FromImage((Image) bitmap1);
      graphics1.Clear(Color.Transparent);
       let mut local1: &Graphics = &graphics1;
      bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.WHITEHEX, 1);
       let mut local2: &Bitmap = &bitmap2;
      double r1 =  num1;
      double g1 =  num2;
      double b1 =  num3;
      double a1 =  num4;
      DrawMod.Draw( local1,  local2, 0, 0,  r1,  g1,  b1,  a1);
      self.TempRegimeColorBig = bitmap1;
      graphics1.Dispose();
      bitmap3: Bitmap = new Bitmap(64, 48);
      bitmap3.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics2 = Graphics.FromImage((Image) bitmap3);
      graphics2.Clear(Color.Transparent);
       let mut local3: &Graphics = &graphics2;
      bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.WHITEHEX);
       let mut local4: &Bitmap = &bitmap4;
      double r2 =  num1;
      double g2 =  num2;
      double b2 =  num3;
      double a2 =  num4;
      DrawMod.Draw( local3,  local4, 0, 0,  r2,  g2,  b2,  a2);
      self.TempRegimeColor = bitmap3;
      graphics2.Dispose();
      bitmap5: Bitmap = new Bitmap(32, 24);
      bitmap5.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics3 = Graphics.FromImage((Image) bitmap5);
      graphics3.Clear(Color.Transparent);
       let mut local5: &Graphics = &graphics3;
      bitmap6: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.WHITEHEX, -1);
       let mut local6: &Bitmap = &bitmap6;
      double r3 =  num1;
      double g3 =  num2;
      double b3 =  num3;
      double a3 =  num4;
      DrawMod.Draw( local5,  local6, 0, 0,  r3,  g3,  b3,  a3);
      self.TempRegimeColorSmall = bitmap5;
      graphics3.Dispose();
    }

    pub fn DoTempCounterAnySize(sizey: i32)
    {
      let mut landscapeTypeCounter: i32 = DrawMod.TGame.Data.LandscapeTypeCounter;
      let mut num1: i32 = 76;
      let mut num2: i32 = 75;
      let mut num3: i32 = 128;
      let mut num4: i32 = 96;
      switch (sizey)
      {
        case -1:
          num2 = 18;
          num1 = 19;
          num4 = 24;
          num3 = 32;
          break;
        case 0:
          num2 = 37;
          num1 = 38;
          num4 = 48;
          num3 = 64;
          break;
      }
      float num5 =  self.Red /  byte.MaxValue;
      float num6 =  self.Green /  byte.MaxValue;
      float num7 =  self.Blue /  byte.MaxValue;
      float num8 =  (1.0 * ( num5 / (( num6 +  num7) / 2.0)));
      float num9 =  (1.0 * ( num6 / (( num5 +  num7) / 2.0)));
      float num10 =  (1.0 * ( num7 / (( num6 +  num5) / 2.0)));
      float num11 =  ((1.0 +  num8) / 2.0);
      float num12 =  ((1.0 +  num9) / 2.0);
      float num13 =  ((1.0 +  num10) / 2.0);
      bitmap1: Bitmap = new Bitmap(num1 * (landscapeTypeCounter + 1), num1);
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      str1: String = "systemgraphics/defaultcounterbig.png";
      if (sizey == -1)
        str1 = "systemgraphics/defaultcountersmall.png";
      if (sizey == 0)
        str1 = "systemgraphics/defaultcounter.png";
      graphicsDirectory: String = DrawMod.TGame.ModSystemGraphicsDirectory;
      if (Strings.Len(graphicsDirectory) > 1)
        str1 = str1.Replace("systemgraphics", graphicsDirectory);
      objBitmap: Bitmap;
      if (File.Exists(BitmapStore.GraphicsPath + str1))
      {
        objBitmap = new Bitmap(BitmapStore.GraphicsPath + str1);
      }
      else
      {
        str2: String = "systemgraphics/defaultcounterbig.png";
        if (sizey == -1)
          str2 = "systemgraphics/defaultcountersmall.png";
        if (sizey == 0)
          str2 = "systemgraphics/defaultcounter.png";
        objBitmap = new Bitmap(BitmapStore.GraphicsPath + str2);
      }
      objBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      let mut num14: i32 = objBitmap.Height - 1;
      pixel1: Color;
      for (let mut y: i32 = 0; y <= num14; y += 1)
      {
        let mut num15: i32 = objBitmap.Width - 1;
        for (let mut x: i32 = 0; x <= num15; x += 1)
        {
          pixel1 = objBitmap.GetPixel(x, y);
          let mut red: i32 =  Math.Round( ( pixel1.R * num5));
          let mut green: i32 =  Math.Round( ( pixel1.G * num6));
          let mut blue: i32 =  Math.Round( ( pixel1.B * num7));
          objBitmap.SetPixel(x, y, Color.FromArgb( pixel1.A, red, green, blue));
        }
      }
      Graphics objGraphics = Graphics.FromImage((Image) bitmap1);
      objGraphics.Clear(Color.Transparent);
      let mut num16: i32 = landscapeTypeCounter;
      for (let mut index: i32 = 0; index <= num16; index += 1)
        DrawMod.DrawSimple( objGraphics,  objBitmap, index * num1, 0);
      objGraphics.Dispose();
      bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.WHITEHEX, 1);
      float num17 = 0.4f;
      float num18 = 0.6f;
      let mut num19: i32 = landscapeTypeCounter;
      Rectangle rectangle1;
      Rectangle rectangle2;
      pixel2: Color;
      for (let mut index1: i32 = 0; index1 <= num19; index1 += 1)
      {
        if (DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexPicID > 0 & Strings.InStr(DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexPicFileName, "trans") < 1 & Strings.InStr(DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexPicFileName, "blackhex") < 1)
        {
          bool flag = false;
          let mut num20: i32 = index1 - 1;
          for (let mut index2: i32 = 0; index2 <= num20; index2 += 1)
          {
            if (DrawMod.TGame.Data.LandscapeTypeObj[index2].UsePreHexTexture && DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexTextureID == DrawMod.TGame.Data.LandscapeTypeObj[index2].PreHexTextureID)
            {
              flag = true;
              Graphics graphics = Graphics.FromImage((Image) bitmap1);
              graphics.CompositingMode = CompositingMode.SourceCopy;
               let mut local1: &Graphics = &graphics;
               let mut local2: &Bitmap = &bitmap1;
              rectangle1 = Rectangle::new(num1 * index2, 0, num1, num1);
              let mut srcrect: &Rectangle = &rectangle1
              rectangle2 = Rectangle::new(num1 * index1, 0, num1, num1);
              let mut destrect: &Rectangle = &rectangle2
              DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
              graphics.Dispose();
              break;
            }
          }
          if (!flag)
          {
            bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexPicID, 1);
            if (!Information.IsNothing( bitmap3))
            {
              long num21 = 0;
              long num22 = 0;
              let mut num23: i32 = num2;
              for (let mut y: i32 = 0; y <= num23; y += 1)
              {
                let mut num24: i32 = num2;
                for (let mut index3: i32 = 0; index3 <= num24; index3 += 1)
                {
                  pixel2 = bitmap3.GetPixel(index3 + 26, y);
                  num21 += (long) ( Math.Round( ( pixel2.G +  pixel2.B +  pixel2.R) / 3.0) *  Math.Round( pixel2.A / 16.0));
                  num22 += (long)  Math.Round( pixel2.A / 16.0);
                }
              }
              let mut num25: i32 =  Math.Round( num21 /  num22);
              let mut num26: i32 = num2;
              for (let mut y1: i32 = 0; y1 <= num26; y1 += 1)
              {
                let mut num27: i32 = num2;
                for (let mut x1: i32 = 0; x1 <= num27; x1 += 1)
                {
                  pixel1 = bitmap1.GetPixel(x1, y1);
                  pixel2 = bitmap3.GetPixel(x1 + 26, y1);
                  let mut r: i32 =  pixel2.R;
                  let mut g: i32 =  pixel2.G;
                  let mut b: i32 =  pixel2.B;
                  let mut a1: i32 =  pixel2.A;
                  let mut num28: i32 =  Math.Round( (r + b + g) / 3.0);
                  num29: i32;
                  num30: i32;
                  num31: i32;
                  if (num28 > num25)
                  {
                    num29 =  Math.Round( pixel1.G +  (Math.Min( pixel1.G * 2,  byte.MaxValue) -  pixel1.G) * ( (num28 - num25) /  byte.MaxValue));
                    num30 =  Math.Round( pixel1.R +  (Math.Min( pixel1.R * 2,  byte.MaxValue) -  pixel1.R) * ( (num28 - num25) /  byte.MaxValue));
                    num31 =  Math.Round( pixel1.B +  (Math.Min( pixel1.B * 2,  byte.MaxValue) -  pixel1.B) * ( (num28 - num25) /  byte.MaxValue));
                  }
                  else
                  {
                    num29 =  Math.Round( pixel1.G -  Math.Min( pixel1.G * 2,  byte.MaxValue) * ( (num25 - num28) /  byte.MaxValue));
                    num30 =  Math.Round( pixel1.R -  Math.Min( pixel1.R * 2,  byte.MaxValue) * ( (num25 - num28) /  byte.MaxValue));
                    num31 =  Math.Round( pixel1.B -  Math.Min( pixel1.B * 2,  byte.MaxValue) * ( (num25 - num28) /  byte.MaxValue));
                  }
                  let mut val2_1: i32 =  Math.Round( g *  num17 +  num29 *  num18);
                  let mut val2_2: i32 =  Math.Round( b *  num17 +  num31 *  num18);
                  let mut val2_3: i32 =  Math.Round( r *  num17 +  num30 *  num18);
                  if (pixel2.A == (byte) 0)
                  {
                    val2_1 =  pixel1.G;
                    val2_2 =  pixel1.B;
                    val2_3 =  pixel1.R;
                  }
                  else if (pixel2.A < byte.MaxValue)
                  {
                    val2_1 =  Math.Round( val2_1 * ( a1 /  byte.MaxValue) +  pixel1.G * ( ( byte.MaxValue - a1) /  byte.MaxValue));
                    val2_2 =  Math.Round( val2_2 * ( a1 /  byte.MaxValue) +  pixel1.B * ( ( byte.MaxValue - a1) /  byte.MaxValue));
                    val2_3 =  Math.Round( val2_3 * ( a1 /  byte.MaxValue) +  pixel1.R * ( ( byte.MaxValue - a1) /  byte.MaxValue));
                  }
                  let mut a2: i32 =  pixel1.A;
                  let mut red: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_3));
                  let mut green: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_1));
                  let mut blue: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_2));
                  let mut x2: i32 = x1 + index1 * num1;
                  let mut y2: i32 = y1;
                  bitmap1.SetPixel(x2, y2, Color.FromArgb(a2, red, green, blue));
                }
              }
            }
          }
        }
        else if (DrawMod.TGame.Data.LandscapeTypeObj[index1].UseSheet & !Information.IsNothing( BitmapStore.GetBitmap(DrawMod.TGame.Data.LandscapeTypeObj[index1].SheetSpriteID)))
        {
          bool flag = false;
          let mut num32: i32 = index1 - 1;
          for (let mut index4: i32 = 0; index4 <= num32; index4 += 1)
          {
            if (DrawMod.TGame.Data.LandscapeTypeObj[index4].UsePreHexTexture && DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexTextureID == DrawMod.TGame.Data.LandscapeTypeObj[index4].PreHexTextureID)
            {
              flag = true;
              Graphics graphics = Graphics.FromImage((Image) bitmap1);
              graphics.CompositingMode = CompositingMode.SourceCopy;
               let mut local3: &Graphics = &graphics;
               let mut local4: &Bitmap = &bitmap1;
              rectangle2 = Rectangle::new(num1 * index4, 0, num1, num1);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(num1 * index1, 0, num1, num1);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
              graphics.Dispose();
              break;
            }
          }
          if (!flag)
          {
            bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.LandscapeTypeObj[index1].SheetSpriteID, 1);
            if (!Information.IsNothing( bitmap4))
            {
              long num33 = 0;
              long num34 = 0;
              let mut num35: i32 = num2;
              for (let mut y: i32 = 0; y <= num35; y += 1)
              {
                let mut num36: i32 = num2;
                for (let mut index5: i32 = 0; index5 <= num36; index5 += 1)
                {
                  pixel2 = bitmap4.GetPixel(index5 + 26, y);
                  num33 += (long) ( Math.Round( ( pixel2.G +  pixel2.B +  pixel2.R) / 3.0) *  Math.Round( pixel2.A / 16.0));
                  num34 += (long)  Math.Round( pixel2.A / 16.0);
                }
              }
              let mut num37: i32 =  Math.Round( num33 /  num34);
              let mut num38: i32 = num2;
              for (let mut y3: i32 = 0; y3 <= num38; y3 += 1)
              {
                let mut num39: i32 = num2;
                for (let mut x3: i32 = 0; x3 <= num39; x3 += 1)
                {
                  pixel1 = bitmap1.GetPixel(x3, y3);
                  pixel2 = bitmap4.GetPixel(x3 + 256 + 26, y3 + 960);
                  let mut r: i32 =  pixel2.R;
                  let mut g: i32 =  pixel2.G;
                  let mut b: i32 =  pixel2.B;
                  let mut num40: i32 =  Math.Round( (r + b + g) / 3.0);
                  num41: i32;
                  num42: i32;
                  num43: i32;
                  if (num40 > num37)
                  {
                    num41 =  Math.Round( pixel1.G +  (Math.Min( pixel1.G * 2,  byte.MaxValue) -  pixel1.G) * ( (num40 - num37) /  byte.MaxValue));
                    num42 =  Math.Round( pixel1.R +  (Math.Min( pixel1.R * 2,  byte.MaxValue) -  pixel1.R) * ( (num40 - num37) /  byte.MaxValue));
                    num43 =  Math.Round( pixel1.B +  (Math.Min( pixel1.B * 2,  byte.MaxValue) -  pixel1.B) * ( (num40 - num37) /  byte.MaxValue));
                  }
                  else
                  {
                    num41 =  Math.Round( pixel1.G -  Math.Min( pixel1.G * 2,  byte.MaxValue) * ( (num37 - num40) /  byte.MaxValue));
                    num42 =  Math.Round( pixel1.R -  Math.Min( pixel1.R * 2,  byte.MaxValue) * ( (num37 - num40) /  byte.MaxValue));
                    num43 =  Math.Round( pixel1.B -  Math.Min( pixel1.B * 2,  byte.MaxValue) * ( (num37 - num40) /  byte.MaxValue));
                  }
                  let mut val2_4: i32 =  Math.Round( g * 0.25 +  num41 * 0.75);
                  let mut val2_5: i32 =  Math.Round( b * 0.25 +  num43 * 0.75);
                  let mut val2_6: i32 =  Math.Round( r * 0.25 +  num42 * 0.75);
                  let mut a: i32 =  pixel1.A;
                  let mut red: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_6));
                  let mut green: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_4));
                  let mut blue: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_5));
                  let mut x4: i32 = x3 + index1 * num1;
                  let mut y4: i32 = y3;
                  bitmap1.SetPixel(x4, y4, Color.FromArgb(a, red, green, blue));
                }
              }
            }
          }
        }
        else if (DrawMod.TGame.Data.LandscapeTypeObj[index1].UsePreHexTexture)
        {
          let mut preHexTextureId: i32 = DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexTextureID;
          bool flag = false;
          let mut num44: i32 = index1 - 1;
          for (let mut index6: i32 = 0; index6 <= num44; index6 += 1)
          {
            if (DrawMod.TGame.Data.LandscapeTypeObj[index6].UsePreHexTexture && DrawMod.TGame.Data.LandscapeTypeObj[index1].PreHexTextureID == DrawMod.TGame.Data.LandscapeTypeObj[index6].PreHexTextureID)
            {
              flag = true;
              Graphics graphics = Graphics.FromImage((Image) bitmap1);
              graphics.CompositingMode = CompositingMode.SourceCopy;
               let mut local5: &Graphics = &graphics;
               let mut local6: &Bitmap = &bitmap1;
              rectangle2 = Rectangle::new(num1 * index6, 0, num1, num1);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(num1 * index1, 0, num1, num1);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
              graphics.Dispose();
              break;
            }
          }
          if (!flag)
          {
            let mut count: i32 = 49152;
            byte[] dst = new byte[count - 1 + 1];
            let mut num45: i32 = (0 + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal * 8 + (0 + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal;
            Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheBig, num45 * count, (Array) dst, 0, count);
            let mut index7: i32 = 0;
            let mut num46: i32 = 0;
            let mut num47: i32 = 0;
            let mut num48: i32 = bitmap2.Height - 1;
            for (let mut index8: i32 = 0; index8 <= num48; index8 += 1)
            {
              let mut num49: i32 = bitmap2.Width - 1;
              for (let mut index9: i32 = 0; index9 <= num49; index9 += 1)
              {
                if (index9 <= num2 & index8 <= num2)
                {
                  index7 += 4;
                  let mut num50: i32 =  dst[index7];
                  let mut num51: i32 =  dst[index7 + 1];
                  let mut num52: i32 =  dst[index7 + 2];
                  num46 +=  Math.Round( (num50 + num51 + num52) / 3.0);
                  num47 += 1;
                }
              }
            }
            let mut num53: i32 =  Math.Round( num46 /  num47);
            let mut index10: i32 = 0;
            let mut num54: i32 = bitmap2.Height - 1;
            for (let mut y5: i32 = 0; y5 <= num54; y5 += 1)
            {
              let mut num55: i32 = bitmap2.Width - 1;
              for (let mut index11: i32 = 0; index11 <= num55; index11 += 1)
              {
                if (index11 <= num2 & y5 <= num2)
                {
                  pixel1 = bitmap1.GetPixel(index11 + index1 * num1, y5);
                  let mut num56: i32 =  dst[index10];
                  let mut num57: i32 =  dst[index10 + 1];
                  let mut num58: i32 =  dst[index10 + 2];
                  let mut num59: i32 =  Math.Round( (num57 + num56 + num58) / 3.0);
                  num60: i32;
                  num61: i32;
                  num62: i32;
                  if (num59 > num53)
                  {
                    num60 =  Math.Round( pixel1.G +  (Math.Min( pixel1.G * 2,  byte.MaxValue) -  pixel1.G) * ( (num59 - num53) /  byte.MaxValue));
                    num61 =  Math.Round( pixel1.R +  (Math.Min( pixel1.R * 2,  byte.MaxValue) -  pixel1.R) * ( (num59 - num53) /  byte.MaxValue));
                    num62 =  Math.Round( pixel1.B +  (Math.Min( pixel1.B * 2,  byte.MaxValue) -  pixel1.B) * ( (num59 - num53) /  byte.MaxValue));
                  }
                  else
                  {
                    num60 =  Math.Round( pixel1.G -  Math.Min( pixel1.G * 2,  byte.MaxValue) * ( (num53 - num59) /  byte.MaxValue));
                    num61 =  Math.Round( pixel1.R -  Math.Min( pixel1.R * 2,  byte.MaxValue) * ( (num53 - num59) /  byte.MaxValue));
                    num62 =  Math.Round( pixel1.B -  Math.Min( pixel1.B * 2,  byte.MaxValue) * ( (num53 - num59) /  byte.MaxValue));
                  }
                  let mut num63: i32 =  Math.Round( num57 * 0.14 +  num58 * 0.7 +  num56 * 0.1);
                  let mut val2_7: i32 =  Math.Round( num63 * 0.4 +  num60 * 0.75);
                  let mut val2_8: i32 =  Math.Round( num63 * 0.4 +  num62 * 0.75);
                  let mut val2_9: i32 =  Math.Round( num63 * 0.4 +  num61 * 0.75);
                  let mut a: i32 =  pixel1.A;
                  let mut red: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_9));
                  let mut green: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_7));
                  let mut blue: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_8));
                  let mut x: i32 = index11 + index1 * num1;
                  let mut y6: i32 = y5;
                  bitmap1.SetPixel(x, y6, Color.FromArgb( pixel1.A, red, green, blue));
                }
                index10 += 4;
              }
            }
          }
        }
        else if (!DrawMod.TGame.Data.LandscapeTypeObj[index1].UseSheet & DrawMod.TGame.Data.LandscapeTypeObj[index1].LayerSpriteID[0] > 0)
        {
          bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.LandscapeTypeObj[index1].LayerSpriteID[0], 1);
          if (!Information.IsNothing( bitmap5))
          {
            let mut num64: i32 = 0;
            let mut num65: i32 = 0;
            let mut num66: i32 = num2;
            for (let mut y: i32 = 0; y <= num66; y += 1)
            {
              let mut num67: i32 = num2;
              for (let mut index12: i32 = 0; index12 <= num67; index12 += 1)
              {
                pixel2 = bitmap5.GetPixel(index12 + 26, y);
                num64 +=  Math.Round( (byte) ((uint) (byte) ((uint) pixel2.G + (uint) pixel2.B) + (uint) pixel2.R) / 3.0) *  Math.Round( pixel2.A / 16.0);
                num65 +=  Math.Round( pixel2.A / 16.0);
              }
            }
            let mut num68: i32 =  Math.Round( num64 /  num65);
            let mut num69: i32 = num2;
            for (let mut y7: i32 = 0; y7 <= num69; y7 += 1)
            {
              let mut num70: i32 = num2;
              for (let mut x5: i32 = 0; x5 <= num70; x5 += 1)
              {
                pixel1 = bitmap1.GetPixel(x5, y7);
                pixel2 = bitmap5.GetPixel(x5 + 26, y7);
                let mut r: i32 =  pixel2.R;
                let mut g: i32 =  pixel2.G;
                let mut b: i32 =  pixel2.B;
                let mut num71: i32 =  Math.Round( (r + b + g) / 3.0);
                num72: i32;
                num73: i32;
                num74: i32;
                if (num71 > num68)
                {
                  num72 =  Math.Round( pixel1.G +  (Math.Min( pixel1.G * 2,  byte.MaxValue) -  pixel1.G) * ( (num71 - num68) / 128.0));
                  num73 =  Math.Round( pixel1.R +  (Math.Min( pixel1.R * 2,  byte.MaxValue) -  pixel1.R) * ( (num71 - num68) / 128.0));
                  num74 =  Math.Round( pixel1.B +  (Math.Min( pixel1.B * 2,  byte.MaxValue) -  pixel1.B) * ( (num71 - num68) / 128.0));
                }
                else
                {
                  num72 =  Math.Round( pixel1.G -  Math.Min( pixel1.G * 2,  byte.MaxValue) * ( (num68 - num71) / 128.0));
                  num73 =  Math.Round( pixel1.R -  Math.Min( pixel1.R * 2,  byte.MaxValue) * ( (num68 - num71) / 128.0));
                  num74 =  Math.Round( pixel1.B -  Math.Min( pixel1.B * 2,  byte.MaxValue) * ( (num68 - num71) / 128.0));
                }
                let mut val2_10: i32 =  Math.Round( g * 0.25 +  num72 * 0.75);
                let mut val2_11: i32 =  Math.Round( b * 0.25 +  num74 * 0.75);
                let mut val2_12: i32 =  Math.Round( r * 0.25 +  num73 * 0.75);
                let mut a: i32 =  pixel1.A;
                let mut red: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_12));
                let mut green: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_10));
                let mut blue: i32 = Math.Max(0, Math.Min( byte.MaxValue, val2_11));
                let mut x6: i32 = x5 + index1 * num1;
                let mut y8: i32 = y7;
                bitmap1.SetPixel(x6, y8, Color.FromArgb(a, red, green, blue));
              }
            }
          }
        }
        else
        {
          let mut num75: i32 = num75;
        }
      }
      bitmap6: Bitmap = new Bitmap(bitmap1.Width, bitmap1.Height);
      bitmap6.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics1 = Graphics.FromImage((Image) bitmap6);
      graphics1.CompositingMode = CompositingMode.SourceCopy;
       let mut local7: &Graphics = &graphics1;
       let mut local8: &Bitmap = &bitmap1;
      rectangle2 = Rectangle::new(0, 0, bitmap1.Width, bitmap1.Height);
      let mut srcrect1: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(0, 0, bitmap1.Width, bitmap1.Height);
      let mut destrect1: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2ColouredNew( local7,  local8, srcrect1, destrect1, 0.9f, 0.9f, 0.9f, 1f);
      graphics1.Dispose();
      graphics1 = (Graphics) null;
      switch (sizey)
      {
        case -1:
          self.TempCountersmall = bitmap1;
          self.TempCountersmallHigh = bitmap6;
          break;
        case 0:
          self.TempCounter = bitmap1;
          self.TempCounterHigh = bitmap6;
          break;
        case 1:
          self.TempCounterBig = bitmap1;
          self.TempCounterBigHigh = bitmap6;
          break;
      }
    }

    pub fn Kill()
    {
      self.TempCounter = (Bitmap) null;
      BitmapStore.RemoveBitmapNr(self.NationalIconSprite);
      BitmapStore.RemoveBitmapNr(self.HQSpriteNr);
    }

    pub fn LoadSprites()
    {
      self.HQSpriteNr = BitmapStore.AddFile(self.HQSpriteFileName, false, true);
      self.HQSpriteNr2 = BitmapStore.AddFile(self.HQSpriteFileName2, false, true);
      self.TempCounter = (Bitmap) null;
      self.NationalIconSprite = BitmapStore.AddFile(self.NationalIconSpriteName, false, true);
      self.RoundelIconSprite = BitmapStore.AddFile(self.RoundelSpriteName, false);
      self.RoundelIconSprite2 = BitmapStore.AddFile(self.RoundelSpriteName2, false);
      self.BannerSpriteNr = BitmapStore.AddFile(self.BannerSpriteFileName, false);
      self.BannerSpriteNr2 = BitmapStore.AddFile(self.BannerSpriteFileName2, false);
      self.SymbolSpriteNr = BitmapStore.AddFile(self.SymbolSpriteName, false);
    }

    pub fn ReplaceNationalSprite(filename: String)
    {
      self.NationalIconSpriteName = filename;
      self.NationalIconSprite = BitmapStore.ReloadFile(self.NationalIconSprite, filename, IsBig: true);
    }

    pub fn ReplaceRoundelSprite(filename: String)
    {
      self.RoundelSpriteName = filename;
      self.RoundelIconSprite = BitmapStore.ReloadFile(self.RoundelIconSprite, filename);
    }

    pub fn ReplaceBannerSprite(filename: String)
    {
      self.BannerSpriteFileName = filename;
      self.BannerSpriteNr = BitmapStore.ReloadFile(self.BannerSpriteNr, filename);
    }

    pub fn ReplaceSymbolSprite(filename: String)
    {
      self.SymbolSpriteName = filename;
      self.SymbolSpriteNr = BitmapStore.ReloadFile(self.SymbolSpriteNr, filename);
    }

    pub fn ReplaceHQSprite(filename: String)
    {
      self.HQSpriteFileName = filename;
      self.HQSpriteNr = BitmapStore.AddFile(self.HQSpriteFileName, false, true);
      if (DrawMod.TGame.Data.Product >= 7)
        return;
      self.DoTempCounter();
    }

    pub fn ReplaceRoundelSprite2(filename: String)
    {
      self.RoundelSpriteName2 = filename;
      self.RoundelIconSprite2 = BitmapStore.ReloadFile(self.RoundelIconSprite2, filename);
    }

    pub fn ReplaceHQSprite2(filename: String)
    {
      self.HQSpriteFileName2 = filename;
      self.HQSpriteNr2 = BitmapStore.AddFile(self.HQSpriteFileName2, false, true);
      if (DrawMod.TGame.Data.Product >= 7)
        return;
      self.DoTempCounter();
    }

    pub fn ReplaceBannerSprite2(filename: String)
    {
      self.BannerSpriteFileName2 = filename;
      self.BannerSpriteNr2 = BitmapStore.ReloadFile(self.BannerSpriteNr2, filename);
    }
  }
}
