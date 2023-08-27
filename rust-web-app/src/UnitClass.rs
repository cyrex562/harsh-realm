// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class UnitClass : ISerializable
  {
    pub Name: String;
    pub SFCount: i32;
    pub SFList: Vec<i32>;
    pub Regime: i32;
    pub HQ: i32;
    pub IsHQ: bool;
    pub X: i32;
    pub Y: i32;
    pub Map: i32;
    pub LandCap: i32;
    pub NavyCap: i32;
    pub AirCap: i32;
    pub DidAttack: bool;
    pub Spotted: bool;
    pub Identified: bool;
    pub Supply: i32;
    pub DidMove: bool;
    pub DidHQ: bool;
    pub offensiveCombat: i32;
    pub defensiveCombat: i32;
    pub MoveAPSpent: i32;
    pub SOInterceptRdnStop: i32;
    pub SODefendPercent: i32;
    pub SOSupReqPercent: i32;
    pub StartIntercept: i32;
    pub SOReplacementPercent: i32;
    pub SODoAS: bool;
    pub SOInterceptFire: i32;
    pub SOReturnFire: i32;
    pub LastSupplyPercent: i32;
    pub SupplyConsume: i32;
    pub SupplyIn: i32;
    pub Fuel: i32;
    pub FuelRequested: i32;
    pub FuelReceived: i32;
    pub SupplyInReq: i32;
    pub SupplyOut: i32;
    pub SupplyReq: i32;
    pub LandStart: i32;
    pub NavyStart: i32;
    pub AirStart: i32;
    pub LandNeed: i32;
    pub NavyNeed: i32;
    pub AirNeed: i32;
    pub OnBoard: i32;
    pub PassengerCounter: i32;
    pub PassengerList: Vec<i32>;
    pub attachedTo: i32;
    pub TransportCounter: i32;
    pub TransportList: Vec<i32>;
    pub moveMode: i32;
    pub FreeCombatX: i32;
    pub FreeCombatY: i32;
    pub FreeCombatMap: i32;
    pub Red: i32;
    pub Blue: i32;
    pub Green: i32;
    pub PreDef: i32;
    pub UnitIsGiven: bool;
    pub AIPlanClass AIPlanRef;
    pub AIPlanNr: i32;
    pub AIUnitGoal: i32;
    pub AIMobilize: bool;
    pub AIDisband: bool;
    pub AIReserve: bool;
    pub AINavtargetX: i32;
    pub AINavtargetY: i32;
    pub AICutoff: i32;
    pub AISubStrictGroup: i32;
    pub AIoriginallyEncircled: bool;
    pub AIoriginallyRetreating: bool;
    pub TempX: i32;
    pub FinalX: i32;
    pub OldX: i32;
    pub TempY: i32;
    pub FinalY: i32;
    pub OldY: i32;
    pub TempSlot: i32;
    pub AIAttack: i32;
    pub AIDefend: i32;
    pub AIFollowup: i32;
    pub AIFallback: i32;
    pub AILeftFlank: i32;
    pub AIRightFlank: i32;
    pub AIGroup: i32;
    pub AIBottleneck: i32;
    pub IsVirtual: bool;
    pub AIAttackStyle: i32;
    pub SupplyLost: i32;
    pub Reserve: i32;
    pub SetAPToZero: i32;
    pub LastAP: i32;
    pub LastMove: bool;
    pub TempOldNr: i32;
    pub Historical: i32;
    pub HistoricalSubPart: i32;
    pub TempType: i32;
    pub TempTypeRoad: i32;
    pub TempTheater: i32;
    pub TempCategory: i32;
    pub TempCategory2: i32;
    pub TempArtDam: i32;
    pub TempEncircled: i32;
    pub TempStrategic: bool;
    pub TempCapType: i32;
    pub TempCapHQ: i32;
    pub TempAttacked: bool;
    pub TempGroup: i32;
    pub TempAIString: String;
    pub TempAIGroup: i32;
    pub TempTopUnit: bool;
    pub TempProtector: bool;
    pub TempAIForbidsMove: bool;
    pub TempLisItemPercentage: i32;
    pub TempJustArrivedFromReserve: bool;
    pub AIid: i32;
    pub TempUnitPower: i32;
    pub TempUnitPowerAbs: i32;
    pub TempUnitAP: i32;
    pub float TempCombatImprovePercent;
    pub TempOwner: i32;
    pub StartPower: i32;
    pub AIModel: i32;
    pub TempUnitSelectable: bool;
    pub FuelUsedMove: i32;
    pub FuelUsedAtt: i32;
    pub FuelUsedDef: i32;
    pub LogCounter: i32;
    pub LogType: Vec<i32>;
    pub LogData1: Vec<i32>;
    pub LogData2: Vec<i32>;
    pub LogData3: Vec<i32>;
    pub StockpileCurrent: i32;
    pub ItemList items;
    pub SimpleList lisInstructions;
    pub SimpleList tempHandledItems;
    pub SimpleList tempRequestItems;
    pub SimpleList tempMaxItems;
    pub tempLisStartHistory: i32;
    pub CoordList tempCoords;
    pub ComplexCoordList tempComplexCoords;
    pub CoordList prevTurnAiFrontHexes;
    pub tempCanAttack: bool;
    pub tempCanAttack2: bool;
    pub apReserve: i32;
    pub LibIdClass LibId;
    pub supplyBaseSupplyIn: i32;
    pub supplyBaseFuelIn: i32;
    pub supplyX: i32;
    pub supplyY: i32;
    pub uncertaintyRolls: i32;
    pub uncertaintyRollsCount: i32;
    pub autoMoveX: i32;
    pub autoMoveY: i32;
    pub txtLog: String;
    pub tempSFTypeBitmap: Bitmap;
    pub long cycleOrder;

    pub UnitClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      let mut num: i32 = num;
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (UnitClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("SFCount", self.SFCount);
      info.AddValue("SFList",  self.SFList);
      info.AddValue("Regime", self.Regime);
      info.AddValue("Supply", self.Supply);
      info.AddValue("HQ", self.HQ);
      info.AddValue("IsHQ", self.IsHQ);
      info.AddValue("X", self.X);
      info.AddValue("Y", self.Y);
      info.AddValue("LandCap", self.LandCap);
      info.AddValue("NavyCap", self.NavyCap);
      info.AddValue("AirCap", self.AirCap);
      info.AddValue("SOInterceptRdnStop", self.SOInterceptRdnStop);
      info.AddValue("SODefendPercent", self.SODefendPercent);
      info.AddValue("SOSupReqPercent", self.SOSupReqPercent);
      info.AddValue("SOReplacementPercent", self.SOReplacementPercent);
      info.AddValue("LastSupplyPercent", self.LastSupplyPercent);
      info.AddValue("SupplyConsume", self.SupplyConsume);
      info.AddValue("PassengerCounter", self.PassengerCounter);
      info.AddValue("PassengerList",  self.PassengerList);
      info.AddValue("FreeCombatX", self.FreeCombatX);
      info.AddValue("FreeCombatY", self.FreeCombatY);
      info.AddValue("FreeCombatMap", self.FreeCombatMap);
      info.AddValue("Red", self.Red);
      info.AddValue("Blue", self.Blue);
      info.AddValue("Green", self.Green);
      info.AddValue("PreDef", self.PreDef);
      info.AddValue("SupplyIn", self.SupplyIn);
      info.AddValue("SupplyInReq", self.SupplyInReq);
      info.AddValue("SupplyOut", self.SupplyOut);
      info.AddValue("SupplyReq", self.SupplyReq);
      info.AddValue("LandStart", self.LandStart);
      info.AddValue("NavyStart", self.NavyStart);
      info.AddValue("AirStart", self.AirStart);
      info.AddValue("LandNeed", self.LandNeed);
      info.AddValue("NavyNeed", self.NavyNeed);
      info.AddValue("AirNeed", self.AirNeed);
      info.AddValue("OnBoard", self.OnBoard);
      info.AddValue("SupplyLost", self.SupplyLost);
      info.AddValue("Reserve", self.Reserve);
      info.AddValue("TransportCounter", self.TransportCounter);
      info.AddValue("TransportList",  self.TransportList);
      info.AddValue("attachedTo", self.attachedTo);
      info.AddValue("moveMode", self.moveMode);
      info.AddValue("AIPlanRef",  self.AIPlanRef);
      info.AddValue("AIUnitGoal", self.AIUnitGoal);
      info.AddValue("AIMobilize", self.AIMobilize);
      info.AddValue("AINavtargetX", self.AINavtargetX);
      info.AddValue("AINavtargetY", self.AINavtargetY);
      info.AddValue("StartIntercept", self.StartIntercept);
      info.AddValue("SODoAS", self.SODoAS);
      info.AddValue("Historical", self.Historical);
      info.AddValue("HistoricalSubPart", self.HistoricalSubPart);
      info.AddValue("AIAttack", self.AIAttack);
      info.AddValue("AIDefend", self.AIDefend);
      info.AddValue("AIFollowup", self.AIFollowup);
      info.AddValue("AIFallback", self.AIFallback);
      info.AddValue("AILeftFlank", self.AIBottleneck);
      info.AddValue("AIRightFlank", self.AIRightFlank);
      info.AddValue("AIGroup", self.AIGroup);
      info.AddValue("AIAttackStyle", self.AIAttackStyle);
      info.AddValue("StartPower", self.StartPower);
      info.AddValue("DidAttack", self.DidAttack);
      info.AddValue("Spotted", self.Spotted);
      info.AddValue("Identified", self.Identified);
      info.AddValue("Map", self.Map);
      info.AddValue("AIModel", self.AIModel);
      info.AddValue("DidMove", self.DidMove);
      info.AddValue("UnitIsGiven", self.UnitIsGiven);
      info.AddValue("FuelUsedMove", self.FuelUsedMove);
      info.AddValue("FuelUsedAtt", self.FuelUsedAtt);
      info.AddValue("FuelUsedDef", self.FuelUsedDef);
      info.AddValue("LogCounter", self.LogCounter);
      info.AddValue("LogType",  self.LogType);
      info.AddValue("LogData1",  self.LogData1);
      info.AddValue("LogData2",  self.LogData2);
      info.AddValue("LogData3",  self.LogData3);
      info.AddValue("StockpileCurrent", self.StockpileCurrent);
      info.AddValue("DidHQ", self.DidHQ);
      info.AddValue("offensiveCombat", self.offensiveCombat);
      info.AddValue("defensiveCombat", self.defensiveCombat);
      info.AddValue("moveApSpent", self.MoveAPSpent);
      info.AddValue("LibId",  self.LibId);
      info.AddValue("AISubStrictGroup", self.AISubStrictGroup);
      info.AddValue("SOInterceptFire", self.SOInterceptFire);
      info.AddValue("SOReturnFire", self.SOReturnFire);
      info.AddValue("Fuel", self.Fuel);
      info.AddValue("FuelRequested", self.FuelRequested);
      info.AddValue("FuelReceived", self.FuelReceived);
      info.AddValue("supplyBaseSupplyIn", self.supplyBaseSupplyIn);
      info.AddValue("supplyBaseFuelIn", self.supplyBaseFuelIn);
      if (Information.IsNothing( self.items))
        self.items = ItemList::new();
      info.AddValue("items",  self.items);
      if (Information.IsNothing( self.lisInstructions))
        self.lisInstructions = SimpleList::new();
      info.AddValue("lisInstructions",  self.lisInstructions);
      info.AddValue("ApReserve", self.apReserve);
      info.AddValue("supplyX", self.supplyX);
      info.AddValue("supplyY", self.supplyY);
      info.AddValue("prevTurnAiFrontHexes",  self.prevTurnAiFrontHexes);
      info.AddValue("cycleOrder", self.cycleOrder);
    }

    protected UnitClass(SerializationInfo info, StreamingContext context)
    {
      self.SFList = new int[1];
      self.PassengerList = new int[1];
      self.TransportList = new int[1];
      self.moveMode = 0;
      self.LogType = new int[1];
      self.LogData1 = new int[1];
      self.LogData2 = new int[1];
      self.LogData3 = new int[1];
      self.txtLog = "";
      self.Name = info.GetString(nameof (Name));
      self.SFCount = info.GetInt32(nameof (SFCount));
      self.SFList = self.SFCount <= -1 ? new int[1] : new int[self.SFCount + 1];
      self.SFList = (int[]) info.GetValue(nameof (SFList), self.SFList.GetType());
      self.Regime = info.GetInt32(nameof (Regime));
      self.Supply = info.GetInt32(nameof (Supply));
      self.HQ = info.GetInt32(nameof (HQ));
      self.IsHQ = info.GetBoolean(nameof (IsHQ));
      self.X = info.GetInt32(nameof (X));
      self.Y = info.GetInt32(nameof (Y));
      self.LandCap = info.GetInt32(nameof (LandCap));
      self.NavyCap = info.GetInt32(nameof (NavyCap));
      self.AirCap = info.GetInt32(nameof (AirCap));
      self.SOInterceptRdnStop = info.GetInt32(nameof (SOInterceptRdnStop));
      self.SODefendPercent = info.GetInt32(nameof (SODefendPercent));
      self.SOSupReqPercent = info.GetInt32(nameof (SOSupReqPercent));
      self.LastSupplyPercent = info.GetInt32(nameof (LastSupplyPercent));
      self.SupplyConsume = info.GetInt32(nameof (SupplyConsume));
      self.PassengerCounter = info.GetInt32(nameof (PassengerCounter));
      self.PassengerList = self.PassengerCounter <= -1 ? new int[1] : new int[self.PassengerCounter + 1];
      self.PassengerList = (int[]) info.GetValue(nameof (PassengerList), self.PassengerList.GetType());
      self.FreeCombatX = info.GetInt32(nameof (FreeCombatX));
      self.FreeCombatY = info.GetInt32(nameof (FreeCombatY));
      self.Red = info.GetInt32(nameof (Red));
      self.Green = info.GetInt32(nameof (Green));
      self.Blue = info.GetInt32(nameof (Blue));
      self.PreDef = info.GetInt32(nameof (PreDef));
      self.SupplyIn = info.GetInt32(nameof (SupplyIn));
      self.SupplyInReq = info.GetInt32(nameof (SupplyInReq));
      self.SupplyOut = info.GetInt32(nameof (SupplyOut));
      self.SupplyReq = info.GetInt32(nameof (SupplyReq));
      self.LandStart = info.GetInt32(nameof (LandStart));
      self.NavyStart = info.GetInt32(nameof (NavyStart));
      self.AirStart = info.GetInt32(nameof (AirStart));
      self.LandNeed = info.GetInt32(nameof (LandNeed));
      self.NavyNeed = info.GetInt32(nameof (NavyNeed));
      self.AirNeed = info.GetInt32(nameof (AirNeed));
      self.OnBoard = info.GetInt32(nameof (OnBoard));
      self.SupplyLost = info.GetInt32(nameof (SupplyLost));
      self.Reserve = info.GetInt32(nameof (Reserve));
      self.AIPlanRef = (AIPlanClass) info.GetValue(nameof (AIPlanRef), typeof (AIPlanClass));
      self.AIUnitGoal = info.GetInt32(nameof (AIUnitGoal));
      self.AIMobilize = info.GetBoolean(nameof (AIMobilize));
      self.AINavtargetX = info.GetInt32(nameof (AINavtargetX));
      self.AINavtargetY = info.GetInt32(nameof (AINavtargetY));
      self.StartIntercept = info.GetInt32(nameof (StartIntercept));
      self.SODoAS = info.GetBoolean(nameof (SODoAS));
      self.LastMove = false;
      self.LastAP = -1;
      if (DrawMod.TGame.Data.Version < 130)
      {
        self.SOReplacementPercent = 100;
        self.FreeCombatMap = 0;
        self.Historical = -1;
        self.HistoricalSubPart = -1;
        self.AIAttack = -1;
        self.AIDefend = -1;
        self.AILeftFlank = -1;
        self.AIRightFlank = -1;
        self.AIFallback = -1;
        self.AIFollowup = -1;
        self.AIGroup = -1;
        self.AIAttackStyle = 2;
        self.StartPower = 0;
        self.DidAttack = false;
        self.Spotted = false;
        self.Identified = false;
        self.Map = 0;
        self.AIModel = -1;
        self.UnitIsGiven = false;
      }
      else if (DrawMod.TGame.Data.Version < 158)
      {
        try
        {
          self.SOReplacementPercent = info.GetInt32(nameof (SOReplacementPercent));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.SOReplacementPercent = 100;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.FreeCombatMap = info.GetInt32(nameof (FreeCombatMap));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.FreeCombatMap = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Historical = info.GetInt32(nameof (Historical));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Historical = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.HistoricalSubPart = info.GetInt32(nameof (HistoricalSubPart));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.HistoricalSubPart = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.AIAttack = info.GetInt32(nameof (AIAttack));
          self.AIDefend = info.GetInt32(nameof (AIDefend));
          self.AIFollowup = info.GetInt32(nameof (AIFollowup));
          self.AIFallback = info.GetInt32(nameof (AIFallback));
          self.AILeftFlank = -1;
          self.AIBottleneck = info.GetInt32(nameof (AILeftFlank));
          self.AIRightFlank = info.GetInt32(nameof (AIRightFlank));
          self.AIGroup = info.GetInt32(nameof (AIGroup));
          self.AIAttackStyle = info.GetInt32(nameof (AIAttackStyle));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AIAttack = -1;
          self.AIDefend = -1;
          self.AILeftFlank = -1;
          self.AIRightFlank = -1;
          self.AIFallback = -1;
          self.AIFollowup = -1;
          self.AIGroup = -1;
          self.AIAttackStyle = 2;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.StartPower = info.GetInt32(nameof (StartPower));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.StartPower = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.DidAttack = info.GetBoolean(nameof (DidAttack));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.DidAttack = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Spotted = info.GetBoolean(nameof (Spotted));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Spotted = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Identified = info.GetBoolean(nameof (Identified));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Identified = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Map = info.GetInt32(nameof (Map));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Map = 0;
          ProjectData.ClearProjectError();
        }
        self.AIModel = -1;
        self.UnitIsGiven = false;
      }
      else
      {
        self.SOReplacementPercent = info.GetInt32(nameof (SOReplacementPercent));
        self.FreeCombatMap = info.GetInt32(nameof (FreeCombatMap));
        self.Historical = info.GetInt32(nameof (Historical));
        self.HistoricalSubPart = info.GetInt32(nameof (HistoricalSubPart));
        self.AIAttack = info.GetInt32(nameof (AIAttack));
        self.AIDefend = info.GetInt32(nameof (AIDefend));
        self.AIFollowup = info.GetInt32(nameof (AIFollowup));
        self.AIFallback = info.GetInt32(nameof (AIFallback));
        self.AILeftFlank = info.GetInt32(nameof (AILeftFlank));
        self.AIRightFlank = info.GetInt32(nameof (AIRightFlank));
        self.AIGroup = info.GetInt32(nameof (AIGroup));
        self.AIAttackStyle = info.GetInt32(nameof (AIAttackStyle));
        self.StartPower = info.GetInt32(nameof (StartPower));
        self.DidAttack = info.GetBoolean(nameof (DidAttack));
        self.Map = info.GetInt32(nameof (Map));
        try
        {
          self.AIModel = info.GetInt32(nameof (AIModel));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AIModel = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.DidMove = info.GetBoolean(nameof (DidMove));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.DidMove = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.UnitIsGiven = info.GetBoolean(nameof (UnitIsGiven));
          self.FuelUsedMove = info.GetInt32(nameof (FuelUsedMove));
          self.FuelUsedAtt = info.GetInt32(nameof (FuelUsedAtt));
          self.FuelUsedDef = info.GetInt32(nameof (FuelUsedDef));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.UnitIsGiven = false;
          self.FuelUsedMove = 0;
          self.FuelUsedAtt = 0;
          self.FuelUsedDef = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Spotted = info.GetBoolean(nameof (Spotted));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Spotted = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Identified = info.GetBoolean(nameof (Identified));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Identified = false;
          ProjectData.ClearProjectError();
        }
      }
      try
      {
        self.LogCounter = info.GetInt32(nameof (LogCounter));
        self.LogType = new int[self.LogCounter + 1];
        self.LogData1 = new int[self.LogCounter + 1];
        self.LogData2 = new int[self.LogCounter + 1];
        self.LogData3 = new int[self.LogCounter + 1];
        self.LogType = (int[]) info.GetValue(nameof (LogType), self.LogType.GetType());
        self.LogData1 = (int[]) info.GetValue(nameof (LogData1), self.LogData1.GetType());
        self.LogData2 = (int[]) info.GetValue(nameof (LogData2), self.LogData2.GetType());
        self.LogData3 = (int[]) info.GetValue(nameof (LogData3), self.LogData3.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LogCounter = -1;
        self.LogType = new int[1];
        self.LogData1 = new int[1];
        self.LogData2 = new int[1];
        self.LogData3 = new int[1];
        ProjectData.ClearProjectError();
      }
      try
      {
        self.StockpileCurrent = info.GetInt32(nameof (StockpileCurrent));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.StockpileCurrent = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.DidHQ = info.GetBoolean(nameof (DidHQ));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.DidHQ = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.offensiveCombat = info.GetInt32(nameof (offensiveCombat));
        self.defensiveCombat = info.GetInt32(nameof (defensiveCombat));
        self.MoveAPSpent = info.GetInt32("moveApSpent");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.offensiveCombat = 0;
        self.defensiveCombat = 0;
        self.MoveAPSpent = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.LibId = LibIdClass::new();
        self.LibId = (LibIdClass) info.GetValue(nameof (LibId), self.LibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LibId = LibIdClass::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        self.AISubStrictGroup = info.GetInt32(nameof (AISubStrictGroup));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.AISubStrictGroup = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.TransportCounter = info.GetInt32(nameof (TransportCounter));
        self.TransportList = self.TransportCounter <= -1 ? new int[1] : new int[self.TransportCounter + 1];
        self.TransportList = (int[]) info.GetValue(nameof (TransportList), self.TransportList.GetType());
        self.attachedTo = info.GetInt32(nameof (attachedTo));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.TransportCounter = -1;
        self.TransportList = new int[1];
        self.attachedTo = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.moveMode = info.GetInt32(nameof (moveMode));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.moveMode = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        if (Information.IsNothing( self.items))
          self.items = ItemList::new();
        self.items = (ItemList) info.GetValue(nameof (items), self.items.GetType());
        if (Information.IsNothing( self.items))
          self.items = ItemList::new();
        if (Information.IsNothing( self.lisInstructions))
          self.lisInstructions = SimpleList::new();
        self.lisInstructions = (SimpleList) info.GetValue(nameof (lisInstructions), self.lisInstructions.GetType());
        if (Information.IsNothing( self.lisInstructions))
          self.lisInstructions = SimpleList::new();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.items = ItemList::new();
        self.lisInstructions = SimpleList::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Fuel = info.GetInt32(nameof (Fuel));
        self.FuelRequested = info.GetInt32(nameof (FuelRequested));
        self.FuelReceived = info.GetInt32(nameof (FuelReceived));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Fuel = 0;
        self.FuelReceived = 0;
        self.FuelRequested = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.supplyBaseSupplyIn = info.GetInt32(nameof (supplyBaseSupplyIn));
        self.supplyBaseFuelIn = info.GetInt32(nameof (supplyBaseFuelIn));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.supplyBaseSupplyIn = 0;
        self.supplyBaseFuelIn = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.SOInterceptFire = info.GetInt32(nameof (SOInterceptFire));
        self.SOReturnFire = info.GetInt32(nameof (SOReturnFire));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SOInterceptFire = 33;
        self.SOReturnFire = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.apReserve = info.GetInt32("ApReserve");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.apReserve = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.supplyX = info.GetInt32(nameof (supplyX));
        self.supplyY = info.GetInt32(nameof (supplyY));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.supplyX = -1;
        self.supplyY = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.prevTurnAiFrontHexes = CoordList::new();
        self.prevTurnAiFrontHexes = (CoordList) info.GetValue(nameof (prevTurnAiFrontHexes), self.prevTurnAiFrontHexes.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.prevTurnAiFrontHexes = (CoordList) null;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.cycleOrder = info.GetInt64(nameof (cycleOrder));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.cycleOrder = 0L;
        ProjectData.ClearProjectError();
      }
      self.TempOwner = -1;
      self.TempCapType = -1;
      self.TempCapHQ = -1;
    }

    pub fn AddLog(type: i32, data1: i32, data2: i32, data3: i32)
    {
      let mut num1: i32 = -1;
      if (Strings.InStr(self.Name, "LVIII") > 0)
        num1 = num1;
      if (type == 605)
        num1 = num1;
      let mut logCounter: i32 = self.LogCounter;
      for (let mut index: i32 = 0; index <= logCounter; index += 1)
      {
        if (self.LogType[index] == type & self.LogData1[index] == data1 & self.LogData2[index] == data2)
        {
          num1 = index;
          break;
        }
      }
      if (num1 == -1)
      {
        self += 1.LogCounter;
        self.LogType = (int[]) Utils.CopyArray((Array) self.LogType, (Array) new int[self.LogCounter + 1]);
        self.LogData1 = (int[]) Utils.CopyArray((Array) self.LogData1, (Array) new int[self.LogCounter + 1]);
        self.LogData2 = (int[]) Utils.CopyArray((Array) self.LogData2, (Array) new int[self.LogCounter + 1]);
        self.LogData3 = (int[]) Utils.CopyArray((Array) self.LogData3, (Array) new int[self.LogCounter + 1]);
        self.LogType[self.LogCounter] = type;
        self.LogData1[self.LogCounter] = data1;
        self.LogData2[self.LogCounter] = data2;
        self.LogData3[self.LogCounter] = data3;
      }
      else
      {
        int[] logData3 = self.LogData3;
        int[] numArray = logData3;
        let mut index1: i32 = num1;
        let mut index2: i32 = index1;
        let mut num2: i32 = logData3[index1] + data3;
        numArray[index2] = num2;
      }
    }

    pub fn SetLog(type: i32, data1: i32, data2: i32, data3: i32)
    {
      let mut index1: i32 = -1;
      let mut logCounter: i32 = self.LogCounter;
      for (let mut index2: i32 = 0; index2 <= logCounter; index2 += 1)
      {
        if (self.LogType[index2] == type & self.LogData1[index2] == data1 & self.LogData2[index2] == data2)
        {
          index1 = index2;
          break;
        }
      }
      if (index1 == -1)
      {
        self += 1.LogCounter;
        self.LogType = (int[]) Utils.CopyArray((Array) self.LogType, (Array) new int[self.LogCounter + 1]);
        self.LogData1 = (int[]) Utils.CopyArray((Array) self.LogData1, (Array) new int[self.LogCounter + 1]);
        self.LogData2 = (int[]) Utils.CopyArray((Array) self.LogData2, (Array) new int[self.LogCounter + 1]);
        self.LogData3 = (int[]) Utils.CopyArray((Array) self.LogData3, (Array) new int[self.LogCounter + 1]);
        self.LogType[self.LogCounter] = type;
        self.LogData1[self.LogCounter] = data1;
        self.LogData2[self.LogCounter] = data2;
        self.LogData3[self.LogCounter] = data3;
      }
      else
        self.LogData3[index1] = data3;
    }

    pub fn CheckLogReturnData3(type: i32, data1: i32) -> i32
    {
      let mut logCounter: i32 = self.LogCounter;
      for (let mut index: i32 = 0; index <= logCounter; index += 1)
      {
        if (self.LogType[index] == type & (self.LogData1[index] == data1 | data1 == -1))
          return self.LogData3[index];
      }
      return 0;
    }

    pub fn ClearLogs()
    {
      self.LogCounter = -1;
      self.LogType = (int[]) Utils.CopyArray((Array) self.LogType, (Array) new int[1]);
      self.LogData1 = (int[]) Utils.CopyArray((Array) self.LogData1, (Array) new int[1]);
      self.LogData2 = (int[]) Utils.CopyArray((Array) self.LogData2, (Array) new int[1]);
      self.LogData3 = (int[]) Utils.CopyArray((Array) self.LogData3, (Array) new int[1]);
    }

    pub fn AddSF(sfnr: i32)
    {
      self += 1.SFCount;
      self.SFList = (int[]) Utils.CopyArray((Array) self.SFList, (Array) new int[self.SFCount + 1]);
      self.SFList[self.SFCount] = sfnr;
    }

    pub fn RemoveSF(sfnr: i32)
    {
      let mut sfCount: i32 = self.SFCount;
      for (let mut index1: i32 = 0; index1 <= sfCount; index1 += 1)
      {
        if (self.SFList[index1] == sfnr)
        {
          if (index1 < self.SFCount)
          {
            let mut num1: i32 = index1;
            let mut num2: i32 = self.SFCount - 1;
            for (let mut index2: i32 = num1; index2 <= num2; index2 += 1)
              self.SFList[index2] = self.SFList[index2 + 1];
          }
          --self.SFCount;
          self.SFList = (int[]) Utils.CopyArray((Array) self.SFList, (Array) new int[self.SFCount + 1]);
          break;
        }
      }
    }

    pub fn AddPassenger(pnr: i32)
    {
      self += 1.PassengerCounter;
      self.PassengerList = (int[]) Utils.CopyArray((Array) self.PassengerList, (Array) new int[self.PassengerCounter + 1]);
      self.PassengerList[self.PassengerCounter] = pnr;
    }

    pub fn RemovePassenger(pnr: i32)
    {
      let mut passengerCounter: i32 = self.PassengerCounter;
      for (let mut index1: i32 = 0; index1 <= passengerCounter; index1 += 1)
      {
        if (self.PassengerList[index1] == pnr)
        {
          if (index1 < self.PassengerCounter)
          {
            let mut num1: i32 = index1;
            let mut num2: i32 = self.PassengerCounter - 1;
            for (let mut index2: i32 = num1; index2 <= num2; index2 += 1)
              self.PassengerList[index2] = self.PassengerList[index2 + 1];
          }
          --self.PassengerCounter;
          self.PassengerList = (int[]) Utils.CopyArray((Array) self.PassengerList, (Array) new int[self.PassengerCounter + 1]);
          break;
        }
      }
    }

    pub fn AddTransport(pnr: i32)
    {
      self += 1.TransportCounter;
      self.TransportList = (int[]) Utils.CopyArray((Array) self.TransportList, (Array) new int[self.TransportCounter + 1]);
      self.TransportList[self.TransportCounter] = pnr;
    }

    pub fn RemoveTransport(pnr: i32)
    {
      let mut transportCounter: i32 = self.TransportCounter;
      for (let mut index1: i32 = 0; index1 <= transportCounter; index1 += 1)
      {
        if (self.TransportList[index1] == pnr)
        {
          if (index1 < self.TransportCounter)
          {
            let mut num1: i32 = index1;
            let mut num2: i32 = self.TransportCounter - 1;
            for (let mut index2: i32 = num1; index2 <= num2; index2 += 1)
              self.TransportList[index2] = self.TransportList[index2 + 1];
          }
          --self.TransportCounter;
          self.TransportList = (int[]) Utils.CopyArray((Array) self.TransportList, (Array) new int[self.TransportCounter + 1]);
          break;
        }
      }
    }

    pub UnitClass(hardcoded: i32)
    {
      self.SFList = new int[1];
      self.PassengerList = new int[1];
      self.TransportList = new int[1];
      self.moveMode = 0;
      self.LogType = new int[1];
      self.LogData1 = new int[1];
      self.LogData2 = new int[1];
      self.LogData3 = new int[1];
      self.txtLog = "";
      self.Name = "New Unit";
      self.SFCount = -1;
      self.AIModel = -1;
      self.TempGroup = -1;
      self.txtLog = "";
      self.Regime = -1;
      self.HQ = -1;
      self.IsHQ = false;
      self.cycleOrder = 0L;
      self.uncertaintyRolls = 0;
      self.uncertaintyRollsCount = 0;
      self.autoMoveX = -1;
      self.autoMoveY = -1;
      self.supplyX = -1;
      self.supplyY = -1;
      self.TempCapType = -1;
      self.TempCapHQ = -1;
      self.Supply = 0;
      self.PassengerCounter = -1;
      self.TransportCounter = -1;
      self.attachedTo = -1;
      self.moveMode = 0;
      self.OnBoard = -1;
      self.FreeCombatX = -1;
      self.FreeCombatY = -1;
      self.Red =  Math.Round( VBMath.Rnd() * 215.0 + 40.0);
      self.Green =  Math.Round( VBMath.Rnd() * 215.0 + 40.0);
      self.Blue =  Math.Round( VBMath.Rnd() * 215.0 + 40.0);
      self.PreDef = -1;
      self.SODoAS = true;
      self.Historical = -1;
      self.apReserve = 0;
      self.AIAttack = -1;
      self.AIDefend = -1;
      self.AILeftFlank = -1;
      self.AIRightFlank = -1;
      self.AIFallback = -1;
      self.AIFollowup = -1;
      self.SOReplacementPercent = 100;
      self.SOSupReqPercent = 100;
      self.SOInterceptRdnStop = 100;
      self.SODefendPercent = 50;
      self.SupplyConsume = 100;
      self.Map = 0;
      self.TempOwner = -1;
      self.LibId = LibIdClass::new();
      self.AISubStrictGroup = -1;
      self.SOInterceptFire = 33;
      self.SOReturnFire = 0;
      self.supplyBaseSupplyIn = 0;
      self.supplyBaseFuelIn = 0;
      self.items = ItemList::new();
      self.lisInstructions = SimpleList::new();
    }

    pub fn ResetAI()
    {
      self.AIAttack = -1;
      self.AIDefend = -1;
      self.AILeftFlank = -1;
      self.AIRightFlank = -1;
      self.AIFallback = -1;
      self.AIFollowup = -1;
    }

    pub fn RealX( game: GameClass) -> i32 => self.OnBoard > -1 ? game.Data.UnitObj[self.OnBoard].X : self.X;

    pub fn RealY( game: GameClass) -> i32 => self.OnBoard > -1 ? game.Data.UnitObj[self.OnBoard].Y : self.Y;

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub fn Kill()
    {
      if (self.SFCount <= -1)
        return;
      let mut num: i32 =  Interaction.MsgBox( "Programm attempting to delete Unit with Subformations in it. END.", Title: ( "Error"));
      ProjectData.EndApp();
    }

    pub fn LoadSprites()
    {
    }
  }
}
