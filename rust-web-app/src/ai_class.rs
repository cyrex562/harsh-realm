// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Imaging;
// usingSystem.IO;
// usingSystem.Windows.Forms;

use game_class::GameClass;


pub const PLANLANDFRONT: i32 =  20;
pub const PLANLANDRESERVE: i32 =  30;
pub const PLANBACK: i32 =  40;
pub const PLANOLDLANDFRONT: i32 =  50;
pub const STANDATTACK: i32 =  1;
pub const STANDDEFEND: i32 =  2;
pub const STANDRETREAT: i32 =  3;
pub const STANDHOME: i32 =  4;
pub const STANDRAID: i32 =  5;
pub const STANDSEASUP: i32 =  6;
pub const STANDAMPH: i32 =  7;
pub const STANDHOME2: i32 =  8;
pub const ROLESTAFF: i32 =  1;
pub const ROLELANDCAP: i32 =  2;
pub const ROLESEACAP: i32 =  3;
pub const ROLEAIRCAP: i32 =  4;
pub const ROLEENGINEER: i32 =  5;
pub const ROLEINFANTRY: i32 =  6;
pub const ROLEINFANTRYSUPPORT: i32 =  7;
pub const ROLEARTILLERY: i32 =  8;
pub const ROLEMOBILIZER: i32 =  9;
pub const ROLEARMOUR: i32 =  10;
pub const ROLEPARATROOP: i32 =  11;
pub const ROLEAA: i32 =  12;
pub const ROLEFIGHTER: i32 =  13;
pub const ROLETACTICALBOMBER: i32 =  14;
pub const ROLESTRATEGICBOMBER: i32 =  15;
pub const ROLETRANSPORTER: i32 =  16;
pub const ROLECARGOSHIP: i32 =  17;
pub const ROLESEASUPRIORITY: i32 =  18;
pub const ROLERAIDER: i32 =  19;
pub const GOALINFANTRY: i32 =  1;
pub const GOALARMOUR: i32 =  2;
pub const GOALARTILLERY: i32 =  3;
pub const GOALENGINEER: i32 =  4;
pub const GOALAIRSUPPORT: i32 =  5;
pub const GOALSTRATEGICBOMBING: i32 =  6;
pub const GOALTRANSPORT: i32 =  7;
pub const GOALCARGO: i32 =  8;
pub const GOALNAVALWAR: i32 =  9;
pub const GOALRAIDER: i32 =  10;

#[derive(Default,Debug,Copy,Clone)]
pub struct AIClass
{
    pub TempAvgUnits: Vec<i32>,
    pub CombatMatrix: Vec<f32>,
    pub game: GameClass,
    pub LogTxt: Vec<String>,
    pub LogCounter: i32,
    pub LogTxt2: Vec<String>,
    pub LogCounter2: i32,
    pub HexOA: Vec<i32>,
    pub OACount: i32,
    pub HexContinent: Vec<i32>,
    pub ContinentCount: i32,
    pub HexSA: Vec<i32>,
    pub HexPlan: Vec<i32>,
    pub HexBackPlan: Vec<i32>,
    pub HexSeaSA: Vec<i32>,
    pub HexSAWithoutTemp: Vec<i32>,
    pub SACount: i32,
    pub SAObj: Vec<SAClass>,
    pub AIVP: Vec<i32>,
    pub Matrix1: Vec<i32>,
    pub Matrix2: Vec<i32>,
    pub UnitMovePhase: Vec<i32>,
    pub TPlanCount: i32,
    pub TPlanObj: Vec<AIPlanClass>,

}

  impl AIClass {

    pub fn new(tgame: GameClass) -> Self
    {
        let mut out = Self::default();
      out.TempAvgUnits = vec![];
      out.CombatMatrix = vec![];
      out.LogTxt = vec![];
      out.LogTxt2 = vec![];
      out.HexOA = new int[1, 1];
      out.HexContinent = new int[1, 1];
      out.HexSA = new int[1, 1];
      out.HexPlan = new int[1, 1];
      out.HexBackPlan = new int[1, 1];
      out.HexSeaSA = new int[1, 1];
      out.HexSAWithoutTemp = new int[1, 1];
      out.SAObj = new SAClass[1];
      out.AIVP = new int[1, 1];
      out.Matrix1 = new int[1, 1];
      out.Matrix2 = new int[1, 1];
      out.UnitMovePhase = new int[1];
      out.TPlanObj = new AIPlanClass[1];
      out.game = tgame;
      out
    }

    pub fn ExecuteAI(&mut self)
    {
      object[,] objArray = new object[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      float aiConservative = self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative;
      let mut tplanCount1: i32 =  self.TPlanCount;
      DateTime now;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        if (self.TPlanObj[index].Type == 40 && self.TPlanObj[index].FriendlyUnitCount > 0)
        {
          Application.DoEvents();
          now = DateTime.Now;
          long num1 = (long) Math.Round( now.Ticks / 1000.0);
          self.ExecLoadUnitOnBoard(index);
          self.ExecuteMovement(index, 1);
          self.ExecUnloadUnit(index);
          now = DateTime.Now;
          self.AddLog2("backplan ops 1 took " + Conversion.Str( ( now.Ticks / 1000.0 -  num1)));
          now = DateTime.Now;
          long num2 = (long) Math.Round( now.Ticks / 1000.0);
          self.ExecuteAirAttack(index, 1f * aiConservative);
          now = DateTime.Now;
          self.AddLog2("backplan ops 2 took " + Conversion.Str( ( now.Ticks / 1000.0 -  num2)));
          now = DateTime.Now;
          long num3 = (long) Math.Round( now.Ticks / 1000.0);
          self.ExecuteNavalAttacks(index, 1f * aiConservative);
          now = DateTime.Now;
          self.AddLog2("backplan ops 3 took " + Conversion.Str( ( now.Ticks / 1000.0 -  num3)));
          now = DateTime.Now;
          long num4 = (long) Math.Round( now.Ticks / 1000.0);
          self.ExecuteEngineer(index);
          self.ExecuteMovement(index, 2);
          self.ExecJoinUnits(index);
          self.ExecuteEngineer(index);
          self.ExecuteMovement(index, 2);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          now = DateTime.Now;
          self.AddLog2("backplan ops 4 took " + Conversion.Str( ( now.Ticks / 1000.0 -  num4)));
        }
      }
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount2; index += 1)
      {
        if (self.TPlanObj[index].Type == 50 && self.TPlanObj[index].FriendlyUnitCount > 0)
        {
          Application.DoEvents();
          now = DateTime.Now;
          long num = (long) Math.Round( now.Ticks / 1000.0);
          self.ExecuteMovement(index, 1);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          self.ExecuteEngineer(index);
          now = DateTime.Now;
          self.AddLog2("executeOLDlandfronts took " + Conversion.Str( ( now.Ticks / 1000.0 -  num)));
        }
      }
      let mut tplanCount3: i32 =  self.TPlanCount;
      for (let mut plannr: i32 =  1; plannr <= tplanCount3; plannr += 1)
      {
        if (self.TPlanObj[plannr].Type == 20 && self.TPlanObj[plannr].FriendlyUnitCount > 0)
        {
          Application.DoEvents();
          now = DateTime.Now;
          long num = (long) Math.Round( now.Ticks / 1000.0);
          self.ExecuteArtilleryAttack(plannr, 1f, 1);
          now = DateTime.Now;
          self.AddLog2("executeartilleryattack took " + Conversion.Str( ( now.Ticks / 1000.0 -  num)));
        }
      }
      let mut tplanCount4: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount4; index += 1)
      {
        if (self.TPlanObj[index].Type == 20)
        {
          if (self.TPlanObj[index].FriendlyUnitCount > 0)
          {
            Application.DoEvents();
            if ( self.game.Data.RuleVar[225] == 0.0)
            {
              now = DateTime.Now;
              long num5 = (long) Math.Round( now.Ticks / 1000.0);
              if (self.TPlanObj[index].Stand == 2)
                self.ExecuteLandFrontAttacks(index, 2f * aiConservative);
              else if (self.TPlanObj[index].Stand == 1)
              {
                float d = self.TPlanObj[index].WeightEnemyForceUnMod / self.TPlanObj[index].WeightFriendlyForce;
                if ( self.TPlanObj[index].WeightEnemyForce /  self.TPlanObj[index].WeightFriendlyForce <  d)
                  d = self.TPlanObj[index].WeightEnemyForce / self.TPlanObj[index].WeightFriendlyForce;
                float num6 =  Math.Sqrt( d);
                if ( num6 < 0.25)
                  num6 = 0.25f;
                if ( num6 > 1.0)
                  num6 = 1f;
                self.ExecuteLandFrontAttacks(index, 1.5f * num6 * aiConservative);
              }
              else if (self.TPlanObj[index].Stand == 3)
                self.ExecuteLandFrontAttacks(index, 2.5f * aiConservative);
              now = DateTime.Now;
              self.AddLog2("ExecuteLandFrontAttacks took " + Conversion.Str( ( now.Ticks / 1000.0 -  num5)));
              now = DateTime.Now;
              long num7 = (long) Math.Round( now.Ticks / 1000.0);
              self.ExecuteMovement(index, 1);
              now = DateTime.Now;
              self.AddLog2("ExecuteMovement took " + Conversion.Str( ( now.Ticks / 1000.0 -  num7)));
            }
            else
            {
              Application.DoEvents();
              now = DateTime.Now;
              long num8 = (long) Math.Round( now.Ticks / 1000.0);
              self.ExecuteMovement(index, 1);
              now = DateTime.Now;
              self.AddLog2("ExecuteMovement took " + Conversion.Str( ( now.Ticks / 1000.0 -  num8)));
              now = DateTime.Now;
              long num9 = (long) Math.Round( now.Ticks / 1000.0);
              if (self.TPlanObj[index].Stand == 2)
                self.ExecuteLandFrontAttacks(index, 2f * aiConservative);
              else if (self.TPlanObj[index].Stand == 1)
              {
                float d = self.TPlanObj[index].WeightEnemyForceUnMod / self.TPlanObj[index].WeightFriendlyForce;
                if ( self.TPlanObj[index].WeightEnemyForce /  self.TPlanObj[index].WeightFriendlyForce <  d)
                  d = self.TPlanObj[index].WeightEnemyForce / self.TPlanObj[index].WeightFriendlyForce;
                float num10 =  Math.Sqrt( d);
                if ( num10 < 0.25)
                  num10 = 0.25f;
                if ( num10 > 1.0)
                  num10 = 1f;
                self.ExecuteLandFrontAttacks(index, 1.5f * num10 * aiConservative);
              }
              else if (self.TPlanObj[index].Stand == 3)
                self.ExecuteLandFrontAttacks(index, 2.5f * aiConservative);
              now = DateTime.Now;
              self.AddLog2("ExecuteLandFrontAttacks took " + Conversion.Str( ( now.Ticks / 1000.0 -  num9)));
            }
            Application.DoEvents();
            now = DateTime.Now;
            long num11 = (long) Math.Round( now.Ticks / 1000.0);
            self.ExecuteEngineer(index);
            now = DateTime.Now;
            self.AddLog2("ExecuteEngineer took " + Conversion.Str( ( now.Ticks / 1000.0 -  num11)));
            Application.DoEvents();
            now = DateTime.Now;
            long num12 = (long) Math.Round( now.Ticks / 1000.0);
            self.ExecuteArtilleryAttack(index, 1f, 2);
            now = DateTime.Now;
            self.AddLog2("ExecuteArtilleryAttack took " + Conversion.Str( ( now.Ticks / 1000.0 -  num12)));
            if (self.TPlanObj[index].Stand == 2)
              self.ExecuteLandFrontAttacks(index, 2.5f * aiConservative);
            else if (self.TPlanObj[index].Stand == 1)
            {
              float d = self.TPlanObj[index].WeightEnemyForceUnMod / self.TPlanObj[index].WeightFriendlyForce;
              if ( self.TPlanObj[index].WeightEnemyForce /  self.TPlanObj[index].WeightFriendlyForce <  d)
                d = self.TPlanObj[index].WeightEnemyForce / self.TPlanObj[index].WeightFriendlyForce;
              float num13 =  Math.Sqrt( d);
              if ( num13 < 0.25)
                num13 = 0.25f;
              if ( num13 > 1.0)
                num13 = 1f;
              self.ExecuteLandFrontAttacks(index, 2f * num13 * aiConservative);
            }
            else if (self.TPlanObj[index].Stand == 3)
              self.ExecuteLandFrontAttacks(index, 3f * aiConservative);
            self.ExecuteMovement(index, 2);
            self.ExecJoinUnits(index);
            self.ExecuteEngineer(index);
            self.ExecuteEngineer(index);
            self.ExecuteEngineer(index);
            self.ExecuteEngineer(index);
          }
        }
        else if (self.TPlanObj[index].Type == 30)
        {
          Application.DoEvents();
          self.ExecuteMovement(index, 1);
        }
      }
      Application.DoEvents();
      now = DateTime.Now;
      long num14 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecJoinUnits();
      now = DateTime.Now;
      self.AddLog2("ExecuteEngineer took " + Conversion.Str( ( now.Ticks / 1000.0 -  num14)));
      Application.DoEvents();
      now = DateTime.Now;
      long num15 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecChangeHQ();
      now = DateTime.Now;
      self.AddLog2("ExecuteChangeHQ+Staff up took " + Conversion.Str( ( now.Ticks / 1000.0 -  num15)));
      Application.DoEvents();
      now = DateTime.Now;
      long num16 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecSplitLandUnits();
      now = DateTime.Now;
      self.AddLog2("ExecSplitLandunits took " + Conversion.Str( ( now.Ticks / 1000.0 -  num16)));
      Application.DoEvents();
      now = DateTime.Now;
      long num17 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecNewairunits(1);
      now = DateTime.Now;
      self.AddLog2("ExecNewairunits took " + Conversion.Str( ( now.Ticks / 1000.0 -  num17)));
      Application.DoEvents();
      now = DateTime.Now;
      long num18 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecNewLandUnits(1);
      now = DateTime.Now;
      self.AddLog2("Execnewlandunits took " + Conversion.Str( ( now.Ticks / 1000.0 -  num18)));
      Application.DoEvents();
      now = DateTime.Now;
      long num19 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecNewNavyunits(1);
      now = DateTime.Now;
      self.AddLog2("Execnewnavyunits took " + Conversion.Str( ( now.Ticks / 1000.0 -  num19)));
      Application.DoEvents();
      now = DateTime.Now;
      long num20 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecDisbandForTransfer();
      now = DateTime.Now;
      self.AddLog2("Execdisbandfortransfer took " + Conversion.Str( ( now.Ticks / 1000.0 -  num20)));
      Application.DoEvents();
      now = DateTime.Now;
      long num21 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecLandTransfers(1);
      now = DateTime.Now;
      self.AddLog2("Execlandtransfers took " + Conversion.Str( ( now.Ticks / 1000.0 -  num21)));
      if ( self.game.Data.RuleVar[253] > 0.0)
      {
        Application.DoEvents();
        now = DateTime.Now;
        long num22 = (long) Math.Round( now.Ticks / 1000.0);
        self.ExecNewLandUnits(2);
        now = DateTime.Now;
        self.AddLog2("Execnewlandunits took " + Conversion.Str( ( now.Ticks / 1000.0 -  num22)));
        Application.DoEvents();
        now = DateTime.Now;
        long num23 = (long) Math.Round( now.Ticks / 1000.0);
        self.ExecLandTransfers(2);
        now = DateTime.Now;
        self.AddLog2("Execlandtransfers took " + Conversion.Str( ( now.Ticks / 1000.0 -  num23)));
      }
      Application.DoEvents();
      now = DateTime.Now;
      long num24 = (long) Math.Round( now.Ticks / 1000.0);
      self.InitResearch();
      now = DateTime.Now;
      self.AddLog2("InitResearch() took " + Conversion.Str( ( now.Ticks / 1000.0 -  num24)));
      Application.DoEvents();
      now = DateTime.Now;
      long num25 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecAirTransfers(1);
      now = DateTime.Now;
      self.AddLog2("Execairtransfers took " + Conversion.Str( ( now.Ticks / 1000.0 -  num25)));
      Application.DoEvents();
      now = DateTime.Now;
      long num26 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecNavyTransfers(1);
      now = DateTime.Now;
      self.AddLog2("Execnavytransfers took " + Conversion.Str( ( now.Ticks / 1000.0 -  num26)));
      Application.DoEvents();
      now = DateTime.Now;
      long num27 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecUpgrades();
      now = DateTime.Now;
      self.AddLog2("ExecUpgrades took " + Conversion.Str( ( now.Ticks / 1000.0 -  num27)));
      Application.DoEvents();
      now = DateTime.Now;
      long num28 = (long) Math.Round( now.Ticks / 1000.0);
      self.BlowBridges();
      now = DateTime.Now;
      self.AddLog2("blowbridges took " + Conversion.Str( ( now.Ticks / 1000.0 -  num28)));
      Application.DoEvents();
      now = DateTime.Now;
      long num29 = (long) Math.Round( now.Ticks / 1000.0);
      self.EmptyHQ();
      now = DateTime.Now;
      self.AddLog2("emptyHQ took " + Conversion.Str( ( now.Ticks / 1000.0 -  num29)));
      Application.DoEvents();
      now = DateTime.Now;
      long num30 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecSetProduction();
      now = DateTime.Now;
      self.AddLog2("Execsetproduction took " + Conversion.Str( ( now.Ticks / 1000.0 -  num30)));
      if ( self.game.Data.RuleVar[875] == 1.0)
      {
        Application.DoEvents();
        now = DateTime.Now;
        long num31 = (long) Math.Round( now.Ticks / 1000.0);
        self.ExecResourceComplient();
        now = DateTime.Now;
        self.AddLog2("ExecResourceComplient took " + Conversion.Str( ( now.Ticks / 1000.0 -  num31)));
      }
      Application.DoEvents();
      now = DateTime.Now;
      long num32 = (long) Math.Round( now.Ticks / 1000.0);
      self.ExecSendStaffUp();
      now = DateTime.Now;
      self.AddLog2("ExecSendStaffUp took " + Conversion.Str( ( now.Ticks / 1000.0 -  num32)));
      for (let mut unitCounter: i32 =  self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        if (self.game.Data.UnitObj[unitCounter].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unitCounter].PreDef == -1 & !self.game.Data.UnitObj[unitCounter].IsHQ && self.game.Data.UnitObj[unitCounter].SFCount == -1)
          self.game.ProcessingObj.DoDisbandUnit(unitCounter);
      }
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
          self.game.Data.UnitObj[index].AIPlanRef = self.game.Data.UnitObj[index].AIPlanNr <= 0 ? (AIPlanClass) null : self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr];
      }
      self.game.AIRunning = false;
    }

    pub fn InitAIOnlyDim(&mut self)
    {
      object[,] objArray = new object[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexOA = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexContinent = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexSA = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexSAWithoutTemp = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexPlan = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexBackPlan = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexSeaSA = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.UnitMovePhase = new int[self.game.Data.UnitCounter + 1000 + 1];
      self.LogTxt = new string[1];
      self.SAObj = new SAClass[1];
      self.SACount = 0;
      self.LogCounter = -1;
      self.OACount = 0;
      self.TPlanCount = 0;
      self.ContinentCount = 0;
    }

    pub fn InitAI(&mut self)
    {
      object[,] objArray = new object[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexOA = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexContinent = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexSA = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexSAWithoutTemp = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexPlan = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexBackPlan = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.HexSeaSA = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.AIVP = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      self.UnitMovePhase = new int[self.game.Data.UnitCounter + 1000 + 1];
      self.LogTxt = new string[1];
      self.SAObj = new SAClass[1];
      self.SACount = 0;
      self.LogCounter = -1;
      self.LogCounter2 = -1;
      self.OACount = 0;
      self.TPlanCount = 0;
      self.game.EditObj.OrderMap = 0;
      self.game.EditObj.MapSelected = 0;
      self.game.EditObj.TargetMap = 0;
      self.ContinentCount = 0;
      self.AddLog2("INIT AI");
      self.InitRandomAI();
      self.TPlanObj = new AIPlanClass[1];
      Application.DoEvents();
      long num1 = (long) Math.Round( DateTime.Now.Ticks / 1000.0);
      self.InitAIVP();
      self.AddLog2("InitAIVP took " + Conversion.Str( ( DateTime.Now.Ticks / 1000.0 -  num1)));
      self.MakeCombatMatrix();
      if ( self.game.Data.RuleVar[903] > 0.0)
      {
        Application.DoEvents();
        DateTime now = DateTime.Now;
        long num2 = (long) Math.Round( now.Ticks / 1000.0);
        self.InitDeclareWar();
        now = DateTime.Now;
        self.AddLog2("InitDeclareWar " + Conversion.Str( ( now.Ticks / 1000.0 -  num2)));
      }
      self.InitDecisions();
      long num3 = (long) Math.Round( DateTime.Now.Ticks / 1000.0);
      self.InitFindOA();
      self.AddLog2("InitFindOA() took " + Conversion.Str( ( DateTime.Now.Ticks / 1000.0 -  num3)));
      long num4 = (long) Math.Round( DateTime.Now.Ticks / 1000.0);
      self.InitFindContinent();
      self.AddLog2("InitFindContinent() took " + Conversion.Str( ( DateTime.Now.Ticks / 1000.0 -  num4)));
      Application.DoEvents();
      long num5 = (long) Math.Round( DateTime.Now.Ticks / 1000.0);
      self.InitGetSA();
      self.AddLog2("InitGetSA() took " + Conversion.Str( ( DateTime.Now.Ticks / 1000.0 -  num5)));
      DateTime now1 = DateTime.Now;
      long num6 = (long) Math.Round( now1.Ticks / 1000.0);
      self.InitGetSeaSA();
      now1 = DateTime.Now;
      self.AddLog2("InitGetSeaSA() took " + Conversion.Str( ( now1.Ticks / 1000.0 -  num6)));
      Application.DoEvents();
      long num7 = (long) Math.Round( DateTime.Now.Ticks / 1000.0);
      self.InitSARelations();
      self.AddLog2("InitSARelations() took " + Conversion.Str( ( DateTime.Now.Ticks / 1000.0 -  num7)));
      DateTime now2 = DateTime.Now;
      long num8 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTPlans();
      now2 = DateTime.Now;
      self.AddLog2("InitTPlans() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num8)));
      now2 = DateTime.Now;
      long num9 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitPlanFrontline();
      now2 = DateTime.Now;
      self.AddLog2("InitPlanFrontline() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num9)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num10 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitUnits();
      now2 = DateTime.Now;
      self.AddLog2("InitUnits() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num10)));
      now2 = DateTime.Now;
      long num11 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitLandReserves();
      now2 = DateTime.Now;
      self.AddLog2("InitLandReserves() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num11)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num12 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitLandReserveMetaHQ();
      now2 = DateTime.Now;
      self.AddLog2("InitLandReserveMetaHQ() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num12)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num13 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitEmergencyUnitSwitch();
      now2 = DateTime.Now;
      self.AddLog2("InitEmergencyUnitSwitch() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num13)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num14 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTPlanForceBalance();
      now2 = DateTime.Now;
      self.AddLog2("InitTPlanForceBalance() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num14)));
      now2 = DateTime.Now;
      long num15 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTPlanForceBalanceNavy();
      now2 = DateTime.Now;
      self.AddLog2("InitTPlanForceBalanceNavy() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num15)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num16 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTPlanStrategicImportance();
      now2 = DateTime.Now;
      self.AddLog2("InitTPlanStrategicImportance() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num16)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num17 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTransferUnits();
      now2 = DateTime.Now;
      self.AddLog2("InitAIVP took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num17)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num18 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitPlanLevelAnalysis();
      now2 = DateTime.Now;
      self.AddLog2("InitPlanLevelAnalysis() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num18)));
      now2 = DateTime.Now;
      long num19 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTacticalHQ();
      now2 = DateTime.Now;
      self.AddLog2("InitTacticalHQ() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num19)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num20 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitTPlanAPCost();
      now2 = DateTime.Now;
      self.AddLog2("InitTPlanAPCost() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num20)));
      now2 = DateTime.Now;
      long num21 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitUnitGoals();
      now2 = DateTime.Now;
      self.AddLog2("InitUnitGoals() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num21)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num22 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitAirTransferUnits();
      now2 = DateTime.Now;
      self.AddLog2("InitAirTransferUnits() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num22)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num23 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitNavyTransferUnits();
      now2 = DateTime.Now;
      self.AddLog2("InitNavyTransferUnits() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num23)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num24 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitStrategicHQAnalysis();
      now2 = DateTime.Now;
      self.AddLog2("InitStrategicHQAnalysis() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num24)));
      Application.DoEvents();
      now2 = DateTime.Now;
      long num25 = (long) Math.Round( now2.Ticks / 1000.0);
      self.InitRiverLine();
      now2 = DateTime.Now;
      self.AddLog2("InitRiverLine() took " + Conversion.Str( ( now2.Ticks / 1000.0 -  num25)));
      self.InitSetStandingOrders();
      Application.DoEvents();
      self.InitShowStats();
      self.game.EventRelatedObj.DoCheckEvents(6);
    }

    pub fn ExecuteEngineer(&mut self, plannr: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList = SimpleList::new();
      if ( self.game.Data.RuleVar[211] < 1.0)
        return;
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr == plannr & self.game.Data.UnitObj[unr].AIUnitGoal == 4)
        {
          engcount: i32;
          engcount += 1;
          Coordinate engineerCoord = self.GetEngineerCoord(engcount, plannr);
          if (engineerCoord.onmap && self.game.Data.UnitObj[unr].X == engineerCoord.x & self.game.Data.UnitObj[unr].Y == engineerCoord.y)
          {
            Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(engineerCoord.x, engineerCoord.y, engineerCoord.map, engineerCoord.data1);
            if (coordinate.onmap)
            {
              self.game.HandyFunctionsObj.InfraHexHighlight(engineerCoord.x, engineerCoord.y, engineerCoord.map, unr);
              if (self.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] == 0)
              {
                self.game.ProcessingObj.BuildInfra(unr, engineerCoord.x, engineerCoord.y, engineerCoord.map, engineerCoord.data1 - 1);
                self.AddLog(self.game.Data.UnitObj[unr].Name + " did infra construction from " + Conversion.Str( engineerCoord.x) + "," + Conversion.Str( engineerCoord.y) + " to " + Conversion.Str( coordinate.x) + "," + Conversion.Str( coordinate.y));
              }
            }
          }
        }
      }
    }

    pub fn ExecUpgrades(&mut self)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount; index += 1)
      {
        let mut hq: i32 =  self.TPlanObj[index].HQ;
        if (hq > -1 && self.game.Data.UnitObj[hq].IsHQ)
        {
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[hq].Regime, (int) Math.Round( self.game.Data.RuleVar[99]), 99, (int) Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y, 0, allowshoredrop: true, SeaBlock: true);
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
          {
            if (self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn &&  self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] <=  self.game.Data.RuleVar[51])
            {
              for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount; sfCount >= 0; sfCount += -1)
              {
                let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[sfCount];
                if (self.game.HandyFunctionsObj.CanUpgrade(sf, unr))
                {
                  let mut qty: i32 =  self.game.HandyFunctionsObj.CanUpgradeMax(sf, unr, hq);
                  if (qty > self.game.Data.SFObj[sf].Qty)
                    qty = self.game.Data.SFObj[sf].Qty;
                  if (qty > 0)
                    self.game.ProcessingObj.DoUpgrade(unr, sf, qty, hq);
                }
              }
            }
          }
        }
      }
    }

    pub fn ExecUnloadUnit(&mut self, plnr: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      OrderResult orderResult1;
      for (let mut unitCounter: i32 =  self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        if (self.game.Data.UnitObj[unitCounter].AIPlanNr == plnr)
        {
          let mut num1: i32 =  1;
          while (num1 == 1)
          {
            num1 = 0;
            if (self.game.Data.UnitObj[unitCounter].PassengerCounter > -1 && self.game.Data.UnitObj[unitCounter].X == self.game.Data.UnitObj[unitCounter].AINavtargetX && self.game.Data.UnitObj[unitCounter].Y == self.game.Data.UnitObj[unitCounter].AINavtargetY)
            {
              SimpleList simpleList = SimpleList::new();
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(self.game.Data.UnitObj[unitCounter].X, self.game.Data.UnitObj[unitCounter].Y, self.game.Data.UnitObj[unitCounter].Map, tfacing);
                if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].CanAmph)
                {
                  let mut num2: i32 =  0;
                  if (self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == -1)
                    num2 = 1;
                  if (self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime > -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime] == 0)
                    num2 = 1;
                  if (num2 == 1 && self.HexSA[coordinate.x, coordinate.y] == self.TPlanObj[plnr].SeaTarget)
                  {
                    let mut num3: i32 =  0;
                    if (self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter > 14 && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime, self.game.Data.Turn))
                      num3 = 1;
                    if (num3 == 0)
                    {
                      tid: i32;
                      simpleList.Add(tid, self.GetHexForceLandStrength(coordinate.x, coordinate.y), coordinate.x, coordinate.y);
                    }
                  }
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
              simpleList.Sort();
              if (simpleList.Counter > -1)
              {
                self.game.SelectX = simpleList.Data1[0];
                self.game.SelectY = simpleList.Data2[0];
                self.game.EditObj.OrderTarget = self.game.Data.UnitObj[unitCounter].PassengerList[0];
                self.game.EditObj.OrderUnit = unitCounter;
                OrderResult orderResult2 = self.game.ProcessingObj.unLoadUnit(self.game.EditObj.OrderTarget, self.game.EditObj.OrderUnit, self.game.SelectX, self.game.SelectY, 0);
                num1 = 1;
                if (orderResult2.OK)
                {
                  if (self.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(self.game.SelectX, self.game.SelectY, 0, self.game.Data.Turn, true))
                  {
                    self.game.EditObj.TargetX = self.game.SelectX;
                    self.game.EditObj.TargetY = self.game.SelectY;
                    self.game.TempCombat = new CombatClass(self.game);
                    Coordinate Target = Coordinate::new();
                    Target.x = self.game.EditObj.TargetX;
                    Target.y = self.game.EditObj.TargetY;
                    self.game.EditObj.TempUnitList = UnitList::new();
                    self.game.EditObj.TempUnitList.add(self.game.EditObj.OrderTarget);
                    orderResult1 = self.game.TempCombat.Init(Target, 1, self.game.EditObj.TempUnitList, 21);
                    self.game.TempCombat.DoBattle();
                    self.game.TempCombat.EndBattle();
                    self.game.TempCombat = (CombatClass) null;
                    self.game.EditObj.TargetX = -1;
                    self.game.EditObj.TargetY = -1;
                  }
                  self.AddLog("Unloaded unit");
                }
              }
            }
          }
        }
      }
      for (let mut unitCounter: i32 =  self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        if (self.game.Data.UnitObj[unitCounter].AIPlanNr == plnr)
        {
          let mut num: i32 =  1;
          while (num == 1)
          {
            num = 0;
            if (self.game.Data.UnitObj[unitCounter].PassengerCounter > -1 && self.TPlanObj[plnr].Stand != 7)
            {
              self.game.SelectX = self.game.Data.UnitObj[unitCounter].X;
              self.game.SelectY = self.game.Data.UnitObj[unitCounter].Y;
              if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType].IsSea)
              {
                self.game.EditObj.OrderTarget = self.game.Data.UnitObj[unitCounter].PassengerList[0];
                self.game.EditObj.OrderUnit = unitCounter;
                orderResult1 = self.game.ProcessingObj.unLoadUnit(self.game.EditObj.OrderTarget, self.game.EditObj.OrderUnit, self.game.SelectX, self.game.SelectY, 0);
                num = 1;
                self.AddLog("Unloaded unit due to lack of AMPH plan");
              }
            }
          }
        }
      }
      self.CratesCheck();
    }

    pub fn ExecLoadUnitOnBoard(&mut self, plnr: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.TPlanObj[plnr].SeaTarget <= 0)
        return;
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        if (self.game.Data.UnitObj[index].AIPlanNr == plnr && self.game.Data.UnitObj[index].AIUnitGoal == 8 && self.game.HandyFunctionsObj.GetUnitCarryCap(index, 1) > 0)
        {
          let mut x: i32 =  self.game.Data.UnitObj[index].X;
          let mut y: i32 =  self.game.Data.UnitObj[index].Y;
          if (x == self.TPlanObj[plnr].FromArea.X & y == self.TPlanObj[plnr].FromArea.Y)
          {
            for (let mut unitCounter2: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter; unitCounter2 >= 0; unitCounter2 += -1)
            {
              let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitList[unitCounter2];
              if (index != unit && self.game.Data.UnitObj[unit].AIPlanNr == plnr && !self.game.Data.UnitObj[unit].AIReserve && !self.game.HandyFunctionsObj.HasUnitAirSF(unit) & !self.game.HandyFunctionsObj.HasUnitNavySF(unit) && self.game.HandyFunctionsObj.HasUnitlandSF(unit) & self.game.HandyFunctionsObj.GetUnitWeight(unit, true) > 0 && self.game.Data.UnitObj[unit].AIUnitGoal == 1 | self.game.Data.UnitObj[unit].AIUnitGoal == 2 | self.game.Data.UnitObj[unit].AIUnitGoal == 3 | self.game.Data.UnitObj[unit].AIUnitGoal == 4 && self.game.HandyFunctionsObj.GetUnitWeight(unit, true) <= self.game.HandyFunctionsObj.GetUnitCarryCap(index, 1, true))
                self.game.ProcessingObj.LoadUnit(unit, index);
            }
          }
        }
      }
    }

    pub fn ExecJoinUnits(&mut self, plannr: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList = SimpleList::new();
      self.AddLog("JOIN UNITS:");
      if ( self.game.Data.RuleVar[211] > 0.0)
      {
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index1, index2].Regime, self.game.Data.Turn))
            {
              let mut unrT: i32 =  -1;
              let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
              for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
              {
                let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3];
                if (plannr == -1 | plannr == self.game.Data.UnitObj[unit].AIPlanNr && Operators.ConditionalCompareObjectGreater(self.GetEPPerTurn(unit),  0, false) && self.game.Data.UnitObj[unit].AIUnitGoal == 4)
                {
                  if (unrT == -1)
                    unrT = unit;
                  else if (self.game.Data.UnitObj[unrT].AIPlanNr == self.game.Data.UnitObj[unit].AIPlanNr)
                  {
                    for (let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount; sfCount >= 0; sfCount += -1)
                    {
                      let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[sfCount];
                      self.game.ProcessingObj.DoTransfer(unit, unrT, 0, sf, self.game.Data.SFObj[sf].Qty, true, false);
                    }
                    if (self.game.Data.UnitObj[unit].IsHQ)
                    {
                      RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                      RegimeClass[] regimeClassArray = regimeObj;
                      let mut turn: i32 =  self.game.Data.Turn;
                      let mut index4: i32 =  turn;
                      regimeClassArray[index4].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[47]));
                    }
                    else
                    {
                      RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                      RegimeClass[] regimeClassArray = regimeObj;
                      let mut turn: i32 =  self.game.Data.Turn;
                      let mut index5: i32 =  turn;
                      regimeClassArray[index5].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                    }
                    self.game.Data.RemoveUnit(unit, ref self.game);
                    break;
                  }
                }
              }
            }
          }
        }
      }
      if (plannr <= -1)
        return;
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 =  self.TPlanObj[plannr].EnemyUnitCount * 2;
        if (num2 == 0)
          num2 = 1;
        let mut num3: i32 =  num2 + 1;
        if (num3 > self.TPlanObj[plannr].FrontSize)
          num3 = self.TPlanObj[plannr].FrontSize;
        if (self.TPlanObj[plannr].FriendlyUnitCount > num3 &&  self.game.Data.RuleVar[249] == 0.0)
        {
          let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
            {
              if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index6, index7].Regime, self.game.Data.Turn))
              {
                let mut index8: i32 =  -1;
                let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index6, index7].UnitCounter;
                for (let mut index9: i32 =  0; index9 <= unitCounter; index9 += 1)
                {
                  let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index6, index7].UnitList[index9];
                  if (plannr == -1 | plannr == self.game.Data.UnitObj[unit].AIPlanNr)
                  {
                    if (index8 == -1)
                      index8 = unit;
                    else if (self.game.Data.UnitObj[index8].AIPlanNr == self.game.Data.UnitObj[unit].AIPlanNr && self.game.Data.UnitObj[unit].AIUnitGoal == self.game.Data.UnitObj[index8].AIUnitGoal && !self.game.Data.UnitObj[unit].IsHQ & !self.game.Data.UnitObj[index8].IsHQ &&  (self.game.HandyFunctionsObj.GetUnitStackPts(unit) + self.game.HandyFunctionsObj.GetUnitStackPts(index8)) <  self.game.Data.RuleVar[184])
                    {
                      for (let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount; sfCount >= 0; sfCount += -1)
                      {
                        let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[sfCount];
                        self.game.ProcessingObj.DoTransfer(unit, index8, 0, sf, self.game.Data.SFObj[sf].Qty, true, false);
                      }
                      if (self.game.Data.UnitObj[unit].IsHQ)
                      {
                        RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut turn: i32 =  self.game.Data.Turn;
                        let mut index10: i32 =  turn;
                        regimeClassArray[index10].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[47]));
                      }
                      else
                      {
                        RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut turn: i32 =  self.game.Data.Turn;
                        let mut index11: i32 =  turn;
                        regimeClassArray[index11].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                      }
                      self.game.Data.RemoveUnit(unit, ref self.game);
                      self.AddLog("Joined unit");
                      num1 = 1;
                      break;
                    }
                  }
                }
              }
            }
          }
        }
        let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index12: i32 =  0; index12 <= mapWidth1; index12 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index13: i32 =  0; index13 <= mapHeight; index13 += 1)
          {
            if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index12, index13].Regime, self.game.Data.Turn))
            {
              let mut index14: i32 =  -1;
              let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index12, index13].UnitCounter;
              for (let mut index15: i32 =  0; index15 <= unitCounter; index15 += 1)
              {
                let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index12, index13].UnitList[index15];
                if (plannr == -1 | plannr == self.game.Data.UnitObj[unit].AIPlanNr)
                {
                  if (index14 == -1)
                    index14 = unit;
                  else if (self.game.Data.UnitObj[index14].AIPlanNr == self.game.Data.UnitObj[unit].AIPlanNr && self.game.Data.UnitObj[unit].AIUnitGoal == self.game.Data.UnitObj[index14].AIUnitGoal &&  self.game.HandyFunctionsObj.GetUnitStackPts(unit) <  self.game.Data.RuleVar[182] &  self.game.HandyFunctionsObj.GetUnitStackPts(index14) <  self.game.Data.RuleVar[182] * 2.0 &&  (self.game.HandyFunctionsObj.GetUnitStackPts(unit) + self.game.HandyFunctionsObj.GetUnitStackPts(index14)) <  self.game.Data.RuleVar[184] && !self.game.Data.UnitObj[unit].IsHQ & !self.game.Data.UnitObj[index14].IsHQ)
                  {
                    for (let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount; sfCount >= 0; sfCount += -1)
                    {
                      let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[sfCount];
                      self.game.ProcessingObj.DoTransfer(unit, index14, 0, sf, self.game.Data.SFObj[sf].Qty, true, false);
                    }
                    if (self.game.Data.UnitObj[unit].IsHQ)
                    {
                      RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                      RegimeClass[] regimeClassArray = regimeObj;
                      let mut turn: i32 =  self.game.Data.Turn;
                      let mut index16: i32 =  turn;
                      regimeClassArray[index16].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[47]));
                    }
                    else
                    {
                      RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                      RegimeClass[] regimeClassArray = regimeObj;
                      let mut turn: i32 =  self.game.Data.Turn;
                      let mut index17: i32 =  turn;
                      regimeClassArray[index17].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                    }
                    self.game.Data.RemoveUnit(unit, ref self.game);
                    self.AddLog("Joined 2 to small unit");
                    num1 = 1;
                    break;
                  }
                }
              }
            }
          }
        }
      }
    }

    pub float GetFriendlyAirRatio(&mut self)
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      num3: i32;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1)
        {
          if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn)
          {
            num1 += self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr);
            num2 += self.game.HandyFunctionsObj.GetPowerPtsAbsoluteForAirOnly(unr);
          }
          else if (self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.UnitObj[unr].Regime, self.game.Data.Turn))
            num3 += self.game.HandyFunctionsObj.GetPowerPtsAbsoluteForAirOnly(unr);
        }
      }
      if (num3 == 0)
        num3 = 1;
      let mut regimeCounter: i32 =  self.game.Data.RegimeCounter;
      num4: i32;
      for (let mut regnr: i32 =  0; regnr <= regimeCounter; regnr += 1)
      {
        if (self.game.HandyFunctionsObj.IsHostileNotSelf(regnr, self.game.Data.Turn) && self.game.Data.Turn != regnr & !self.game.Data.RegimeObj[regnr].Sleep)
          num4 += 1;
      }
      if (num4 < 1)
        num4 = 1;
      let mut num5: i32 =  (int) Math.Round( num3 /  num4);
      float friendlyAirRatio =  num2 /  num5;
      if ( friendlyAirRatio < 1.0)
      {
        if ( num1 /  num5 > 5.0)
          friendlyAirRatio += 0.6f;
        if ( num1 /  num5 > 10.0)
          friendlyAirRatio += 0.12f;
        if ( num1 /  num5 > 20.0)
          friendlyAirRatio += 0.12f;
        if ( num1 /  num5 > 30.0)
          friendlyAirRatio += 0.12f;
        if ( num1 /  num5 > 40.0)
          friendlyAirRatio += 0.12f;
        if ( num1 /  num5 > 40.0)
          friendlyAirRatio += 0.12f;
        if ( num1 /  num5 > 50.0)
          friendlyAirRatio += 0.22f;
        if ( num1 /  num5 > 99.0)
          friendlyAirRatio += 0.36f;
      }
      return friendlyAirRatio;
    }

    pub fn ExecSetProduction(&mut self)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList = SimpleList::new();
      int[] numArray2 = new int[self.game.Data.UnitCounter + 1];
      self.AddLog("");
      self.AddLog("PRODUCTION:");
      let mut num1: i32 =  0;
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        if (self.TPlanObj[index].Type == 30 & self.TPlanObj[index].HQ > -1 && self.TPlanObj[index].MetaChainNr > num1)
          num1 = self.TPlanObj[index].MetaChainNr;
      }
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount2; index += 1)
      {
        if (self.TPlanObj[index].Type == 20 && self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime == self.game.Data.Turn && self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Location > -1 && self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Location].HQ > -1)
        {
          self.SAObj[self.GetSANr(self.TPlanObj[index].TooArea)].LandReservePlan = self.SAObj[self.GetSANr(self.TPlanObj[index].FromArea)].LandReservePlan;
          if (self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].FromArea.X, self.TPlanObj[index].FromArea.Y].Location > -1 && self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Location > -1)
            self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Location].HQ = self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].FromArea.X, self.TPlanObj[index].FromArea.Y].Location].HQ;
        }
      }
      for (; num1 > -1; --num1)
      {
        let mut tplanCount3: i32 =  self.TPlanCount;
        for (let mut index1: i32 =  1; index1 <= tplanCount3; index1 += 1)
        {
          hq1: i32;
          if (self.TPlanObj[index1].Type == 30 & self.TPlanObj[index1].HQ > -1 & self.TPlanObj[index1].MetaChainNr == num1)
          {
            let mut num2: i32 =  0;
            let mut saCount1: i32 =  self.SACount;
            for (let mut index2: i32 =  1; index2 <= saCount1; index2 += 1)
            {
              if (self.SAObj[index2].LandReservePlan == index1)
              {
                let mut location: i32 =  self.game.Data.MapObj[0].HexObj[self.SAObj[index2].X, self.SAObj[index2].Y].Location;
                if (location > -1 && self.game.Data.LocTypeObj[self.game.Data.LocObj[location].Type].MaxProd > 0 && self.game.Data.MapObj[0].HexObj[self.SAObj[index2].X, self.SAObj[index2].Y].Regime == self.game.Data.Turn && self.ProdGetSupplyItem(location) > -1)
                  num2 = (int) Math.Round( num2 +  self.game.Data.LocTypeObj[self.game.Data.LocObj[location].Type].MaxProd /  self.game.Data.ItemTypeObj[self.ProdGetSupplyItem(location)].ProdWeight);
              }
            }
            let mut num3: i32 =  0;
            do
            {
              SimpleList Expression = (SimpleList) null;
              let mut saCount2: i32 =  self.SACount;
              for (let mut index3: i32 =  1; index3 <= saCount2; index3 += 1)
              {
                if (self.SAObj[index3].LandReservePlan == index1)
                {
                  let mut locnr: i32 =  self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Location;
                  if (locnr > -1 & self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Regime == self.game.Data.Turn)
                  {
                    if (self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].AutoProd == -1)
                    {
                      if (self.game.HandyFunctionsObj.GetLocTypeItemTypesAmmount(self.game.Data.LocObj[locnr].Type) == num3 | num3 == 99 & self.game.HandyFunctionsObj.GetLocTypeItemTypesAmmount(self.game.Data.LocObj[locnr].Type) >= 99)
                      {
                        if (Information.IsNothing( Expression))
                          Expression = self.ProdWishes(index1, locnr);
                        if (self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd > 0 & self.game.HandyFunctionsObj.CanLocProduce(locnr, self.game.Data.Turn))
                        {
                          Application.DoEvents();
                          self.AddLog("Plan #" + Conversion.Str( index1) + ", Location: " + self.game.Data.LocObj[locnr].Name);
                          simpleList = SimpleList::new();
                          hq1 = self.TPlanObj[index1].HQ;
                          let mut landCap: i32 =  self.game.Data.UnitObj[hq1].LandCap;
                          let mut capPts: i32 =  self.game.HandyFunctionsObj.GetCapPts(hq1, 0);
                          Number1: i32;
                          if (capPts == 0)
                            Number1 = 1;
                          else if ( landCap /  capPts < 0.2)
                            Number1 = 1;
                          if (self.ProdGetRole(locnr, 2) == -1)
                            Number1 = 0;
                          if ( self.game.Data.RuleVar[253] == 0.0)
                            Number1 = 0;
                          if (Number1 == 0)
                          {
                            let mut navyCap: i32 =  self.game.Data.UnitObj[hq1].NavyCap;
                            capPts = self.game.HandyFunctionsObj.GetCapPts(hq1, 1);
                            if (self.SAObj[index3].SeaNeighbourCount > 0)
                            {
                              if (capPts == 0)
                                Number1 = 2;
                              else if ( navyCap /  capPts < 0.2)
                                Number1 = 2;
                              if (self.ProdGetRole(locnr, 3) == -1)
                                Number1 = 0;
                            }
                          }
                          if ( self.game.Data.RuleVar[253] == 0.0)
                            self.TPlanObj[index1].LandTransferMobility = 0;
                          if ( self.game.Data.RuleVar[253] == 0.0 & capPts > 0)
                            Number1 = 0;
                          let mut num4: i32 =  0;
                          let mut num5: i32 =  self.game.HandyFunctionsObj.GetRealHQSupplyPts(self.TPlanObj[index1].HQ);
                          let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
                          for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
                          {
                            if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr > 0)
                            {
                              let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
                              if (self.IsHQinChain(unr, hq1))
                                num4 += self.game.HandyFunctionsObj.UnitSupplyNeed(unr, true);
                            }
                          }
                          if (num5 < 0)
                            num5 = 0;
                          let mut Number2: i32 =  (int) Math.Round( num4 * 1.5 -  num5);
                          if (Number2 < 0)
                            Number2 = 0;
                          self.AddLog("Supply before town divider: " + Conversion.Str( Number2));
                          let mut Number3: i32 =  self.ProdGetSupplyItem(locnr) != -1 ? (int) Math.Round( Number2 * ( self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd /  self.game.Data.ItemTypeObj[self.ProdGetSupplyItem(locnr)].ProdWeight /  num2)) : 0;
                          let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
                          Number4: i32;
                          for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
                          {
                            if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & self.game.Data.UnitObj[unr].IsHQ && self.game.Data.UnitObj[unr].AIPlanNr > 0)
                            {
                              let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
                              if (self.TPlanObj[aiPlanNr].Type == 20 && self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan == index1 && self.game.HandyFunctionsObj.GetStaffPercent(unr) < 100)
                                Number4 = 1;
                            }
                          }
                          if (self.game.HandyFunctionsObj.GetStaffPercent(self.TPlanObj[index1].HQ) > 200)
                            Number4 = 0;
                          if (self.ProdGetRole(locnr, 1) == -1)
                            Number4 = 0;
                          if ( self.game.Data.RuleVar[(int) byte.MaxValue] == 1.0)
                            Number4 = 0;
                          let mut num6: i32 =  0;
                          let mut num7: i32 =  0;
                          if ( self.game.Data.RuleVar[211] > 0.0)
                          {
                            let mut unitCounter3: i32 =  self.game.Data.UnitCounter;
                            for (let mut unr: i32 =  0; unr <= unitCounter3; unr += 1)
                            {
                              if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & self.game.Data.UnitObj[unr].AIUnitGoal == 4 && self.game.Data.UnitObj[unr].AIPlanNr > 0)
                              {
                                let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
                                if (self.TPlanObj[aiPlanNr].Type == 20 | self.TPlanObj[aiPlanNr].Type == 40 && self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan == index1)
                                {
                                  let mut integer: i32 =  Conversions.ToInteger(Operators.AddObject( Conversions.ToInteger(self.GetEPPerTurn(unr)), self.GetEPPerTurn(self.TPlanObj[index1].HQ)));
                                  num6 += integer;
                                  num7 += 1;
                                }
                              }
                            }
                          }
                          let mut num8: i32 =  num7 <= 0 ? 0 : ( VBMath.Rnd() *  self.game.Data.RuleVar[215] >=  num6 /  num7 ? 1 : 0);
                          Number5: i32;
                          if ( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts <  self.game.Data.RuleVar[181])
                            Number5 = 1;
                          if ( VBMath.Rnd() <  self.game.Data.RuleVar[226])
                            Number5 = 1;
                          if (self.ProdGetPPItem(locnr) == -1)
                            Number5 = 0;
                          self.AddLog("Landcap: " + Conversion.Str( Number1) + ", Supply: " + Conversion.Str( Number3) + ", Staff=" + Conversion.Str( Number4) + ", PP=" + Conversion.Str( Number5));
                          self.AddLog("LandMobAlert=" + Conversion.Str( self.TPlanObj[index1].LandTransferMobility) + ", SeaMobAlert=" + Conversion.Str( self.TPlanObj[index1].SeaTransferMobility));
                          if (Number5 > 0 & Number4 > 0)
                          {
                            if ( VBMath.Rnd() > 0.5)
                              Number4 = 0;
                            else
                              Number5 = 0;
                          }
                          if (Number1 > 0 & (Number5 > 0 | Number4 > 0))
                          {
                            if ( VBMath.Rnd() > 0.5)
                            {
                              Number1 = 0;
                            }
                            else
                            {
                              Number5 = 0;
                              Number4 = 0;
                            }
                          }
                          self.game.Data.LocObj[locnr].HQ = self.TPlanObj[index1].HQ;
                          let mut index4: i32 =  0;
                          let mut num9: i32 =  0;
                          if (self.game.Data.LocObj[locnr].X == 22 & self.game.Data.LocObj[locnr].Y == 31)
                            locnr = locnr;
                          if (Number3 > -1)
                          {
                            let mut index5: i32 =  self.ProdGetSupplyItem(locnr);
                            if (Operators.CompareString(self.game.Data.LocObj[locnr].Name, "Aqaba", false) == 0)
                              index5 = index5;
                            if (index5 > -1)
                            {
                              let mut num10: i32 =  (int) Math.Round(100.0 * ( (self.game.Data.ItemTypeObj[index5].ProdWeight * Number3) /  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd));
                              if (num10 < 10)
                                num10 = 10;
                              if (num10 > 100)
                              {
                                if (self.game.Data.UnitObj[self.TPlanObj[index1].HQ].HQ > -1)
                                {
                                  int[] numArray3 = numArray2;
                                  int[] numArray4 = numArray3;
                                  let mut hq2: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].HQ;
                                  let mut index6: i32 =  hq2;
                                  let mut num11: i32 =  (int) Math.Round( numArray3[hq2] + ( Number3 -  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd /  self.game.Data.ItemTypeObj[index5].ProdWeight));
                                  numArray4[index6] = num11;
                                }
                                num10 = 100;
                              }
                              self.game.Data.LocObj[locnr].Production[0] = index5;
                              self.game.Data.LocObj[locnr].ProdPercent[0] = num10;
                              index4 = 1;
                              num9 = num10;
                            }
                          }
                          if (num9 < 100 & index4 < 4)
                          {
                            let mut num12: i32 =  -1;
                            if (Number5 > 0 & num12 == -1)
                            {
                              num12 = self.ProdGetPPItem(locnr);
                              Number5 = 0;
                            }
                            if (Number1 == 1 & num12 == -1)
                            {
                              num12 = self.ProdGetRole(locnr, 2);
                              Number1 = 0;
                            }
                            if (Number1 == 2 & num12 == -1)
                            {
                              num12 = self.ProdGetRole(locnr, 3);
                              Number1 = 0;
                            }
                            if (Number4 > 0 & num12 == -1)
                            {
                              num12 = self.ProdGetRole(locnr, 1);
                              Number4 = 0;
                            }
                            let mut num13: i32 =  100 - num9;
                            if (Number5 == 1)
                            {
                              if ( num13 > 30.0 + 30.0 *  self.game.Data.RuleVar[226])
                                num13 = (int) Math.Round(30.0 + 30.0 *  self.game.Data.RuleVar[226]);
                            }
                            else if (num13 > 30)
                              num13 = 30;
                            if (num12 > -1)
                            {
                              self.game.Data.LocObj[locnr].ProdPercent[index4] = num13;
                              self.game.Data.LocObj[locnr].Production[index4] = num12;
                              index4 += 1;
                              num9 += num13;
                            }
                          }
                          let mut num14: i32 =  0;
                          if (num9 < 100 & index4 < 4 & Expression.Counter > -1)
                          {
                            let mut num15: i32 =  0;
                            num14 = 0;
                            while (num15 == 0 & num14 < 20)
                            {
                              num14 += 1;
                              let mut index7: i32 =  (int) Math.Round( Conversion.Int(VBMath.Rnd() *  (Expression.Counter + 1)));
                              let mut index8: i32 =  self.ProdGetRole(locnr, Expression.Weight[index7]);
                              if (index8 == -1)
                              {
                                index8 = self.ProdGetRole(locnr, Expression.Data1[index7]);
                                if (Expression.Data3[index7] > -1 && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, Expression.Data3[index7]).result)
                                  index8 = Expression.Data3[index7];
                              }
                              else if (Expression.Data2[index7] > -1 && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, Expression.Data2[index7]).result)
                                index8 = Expression.Data2[index7];
                              if ( self.game.Data.RuleVar[253] > 0.0)
                              {
                                if (self.TPlanObj[index1].LandTransferMobility == 1 & self.TPlanObj[index1].SeaTransferMobility == 0)
                                {
                                  if (self.ProdGetRole(locnr, 2) > -1)
                                    index8 = self.ProdGetRole(locnr, 2);
                                }
                                else if (self.TPlanObj[index1].LandTransferMobility == 0 & self.TPlanObj[index1].SeaTransferMobility == 1)
                                {
                                  if (self.ProdGetRole(locnr, 3) > -1)
                                    index8 = self.ProdGetRole(locnr, 3);
                                }
                                else if (self.TPlanObj[index1].LandTransferMobility == 1 & self.TPlanObj[index1].SeaTransferMobility == 1)
                                {
                                  if ( VBMath.Rnd() > 0.5)
                                  {
                                    if (self.ProdGetRole(locnr, 2) > -1)
                                      index8 = self.ProdGetRole(locnr, 2);
                                  }
                                  else if (self.ProdGetRole(locnr, 3) > -1)
                                    index8 = self.ProdGetRole(locnr, 3);
                                }
                              }
                              if ( self.game.Data.RuleVar[211] == 1.0 & num8 == 1 &  VBMath.Rnd() > 0.5)
                              {
                                if (self.ProdGetRole(locnr, 5) > -1)
                                  index8 = self.ProdGetRole(locnr, 5);
                              }
                              else if ( VBMath.Rnd() <  self.game.Data.RuleVar[226])
                                index8 = self.ProdGetPPItem(locnr);
                              let mut num16: i32 =  (int) Math.Round(Conversion.Int( (100 - num9) / 2.0));
                              if ( self.game.Data.LocObj[locnr].ProdPointRemainder[index4] >=  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd * ( num16 / 150.0))
                              {
                                num9 += self.game.Data.LocObj[locnr].ProdPercent[index4];
                                let mut index9: i32 =  self.game.Data.LocObj[locnr].Production[index4];
                                if (index9 > -1 & num16 > 0 &&  self.game.Data.ItemTypeObj[index9].ProdWeight >  self.game.HandyFunctionsObj.GetProdPtsForLoc2(locnr) * ( num16 / 100.0))
                                  num16 = 100 - num9;
                                self.game.Data.LocObj[locnr].ProdPercent[index4] = num16;
                                index4 += 1;
                                break;
                              }
                              if (index8 > -1 & num16 > 0 &&  self.game.Data.ItemTypeObj[index8].ProdWeight >  self.game.HandyFunctionsObj.GetProdPtsForLoc2(locnr) * ( num16 / 100.0))
                                num16 = 100 - num9;
                              if (index8 > -1)
                              {
                                self.game.Data.LocObj[locnr].Production[index4] = index8;
                                self.game.Data.LocObj[locnr].ProdPercent[index4] = num16;
                                index4 += 1;
                                num9 += num16;
                                num15 = 1;
                              }
                            }
                          }
                          if (num9 < 100 & index4 < 4)
                          {
                            let mut num17: i32 =  0;
                            num14 = 0;
                            while (num17 == 0 & num14 < 20)
                            {
                              num14 += 1;
                              let mut num18: i32 =  -1;
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 6);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 7);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 8);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 10);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 12);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 1);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 5);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 9);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 12);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 19);
                              if (num18 == -1)
                                num18 = self.ProdGetRole(locnr, 18);
                              if (num18 == -1)
                                num18 = self.ProdGetPPItem(locnr);
                              if (num18 == -1 &&  self.game.Data.RuleVar[253] > 0.0)
                              {
                                if (self.TPlanObj[index1].LandTransferMobility == 1 & self.TPlanObj[index1].SeaTransferMobility == 0)
                                {
                                  if (self.ProdGetRole(locnr, 2) > -1)
                                    num18 = self.ProdGetRole(locnr, 2);
                                }
                                else if (self.TPlanObj[index1].LandTransferMobility == 0 & self.TPlanObj[index1].SeaTransferMobility == 1)
                                {
                                  if (self.ProdGetRole(locnr, 3) > -1)
                                    num18 = self.ProdGetRole(locnr, 3);
                                }
                                else if (self.TPlanObj[index1].LandTransferMobility == 1 & self.TPlanObj[index1].SeaTransferMobility == 1)
                                {
                                  if ( VBMath.Rnd() > 0.5)
                                  {
                                    if (self.ProdGetRole(locnr, 2) > -1)
                                      num18 = self.ProdGetRole(locnr, 2);
                                  }
                                  else if (self.ProdGetRole(locnr, 3) > -1)
                                    num18 = self.ProdGetRole(locnr, 3);
                                }
                              }
                              let mut num19: i32 =  100 - num9;
                              if ( self.game.Data.LocObj[locnr].ProdPointRemainder[index4] >=  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd * ( num19 / 150.0))
                              {
                                let mut num20: i32 =  num9 + self.game.Data.LocObj[locnr].ProdPercent[index4];
                                self.game.Data.LocObj[locnr].ProdPercent[index4] = num19;
                                index4 += 1;
                                num9 = num20 + num19;
                                break;
                              }
                              if (num18 > -1)
                              {
                                self.game.Data.LocObj[locnr].Production[index4] = num18;
                                self.game.Data.LocObj[locnr].ProdPercent[index4] = num19;
                                index4 += 1;
                                num9 += num19;
                                num17 = 1;
                              }
                            }
                          }
                          num21: i32;
                          if (index4 < 3 & num14 > 18 & num9 < 100 & Number3 > 0)
                          {
                            let mut supplyItem: i32 =  self.ProdGetSupplyItem(locnr);
                            if (supplyItem > -1)
                            {
                              num21 = self.game.Data.ItemTypeObj[supplyItem].ProdWeight * Number3;
                              let mut num22: i32 =  100 - num9;
                              if (num22 > 100)
                              {
                                if (self.game.Data.UnitObj[self.TPlanObj[index1].HQ].HQ > -1)
                                {
                                  int[] numArray5 = numArray2;
                                  int[] numArray6 = numArray5;
                                  let mut hq3: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].HQ;
                                  let mut index10: i32 =  hq3;
                                  let mut num23: i32 =  (int) Math.Round( numArray5[hq3] + ( Number3 -  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd /  self.game.Data.ItemTypeObj[supplyItem].ProdWeight));
                                  numArray6[index10] = num23;
                                }
                                num22 = 100;
                              }
                              self.game.Data.LocObj[locnr].Production[0] = supplyItem;
                              int[] prodPercent = self.game.Data.LocObj[locnr].ProdPercent;
                              int[] numArray7 = prodPercent;
                              let mut index11: i32 =  0;
                              let mut index12: i32 =  index11;
                              let mut num24: i32 =  prodPercent[index11] + num22;
                              numArray7[index12] = num24;
                              index4 = 1;
                              num9 += num22;
                            }
                          }
                          if (num14 > 18 & num9 < 100 & index4 == 2 &  self.game.Data.RuleVar[226] > 0.0 &  self.game.Data.RuleVar[501] == 0.0)
                          {
                            let mut num25: i32 =  100 - num9;
                            self.game.Data.LocObj[locnr].Production[index4] = self.ProdGetPPItem(locnr);
                            self.game.Data.LocObj[locnr].ProdPercent[index4] = num25;
                            index4 += 1;
                            num9 += num25;
                            if (index4 < 4)
                            {
                              self.game.Data.LocObj[locnr].Production[index4] = -1;
                              self.game.Data.LocObj[locnr].ProdPercent[index4] = 0;
                            }
                          }
                          if (index4 < 3 & num14 > 18 & num9 < 100)
                          {
                            let mut supplyItem: i32 =  self.ProdGetSupplyItem(locnr);
                            if (supplyItem > -1)
                            {
                              num21 = self.game.Data.ItemTypeObj[supplyItem].ProdWeight * Number3;
                              let mut num26: i32 =  100 - num9;
                              if (num26 > 100)
                              {
                                if (self.game.Data.UnitObj[self.TPlanObj[index1].HQ].HQ > -1)
                                {
                                  int[] numArray8 = numArray2;
                                  int[] numArray9 = numArray8;
                                  let mut hq4: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].HQ;
                                  let mut index13: i32 =  hq4;
                                  let mut num27: i32 =  (int) Math.Round( numArray8[hq4] + ( Number3 -  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd /  self.game.Data.ItemTypeObj[supplyItem].ProdWeight));
                                  numArray9[index13] = num27;
                                }
                                num26 = 100;
                              }
                              self.game.Data.LocObj[locnr].Production[0] = supplyItem;
                              int[] prodPercent = self.game.Data.LocObj[locnr].ProdPercent;
                              int[] numArray10 = prodPercent;
                              let mut index14: i32 =  0;
                              let mut index15: i32 =  index14;
                              let mut num28: i32 =  prodPercent[index14] + num26;
                              numArray10[index15] = num28;
                              index4 = 1;
                              num9 += num26;
                            }
                          }
                          if (num9 < 100 & index4 < 4 & Expression.Counter > -1)
                          {
                            let mut num29: i32 =  0;
                            let mut num30: i32 =  0;
                            while (num29 == 0 & num30 < 20)
                            {
                              num30 += 1;
                              let mut index16: i32 =  (int) Math.Round( Conversion.Int(VBMath.Rnd() *  (Expression.Counter + 1)));
                              let mut role: i32 =  self.ProdGetRole(locnr, Expression.Weight[index16]);
                              if (role == -1)
                              {
                                role = self.ProdGetRole(locnr, Expression.Data1[index16]);
                                if (Expression.Data3[index16] > -1 && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, Expression.Data3[index16]).result)
                                  role = Expression.Data3[index16];
                              }
                              else if (Expression.Data2[index16] > -1 && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, Expression.Data2[index16]).result)
                                role = Expression.Data2[index16];
                              if ( self.game.Data.RuleVar[211] == 1.0 & num8 == 1 && self.ProdGetRole(locnr, 5) > -1)
                                role = self.ProdGetRole(locnr, 5);
                              let mut num31: i32 =  100 - num9;
                              if ( self.game.Data.LocObj[locnr].ProdPointRemainder[index4] >=  self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd * ( num31 / 100.0))
                              {
                                let mut num32: i32 =  num9 + self.game.Data.LocObj[locnr].ProdPercent[index4];
                                self.game.Data.LocObj[locnr].ProdPercent[index4] = num31;
                                index4 += 1;
                                break;
                              }
                              if (role > -1)
                              {
                                self.game.Data.LocObj[locnr].Production[index4] = role;
                                self.game.Data.LocObj[locnr].ProdPercent[index4] = num31;
                                index4 += 1;
                                num9 += num31;
                                num29 = 1;
                              }
                            }
                          }
                          if (index4 <= 3)
                          {
                            self.game.Data.LocObj[locnr].Production[3] = -1;
                            self.game.Data.LocObj[locnr].ProdPercent[3] = 0;
                            if (self.game.Data.LocObj[locnr].ProdPointRemainder[3] > 0)
                            {
                              int[] prodPointRemainder = self.game.Data.LocObj[locnr].ProdPointRemainder;
                              int[] numArray11 = prodPointRemainder;
                              let mut index17: i32 =  2;
                              let mut index18: i32 =  index17;
                              let mut num33: i32 =  prodPointRemainder[index17] + self.game.Data.LocObj[locnr].ProdPointRemainder[3];
                              numArray11[index18] = num33;
                              self.game.Data.LocObj[locnr].ProdPointRemainder[3] = 0;
                            }
                          }
                          if (index4 <= 2)
                          {
                            self.game.Data.LocObj[locnr].Production[2] = -1;
                            self.game.Data.LocObj[locnr].ProdPercent[2] = 0;
                            if (self.game.Data.LocObj[locnr].ProdPointRemainder[2] > 0)
                            {
                              int[] prodPointRemainder = self.game.Data.LocObj[locnr].ProdPointRemainder;
                              int[] numArray12 = prodPointRemainder;
                              let mut index19: i32 =  1;
                              let mut index20: i32 =  index19;
                              let mut num34: i32 =  prodPointRemainder[index19] + self.game.Data.LocObj[locnr].ProdPointRemainder[2];
                              numArray12[index20] = num34;
                              self.game.Data.LocObj[locnr].ProdPointRemainder[2] = 0;
                            }
                          }
                          if (index4 <= 1)
                          {
                            self.game.Data.LocObj[locnr].Production[1] = -1;
                            self.game.Data.LocObj[locnr].ProdPercent[1] = 0;
                            if (self.game.Data.LocObj[locnr].ProdPointRemainder[1] > 0)
                            {
                              int[] prodPointRemainder = self.game.Data.LocObj[locnr].ProdPointRemainder;
                              int[] numArray13 = prodPointRemainder;
                              let mut index21: i32 =  0;
                              let mut index22: i32 =  index21;
                              let mut num35: i32 =  prodPointRemainder[index21] + self.game.Data.LocObj[locnr].ProdPointRemainder[1];
                              numArray13[index22] = num35;
                              self.game.Data.LocObj[locnr].ProdPointRemainder[1] = 0;
                            }
                          }
                        }
                      }
                    }
                    else if (self.game.Data.LocObj[locnr].Production[0] > -1)
                      self.game.Data.LocObj[locnr].HQ = self.TPlanObj[index1].HQ;
                  }
                }
              }
              num3 += 1;
            }
            while (num3 <= 99);
          }
          let mut num36: i32 =  0;
          let mut num37: i32 =  0;
          if (hq1 > -1 && self.game.Data.UnitObj[hq1].PreDef == -1)
          {
            let mut locCounter1: i32 =  self.game.Data.LocCounter;
            for (let mut locnr: i32 =  0; locnr <= locCounter1; locnr += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime == self.game.Data.Turn && self.game.Data.LocObj[locnr].HQ == hq1)
              {
                let mut prodslot: i32 =  0;
                do
                {
                  if (self.game.Data.LocObj[locnr].Production[prodslot] > -1 && self.game.Data.ItemTypeObj[self.game.Data.LocObj[locnr].Production[prodslot]].IsSupply)
                    num36 = (int) Math.Round( num36 + self.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, locnr, true, false));
                  prodslot += 1;
                }
                while (prodslot <= 3);
              }
            }
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
            {
              if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.HandyFunctionsObj.IsUnitInHQChain(unr, hq1))
                num37 += self.game.HandyFunctionsObj.UnitSupplyNeed(unr, true);
            }
            if (num37 > num36)
            {
              let mut locCounter2: i32 =  self.game.Data.LocCounter;
              for (let mut index23: i32 =  0; index23 <= locCounter2; index23 += 1)
              {
                if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index23].X, self.game.Data.LocObj[index23].Y].Regime == self.game.Data.Turn && self.game.Data.LocObj[index23].HQ == hq1)
                {
                  let mut index24: i32 =  0;
                  do
                  {
                    if (self.game.Data.LocObj[index23].Production[index24] > -1 && self.game.Data.ItemTypeObj[self.game.Data.LocObj[index23].Production[index24]].IsSFType > -1)
                    {
                      self.game.Data.LocObj[index23].Production[index24] = -1;
                      self.game.Data.LocObj[index23].ProdPercent[index24] = 0;
                    }
                    index24 += 1;
                  }
                  while (index24 <= 3);
                }
              }
            }
          }
        }
      }
      let mut locCounter: i32 =  self.game.Data.LocCounter;
      for (let mut index: i32 =  0; index <= locCounter; index += 1)
      {
        if (self.game.Data.LocTypeObj[self.game.Data.LocObj[index].Type].NoHQ)
          self.game.Data.LocObj[index].HQ = -1;
      }
    }

    pub SimpleList ProdWishes(&mut self landres: i32, locnr: i32)
    {
      SimpleList simpleList1 = SimpleList::new();
      let mut num1: i32 =  0;
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & !self.game.Data.UnitObj[unr].IsHQ && self.game.Data.UnitObj[unr].AIPlanNr > 0 & self.game.Data.UnitObj[unr].PreDef == -1)
        {
          let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
          let mut num2: i32 =  0;
          if (self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan == landres)
            num2 = 1;
          if (self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan > 0)
          {
            let mut hq1: i32 =  self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan].HQ;
            let mut hq2: i32 =  self.TPlanObj[landres].HQ;
            if (hq1 > -1 & hq2 > -1 && self.IsHQinChain(hq1, hq2))
              num2 = 1;
          }
          if (num2 == 1)
          {
            let mut num3: i32 =  0;
            if (self.TPlanObj[aiPlanNr].Type == 20)
              num3 = 1;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 5)
              num3 = 1;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 9)
              num3 = 1;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 10)
              num3 = 1;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 8)
              num3 = 1;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 1)
              num3 = 1;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 2)
              num3 = 1;
            if (self.HexOA[self.TPlanObj[landres].FromArea.X, self.TPlanObj[landres].FromArea.Y] != self.HexOA[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y])
              num3 = 0;
            if (self.TPlanObj[aiPlanNr].Type == 40 & self.game.Data.UnitObj[unr].AIUnitGoal == 8 && !self.needcargoships(landres))
              num3 = 0;
            if ( self.game.Data.RuleVar[224] == 0.0 & self.game.Data.UnitObj[unr].AIUnitGoal == 5)
              num3 = 0;
            if (num3 == 1)
            {
              let mut prodpts: i32 =  (int) Math.Round( self.game.Data.LocTypeObj[self.game.Data.LocObj[locnr].Type].MaxProd / 2.0);
              RoleSFResult roleSfResult = self.LandTransferWhatWantsUnit(unr, 1, prodpts: prodpts);
              let mut rolenr1: i32 =  roleSfResult.rolenr;
              let mut itemtypenr1: i32 =  roleSfResult.itemtypenr;
              roleSfResult = self.LandTransferWhatWantsUnit(unr, 2, prodpts: prodpts);
              let mut rolenr2: i32 =  roleSfResult.rolenr;
              let mut itemtypenr2: i32 =  roleSfResult.itemtypenr;
              num1 += 1;
              if (rolenr1 > -1)
              {
                simpleList1.Add(num1, rolenr1, rolenr2, itemtypenr1, itemtypenr2);
                str: String;
                if (rolenr1 == 6)
                  str = "INFANTRY".to_owned();
                if (rolenr1 == 7)
                  str = "INFANTRYSUPPORT".to_owned();
                if (rolenr1 == 8)
                  str = "ARTILERY".to_owned();
                if (rolenr1 == 9)
                  str = "MOBILIZER".to_owned();
                if (rolenr1 == 10)
                  str = "ARMOUR".to_owned();
                if (rolenr1 == 1)
                  str = "STAFF".to_owned();
                if (rolenr1 == 2)
                  str = "LANDCAP".to_owned();
                if (rolenr1 == 3)
                  str = "SEACAP".to_owned();
                if (rolenr1 == 5)
                  str = "ENGINEER".to_owned();
                if (rolenr1 == 17)
                  str = "CARGOSHIP".to_owned();
                if (rolenr1 == 19)
                  str = "RAIDER".to_owned();
                if (rolenr1 == 18)
                  str = "SEA SUPRIORITY";
                if (rolenr1 == 13)
                  str = "FIGHTER".to_owned();
                if (rolenr1 == 14)
                  str = "TACTICAL BOMBER";
                self.AddLog("RL#" + Conversion.Str( num1) + "..." + self.game.Data.UnitObj[unr].Name + " => ROLE = " + str);
              }
            }
          }
        }
      }
      if (simpleList1.Counter == -1)
      {
        let mut tplanCount: i32 =  self.TPlanCount;
        for (let mut landres1: i32 =  1; landres1 <= tplanCount; landres1 += 1)
        {
          if (self.TPlanObj[landres1].Type == 30 && self.TPlanObj[landres1].HQ > -1 & landres1 != landres && self.game.Data.UnitObj[self.TPlanObj[landres1].HQ].HQ == self.TPlanObj[landres].HQ)
          {
            SimpleList simpleList2 = SimpleList::new();
            SimpleList simpleList3 = self.ProdWishes(landres1, locnr);
            if (simpleList3.Counter > -1)
            {
              let mut counter: i32 =  simpleList3.Counter;
              for (let mut index: i32 =  0; index <= counter; index += 1)
              {
                num1 += 1;
                simpleList1.Add(num1, simpleList3.Weight[index], simpleList3.Data1[index], simpleList3.Data2[index], simpleList3.Data3[index], simpleList3.Data4[index]);
              }
            }
          }
        }
      }
      for (let mut counter: i32 =  simpleList1.Counter; counter >= 0; counter += -1)
      {
        if (simpleList1.Data2[counter] == -1 & simpleList1.Data3[counter] == -1)
          simpleList1.Remove(simpleList1.Id[counter]);
      }
      if (self.CanProduceAir() &  self.game.Data.RuleVar[221] > 0.0 & self.game.Data.Round > 3 &&  self.GetAirPart(landres) <  self.game.Data.RuleVar[224])
      {
        for (let mut counter: i32 =  simpleList1.Counter; counter >= 0; counter += -1)
        {
          if (simpleList1.Data2[counter] > -1)
          {
            let mut isSfType: i32 =  self.game.Data.ItemTypeObj[simpleList1.Data2[counter]].IsSFType;
            if (isSfType > -1 && self.game.Data.SFTypeObj[isSfType].Theater != 2 &&  VBMath.Rnd() > 0.66)
              simpleList1.Remove(simpleList1.Id[counter]);
          }
        }
      }
      return simpleList1;
    }

    pub float GetAirPart(landres: i32)
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (self.game.Data.UnitObj[index1].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index1].PreDef == -1 & self.game.Data.UnitObj[index1].X > -1 && self.HexSA[self.game.Data.UnitObj[index1].X, self.game.Data.UnitObj[index1].Y] > 0 && self.SAObj[self.HexSA[self.game.Data.UnitObj[index1].X, self.game.Data.UnitObj[index1].Y]].LandReservePlan == landres)
        {
          let mut sfCount: i32 =  self.game.Data.UnitObj[index1].SFCount;
          for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[index1].SFList[index2];
            let mut type: i32 =  self.game.Data.SFObj[sf].Type;
            if (self.game.Data.SFTypeObj[type].Theater == 2)
              num1 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
            num2 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
          }
        }
      }
      if (num2 < 1)
        num2 = 1;
      return  num1 /  num2;
    }

    pub needcargoships: bool(landres: i32)
    {
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      let mut Left: i32 =  0;
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & !self.game.Data.UnitObj[unr].IsHQ && self.game.Data.UnitObj[unr].AIPlanNr > 0 & !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].PreDef == -1 && self.SAObj[self.GetAreaNr(self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].FromArea)].LandReservePlan == landres)
        {
          num2 += self.game.HandyFunctionsObj.GetCarryCapPts(unr, 1);
          if (!self.game.Data.UnitObj[unr].AIReserve)
            Left = Conversions.ToInteger(Operators.AddObject( Left, self.game.HandyFunctionsObj.GetUnitNonSeaWeight(unr, false)));
        }
      }
      if (num2 >= Left)
        num1 = 0;
      return num1 != 0;
    }

    pub object GetEPPerTurn(unr: i32)
    {
      let mut epPerTurn: i32 =  0;
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
        if (self.game.Data.SFTypeObj[type].EP > 0)
          epPerTurn += self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].EP;
      }
      return  epPerTurn;
    }

    pub IsHQinChain: bool(unr: i32, hq: i32)
    {
      num: i32;
      do
      {
        num += 1;
        let mut hq1: i32 =  self.game.Data.UnitObj[unr].HQ;
        if (hq1 == -1)
          return false;
        if (hq1 == hq)
          return true;
        unr = hq1;
      }
      while (num <= 9);
      self.game.Data.UnitObj[unr].HQ = -1;
      return false;
    }

    pub fn ProdGetRole(locnr: i32, rolenr: i32, bool returnsftypnr = false) -> i32
    {
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      CanProduceItemResult produceItemResult = CanProduceItemResult::new();
      if (rolenr == -1)
        return -1;
      let mut itemTypeCounter: i32 =  self.game.Data.ItemTypeCounter;
      for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
      {
        if (self.game.Data.ItemTypeObj[itemtypenr].IsSFType > -1)
        {
          if (returnsftypnr | locnr == -1)
          {
            produceItemResult.result = false;
            let mut locCounter: i32 =  self.game.Data.LocCounter;
            for (let mut locnr1: i32 =  0; locnr1 <= locCounter; locnr1 += 1)
            {
              if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[locnr1].X, self.game.Data.LocObj[locnr1].Y].Regime == self.game.Data.Turn)
              {
                produceItemResult = self.game.HandyFunctionsObj.CanProduceItem(locnr1, self.game.Data.Turn, itemtypenr);
                if (produceItemResult.result)
                  break;
              }
            }
          }
          else
            produceItemResult = self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, itemtypenr);
          if (produceItemResult.result)
          {
            let mut isSfType: i32 =  self.game.Data.ItemTypeObj[itemtypenr].IsSFType;
            num3: i32;
            if (self.game.Data.SFTypeObj[isSfType].AIRoleScore[rolenr] > num3)
            {
              num1 = itemtypenr;
              num2 = isSfType;
              num3 = self.game.Data.SFTypeObj[isSfType].AIRoleScore[rolenr];
            }
          }
        }
      }
      return returnsftypnr ? num2 : num1;
    }

    pub fn ProdGetPPItem(locnr: i32) -> i32
    {
      let mut itemTypeCounter: i32 =  self.game.Data.ItemTypeCounter;
      for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
      {
        if (self.game.Data.ItemTypeObj[itemtypenr].IsResPt && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, itemtypenr).result)
          return itemtypenr;
      }
      return -1;
    }

    pub fn ProdGetSupplyItem(locnr: i32) -> i32
    {
      let mut itemTypeCounter: i32 =  self.game.Data.ItemTypeCounter;
      for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
      {
        if (self.game.Data.ItemTypeObj[itemtypenr].IsSupply && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, itemtypenr).result)
          return itemtypenr;
      }
      return -1;
    }

    pub CanProduceAir: bool()
    {
      let mut num: i32 =  0;
      let mut itemTypeCounter: i32 =  self.game.Data.ItemTypeCounter;
      for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
      {
        let mut isSfType: i32 =  self.game.Data.ItemTypeObj[itemtypenr].IsSFType;
        if (isSfType > -1 && self.game.Data.SFTypeObj[isSfType].Theater == 2 && self.game.HandyFunctionsObj.CanProduceItem(-1, self.game.Data.Turn, itemtypenr, self.game.Data.RegimeObj[self.game.Data.Turn].People).result)
          num += 1;
      }
      return num != 0;
    }

    pub fn ExecNewairunits(phase: i32)
    {
      bool[] flagArray = new bool[self.game.Data.UnitCounter + 1];
      int[] numArray1 = new int[self.game.Data.UnitCounter + 1];
      if ( self.game.Data.RuleVar[221] < 1.0 ||  self.game.Data.RuleVar[249] == 1.0)
        return;
      self.AddLog("");
      self.AddLog("New Air Units:");
      if (!self.CanProduceAir())
        return;
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      for (let mut index1: i32 =  0; index1 <= unitCounter1; index1 += 1)
      {
        if (self.game.Data.UnitObj[index1].AIUnitGoal == 5 & self.game.Data.UnitObj[index1].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index1].PreDef == -1)
        {
          let mut index2: i32 =  self.HexSA[self.game.Data.UnitObj[index1].X, self.game.Data.UnitObj[index1].Y];
          if (self.SAObj[index2].LandReservePlan > 0 && self.TPlanObj[self.SAObj[index2].LandReservePlan].HQ > -1)
          {
            int[] numArray2 = numArray1;
            int[] numArray3 = numArray2;
            let mut hq: i32 =  self.TPlanObj[self.SAObj[index2].LandReservePlan].HQ;
            let mut index3: i32 =  hq;
            let mut num2: i32 =  numArray2[hq] + 1;
            numArray3[index3] = num2;
          }
          num1 += 1;
        }
      }
      let mut tplanCount1: i32 =  self.TPlanCount;
      num3: i32;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        if (self.TPlanObj[index].Type == 40)
          num3 += 1;
      }
      if (num1 >= num3)
        return;
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut Number: i32 =  1; Number <= tplanCount2; Number += 1)
      {
        if (self.TPlanObj[Number].Type == 40)
        {
          let mut num4: i32 =  0;
          let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter2; index += 1)
          {
            if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index].AIUnitGoal == 5 & self.game.Data.UnitObj[index].AIPlanNr == Number)
              num4 += 1;
          }
          if (num4 == 0 &&  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0 && self.SAObj[self.GetAreaNr(self.TPlanObj[Number].FromArea)].LandReservePlan > 0)
          {
            let mut hq: i32 =  self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[Number].FromArea)].LandReservePlan].HQ;
            if (hq > -1 && self.game.HandyFunctionsObj.HasUnitAirSF(hq) | numArray1[hq] == 0 && self.TPlanObj[Number].FriendlyAir < 1 | numArray1[hq] == 0 && !flagArray[hq] & self.game.Data.UnitObj[hq].AIPlanNr > 0)
            {
              let mut x: i32 =  self.TPlanObj[self.game.Data.UnitObj[hq].AIPlanNr].FromArea.X;
              let mut y: i32 =  self.TPlanObj[self.game.Data.UnitObj[hq].AIPlanNr].FromArea.Y;
              if (self.game.HandyFunctionsObj.IsHexAirfield(x, y, 0) & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = Number;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 5;
                self.AddLog("New Air unit placed at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( Number));
                int[] numArray4 = numArray1;
                int[] numArray5 = numArray4;
                let mut index4: i32 =  hq;
                let mut index5: i32 =  index4;
                let mut num5: i32 =  numArray4[index4] + 1;
                numArray5[index5] = num5;
                flagArray[hq] = true;
              }
            }
          }
        }
      }
    }

    pub MakeNavyActive: bool(plnr: i32)
    {
      let mut num1: i32 =  1;
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].AIPlanNr == plnr && self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & self.game.HandyFunctionsObj.HasUnitNavySF(unr))
          return true;
      }
      let mut saCount1: i32 =  self.SACount;
      for (let mut area1: i32 =  1; area1 <= saCount1; area1 += 1)
      {
        if (self.SAObj[area1].ConstitutantCount < 1)
        {
          let mut saCount2: i32 =  self.SACount;
          for (let mut index: i32 =  1; index <= saCount2; index += 1)
          {
            if (self.SAObj[index].ConstitutantCount < 1 && self.HexOA[self.SAObj[index].X, self.SAObj[index].Y] == self.HexOA[self.TPlanObj[plnr].FromArea.X, self.TPlanObj[plnr].FromArea.Y] && area1 != index)
            {
              let mut num2: i32 =  0;
              if (self.game.Data.MapObj[0].HexObj[self.SAObj[area1].X, self.SAObj[area1].Y].Regime != -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[self.SAObj[area1].X, self.SAObj[area1].Y].Regime] == 0)
                num2 = 1;
              if (num2 == 1)
              {
                if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[self.SAObj[index].X, self.SAObj[index].Y].Regime, self.game.Data.Turn))
                {
                  if (self.SAObj[area1].IsNeighbour(index))
                    num1 = 0;
                }
                else if (self.JoinedNeighbour(area1, index))
                  num1 = 0;
              }
            }
          }
        }
      }
      return num1 == 1;
    }

    pub fn ExecNewNavyunits(phase: i32)
    {
      bool[] flagArray = new bool[self.game.Data.UnitCounter + 1];
      int[] numArray = new int[self.game.Data.UnitCounter + 1];
      if ( self.game.Data.RuleVar[227] < 1.0)
        return;
      self.AddLog("");
      self.AddLog("New Navy Units:");
      if ( self.game.Data.RuleVar[249] == 1.0)
        return;
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 40 & self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].SeaNeighbourCount > 0 && self.TPlanObj[index1].SeaTarget > 0 && self.MakeNavyActive(index1))
        {
          let mut num1: i32 =  0;
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
          {
            if (self.game.Data.UnitObj[index2].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index2].AIPlanNr == index1 && self.game.Data.UnitObj[index2].AIUnitGoal == 10 | self.game.Data.UnitObj[index2].AIUnitGoal == 9 | self.game.Data.UnitObj[index2].AIUnitGoal == 8)
            {
              num1 += 1;
              if (self.game.Data.UnitObj[index2].SFCount == -1 & self.TPlanObj[index1].FromArea.LandReservePlan > -1)
              {
                let mut hq: i32 =  self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan].HQ;
                if (hq > -1)
                  flagArray[hq] = true;
              }
            }
          }
          let mut num2: i32 =  0;
          if ( num2 <  self.game.Data.RuleVar[239])
            num2 = (int) Math.Round( self.game.Data.RuleVar[239]);
          if (self.TPlanObj[index1].SeaTarget > 0 & self.TPlanObj[index1].SeaTarget <= self.SACount)
          {
            let mut num3: i32 =  self.game.HandyFunctionsObj.Distance(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y, 0, self.SAObj[self.TPlanObj[index1].SeaTarget].X, self.SAObj[self.TPlanObj[index1].SeaTarget].Y, 0);
            if (num3 > 10)
              num2 = (int) Math.Round( num2 + Math.Sqrt( (num3 - 10) / 10.0) *  self.game.Data.RuleVar[239]);
          }
          if (self.TPlanObj[index1].EnemyNavy == 0)
            num2 = self.TPlanObj[index1].SeaTarget >= 1 ? (int) Math.Round(1.0 +  num2 / 2.0) : 0;
          if (num1 < num2 &&  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0 && self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan > 0)
          {
            let mut hq: i32 =  self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan].HQ;
            if (hq > -1 && !flagArray[hq])
            {
              let mut x: i32 =  self.game.Data.UnitObj[hq].X;
              let mut y: i32 =  self.game.Data.UnitObj[hq].Y;
              if (self.game.HandyFunctionsObj.IsHexPort(x, y, 0) & self.game.Data.MapObj[0].HexObj[x, y].Regime == self.game.Data.Turn & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = index1;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 0;
                self.AddLog("New Naval unit placed at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( index1));
                flagArray[hq] = true;
                self.InitUnitGoals(self.game.Data.UnitCounter);
              }
            }
          }
        }
      }
    }

    pub Existingunitsok: bool(plnr: i32)
    {
      let mut num1: i32 =  999;
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num2: i32;
      num3: i32;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].AIPlanNr == plnr & self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].X > -1)
        {
          let mut powerPtsAbsolute: i32 =  self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true);
          num2 += powerPtsAbsolute;
          num3 += 1;
          if (powerPtsAbsolute < num1)
            num1 = powerPtsAbsolute;
        }
      }
      if (num3 < 1)
        return true;
      let mut num4: i32 =  (int) Math.Round( num2 /  num3);
      return  num1 >=  self.game.Data.RuleVar[182] &&  num4 >=  self.game.Data.RuleVar[247];
    }

    pub fn GetPlanUnitGoalPercent(plnr: i32, goaltype: i32) -> i32
    {
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      let mut num4: i32 =  0;
      let mut num5: i32 =  0;
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (!self.game.Data.UnitObj[index].IsHQ && self.game.Data.UnitObj[index].AIPlanNr == plnr & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
        {
          if (self.game.Data.UnitObj[index].AIUnitGoal > 0 & self.game.Data.UnitObj[index].PreDef == -1)
          {
            let mut aiUnitGoal: i32 =  self.game.Data.UnitObj[index].AIUnitGoal;
            if (aiUnitGoal == 1)
            {
              num1 += 1;
              num4 += 1;
            }
            if (aiUnitGoal == 2)
            {
              num2 += 1;
              num4 += 1;
            }
            if (aiUnitGoal == 3)
            {
              num3 += 1;
              num4 += 1;
            }
          }
          num5 += 1;
        }
      }
      if (num4 == 0)
        return 0;
      switch (goaltype)
      {
        case 1:
          return (int) Math.Round( (num1 * 100) /  num4);
        case 2:
          return (int) Math.Round( (num2 * 100) /  num4);
        case 3:
          return (int) Math.Round( (num3 * 100) /  num4);
        default:
          return 0;
      }
    }

    pub fn ExecNewLandUnits(phase: i32)
    {
      self.AddLog("");
      self.AddLog("New Units:");
      if ( self.game.Data.RuleVar[249] == 1.0)
        return;
      if (phase == 1)
      {
        let mut tplanCount: i32 =  self.TPlanCount;
        for (let mut index1: i32 =  1; index1 <= tplanCount; index1 += 1)
        {
          if (self.TPlanObj[index1].Type == 20)
          {
            float num1 =  self.TPlanObj[index1].FriendlyUnitCount;
            if ( num1 < 1.0)
              num1 = 0.25f;
            let mut num2: i32 =  0;
            if (self.TPlanObj[index1].Stand == 1)
            {
              if ( self.GetPlanUnitGoalPercent(index1, 2) <  self.game.Data.RuleVar[171] * 0.66)
                num2 = 1;
              if ( self.GetPlanUnitGoalPercent(index1, 1) <  self.game.Data.RuleVar[172] * 0.66)
                num2 = 1;
              if ( self.GetPlanUnitGoalPercent(index1, 3) <  self.game.Data.RuleVar[173])
                num2 = 1;
            }
            else
            {
              if ( self.GetPlanUnitGoalPercent(index1, 2) <  self.game.Data.RuleVar[161] * 0.66)
                num2 = 1;
              if ( self.GetPlanUnitGoalPercent(index1, 1) <  self.game.Data.RuleVar[162] * 0.66)
                num2 = 1;
              if ( self.GetPlanUnitGoalPercent(index1, 3) <  self.game.Data.RuleVar[163])
                num2 = 1;
            }
            if (num2 == 1)
              self.AddLog("Set ok3=1 because we need other unitgoals in plan too.");
            if ( (self.TPlanObj[index1].FrontSize * 100) /  num1 >  self.game.Data.RuleVar[155] | num2 == 1)
            {
              let mut num3: i32 =  self.TPlanObj[index1].EnemyUnitCount * 1;
              if (num3 == 0)
                num3 = 1;
              if (self.TPlanObj[index1].FriendlyUnitCount < num3 & self.Existingunitsok(index1) | num2 == 1 &&  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0 && self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan > 0)
              {
                let mut hq: i32 =  self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan].HQ;
                let mut num4: i32 =  0;
                if (hq > -1)
                {
                  let mut sfCount: i32 =  self.game.Data.UnitObj[hq].SFCount;
                  for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
                  {
                    let mut sf: i32 =  self.game.Data.UnitObj[hq].SFList[index2];
                    let mut type: i32 =  self.game.Data.SFObj[sf].Type;
                    if (self.game.Data.SFTypeObj[type].AIRoleScore[1] < 1 && self.game.Data.SFTypeObj[type].AIRoleScore[2] < 1)
                      num4 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
                  }
                }
                if (self.TPlanObj[index1].FriendlyUnitCount == 0 & self.TPlanObj[index1].TooArea.aivp > 0)
                  num4 = 1;
                if (num4 > 0 & hq > -1 && self.game.Data.UnitObj[hq].LandCap > 0 |  self.game.Data.RuleVar[253] == 0.0 | self.TPlanObj[index1].FriendlyUnitCount == 0 & self.TPlanObj[index1].TooArea.aivp > 0)
                {
                  let mut num5: i32 =  0;
                  let mut unitCounter: i32 =  self.game.Data.UnitCounter;
                  for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
                  {
                    if (self.game.Data.UnitObj[unr].AIPlanNr == index1 && self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn &&  self.game.HandyFunctionsObj.GetUnitStackPts(unr) <  self.game.Data.RuleVar[182])
                      num5 = 1;
                  }
                  if (num5 == 0)
                  {
                    x: i32;
                    y: i32;
                    num6: i32;
                    if (self.TPlanObj[index1].HQ > -1)
                    {
                      x = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X;
                      y = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y;
                      num6 = self.TPlanObj[index1].HQ;
                    }
                    else
                    {
                      num6 = self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan <= 0 ? -1 : self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan].HQ;
                      x = self.TPlanObj[index1].FromArea.X;
                      y = self.TPlanObj[index1].FromArea.Y;
                      if (self.TPlanObj[index1].Stand == 3)
                      {
                        let mut neighbourForRetreater: i32 =  self.GetBestNeighbourForRetreater(self.GetAreaNr(self.TPlanObj[index1].FromArea));
                        if (neighbourForRetreater > -1)
                        {
                          x = self.SAObj[neighbourForRetreater].X;
                          y = self.SAObj[neighbourForRetreater].Y;
                        }
                      }
                    }
                    if (self.game.Data.MapObj[0].HexObj[x, y].Regime == self.game.Data.Turn & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
                    {
                      self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                      self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = num6;
                      self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = index1;
                      tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                      aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                      let mut index3: i32 =  index1;
                      let mut index4: i32 =  index3;
                      aiPlanClassArray[index4].FriendlyUnitCount = tplanObj[index3].FriendlyUnitCount + 1;
                      self.AddLog("New unit placed at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( index1));
                      self.InitUnitGoals(self.game.Data.UnitCounter);
                    }
                  }
                }
              }
            }
          }
          if (self.TPlanObj[index1].Type == 40 && self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan > 0)
          {
            let mut num7: i32 =  0;
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
            {
              if (self.game.Data.UnitObj[index5].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index5].AIPlanNr == index1 && self.game.Data.UnitObj[index5].AIUnitGoal == 1 && self.game.Data.UnitObj[index5].X > -1 & !self.game.Data.UnitObj[index5].AIReserve && self.HexSA[self.game.Data.UnitObj[index5].X, self.game.Data.UnitObj[index5].Y] == self.GetAreaNr(self.TPlanObj[index1].FromArea))
                num7 += 1;
            }
            if (num7 < 2 && self.TPlanObj[index1].EnemyUnitCount < 1 && !self.EmptyUnitForPlan(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y, index1, 1) && self.TPlanObj[index1].SeaTarget > 0 & self.TPlanObj[index1].SeaStand == 7 && self.MakeNavyActive(index1))
            {
              x: i32;
              y: i32;
              num8: i32;
              if (self.TPlanObj[index1].HQ > -1)
              {
                x = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X;
                y = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y;
                num8 = self.TPlanObj[index1].HQ;
              }
              else
              {
                num8 = self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan <= 0 ? -1 : self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan].HQ;
                x = self.TPlanObj[index1].FromArea.X;
                y = self.TPlanObj[index1].FromArea.Y;
              }
              if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[x, y].Regime, self.game.Data.Turn) & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                RegimeClass[] regimeClassArray = regimeObj;
                let mut turn: i32 =  self.game.Data.Turn;
                let mut index6: i32 =  turn;
                regimeClassArray[index6].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = num8;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = index1;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 1;
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index7: i32 =  index1;
                let mut index8: i32 =  index7;
                aiPlanClassArray[index8].FriendlyUnitCount = tplanObj[index7].FriendlyUnitCount + 1;
                self.AddLog("New INF unit placed for AMPHIBIOUS OPS at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( index1));
              }
            }
          }
          if (self.TPlanObj[index1].Type == 40 && self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan > 0)
          {
            let mut num9: i32 =  0;
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut index9: i32 =  0; index9 <= unitCounter; index9 += 1)
            {
              if (self.game.Data.UnitObj[index9].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index9].AIPlanNr == index1 && self.game.Data.UnitObj[index9].AIUnitGoal == 1 && self.game.Data.UnitObj[index9].X > -1 & self.game.Data.UnitObj[index9].AIReserve && self.HexSA[self.game.Data.UnitObj[index9].X, self.game.Data.UnitObj[index9].Y] == self.GetAreaNr(self.TPlanObj[index1].FromArea))
                num9 += 1;
            }
            if (num9 < 1 && self.getfrontplan(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y) == -1 && !self.EmptyUnitForPlan(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y, index1, 1))
            {
              let mut num10: i32 =  0;
              if (self.TPlanObj[index1].SeaTarget < 1 & self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].SeaNeighbourCount > 0)
                num10 = 1;
              if (num10 == 0 &  self.game.Data.RuleVar[252] > 0.0 && self.TPlanObj[index1].AssemblyArea == 1)
                num10 = 1;
              if (num10 == 0 &  self.game.Data.RuleVar[256] > 0.0 && self.TPlanObj[index1].AssemblyArea == 1)
                num10 = 1;
              if (num10 == 1)
              {
                x: i32;
                y: i32;
                num11: i32;
                if (self.TPlanObj[index1].HQ > -1)
                {
                  x = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X;
                  y = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y;
                  num11 = self.TPlanObj[index1].HQ;
                }
                else
                {
                  num11 = self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan <= 0 ? -1 : self.TPlanObj[self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan].HQ;
                  x = self.TPlanObj[index1].FromArea.X;
                  y = self.TPlanObj[index1].FromArea.Y;
                }
                if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
                {
                  RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                  RegimeClass[] regimeClassArray = regimeObj;
                  let mut turn: i32 =  self.game.Data.Turn;
                  let mut index10: i32 =  turn;
                  regimeClassArray[index10].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                  self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = num11;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = index1;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal =  VBMath.Rnd() <= 0.5 ? 2 : 1;
                  tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                  aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                  let mut index11: i32 =  index1;
                  let mut index12: i32 =  index11;
                  aiPlanClassArray[index12].FriendlyUnitCount = tplanObj[index11].FriendlyUnitCount + 1;
                  self.AddLog("New INF or ARM unit placed for BACKPLAN WITHOUT AMPH/FRONT or DEFENSE IN DEPTH at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( index1));
                }
              }
            }
          }
        }
      }
      if (phase != 2)
        return;
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index13: i32 =  1; index13 <= tplanCount1; index13 += 1)
      {
        let mut hq: i32 =  self.TPlanObj[index13].HQ;
        if (hq > -1)
        {
          let mut num12: i32 =  0;
          let mut num13: i32 =  0;
          let mut num14: i32 =  0;
          let mut num15: i32 =  0;
          let mut x: i32 =  self.game.Data.UnitObj[hq].X;
          let mut y: i32 =  self.game.Data.UnitObj[hq].Y;
          let mut sfCount: i32 =  self.game.Data.UnitObj[hq].SFCount;
          for (let mut index14: i32 =  0; index14 <= sfCount; index14 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[hq].SFList[index14];
            let mut type: i32 =  self.game.Data.SFObj[sf].Type;
            if (self.game.Data.SFTypeObj[type].AIRoleScore[1] < 1 && self.game.Data.SFTypeObj[type].AIRoleScore[2] < 1 && self.game.Data.SFTypeObj[type].AIRoleScore[5] < 1 && self.game.Data.SFTypeObj[type].AIRoleScore[7] < 25 && self.game.Data.SFTypeObj[type].AIRoleScore[12] < 25)
            {
              num12 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[6] > 50)
                num13 = 1;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[10] > 50)
                num14 = 1;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[8] > 50)
                num15 = 1;
            }
          }
          if (num12 > 0 & (num13 > 0 | num14 > 0 | num15 > 0))
          {
            let mut Number: i32 =  self.getclosestplan(x, y, 20);
            if (Number > -1)
            {
              let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
              for (let mut index15: i32 =  0; index15 <= unitCounter; index15 += 1)
              {
                let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitList[index15];
                if (!self.game.Data.UnitObj[unit].IsHQ & self.game.Data.UnitObj[unit].SFCount == -1 && self.game.Data.UnitObj[unit].AIUnitGoal == 1 | self.game.Data.UnitObj[unit].AIUnitGoal == 2 | self.game.Data.UnitObj[unit].AIUnitGoal == 3)
                {
                  self.AddLog("Already empty unit at HQ");
                  Number = -1;
                }
              }
            }
            if (Number > -1)
            {
              if (( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0) & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = Number;
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index16: i32 =  index13;
                let mut index17: i32 =  index16;
                aiPlanClassArray[index17].FriendlyUnitCount = tplanObj[index16].FriendlyUnitCount + 1;
                self.AddLog("New unit placed at Landreserve " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( Number));
                if (num14 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 2;
                  num14 = 0;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIMobilize = true;
                }
                else if (num15 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 3;
                  num15 = 0;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIMobilize = true;
                }
                else if (num13 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 1;
                  num13 = 0;
                }
              }
              if ( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0 && num14 > 0 | num15 > 0 | num13 > 0 & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = Number;
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index18: i32 =  index13;
                let mut index19: i32 =  index18;
                aiPlanClassArray[index19].FriendlyUnitCount = tplanObj[index18].FriendlyUnitCount + 1;
                self.AddLog("New unit placed at Landreserve " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( Number));
                if (num14 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 2;
                  num14 = 0;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIMobilize = true;
                }
                else if (num15 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 3;
                  num15 = 0;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIMobilize = true;
                }
                else if (num13 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 1;
                  num13 = 0;
                }
              }
              if (( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0) & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = Number;
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index20: i32 =  index13;
                let mut index21: i32 =  index20;
                aiPlanClassArray[index21].FriendlyUnitCount = tplanObj[index20].FriendlyUnitCount + 1;
                self.AddLog("New unit placed at Landreserve " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( Number));
                if (num14 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 2;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIMobilize = true;
                }
                else if (num15 == 1)
                {
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 3;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIMobilize = true;
                }
                else if (num13 == 1)
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 1;
              }
            }
          }
        }
      }
    }

    pub EmptyUnitForPlan: bool(x: i32, y: i32, plannr: i32, goal: i32)
    {
      let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[x, y].UnitList[index]].AIUnitGoal == goal && self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[x, y].UnitList[index]].SFCount == -1)
          return true;
      }
      return false;
    }

    pub fn ExecSplitLandUnits()
    {
      self.AddLog("");
      self.AddLog("Splitting Units:");
      if ( self.game.Data.RuleVar[249] == 1.0)
        return;
      let mut tplanCount1: i32 =  self.TPlanCount;
      num1: i32;
      for (let mut Number: i32 =  1; Number <= tplanCount1; Number += 1)
      {
        if (self.TPlanObj[Number].Type == 20)
        {
          float num2 =  self.TPlanObj[Number].FriendlyUnitCount;
          if ( num2 < 1.0)
            num2 = 0.25f;
          if ( ((self.TPlanObj[Number].FrontSize + 1) * 100) /  num2 >  self.game.Data.RuleVar[155])
          {
            let mut num3: i32 =  self.TPlanObj[Number].EnemyUnitCount * 1;
            if (num3 == 0)
              num3 = 1;
            if (self.TPlanObj[Number].FriendlyUnitCount < num3 & self.TPlanObj[Number].FriendlyUnitCount > 0 &&  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0 && self.SAObj[self.GetAreaNr(self.TPlanObj[Number].FromArea)].LandReservePlan > 0)
            {
              SimpleList simpleList = SimpleList::new();
              num1 = 0;
              let mut unitCounter: i32 =  self.game.Data.UnitCounter;
              for (let mut index: i32 =  0; index <= unitCounter; index += 1)
              {
                if (self.game.Data.UnitObj[index].AIPlanNr == Number && self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && !self.game.Data.UnitObj[index].IsHQ)
                {
                  let mut unitStackPts: i32 =  self.game.HandyFunctionsObj.GetUnitStackPts(index);
                  if ( unitStackPts >=  self.game.Data.RuleVar[247] * 2.5)
                    simpleList.Add(index, unitStackPts);
                }
              }
              simpleList.Sort();
              if (simpleList.Counter > -1)
              {
                let mut unr: i32 =  simpleList.Id[simpleList.Counter];
                let mut x: i32 =  self.game.Data.UnitObj[unr].X;
                let mut y: i32 =  self.game.Data.UnitObj[unr].Y;
                let mut hq: i32 =  self.game.Data.UnitObj[unr].HQ;
                if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
                {
                  self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                  self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = Number;
                  tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                  aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                  let mut index1: i32 =  Number;
                  let mut index2: i32 =  index1;
                  aiPlanClassArray[index2].FriendlyUnitCount = tplanObj[index1].FriendlyUnitCount + 1;
                  self.AddLog("Unit " + self.game.Data.UnitObj[unr].Name + " split at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( Number));
                  for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount; sfCount >= 0; sfCount += -1)
                  {
                    let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[sfCount];
                    let mut Qty: i32 =  (int) Math.Round(Conversion.Int( self.game.Data.SFObj[sf].Qty / 2.0));
                    if (Qty == 0 & self.game.Data.SFObj[sf].Qty > 0 &&  VBMath.Rnd() > 0.5)
                      Qty = 1;
                    self.game.ProcessingObj.DoTransfer(unr, self.game.Data.UnitCounter, 0, sf, Qty, true);
                  }
                }
              }
            }
          }
        }
      }
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut Number: i32 =  1; Number <= tplanCount2; Number += 1)
      {
        if (self.TPlanObj[Number].Type == 20 | self.TPlanObj[Number].Type == 40 &&  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0 && self.SAObj[self.GetAreaNr(self.TPlanObj[Number].FromArea)].LandReservePlan > 0)
        {
          num1 = 0;
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut unr1: i32 =  0; unr1 <= unitCounter; unr1 += 1)
          {
            if (self.game.Data.UnitObj[unr1].AIPlanNr == Number && self.game.Data.UnitObj[unr1].Regime == self.game.Data.Turn && !self.game.Data.UnitObj[unr1].IsHQ && self.game.Data.UnitObj[unr1].AIUnitGoal == 5 | self.game.Data.UnitObj[unr1].AIUnitGoal == 1 | self.game.Data.UnitObj[unr1].AIUnitGoal == 2 | self.game.Data.UnitObj[unr1].AIUnitGoal == 3 &&  (self.game.HandyFunctionsObj.GetUnitStackPts(unr1) + self.game.HandyFunctionsObj.GetUnitairStackPts(unr1)) >  self.game.Data.RuleVar[30] &&  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46])
            {
              let mut unr2: i32 =  unr1;
              let mut x: i32 =  self.game.Data.UnitObj[unr2].X;
              let mut y: i32 =  self.game.Data.UnitObj[unr2].Y;
              if (x > -1 & self.game.Data.UnitObj[unr2].PreDef == -1 && self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                let mut hq: i32 =  self.game.Data.UnitObj[unr2].HQ;
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = Number;
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index3: i32 =  Number;
                let mut index4: i32 =  index3;
                aiPlanClassArray[index4].FriendlyUnitCount = tplanObj[index3].FriendlyUnitCount + 1;
                self.AddLog("TO FAT Unit " + self.game.Data.UnitObj[unr2].Name + " split at " + Conversion.Str( x) + "," + Conversion.Str( y) + " for plan #" + Conversion.Str( Number));
                for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr2].SFCount; sfCount >= 0; sfCount += -1)
                {
                  let mut sf: i32 =  self.game.Data.UnitObj[unr2].SFList[sfCount];
                  let mut Qty: i32 =  (int) Math.Round(Conversion.Int( self.game.Data.SFObj[sf].Qty / 2.0));
                  if (Qty == 0 & self.game.Data.SFObj[sf].Qty > 0 &&  VBMath.Rnd() > 0.5)
                    Qty = 1;
                  self.game.ProcessingObj.DoTransfer(unr2, self.game.Data.UnitCounter, 0, sf, Qty, true);
                }
              }
            }
          }
        }
      }
    }

    pub fn ExecDisbandForTransfer()
    {
      let mut num: i32 =  1;
      while (num == 1)
      {
        num = 0;
        let mut tplanCount: i32 =  self.TPlanCount;
        for (let mut Number: i32 =  1; Number <= tplanCount; Number += 1)
        {
          if (self.TPlanObj[Number].Type == 30)
          {
            let mut hq: i32 =  self.TPlanObj[Number].HQ;
            if (hq > -1)
            {
              let mut x: i32 =  self.game.Data.UnitObj[hq].X;
              let mut y: i32 =  self.game.Data.UnitObj[hq].Y;
              for (let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter; unitCounter >= 0; unitCounter += -1)
              {
                let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitList[unitCounter];
                if (self.game.Data.UnitObj[unit].AIDisband & hq != unit && self.game.Data.UnitObj[unit].SFCount > -1)
                {
                  for (let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount; sfCount >= 0; sfCount += -1)
                  {
                    let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[sfCount];
                    let mut qty: i32 =  self.game.Data.SFObj[sf].Qty;
                    self.game.ProcessingObj.DoTransfer(unit, hq, 0, sf, qty, true, false);
                  }
                  self.AddLog("LandResPlannr: " + Conversion.Str( Number) + ", Disbanded unit " + self.game.Data.UnitObj[unit].Name);
                  if (self.game.Data.UnitObj[unit].IsHQ)
                  {
                    RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                    RegimeClass[] regimeClassArray = regimeObj;
                    let mut turn: i32 =  self.game.Data.Turn;
                    let mut index: i32 =  turn;
                    regimeClassArray[index].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[47]));
                  }
                  else
                  {
                    RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                    RegimeClass[] regimeClassArray = regimeObj;
                    let mut turn: i32 =  self.game.Data.Turn;
                    let mut index: i32 =  turn;
                    regimeClassArray[index].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                  }
                  self.game.Data.RemoveUnit(unit, ref self.game);
                  num = 1;
                  break;
                }
              }
            }
          }
        }
      }
    }

    pub fn EmptyHQ()
    {
      if ( self.game.Data.RuleVar[253] > 0.0)
        return;
      for (let mut unitCounter1: i32 =  self.game.Data.UnitCounter; unitCounter1 >= 0; unitCounter1 += -1)
      {
        if (self.game.Data.UnitObj[unitCounter1].IsHQ & self.game.Data.UnitObj[unitCounter1].PreDef == -1 & self.game.Data.UnitObj[unitCounter1].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unitCounter1].X > -1)
        {
          let mut unr: i32 =  unitCounter1;
          let mut x: i32 =  self.game.Data.UnitObj[unitCounter1].X;
          let mut y: i32 =  self.game.Data.UnitObj[unitCounter1].Y;
          let mut num1: i32 =  0;
          while (num1 == 0)
          {
            num1 = 1;
            for (let mut sfCount: i32 =  self.game.Data.UnitObj[unitCounter1].SFCount; sfCount >= 0; sfCount += -1)
            {
              let mut sf: i32 =  self.game.Data.UnitObj[unitCounter1].SFList[sfCount];
              let mut type: i32 =  self.game.Data.SFObj[sf].Type;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[1] < 10)
              {
                let mut num2: i32 =  0;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[6] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[7] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[5] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[10] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[17] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[13] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[8] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[18] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[19] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[14] > 0)
                  num2 = 1;
                if (self.game.Data.SFTypeObj[type].AIRoleScore[15] > 0)
                  num2 = 1;
                if (num2 == 1 & self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
                {
                  RegimeClass[] regimeObj = self.game.Data.RegimeObj;
                  RegimeClass[] regimeClassArray = regimeObj;
                  let mut turn: i32 =  self.game.Data.Turn;
                  let mut index: i32 =  turn;
                  regimeClassArray[index].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[46]));
                  self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                  let mut Qty: i32 =  self.game.Data.SFObj[sf].Qty;
                  let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
                  if (Qty > 0)
                  {
                    let mut num3: i32 =  Qty * self.game.Data.SFTypeObj[type].Frontage;
                    if (num3 > 60)
                      Qty = (int) Math.Round(Math.Min(Conversion.Int( Qty / 2.0), Conversion.Int( Qty * (60.0 /  num3))));
                    if (Qty == 0)
                      Qty = 1;
                    self.game.Data.UnitObj[unitCounter2].HQ = unr;
                    self.game.ProcessingObj.DoTransfer(unr, unitCounter2, self.game.Data.SFTypeObj[type].Theater, sf, Qty, true);
                    num1 = 0;
                  }
                }
              }
            }
          }
        }
      }
    }

    pub fn ExecLandTransfers(phase: i32)
    {
      int[] numArray1 = new int[self.game.Data.UnitCounter + 1];
      int[] numArray2 = new int[self.game.Data.UnitCounter + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut tplanCount1: i32 =  self.TPlanCount;
      num1: i32;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        if (self.TPlanObj[index].Type == 20)
          num1 = (int) Math.Round( ( num1 + self.TPlanObj[index].WeightEnemyForce));
      }
      if (num1 == 0)
        num1 = 1;
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount2; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 30 | self.TPlanObj[index1].Type == 20 && self.TPlanObj[index1].HQ > -1)
        {
          self.AddLog("");
          self.AddLog("*TransferExec for " + self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Name + " PHASE " + Conversion.Str( phase));
          let mut x: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X;
          let mut y: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y;
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0);
          let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
              numArray3[index2, index3] = self.game.EditObj.TempValue[0].Value[index2, index3];
          }
          numArray3[x, y] = 0;
          SimpleList simpleList1 = SimpleList::new();
          let mut num2: i32 =  0;
          let mut tplanCount3: i32 =  self.TPlanCount;
          num3: i32;
          for (let mut index4: i32 =  1; index4 <= tplanCount3; index4 += 1)
          {
            if ( self.game.Data.RuleVar[253] == 0.0 | phase == 1 & (self.TPlanObj[index4].Type == 50 | self.TPlanObj[index4].Type == 20 | self.TPlanObj[index4].Type == 40) & self.SAObj[self.GetAreaNr(self.TPlanObj[index4].FromArea)].LandReservePlan == index1 | self.TPlanObj[index4].Type == 20 & phase == 2)
            {
              if ( numArray3[self.TPlanObj[index4].FromArea.X, self.TPlanObj[index4].FromArea.Y] <  self.game.Data.RuleVar[78] | phase == 2)
              {
                let mut num4: i32 =  0;
                if (phase == 2)
                  num4 = 100;
                let mut num5: i32 =  0;
                if (self.TPlanObj[index4].Type == 20)
                {
                  if ( self.TPlanObj[index4].WeightEnemyForce >  self.TPlanObj[index4].WeightFriendlyForce)
                  {
                    let mut num6: i32 =  (int) Math.Round(100.0 * ( self.TPlanObj[index4].WeightEnemyForce / ( self.TPlanObj[index4].WeightFriendlyForce + 1.0)));
                    num5 = (int) Math.Round(100.0 * ( self.TPlanObj[index4].WeightEnemyForce / ( self.TPlanObj[index4].WeightFriendlyForce + 1.0))) + self.TPlanObj[index1].WeightStrategic;
                    num3 = (int) Math.Round( ( (num6 * (self.TPlanObj[index4].WeightStrategic + 1)) * (self.TPlanObj[index4].WeightEnemyForce /  num1)));
                  }
                  else
                  {
                    num5 = (int) Math.Round(100.0 * ( self.TPlanObj[index4].WeightEnemyForce / ( self.TPlanObj[index4].WeightFriendlyForce + 1.0))) + self.TPlanObj[index1].WeightStrategic;
                    num3 = (int) Math.Round( ( ((int) Math.Round(100.0 * ( self.TPlanObj[index4].WeightEnemyForce / ( self.TPlanObj[index4].WeightFriendlyForce + 1.0))) * (self.TPlanObj[index4].WeightStrategic + 1)) * (self.TPlanObj[index4].WeightEnemyForce /  num1)));
                    if (num3 == 0)
                      num3 = 10 * (self.TPlanObj[index4].WeightStrategic + 1);
                  }
                }
                else if (self.TPlanObj[index4].Type == 40)
                {
                  if ( self.game.Data.RuleVar[252] > 0.0 & self.TPlanObj[index4].AssemblyArea == 1)
                  {
                    num3 = (int) Math.Round( ( (self.TPlanObj[index4].WeightStrategic * 25) * self.game.Data.RuleVar[254]));
                    let mut closestFrontline: i32 =  self.GetClosestFrontline(self.TPlanObj[index4].FromArea.X, self.TPlanObj[index4].FromArea.Y);
                    if (closestFrontline > 0)
                    {
                      if (self.TPlanObj[closestFrontline].Stand == 2)
                        num3 *= 3;
                      if (self.TPlanObj[closestFrontline].Stand == 3)
                        num3 *= 6;
                    }
                  }
                  else
                    num3 = self.TPlanObj[index4].WeightStrategic * 10;
                  if (num3 == 0)
                    num3 = 1;
                  num5 = num3;
                }
                else if (self.TPlanObj[index4].Type == 50)
                {
                  num3 = 50 * (self.SAObj[self.GetAreaNr(self.TPlanObj[index4].FromArea)].fuzzyvp + self.SAObj[self.GetAreaNr(self.TPlanObj[index4].TooArea)].fuzzyvp);
                  num5 = num3;
                }
                if (num3 > 0)
                  num4 += num3;
                if (num4 > 0)
                {
                  self.AddLog("Plan #" + Conversion.Str( index4) + " importance=" + Conversion.Str( num5) + ", weight=" + Conversion.Str( num4));
                  simpleList1.Add(index4, num4, num5);
                  num2 += num4;
                }
              }
              else
                self.AddLog("Plan #" + Conversion.Str( index4) + " IS OUT OF RANGE");
            }
          }
          let mut num7: i32 =  0;
          if (self.TPlanObj[index1].Type == 30 && phase == 1)
          {
            let mut tplanCount4: i32 =  self.TPlanCount;
            for (let mut index5: i32 =  1; index5 <= tplanCount4; index5 += 1)
            {
              if (self.TPlanObj[index5].Type == 30 && index5 != index1 && self.TPlanObj[index1].WeightStrategic < self.TPlanObj[index5].WeightStrategic && self.TPlanObj[index5].MetaChainNr > self.TPlanObj[index1].MetaChainNr &&  numArray3[self.TPlanObj[index5].FromArea.X, self.TPlanObj[index5].FromArea.Y] <  self.game.Data.RuleVar[78])
              {
                let mut num8: i32 =  self.TPlanObj[index5].WeightStrategic - self.TPlanObj[index1].WeightStrategic;
                let mut num9: i32 =  self.TPlanObj[index5].WeightStrategic - self.TPlanObj[index1].WeightStrategic;
                if (num9 > 0)
                {
                  simpleList1.Add(index5, num9, num8);
                  self.AddLog("Plan #" + Conversion.Str( index5) + " importance=" + Conversion.Str( num8) + ", weight=" + Conversion.Str( num9));
                  num2 += num9;
                  num7 += 1;
                }
              }
            }
          }
          self.AddLog("=>toUnits");
          if (simpleList1.Counter > -1)
          {
            SimpleList simpleList2 = SimpleList::new();
            let mut num10: i32 =  0;
            let mut counter1: i32 =  simpleList1.Counter;
            for (let mut index6: i32 =  0; index6 <= counter1; index6 += 1)
            {
              let mut plnr: i32 =  simpleList1.Id[index6];
              num11: i32;
              if (phase == 1)
                num11 = 9999;
              if (phase == 2)
                num11 = 1;
              if ( self.game.Data.RuleVar[253] == 0.0)
                num11 = 9999;
              let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
              for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
              {
                if (unr != self.TPlanObj[index1].HQ & self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr == plnr | self.TPlanObj[plnr].HQ == unr && numArray3[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] < num11 & !self.game.Data.UnitObj[unr].AIDisband && self.game.Data.UnitObj[unr].AIUnitGoal != 5 && !(self.game.Data.UnitObj[unr].IsHQ & self.TPlanObj[index1].Type != 30))
                {
                  num12: i32;
                  if (self.TPlanObj[plnr].Type != 30)
                  {
                    let mut num13: i32 =  (int) Math.Round(100.0 * ( simpleList1.Weight[index6] /  num2));
                    num3 = self.game.HandyFunctionsObj.GetUnitStackPts(unr);
                    if (num3 < 1)
                      num3 = 1;
                    if (!self.game.Data.UnitObj[unr].IsHQ &  num3 >  self.game.Data.RuleVar[247])
                      num13 = (int) Math.Round( ( num13 * (self.game.Data.RuleVar[247] /  num3)));
                    if (self.game.Data.UnitObj[unr].IsHQ & self.game.HandyFunctionsObj.GetStaffPercent(unr) < 100)
                      num13 *= 3;
                    if (!self.game.Data.UnitObj[unr].IsHQ &  num3 <  self.game.Data.RuleVar[182])
                      num13 *= 3;
                    if (0 > num13)
                      num13 = 0;
                    num12 = num13 + 1;
                  }
                  else
                    num12 = 1;
                  num10 += num12;
                }
              }
              if (num10 > 0)
              {
                let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
                for (let mut index7: i32 =  0; index7 <= unitCounter2; index7 += 1)
                {
                  if (index7 != self.TPlanObj[index1].HQ & self.game.Data.UnitObj[index7].X > -1 & self.game.Data.UnitObj[index7].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index7].AIPlanNr == plnr | self.TPlanObj[plnr].HQ == index7 && numArray3[self.game.Data.UnitObj[index7].X, self.game.Data.UnitObj[index7].Y] < num11 & !self.game.Data.UnitObj[index7].AIDisband && !(self.game.Data.UnitObj[index7].IsHQ & self.TPlanObj[index1].Type != 30))
                  {
                    num14: i32;
                    if (self.TPlanObj[plnr].Type != 30)
                    {
                      let mut num15: i32 =  (int) Math.Round(100.0 * ( simpleList1.Weight[index6] /  num2));
                      num3 = self.game.HandyFunctionsObj.GetUnitStackPts(index7);
                      if (num3 < 1)
                        num3 = 1;
                      if (!self.game.Data.UnitObj[index7].IsHQ &  num3 >  self.game.Data.RuleVar[247])
                        num15 = (int) Math.Round( ( num15 * (self.game.Data.RuleVar[247] /  num3)));
                      if (self.game.Data.UnitObj[index7].IsHQ & self.game.HandyFunctionsObj.GetStaffPercent(index7) < 100)
                        num15 *= 3;
                      if (!self.game.Data.UnitObj[index7].IsHQ &  num3 <  self.game.Data.RuleVar[182])
                        num15 *= 3;
                      if (0 > num15)
                        num15 = 0;
                      num14 = num15 + 1;
                    }
                    else
                      num14 = 1;
                    let mut num16: i32 =  num14;
                    if (self.game.Data.UnitObj[index7].AIUnitGoal == 4)
                      num16 *= self.PlanEngineerNeedScore(plnr);
                    if (self.game.Data.UnitObj[index7].AIUnitGoal != 5)
                    {
                      simpleList2.Add(index7, num16);
                      self.AddLog(self.game.Data.UnitObj[index7].Name + " => gets weight= " + Conversion.Str( num16));
                    }
                  }
                }
              }
            }
            let mut num17: i32 =  (int) Math.Round( num10 / 2.0);
            if (phase == 1)
              self.TPlanObj[index1].LandTransferMobility = 0;
            let mut landCap: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap;
            let mut num18: i32 =  0;
            if (simpleList2.Counter > -1)
            {
              let mut powerPtsAbsolute: i32 =  self.game.HandyFunctionsObj.GetPowerPtsAbsolute(self.TPlanObj[index1].HQ);
              let mut Number1: i32 =  1;
              num19: i32;
              do
              {
                let mut Number2: i32 =  0;
                let mut Number3: i32 =  0;
                num3 = 0;
                let mut num20: i32 =  0;
                let mut num21: i32 =  0;
                let mut num22: i32 =  0;
                num19 = 0;
                if (Number1 == 1)
                {
                  Number2 = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap;
                  Number3 = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].NavyCap;
                }
                if ( self.game.Data.RuleVar[253] == 0.0)
                {
                  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap = 99999;
                  Number2 = self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap;
                }
                self.AddLog("ROLECYCLE=" + Conversion.Str( Number1) + " , capleftland=" + Conversion.Str( Number2) + ", capleftnavy=" + Conversion.Str( Number3));
                for (; num20 == 0 & num21 < 4999 &&  self.game.HandyFunctionsObj.GetPowerPtsAbsolute(self.TPlanObj[index1].HQ) >=  powerPtsAbsolute * 0.9; num21 += 1)
                {
                  for (let mut counter2: i32 =  simpleList2.Counter; counter2 >= 0; counter2 += -1)
                  {
                    if ( VBMath.Rnd() *  num17 <  simpleList2.Weight[counter2] && self.game.Data.UnitObj[simpleList2.Id[counter2]].AIUnitGoal != 5)
                    {
                      RoleSFResult roleSfResult = self.LandTransferWhatWantsUnit(simpleList2.Id[counter2], 1, self.TPlanObj[index1].HQ, onlyrole: true);
                      let mut rolenr: i32 =  roleSfResult.rolenr;
                      if (rolenr > -1)
                      {
                        let mut SfNr: i32 =  self.LandTransferGetSF(self.TPlanObj[index1].HQ, rolenr, roleSfResult.sftypenr);
                        if (rolenr == 9 & phase == 2)
                          SfNr = -1;
                        if (phase == 1 && rolenr == 9 & SfNr > -1 && self.game.Data.SFTypeObj[self.game.Data.SFObj[SfNr].Type].AIRoleScore[2] > 0 &&  num18 >=  landCap * 0.33)
                          SfNr = -1;
                        if (SfNr == -1)
                        {
                          roleSfResult = self.LandTransferWhatWantsUnit(simpleList2.Id[counter2], 2, self.TPlanObj[index1].HQ, onlyrole: true);
                          rolenr = roleSfResult.rolenr;
                          if (rolenr > -1)
                            SfNr = self.LandTransferGetSF(self.TPlanObj[index1].HQ, rolenr, roleSfResult.sftypenr);
                          if (rolenr == 9 & phase == 2)
                            SfNr = -1;
                        }
                        if (phase == 1 && rolenr == 9 & SfNr > -1 && self.game.Data.SFTypeObj[self.game.Data.SFObj[SfNr].Type].AIRoleScore[2] > 0 &&  num18 >=  landCap * 0.33)
                          SfNr = -1;
                        if (SfNr > -1)
                        {
                          let mut type: i32 =  self.game.Data.SFObj[SfNr].Type;
                          let mut num23: i32 =  self.game.Data.SFTypeObj[type].Weight * numArray3[self.game.Data.UnitObj[simpleList2.Id[counter2]].X, self.game.Data.UnitObj[simpleList2.Id[counter2]].Y];
                          if ( self.game.Data.RuleVar[253] == 0.0)
                            num23 = 0;
                          let mut num24: i32 =  self.game.Data.SFTypeObj[type].Theater != 0 ? 0 : self.game.Data.SFTypeObj[type].Cap;
                          let mut num25: i32 =  self.game.Data.SFTypeObj[type].Theater != 1 ? 0 : self.game.Data.SFTypeObj[type].Cap;
                          if ( self.game.Data.RuleVar[253] == 0.0 | phase == 2 | Number2 - num24 >= 0 & Number3 - num25 >= 0)
                          {
                            if (!self.game.Data.UnitObj[simpleList2.Id[counter2]].IsHQ && self.game.HandyFunctionsObj.GetUnitSFNr(simpleList2.Id[counter2], type, self.game.Data.SFObj[SfNr].People) == -1 && self.game.Data.UnitObj[simpleList2.Id[counter2]].SFCount > 6)
                              num23 = 9999;
                            if (!self.game.Data.UnitObj[simpleList2.Id[counter2]].IsHQ && self.game.Data.SFTypeObj[type].Theater == 0 & self.game.HandyFunctionsObj.HasUnitNavySF(simpleList2.Id[counter2]))
                              num23 = 99999;
                            if (num23 < 99999)
                            {
                              if (num23 <= self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap | phase == 2)
                              {
                                let mut num26: i32 =  1;
                                self.game.ProcessingObj.DoTransfer(self.TPlanObj[index1].HQ, simpleList2.Id[counter2], 0, SfNr, num26, AddtoHistory: false);
                                self.AddLog("Transfered " + Conversion.Str( num26) + "x " + self.game.Data.SFTypeObj[type].Name + " to " + self.game.Data.UnitObj[simpleList2.Id[counter2]].Name);
                                Number2 -= num24;
                                Number3 -= num25;
                                if ( self.game.Data.RuleVar[253] == 0.0)
                                {
                                  Number2 = 99999;
                                  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap = 99999;
                                }
                                if (phase == 1 && rolenr == 9 & SfNr > -1)
                                  num18 += self.game.Data.SFTypeObj[type].Cap * num26;
                                num3 = 1;
                              }
                              else
                                num19 = 1;
                            }
                          }
                        }
                        else
                        {
                          num17 -= simpleList2.Weight[counter2];
                          simpleList2.Remove(simpleList2.Id[counter2]);
                          if (0 > num17)
                            num17 = 0;
                        }
                      }
                    }
                  }
                  if (num3 == 0)
                  {
                    num22 += 1;
                  }
                  else
                  {
                    num22 = 0;
                    num3 = 0;
                  }
                  if (num22 > (simpleList2.Counter + 1) * 10)
                    num20 = 1;
                }
                Application.DoEvents();
                Number1 += 1;
              }
              while (Number1 <= 1);
              if (phase == 1 & Number1 == 1)
                self.TPlanObj[index1].LandTransferMobility = num19;
            }
          }
        }
      }
    }

    pub fn ExecAirTransfers(phase: i32)
    {
      let mut tplanCount1: i32 =  self.TPlanCount;
      num1: i32;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        if (self.TPlanObj[index].Type == 20)
          num1 = (int) Math.Round( ( num1 + self.TPlanObj[index].WeightEnemyForce));
      }
      if (num1 == 0)
        num1 = 1;
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount2; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 30 && self.TPlanObj[index1].HQ > -1)
        {
          self.AddLog("");
          self.AddLog("*AIRTransferExec for " + self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Name + " PHASE " + Conversion.Str( phase));
          let mut x: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X;
          let mut y: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y;
          let mut increaseap: i32 =  self.game.HandyFunctionsObj.GetLowestAirAp(self.TPlanObj[index1].HQ);
          if (increaseap < 100)
            increaseap = 100 - increaseap;
          if (0 > increaseap)
            increaseap = 0;
          if (increaseap > 100)
            increaseap = 100;
          self.game.HandyFunctionsObj.MakeMovePrediction(self.TPlanObj[index1].HQ, x, y, 0, false, PredictAirOnly: true, increaseap: increaseap, IsTransfer: true);
          SimpleList simpleList1 = SimpleList::new();
          let mut num2: i32 =  0;
          let mut tplanCount3: i32 =  self.TPlanCount;
          num3: i32;
          for (let mut index2: i32 =  1; index2 <= tplanCount3; index2 += 1)
          {
            if (phase == 1 & (self.TPlanObj[index2].Type == 20 | self.TPlanObj[index2].Type == 40) | self.TPlanObj[index2].Type == 20 & phase == 2)
            {
              if ( self.game.EditObj.TempValue[0].Value[self.TPlanObj[index2].FromArea.X, self.TPlanObj[index2].FromArea.Y] <  self.game.Data.RuleVar[78] | phase == 2)
              {
                let mut num4: i32 =  0;
                let mut num5: i32 =  0;
                if (self.TPlanObj[index2].Type == 20)
                {
                  if ( self.TPlanObj[index2].WeightEnemyForce >  self.TPlanObj[index2].WeightFriendlyForce)
                  {
                    let mut num6: i32 =  (int) Math.Round(100.0 * ( self.TPlanObj[index2].WeightEnemyForce / ( self.TPlanObj[index2].WeightFriendlyForce + 1.0)));
                    num5 = (int) Math.Round(100.0 * ( self.TPlanObj[index2].WeightEnemyForce / ( self.TPlanObj[index2].WeightFriendlyForce + 1.0))) + self.TPlanObj[index1].WeightStrategic;
                    num3 = (int) Math.Round( ( (num6 * (self.TPlanObj[index2].WeightStrategic + 1)) * (self.TPlanObj[index2].WeightEnemyForce /  num1)));
                  }
                  else
                  {
                    num5 = (int) Math.Round(100.0 * ( self.TPlanObj[index2].WeightEnemyForce / ( self.TPlanObj[index2].WeightFriendlyForce + 1.0))) + self.TPlanObj[index1].WeightStrategic;
                    num3 = (int) Math.Round( ( ((int) Math.Round(100.0 * ( self.TPlanObj[index2].WeightEnemyForce / ( self.TPlanObj[index2].WeightFriendlyForce + 1.0))) * (self.TPlanObj[index2].WeightStrategic + 1)) * (self.TPlanObj[index2].WeightEnemyForce /  num1)));
                    if (num3 == 0)
                      num3 = 10 * (self.TPlanObj[index2].WeightStrategic + 1);
                  }
                }
                else if (self.TPlanObj[index2].Type == 40)
                {
                  num3 = self.TPlanObj[index2].WeightStrategic * 5;
                  if (num3 == 0)
                    num3 = 1;
                  num5 = num3;
                }
                if (num3 > 0)
                  num4 += num3;
                if (num4 > 0)
                {
                  self.AddLog("Plan #" + Conversion.Str( index2) + " importance=" + Conversion.Str( num5) + ", weight=" + Conversion.Str( num4));
                  simpleList1.Add(index2, num4, num5);
                  num2 += num4;
                }
              }
              else
                self.AddLog("Plan #" + Conversion.Str( index2) + " IS OUT OF RANGE");
            }
          }
          let mut num7: i32 =  0;
          if (phase == 1)
          {
            let mut tplanCount4: i32 =  self.TPlanCount;
            for (let mut index3: i32 =  1; index3 <= tplanCount4; index3 += 1)
            {
              if (self.TPlanObj[index3].Type == 30 && index3 != index1 && self.TPlanObj[index1].WeightStrategic < self.TPlanObj[index3].WeightStrategic &&  self.game.EditObj.TempValue[0].Value[self.TPlanObj[index3].FromArea.X, self.TPlanObj[index3].FromArea.Y] <  self.game.Data.RuleVar[78])
              {
                let mut num8: i32 =  self.TPlanObj[index3].WeightStrategic - self.TPlanObj[index1].WeightStrategic;
                let mut num9: i32 =  self.TPlanObj[index3].WeightStrategic - self.TPlanObj[index1].WeightStrategic;
                if (num9 > 0)
                {
                  simpleList1.Add(index3, num9, num8);
                  self.AddLog("Plan #" + Conversion.Str( index3) + " importance=" + Conversion.Str( num8) + ", weight=" + Conversion.Str( num9));
                  num2 += num9;
                  num7 += 1;
                  num3 = 1;
                }
              }
            }
          }
          self.AddLog("=>toUnits");
          if (simpleList1.Counter > -1)
          {
            SimpleList simpleList2 = SimpleList::new();
            let mut counter1: i32 =  simpleList1.Counter;
            num10: i32;
            for (let mut index4: i32 =  0; index4 <= counter1; index4 += 1)
            {
              let mut plnr: i32 =  simpleList1.Id[index4];
              num10 = 0;
              num11: i32;
              if (phase == 1)
                num11 = 9999;
              if (phase == 2)
                num11 = 1;
              let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
              for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
              {
                if (unr != self.TPlanObj[index1].HQ & self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr == plnr | self.TPlanObj[plnr].HQ == unr | phase == 2 && self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] < num11 && self.game.Data.UnitObj[unr].AIUnitGoal == 5 &&  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y) >= 1.0)
                {
                  num12: i32;
                  if (self.TPlanObj[plnr].Type != 30)
                  {
                    num12 = (int) Math.Round( (self.game.Data.RuleVar[31] -  self.game.HandyFunctionsObj.GetUnitStackPts(unr)));
                    if (self.game.HandyFunctionsObj.GetUnitStackPts(unr) < 10)
                      num12 = (int) Math.Round( num12 * (1.0 +  (10 - self.game.HandyFunctionsObj.GetUnitStackPts(unr)) / 5.0));
                    if (0 > num12)
                      num12 = 0;
                  }
                  else
                    num12 = 1;
                  num10 += num12;
                }
              }
              if (num10 > 0)
              {
                let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
                for (let mut index5: i32 =  0; index5 <= unitCounter2; index5 += 1)
                {
                  if (index5 != self.TPlanObj[index1].HQ & self.game.Data.UnitObj[index5].X > -1 & self.game.Data.UnitObj[index5].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index5].AIPlanNr == plnr | self.TPlanObj[plnr].HQ == index5 | phase == 2 && self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index5].X, self.game.Data.UnitObj[index5].Y] < num11)
                  {
                    num13: i32;
                    if (self.TPlanObj[plnr].Type != 30)
                    {
                      num13 = (int) Math.Round( (self.game.Data.RuleVar[31] -  self.game.HandyFunctionsObj.GetUnitStackPts(index5)));
                      if (self.game.HandyFunctionsObj.GetUnitStackPts(index5) < 10)
                        num13 = (int) Math.Round( num13 * (1.0 +  (10 - self.game.HandyFunctionsObj.GetUnitStackPts(index5)) / 5.0));
                      if (0 > num13)
                        num13 = 0;
                    }
                    else
                      num13 = 1;
                    let mut num14: i32 =  (int) Math.Round(Conversion.Int( num13 * ( simpleList1.Data1[index4] /  num10)));
                    if (self.game.Data.UnitObj[index5].AIUnitGoal == 4)
                      num14 *= self.PlanEngineerNeedScore(plnr);
                    if (self.game.Data.UnitObj[index5].AIUnitGoal == 5 &&  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[index5].X, self.game.Data.UnitObj[index5].Y) >= 1.0)
                    {
                      simpleList2.Add(index5, num14);
                      self.AddLog(self.game.Data.UnitObj[index5].Name + " => gets weight= " + Conversion.Str( num14));
                    }
                  }
                }
              }
            }
            let mut num15: i32 =  0;
            num3 = 0;
            let mut num16: i32 =  0;
            let mut num17: i32 =  0;
            if (phase == 1)
              self.TPlanObj[index1].LandTransferMobility = 0;
            if (simpleList2.Counter > -1)
            {
              for (; num16 == 0 & num7 < 1999; num7 += 1)
              {
                let mut counter2: i32 =  simpleList2.Counter;
                for (let mut index6: i32 =  0; index6 <= counter2; index6 += 1)
                {
                  if (self.game.Data.UnitObj[simpleList2.Id[index6]].AIUnitGoal == 5 &&  VBMath.Rnd() *  num10 <  simpleList2.Weight[index6])
                  {
                    RoleSFResult roleSfResult = self.LandTransferWhatWantsUnit(simpleList2.Id[index6], 1, self.TPlanObj[index1].HQ, onlyrole: true);
                    let mut rolenr1: i32 =  roleSfResult.rolenr;
                    if (rolenr1 > -1 & (rolenr1 == 13 | rolenr1 == 14))
                    {
                      let mut sf: i32 =  self.LandTransferGetSF(self.TPlanObj[index1].HQ, rolenr1, roleSfResult.sftypenr);
                      if (sf == -1)
                      {
                        roleSfResult = self.LandTransferWhatWantsUnit(simpleList2.Id[index6], 2, self.TPlanObj[index1].HQ, onlyrole: true);
                        let mut rolenr2: i32 =  roleSfResult.rolenr;
                        sf = self.LandTransferGetSF(self.TPlanObj[index1].HQ, rolenr2, roleSfResult.sftypenr);
                      }
                      if (sf > -1)
                      {
                        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
                        let mut num18: i32 =  self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[simpleList2.Id[index6]].X, self.game.Data.UnitObj[simpleList2.Id[index6]].Y];
                        if (!self.game.Data.UnitObj[simpleList2.Id[index6]].IsHQ && self.game.HandyFunctionsObj.GetUnitSFNr(simpleList2.Id[index6], type, self.game.Data.SFObj[sf].People) == -1 && self.game.Data.UnitObj[simpleList2.Id[index6]].SFCount > 6)
                          num18 = 9999;
                        if (num18 < 9999)
                        {
                          self.game.ProcessingObj.DoTransfer(self.TPlanObj[index1].HQ, simpleList2.Id[index6], 2, sf, 1, true, false);
                          self.AddLog("Transfered 1x " + self.game.Data.SFTypeObj[type].Name + " to " + self.game.Data.UnitObj[simpleList2.Id[index6]].Name);
                          num3 = 1;
                        }
                        else
                          num17 = 1;
                      }
                    }
                  }
                }
                if (num3 == 0)
                {
                  num15 += 1;
                }
                else
                {
                  num15 = 0;
                  num3 = 0;
                }
                if (num15 > 20)
                  num16 = 1;
              }
              if (phase == 1)
                self.TPlanObj[index1].LandTransferMobility = num17;
            }
          }
        }
      }
    }

    pub fn ExecNavyTransfers(phase: i32)
    {
      let mut tplanCount1: i32 =  self.TPlanCount;
      num1: i32;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        if (self.TPlanObj[index].Type == 20)
          num1 = (int) Math.Round( ( num1 + self.TPlanObj[index].WeightEnemyForce));
      }
      if (num1 == 0)
        ;
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount2; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 30 && self.TPlanObj[index1].HQ > -1)
        {
          self.AddLog("");
          self.AddLog("*NavyTransferExec for " + self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Name + " PHASE " + Conversion.Str( phase));
          let mut x: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X;
          let mut y: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y;
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[1]), 1, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0, false, istransfer: true);
          SimpleList simpleList1 = SimpleList::new();
          let mut num2: i32 =  0;
          let mut num3: i32 =  0;
          if (phase == 1)
          {
            let mut tplanCount3: i32 =  self.TPlanCount;
            for (let mut index2: i32 =  1; index2 <= tplanCount3; index2 += 1)
            {
              if (self.TPlanObj[index2].Type == 30 && index2 != index1 && self.TPlanObj[index1].WeightStrategic < self.TPlanObj[index2].WeightStrategic &&  self.game.EditObj.TempValue[0].Value[self.TPlanObj[index2].FromArea.X, self.TPlanObj[index2].FromArea.Y] <  self.game.Data.RuleVar[78])
              {
                let mut num4: i32 =  self.TPlanObj[index2].WeightStrategic - self.TPlanObj[index1].WeightStrategic;
                let mut num5: i32 =  self.TPlanObj[index2].WeightStrategic - self.TPlanObj[index1].WeightStrategic;
                if (num5 > 0)
                {
                  simpleList1.Add(index2, num5, num4);
                  self.AddLog("Plan #" + Conversion.Str( index2) + " importance=" + Conversion.Str( num4) + ", weight=" + Conversion.Str( num5));
                  num2 += num5;
                  num3 += 1;
                }
              }
            }
          }
          self.AddLog("=>toUnits");
          if (simpleList1.Counter > -1)
          {
            SimpleList simpleList2 = SimpleList::new();
            let mut counter1: i32 =  simpleList1.Counter;
            num6: i32;
            for (let mut index3: i32 =  0; index3 <= counter1; index3 += 1)
            {
              let mut index4: i32 =  simpleList1.Id[index3];
              num6 = 0;
              num7: i32;
              if (phase == 1)
                num7 = 9999;
              if (phase == 2)
                num7 = 1;
              let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
              for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
              {
                if (unr != self.TPlanObj[index1].HQ & self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr == index4 | self.TPlanObj[index4].HQ == unr | phase == 2 && self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] < num7)
                {
                  num8: i32;
                  if (self.TPlanObj[index4].Type != 30)
                  {
                    num8 = (int) Math.Round( (self.game.Data.RuleVar[31] -  self.game.HandyFunctionsObj.GetUnitStackPts(unr)));
                    if (self.game.HandyFunctionsObj.GetUnitStackPts(unr) < 10)
                      num8 = (int) Math.Round( num8 * (1.0 +  (10 - self.game.HandyFunctionsObj.GetUnitStackPts(unr)) / 5.0));
                    if (0 > num8)
                      num8 = 0;
                  }
                  else
                    num8 = 1;
                  num6 += num8;
                }
              }
              if (num6 > 0)
              {
                let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
                for (let mut index5: i32 =  0; index5 <= unitCounter2; index5 += 1)
                {
                  if (index5 != self.TPlanObj[index1].HQ & self.game.Data.UnitObj[index5].X > -1 & self.game.Data.UnitObj[index5].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index5].AIPlanNr == index4 | self.TPlanObj[index4].HQ == index5 | phase == 2 && self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index5].X, self.game.Data.UnitObj[index5].Y] < num7)
                  {
                    num9: i32;
                    if (self.TPlanObj[index4].Type != 30)
                    {
                      num9 = (int) Math.Round( (self.game.Data.RuleVar[31] -  self.game.HandyFunctionsObj.GetUnitStackPts(index5)));
                      if (self.game.HandyFunctionsObj.GetUnitStackPts(index5) < 10)
                        num9 = (int) Math.Round( num9 * (1.0 +  (10 - self.game.HandyFunctionsObj.GetUnitStackPts(index5)) / 5.0));
                      if (0 > num9)
                        num9 = 0;
                    }
                    else
                      num9 = 1;
                    let mut num10: i32 =  (int) Math.Round(Conversion.Int( num9 * ( simpleList1.Data1[index3] /  num6)));
                    simpleList2.Add(index5, num10);
                    self.AddLog(self.game.Data.UnitObj[index5].Name + " => gets weight= " + Conversion.Str( num10));
                  }
                }
              }
            }
            let mut num11: i32 =  0;
            let mut num12: i32 =  0;
            let mut num13: i32 =  0;
            let mut num14: i32 =  0;
            if (phase == 1)
              self.TPlanObj[index1].SeaTransferMobility = 0;
            let mut num15: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].NavyCap;
            let mut landCap: i32 =  self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap;
            let mut num16: i32 =  0;
            let mut num17: i32 =  0;
            if (simpleList2.Counter > -1)
            {
              for (; num13 == 0 & num3 < 1999; num3 += 1)
              {
                let mut counter2: i32 =  simpleList2.Counter;
                for (let mut index6: i32 =  0; index6 <= counter2; index6 += 1)
                {
                  if ( VBMath.Rnd() *  num6 <  simpleList2.Weight[index6])
                  {
                    RoleSFResult roleSfResult = self.LandTransferWhatWantsUnit(simpleList2.Id[index6], 1, self.TPlanObj[index1].HQ, onlyrole: true);
                    let mut rolenr1: i32 =  roleSfResult.rolenr;
                    if (rolenr1 > -1)
                    {
                      let mut sf: i32 =  self.LandTransferGetSF(self.TPlanObj[index1].HQ, rolenr1, roleSfResult.sftypenr);
                      if (sf == -1)
                      {
                        roleSfResult = self.LandTransferWhatWantsUnit(simpleList2.Id[index6], 2, self.TPlanObj[index1].HQ, onlyrole: true);
                        let mut rolenr2: i32 =  roleSfResult.rolenr;
                        sf = self.LandTransferGetSF(self.TPlanObj[index1].HQ, rolenr2, roleSfResult.sftypenr);
                      }
                      if (sf > -1)
                      {
                        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
                        let mut num18: i32 =  self.game.Data.SFTypeObj[type].Weight * self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[simpleList2.Id[index6]].X, self.game.Data.UnitObj[simpleList2.Id[index6]].Y];
                        if (!self.game.Data.UnitObj[simpleList2.Id[index6]].IsHQ && self.game.HandyFunctionsObj.GetUnitSFNr(simpleList2.Id[index6], type, self.game.Data.SFObj[sf].People) == -1 && self.game.Data.UnitObj[simpleList2.Id[index6]].SFCount > 6)
                          num18 = 9999;
                        if (self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[simpleList2.Id[index6]].X, self.game.Data.UnitObj[simpleList2.Id[index6]].Y] < 9999 && self.game.HandyFunctionsObj.IsHexPort(self.game.Data.UnitObj[self.TPlanObj[index1].HQ].X, self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Y, 0) && self.game.HandyFunctionsObj.IsHexPort(self.game.Data.UnitObj[simpleList2.Id[index6]].X, self.game.Data.UnitObj[simpleList2.Id[index6]].Y, 0) && self.game.Data.SFTypeObj[type].Theater == 1)
                          num18 = 0;
                        if (self.game.Data.SFTypeObj[type].AIRoleScore[3] > 0 && self.game.Data.UnitObj[self.TPlanObj[index1].HQ].NavyCap > self.game.Data.SFTypeObj[type].Cap)
                        {
                          let mut num19: i32 =  (int) Math.Round( (num15 - (num15 - self.game.Data.UnitObj[self.TPlanObj[index1].HQ].NavyCap)) / 2.0);
                          if (num19 > 0)
                          {
                            if (num16 + self.game.Data.SFTypeObj[type].Cap > num19)
                              num18 = 99999;
                            else
                              num16 += self.game.Data.SFTypeObj[type].Cap;
                          }
                        }
                        if (self.game.Data.SFTypeObj[type].AIRoleScore[17] > 0 && self.game.Data.SFObj[sf].Qty < 2)
                          num18 = 999999;
                        if (self.game.Data.SFTypeObj[type].AIRoleScore[2] > 0 && self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap > self.game.Data.SFTypeObj[type].Cap)
                        {
                          let mut num20: i32 =  (int) Math.Round( (landCap - (landCap - self.game.Data.UnitObj[self.TPlanObj[index1].HQ].LandCap)) / 2.0);
                          if (num20 > 0)
                          {
                            if (num17 + self.game.Data.SFTypeObj[type].Cap > num20)
                              num18 = 99999;
                            else
                              num17 += self.game.Data.SFTypeObj[type].Cap;
                          }
                        }
                        if (num18 < 9999)
                        {
                          if (num18 <= self.game.Data.UnitObj[self.TPlanObj[index1].HQ].NavyCap |  self.game.Data.RuleVar[253] == 0.0)
                          {
                            self.game.ProcessingObj.DoTransfer(self.TPlanObj[index1].HQ, simpleList2.Id[index6], 1, sf, 1, AddtoHistory: false);
                            self.AddLog("Transfered 1x " + self.game.Data.SFTypeObj[type].Name + " to " + self.game.Data.UnitObj[simpleList2.Id[index6]].Name);
                            if ( self.game.Data.RuleVar[253] == 0.0)
                            {
                              num15 = 99999;
                              self.game.Data.UnitObj[self.TPlanObj[index1].HQ].NavyCap = 99999;
                            }
                            num12 = 1;
                          }
                          else
                            num14 = 1;
                        }
                      }
                    }
                  }
                }
                if (num12 == 0)
                {
                  num11 += 1;
                }
                else
                {
                  num11 = 0;
                  num12 = 0;
                }
                if (num11 > 20)
                  num13 = 1;
              }
              if (phase == 1)
                self.TPlanObj[index1].SeaTransferMobility = num14;
            }
          }
        }
      }
    }

    pub RoleSFResult LandTransferWhatWantsUnit(
      unr: i32,
      info: i32,
      let mut hq: i32 =  -1,
      let mut prodpts: i32 =  -1,
      bool onlyrole = false)
    {
      RoleSFResult roleSfResult = RoleSFResult::new();
      let mut role: i32 =  -1;
      if (self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 30)
      {
        if (hq > -1)
        {
          if (self.LandTransferGetSF(hq, 6) > -1)
            role = 6;
          else if (self.LandTransferGetSF(hq, 10) > -1)
            role = 10;
          else if (self.LandTransferGetSF(hq, 8) > -1)
            role = 8;
          else if (self.LandTransferGetSF(hq, 7) > -1)
            role = 7;
          else if (self.LandTransferGetSF(hq, 5) > -1)
            role = 5;
          else if (self.LandTransferGetSF(hq, 1) > -1)
            role = 1;
          else if (self.LandTransferGetSF(hq, 18) > -1)
            role = 18;
          else if (self.LandTransferGetSF(hq, 19) > -1)
            role = 19;
          else if (self.LandTransferGetSF(hq, 17) > -1)
            role = 17;
          else if (self.LandTransferGetSF(hq, 9) > -1)
            role = 9;
          else if (self.LandTransferGetSF(hq, 2) > -1)
            role = 2;
        }
        else
        {
          if (info == 1)
            role = 6;
          if (info == 2)
            role = 10;
        }
        roleSfResult.rolenr = role;
        roleSfResult.sftypenr = -1;
        return roleSfResult;
      }
      if (self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 20 | self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 50 | self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 40)
      {
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 4)
        {
          if (info == 1)
            role =  self.game.Data.RuleVar[214] != 1.0 ? 5 : (!Operators.ConditionalCompareObjectGreaterEqual(self.GetEPPerTurn(unr),  self.game.Data.RuleVar[215], false) ? 5 : (!Operators.ConditionalCompareObjectLess( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false) ? -1 : 9));
          if (info == 2)
            role = 5;
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 1)
        {
          if (Conversions.ToBoolean(Operators.AndObject( self.game.Data.UnitObj[unr].AIMobilize, Operators.CompareObjectLessEqual( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))))
          {
            if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) >  self.game.Data.RuleVar[247])
            {
              if (info == 1)
                role = 9;
              if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) <  self.game.Data.RuleVar[182])
              {
                if (info == 2)
                  role = 6;
              }
              else if (info == 2)
                role = -1;
            }
            else
            {
              if (info == 1)
                role = 6;
              if (info == 2)
                role = 7;
            }
            if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.game.Data.RuleVar[258] <  self.GetFriendlyAirRatio() &  self.GetFriendlyAirRatio() < 1.0 &  self.game.Data.RuleVar[258] > 0.0)
            {
              if (self.GetRolePercent(unr, 12) < 10)
              {
                if (info == 1)
                  role = 12;
                if (info == 2)
                  role = 6;
              }
            }
            else if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258] && self.GetRolePercent(unr, 12) < 25)
            {
              if (info == 1)
                role = 12;
              if (info == 2)
                role = 6;
            }
          }
          else if (Conversions.ToBoolean(Operators.AndObject( (!self.game.Data.UnitObj[unr].AIMobilize & !self.game.Data.UnitObj[unr].AIReserve), Operators.CompareObjectLess( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitSpecialAIWeightWithoutLandCarryCap(unr), false))))
          {
            if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) >  self.game.Data.RuleVar[247])
            {
              if (info == 1)
                role = 9;
              if (info == 2)
                role = 6;
            }
            else
            {
              if (info == 1)
                role = 6;
              if (info == 2)
                role = 7;
            }
          }
          else if ( self.GetRolePercent(unr, 6) <  self.game.Data.RuleVar[156])
          {
            if (info == 1)
              role = 6;
            if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) <  self.game.Data.RuleVar[182])
            {
              if (info == 2)
                role = 6;
            }
            else if (info == 2)
              role = 7;
          }
          else
          {
            if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) <  self.game.Data.RuleVar[182])
            {
              if (info == 1)
                role = 6;
            }
            else if (info == 1)
              role = 7;
            if (info == 2)
              role = 6;
          }
          if (self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 40 && self.game.Data.UnitObj[unr].AIReserve)
          {
            if (self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true) > self.GetAbsolutePowerForReserveUnit(self.game.Data.UnitObj[unr].AIPlanNr))
              role = -1;
            if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.game.Data.RuleVar[258] <  self.GetFriendlyAirRatio() &  self.GetFriendlyAirRatio() < 1.0 &  self.game.Data.RuleVar[258] > 0.0)
            {
              if (self.GetRolePercent(unr, 12) < 20 && info == 1)
                role = 12;
            }
            else if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258] && self.GetRolePercent(unr, 12) < 40 && info == 1)
              role = 12;
          }
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 5)
        {
          let mut num: i32 =  1;
          if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258])
            num = 0;
          if (self.game.HandyFunctionsObj.GetPowerPtsAbsoluteForAirOnly(unr) > 0)
            num =  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y) >= 1.0 ? num : 0;
          if (num == 1)
          {
            if ( self.GetRolePercent(unr, 13) > 0.5)
            {
              if (info == 1)
                role = 14;
              if (info == 2)
                role = 13;
              if (self.game.Data.UnitObj[unr].AIPlanNr > 0)
              {
                let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
                if (self.TPlanObj[aiPlanNr].EnemyAir > self.TPlanObj[aiPlanNr].FriendlyAir)
                {
                  if (info == 1)
                    role = 13;
                  if (info == 2)
                    role = 14;
                }
              }
            }
            else
            {
              if (info == 1)
                role = 13;
              if (info == 2)
                role = 14;
            }
          }
          else
            role = -1;
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 9)
        {
          if (info == 1)
            role = 18;
          if (info == 2)
            role = 19;
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 10)
        {
          if (info == 1)
            role = 19;
          if (info == 2)
            role = 18;
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 8)
        {
          if (self.GetRolePercent(unr, 17) <= 0)
          {
            if (info == 1)
              role = 17;
            if (info == 2)
              role = 18;
          }
          else if ( self.GetRolePercent(unr, 18) > 0.5)
          {
            if (info == 1)
              role = 17;
            if (info == 2)
              role = 18;
          }
          else
          {
            if (info == 1)
              role = 18;
            if (info == 2)
              role = -1;
          }
          if ( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 1) >  self.game.Data.RuleVar[30] * 2.1)
          {
            if (info == 1)
              role = 18;
            if (info == 2)
              role = -1;
          }
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 2)
        {
          if (Conversions.ToBoolean(Operators.AndObject( self.game.Data.UnitObj[unr].AIMobilize, Operators.CompareObjectLessEqual( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))))
          {
            if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) >  self.game.Data.RuleVar[247])
            {
              if (info == 1)
                role = 9;
              if (info == 2)
                role = 10;
            }
            else
            {
              if (info == 1)
                role = 10;
              if (info == 2)
                role = 6;
            }
          }
          else if ( self.GetRolePercent(unr, 10) <  self.game.Data.RuleVar[157])
          {
            if (info == 1)
              role = 10;
            if (info == 2)
              role = 6;
          }
          else
          {
            if (info == 1)
              role = 6;
            if (info == 2)
              role = 10;
          }
          if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.game.Data.RuleVar[258] <  self.GetFriendlyAirRatio() &  self.GetFriendlyAirRatio() < 1.0 &  self.game.Data.RuleVar[258] > 0.0)
          {
            if (self.GetRolePercent(unr, 12) < 25)
            {
              if (info == 1)
                role = 12;
              if (info == 2)
                role = 10;
            }
          }
          else if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258] && self.GetRolePercent(unr, 12) < 25)
          {
            if (info == 1)
              role = 12;
            if (info == 2)
              role = 10;
          }
          if (self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 40 && self.game.Data.UnitObj[unr].AIReserve && self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true) > self.GetAbsolutePowerForReserveUnit(self.game.Data.UnitObj[unr].AIPlanNr))
            role = -1;
        }
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 3)
        {
          if (Conversions.ToBoolean(Operators.AndObject( self.game.Data.UnitObj[unr].AIMobilize, Operators.CompareObjectLess( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))))
          {
            if ( self.game.HandyFunctionsObj.GetUnitStackPts(unr) >  self.game.Data.RuleVar[247])
            {
              if (info == 1)
                role = 9;
              if (info == 2)
                role = 8;
            }
            else
            {
              if (info == 2)
                role = 8;
              if (info == 1)
                role = 6;
            }
            if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.game.Data.RuleVar[258] <  self.GetFriendlyAirRatio() &  self.GetFriendlyAirRatio() < 1.0 &  self.game.Data.RuleVar[258] > 0.0)
            {
              if (info == 1)
                role = 12;
              if (info == 2)
                role = 8;
            }
            else if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258])
            {
              if (info == 1)
                role = 12;
              if (info == 2)
                role = 8;
            }
          }
          else if ( self.GetRolePercent(unr, 8) <  self.game.Data.RuleVar[158])
          {
            if (info == 1)
              role = 8;
            if (info == 2)
              role = 6;
            if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.game.Data.RuleVar[258] <  self.GetFriendlyAirRatio() &  self.GetFriendlyAirRatio() < 1.0 &  self.game.Data.RuleVar[258] > 0.0)
            {
              if (self.GetRolePercent(unr, 12) < 10)
              {
                if (info == 1)
                  role = 12;
                if (info == 2)
                  role = 8;
              }
            }
            else if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258] && self.GetRolePercent(unr, 12) < 25)
            {
              if (info == 1)
                role = 12;
              if (info == 2)
                role = 8;
            }
          }
          else
          {
            if (info == 2)
              role = 8;
            if (info == 1)
              role = 6;
          }
          if (self.TPlanObj[self.game.Data.UnitObj[unr].AIPlanNr].Type == 40 && self.game.Data.UnitObj[unr].AIReserve && self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true) > self.GetAbsolutePowerForReserveUnit(self.game.Data.UnitObj[unr].AIPlanNr))
            role = -1;
        }
        if (self.game.Data.UnitObj[unr].IsHQ)
        {
          if (self.game.HandyFunctionsObj.GetStaffPercent(unr) < 100)
          {
            if ( self.game.Data.RuleVar[(int) byte.MaxValue] < 1.0)
            {
              if (info == 1)
                role = 1;
              if (info == 2)
                role = -1;
            }
            else
            {
              if (info == 1)
                role = 6;
              if (info == 2)
                role = -1;
            }
          }
          else
          {
            if (Operators.ConditionalCompareObjectLess( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))
            {
              if (info == 1)
                role = 9;
              if (info == 2)
                role = -1;
            }
            else if ( self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true) <  self.game.Data.RuleVar[182])
            {
              if (info == 1)
                role = 6;
              if (info == 2)
                role = -1;
            }
            else
              role = -1;
            if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.game.Data.RuleVar[258] <  self.GetFriendlyAirRatio() &  self.GetFriendlyAirRatio() < 1.0 &  self.game.Data.RuleVar[258] > 0.0)
            {
              if (self.GetRolePercent(unr, 12) < 10)
              {
                if (info == 1)
                  role = 12;
                if (info == 2)
                  role = -1;
              }
            }
            else if ( self.game.Data.Round >  self.game.Data.RuleVar[259] &  self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258] && self.GetRolePercent(unr, 12) < 25)
            {
              if (info == 1)
                role = 12;
              if (info == 2)
                role = -1;
            }
          }
        }
        roleSfResult.rolenr = role;
        roleSfResult.itemtypenr = -1;
        roleSfResult.sftypenr = -1;
        if (!onlyrole)
        {
          if (role > -1 & role != 9)
          {
            roleSfResult.itemtypenr = self.FindBestSuitedItemType(unr, role, prodpts);
            roleSfResult.sftypenr = roleSfResult.itemtypenr <= -1 ? -1 : self.game.Data.ItemTypeObj[roleSfResult.itemtypenr].IsSFType;
          }
          else
          {
            roleSfResult.itemtypenr = -1;
            roleSfResult.sftypenr = -1;
          }
        }
        return roleSfResult;
      }
      roleSfResult.rolenr = -1;
      roleSfResult.sftypenr = -1;
      roleSfResult.itemtypenr = -1;
      return roleSfResult;
    }

    pub fn GetRolePercent(unr: i32, rolenr: i32) -> i32
    {
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      num1: i32;
      num2: i32;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
        num1 += self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].PowerPts;
        if (self.game.Data.SFTypeObj[type].AIRoleScore[rolenr] > 0)
          num2 = (int) Math.Round( num2 +  (self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].PowerPts) * ( self.game.Data.SFTypeObj[type].AIRoleScore[rolenr] / 100.0));
      }
      return num1 == 0 ? 0 : (int) Math.Round(Conversion.Int( (100 * num2) /  num1));
    }

    pub fn LandTransferGetSF(unr: i32, roletype: i32, let mut sftypenr: i32 =  -1) -> i32
    {
      let mut sf1: i32 =  -1;
      if (roletype == -1)
        return -1;
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf2: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 =  self.game.Data.SFObj[sf2].Type;
        let mut num1: i32 =  self.game.Data.SFTypeObj[type].AIRoleScore[roletype];
        if (type == sftypenr)
          num1 += 99999;
        if (roletype == 6 & self.game.Data.SFTypeObj[type].AIRoleScore[1] > num1)
          num1 = -1;
        num2: i32;
        if (num1 > num2)
        {
          num2 = num1;
          sf1 = sf2;
        }
      }
      return sf1;
    }

    pub fn ExecSendStaffUp()
    {
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut unr1: i32 =  0; unr1 <= unitCounter1; unr1 += 1)
      {
        if (self.game.Data.UnitObj[unr1].X > -1 & self.game.Data.UnitObj[unr1].PreDef == -1 && self.game.Data.UnitObj[unr1].Regime == self.game.Data.Turn)
        {
          if (!self.game.Data.UnitObj[unr1].IsHQ && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y].LandscapeType].IsSea)
          {
            for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr1].SFCount; sfCount >= 0; sfCount += -1)
            {
              let mut sf: i32 =  self.game.Data.UnitObj[unr1].SFList[sfCount];
              let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr1].SFList[sfCount]].Type;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[18] > 0 & self.game.Data.UnitObj[unr1].AIPlanNr > 0)
              {
                if (self.HexBackPlan[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y] < 1)
                {
                  let mut num: i32 =  self.HexPlan[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y];
                }
                if (self.TPlanObj[self.game.Data.UnitObj[unr1].AIPlanNr].SeaStand == 4 | self.TPlanObj[self.game.Data.UnitObj[unr1].AIPlanNr].SeaStand == 8)
                {
                  let mut x: i32 =  self.game.Data.UnitObj[unr1].X;
                  let mut y: i32 =  self.game.Data.UnitObj[unr1].Y;
                  let mut unitCounter2: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
                  for (let mut index: i32 =  0; index <= unitCounter2; index += 1)
                  {
                    let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitList[index];
                    if (unit != unr1 && self.game.Data.UnitObj[unit].AIUnitGoal == 9 | self.game.Data.UnitObj[unit].AIUnitGoal == 10)
                    {
                      let mut unr2: i32 =  unit;
                      let mut qty: i32 =  self.game.Data.SFObj[sf].Qty;
                      if (qty > 0)
                      {
                        self.game.HandyFunctionsObj.AddTroops3(unr2, type, self.game.Data.SFObj[sf].People, qty, self.game.Data.SFObj[sf].Xp, self.game.Data.SFObj[sf].Rdn, 0, self.game.Data.SFObj[sf].Mor, MoveType: self.game.Data.SFObj[sf].MoveType);
                        self.game.HandyFunctionsObj.RemoveTroops(unr1, type, self.game.Data.SFObj[sf].People, qty, self.game.Data.SFObj[sf].MoveType);
                      }
                    }
                  }
                }
              }
            }
          }
          if (!self.game.Data.UnitObj[unr1].IsHQ && Operators.ConditionalCompareObjectGreater(self.GetEPPerTurn(unr1),  self.game.Data.RuleVar[215], false))
          {
            for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr1].SFCount; sfCount >= 0; sfCount += -1)
            {
              let mut sf: i32 =  self.game.Data.UnitObj[unr1].SFList[sfCount];
              let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr1].SFList[sfCount]].Type;
              if (self.game.Data.SFTypeObj[type].EP > 0)
              {
                let mut landReservePlan: i32 =  self.SAObj[self.HexSA[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y]].LandReservePlan;
                if (landReservePlan > 0)
                {
                  let mut hq: i32 =  self.TPlanObj[landReservePlan].HQ;
                  if (hq > -1)
                  {
                    let mut num: i32 =  (int) Math.Round( self.game.Data.SFObj[sf].Qty / 3.0);
                    if (num > 0)
                    {
                      let mut x: i32 =  self.game.Data.UnitObj[unr1].X;
                      let mut y: i32 =  self.game.Data.UnitObj[unr1].Y;
                      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0);
                      if (self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] < 9999)
                      {
                        self.game.HandyFunctionsObj.AddTroops3(hq, type, self.game.Data.SFObj[sf].People, num, self.game.Data.SFObj[sf].Xp, self.game.Data.SFObj[sf].Rdn, 0, self.game.Data.SFObj[sf].Mor, MoveType: self.game.Data.SFObj[sf].MoveType);
                        self.game.HandyFunctionsObj.RemoveTroops(unr1, type, self.game.Data.SFObj[sf].People, num, self.game.Data.SFObj[sf].MoveType);
                      }
                    }
                  }
                }
              }
            }
          }
          if (!self.game.Data.UnitObj[unr1].IsHQ)
          {
            let mut integer: i32 =  Conversions.ToInteger(Operators.SubtractObject( self.game.HandyFunctionsObj.GetUnitCarryCap(unr1, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr1)));
            for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr1].SFCount; sfCount >= 0; sfCount += -1)
            {
              let mut sf: i32 =  self.game.Data.UnitObj[unr1].SFList[sfCount];
              let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr1].SFList[sfCount]].Type;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[9] > 0 & self.game.Data.UnitObj[unr1].AIPlanNr > 0 && self.HexSA[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y] > 0)
              {
                let mut landReservePlan: i32 =  self.SAObj[self.HexSA[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y]].LandReservePlan;
                integer -= self.game.Data.SFTypeObj[type].CarryCap;
                if (landReservePlan > 0 & integer > 0)
                {
                  let mut hq: i32 =  self.TPlanObj[landReservePlan].HQ;
                  if (hq > -1)
                  {
                    let mut num: i32 =  self.game.Data.SFObj[sf].Qty;
                    if (num > 1)
                      num = 1;
                    if (num > 0)
                    {
                      let mut x: i32 =  self.game.Data.UnitObj[unr1].X;
                      let mut y: i32 =  self.game.Data.UnitObj[unr1].Y;
                      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0);
                      if (self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] < 9999)
                      {
                        self.game.HandyFunctionsObj.AddTroops3(hq, type, self.game.Data.SFObj[sf].People, num, self.game.Data.SFObj[sf].Xp, self.game.Data.SFObj[sf].Rdn, 0, self.game.Data.SFObj[sf].Mor, MoveType: self.game.Data.SFObj[sf].MoveType);
                        self.game.HandyFunctionsObj.RemoveTroops(unr1, type, self.game.Data.SFObj[sf].People, num, self.game.Data.SFObj[sf].MoveType);
                      }
                    }
                  }
                }
              }
            }
          }
          if (!self.game.Data.UnitObj[unr1].IsHQ && self.game.Data.UnitObj[unr1].AIUnitGoal == 1 & !self.game.Data.UnitObj[unr1].AIReserve & !self.game.Data.UnitObj[unr1].AIMobilize)
          {
            for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr1].SFCount; sfCount >= 0; sfCount += -1)
            {
              let mut sf: i32 =  self.game.Data.UnitObj[unr1].SFList[sfCount];
              let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr1].SFList[sfCount]].Type;
              if (self.game.Data.SFTypeObj[type].AIRoleScore[12] > 0 & self.game.Data.UnitObj[unr1].AIPlanNr > 0)
              {
                let mut landReservePlan: i32 =  self.SAObj[self.HexSA[self.game.Data.UnitObj[unr1].X, self.game.Data.UnitObj[unr1].Y]].LandReservePlan;
                if (landReservePlan > 0)
                {
                  let mut hq: i32 =  self.TPlanObj[landReservePlan].HQ;
                  if (hq > -1)
                  {
                    let mut qty: i32 =  self.game.Data.SFObj[sf].Qty;
                    if (qty > 0)
                    {
                      let mut x: i32 =  self.game.Data.UnitObj[unr1].X;
                      let mut y: i32 =  self.game.Data.UnitObj[unr1].Y;
                      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0);
                      if (self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] < 9999)
                      {
                        self.game.HandyFunctionsObj.AddTroops3(hq, type, self.game.Data.SFObj[sf].People, qty, self.game.Data.SFObj[sf].Xp, self.game.Data.SFObj[sf].Rdn, 0, self.game.Data.SFObj[sf].Mor, MoveType: self.game.Data.SFObj[sf].MoveType);
                        self.game.HandyFunctionsObj.RemoveTroops(unr1, type, self.game.Data.SFObj[sf].People, qty, self.game.Data.SFObj[sf].MoveType);
                      }
                    }
                  }
                }
              }
            }
          }
          for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr1].SFCount; sfCount >= 0; sfCount += -1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[unr1].SFList[sfCount];
            let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr1].SFList[sfCount]].Type;
            if (self.game.Data.SFTypeObj[type].StaffPts > 0 && self.game.Data.UnitObj[unr1].HQ > -1)
            {
              let mut num1: i32 =  0;
              if (!self.game.Data.UnitObj[unr1].IsHQ)
                num1 = 1;
              if (self.game.HandyFunctionsObj.GetStaffPercent(unr1) > 150)
                num1 = 2;
              if (num1 > 0)
              {
                let mut num2: i32 =  self.game.Data.SFObj[sf].Qty;
                if (num1 == 2)
                  num2 = (int) Math.Round( num2 / 3.0);
                if (num2 > 0)
                {
                  let mut x: i32 =  self.game.Data.UnitObj[unr1].X;
                  let mut y: i32 =  self.game.Data.UnitObj[unr1].Y;
                  self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0);
                  if (self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[self.game.Data.UnitObj[unr1].HQ].X, self.game.Data.UnitObj[self.game.Data.UnitObj[unr1].HQ].Y] < 9999)
                  {
                    self.game.HandyFunctionsObj.AddTroops3(self.game.Data.UnitObj[unr1].HQ, type, self.game.Data.SFObj[sf].People, num2, self.game.Data.SFObj[sf].Xp, self.game.Data.SFObj[sf].Rdn, 0, self.game.Data.SFObj[sf].Mor, MoveType: self.game.Data.SFObj[sf].MoveType);
                    self.game.HandyFunctionsObj.RemoveTroops(unr1, type, self.game.Data.SFObj[sf].People, num2, self.game.Data.SFObj[sf].MoveType);
                  }
                }
              }
            }
            if (self.game.Data.SFTypeObj[type].Cap > 0 & self.game.Data.SFTypeObj[type].Theater == 0 && self.game.Data.UnitObj[unr1].HQ > -1)
            {
              let mut num3: i32 =  0;
              if (self.game.Data.UnitObj[self.game.Data.UnitObj[unr1].HQ].LandCap < self.game.Data.UnitObj[unr1].LandCap)
                num3 = 2;
              if (num3 > 0)
              {
                let mut num4: i32 =  (int) Math.Round( self.game.Data.SFObj[sf].Qty / 3.0);
                if (num4 > 0)
                {
                  let mut x: i32 =  self.game.Data.UnitObj[unr1].X;
                  let mut y: i32 =  self.game.Data.UnitObj[unr1].Y;
                  self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[78]), x, y, 0);
                  if (self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[self.game.Data.UnitObj[unr1].HQ].X, self.game.Data.UnitObj[self.game.Data.UnitObj[unr1].HQ].Y] < 9999)
                  {
                    self.game.HandyFunctionsObj.AddTroops3(self.game.Data.UnitObj[unr1].HQ, type, self.game.Data.SFObj[sf].People, num4, self.game.Data.SFObj[sf].Xp, self.game.Data.SFObj[sf].Rdn, 0, self.game.Data.SFObj[sf].Mor, MoveType: self.game.Data.SFObj[sf].MoveType);
                    self.game.HandyFunctionsObj.RemoveTroops(unr1, type, self.game.Data.SFObj[sf].People, num4, self.game.Data.SFObj[sf].MoveType);
                  }
                }
              }
            }
          }
        }
      }
    }

    pub fn ExecChangeHQ()
    {
      self.TempAvgUnits = new int[self.TPlanCount + 1];
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index: i32 =  0; index <= tplanCount1; index += 1)
        self.TempAvgUnits[index] = -1;
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 && self.game.Data.UnitObj[unr].X > -1 && self.game.Data.UnitObj[unr].IsHQ && self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].HQ == -1)
        {
          let mut num: i32 =  0;
          let mut tplanCount2: i32 =  self.TPlanCount;
          for (let mut index: i32 =  1; index <= tplanCount2; index += 1)
          {
            if (self.TPlanObj[index].HQ == unr)
              num = 1;
          }
          if (num == 0 & self.game.Data.UnitObj[unr].SFCount <= 8)
          {
            self.AddLog(self.game.Data.UnitObj[unr].Name + " has been degraded from hq to normal unit.");
            self.game.Data.UnitObj[unr].IsHQ = false;
            RegimeClass[] regimeObj1 = self.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray1 = regimeObj1;
            let mut turn1: i32 =  self.game.Data.Turn;
            let mut index1: i32 =  turn1;
            regimeClassArray1[index1].ResPts = (int) Math.Round( ( regimeObj1[turn1].ResPts + self.game.Data.RuleVar[47]));
            RegimeClass[] regimeObj2 = self.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray2 = regimeObj2;
            let mut turn2: i32 =  self.game.Data.Turn;
            let mut index2: i32 =  turn2;
            regimeClassArray2[index2].ResPts = (int) Math.Round( ( regimeObj2[turn2].ResPts - self.game.Data.RuleVar[46]));
            if (self.game.HandyFunctionsObj.HasUnitNavySF(unr))
            {
              let mut x: i32 =  self.game.Data.UnitObj[unr].X;
              let mut y: i32 =  self.game.Data.UnitObj[unr].Y;
              let mut hq: i32 =  self.game.Data.UnitObj[unr].HQ;
              if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = -1;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 8;
                for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount; sfCount >= 0; sfCount += -1)
                {
                  let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[sfCount];
                  if (self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 1)
                  {
                    self.game.Data.UnitObj[unr].RemoveSF(sf);
                    self.game.Data.UnitObj[self.game.Data.UnitCounter].AddSF(sf);
                  }
                }
              }
            }
            if (self.game.HandyFunctionsObj.HasUnitAirSF(unr))
            {
              let mut x: i32 =  self.game.Data.UnitObj[unr].X;
              let mut y: i32 =  self.game.Data.UnitObj[unr].Y;
              let mut hq: i32 =  self.game.Data.UnitObj[unr].HQ;
              if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
              {
                self.game.ProcessingObj.NewUnit(x, y, 0, false, self.game.Data.Turn);
                self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = hq;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = -1;
                self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 5;
                for (let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount; sfCount >= 0; sfCount += -1)
                {
                  let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[sfCount];
                  if (self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 2)
                  {
                    self.game.Data.UnitObj[unr].RemoveSF(sf);
                    self.game.Data.UnitObj[self.game.Data.UnitCounter].AddSF(sf);
                  }
                }
              }
            }
            let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
            for (let mut index3: i32 =  0; index3 <= unitCounter2; index3 += 1)
            {
              if (self.game.Data.UnitObj[index3].HQ == unr)
                self.game.Data.UnitObj[index3].HQ = -1;
            }
          }
        }
      }
      let mut tplanCount3: i32 =  self.TPlanCount;
      for (let mut index4: i32 =  1; index4 <= tplanCount3; index4 += 1)
      {
        self.AddLog("ChangeHQ for " + Conversion.Str( index4) + "?");
        SimpleList simpleList = SimpleList::new();
        let mut unitCounter3: i32 =  self.game.Data.UnitCounter;
        for (let mut tid: i32 =  0; tid <= unitCounter3; tid += 1)
        {
          if (self.game.Data.UnitObj[tid].AIPlanNr == index4 & self.game.Data.UnitObj[tid].IsHQ & self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn)
          {
            let mut num1: i32 =  (int) Math.Round( (self.game.Data.RuleVar[3] + Conversion.Int(self.game.Data.RuleVar[3] / 5f)));
            let mut tdata1: i32 =  self.AverageDistanceUnitsInAP(index4, self.game.Data.UnitObj[tid].X, self.game.Data.UnitObj[tid].Y, true) * self.AverageDistanceUnits(index4, self.game.Data.UnitObj[tid].X, self.game.Data.UnitObj[tid].Y);
            let mut num2: i32 =  num1 - tdata1;
            self.AddLog(self.game.Data.UnitObj[tid].Name + " gets weight = " + Conversion.Str( num2));
            simpleList.Add(tid, num2, tdata1);
          }
        }
        let mut tplanCount4: i32 =  self.TPlanCount;
        for (let mut index5: i32 =  1; index5 <= tplanCount4; index5 += 1)
        {
          if (index5 != index4)
          {
            if (self.GetAreaNr(self.TPlanObj[index5].FromArea) == self.GetAreaNr(self.TPlanObj[index4].FromArea) & self.TPlanObj[index5].HQ > -1)
            {
              let mut num3: i32 =  (int) Math.Round( self.game.Data.RuleVar[3]);
              if (self.TPlanObj[index5].Type == 20)
                num3 += 100;
              let mut tdata1: i32 =  self.AverageDistanceUnitsInAP(index4, self.game.Data.UnitObj[self.TPlanObj[index5].HQ].X, self.game.Data.UnitObj[self.TPlanObj[index5].HQ].Y, true);
              let mut num4: i32 =  num3 - tdata1;
              self.AddLog(self.game.Data.UnitObj[self.TPlanObj[index5].HQ].Name + " gets weight = " + Conversion.Str( num4));
              simpleList.Add(self.TPlanObj[index5].HQ, num4, tdata1);
            }
            else if (self.TPlanObj[index5].HQ > -1)
            {
              let mut num5: i32 =  self.AreaDistance2(self.GetAreaNr(self.TPlanObj[index5].FromArea), self.GetAreaNr(self.TPlanObj[index4].FromArea));
              let mut num6: i32 =  0;
              if (num5 <= 0)
              {
                num5 = self.AreaDistanceIncludingSea(self.GetAreaNr(self.TPlanObj[index5].FromArea), self.GetAreaNr(self.TPlanObj[index4].FromArea));
                num6 = 1;
              }
              if (num5 > 0)
              {
                let mut num7: i32 =  (int) Math.Round( (self.game.Data.RuleVar[3] -  (num5 * 10)));
                if (num6 == 1)
                  num7 -= 5;
                if (self.TPlanObj[index5].Type == 20)
                  num7 += 5;
                let mut tdata1: i32 =  self.AverageDistanceUnitsInAP(index4, self.game.Data.UnitObj[self.TPlanObj[index5].HQ].X, self.game.Data.UnitObj[self.TPlanObj[index5].HQ].Y, true);
                let mut num8: i32 =  num7 - tdata1;
                self.AddLog(self.game.Data.UnitObj[self.TPlanObj[index5].HQ].Name + " gets weight = " + Conversion.Str( num8));
                simpleList.Add(self.TPlanObj[index5].HQ, num8, tdata1);
              }
            }
          }
        }
        simpleList.Sort();
        if (simpleList.Counter > -1)
        {
          let mut unitCounter4: i32 =  self.game.Data.UnitCounter;
          for (let mut Unr: i32 =  1; Unr <= unitCounter4; Unr += 1)
          {
            if (self.game.Data.UnitObj[Unr].AIPlanNr == index4 && !self.game.Data.UnitObj[Unr].IsHQ && self.game.Data.UnitObj[Unr].HQ != simpleList.Id[simpleList.Counter] && self.game.Data.UnitObj[Unr].X > -1 & self.game.Data.UnitObj[Unr].Regime == self.game.Data.Turn && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[Unr].X, self.game.Data.UnitObj[Unr].Y].LandscapeType].IsSea)
            {
              self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[Unr].X, self.game.Data.UnitObj[Unr].Y, 0);
              let mut num9: i32 =  1;
              if (self.game.Data.UnitObj[Unr].HQ > -1)
              {
                let mut num10: i32 =  self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[self.game.Data.UnitObj[Unr].HQ].X, self.game.Data.UnitObj[self.game.Data.UnitObj[Unr].HQ].Y];
                let mut num11: i32 =  (int) Math.Round( ( self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[simpleList.Id[simpleList.Counter]].X, self.game.Data.UnitObj[simpleList.Id[simpleList.Counter]].Y] + Conversion.Int(self.game.Data.RuleVar[3] / 4f)));
                let mut num12: i32 =  self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[self.game.Data.UnitObj[Unr].HQ].X, self.game.Data.UnitObj[self.game.Data.UnitObj[Unr].HQ].Y, 0, self.game.Data.UnitObj[Unr].X, self.game.Data.UnitObj[Unr].Y, 0);
                let mut num13: i32 =  self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[simpleList.Id[simpleList.Counter]].X, self.game.Data.UnitObj[simpleList.Id[simpleList.Counter]].Y, 0, self.game.Data.UnitObj[Unr].X, self.game.Data.UnitObj[Unr].Y, 0);
                if (num10 < num11 & num13 + 2 > num12)
                  num9 = 0;
                if (self.game.Data.UnitObj[Unr].HQ > -1 && self.game.Data.UnitObj[self.game.Data.UnitObj[Unr].HQ].AIPlanNr > 0 && self.TPlanObj[self.game.Data.UnitObj[self.game.Data.UnitObj[Unr].HQ].AIPlanNr].Type == 30)
                  num9 = 1;
                if (num9 == 0)
                  self.AddLog("Will not switch " + self.game.Data.UnitObj[Unr].Name + " to best HQ. Since it is less than 50ap closer && less then 2 hex closer.");
              }
              if (num9 == 1)
                self.game.ProcessingObj.SetUnitHq(Unr, simpleList.Id[simpleList.Counter]);
            }
          }
        }
        if (self.TPlanObj[index4].HQ > -1 & self.TPlanObj[index4].Type == 20)
        {
          let mut areaNr: i32 =  self.GetAreaNr(self.TPlanObj[index4].FromArea);
          if (areaNr > 0 && self.SAObj[areaNr].LandReservePlan > 0)
          {
            let mut landReservePlan: i32 =  self.SAObj[areaNr].LandReservePlan;
            if (self.TPlanObj[landReservePlan].HQ > -1 && self.game.Data.UnitObj[self.TPlanObj[index4].HQ].HQ != self.TPlanObj[landReservePlan].HQ)
            {
              self.game.ProcessingObj.SetUnitHq(self.TPlanObj[index4].HQ, self.TPlanObj[landReservePlan].HQ);
              self.AddLog("-Changed " + self.game.Data.UnitObj[self.TPlanObj[index4].HQ].Name + " HQ to " + self.game.Data.UnitObj[self.TPlanObj[landReservePlan].HQ].Name);
            }
          }
        }
      }
    }

    pub fn ExecuteMovement(PlanNr: i32, MovePhaseNr: i32)
    {
      object[,] objArray1 = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      CoordList coordList = CoordList::new();
      SimpleList UL = SimpleList::new();
      SimpleList SL = SimpleList::new();
      if (PlanNr < 1)
        return;
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      Number1: i32;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        if (self.game.Data.UnitObj[index].AIPlanNr == PlanNr & self.game.Data.UnitObj[index].PreDef <= -1 && self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
        {
          UL.Add(index, self.GetForceLandStrength(index), -1);
          if (!(self.game.Data.UnitObj[index].IsHQ & self.TPlanObj[PlanNr].Type == 20) && !(self.game.Data.UnitObj[index].IsHQ & self.TPlanObj[PlanNr].Type == 30) && self.game.Data.UnitObj[index].AICutoff <= 0 && self.game.Data.UnitObj[index].AIUnitGoal != 4 && self.game.Data.UnitObj[index].AIUnitGoal != 5 && self.game.Data.UnitObj[index].AIUnitGoal != 3 && self.game.Data.UnitObj[index].AIUnitGoal != 9 && self.game.Data.UnitObj[index].AIUnitGoal != 10 && self.game.Data.UnitObj[index].AIUnitGoal != 8)
          {
            if (!self.game.Data.UnitObj[index].AIReserve)
            {
              if (self.game.Data.UnitObj[index].X != -1 && !self.game.Data.UnitObj[index].AIDisband)
                Number1 += 1;
            }
          }
        }
      }
      self.AddLog("");
      self.AddLog("Plan " + Conversion.Str( PlanNr) + " execution: .. av.units=" + Conversion.Str( Number1));
      tid1: i32;
      Coordinate Expression;
      if (Number1 > 0)
      {
        if (self.TPlanObj[PlanNr].Type == 20)
        {
          num1: i32;
          num2: i32;
          if (self.TPlanObj[PlanNr].Stand == 1 | self.TPlanObj[PlanNr].Stand == 2 & self.TPlanObj[PlanNr].RiverLine < 1)
          {
            tid1 = 0;
            let mut num3: i32 =  !Information.IsNothing( self.TPlanObj[PlanNr].TooArea) ? self.game.HandyFunctionsObj.Distance(self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 0, self.TPlanObj[PlanNr].TooArea.X, self.TPlanObj[PlanNr].TooArea.Y, 0) : 15;
            if (num3 < 10)
              num3 = 10;
            let mut MaxDist: i32 =  num3 * 2;
            self.SetMatrix1(self.TPlanObj[PlanNr].TooArea.X, self.TPlanObj[PlanNr].TooArea.Y, MaxDist: MaxDist);
            self.Matrix2 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
                self.Matrix2[index1, index2] = self.Matrix1[index1, index2];
            }
            let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
            {
              if (self.game.Data.UnitObj[unr].Regime != self.game.Data.Turn & self.game.Data.UnitObj[unr].PreDef <= -1)
              {
                let mut x: i32 =  self.game.Data.UnitObj[unr].X;
                let mut y: i32 =  self.game.Data.UnitObj[unr].Y;
                if (x > -1 && self.HexSA[x, y] == self.GetAreaNr(self.TPlanObj[PlanNr].TooArea))
                {
                  let mut nr: i32 =  SL.FindNr(-1, x, y);
                  let mut num4: i32 =  self.GetForceLandStrength(unr);
                  let mut num5: i32 =  (int) Math.Round(Conversion.Int( (self.GetClosestFrontlineDistance(self.GetAreaNr(self.TPlanObj[PlanNr].FromArea), x, y) + self.game.HandyFunctionsObj.Distance(x, y, 0, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 0)) / 2.0));
                  if (num5 < 1)
                    num5 = 1;
                  if (self.TPlanObj[PlanNr].Stand == 1)
                    num4 = (int) Math.Round( ( num4 * (1f + self.game.Data.RuleVar[225])));
                  let mut tweight: i32 =  (int) Math.Round( num4 /  num5);
                  if (nr == -1)
                  {
                    tid1 += 1;
                    let mut tid2: i32 =  tid1;
                    SL.Add(tid2, tweight, x, y, self.Matrix1[x, y]);
                  }
                  else
                  {
                    int[] weight = SL.Weight;
                    int[] numArray = weight;
                    let mut index3: i32 =  nr;
                    let mut index4: i32 =  index3;
                    let mut num6: i32 =  weight[index3] + self.GetForceLandStrength(unr);
                    numArray[index4] = num6;
                  }
                  num1 += tweight;
                  num2 += 1;
                }
              }
            }
          }
          num7: i32;
          if (self.TPlanObj[PlanNr].Stand == 1)
          {
            if (num2 == 0)
              num2 = 1;
            if (num1 == 0)
              num1 = 100;
            num7 = (int) Math.Round( num1 /  num2);
            let mut x: i32 =  self.TPlanObj[PlanNr].TooArea.X;
            let mut y: i32 =  self.TPlanObj[PlanNr].TooArea.Y;
            let mut tweight: i32 =  (int) Math.Round( ( (self.TPlanObj[PlanNr].TooArea.fuzzyvp * num7) * self.game.Data.RuleVar[151]));
            if (self.TPlanObj[PlanNr].TooArea.ConstitutantCount > 0)
              tweight = (int) Math.Round( tweight / 20.0);
            if ( self.game.Data.RuleVar[225] > 0.0)
              tweight = (int) Math.Round( tweight / Math.Pow( self.game.Data.RuleVar[225] + 1.0, 2.0));
            if (tweight < 0)
              tweight = 0;
            let mut nr: i32 =  SL.FindNr(-1, x, y);
            if (nr == -1)
            {
              tid1 += 1;
              SL.Add(tid1, tweight, x, y, (int) Math.Round( self.game.Data.RuleVar[152]));
            }
            else
            {
              int[] weight = SL.Weight;
              int[] numArray = weight;
              let mut index5: i32 =  nr;
              let mut index6: i32 =  index5;
              let mut num8: i32 =  weight[index5] + tweight;
              numArray[index6] = num8;
            }
          }
          if (self.TPlanObj[PlanNr].Stand == 2 & self.TPlanObj[PlanNr].RiverLine < 1 | self.TPlanObj[PlanNr].Stand == 1)
          {
            let mut num9: i32 =  !Information.IsNothing( self.TPlanObj[PlanNr].TooArea) ? self.game.HandyFunctionsObj.Distance(self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 0, self.TPlanObj[PlanNr].TooArea.X, self.TPlanObj[PlanNr].TooArea.Y, 0) : 15;
            if (num9 < 10)
              num9 = 10;
            let mut MaxDist: i32 =  num9 * 2;
            self.SetMatrix1(self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, MaxDist: MaxDist);
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index7: i32 =  0; index7 <= mapWidth; index7 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
              {
                if (self.HexPlan[index7, index8] == PlanNr)
                {
                  if (self.TPlanObj[PlanNr].Stand == 2)
                  {
                    let mut tweight: i32 =  (int) Math.Round(Conversion.Int( self.Matrix1[index7, index8] /  self.TPlanObj[PlanNr].FrontSize));
                    if (self.AIVP[index7, index8] > 0)
                      tweight *= 2;
                    if (tweight < 0)
                      tweight = 0;
                    let mut nr: i32 =  SL.FindNr(-1, index7, index8);
                    if (nr == -1)
                    {
                      tid1 += 1;
                      SL.Add(tid1, tweight, index7, index8, self.Matrix1[index7, index8]);
                    }
                    else
                    {
                      int[] weight = SL.Weight;
                      int[] numArray = weight;
                      let mut index9: i32 =  nr;
                      let mut index10: i32 =  index9;
                      let mut num10: i32 =  weight[index9] + tweight;
                      numArray[index10] = num10;
                    }
                  }
                  if (self.TPlanObj[PlanNr].Stand == 1)
                  {
                    object obj = self.BestMatrix2(index7, index8, 2);
                    Coordinate coordinate;
                    Expression = obj != null ? (Coordinate) obj : coordinate;
                    if (Expression.onmap)
                    {
                      let mut nr: i32 =  SL.FindNr(-1, Expression.x, Expression.y);
                      if (nr == -1)
                      {
                        tid1 += 1;
                        let mut tweight: i32 =  (int) Math.Round( num7 / 3.0);
                        if (self.AIVP[Expression.x, Expression.y] > 0)
                          tweight *= 2;
                        SL.Add(tid1, tweight, Expression.x, Expression.y, self.Matrix1[index7, index8]);
                      }
                      else
                      {
                        let mut num11: i32 =  (int) Math.Round( num7 / 3.0);
                        if (self.AIVP[Expression.x, Expression.y] > 0)
                          num11 *= 2;
                        int[] weight = SL.Weight;
                        int[] numArray = weight;
                        let mut index11: i32 =  nr;
                        let mut index12: i32 =  index11;
                        let mut num12: i32 =  weight[index11] + num11;
                        numArray[index12] = num12;
                      }
                    }
                    else
                    {
                      tid1 += 1;
                      let mut tweight: i32 =  (int) Math.Round( num7 / 3.0);
                      SL.Add(tid1, tweight, index7, index8, self.Matrix1[index7, index8]);
                    }
                  }
                }
              }
            }
          }
          if (self.TPlanObj[PlanNr].Stand == 2)
          {
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index13: i32 =  0; index13 <= mapWidth; index13 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
              {
                if (self.HexPlan[index13, index14] == PlanNr && self.TPlanObj[PlanNr].Stand == 2)
                {
                  let mut tweight: i32 =  (int) Math.Round( ( (int) Math.Round( self.Matrix1[index13, index14] /  (self.TPlanObj[PlanNr].FrontSize + 1)) * self.GetEntrenchMod(index13, index14)));
                  if (tweight < 0)
                    tweight = 0;
                  let mut nr: i32 =  SL.FindNr(-1, index13, index14);
                  if (nr == -1)
                  {
                    tid1 += 1;
                    SL.Add(tid1, tweight, index13, index14, self.Matrix1[index13, index14]);
                  }
                  else
                  {
                    int[] weight = SL.Weight;
                    int[] numArray = weight;
                    let mut index15: i32 =  nr;
                    let mut index16: i32 =  index15;
                    let mut num13: i32 =  weight[index15] + tweight;
                    numArray[index16] = num13;
                  }
                }
              }
            }
          }
          let mut num14: i32 =  1;
          if ( self.game.Data.RuleVar[251] == 0.0)
            num14 = 0;
          if (num14 == 1)
          {
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index17: i32 =  0; index17 <= mapWidth; index17 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index18: i32 =  0; index18 <= mapHeight; index18 += 1)
              {
                if (self.HexSA[index17, index18] == self.GetAreaNr(self.TPlanObj[PlanNr].FromArea))
                {
                  let mut d: i32 =  0;
                  let mut unitCounter3: i32 =  self.game.Data.MapObj[0].HexObj[index17, index18].UnitCounter;
                  for (let mut index19: i32 =  0; index19 <= unitCounter3; index19 += 1)
                  {
                    let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index17, index18].UnitList[index19];
                    d += self.game.HandyFunctionsObj.GetPowerPtsAbsoluteForAirOnly(unit);
                  }
                  if (d > 0)
                  {
                    let mut num15: i32 =  self.GetClosestEnemyDistance(index17, index18, true);
                    if (num15 < 1)
                      num15 = 1;
                    if (num15 > 10)
                      num15 = 10;
                    if (num15 <= 6)
                    {
                      let mut nr: i32 =  SL.FindNr(-1, index17, index18);
                      let mut tweight: i32 =  (int) Math.Round(Math.Sqrt( d) * 25.0 /  num15);
                      if (self.TPlanObj[PlanNr].Stand == 1)
                        tweight = (int) Math.Round( ( tweight * (self.game.Data.RuleVar[251] / 2f)));
                      if (nr == -1)
                      {
                        tid1 += 1;
                        SL.Add(tid1, tweight, index17, index18, self.Matrix1[index17, index18]);
                        SL.Data4[SL.Counter] = 2;
                      }
                      else
                      {
                        int[] weight = SL.Weight;
                        int[] numArray = weight;
                        let mut index20: i32 =  nr;
                        let mut index21: i32 =  index20;
                        let mut num16: i32 =  weight[index20] + tweight;
                        numArray[index21] = num16;
                      }
                    }
                  }
                }
              }
            }
          }
          if (self.TPlanObj[PlanNr].Stand == 2 | self.TPlanObj[PlanNr].Stand == 1 && self.AIVP[self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y] > 0)
          {
            let mut num17: i32 =  1;
            if (self.TPlanObj[PlanNr].Stand == 1 &&  self.game.Data.RuleVar[251] == 0.0)
              num17 = 0;
            if (num17 == 1)
            {
              if (num2 == 0)
                num2 = 1;
              if (num1 == 0)
                num1 = 100;
              let mut num18: i32 =  (int) Math.Round( num1 /  num2);
              if ( self.TPlanObj[PlanNr].WeightFriendlyForce > 0.0)
                num18 = (int) Math.Round( ( num18 * (self.TPlanObj[PlanNr].WeightEnemyForce / self.TPlanObj[PlanNr].WeightFriendlyForce)));
              let mut num19: i32 =  self.TPlanObj[PlanNr].Stand != 2 ? (int) Math.Round( num18 / 5.0) : (int) Math.Round( num18 / 2.0);
              let mut x: i32 =  self.TPlanObj[PlanNr].FromArea.X;
              let mut y: i32 =  self.TPlanObj[PlanNr].FromArea.Y;
              let mut tweight: i32 =  (int) Math.Round( ( (self.TPlanObj[PlanNr].FromArea.fuzzyvp * num19) * self.game.Data.RuleVar[151]));
              if (tweight < 0)
                tweight = 0;
              let mut nr: i32 =  SL.FindNr(-1, x, y);
              let mut frontlineDistance: i32 =  self.GetClosestFrontlineDistance(self.GetAreaNr(self.TPlanObj[PlanNr].TooArea), x, y, true);
              let mut tdata4: i32 =  0;
              num20: i32;
              if (frontlineDistance <= 1)
              {
                num20 = self.TPlanObj[PlanNr].Stand != 1 ? 2 : 1;
                tdata4 = 1;
              }
              else if (frontlineDistance <= 2)
              {
                num20 = 1;
                tdata4 = 1;
              }
              else if (frontlineDistance <= 3)
              {
                num20 = 1;
                tdata4 = 1;
                tweight = (int) Math.Round( tweight / 3.0);
              }
              else if (frontlineDistance <= 4)
              {
                num20 = 1;
                tweight = (int) Math.Round( tweight / 5.0);
              }
              else if (frontlineDistance <= 7)
              {
                num20 = 1;
                tweight = (int) Math.Round( tweight / 8.0);
              }
              else
              {
                num20 = 1;
                tweight = (int) Math.Round( tweight / 15.0);
              }
              if (self.TPlanObj[PlanNr].Stand == 1)
                tweight = (int) Math.Round( ( tweight * self.game.Data.RuleVar[251]));
              if (nr == -1)
              {
                if (num20 == 0)
                  num20 = 1;
                let mut num21: i32 =  num20;
                for (let mut index: i32 =  1; index <= num21; index += 1)
                {
                  tid1 += 1;
                  SL.Add(tid1, tweight, x, y, (int) Math.Round( self.game.Data.RuleVar[152]), tdata4);
                }
              }
              else
              {
                int[] weight = SL.Weight;
                int[] numArray = weight;
                let mut index22: i32 =  nr;
                let mut index23: i32 =  index22;
                let mut num22: i32 =  weight[index22] + tweight * num20;
                numArray[index23] = num22;
                SL.Data4[nr] = tdata4;
              }
            }
          }
          if (self.TPlanObj[PlanNr].Stand == 3)
          {
            let mut saCount: i32 =  self.SACount;
            for (let mut index: i32 =  1; index <= saCount; index += 1)
            {
              if (self.IsAreaNeighbour(index, self.GetAreaNr(self.TPlanObj[PlanNr].FromArea)) && self.GetFriendlyAreaNeighbours(index, false) == self.SAObj[index].NeighbourCount)
              {
                tid1 += 1;
                let mut x: i32 =  self.SAObj[index].X;
                let mut y: i32 =  self.SAObj[index].Y;
                let mut tweight: i32 =  (int) Math.Round( ( self.SAObj[index].fuzzyvp * self.game.Data.RuleVar[151]));
                if (tweight < 0)
                  tweight = 0;
                SL.Add(tid1, tweight, x, y, (int) Math.Round( self.game.Data.RuleVar[152]));
              }
            }
          }
        }
        else if (self.TPlanObj[PlanNr].Type == 40)
        {
          num: i32;
          tid1 = num + 1;
          let mut tweight: i32 =  (int) Math.Round( (self.game.Data.RuleVar[151] *  self.TPlanObj[PlanNr].FromArea.fuzzyvp));
          if (tweight < 0)
            tweight = 0;
          SL.Add(tid1, tweight, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 2000);
        }
      }
      let mut counter1: i32 =  SL.Counter;
      for (let mut Number2: i32 =  0; Number2 <= counter1; Number2 += 1)
        self.AddLog("FLAG " + Conversion.Str( Number2) + ": hex(" + Conversion.Str( SL.Data1[Number2]) + "," + Conversion.Str( SL.Data2[Number2]) + "), Weight=" + Conversion.Str( SL.Weight[Number2]) + ", Data4=" + Conversion.Str( SL.Data4[Number2]));
      let mut num23: i32 =  1;
      SimpleList simpleList1 = SimpleList::new();
      if (SL.Counter > -1)
      {
        while (num23 == 1)
        {
          num23 = 0;
          if (Number1 > SL.Counter + 1)
          {
            SL.Sort();
            Expression = self.ClosestFriendlyHex(SL.Data1[SL.Counter], SL.Data2[SL.Counter], ref SL);
            if (!Expression.onmap)
            {
              Expression = self.ClosestUnFriendlyHex(SL.Data1[SL.Counter], SL.Data2[SL.Counter], ref SL);
              if (!Expression.onmap)
              {
                Expression.x = SL.Data1[SL.Counter];
                Expression.y = SL.Data2[SL.Counter];
                Expression.onmap = true;
              }
            }
            let mut tweight: i32 =  (int) Math.Round(Conversion.Int( SL.Weight[SL.Counter] / 6.0));
            int[] weight = SL.Weight;
            int[] numArray = weight;
            let mut counter2: i32 =  SL.Counter;
            let mut index: i32 =  counter2;
            let mut num24: i32 =  weight[counter2] - tweight;
            numArray[index] = num24;
            tid1 += 1;
            SL.Add(tid1, tweight, Expression.x, Expression.y, self.Matrix1[Expression.x, Expression.y]);
            num23 = 1;
          }
          else if (Number1 < SL.Counter + 1 & SL.Counter > 0)
          {
            SimpleList simpleList2 = SimpleList::new();
            let mut tid3: i32 =  0;
            let mut counter3: i32 =  SL.Counter;
            for (let mut tdata1: i32 =  0; tdata1 <= counter3; tdata1 += 1)
            {
              let mut counter4: i32 =  SL.Counter;
              for (let mut tdata2: i32 =  0; tdata2 <= counter4; tdata2 += 1)
              {
                if (tdata1 != tdata2)
                {
                  let mut num25: i32 =  self.game.HandyFunctionsObj.Distance(SL.Data1[tdata1], SL.Data2[tdata1], 0, SL.Data1[tdata2], SL.Data2[tdata2], 0);
                  tid3 += 1;
                  if (num25 == 0)
                    num25 = 1;
                  let mut tweight: i32 =  (int) Math.Round( num25 * Math.Sqrt( SL.Weight[tdata1]) * Math.Sqrt( SL.Weight[tdata2]));
                  simpleList2.Add(tid3, tweight, tdata1, tdata2);
                }
              }
            }
            simpleList2.Sort();
            if (SL.Weight[simpleList2.Data1[0]] + SL.Data4[simpleList2.Data1[0]] * 5000 > SL.Weight[simpleList2.Data2[0]] + SL.Data4[simpleList2.Data2[0]] * 5000)
            {
              self.AddLog("Join flag " + Conversion.Str( simpleList2.Data2[0]) + " with flag " + Conversion.Str( simpleList2.Data1[0]));
              int[] weight = SL.Weight;
              int[] numArray1 = weight;
              int[] data1 = simpleList2.Data1;
              int[] numArray2 = data1;
              let mut index24: i32 =  0;
              let mut index25: i32 =  index24;
              let mut index26: i32 =  numArray2[index25];
              let mut num26: i32 =  weight[data1[index24]] + SL.Weight[simpleList2.Data2[0]];
              numArray1[index26] = num26;
              SL.Remove(SL.Id[simpleList2.Data2[0]]);
            }
            else
            {
              self.AddLog("Join flag " + Conversion.Str( simpleList2.Data1[0]) + " with flag " + Conversion.Str( simpleList2.Data2[0]));
              int[] weight = SL.Weight;
              int[] numArray3 = weight;
              int[] data2 = simpleList2.Data2;
              int[] numArray4 = data2;
              let mut index27: i32 =  0;
              let mut index28: i32 =  index27;
              let mut index29: i32 =  numArray4[index28];
              let mut num27: i32 =  weight[data2[index27]] + SL.Weight[simpleList2.Data1[0]];
              numArray3[index29] = num27;
              SL.Remove(SL.Id[simpleList2.Data1[0]]);
            }
            num23 = 1;
          }
        }
        if (self.TPlanObj[PlanNr].Type == 20 &  self.game.Data.RuleVar[225] > 0.0 & self.TPlanObj[PlanNr].Stand == 1)
        {
          num28: i32;
          if ( self.game.Data.RuleVar[225] == 1.0)
            num28 = (int) Math.Round( SL.Counter * 0.5);
          if ( self.game.Data.RuleVar[225] == 2.0)
            num28 = (int) Math.Round( SL.Counter * 0.8);
          if (num28 == 0)
            num28 = 1;
          if (num28 >= SL.Counter + 1)
            num28 = SL.Counter - 1;
          if (num28 > 0)
          {
            SimpleList simpleList3 = SimpleList::new();
            let mut tid4: i32 =  0;
            let mut counter5: i32 =  SL.Counter;
            for (let mut index: i32 =  0; index <= counter5; index += 1)
            {
              let mut num29: i32 =  (int) Math.Round(Math.Pow( self.game.HandyFunctionsObj.Distance(SL.Data1[index], SL.Data2[index], 0, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 0), 1.5));
              if (num29 == 0)
                num29 = 1;
              let mut tweight: i32 =  (int) Math.Round( SL.Weight[index] /  num29);
              tid4 += 1;
              if (tweight == 0)
                tweight = 1;
              if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[SL.Data1[index], SL.Data2[index]].Regime, self.game.Data.Turn) | self.game.Data.MapObj[0].HexObj[SL.Data1[index], SL.Data2[index]].Regime == -1)
                tweight *= num29;
              if (SL.Data4[index] == 1)
                tweight += 1000;
              if (SL.Data4[index] == 2)
                tweight -= 1000;
              simpleList3.Add(tid4, tweight, SL.Id[index]);
            }
            simpleList3.Sort();
            let mut num30: i32 =  num28 - 1;
            for (let mut index: i32 =  0; index <= num30; index += 1)
            {
              if (index >= 0)
                SL.Remove(simpleList3.Data1[index]);
            }
          }
          let mut num31: i32 =  1;
          while (num31 == 1)
          {
            num31 = 0;
            if (Number1 > SL.Counter + 1)
            {
              SL.Sort();
              Expression.x = SL.Data1[SL.Counter];
              Expression.y = SL.Data2[SL.Counter];
              Expression.onmap = true;
              let mut tweight: i32 =  (int) Math.Round(Conversion.Int( SL.Weight[SL.Counter] / 3.0));
              int[] weight = SL.Weight;
              int[] numArray = weight;
              let mut counter6: i32 =  SL.Counter;
              let mut index: i32 =  counter6;
              let mut num32: i32 =  weight[counter6] - tweight;
              numArray[index] = num32;
              tid1 += 1;
              SL.Add(tid1, tweight, Expression.x, Expression.y, self.Matrix1[Expression.x, Expression.y]);
              num31 = 1;
            }
          }
        }
        SL.Sort();
        let mut counter7: i32 =  SL.Counter;
        for (let mut Number3: i32 =  0; Number3 <= counter7; Number3 += 1)
          self.AddLog("FLAG.. " + Conversion.Str( Number3) + ": hex(" + Conversion.Str( SL.Data1[Number3]) + "," + Conversion.Str( SL.Data2[Number3]) + "), Weight=" + Conversion.Str( SL.Weight[Number3]));
        object[,] objArray2 = new object[SL.Counter + 1, UL.Counter + 1];
        let mut counter8: i32 =  UL.Counter;
        for (let mut index: i32 =  0; index <= counter8; index += 1)
        {
          if (!(self.TPlanObj[PlanNr].Type == 20 & self.game.Data.UnitObj[UL.Id[index]].IsHQ) && !(self.TPlanObj[PlanNr].Type == 30 & self.game.Data.UnitObj[UL.Id[index]].IsHQ) && self.game.Data.UnitObj[UL.Id[index]].AICutoff == 0 & !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 4 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 3) && !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 5 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 10) && !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 9 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 8) && self.game.Data.UnitObj[UL.Id[index]].X != -1 & !self.game.Data.UnitObj[UL.Id[index]].AIReserve & !self.game.Data.UnitObj[UL.Id[index]].AIDisband)
          {
            self.game.HandyFunctionsObj.MakeMovePrediction(UL.Id[index], self.game.Data.UnitObj[UL.Id[index]].X, self.game.Data.UnitObj[UL.Id[index]].Y, 0, attack: true, increaseap: 200);
            let mut counter9: i32 =  SL.Counter;
            for (let mut Number4: i32 =  0; Number4 <= counter9; Number4 += 1)
            {
              let mut Number5: i32 =  (int) Math.Round( ( self.game.EditObj.TempValue[0].Value[SL.Data1[Number4], SL.Data2[Number4]] * self.GetEntrenchMod(UL.Id[index])));
              if (Number5 < 9999)
                Number5 = (int) Math.Round( Number5 * (Math.Pow( self.GetTerrainMovePathMod(UL.Id[index], SL.Data1[Number4], SL.Data2[Number4]), 2.0) / 1.0));
              let mut num33: i32 =  0;
              if (Number5 < 9999)
                num33 = (int) Math.Round(Math.Sqrt(Math.Sqrt( self.game.HandyFunctionsObj.GetPowerPtsAbsolute(UL.Id[index]))));
              if (num33 >= 1)
                Number5 = (int) Math.Round(Conversion.Int( Number5 /  num33));
              objArray2[Number4, index] =  Number5;
              self.AddLog(self.game.Data.UnitObj[UL.Id[index]].Name + " => flag " + Conversion.Str( Number4) + " = " + Conversion.Str( Number5));
            }
            Application.DoEvents();
          }
        }
        for (let mut counter10: i32 =  SL.Counter; counter10 >= 0; counter10 += -1)
        {
          let mut num34: i32 =  9999;
          let mut index30: i32 =  -1;
          let mut counter11: i32 =  UL.Counter;
          for (let mut index31: i32 =  0; index31 <= counter11; index31 += 1)
          {
            if (!(self.TPlanObj[PlanNr].Type == 20 & self.game.Data.UnitObj[UL.Id[index31]].IsHQ) && !(self.TPlanObj[PlanNr].Type == 30 & self.game.Data.UnitObj[UL.Id[counter10]].IsHQ) && self.game.Data.UnitObj[UL.Id[index31]].AICutoff == 0 & !(self.game.Data.UnitObj[UL.Id[index31]].AIUnitGoal == 4 | self.game.Data.UnitObj[UL.Id[index31]].AIUnitGoal == 3) && !(self.game.Data.UnitObj[UL.Id[index31]].AIUnitGoal == 5 | self.game.Data.UnitObj[UL.Id[index31]].AIUnitGoal == 10) && !(self.game.Data.UnitObj[UL.Id[index31]].AIUnitGoal == 9 | self.game.Data.UnitObj[UL.Id[index31]].AIUnitGoal == 8) && self.game.Data.UnitObj[UL.Id[index31]].X != -1 & !self.game.Data.UnitObj[UL.Id[index31]].AIReserve & !self.game.Data.UnitObj[UL.Id[index31]].AIDisband && Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectLess(objArray2[counter10, index31],  num34, false),  (UL.Data1[index31] == -1))))
            {
              num34 = Conversions.ToInteger(objArray2[counter10, index31]);
              index30 = index31;
              self.AddLog(self.game.Data.UnitObj[UL.Id[index31]].Name + " => ASSIGNED TO flag " + Conversion.Str( counter10) + " with temphigh = " + Conversion.Str( num34));
            }
          }
          if (index30 > -1)
            UL.Data1[index30] = counter10;
        }
        let mut counter12: i32 =  UL.Counter;
        for (let mut index: i32 =  0; index <= counter12; index += 1)
        {
          let mut num35: i32 =  9999;
          let mut num36: i32 =  -1;
          if (UL.Data1[index] == -1)
          {
            for (let mut counter13: i32 =  SL.Counter; counter13 >= 0; counter13 += -1)
            {
              if (!(self.TPlanObj[PlanNr].Type == 20 & self.game.Data.UnitObj[UL.Id[index]].IsHQ) && !(self.TPlanObj[PlanNr].Type == 30 & self.game.Data.UnitObj[UL.Id[counter13]].IsHQ) && self.game.Data.UnitObj[UL.Id[index]].AICutoff == 0 & !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 4 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 3) && !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 5 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 10) && !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 9 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 8) && self.game.Data.UnitObj[UL.Id[index]].X != -1 & !self.game.Data.UnitObj[UL.Id[index]].AIReserve & !self.game.Data.UnitObj[UL.Id[index]].AIDisband && Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectLess(objArray2[counter13, index],  num35, false),  (UL.Data1[index] == -1))))
              {
                self.AddLog(self.game.Data.UnitObj[UL.Id[index]].Name + " => (LAST DITCH ASSIGN) ASSIGNED TO flag " + Conversion.Str( counter13) + " with temphigh = " + Conversion.Str( num35));
                num35 = Conversions.ToInteger(objArray2[counter13, index]);
                num36 = counter13;
              }
            }
            if (num36 > -1)
              UL.Data1[index] = num36;
          }
        }
        let mut counter14: i32 =  UL.Counter;
        for (let mut index: i32 =  0; index <= counter14; index += 1)
        {
          let mut Number6: i32 =  9999;
          let mut num37: i32 =  -1;
          if (UL.Data1[index] == -1)
          {
            for (let mut counter15: i32 =  SL.Counter; counter15 >= 0; counter15 += -1)
            {
              if (!(self.TPlanObj[PlanNr].Type == 20 & self.game.Data.UnitObj[UL.Id[index]].IsHQ) && !(self.TPlanObj[PlanNr].Type == 30 & self.game.Data.UnitObj[UL.Id[counter15]].IsHQ) && self.game.Data.UnitObj[UL.Id[index]].AICutoff == 0 & !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 4 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 3) && !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 5 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 10) && !(self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 9 | self.game.Data.UnitObj[UL.Id[index]].AIUnitGoal == 8) && self.game.Data.UnitObj[UL.Id[index]].X != -1 & !self.game.Data.UnitObj[UL.Id[index]].AIReserve & !self.game.Data.UnitObj[UL.Id[index]].AIDisband && UL.Data1[index] == -1)
              {
                self.AddLog(self.game.Data.UnitObj[UL.Id[index]].Name + " => (REALLY LAST DITCH ASSIGN) ASSIGNED TO flag " + Conversion.Str( counter15) + " with temphigh = " + Conversion.Str( Number6));
                Conversions.ToInteger(objArray2[counter15, index]);
                num37 = counter15;
                break;
              }
            }
            if (num37 > -1)
              UL.Data1[index] = num37;
          }
        }
      }
      if (self.TPlanObj[PlanNr].Type == 20 | self.TPlanObj[PlanNr].Type == 40 | self.TPlanObj[PlanNr].Type == 30 | self.TPlanObj[PlanNr].Type == 50)
      {
        let mut counter16: i32 =  UL.Counter;
        for (let mut index32: i32 =  0; index32 <= counter16; index32 += 1)
        {
          if (UL.Data1[index32] == -1 & self.game.Data.UnitObj[UL.Id[index32]].X != -1)
          {
            let mut index33: i32 =  UL.Id[index32];
            if (self.game.Data.UnitObj[index33].IsHQ)
            {
              let mut num38: i32 =  UL.Data1[index32];
              Expression = self.SetMatrixHQ(UL, index33);
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, self.Matrix2[Expression.x, Expression.y]);
                self.AddLog("HQ FLAG ADDED to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
            else if (self.game.Data.UnitObj[index33].AIDisband)
            {
              let mut index34: i32 =  self.HexSA[self.game.Data.UnitObj[index33].X, self.game.Data.UnitObj[index33].Y];
              if (index34 > 0)
              {
                let mut landReservePlan: i32 =  self.SAObj[index34].LandReservePlan;
                if (landReservePlan > 0)
                {
                  if (self.TPlanObj[landReservePlan].HQ > -1)
                  {
                    Expression = Coordinate::new();
                    Expression.x = self.game.Data.UnitObj[self.TPlanObj[landReservePlan].HQ].X;
                    Expression.y = self.game.Data.UnitObj[self.TPlanObj[landReservePlan].HQ].Y;
                    tid1 += 1;
                    SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                    UL.Data1[index32] = SL.Counter;
                    self.AddLog("DISBAND UNIT FLAG ADDED..  to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                  }
                  else
                    self.game.Data.UnitObj[index33].AIReserve = true;
                }
                else
                  self.game.Data.UnitObj[index33].AIReserve = true;
              }
              else
                self.game.Data.UnitObj[index33].AIReserve = true;
            }
            else if (self.game.Data.UnitObj[index33].AIReserve)
            {
              resnr: i32;
              resnr += 1;
              Expression = self.GetReserveCoord(PlanNr, resnr);
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                self.AddLog("RESERVE UNIT FLAG ADDED..  to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
              else
              {
                tid1 += 1;
                SL.Add(tid1, 100, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 100);
                self.AddLog("RESERVE UNIT FLAG ADDED..  to  (" + Conversion.Str( self.TPlanObj[PlanNr].FromArea.X) + "," + Conversion.Str( self.TPlanObj[PlanNr].FromArea.Y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
            else if (self.game.Data.UnitObj[index33].AICutoff > 0)
            {
              Expression = self.GetEscapeCoord(self.game.Data.UnitObj[index33].X, self.game.Data.UnitObj[index33].Y, self.GetAreaNr(self.TPlanObj[PlanNr].FromArea));
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                self.AddLog("CUTTEN OFF UNIT FLAG ADDED.. (WE SEE ESCAPE OPTION) to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
              else
              {
                tid1 += 1;
                SL.Add(tid1, 100, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 100);
                self.AddLog("CUTTEN OFF UNIT FLAG ADDED to (" + Conversion.Str( self.TPlanObj[PlanNr].FromArea.X) + "," + Conversion.Str( self.TPlanObj[PlanNr].FromArea.Y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
            else if (self.game.Data.UnitObj[index33].AIUnitGoal == 4)
            {
              engcount: i32;
              engcount += 1;
              Expression = self.GetEngineerCoord(engcount, PlanNr);
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                self.AddLog("ENGINEER UNIT FLAG ADDED..  to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
              else
              {
                tid1 += 1;
                SL.Add(tid1, 100, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 100);
                self.AddLog("ENGINEER UNIT FLAG ADDED to (" + Conversion.Str( self.TPlanObj[PlanNr].FromArea.X) + "," + Conversion.Str( self.TPlanObj[PlanNr].FromArea.Y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
            else if (self.game.Data.UnitObj[index33].AIUnitGoal == 9 | self.game.Data.UnitObj[index33].AIUnitGoal == 10 | self.game.Data.UnitObj[index33].AIUnitGoal == 8)
            {
              Expression = self.GetNavalWarCoord(index33, PlanNr);
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                self.AddLog("NAVAL UNIT FLAG ADDED..  to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
              else
              {
                tid1 += 1;
                SL.Add(tid1, 100, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 100);
                self.AddLog("NAVAL UNIT FLAG ADDED to (" + Conversion.Str( self.TPlanObj[PlanNr].FromArea.X) + "," + Conversion.Str( self.TPlanObj[PlanNr].FromArea.Y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
            else if (self.game.Data.UnitObj[index33].AIUnitGoal == 5)
            {
              Expression = self.GetAirSupportCoord(index33, PlanNr);
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                self.AddLog("AIRSUPPORT UNIT FLAG ADDED..  to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
              else
              {
                tid1 += 1;
                SL.Add(tid1, 100, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 100);
                self.AddLog("AIRSUPPORT UNIT FLAG ADDED to (" + Conversion.Str( self.TPlanObj[PlanNr].FromArea.X) + "," + Conversion.Str( self.TPlanObj[PlanNr].FromArea.Y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
            else if (self.game.Data.UnitObj[index33].AIUnitGoal == 3)
            {
              Expression = self.GetArtilleryCoord(index33, PlanNr);
              if (MovePhaseNr < 2 & self.TPlanObj[PlanNr].Stand == 1)
              {
                Expression.x = self.game.Data.UnitObj[index33].X;
                Expression.y = self.game.Data.UnitObj[index33].Y;
                Expression.onmap = true;
              }
              if (self.TPlanObj[PlanNr].Stand == 3)
              {
                Expression.x = self.TPlanObj[PlanNr].TooArea.X;
                Expression.y = self.TPlanObj[PlanNr].TooArea.Y;
                Expression.onmap = true;
              }
              if (Expression.onmap)
              {
                tid1 += 1;
                SL.Add(tid1, 100, Expression.x, Expression.y, 100);
                self.AddLog("ARTILLERY UNIT FLAG ADDED..  to (" + Conversion.Str( Expression.x) + "," + Conversion.Str( Expression.y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
              else
              {
                tid1 += 1;
                SL.Add(tid1, 100, self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 100);
                self.AddLog("ARTILLERY UNIT FLAG ADDED to (" + Conversion.Str( self.TPlanObj[PlanNr].FromArea.X) + "," + Conversion.Str( self.TPlanObj[PlanNr].FromArea.Y) + ")");
                UL.Data1[index32] = SL.Counter;
              }
            }
          }
        }
      }
      let mut counter17: i32 =  UL.Counter;
      for (let mut index: i32 =  0; index <= counter17; index += 1)
      {
        if (UL.Data1[index] > -1)
          self.AddLog("UNIT " + self.game.Data.UnitObj[UL.Id[index]].Name + " --> flag(" + Conversion.Str( SL.Data1[UL.Data1[index]]) + "," + Conversion.Str( SL.Data2[UL.Data1[index]]) + ")");
        else
          self.AddLog("UNIT " + self.game.Data.UnitObj[UL.Id[index]].Name + " --> NO FLAG!! ");
      }
      self.AddLog("");
      self.AddLog("Unit Movements:");
      for (let mut counter18: i32 =  UL.Counter; counter18 >= 0; counter18 += -1)
      {
        if (UL.Data1[counter18] > -1 & self.game.Data.UnitObj[UL.Id[counter18]].X != -1)
        {
          let mut index35: i32 =  UL.Id[counter18];
          let mut num39: i32 =  0;
          let mut num40: i32 =  1;
          if (MovePhaseNr == 1 &  self.game.Data.RuleVar[225] > 0.0 && !(self.game.Data.UnitObj[index35].AIUnitGoal == 3 | self.game.Data.UnitObj[index35].AIUnitGoal == 4) && !(self.game.Data.UnitObj[index35].AIUnitGoal == 5 | self.game.Data.UnitObj[index35].AIUnitGoal == 10) && !(self.game.Data.UnitObj[index35].AIUnitGoal == 9 | self.game.Data.UnitObj[index35].AIUnitGoal == 8) && self.game.Data.UnitObj[UL.Id[counter18]].X != -1 & !self.game.Data.UnitObj[UL.Id[counter18]].AIReserve & !self.game.Data.UnitObj[UL.Id[counter18]].AIDisband && self.TPlanObj[PlanNr].Type == 20)
          {
            let mut tfacing: i32 =  1;
            do
            {
              Expression = self.game.HandyFunctionsObj.HexNeighbour(self.game.Data.UnitObj[index35].X, self.game.Data.UnitObj[index35].Y, 0, tfacing);
              if (Expression.onmap && self.game.Data.MapObj[0].HexObj[Expression.x, Expression.y].Regime > -1 && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[Expression.x, Expression.y].Regime) && self.game.Data.MapObj[0].HexObj[Expression.x, Expression.y].UnitCounter > -1)
                num40 = 0;
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
          if (num40 == 1)
          {
            let mut num41: i32 =  !Information.IsNothing( self.TPlanObj[PlanNr].TooArea) ? self.game.HandyFunctionsObj.Distance(self.TPlanObj[PlanNr].FromArea.X, self.TPlanObj[PlanNr].FromArea.Y, 0, self.TPlanObj[PlanNr].TooArea.X, self.TPlanObj[PlanNr].TooArea.Y, 0) : 15;
            if (num41 < 10)
              num41 = 10;
            let mut MaxDist: i32 =  num41 * 2;
            if (self.game.Data.UnitObj[index35].AIUnitGoal == 5)
              self.SetMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]]);
            else if (self.game.Data.UnitObj[index35].AIUnitGoal == 9 | self.game.Data.UnitObj[index35].AIUnitGoal == 10 | self.game.Data.UnitObj[index35].AIUnitGoal == 8)
              self.SetNavalMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]]);
            else if (self.game.Data.UnitObj[index35].AIPlanNr > 0)
            {
              if (self.GetAreaNr(self.TPlanObj[self.game.Data.UnitObj[index35].AIPlanNr].FromArea) == self.HexSA[self.game.Data.UnitObj[index35].X, self.game.Data.UnitObj[index35].Y])
              {
                if (self.TPlanObj[self.game.Data.UnitObj[index35].AIPlanNr].Type == 40 | self.TPlanObj[self.game.Data.UnitObj[index35].AIPlanNr].Type == 30)
                  self.SetMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]], unitnr: index35, onlyinplanarea: true, hq: self.game.Data.UnitObj[index35].HQ, MaxDist: MaxDist);
                else if (self.TPlanObj[self.game.Data.UnitObj[index35].AIPlanNr].FromArea.ConstitutantCount > 0)
                  self.SetMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]], true, index35, true, MaxDist: MaxDist);
                else
                  self.SetMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]], true, index35, true, hq: self.game.Data.UnitObj[index35].HQ, MaxDist: MaxDist);
              }
              else
                self.SetMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]], MaxDist: MaxDist);
            }
            else
              self.SetMatrix1(SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]], MaxDist: MaxDist);
            if (self.game.Data.UnitObj[index35].AIUnitGoal == 5)
            {
              self.game.HandyFunctionsObj.MakeMovePrediction(index35, self.game.Data.UnitObj[index35].X, self.game.Data.UnitObj[index35].Y, 0, ismove: true);
            }
            else
            {
              let mut gothroughenemy: i32 =  (int) Math.Round( ( self.game.HandyFunctionsObj.GetPower(index35, self.game.Data.Turn) / (2f * self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative)));
              self.game.HandyFunctionsObj.MakeMovePrediction(index35, self.game.Data.UnitObj[index35].X, self.game.Data.UnitObj[index35].Y, 0, increaseap: 250, ismove: true, gothroughenemy: gothroughenemy);
            }
            OrderResult orderResult = (OrderResult) null;
            index36: i32;
            index37: i32;
            if (self.game.EditObj.TempValue[0].Value[SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]]] >= 9999)
            {
              let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut index38: i32 =  0; index38 <= mapWidth; index38 += 1)
              {
                let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut index39: i32 =  0; index39 <= mapHeight; index39 += 1)
                {
                  if (self.game.EditObj.TempValue[0].Value[index38, index39] <= self.game.HandyFunctionsObj.GetLowestAp(index35) && self.Matrix1[index38, index39] > num39)
                  {
                    index36 = index38;
                    index37 = index39;
                    num39 = self.Matrix1[index38, index39];
                  }
                }
              }
              if (num39 > 0)
              {
                if ( self.game.Data.RuleVar[253] == 1.0 & !self.game.HandyFunctionsObj.HasUnitNavySF(index35) & !self.game.HandyFunctionsObj.HasUnitAirSF(index35))
                {
                  if (self.game.Data.MapObj[0].HexObj[SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]]].UnitCounter <= 14 & self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]]].Regime, self.game.Data.Turn))
                  {
                    orderResult = (OrderResult) self.game.ProcessingObj.DoStrategicTransfer(-1, index35, 0, SL.Data1[UL.Data1[counter18]], SL.Data2[UL.Data1[counter18]], 0);
                    index36 = SL.Data1[UL.Data1[counter18]];
                    index37 = SL.Data2[UL.Data1[counter18]];
                    s: String = self.game.Data.UnitObj[index35].Name + " strategicly transfers to " + Conversion.Str( index36) + "," + Conversion.Str( index37);
                    self.UnitMovePhase[index35] = MovePhaseNr;
                    self.AddLog(s);
                  }
                }
                else if (!(self.game.Data.MapObj[0].HexObj[index36, index37].UnitCounter > 14 & self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index36, index37].Regime, self.game.Data.Turn)))
                {
                  orderResult = self.game.ProcessingObj.ExecuteMovement(index35, self.game.Data.UnitObj[index35].X, self.game.Data.UnitObj[index35].Y, 0, index36, index37, 0);
                  s: String = self.game.Data.UnitObj[index35].Name + " moves to " + Conversion.Str( index36) + "," + Conversion.Str( index37);
                  self.UnitMovePhase[index35] = MovePhaseNr;
                  self.AddLog(s);
                }
              }
            }
            else
            {
              let mut num42: i32 =  0;
              let mut num43: i32 =  0;
              index36 = SL.Data1[UL.Data1[counter18]];
              index37 = SL.Data2[UL.Data1[counter18]];
              if (!(index36 == self.game.Data.UnitObj[index35].X & index37 == self.game.Data.UnitObj[index35].Y))
              {
                while (num42 == 0)
                {
                  if (self.game.EditObj.TempValue[0].Value[index36, index37] <= self.game.HandyFunctionsObj.GetLowestAp(index35))
                  {
                    num43 = 1;
                    break;
                  }
                  Expression = self.game.EditObj.TempCameFrom[0].Value[index36, index37];
                  if (!Information.IsNothing( Expression) && Expression.onmap)
                  {
                    index36 = Expression.x;
                    index37 = Expression.y;
                  }
                  else
                    break;
                }
              }
              else
                num43 = 0;
              if (num43 > 0 & !(self.game.Data.MapObj[0].HexObj[index36, index37].UnitCounter > 14 & self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index36, index37].Regime, self.game.Data.Turn)))
              {
                orderResult = self.game.ProcessingObj.ExecuteMovement(index35, self.game.Data.UnitObj[index35].X, self.game.Data.UnitObj[index35].Y, 0, index36, index37, 0);
                s: String = self.game.Data.UnitObj[index35].Name + " moves to " + Conversion.Str( index36) + "," + Conversion.Str( index37);
                self.UnitMovePhase[index35] = MovePhaseNr;
                self.AddLog(s);
              }
            }
          }
          else
            self.AddLog(self.game.Data.UnitObj[index35].Name + " stays on spot in this move phase due to possible enemy target near.");
        }
      }
      self.CratesCheck();
    }

    pub float GetTerrainMovePathMod(unr: i32, x: i32, y: i32)
    {
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      let mut sfCount1: i32 =  self.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 =  0; index <= sfCount1; index += 1)
      {
        let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr].SFList[index]].Type;
        let mut qty: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr].SFList[index]].Qty;
        let mut landscapeType: i32 =  self.game.Data.MapObj[0].HexObj[x, y].LandscapeType;
        num2 += self.game.Data.SFTypeObj[type].PowerPts * qty;
      }
      let mut num3: i32 =  0;
      float num4;
      while (num1 == 1)
      {
        num1 = 0;
        num3 += 1;
        let mut sfCount2: i32 =  self.game.Data.UnitObj[unr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount2; index += 1)
        {
          let mut type: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr].SFList[index]].Type;
          let mut qty: i32 =  self.game.Data.SFObj[self.game.Data.UnitObj[unr].SFList[index]].Qty;
          let mut landscapeType: i32 =  self.game.Data.MapObj[0].HexObj[x, y].LandscapeType;
          let mut num5: i32 =  self.game.Data.SFTypeObj[type].PowerPts * qty;
          num4 +=  (( self.game.Data.SFTypeObj[type].CombatModAtt[landscapeType] +  self.game.Data.SFTypeObj[type].CombatModDef[landscapeType]) / 2.0 * ( num5 /  num2));
        }
        Coordinate coordinate = self.game.EditObj.TempCameFrom[0].Value[x, y];
        if (coordinate.onmap)
        {
          x = coordinate.x;
          y = coordinate.y;
          num1 = 1;
        }
      }
      return num4 /  num3;
    }

    pub fn ExecuteLandFrontAttacks(plannr: i32, float advneed)
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      float num1 = 2f;
      do
      {
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea && self.HexSA[index1, index2] == self.GetAreaNr(self.TPlanObj[plannr].TooArea) && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index1, index2].Regime) & self.game.Data.MapObj[0].HexObj[index1, index2].get_SeeNow(self.game.Data.Turn) > 0)
            {
              let mut Number1: i32 =  self.GetHexForceLandStrength(index1, index2);
              if (Number1 == 0)
                Number1 = 1;
              SimpleList simpleList = SimpleList::new();
              let mut num2: i32 =  0;
              let mut num3: i32 =  0;
              let mut num4: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].get_BattleStack(self.game.Data.Turn);
              self.game.EditObj.TempUnitList = UnitList::new();
              self.game.EditObj.OrderX = index1;
              self.game.EditObj.OrderY = index2;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(index1, index2, 0, tfacing);
                if (coordinate.onmap && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime, self.game.Data.Turn) & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                {
                  let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter;
                  for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                  {
                    let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[index3];
                    if (self.game.HandyFunctionsObj.MoveApCostPreview(unit, self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y, self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y, 0, index1, index2, 0, true).x <= self.game.HandyFunctionsObj.GetLowestAp(unit))
                    {
                      if ( self.game.Data.RuleVar[30] >  num4)
                      {
                        let mut num5: i32 =  self.GetForceLandStrength(unit, asattack: true, attackx: index1, attacky: index2);
                        if (self.TPlanObj[plannr].Stand == 2)
                          num5 = (int) Math.Round( ( num5 * (1f / self.GetEntrenchMod(unit))));
                        let mut tweight: i32 =  (int) Math.Round( num5 * (Math.Pow( self.game.HandyFunctionsObj.GetAverageRdn(unit), 2.0) / 10000.0));
                        num2 += tweight;
                        num3 += tweight;
                        num4 += self.game.HandyFunctionsObj.GetUnitStackPts(unit);
                        simpleList.Add(unit, tweight);
                        self.game.EditObj.TempUnitList.add(unit);
                      }
                      else
                      {
                        let mut num6: i32 =  self.GetForceLandStrength(unit, asattack: true, attackx: index1, attacky: index2);
                        if (self.TPlanObj[plannr].Stand == 2)
                          num6 = (int) Math.Round( ( num6 * (1f / self.GetEntrenchMod(unit))));
                        let mut num7: i32 =  (int) Math.Round( num6 * (Math.Pow( self.game.HandyFunctionsObj.GetAverageRdn(unit), 2.0) / 10000.0));
                        num3 += num7;
                      }
                    }
                  }
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (num2 > 0)
              {
                float concentricBonus2 = self.game.HandyFunctionsObj.GetConcentricBonus2();
                if ( concentricBonus2 >=  num1)
                {
                  let mut Number2: i32 =  (int) Math.Round( ( num2 * concentricBonus2));
                  if ( self.game.HandyFunctionsObj.GetHexStackPts(index1, index2, 0) >  self.game.Data.RuleVar[30] * 1.5)
                  {
                    float num8 = self.game.Data.RuleVar[30] * 1.5f /  self.game.HandyFunctionsObj.GetHexStackPts(index1, index2, 0);
                    if ( num8 > 1.0)
                      num8 = 1f;
                    Number1 = (int) Math.Round( Conversion.Int( Number1 * num8));
                  }
                  if (num3 > Number2 & Number2 > 0)
                    Number2 = (int) Math.Round( Number2 * ( num3 /  Number2));
                  if ( Number2 /  Number1 >=  advneed)
                  {
                    self.game.TempCombat = new CombatClass(self.game);
                    Coordinate Target = Coordinate::new();
                    Target.x = index1;
                    Target.y = index2;
                    self.game.EditObj.TempUnitList = UnitList::new();
                    self.AddLog("* Battle versus " + Conversion.Str( index1) + "," + Conversion.Str( index2) + " with " + Conversion.Str( Number2) + " vs " + Conversion.Str( Number1) + " force.");
                    let mut counter: i32 =  simpleList.Counter;
                    for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
                      self.game.EditObj.TempUnitList.add(simpleList.Id[index4]);
                    self.game.TempCombat.Init(Target, 1, self.game.EditObj.TempUnitList, 2);
                    self.game.TempCombat.DoBattle();
                    self.game.TempCombat.EndBattle();
                    self.game.TempCombat = (CombatClass) null;
                  }
                }
              }
            }
          }
        }
        num1 += -0.5f;
      }
      while ( num1 >= 0.0);
    }

    pub fn ExecuteNavalAttacks(plannr: i32, float advneed)
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea | self.game.HandyFunctionsObj.IsHexPort(index1, index2, 0) && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[0]].Regime) & self.game.Data.MapObj[0].HexObj[index1, index2].get_SeeNow(self.game.Data.Turn) > 0)
          {
            let mut Number1: i32 =  self.GetHexForceLandStrength(index1, index2);
            if (Number1 == 0)
              Number1 = 1;
            SimpleList simpleList = SimpleList::new();
            let mut num1: i32 =  0;
            self.game.EditObj.TempUnitList = UnitList::new();
            self.game.EditObj.OrderX = index1;
            self.game.EditObj.OrderY = index2;
            let mut tfacing: i32 =  1;
            do
            {
              Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(index1, index2, 0, tfacing);
              if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter > -1 & self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[0]].Regime, self.game.Data.Turn))
              {
                let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter;
                for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                {
                  let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[index3];
                  if (self.game.Data.UnitObj[unit].AIPlanNr == plannr && self.game.HandyFunctionsObj.MoveApCostPreview(unit, self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y, self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y, self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y, 0, index1, (uint) index2 > 0U, IgnoreBridges: true).x <= self.game.HandyFunctionsObj.GetLowestAp(unit))
                  {
                    num1 += self.GetForceNavalStrength(unit, asattack: true, attackx: index1, attacky: index2);
                    simpleList.Add(unit, self.GetForceNavalStrength(unit, asattack: true, attackx: index1, attacky: index2));
                    self.game.EditObj.TempUnitList.add(unit);
                  }
                }
              }
              tfacing += 1;
            }
            while (tfacing <= 6);
            if (num1 > 0)
            {
              let mut Number2: i32 =  (int) Math.Round( ( num1 * self.game.HandyFunctionsObj.GetConcentricBonus2()));
              if ( self.game.HandyFunctionsObj.GetHexStackPts(index1, index2, 0) >  self.game.Data.RuleVar[30])
              {
                float num2 = self.game.Data.RuleVar[30] /  self.game.HandyFunctionsObj.GetHexStackPts(index1, index2, 0);
                if ( num2 > 1.0)
                  num2 = 1f;
                Number1 = (int) Math.Round( Conversion.Int( Number1 * num2));
              }
              if ( Number2 /  Number1 >=  advneed & Number2 > 0)
              {
                self.game.TempCombat = new CombatClass(self.game);
                Coordinate Target = Coordinate::new();
                Target.x = index1;
                Target.y = index2;
                self.game.EditObj.TempUnitList = UnitList::new();
                self.AddLog("* Nav Battle versus " + Conversion.Str( index1) + "," + Conversion.Str( index2) + " with " + Conversion.Str( Number2) + " vs " + Conversion.Str( Number1) + " force.");
                let mut counter: i32 =  simpleList.Counter;
                for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
                  self.game.EditObj.TempUnitList.add(simpleList.Id[index4]);
                self.game.TempCombat.Init(Target, 1, self.game.EditObj.TempUnitList, 12);
                self.game.TempCombat.DoBattle();
                self.game.TempCombat.EndBattle();
                self.game.TempCombat = (CombatClass) null;
              }
            }
          }
        }
      }
    }

    pub fn ExecuteAirAttack(plannr: i32, float advneed)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray4: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray5: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      int[,,] numArray6 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1, 11];
      if (self.TPlanObj[plannr].FriendlyAir < 1)
        return;
      self.AddLog("AirAttacks for plannr: " + Conversion.Str( plannr));
      let mut index1: i32 =  -1;
      let mut index2: i32 =  0;
      do
      {
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index3: i32 =  0; index3 <= mapWidth; index3 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
            numArray6[index3, index4, index2] = -1;
        }
        index2 += 1;
      }
      while (index2 <= 10);
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & self.game.Data.UnitObj[unr].X > -1 && self.game.Data.UnitObj[unr].AIPlanNr == plannr && self.game.Data.UnitObj[unr].AIUnitGoal == 5 && self.GetRolePercent(unr, 13) > 0 | self.GetRolePercent(unr, 14) > 0)
        {
          self.game.HandyFunctionsObj.MakeMovePrediction(unr, self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0, false, PredictAirOnly: true, attack: true);
          index1 += 1;
          if (index1 <= 10)
          {
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index5: i32 =  0; index5 <= mapWidth; index5 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
              {
                if (self.game.EditObj.TempValue[0].Value[index5, index6] < self.game.HandyFunctionsObj.GetLowestAirAp(unr) && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index5, index6].Regime) & self.game.Data.MapObj[0].HexObj[index5, index6].get_SeeNow(self.game.Data.Turn) > 0 && self.game.Data.MapObj[0].HexObj[index5, index6].UnitCounter > -1 && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index5, index6].UnitList[0]].Regime))
                {
                  numArray7: Vec<i32> = numArray1;
                  numArray8: Vec<i32> = numArray7;
                  let mut index7: i32 =  index5;
                  let mut index8: i32 =  index7;
                  let mut index9: i32 =  index6;
                  let mut index10: i32 =  index9;
                  let mut num1: i32 =  numArray7[index7, index9] + self.GetForceAirStrength(unr, asattack: true, attackx: index5, attacky: index6);
                  numArray8[index8, index10] = num1;
                  numArray9: Vec<i32> = numArray2;
                  numArray10: Vec<i32> = numArray9;
                  let mut index11: i32 =  index5;
                  let mut index12: i32 =  index11;
                  let mut index13: i32 =  index6;
                  let mut index14: i32 =  index13;
                  let mut num2: i32 =  numArray9[index11, index13] + self.GetForceAirStrength(unr);
                  numArray10[index12, index14] = num2;
                  numArray6[index5, index6, index1] = unr;
                  numArray11: Vec<i32> = numArray3;
                  numArray12: Vec<i32> = numArray11;
                  let mut index15: i32 =  index5;
                  let mut index16: i32 =  index15;
                  let mut index17: i32 =  index6;
                  let mut index18: i32 =  index17;
                  let mut num3: i32 =  numArray11[index15, index17] + 1;
                  numArray12[index16, index18] = num3;
                  numArray4[index5, index6] = self.GetHexForceLandStrength(index5, index6);
                }
              }
            }
          }
        }
      }
      let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime != self.game.Data.Turn & self.game.Data.UnitObj[unr].X > -1 && self.game.Data.RegimeObj[self.game.Data.UnitObj[unr].Regime].RegimeRel[self.game.Data.Turn] == 0 && self.game.HandyFunctionsObj.HasUnitAirSF(unr) && self.GetRolePercent(unr, 13) > 0)
        {
          self.game.HandyFunctionsObj.MakeMovePrediction(unr, self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0, false, PredictAirOnly: true, attack: true);
          let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index19: i32 =  0; index19 <= mapWidth; index19 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index20: i32 =  0; index20 <= mapHeight; index20 += 1)
            {
              if (self.game.EditObj.TempValue[0].Value[index19, index20] <= 50)
              {
                numArray13: Vec<i32> = numArray5;
                numArray14: Vec<i32> = numArray13;
                let mut index21: i32 =  index19;
                let mut index22: i32 =  index21;
                let mut index23: i32 =  index20;
                let mut index24: i32 =  index23;
                let mut num: i32 =  numArray13[index21, index23] + self.game.HandyFunctionsObj.GetPowerPtsAbsoluteForAirOnly(unr);
                numArray14[index22, index24] = num;
              }
            }
          }
        }
      }
      let mut unitCounter3: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter3; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime != self.game.Data.Turn & self.game.Data.UnitObj[unr].X > -1 && self.game.Data.RegimeObj[self.game.Data.UnitObj[unr].Regime].RegimeRel[self.game.Data.Turn] == 0)
        {
          let mut num4: i32 =  self.game.HandyFunctionsObj.HasUnitAA(unr);
          if (num4 > 0)
          {
            let mut x: i32 =  self.game.Data.UnitObj[unr].X;
            let mut y: i32 =  self.game.Data.UnitObj[unr].Y;
            let mut num5: i32 =  x - (num4 + 1);
            let mut num6: i32 =  x + (num4 + 1);
            for (let mut x1: i32 =  num5; x1 <= num6; x1 += 1)
            {
              let mut num7: i32 =  y - (num4 + 1);
              let mut num8: i32 =  y + (num4 + 1);
              for (let mut y1: i32 =  num7; y1 <= num8; y1 += 1)
              {
                if (x1 >= 0 & x1 <= self.game.Data.MapObj[0].MapWidth & y1 >= 0 & y1 <= self.game.Data.MapObj[0].MapHeight && self.game.HandyFunctionsObj.Distance(x1, y1, 0, x, y, 0) <= num4)
                {
                  let mut num9: i32 =  0;
                  let mut index25: i32 =  unr;
                  let mut sfCount: i32 =  self.game.Data.UnitObj[index25].SFCount;
                  for (let mut index26: i32 =  0; index26 <= sfCount; index26 += 1)
                  {
                    let mut sf: i32 =  self.game.Data.UnitObj[index25].SFList[index26];
                    let mut type: i32 =  self.game.Data.SFObj[sf].Type;
                    num10: i32;
                    if (self.game.Data.SFTypeObj[type].AIRoleScore[12] > 0)
                      num10 = (int) Math.Round( (self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty) * ( self.game.Data.SFTypeObj[type].AIRoleScore[12] / 100.0));
                    num9 += num10;
                  }
                  numArray15: Vec<i32> = numArray5;
                  numArray16: Vec<i32> = numArray15;
                  let mut index27: i32 =  x1;
                  let mut index28: i32 =  index27;
                  let mut index29: i32 =  y1;
                  let mut index30: i32 =  index29;
                  let mut num11: i32 =  numArray15[index27, index29] + num9;
                  numArray16[index28, index30] = num11;
                }
              }
            }
          }
        }
      }
      if (index1 <= -1)
        return;
      let mut Number1: i32 =  -1;
      let mut Number2: i32 =  -1;
      let mut num12: i32 =  0;
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index31: i32 =  0; index31 <= mapWidth1; index31 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index32: i32 =  0; index32 <= mapHeight; index32 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index31, index32].UnitCounter > -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index31, index32].UnitList[0]].Regime] == 0)
          {
            let mut num13: i32 =  (int) Math.Round( ( (numArray1[index31, index32] + numArray4[index31, index32]) + VBMath.Rnd() *  numArray4[index31, index32]));
            if ( self.game.Data.RuleVar[834] > 0.0)
            {
              if (self.game.Data.MapObj[0].HexObj[index31, index32].get_BattleStackAir(self.game.Data.Turn) > (int) Math.Round( self.game.Data.RuleVar[833] * 0.5))
                num13 = (int) Math.Round( num13 / 2.0);
              else if (self.game.Data.MapObj[0].HexObj[index31, index32].get_BattleStackAir(self.game.Data.Turn) > (int) Math.Round( (self.game.Data.RuleVar[833] * 1f)))
                num13 = (int) Math.Round( num13 / 4.0);
            }
            if (numArray5[index31, index32] * 3 > numArray2[index31, index32])
              num13 = (int) Math.Round( num13 -  numArray5[index31, index32] / 3.0);
            if (numArray5[index31, index32] > numArray2[index31, index32])
              num13 = 0;
            if (num13 > num12 & num13 > 0)
            {
              Number1 = index31;
              Number2 = index32;
              num12 = num13;
            }
          }
        }
      }
      if (!(Number1 > -1 & Number2 > -1))
        return;
      self.game.EditObj.TempUnitList = UnitList::new();
      let mut index33: i32 =  0;
      do
      {
        if (numArray6[Number1, Number2, index33] > -1)
          self.game.EditObj.TempUnitList.add(numArray6[Number1, Number2, index33]);
        index33 += 1;
      }
      while (index33 <= 10);
      self.game.TempCombat = new CombatClass(self.game);
      Coordinate Target = Coordinate::new();
      Target.x = Number1;
      Target.y = Number2;
      self.AddLog("* AIR Battle versus " + Conversion.Str( Number1) + "," + Conversion.Str( Number2));
      self.game.TempCombat.Init(Target, 1, self.game.EditObj.TempUnitList, 14);
      self.game.TempCombat.DoBattle();
      self.game.TempCombat.EndBattle();
      self.game.TempCombat = (CombatClass) null;
    }

    pub fn ExecuteAirAttack_OLD(plannr: i32, float advneed)
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.TPlanObj[plannr].FriendlyAir < 1)
        return;
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea && -(-1 < self.AreaDistance(self.HexSA[index1, index2], self.GetAreaNr(self.TPlanObj[plannr].FromArea)) ? 1 : 0) <= 2 && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index1, index2].Regime) & self.game.Data.MapObj[0].HexObj[index1, index2].get_SeeNow(self.game.Data.Turn) > 0)
          {
            let mut Number1: i32 =  self.GetHexForceLandStrength(index1, index2);
            if (Number1 == 0)
              Number1 = 1;
            SimpleList simpleList = SimpleList::new();
            let mut Number2: i32 =  0;
            self.game.EditObj.TempUnitList = UnitList::new();
            self.game.EditObj.OrderX = index1;
            self.game.EditObj.OrderY = index2;
            let mut num1: i32 =  (int) Math.Round( ( index1 - self.game.Data.RuleVar[223]));
            let mut num2: i32 =  (int) Math.Round( ( index1 + self.game.Data.RuleVar[223]));
            for (let mut index3: i32 =  num1; index3 <= num2; index3 += 1)
            {
              Coordinate coordinate;
              coordinate.x = index3;
              if (self.game.Data.MapObj[0].MapLoop & coordinate.x < 0)
                coordinate.x = self.game.Data.MapObj[0].MapWidth + coordinate.x + 1;
              if (self.game.Data.MapObj[0].MapLoop & coordinate.x > self.game.Data.MapObj[0].MapWidth)
                coordinate.x = coordinate.x - self.game.Data.MapObj[0].MapWidth - 1;
              ref Coordinate local = ref coordinate;
              let mut num3: i32 =  (int) Math.Round( ( index2 - self.game.Data.RuleVar[223]));
              let mut num4: i32 =  (int) Math.Round( ( index2 + self.game.Data.RuleVar[223]));
              for (local.y = num3; coordinate.y <= num4; coordinate += 1.y)
              {
                if (coordinate.x > -1 & coordinate.y > -1 && coordinate.x <= self.game.Data.MapObj[0].MapWidth & coordinate.y <= self.game.Data.MapObj[0].MapHeight && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == self.game.Data.Turn & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                {
                  let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter;
                  for (let mut index4: i32 =  0; index4 <= unitCounter; index4 += 1)
                  {
                    let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[index4];
                    if (self.game.Data.UnitObj[unit].AIPlanNr == plannr && self.game.Data.UnitObj[unit].AIUnitGoal == 5)
                    {
                      self.game.HandyFunctionsObj.MakeMovePrediction(unit, self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y, 0, false, PredictAirOnly: true, attack: true);
                      if (self.game.EditObj.TempValue[0].Value[index1, index2] <= self.game.HandyFunctionsObj.GetLowestAp(unit))
                      {
                        Number2 += self.GetForceLandStrength(unit, asattack: true, attackx: index1, attacky: index2);
                        simpleList.Add(unit, self.GetForceLandStrength(unit, asattack: true, attackx: index1, attacky: index2));
                        self.game.EditObj.TempUnitList.add(unit);
                      }
                    }
                  }
                }
              }
            }
            if (Number2 > 0)
            {
              self.game.TempCombat = new CombatClass(self.game);
              Coordinate Target = Coordinate::new();
              Target.x = index1;
              Target.y = index2;
              self.game.EditObj.TempUnitList = UnitList::new();
              self.AddLog("* Battle versus " + Conversion.Str( index1) + "," + Conversion.Str( index2) + " with " + Conversion.Str( Number2) + " vs " + Conversion.Str( Number1) + " force.");
              let mut counter: i32 =  simpleList.Counter;
              for (let mut index5: i32 =  0; index5 <= counter; index5 += 1)
                self.game.EditObj.TempUnitList.add(simpleList.Id[index5]);
              self.game.TempCombat.Init(Target, 1, self.game.EditObj.TempUnitList, 14);
              self.game.TempCombat.DoBattle();
              self.game.TempCombat.EndBattle();
              self.game.TempCombat = (CombatClass) null;
            }
          }
        }
      }
    }

    pub fn ExecuteArtilleryAttack(plannr: i32, float advneed, phase: i32)
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea && self.HexSA[index1, index2] == self.GetAreaNr(self.TPlanObj[plannr].TooArea) && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index1, index2].Regime) & self.game.Data.MapObj[0].HexObj[index1, index2].get_SeeNow(self.game.Data.Turn) > 0)
          {
            let mut Number1: i32 =  self.GetHexForceLandStrength(index1, index2);
            if (Number1 == 0)
              Number1 = 1;
            SimpleList simpleList = SimpleList::new();
            let mut Number2: i32 =  0;
            let mut num1: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].get_BattleStackArt(self.game.Data.Turn);
            self.game.EditObj.TempUnitList = UnitList::new();
            self.game.EditObj.OrderX = index1;
            self.game.EditObj.OrderY = index2;
            let mut num2: i32 =  index1 - 5;
            let mut num3: i32 =  index1 + 5;
            for (let mut index3: i32 =  num2; index3 <= num3; index3 += 1)
            {
              Coordinate coordinate1;
              coordinate1.x = index3;
              if (self.game.Data.MapObj[0].MapLoop & coordinate1.x < 0)
                coordinate1.x = self.game.Data.MapObj[0].MapWidth + coordinate1.x + 1;
              if (self.game.Data.MapObj[0].MapLoop & coordinate1.x > self.game.Data.MapObj[0].MapWidth)
                coordinate1.x = coordinate1.x - self.game.Data.MapObj[0].MapWidth - 1;
              ref Coordinate local = ref coordinate1;
              let mut num4: i32 =  index2 - 5;
              let mut num5: i32 =  index2 + 5;
              for (local.y = num4; coordinate1.y <= num5; coordinate1 += 1.y)
              {
                if (coordinate1.x > -1 & coordinate1.y > -1 && coordinate1.x <= self.game.Data.MapObj[0].MapWidth & coordinate1.y <= self.game.Data.MapObj[0].MapHeight && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].Regime, self.game.Data.Turn) & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LandscapeType].IsSea)
                {
                  let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].UnitCounter;
                  for (let mut index4: i32 =  0; index4 <= unitCounter; index4 += 1)
                  {
                    let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].UnitList[index4];
                    if (self.game.Data.UnitObj[unit].AIPlanNr == plannr & self.game.Data.UnitObj[unit].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unit].AIUnitGoal == 3 & self.game.HandyFunctionsObj.GetMaxArtRange(unit, 0) >= self.game.HandyFunctionsObj.Distance(coordinate1.x, coordinate1.y, 0, index1, index2, 0))
                    {
                      let mut num6: i32 =  1;
                      let mut tfacing: i32 =  1;
                      do
                      {
                        Coordinate coordinate2 = self.game.HandyFunctionsObj.HexNeighbour(coordinate1.x, coordinate1.y, 0, tfacing);
                        if (coordinate2.onmap && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].Regime) && self.GetHexForceLandStrength(coordinate2.x, coordinate2.y) > 0)
                        {
                          num6 = 0;
                          break;
                        }
                        tfacing += 1;
                      }
                      while (tfacing <= 6);
                      if (phase == 2)
                        num6 = 1;
                      if (num6 == 1 && 10 <= self.game.HandyFunctionsObj.GetLowestAp(unit))
                      {
                        Number2 += self.GetForceLandStrength(unit, asattack: true, attackx: index1, attacky: index2);
                        num1 += self.game.HandyFunctionsObj.GetUnitStackPtsArt(unit);
                        if (phase > 1 |  num1 <  self.game.Data.RuleVar[834] |  self.game.Data.RuleVar[834] < 1.0)
                        {
                          simpleList.Add(unit, self.GetForceLandStrength(unit, asattack: true, attackx: index1, attacky: index2));
                          self.game.EditObj.TempUnitList.add(unit);
                        }
                      }
                    }
                  }
                }
              }
            }
            if (Number2 > 0 & simpleList.Counter > -1)
            {
              self.game.TempCombat = new CombatClass(self.game);
              Coordinate Target = Coordinate::new();
              Target.x = index1;
              Target.y = index2;
              self.game.EditObj.TempUnitList = UnitList::new();
              self.AddLog("* Battle versus " + Conversion.Str( index1) + "," + Conversion.Str( index2) + " with " + Conversion.Str( Number2) + " vs " + Conversion.Str( Number1) + " force.");
              let mut counter: i32 =  simpleList.Counter;
              for (let mut index5: i32 =  0; index5 <= counter; index5 += 1)
                self.game.EditObj.TempUnitList.add(simpleList.Id[index5]);
              self.game.TempCombat.Init(Target, 1, self.game.EditObj.TempUnitList, 11);
              self.game.TempCombat.DoBattle();
              self.game.TempCombat.EndBattle();
              self.game.TempCombat = (CombatClass) null;
            }
          }
        }
      }
    }

    pub fn InitDecisions()
    {
      num1: i32;
      do
      {
        Number: i32;
        Number += 1;
        self.AddLog("INIT DECISION ROUND " + Conversion.Str( Number));
        num1 = 0;
        SimpleList simpleList = SimpleList::new();
        let mut locTypeCounter: i32 =  self.game.Data.LocTypeCounter;
        for (let mut index1: i32 =  0; index1 <= locTypeCounter; index1 += 1)
        {
          if (self.game.Data.LocTypeObj[index1].AICanBuild)
          {
            let mut num2: i32 =  1;
            let mut index2: i32 =  0;
            do
            {
              if (self.game.Data.LocTypeObj[index1].VarType[index2] > -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[self.game.Data.LocTypeObj[index1].VarType[index2]] < self.game.Data.LocTypeObj[index1].VarQty[index2])
                num2 = 0;
              if (self.game.Data.LocTypeObj[index1].Research[index2] > -1 && !self.game.Data.RegimeObj[self.game.Data.Turn].ResField[self.game.Data.LocTypeObj[index1].Research[index2]])
                num2 = 0;
              index2 += 1;
            }
            while (index2 <= 4);
            if (num2 == 1)
            {
              self.game.EditObj.AreaX = -1;
              self.game.EditObj.AreaY = -1;
              Coordinate locationForLocType;
              if (self.game.Data.LocTypeObj[index1].AILocEvent > -1)
              {
                self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.Data.LocTypeObj[index1].AILocEvent, index1);
                locationForLocType.x = self.game.EditObj.AreaX;
                locationForLocType.y = self.game.EditObj.AreaY;
              }
              else
                locationForLocType = self.AutoFindLocationForLocType(index1);
              if (locationForLocType.x > -1 && self.game.EditObj.TempValue[0].Value[locationForLocType.x, locationForLocType.y] == 1)
              {
                let mut aiPriority: i32 =  self.game.Data.LocTypeObj[index1].AIPriority;
                if (self.game.Data.LocTypeObj[index1].AIEvent > -1)
                {
                  self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.Data.LocTypeObj[index1].AIEvent, index1);
                  aiPriority += self.game.EditObj.AreaX;
                }
                if (aiPriority > 0)
                {
                  tid: i32;
                  tid += 1;
                  simpleList.Add(tid, aiPriority, 1, index1, locationForLocType.x, locationForLocType.y);
                  num1 = 1;
                }
              }
            }
          }
        }
        simpleList.Sort();
        let mut counter: i32 =  simpleList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          self.AddLog(Conversion.Str( simpleList.Id[index]) + ", weight=" + Conversion.Str( simpleList.Weight[index]) + " data= " + Conversion.Str( simpleList.Data1[index]) + "," + Conversion.Str( simpleList.Data2[index]) + "," + Conversion.Str( simpleList.Data3[index]) + ",");
        if (simpleList.Counter > -1)
        {
          let mut num3: i32 =  simpleList.Data1[simpleList.Counter];
          let mut index: i32 =  simpleList.Data2[simpleList.Counter];
          let mut num4: i32 =  simpleList.Data3[simpleList.Counter];
          let mut num5: i32 =  simpleList.Data4[simpleList.Counter];
          if (num3 == 1)
          {
            self.AddLog("CONSTRUCT====>build type " + self.game.Data.LocTypeObj[index].Name + " on hex " + Conversion.Str( num4) + "," + Conversion.Str( num5));
            self.game.ProcessingObj.Build(-1, num4, num5, 0, index, self.game.Data.Turn);
            if (self.game.Data.LocTypeObj[index].AIAfterBuildEvent > -1)
              self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.Data.LocTypeObj[index].AIAfterBuildEvent, index);
          }
        }
      }
      while (num1 == 1);
    }

    pub Coordinate AutoFindLocationForLocType(loctyp: i32)
    {
      let mut num1: i32 =  -1;
      let mut num2: i32 =  9999999;
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.game.HandyFunctionsObj.RedimTempValue(0);
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 =  0; x <= mapWidth1; x += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 =  0; y <= mapHeight; y += 1)
        {
          if (self.game.HandyFunctionsObj.HasHexRoad(x, y, 0) |  self.game.Data.RuleVar[864] < 1.0)
          {
            if (self.game.Data.MapObj[0].HexObj[x, y].Regime > -1)
              numArray[x, y] = !self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[x, y].Regime) ? (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[x, y].Regime) ? 6000 : (self.game.Data.Turn != self.game.Data.MapObj[0].HexObj[x, y].Regime ? 7500 : 0)) : 9000;
            else if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[x, y].LandscapeType].IsSea)
              numArray[x, y] = 6000;
          }
        }
      }
      let mut num3: i32 =  1;
      let mut num4: i32 =  0;
      Coordinate locationForLocType;
      while (num3 == 1 & num4 < 999)
      {
        num4 += 1;
        num3 = 0;
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            let mut tfacing: i32 =  1;
            do
            {
              locationForLocType = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
              if (locationForLocType.onmap &&  numArray[locationForLocType.x, locationForLocType.y] < Conversion.Int( numArray[cx, cy] * 0.9))
              {
                numArray[locationForLocType.x, locationForLocType.y] = (int) Math.Round(Conversion.Int( numArray[cx, cy] * 0.9));
                num3 = 1;
              }
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
        }
      }
      let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
      index1: i32;
      num5: i32;
      for (index1 = 0; index1 <= mapWidth3; index1 += 1)
      {
        let mut mapHeight1: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight1; index2 += 1)
        {
          let mut num6: i32 =  0;
          if (self.game.Data.MapObj[0].HexObj[index1, index2].Regime == self.game.Data.Turn && self.game.Data.MapObj[0].HexObj[index1, index2].Location == -1)
          {
            let mut buildGround: i32 =  self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].BuildGround;
            if (self.game.Data.LocTypeObj[loctyp].BuildgroundType[buildGround] && self.game.HandyFunctionsObj.IsHexNextToSea(index1, index2, 0) | !self.game.Data.LocTypeObj[loctyp].IsPort && self.game.HandyFunctionsObj.HasHexRoad(index1, index2, 0) |  self.game.Data.RuleVar[864] < 1.0)
            {
              bool flag = true;
              let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut x2: i32 =  0; x2 <= mapWidth4; x2 += 1)
              {
                let mut mapHeight2: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut y2: i32 =  0; y2 <= mapHeight2; y2 += 1)
                {
                  let mut location: i32 =  self.game.Data.MapObj[0].HexObj[x2, y2].Location;
                  if (location > -1)
                  {
                    let mut locTypeGroup: i32 =  self.game.Data.LocTypeObj[self.game.Data.LocObj[location].Type].LocTypeGroup;
                    let mut num7: i32 =  self.game.HandyFunctionsObj.Distance(index1, index2, 0, x2, y2, 0);
                    if (self.game.Data.LocTypeObj[loctyp].MinDistance[locTypeGroup] > -1 && self.game.Data.LocTypeObj[loctyp].MinDistance[locTypeGroup] > num7)
                      flag = false;
                  }
                }
              }
              if (self.game.Data.LocTypeObj[loctyp].SlotType > -1 && self.game.Data.MapObj[0].HexObj[index1, index2].AreaCode[self.game.Data.LocTypeObj[loctyp].SlotType] != self.game.Data.LocTypeObj[loctyp].SlotValue)
                flag = false;
              if (flag)
              {
                num6 = 1;
                if (numArray[index1, index2] < num2)
                {
                  num2 = numArray[index1, index2];
                  num1 = index1;
                  num5 = index2;
                }
              }
            }
          }
          self.game.EditObj.TempValue[0].Value[index1, index2] = num6 != 1 ? 0 : 1;
        }
      }
      locationForLocType.x = num1;
      locationForLocType.y = num5;
      locationForLocType.map = 0;
      locationForLocType.onmap = true;
      if (index1 == -1)
      {
        locationForLocType.onmap = false;
        locationForLocType.y = -1;
      }
      return locationForLocType;
    }

    pub fn InitRandomAI()
    {
      if ( self.game.Data.RuleVar[248] < 1.0)
        return;
      Random random = new Random((int) Math.Round( self.game.Data.GameID /  ((1 + self.game.Data.Turn) * 10) +  (self.game.Data.Turn * 100)));
      float num1 =  random.Next(1, 100) / 100f;
      float num2 =  random.Next(1, 100) / 100f;
      float num3 =  random.Next(1, 100) / 100f;
      float num4 =  random.Next(1, 100) / 100f;
      float num5 =  random.Next(1, 100) / 100f;
      if ( num1 < 0.33)
      {
        self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative =  ( num4 / 3.0 + 0.33);
        if ( self.game.Data.RuleVar[226] > 0.0)
          self.game.Data.RuleVar[226] =  (0.1 +  num3 / 5.0);
        if ( self.game.Data.RuleVar[501] == 1.0)
          self.game.Data.RuleVar[226] = 0.0f;
        self.game.Data.RuleVar[225] = 0.0f;
        self.AddLog("THIS IS AN OFFENSIVE AI");
      }
      else if ( num1 < 0.66)
      {
        self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative =  ( num4 / 4.0 + 0.88);
        if ( self.game.Data.RuleVar[226] > 0.0)
          self.game.Data.RuleVar[226] =  (0.1 +  num3 / 4.0);
        if ( self.game.Data.RuleVar[501] == 1.0)
          self.game.Data.RuleVar[226] = 0.0f;
        self.game.Data.RuleVar[225] = 0.0f;
        self.AddLog("THIS IS A NORMAL AI");
      }
      else
      {
        self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative =  ( num4 / 3.0 + 1.25);
        if ( self.game.Data.RuleVar[226] > 0.0)
          self.game.Data.RuleVar[226] =  (0.1 +  num3 / 3.0);
        if ( self.game.Data.RuleVar[501] == 1.0)
          self.game.Data.RuleVar[226] = 0.0f;
        self.game.Data.RuleVar[225] = 0.0f;
        self.AddLog("FRONTLINE FOCUS");
        self.AddLog("THIS IS A DEFENSIVE AI");
      }
      self.game.Data.RuleVar[224] =  (0.05 +  num5 / 6.0);
      self.AddLog("CONSERVATIVE = " + Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative));
      self.AddLog("AIR TO LAND RATIO = " + Conversion.Str( self.game.Data.RuleVar[224]));
      self.AddLog("RESEARCH = " + Conversion.Str( self.game.Data.RuleVar[226]));
      self.AddLog("-------------------------");
    }

    pub HasRegimeAir: bool(regnr: i32)
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 && self.game.Data.UnitObj[unr].Regime == regnr && self.game.HandyFunctionsObj.HasUnitAirSF(unr))
          return true;
      }
      return false;
    }

    pub fn InitStrategicHQAnalysis()
    {
      int[] numArray1 = new int[self.TPlanCount + 1];
      int[] numArray2 = new int[self.TPlanCount + 1];
      int[] numArray3 = new int[self.TPlanCount + 1];
      self.AddLog(" ");
      self.AddLog("STRATEGIC HQ ANALYSIS:");
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount1; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 20 && self.SAObj[self.GetSANr(self.TPlanObj[index1].FromArea)].LandReservePlan > 0)
        {
          let mut landReservePlan: i32 =  self.SAObj[self.GetSANr(self.TPlanObj[index1].FromArea)].LandReservePlan;
          int[] numArray4 = numArray1;
          int[] numArray5 = numArray4;
          let mut index2: i32 =  landReservePlan;
          let mut index3: i32 =  index2;
          let mut num1: i32 =  numArray4[index2] + self.TPlanObj[index1].WeightStrategic;
          numArray5[index3] = num1;
          int[] numArray6 = numArray2;
          int[] numArray7 = numArray6;
          let mut index4: i32 =  landReservePlan;
          let mut index5: i32 =  index4;
          let mut num2: i32 =  (int) Math.Round( ( numArray6[index4] + self.TPlanObj[index1].WeightFriendlyForce));
          numArray7[index5] = num2;
          int[] numArray8 = numArray3;
          int[] numArray9 = numArray8;
          let mut index6: i32 =  landReservePlan;
          let mut index7: i32 =  index6;
          let mut num3: i32 =  (int) Math.Round( ( numArray8[index6] + self.TPlanObj[index1].WeightEnemyForceUnMod));
          numArray9[index7] = num3;
        }
        if (self.TPlanObj[index1].Type == 40 && self.SAObj[self.GetSANr(self.TPlanObj[index1].FromArea)].LandReservePlan > 0)
        {
          if (self.TPlanObj[index1].SeaTarget > 0)
          {
            let mut landReservePlan: i32 =  self.SAObj[self.GetSANr(self.TPlanObj[index1].FromArea)].LandReservePlan;
            int[] numArray10 = numArray1;
            int[] numArray11 = numArray10;
            let mut index8: i32 =  landReservePlan;
            let mut index9: i32 =  index8;
            let mut num4: i32 =  numArray10[index8] + self.TPlanObj[index1].WeightStrategic;
            numArray11[index9] = num4;
            int[] numArray12 = numArray2;
            int[] numArray13 = numArray12;
            let mut index10: i32 =  landReservePlan;
            let mut index11: i32 =  index10;
            let mut num5: i32 =  (int) Math.Round( ( numArray12[index10] + self.TPlanObj[index1].WeightFriendlyForce));
            numArray13[index11] = num5;
            int[] numArray14 = numArray3;
            int[] numArray15 = numArray14;
            let mut index12: i32 =  landReservePlan;
            let mut index13: i32 =  index12;
            let mut num6: i32 =  (int) Math.Round( ( numArray14[index12] + self.TPlanObj[index1].WeightEnemyForceUnMod));
            numArray15[index13] = num6;
            int[] numArray16 = numArray2;
            int[] numArray17 = numArray16;
            let mut index14: i32 =  landReservePlan;
            let mut index15: i32 =  index14;
            let mut num7: i32 =  numArray16[index14] + self.TPlanObj[index1].FriendlyNavy;
            numArray17[index15] = num7;
            int[] numArray18 = numArray3;
            int[] numArray19 = numArray18;
            let mut index16: i32 =  landReservePlan;
            let mut index17: i32 =  index16;
            let mut num8: i32 =  numArray18[index16] + self.TPlanObj[index1].EnemyNavy;
            numArray19[index17] = num8;
          }
          else
          {
            let mut landReservePlan: i32 =  self.SAObj[self.GetSANr(self.TPlanObj[index1].FromArea)].LandReservePlan;
            int[] numArray20 = numArray1;
            int[] numArray21 = numArray20;
            let mut index18: i32 =  landReservePlan;
            let mut index19: i32 =  index18;
            let mut num9: i32 =  numArray20[index18] + self.TPlanObj[index1].WeightStrategic;
            numArray21[index19] = num9;
            int[] numArray22 = numArray2;
            int[] numArray23 = numArray22;
            let mut index20: i32 =  landReservePlan;
            let mut index21: i32 =  index20;
            let mut num10: i32 =  numArray22[index20] + self.TPlanObj[index1].FriendlyNavy;
            numArray23[index21] = num10;
            int[] numArray24 = numArray3;
            int[] numArray25 = numArray24;
            let mut index22: i32 =  landReservePlan;
            let mut index23: i32 =  index22;
            let mut num11: i32 =  numArray24[index22] + self.TPlanObj[index1].EnemyNavy;
            numArray25[index23] = num11;
          }
        }
      }
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index24: i32 =  1; index24 <= tplanCount2; index24 += 1)
      {
        if (self.TPlanObj[index24].Type == 30)
        {
          let mut num12: i32 =  numArray1[index24] * 10;
          if (numArray2[index24] == 0)
            numArray2[index24] = 1;
          let mut num13: i32 =  (int) Math.Round( num12 * ( numArray3[index24] /  numArray2[index24]));
          self.TPlanObj[index24].WeightStrategic = num13;
          tplanObj: Vec<AIPlanClass> = self.TPlanObj;
          aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
          let mut index25: i32 =  index24;
          let mut index26: i32 =  index25;
          aiPlanClassArray[index26].WeightStrategic = tplanObj[index25].WeightStrategic + 100;
        }
      }
    }

    pub fn InitShowStats()
    {
      self.AddLog(" ");
      self.AddLog("PLAN STATS::");
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut Number: i32 =  1; Number <= tplanCount1; Number += 1)
      {
        if (self.TPlanObj[Number].Type == 20 | self.TPlanObj[Number].Type == 50)
        {
          self.AddLog(" ");
          self.AddLog("*Plan " + Strings.Trim(Conversion.Str( Number)) + ": ");
          str1: String = "";
          if (self.TPlanObj[Number].Type == 20)
            str1 = "LANDFRONT".to_owned();
          if (self.TPlanObj[Number].Type == 50)
            str1 = "OLD-LANDFRONT";
          self.AddLog(str1 + " from " + self.game.HandyFunctionsObj.GetHexName(self.TPlanObj[Number].FromArea.X, self.TPlanObj[Number].FromArea.Y, 0) + "(" + Strings.Trim(Conversion.Str( self.TPlanObj[Number].FromArea.X)) + "," + Strings.Trim(Conversion.Str( self.TPlanObj[Number].FromArea.Y)) + ")" + " to " + self.game.HandyFunctionsObj.GetHexName(self.TPlanObj[Number].TooArea.X, self.TPlanObj[Number].TooArea.Y, 0) + "(" + Strings.Trim(Conversion.Str( self.TPlanObj[Number].TooArea.X)) + "," + Strings.Trim(Conversion.Str( self.TPlanObj[Number].TooArea.Y)) + "" + ", strategic-weight: " + Conversion.Str( self.TPlanObj[Number].WeightStrategic));
          self.AddLog("Friendly Force = " + Conversion.Str( self.TPlanObj[Number].WeightFriendlyForce) + ", Enemy Force = " + Conversion.Str( self.TPlanObj[Number].WeightEnemyForce) + ", Enemy UnMod = " + Conversion.Str( self.TPlanObj[Number].WeightEnemyForceUnMod));
          str2: String = "Units: ";
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (self.game.Data.UnitObj[index].AIPlanNr == Number & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
            {
              str3: String = str2 + self.game.Data.UnitObj[index].Name;
              if (self.game.Data.UnitObj[index].AIUnitGoal == 1)
                str3 += "(INF)";
              if (self.game.Data.UnitObj[index].AIUnitGoal == 2)
                str3 += "(ARM)";
              if (self.game.Data.UnitObj[index].AIUnitGoal == 3)
                str3 += "(ART)";
              if (self.game.Data.UnitObj[index].AIUnitGoal == 4)
                str3 += "(ENG)";
              if (self.game.Data.UnitObj[index].AIMobilize)
                str3 += "(MOB)";
              if (self.game.Data.UnitObj[index].AIReserve)
                str3 += "(RESRV)";
              str2 = str3 + ", ";
            }
          }
          self.AddLog(str2 + " (count: F=" + Conversion.Str( self.TPlanObj[Number].FriendlyUnitCount) + "/E=" + Conversion.Str( self.TPlanObj[Number].EnemyUnitCount) + ")");
          s: String = "Stand: ";
          if (self.TPlanObj[Number].Stand == 1)
            s += "ATTACK".to_owned();
          if (self.TPlanObj[Number].Stand == 2)
            s += "DEFEND".to_owned();
          if (self.TPlanObj[Number].Stand == 3)
            s += "RETREAT".to_owned();
          self.AddLog(s);
          if (self.TPlanObj[Number].RiverLine == 1)
            self.AddLog("RIVERLINE DEFEND PLAN");
          self.AddLog("CurrentApCost: " + Conversion.Str( self.TPlanObj[Number].CurrentAPCost) + ", PossibleApCost: " + Conversion.Str( self.TPlanObj[Number].PossibleAPCost) + ", AverageunitAPDistance =" + Conversion.Str( self.TPlanObj[Number].AverageUnitAPCost));
          str4: String = "HQ: ";
          self.AddLog(self.TPlanObj[Number].HQ != -1 ? str4 + self.game.Data.UnitObj[self.TPlanObj[Number].HQ].Name : str4 + "n/a");
        }
      }
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount2; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 40)
        {
          self.AddLog(" ");
          self.AddLog("*Plan " + Strings.Trim(Conversion.Str( index1)) + ": ");
          self.AddLog("PLANBACK" + " from " + self.game.HandyFunctionsObj.GetHexName(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y, 0) + "(" + Strings.Trim(Conversion.Str( self.TPlanObj[index1].FromArea.X)) + "," + Strings.Trim(Conversion.Str( self.TPlanObj[index1].FromArea.Y)) + ")" + ", strategic-weight: " + Conversion.Str( self.TPlanObj[index1].WeightStrategic));
          self.AddLog("Friendly Force = " + Conversion.Str( self.TPlanObj[index1].WeightFriendlyForce) + ", Enemy Force = " + Conversion.Str( self.TPlanObj[index1].WeightEnemyForce) + ", Enemy UnMod = " + Conversion.Str( self.TPlanObj[index1].WeightEnemyForceUnMod));
          self.AddLog("Friendly Naval Force = " + Conversion.Str( self.TPlanObj[index1].FriendlyNavy) + ", Enemy Naval Force = " + Conversion.Str( self.TPlanObj[index1].EnemyNavy));
          s: String = "Units: ";
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
          {
            if (self.game.Data.UnitObj[index2].AIPlanNr == index1 & self.game.Data.UnitObj[index2].Regime == self.game.Data.Turn)
            {
              str: String = s + self.game.Data.UnitObj[index2].Name;
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 1)
                str += "(INF)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 2)
                str += "(ARM)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 3)
                str += "(ART)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 4)
                str += "(ENG)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 5)
                str += "(AIR)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 9)
                str += "(NAVWAR)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 8)
                str += "(CARGO)";
              if (self.game.Data.UnitObj[index2].AIUnitGoal == 10)
                str += "(RAIDER)";
              if (self.game.Data.UnitObj[index2].AIReserve)
                str += "(RESRV)";
              s = str + ", ";
            }
          }
          self.AddLog(s);
          if (self.TPlanObj[index1].SeaStand > 0)
          {
            if (self.TPlanObj[index1].SeaStand == 4)
              s = "Sea Stand: HOME";
            if (self.TPlanObj[index1].SeaStand == 5)
              s = "Sea Stand: RAID";
            if (self.TPlanObj[index1].SeaStand == 6)
              s = "Sea Stand: SEASUP";
            if (self.TPlanObj[index1].SeaStand == 7)
              s = "Sea Stand: AMPH";
          }
          else
            s = "No Sea Stand";
          self.AddLog(s);
          self.AddLog(self.TPlanObj[index1].SeaTarget <= 0 ? "No Sea Target" : "Sea Target is Area# " + Conversion.Str( self.TPlanObj[index1].SeaTarget));
          if (self.MakeNavyActive(index1))
            self.AddLog("Navy is ACTIVE");
          else
            self.AddLog("Navy is NOT active");
          if (self.TPlanObj[index1].AssemblyArea == 1)
            self.AddLog("ASSEMBLY AREA FOR DEFENSE IN DEPTH");
          self.AddLog("CurrentApCost: " + Conversion.Str( self.TPlanObj[index1].CurrentAPCost) + ", PossibleApCost: " + Conversion.Str( self.TPlanObj[index1].PossibleAPCost) + ", AverageunitAPDistance =" + Conversion.Str( self.TPlanObj[index1].AverageUnitAPCost));
          self.AddLog("CurrentBackRoad to Area: " + Conversion.Str( self.TPlanObj[index1].CurrentBackRoad));
          str5: String = "HQ: ";
          self.AddLog(self.TPlanObj[index1].HQ != -1 ? str5 + self.game.Data.UnitObj[self.TPlanObj[index1].HQ].Name : str5 + "n/a");
        }
      }
      self.AddLog("");
      self.AddLog("LANDRESERVE PLANS:");
      let mut tplanCount3: i32 =  self.TPlanCount;
      for (let mut Number1: i32 =  1; Number1 <= tplanCount3; Number1 += 1)
      {
        if (self.TPlanObj[Number1].Type == 30)
        {
          self.AddLog(" ");
          self.AddLog("*Plan " + Strings.Trim(Conversion.Str( Number1)) + ": ");
          str6: String = "";
          if (self.TPlanObj[Number1].Type == 20)
            str6 = "LANDRESERVE".to_owned();
          self.AddLog(str6 + " at " + self.game.HandyFunctionsObj.GetHexName(self.TPlanObj[Number1].FromArea.X, self.TPlanObj[Number1].FromArea.Y, 0) + "(" + Strings.Trim(Conversion.Str( self.TPlanObj[Number1].FromArea.X)) + "," + Strings.Trim(Conversion.Str( self.TPlanObj[Number1].FromArea.Y)) + ")");
          str7: String = "Units: ";
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (self.game.Data.UnitObj[index].AIPlanNr == Number1 & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
              str7 = str7 + self.game.Data.UnitObj[index].Name + ", ";
          }
          self.AddLog(str7 + " (count=" + Conversion.Str( self.TPlanObj[Number1].FriendlyUnitCount) + ")");
          s: String = "Areas: ";
          let mut saCount: i32 =  self.SACount;
          for (let mut Number2: i32 =  1; Number2 <= saCount; Number2 += 1)
          {
            if (self.SAObj[Number2].LandReservePlan == Number1)
              s = s + self.game.HandyFunctionsObj.GetHexName(self.SAObj[Number2].X, self.SAObj[Number2].Y, 0) + "(#" + Conversion.Str( Number2) + "), ";
          }
          self.AddLog(s);
          str8: String = "HQ: ";
          self.AddLog(self.TPlanObj[Number1].HQ != -1 ? str8 + self.game.Data.UnitObj[self.TPlanObj[Number1].HQ].Name : str8 + "n/a");
          self.AddLog("ProdPts Total: " + Conversion.Str( self.TPlanObj[Number1].ProdPts) + " , Strategic Weight: " + Conversion.Str( self.TPlanObj[Number1].WeightStrategic));
          self.AddLog("CurrentHighestApCost: " + Conversion.Str( self.TPlanObj[Number1].CurrentAPCost) + ", PossibleHighestApCost: " + Conversion.Str( self.TPlanObj[Number1].PossibleAPCost));
          self.AddLog("MetaChainNr: " + Conversion.Str( self.TPlanObj[Number1].MetaChainNr));
        }
      }
    }

    pub fn GetPlanEPPerTurn(plannr: i32) -> i32
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      integer: i32;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].X > -1 && self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr == plannr)
          integer = Conversions.ToInteger(Operators.AddObject( integer, self.GetEPPerTurn(unr)));
      }
      return integer;
    }

    pub fn InitUnitGoals(let mut specificunit: i32 =  -1)
    {
      self.AddLog("");
      self.AddLog("Assign UnitGoals:");
      let mut num1: i32 =  -1;
      if (specificunit > -1)
        num1 = self.game.Data.UnitObj[specificunit].AIPlanNr;
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount1; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 20 | self.TPlanObj[index1].Type == 40 & (index1 == num1 | num1 == -1))
        {
          self.PlanEngineerNeedScore(index1);
          let mut num2: i32 =  0;
          let mut num3: i32 =  0;
          let mut num4: i32 =  0;
          let mut num5: i32 =  0;
          let mut num6: i32 =  0;
          let mut num7: i32 =  0;
          let mut num8: i32 =  0;
          let mut num9: i32 =  0;
          let mut num10: i32 =  0;
          let mut num11: i32 =  0;
          let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
          for (let mut index2: i32 =  0; index2 <= unitCounter1; index2 += 1)
          {
            if (!self.game.Data.UnitObj[index2].IsHQ && self.game.Data.UnitObj[index2].AIPlanNr == index1 & self.game.Data.UnitObj[index2].Regime == self.game.Data.Turn)
            {
              if (self.game.Data.UnitObj[index2].AIUnitGoal > 0)
              {
                let mut aiUnitGoal: i32 =  self.game.Data.UnitObj[index2].AIUnitGoal;
                if (aiUnitGoal == 1)
                {
                  num2 += 1;
                  num6 += 1;
                }
                if (aiUnitGoal == 2)
                {
                  num3 += 1;
                  num6 += 1;
                }
                if (aiUnitGoal == 3)
                {
                  num4 += 1;
                  num6 += 1;
                }
                if (aiUnitGoal == 4)
                {
                  num5 += 1;
                  num6 += 1;
                }
                if (aiUnitGoal == 10)
                {
                  num9 += 1;
                  num8 += 1;
                }
                if (aiUnitGoal == 9)
                {
                  num10 += 1;
                  num8 += 1;
                }
                if (aiUnitGoal == 8)
                {
                  num11 += 1;
                  num8 += 1;
                }
              }
              num7 += 1;
            }
          }
          if (num6 == 0)
            num6 = 1;
          if (num8 == 0)
            num8 = 1;
          let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
          num12: i32;
          bool Number;
          str: String;
          for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
          {
            if (self.game.Data.Round == 1 | self.game.Data.UnitObj[unr].AIUnitGoal < 1 && !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIUnitGoal == 0 && self.game.Data.UnitObj[unr].AIPlanNr == index1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && unr == specificunit | specificunit == -1)
            {
              num12 = -1;
              Number = false;
              if ( self.GetRolePercent(unr, 10) >=  self.game.Data.RuleVar[157] * 0.6)
              {
                num3 += 1;
                num6 += 1;
                num12 = 2;
                str = "ARMOUR".to_owned();
                if (self.TPlanObj[index1].Stand == 1)
                {
                  if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[154])
                    Number = true;
                }
                else if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[153])
                  Number = true;
                if ( self.game.Data.RuleVar[164] == 1.0)
                  Number = true;
                if (!Number && Operators.ConditionalCompareObjectGreaterEqual( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))
                  Number = true;
              }
              else if (self.GetRolePercent(unr, 5) >= 50)
              {
                num5 += 1;
                num6 += 1;
                num12 = 4;
                str = "ENGINEER".to_owned();
                if ( self.game.Data.RuleVar[214] == 1.0)
                  Number = true;
              }
              else if ( self.GetRolePercent(unr, 8) >=  self.game.Data.RuleVar[158] * 0.3)
              {
                num4 += 1;
                num6 += 1;
                num12 = 3;
                str = "ART".to_owned();
                if (self.TPlanObj[index1].Stand == 1)
                {
                  if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[154])
                    Number = true;
                }
                else if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[153])
                  Number = true;
                if ( self.game.Data.RuleVar[165] == 1.0)
                  Number = true;
                if (!Number && Operators.ConditionalCompareObjectGreaterEqual( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))
                  Number = true;
              }
              else if ( self.GetRolePercent(unr, 6) >=  self.game.Data.RuleVar[156] * 0.8)
              {
                num2 += 1;
                num6 += 1;
                num12 = 1;
                str = "INF".to_owned();
                if (self.TPlanObj[index1].Stand == 1)
                {
                  if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[154])
                    Number = true;
                }
                else if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[153])
                  Number = true;
                if (!Number && Operators.ConditionalCompareObjectGreaterEqual( self.game.HandyFunctionsObj.GetUnitCarryCap(unr, 0), self.game.HandyFunctionsObj.GetUnitWeightWithoutLandCarryCap(unr), false))
                  Number = true;
              }
              else if (self.GetRolePercent(unr, 17) > 0)
              {
                num12 = 8;
                str = "CARGO".to_owned();
              }
              else if (self.GetRolePercent(unr, 19) > 30)
              {
                num12 = 10;
                str = "RAIDER".to_owned();
              }
              else if (self.GetRolePercent(unr, 18) > 0)
              {
                num12 = 9;
                str = "NAVALWAR".to_owned();
              }
              else if (self.game.HandyFunctionsObj.HasUnitAirSF(unr))
              {
                num12 = 5;
                str = "AIRSUPPORT".to_owned();
                self.game.Data.UnitObj[unr].SOInterceptRdnStop = 75;
              }
              if (num12 > -1)
              {
                if (self.TPlanObj[index1].Type == 20 && self.game.Data.MapObj[0].HexObj[self.TPlanObj[index1].TooArea.X, self.TPlanObj[index1].TooArea.Y].Regime == -1 |  self.TPlanObj[index1].WeightFriendlyForce >  self.TPlanObj[index1].WeightEnemyForce * 4.0)
                  Number = true;
                self.game.Data.UnitObj[unr].AIUnitGoal = num12;
                self.game.Data.UnitObj[unr].AIMobilize = Number;
                self.AddLog("AFTER ANALYSE OF COMPOSITION Unit: " + self.game.Data.UnitObj[unr].Name + " has been assigned goal: " + str + ", mobilize=" + Conversion.Str( Number));
              }
            }
          }
          let mut unitCounter3: i32 =  self.game.Data.UnitCounter;
          for (let mut unr: i32 =  0; unr <= unitCounter3; unr += 1)
          {
            if (specificunit == -1 | unr == specificunit)
            {
              if (!self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIUnitGoal == 0 && self.game.Data.UnitObj[unr].AIPlanNr == index1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn)
              {
                if (self.TPlanObj[index1].Type == 20)
                {
                  num12 = 1;
                  Number = false;
                  str = "INFANTRY".to_owned();
                  if (self.TPlanObj[index1].Stand == 2)
                  {
                    if ( self.TPlanObj[index1].WeightEnemyForce < 5.0)
                    {
                      num12 = 2;
                      str = "ARMOUR".to_owned();
                      num2 += 1;
                      num6 += 1;
                      Number = true;
                    }
                    else if ( (num2 * 100) /  num6 <  self.game.Data.RuleVar[162])
                    {
                      num12 = 1;
                      str = "INFANTRY ";
                      num2 += 1;
                      num6 += 1;
                      if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[153])
                        Number = true;
                    }
                    else if ( (num3 * 100) /  num6 <  self.game.Data.RuleVar[161] & 100.0 * (1.0 /  num7) <=  self.game.Data.RuleVar[161])
                    {
                      num12 = 2;
                      str = "ARMOUR".to_owned();
                      num3 += 1;
                      num6 += 1;
                      if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[153])
                        Number = true;
                      if ( self.game.Data.RuleVar[164] == 1.0)
                        Number = true;
                    }
                    else if ( (num4 * 100) /  num6 <  self.game.Data.RuleVar[163] & 100.0 * (1.0 /  num7) <=  self.game.Data.RuleVar[163])
                    {
                      num12 = 3;
                      str = "ARTILLERY".to_owned();
                      num4 += 1;
                      num6 += 1;
                      if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[153])
                        Number = true;
                      if ( self.game.Data.RuleVar[165] == 1.0)
                        Number = true;
                    }
                  }
                  else if (self.TPlanObj[index1].Stand == 1)
                  {
                    if ( (num3 * 100) /  num6 <  self.game.Data.RuleVar[171] & 100.0 * (1.0 /  num7) <=  self.game.Data.RuleVar[171])
                    {
                      num12 = 2;
                      str = "ARMOUR".to_owned();
                      num3 += 1;
                      num6 += 1;
                      if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[154])
                        Number = true;
                      if ( self.game.Data.RuleVar[164] == 1.0)
                        Number = true;
                    }
                    else if ( (num2 * 100) /  num6 <  self.game.Data.RuleVar[172])
                    {
                      num12 = 1;
                      str = "INFANTRY".to_owned();
                      num2 += 1;
                      num6 += 1;
                      if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[154])
                        Number = true;
                    }
                    else if ( (num4 * 100) /  num6 <  self.game.Data.RuleVar[173] & 100.0 * (1.0 /  num7) <=  self.game.Data.RuleVar[173])
                    {
                      num12 = 3;
                      str = "ARTILLERY".to_owned();
                      num4 += 1;
                      num6 += 1;
                      if ( VBMath.Rnd() * 100.0 <=  self.game.Data.RuleVar[154])
                        Number = true;
                      if ( self.game.Data.RuleVar[165] == 1.0)
                        Number = true;
                    }
                  }
                }
                else if (self.TPlanObj[index1].Type == 40)
                {
                  bool flag = self.MakeNavyActive(index1);
                  if (!self.game.HandyFunctionsObj.HasUnitlandSF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) & flag & self.TPlanObj[index1].SeaStand == 5 & self.game.HandyFunctionsObj.IsHexHarbourOrSea(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0))
                  {
                    num12 = 9;
                    Number = false;
                    str = "NAVALWAR".to_owned();
                    if ( (num11 * 100) /  num8 <  self.game.Data.RuleVar[232])
                    {
                      num12 = 8;
                      str = "CARGO".to_owned();
                      num11 += 1;
                      num8 += 1;
                    }
                    else if ( (num10 * 100) /  num8 <  self.game.Data.RuleVar[231])
                    {
                      num12 = 9;
                      str = "NAVWAR".to_owned();
                      num10 += 1;
                      num8 += 1;
                    }
                    else if ( (num9 * 100) /  num8 <  self.game.Data.RuleVar[230])
                    {
                      num12 = 10;
                      str = "RAIDER".to_owned();
                      num9 += 1;
                      num8 += 1;
                    }
                  }
                  else if (!self.game.HandyFunctionsObj.HasUnitlandSF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) & flag & self.TPlanObj[index1].SeaStand == 6 & self.game.HandyFunctionsObj.IsHexHarbourOrSea(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0))
                  {
                    num12 = 9;
                    Number = false;
                    str = "NAVALWAR".to_owned();
                    if ( (num11 * 100) /  num8 <  self.game.Data.RuleVar[235])
                    {
                      num12 = 8;
                      str = "CARGO".to_owned();
                      num11 += 1;
                      num8 += 1;
                    }
                    else if ( (num10 * 100) /  num8 <  self.game.Data.RuleVar[234])
                    {
                      num12 = 9;
                      str = "NAVWAR".to_owned();
                      num10 += 1;
                      num8 += 1;
                    }
                    else if ( (num9 * 100) /  num8 <  self.game.Data.RuleVar[233])
                    {
                      num12 = 10;
                      str = "RAIDER".to_owned();
                      num9 += 1;
                      num8 += 1;
                    }
                  }
                  else if (!self.game.HandyFunctionsObj.HasUnitlandSF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) & flag & self.TPlanObj[index1].SeaStand == 7 & self.game.HandyFunctionsObj.IsHexHarbourOrSea(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0))
                  {
                    num12 = 8;
                    Number = false;
                    str = "NAVALWAR".to_owned();
                    if ( (num11 * 100) /  num8 <  self.game.Data.RuleVar[238])
                    {
                      num12 = 8;
                      str = "CARGO".to_owned();
                      num11 += 1;
                      num8 += 1;
                    }
                    else if ( (num10 * 100) /  num8 <  self.game.Data.RuleVar[237])
                    {
                      num12 = 9;
                      str = "NAVWAR".to_owned();
                      num10 += 1;
                      num8 += 1;
                    }
                    else if ( (num9 * 100) /  num8 <  self.game.Data.RuleVar[236])
                    {
                      num12 = 10;
                      str = "RAIDER".to_owned();
                      num9 += 1;
                      num8 += 1;
                    }
                    if (self.TPlanObj[index1].SeaTarget > 0 & self.TPlanObj[index1].SeaTarget <= self.SACount)
                    {
                      if (self.game.Data.MapObj[0].HexObj[self.SAObj[self.TPlanObj[index1].SeaTarget].X, self.SAObj[self.TPlanObj[index1].SeaTarget].Y].Regime == -1)
                      {
                        num12 = 8;
                        str = "CARGO".to_owned();
                      }
                    }
                    else
                      self.TPlanObj[index1].SeaStand = 0;
                  }
                  else if (!self.game.HandyFunctionsObj.HasUnitlandSF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) & flag & self.TPlanObj[index1].SeaStand == 4 & self.game.HandyFunctionsObj.IsHexHarbourOrSea(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0))
                  {
                    num12 = 9;
                    Number = false;
                    str = "NAVALWAR".to_owned();
                    if ( (num11 * 100) /  num8 <  self.game.Data.RuleVar[242])
                    {
                      num12 = 8;
                      str = "CARGO".to_owned();
                      num11 += 1;
                      num8 += 1;
                    }
                    else if ( (num10 * 100) /  num8 <  self.game.Data.RuleVar[241])
                    {
                      num12 = 9;
                      str = "NAVWAR".to_owned();
                      num10 += 1;
                      num8 += 1;
                    }
                    else if ( (num9 * 100) /  num8 <  self.game.Data.RuleVar[240])
                    {
                      num12 = 10;
                      str = "RAIDER".to_owned();
                      num9 += 1;
                      num8 += 1;
                    }
                  }
                }
                self.game.Data.UnitObj[unr].AIUnitGoal = num12;
                self.game.Data.UnitObj[unr].AIMobilize = Number;
                self.AddLog("Unit: " + self.game.Data.UnitObj[unr].Name + " has been assigned goal: " + str + ", mobilize=" + Conversion.Str( Number));
              }
              if ( self.game.Data.RuleVar[211] > 0.0 &  self.game.Data.RuleVar[32] > -1.0 && !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIPlanNr == index1 && self.game.Data.UnitObj[unr].AIUnitGoal != 4 && self.game.HandyFunctionsObj.GetUnitEP(unr) >= self.game.Data.RoadTypeObj[(int) Math.Round( self.game.Data.RuleVar[32])].EPCost &&  self.GetRolePercent(unr, 5) > 1.0 +  VBMath.Rnd() * 100.0)
              {
                str = "ENGINEER".to_owned();
                num12 = 4;
                self.game.Data.UnitObj[unr].AIUnitGoal = num12;
                num5 += 1;
                self.AddLog("Unit: " + self.game.Data.UnitObj[unr].Name + " has been re-assigned goal: " + str);
              }
              if ( self.game.Data.RuleVar[221] > 0.0 && !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIPlanNr == index1 && self.TPlanObj[index1].Type == 40 && self.game.HandyFunctionsObj.HasUnitAirSF(unr) & !self.game.HandyFunctionsObj.HasUnitNavySF(unr) && self.game.Data.UnitObj[unr].AIUnitGoal != 5)
              {
                str = "AIRSUPPORT".to_owned();
                num12 = 5;
                self.game.Data.UnitObj[unr].AIUnitGoal = num12;
                self.game.Data.UnitObj[unr].SOInterceptRdnStop = 75;
                self.AddLog("Unit: " + self.game.Data.UnitObj[unr].Name + " has been re-assigned goal: " + str);
              }
              if ( self.game.Data.RuleVar[227] > 0.0 && !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIPlanNr == index1 && self.TPlanObj[index1].Type == 40 && self.MakeNavyActive(index1) && self.game.HandyFunctionsObj.HasUnitNavySF(unr) && !(self.game.Data.UnitObj[unr].AIUnitGoal == 10 | self.game.Data.UnitObj[unr].AIUnitGoal == 8 | self.game.Data.UnitObj[unr].AIUnitGoal == 9) && self.game.HandyFunctionsObj.IsHexHarbourOrSea(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0))
              {
                str = "GOALNAVALWAR".to_owned();
                num12 = 9;
                self.game.Data.UnitObj[unr].AIUnitGoal = num12;
                self.AddLog("Unit: " + self.game.Data.UnitObj[unr].Name + " has been re-assigned goal: " + str);
              }
            }
          }
        }
        if ((self.TPlanObj[index1].Type == 20 | self.TPlanObj[index1].Type == 40) & num1 == -1)
        {
          let mut Number: i32 =  self.PlanEngineerNeedScore(index1);
          num13: i32;
          if (self.TPlanObj[index1].Type == 20)
            num13 = (int) Math.Round( self.game.Data.RuleVar[216]);
          if (self.TPlanObj[index1].Type == 40)
            num13 = (int) Math.Round( self.game.Data.RuleVar[217]);
          let mut num14: i32 =  0;
          let mut num15: i32 =  0;
          let mut num16: i32 =  0;
          let mut num17: i32 =  0;
          let mut num18: i32 =  0;
          let mut unitCounter4: i32 =  self.game.Data.UnitCounter;
          for (let mut unr: i32 =  0; unr <= unitCounter4; unr += 1)
          {
            if (!self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIUnitGoal > 0 && self.game.Data.UnitObj[unr].AIPlanNr == index1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn)
            {
              let mut aiUnitGoal: i32 =  self.game.Data.UnitObj[unr].AIUnitGoal;
              if (aiUnitGoal == 1)
                num14 += 1;
              if (aiUnitGoal == 2)
                num15 += 1;
              if (aiUnitGoal == 3)
                num16 += 1;
              if (aiUnitGoal == 4)
                num17 = Conversions.ToInteger(Operators.AddObject( num17, self.GetEPPerTurn(unr)));
              num18 += 1;
            }
          }
          self.AddLog("Plan #" + Conversion.Str( index1) + " engineer need score =" + Conversion.Str( Number) + ", geng=" + Conversion.Str( num17));
          if ( self.game.Data.RuleVar[211] > 0.0 & !(num18 == 0 & self.TPlanObj[index1].Type == 20 & self.TPlanObj[index1].EnemyUnitCount > 0))
          {
            if (Number > num13 &  num17 <  self.game.Data.RuleVar[215] * 2.0)
            {
              SimpleList simpleList = SimpleList::new();
              let mut tplanCount2: i32 =  self.TPlanCount;
              for (let mut index3: i32 =  1; index3 <= tplanCount2; index3 += 1)
              {
                if (index1 != index3 & self.HasPlanEngineerUnit(index3) > -1)
                {
                  let mut num19: i32 =  self.PlanEngineerNeedScore(index3);
                  if (num19 < num13)
                  {
                    let mut num20: i32 =  1;
                    num21: i32;
                    if (self.GetAreaNr(self.TPlanObj[index3].FromArea) == self.GetAreaNr(self.TPlanObj[index1].FromArea))
                    {
                      num21 = 1;
                    }
                    else
                    {
                      let mut num22: i32 =  self.AreaDistance(self.GetAreaNr(self.TPlanObj[index3].FromArea), self.GetAreaNr(self.TPlanObj[index1].FromArea), true);
                      num21 = num22 != 0 ? num22 * num22 : 9999;
                    }
                    let mut num23: i32 =  num19 * num21;
                    if (self.TPlanObj[index3].Type == 40 & self.TPlanObj[index3].CurrentBackRoad > 0)
                      num23 = 9999;
                    num24: i32;
                    if (self.TPlanObj[index3].Type == 20)
                      num24 = (int) Math.Round( self.game.Data.RuleVar[216]);
                    if (self.TPlanObj[index3].Type == 40)
                      num24 = (int) Math.Round( self.game.Data.RuleVar[217]);
                    if (self.PlanEngineerNeedScore(index3) > num24 &  num17 >=  self.game.Data.RuleVar[215])
                      num20 = 0;
                    if (self.PlanEngineerNeedScore(index3) > num24 &  self.GetPlanEPPerTurn(index3) <  self.game.Data.RuleVar[215])
                      num20 = 0;
                    if (num20 == 1 & num23 < 9999)
                    {
                      self.AddLog("-" + Conversion.Str( index3) + " gets weight=" + Conversion.Str( num23));
                      simpleList.Add(index3, num23);
                    }
                  }
                }
              }
              if (simpleList.Counter > -1)
              {
                simpleList.Sort();
                if (simpleList.Weight[0] < num13)
                {
                  let mut index4: i32 =  self.HasPlanEngineerUnit(simpleList.Id[0]);
                  self.game.Data.UnitObj[index4].AIPlanNr = index1;
                  self.AddLog("Switched " + self.game.Data.UnitObj[index4].Name + " from plannr " + Conversion.Str( simpleList.Id[0]) + " to " + Conversion.Str( index1));
                  num17 += 1;
                }
              }
            }
            let mut num25: i32 =  0;
            let mut unitCounter5: i32 =  self.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter5; unr += 1)
            {
              if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn & self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].PreDef == -1)
              {
                let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
                if (aiPlanNr > 0 && self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan > -1)
                {
                  let mut landReservePlan1: i32 =  self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan;
                  let mut landReservePlan2: i32 =  self.SAObj[self.GetAreaNr(self.TPlanObj[index1].FromArea)].LandReservePlan;
                  if (landReservePlan1 > 0 & landReservePlan1 == landReservePlan2 && self.game.Data.UnitObj[unr].AIUnitGoal == 4 && Operators.ConditionalCompareObjectLess(self.GetEPPerTurn(unr),  self.game.Data.RuleVar[215], false))
                  {
                    num25 += 1;
                    self.AddLog("Found 1 engineer in same landreserve with not at ideal filling yet (if 5x found no new ones allowed).");
                  }
                }
              }
            }
            if ( self.game.Data.RuleVar[249] == 0.0 && Number > num13 & num17 == 0 & num25 < 5)
            {
              Coordinate engineerCoord = self.GetEngineerCoord(1, index1);
              if (engineerCoord.onmap && self.game.Data.MapObj[0].HexObj[engineerCoord.x, engineerCoord.y].Regime == self.game.Data.Turn)
              {
                let mut num26: i32 =  0;
                let mut unitCounter6: i32 =  self.game.Data.MapObj[0].HexObj[engineerCoord.x, engineerCoord.y].UnitCounter;
                for (let mut index5: i32 =  0; index5 <= unitCounter6; index5 += 1)
                {
                  if (self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[engineerCoord.x, engineerCoord.y].UnitList[index5]].AIUnitGoal == 4)
                    num26 += 1;
                }
                if (num26 <= 1)
                {
                  index6: i32;
                  index7: i32;
                  if (( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >=  self.game.Data.RuleVar[46] |  self.game.Data.RuleVar[863] > 0.0) & self.game.Data.MapObj[0].HexObj[index6, index7].UnitCounter < 15)
                  {
                    self.game.ProcessingObj.NewUnit(engineerCoord.x, engineerCoord.y, 0, false, self.game.Data.Turn);
                    self.game.Data.UnitObj[self.game.Data.UnitCounter].HQ = self.TPlanObj[index1].HQ;
                    self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = index1;
                    self.game.Data.UnitObj[self.game.Data.UnitCounter].AIUnitGoal = 4;
                    tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                    aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                    let mut index8: i32 =  index1;
                    let mut index9: i32 =  index8;
                    aiPlanClassArray[index9].FriendlyUnitCount = tplanObj[index8].FriendlyUnitCount + 1;
                    self.AddLog("New engineer goal unit placed at " + Conversion.Str( engineerCoord.x) + "," + Conversion.Str( engineerCoord.y) + " for plan #" + Conversion.Str( index1));
                  }
                }
                else
                  self.AddLog("No new unit placed since there is already at least 2 engineer units on this hex");
              }
            }
          }
        }
        if (self.TPlanObj[index1].Type == 40 & self.TPlanObj[index1].SeaTarget < 1 & (num1 == index1 | num1 == -1))
        {
          let mut num27: i32 =  0;
          let mut saCount: i32 =  self.SACount;
          for (let mut index10: i32 =  1; index10 <= saCount; index10 += 1)
          {
            if (self.GetAreaNr(self.TPlanObj[index1].FromArea) != index10 && self.HexContinent[self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y] == self.HexContinent[self.SAObj[index10].X, self.SAObj[index10].Y])
            {
              let mut neighbourCount: i32 =  self.SAObj[index10].NeighbourCount;
              for (let mut index11: i32 =  1; index11 <= neighbourCount; index11 += 1)
              {
                if (self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[self.SAObj[self.SAObj[index10].Neighbour[index11]].X, self.SAObj[self.SAObj[index10].Neighbour[index11]].Y].Regime))
                  num27 += 1;
              }
            }
          }
          let mut Number1: i32 =  0;
          if (num27 == 0)
            Number1 = 1;
          if (num27 > 0 & self.getfrontplan(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y) == -1)
            Number1 = 2;
          if (Number1 > 0)
          {
            let mut absoluteLandForRegime: i32 =  self.game.HandyFunctionsObj.GetPowerPtsAbsoluteLandForRegime(self.game.Data.Turn);
            let mut totalStrategicValue: i32 =  self.GetTotalStrategicValue();
            let mut weightStrategic: i32 =  self.TPlanObj[index1].WeightStrategic;
            let mut Number2: i32 =  (int) Math.Round( absoluteLandForRegime * ( weightStrategic /  totalStrategicValue) * ( self.game.Data.RuleVar[244] *  self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative / 100.0));
            if (Number1 == 2)
              Number2 = (int) Math.Round( Number2 * 0.5);
            let mut num28: i32 =  0;
            self.AddLog("Plan #" + Conversion.Str( index1) + " needs a reserve (typ" + Conversion.Str( Number1) + ") of " + Conversion.Str( Number2) + " absolute power pts.");
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
            {
              if (self.game.Data.UnitObj[unr].AIPlanNr == index1)
              {
                self.game.Data.UnitObj[unr].AIDisband = false;
                self.game.Data.UnitObj[unr].AIReserve = false;
                if (self.game.Data.UnitObj[unr].AIUnitGoal == 1 | self.game.Data.UnitObj[unr].AIUnitGoal == 2 | self.game.Data.UnitObj[unr].AIUnitGoal == 3 | self.game.Data.UnitObj[unr].AIUnitGoal == 4)
                {
                  if (Number2 >= num28)
                    self.game.Data.UnitObj[unr].AIReserve = true;
                  else
                    self.game.Data.UnitObj[unr].AIDisband = true;
                  num28 += self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true);
                }
              }
            }
          }
        }
        if (self.TPlanObj[index1].Type == 40 & self.TPlanObj[index1].SeaTarget > 0 & (num1 == -1 | num1 == index1) && self.MakeNavyActive(index1))
        {
          let mut absoluteLandForRegime: i32 =  self.game.HandyFunctionsObj.GetPowerPtsAbsoluteLandForRegime(self.game.Data.Turn);
          let mut totalStrategicValue: i32 =  self.GetTotalStrategicValue();
          let mut weightStrategic: i32 =  self.TPlanObj[index1].WeightStrategic;
          let mut Number: i32 =  (int) Math.Round( absoluteLandForRegime * ( weightStrategic /  totalStrategicValue) * ( self.game.Data.RuleVar[244] *  self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative / 100.0));
          let mut num29: i32 =  0;
          let mut num30: i32 =  0;
          if (self.getfrontplan(self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y) == -1)
          {
            self.AddLog("Plan amph #" + Conversion.Str( index1) + " needs a reserve (typ amph reserve) of " + Conversion.Str( Number) + " absolute power pts.");
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
            {
              if (self.game.Data.UnitObj[unr].AIPlanNr == index1 && self.TPlanObj[index1].SeaStand == 7)
              {
                self.game.Data.UnitObj[unr].AIReserve = false;
                if (self.game.Data.UnitObj[unr].AIUnitGoal == 1 | self.game.Data.UnitObj[unr].AIUnitGoal == 2 | self.game.Data.UnitObj[unr].AIUnitGoal == 3 | self.game.Data.UnitObj[unr].AIUnitGoal == 4)
                {
                  num30 += 1;
                  self.game.Data.UnitObj[unr].AIReserve = Number >= num29 & num30 > 1;
                  num29 += self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr, true);
                }
              }
            }
          }
          else
          {
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut index12: i32 =  0; index12 <= unitCounter; index12 += 1)
            {
              if (self.game.Data.UnitObj[index12].AIPlanNr == index1 && self.TPlanObj[index1].SeaStand == 7)
                self.game.Data.UnitObj[index12].AIReserve = false;
            }
          }
        }
      }
    }

    pub fn GetAbsolutePowerForReserveUnit(plnr: i32) -> i32
    {
      object Counter;
      object LoopForResult;
      object CounterResult;
      object obj;
      if (ObjectFlowControl.ForLoopControl.ForLoopInitObj(Counter,  0,  self.game.Data.UnitCounter,  1, ref LoopForResult, ref CounterResult))
      {
        do
        {
          if (self.game.Data.UnitObj[Conversions.ToInteger(CounterResult)].AIPlanNr == plnr & self.game.Data.UnitObj[Conversions.ToInteger(CounterResult)].AIReserve)
            obj = Operators.AddObject(obj,  1);
        }
        while (ObjectFlowControl.ForLoopControl.ForNextCheckObj(CounterResult, LoopForResult, ref CounterResult));
      }
      object absoluteLandForRegime =  self.game.HandyFunctionsObj.GetPowerPtsAbsoluteLandForRegime(self.game.Data.Turn);
      object totalStrategicValue =  self.GetTotalStrategicValue();
      object weightStrategic =  self.TPlanObj[plnr].WeightStrategic;
      object Left = Operators.MultiplyObject(Operators.MultiplyObject(absoluteLandForRegime, Operators.DivideObject(weightStrategic, totalStrategicValue)),   ( self.game.Data.RuleVar[244] *  self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative / 100.0));
      if (Operators.ConditionalCompareObjectEqual(obj,  0, false))
        obj =  1;
      return Conversions.ToInteger(Conversion.Int(Operators.DivideObject(Left, obj)));
    }

    pub fn GetTotalStrategicValue() -> i32
    {
      let mut tplanCount: i32 =  self.TPlanCount;
      totalStrategicValue: i32;
      for (let mut index: i32 =  1; index <= tplanCount; index += 1)
      {
        if (self.TPlanObj[index].Type == 40)
          totalStrategicValue += self.TPlanObj[index].WeightStrategic;
      }
      return totalStrategicValue;
    }

    pub fn HasPlanEngineerUnit(plannr: i32) -> i32
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index].AIPlanNr == plannr && self.game.Data.UnitObj[index].X > -1 && self.game.Data.UnitObj[index].AIUnitGoal == 4)
          return index;
      }
      return -1;
    }

    pub fn InitNavyTransferUnits()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      bool[] flagArray = new bool[self.game.Data.UnitCounter + 1];
      self.AddLog("*Transfer naval units");
      self.AddLog("");
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        SimpleList simpleList3 = SimpleList::new();
        let mut tplanCount: i32 =  self.TPlanCount;
        for (let mut tid: i32 =  1; tid <= tplanCount; tid += 1)
        {
          if (self.TPlanObj[tid].Type == 40 && self.HexSeaSA[self.TPlanObj[tid].FromArea.X, self.TPlanObj[tid].FromArea.Y] > 0)
          {
            let mut num2: i32 =  10;
            let mut num3: i32 =  !(self.TPlanObj[tid].SeaTarget > 0 & self.TPlanObj[tid].SeaTarget <= self.SACount) ? 5 : (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[self.SAObj[self.TPlanObj[tid].SeaTarget].X, self.SAObj[self.TPlanObj[tid].SeaTarget].Y].Regime, self.game.Data.Turn) ? num2 * self.SAObj[self.TPlanObj[tid].SeaTarget].fuzzyvp : num2 * 1);
            float num4 = !(self.TPlanObj[tid].FriendlyNavy > 0 & self.TPlanObj[tid].EnemyNavy > 0) ? 1f :  self.TPlanObj[tid].EnemyNavy /  self.TPlanObj[tid].FriendlyNavy;
            if ( num4 > 10.0)
              num4 = 10f;
            let mut tweight: i32 =  (int) Math.Round( Conversion.Int( num3 * num4));
            if (tweight < 1)
              tweight = 1;
            simpleList3.Add(tid, tweight);
          }
        }
        simpleList3.Sort();
        let mut counter1: i32 =  simpleList3.Counter;
        for (let mut index: i32 =  0; index <= counter1; index += 1)
          self.AddLog("Plan #" + Conversion.Str( simpleList3.Id[index]) + " ... weight = " + Conversion.Str( simpleList3.Weight[index]));
        if (simpleList3.Counter > -1)
        {
          SimpleList simpleList4 = SimpleList::new();
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
          {
            if (self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn & self.game.Data.UnitObj[tid].PreDef == -1)
            {
              let mut num5: i32 =  0;
              if (self.game.Data.UnitObj[tid].AIUnitGoal == 9 | self.game.Data.UnitObj[tid].AIUnitGoal == 10 | self.game.Data.UnitObj[tid].AIUnitGoal == 8)
                num5 = 1;
              if (self.TPlanObj[simpleList3.Id[simpleList3.Counter]].SeaTarget < 1 & self.game.Data.UnitObj[tid].AIUnitGoal == 8)
                num5 = 0;
              if (self.game.Data.UnitObj[tid].SFCount == -1)
                num5 = 0;
              if (self.game.Data.UnitObj[tid].AIUnitGoal == 8)
              {
                if (!self.MakeNavyActive(simpleList3.Id[simpleList3.Counter]))
                  num5 = 0;
                if (self.TPlanObj[simpleList3.Id[simpleList3.Counter]].SeaStand != 7)
                  num5 = 0;
              }
              if (num5 == 1)
              {
                let mut aiPlanNr: i32 =  self.game.Data.UnitObj[tid].AIPlanNr;
                if (aiPlanNr > 0)
                {
                  let mut nr: i32 =  simpleList3.FindNr(aiPlanNr);
                  if (nr > -1 && simpleList3.Weight[nr] * 4 < simpleList3.Weight[simpleList3.Counter] & !flagArray[tid])
                  {
                    self.SetNavalMatrix1(self.game.Data.UnitObj[tid].X, self.game.Data.UnitObj[tid].Y);
                    let mut tweight: i32 =  (int) Math.Round( self.Matrix1[self.TPlanObj[simpleList3.Id[simpleList3.Counter]].FromArea.X, self.TPlanObj[simpleList3.Id[simpleList3.Counter]].FromArea.Y] / Math.Sqrt( simpleList3.Weight[nr]));
                    if (tweight > 0)
                      simpleList4.Add(tid, tweight);
                  }
                }
              }
            }
          }
          simpleList4.Sort();
          self.AddLog("Find unit for transfer to plan #" + Conversion.Str( simpleList3.Id[simpleList3.Counter]));
          let mut counter2: i32 =  simpleList4.Counter;
          for (let mut index: i32 =  0; index <= counter2; index += 1)
            self.AddLog("Unit #" + self.game.Data.UnitObj[simpleList4.Id[index]].Name + " ... weight = " + Conversion.Str( simpleList4.Weight[index]));
          if (simpleList4.Counter > -1)
          {
            self.AddLog("Changing naval unit " + self.game.Data.UnitObj[simpleList4.Id[simpleList4.Counter]].Name + " to plan # " + Conversion.Str( simpleList3.Id[simpleList3.Counter]));
            num1 = 1;
            flagArray[simpleList4.Id[simpleList4.Counter]] = true;
            self.game.Data.UnitObj[simpleList4.Id[simpleList4.Counter]].AIPlanNr = simpleList3.Id[simpleList3.Counter];
            self.InitTPlanForceBalanceNavy();
          }
        }
      }
    }

    pub fn InitSetStandingOrders()
    {
      float friendlyAirRatio = self.GetFriendlyAirRatio();
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].X > -1 && self.game.HandyFunctionsObj.HasUnitAirSF(unr))
        {
          self.game.Data.UnitObj[unr].SOInterceptRdnStop =  friendlyAirRatio < 1.0 ? ( friendlyAirRatio < 0.5 ? (!( friendlyAirRatio >= 0.25 &  VBMath.Rnd() > 0.75) ? 100 : 75) : 75) : 50;
          let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
          self.game.Data.UnitObj[unr].SODefendPercent = aiPlanNr <= 0 ? 50 : (self.TPlanObj[aiPlanNr].Stand != 3 ? 50 : 5);
        }
      }
    }

    pub fn InitAirTransferUnits()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      int[] numArray2 = new int[self.TPlanCount + 1];
      bool[] flagArray = new bool[self.game.Data.UnitCounter + 1];
      int[] numArray3 = new int[self.TPlanCount + 1];
      int[] numArray4 = new int[self.TPlanCount + 1];
      int[] numArray5 = new int[self.TPlanCount + 1];
      int[] numArray6 = new int[self.TPlanCount + 1];
      if ( self.game.Data.RuleVar[221] < 1.0)
        return;
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      while (num1 == 1 & num2 <= self.TPlanCount * 2)
      {
        num1 = 0;
        num2 += 1;
        SimpleList simpleList3 = SimpleList::new();
        let mut tplanCount1: i32 =  self.TPlanCount;
        for (let mut index1: i32 =  1; index1 <= tplanCount1; index1 += 1)
        {
          if (self.TPlanObj[index1].Type == 40)
          {
            let mut num3: i32 =  0;
            let mut d1: i32 =  0;
            let mut num4: i32 =  0;
            let mut num5: i32 =  0;
            let mut num6: i32 =  0;
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
              {
                if (self.HexBackPlan[index2, index3] == index1)
                {
                  let mut forceLandStrength1: i32 =  self.GetHexForceLandStrength(index2, index3, true);
                  self.GetHexForceAirStrength(index2, index3, true);
                  num3 += forceLandStrength1;
                  if (self.game.HandyFunctionsObj.IsHexAirfield(index2, index3, 0) & num5 == 0)
                  {
                    num5 = 1;
                    let mut num7: i32 =  (int) Math.Round( ( index2 - self.game.Data.RuleVar[223]));
                    let mut num8: i32 =  (int) Math.Round( ( index2 + self.game.Data.RuleVar[223]));
                    for (let mut index4: i32 =  num7; index4 <= num8; index4 += 1)
                    {
                      let mut index5: i32 =  index4;
                      if (self.game.Data.MapObj[0].MapLoop & index5 < 0)
                        index5 = self.game.Data.MapObj[0].MapWidth + index5 + 1;
                      if (self.game.Data.MapObj[0].MapLoop & index5 > self.game.Data.MapObj[0].MapWidth)
                        index5 = index5 - self.game.Data.MapObj[0].MapWidth - 1;
                      let mut num9: i32 =  (int) Math.Round( ( index3 - self.game.Data.RuleVar[223]));
                      let mut num10: i32 =  (int) Math.Round( ( index3 + self.game.Data.RuleVar[223]));
                      for (let mut index6: i32 =  num9; index6 <= num10; index6 += 1)
                      {
                        if (index5 > -1 & index6 > -1 && index5 <= self.game.Data.MapObj[0].MapWidth & index6 <= self.game.Data.MapObj[0].MapHeight)
                        {
                          if (self.game.Data.MapObj[0].HexObj[index5, index6].Location > -1 & self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index5, index6].Regime, self.game.Data.Turn) &&  self.game.HandyFunctionsObj.Distance(index2, index3, 0, index5, index6, 0) <=  self.game.Data.RuleVar[223] *  self.game.Data.RuleVar[147])
                            num6 += self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[index5, index6].Location].Type].MaxProd;
                          if (self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index5, index6].Regime) && self.game.Data.MapObj[0].HexObj[index5, index6].UnitCounter > -1 &&  self.game.HandyFunctionsObj.Distance(index2, index3, 0, index5, index6, 0) <=  self.game.Data.RuleVar[223])
                          {
                            let mut forceLandStrength2: i32 =  self.GetHexForceLandStrength(index5, index6, true);
                            let mut forceAirStrength: i32 =  self.GetHexForceAirStrength(index5, index6);
                            d1 += forceLandStrength2;
                            num4 += forceAirStrength;
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            let mut num11: i32 =  0;
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
            {
              if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].X > -1 && self.game.Data.UnitObj[unr].AIPlanNr == index1)
                num11 += self.game.HandyFunctionsObj.GetPowerPtsAbsoluteForAirOnly(unr);
            }
            if (num3 == 0)
              num3 = 1;
            self.TPlanObj[index1].FriendlyAir = num11;
            self.TPlanObj[index1].EnemyAir = num4;
            self.TPlanObj[index1].EnemyTroops = d1;
            self.TPlanObj[index1].ProdPtsInRange = num6;
            numArray3[index1] = num11;
            numArray4[index1] = num4;
            numArray5[index1] = num3;
            numArray6[index1] = d1;
            let mut num12: i32 =  (int) Math.Round( (int) Math.Round( (int) Math.Round(1000.0 - Math.Sqrt( (num4 * 10))) - Math.Sqrt( d1)) - Math.Sqrt( num6 / 10.0));
            if (num4 < 1 & d1 < 1)
              num12 += 250;
            if (num11 < 1)
              num11 = 1;
            let mut d2: i32 =  num11 * 10;
            let mut num13: i32 =  (int) Math.Round( num12 + Math.Sqrt( d2));
            simpleList3.Add(index1, num13);
            self.AddLog("plnr " + Conversion.Str( index1) + " weight= " + Conversion.Str( num13));
          }
        }
        if (simpleList3.Counter > 0)
        {
          simpleList3.Sort();
          let mut Number1: i32 =  simpleList3.Id[0];
          let mut tplanCount2: i32 =  self.TPlanCount;
          for (let mut Number2: i32 =  1; Number2 <= tplanCount2; Number2 += 1)
          {
            if (self.TPlanObj[Number2].Type == 40 && self.HasAreaAirfield(self.GetAreaNr(self.TPlanObj[Number2].FromArea)))
            {
              let mut num14: i32 =  0;
              if (self.HexOA[self.TPlanObj[Number1].FromArea.X, self.TPlanObj[Number1].FromArea.Y] == self.HexOA[self.TPlanObj[Number2].FromArea.X, self.TPlanObj[Number2].FromArea.Y])
                num14 = 1;
              if (Number1 == Number2)
                num14 = 0;
              if (num14 == 1)
              {
                let mut unitCounter: i32 =  self.game.Data.UnitCounter;
                for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
                {
                  if (self.game.Data.UnitObj[unr].AIUnitGoal == 5 & self.game.Data.UnitObj[unr].AIPlanNr == Number2 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn)
                  {
                    let mut num15: i32 =  numArray3[Number2] - self.GetForceAirStrength(unr, true);
                    if (1 > num15)
                      num15 = 1;
                    let mut d3: i32 =  num15 * 10;
                    let mut d4: i32 =  numArray4[Number2] * 10;
                    let mut d5: i32 =  numArray6[Number2];
                    let mut prodPtsInRange1: i32 =  self.TPlanObj[Number2].ProdPtsInRange;
                    let mut num16: i32 =  (int) Math.Round( (int) Math.Round( (int) Math.Round(1000.0 - Math.Sqrt( d4)) - Math.Sqrt( d5)) - Math.Sqrt( prodPtsInRange1 / 10.0));
                    if (d4 < 1 & d5 < 1)
                      num16 += 250;
                    let mut Number3: i32 =  (int) Math.Round( num16 + Math.Sqrt( d3));
                    let mut num17: i32 =  numArray3[Number1] + self.GetForceAirStrength(unr, true);
                    if (1 > num17)
                      num17 = 1;
                    let mut d6: i32 =  num17 * 10;
                    let mut d7: i32 =  numArray4[Number1] * 10;
                    let mut d8: i32 =  numArray6[Number1];
                    let mut prodPtsInRange2: i32 =  self.TPlanObj[Number2].ProdPtsInRange;
                    let mut num18: i32 =  (int) Math.Round( (int) Math.Round( (int) Math.Round(1000.0 - Math.Sqrt( d7)) - Math.Sqrt( d8)) - Math.Sqrt( prodPtsInRange2 / 10.0));
                    if (d7 < 1 & d8 < 1)
                      Number3 += 250;
                    let mut Number4: i32 =  (int) Math.Round( num18 + Math.Sqrt( d6));
                    self.AddLog(Conversion.Str( Number1) + " will be " + Conversion.Str( Number4) + " while source plan " + Conversion.Str( Number2) + " will be " + Conversion.Str( Number3));
                    if ( Number3 >  Number4 * 1.15)
                    {
                      self.AddLog("Switching Airunit " + self.game.Data.UnitObj[unr].Name + " from plan#" + Conversion.Str( Number2) + " to plan#" + Conversion.Str( Number1));
                      self.AddLog("pred.weight= " + Conversion.Str( Number3) + " wile sl.weight= " + Conversion.Str( simpleList3.Weight[simpleList3.Counter]));
                      self.game.Data.UnitObj[unr].AIPlanNr = Number1;
                      num1 = 1;
                      break;
                    }
                  }
                }
                if (num1 == 1)
                  break;
              }
            }
          }
        }
      }
    }

    pub HasAreaAirfield: bool(nr: i32)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 =  0; x <= mapWidth; x += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 =  0; y <= mapHeight; y += 1)
        {
          if (self.HexSA[x, y] == nr & self.game.HandyFunctionsObj.IsHexAirfield(x, y, 0))
            return true;
        }
      }
      return false;
    }

    pub fn InitTransferUnits()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      int[] numArray2 = new int[self.TPlanCount + 1];
      bool[] flagArray = new bool[self.game.Data.UnitCounter + 1];
      self.AddLog("");
      self.AddLog("*Consider assigning Units to different Plans");
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      while (num1 == 1 & num2 <= self.TPlanCount * 2)
      {
        num1 = 0;
        num2 += 1;
        SimpleList simpleList3 = SimpleList::new();
        let mut tplanCount: i32 =  self.TPlanCount;
        num3: i32;
        for (let mut index: i32 =  1; index <= tplanCount; index += 1)
        {
          if (self.TPlanObj[index].Type == 20)
          {
            if ( self.TPlanObj[index].WeightFriendlyForce > 0.0)
            {
              num3 = (int) Math.Round(200.0 * ( self.TPlanObj[index].WeightEnemyForceUnMod /  self.TPlanObj[index].WeightFriendlyForce));
            }
            else
            {
              try
              {
                num3 =  self.TPlanObj[index].WeightEnemyForceUnMod >= 1.0 ? (int) Math.Round(400.0 + 20.0 *  self.TPlanObj[index].WeightEnemyForceUnMod) : 100;
              }
              catch (Exception ex)
              {
                ProjectData.SetProjectError(ex);
                ProjectData.ClearProjectError();
              }
            }
            num3 = (int) Math.Round( (int) Math.Round( ( num3 + self.TPlanObj[index].WeightFriendlyForce * self.GetPercentCuttenOff(index))) + Math.Sqrt( self.TPlanObj[index].WeightStrategic) * 100.0);
            let mut regime: i32 =  self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime;
            if ( self.game.Data.RuleVar[264] == 0.0 && regime > -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regime] == 0 && self.game.Data.RegimeObj[regime].AI)
            {
              num3 = (int) Math.Round( num3 * 0.5);
              if (1 > num3)
                num3 = 1;
              if ((self.game.Data.GameID + regime + self.game.Data.Turn) % 2 == 0)
              {
                num3 = (int) Math.Round( num3 * 0.5);
                if (1 > num3)
                  num3 = 1;
              }
              else if ((self.game.Data.GameID + regime + self.game.Data.Turn) % 3 == 0)
              {
                num3 = (int) Math.Round( num3 * 0.1);
                if (1 > num3)
                  num3 = 1;
              }
            }
            self.AddLog("Plan #" + Conversion.Str( index) + " gets weight=" + Conversion.Str( num3));
            simpleList3.Add(index, num3);
          }
        }
        simpleList3.Sort();
        if (simpleList3.Counter > -1)
        {
          for (let mut counter1: i32 =  simpleList3.Counter; counter1 >= 0; counter1 += -1)
          {
            SimpleList simpleList4 = SimpleList::new();
            let mut Number1: i32 =  simpleList3.Id[counter1];
            let mut counter2: i32 =  simpleList3.Counter;
            for (let mut index: i32 =  0; index <= counter2; index += 1)
            {
              if (index != counter1 && simpleList3.Weight[index] < simpleList3.Weight[counter1])
              {
                let mut tid: i32 =  simpleList3.Id[index];
                if (self.GetAreaNr(self.TPlanObj[Number1].FromArea) == self.GetAreaNr(self.TPlanObj[tid].FromArea))
                {
                  num3 = 0;
                }
                else
                {
                  num3 = self.AreaDistance(self.GetAreaNr(self.TPlanObj[Number1].FromArea), self.GetAreaNr(self.TPlanObj[tid].FromArea), true);
                  if (num3 == 0)
                    num3 = 999;
                }
                if (num3 < 99)
                {
                  let mut num4: i32 =  simpleList3.Weight[counter1] - simpleList3.Weight[index];
                  let mut tweight: i32 =  num3 != 0 ? (int) Math.Round( num4 /  num3) : num4 * 2;
                  if (self.TPlanObj[tid].FriendlyUnitCount < 2)
                    tweight = 0;
                  if ( self.TPlanObj[tid].WeightFriendlyForce == 0.0)
                    tweight = 0;
                  simpleList4.Add(tid, tweight);
                }
              }
            }
            simpleList4.Sort();
            if (simpleList4.Counter > -1)
            {
              let mut Number2: i32 =  simpleList4.Id[simpleList4.Counter];
              if (simpleList4.Weight[simpleList4.Counter] > 50 + numArray2[Number2])
              {
                let mut num5: i32 =  9999;
                let mut index1: i32 =  -1;
                let mut unitCounter: i32 =  self.game.Data.UnitCounter;
                for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
                {
                  if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && !flagArray[unr] && !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].AIUnitGoal != 4 && self.game.Data.UnitObj[unr].AIPlanNr == Number2 & self.game.Data.UnitObj[unr].AIUnitGoal != 4 && self.HexSA[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] == self.GetAreaNr(self.TPlanObj[Number2].FromArea))
                  {
                    let mut num6: i32 =  self.GetForceLandStrength(unr);
                    if (self.AIVP[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] > 0)
                      num6 *= 4;
                    if (self.game.Data.UnitObj[unr].HQ > -1 & self.game.Data.UnitObj[unr].HQ == self.TPlanObj[Number1].HQ)
                      num6 = (int) Math.Round( num6 / 2.0);
                    if (self.game.Data.UnitObj[unr].X == self.TPlanObj[Number2].FromArea.X & self.game.Data.UnitObj[unr].Y == self.TPlanObj[Number2].FromArea.Y)
                      num6 = 999999;
                    if (num6 < num5 &  num6 >=  self.game.Data.RuleVar[183] & num6 < 9999)
                    {
                      num5 = num6;
                      index1 = unr;
                    }
                  }
                }
                if (index1 > -1)
                {
                  self.AddLog("Switching " + self.game.Data.UnitObj[index1].Name + " from plan#" + Conversion.Str( Number2) + " to plan#" + Conversion.Str( Number1));
                  self.game.Data.UnitObj[index1].AIPlanNr = Number1;
                  flagArray[index1] = true;
                  tplanObj1: Vec<AIPlanClass> = self.TPlanObj;
                  aiPlanClassArray1: Vec<AIPlanClass> = tplanObj1;
                  let mut index2: i32 =  Number1;
                  let mut index3: i32 =  index2;
                  aiPlanClassArray1[index3].WeightFriendlyForce = tplanObj1[index2].WeightFriendlyForce +  num5;
                  tplanObj2: Vec<AIPlanClass> = self.TPlanObj;
                  aiPlanClassArray2: Vec<AIPlanClass> = tplanObj2;
                  let mut index4: i32 =  Number1;
                  let mut index5: i32 =  index4;
                  aiPlanClassArray2[index5].FriendlyUnitCount = tplanObj2[index4].FriendlyUnitCount + 1;
                  tplanObj3: Vec<AIPlanClass> = self.TPlanObj;
                  aiPlanClassArray3: Vec<AIPlanClass> = tplanObj3;
                  let mut index6: i32 =  Number2;
                  let mut index7: i32 =  index6;
                  aiPlanClassArray3[index7].WeightFriendlyForce = tplanObj3[index6].WeightFriendlyForce -  num5;
                  tplanObj4: Vec<AIPlanClass> = self.TPlanObj;
                  aiPlanClassArray4: Vec<AIPlanClass> = tplanObj4;
                  let mut index8: i32 =  Number2;
                  let mut index9: i32 =  index8;
                  aiPlanClassArray4[index9].FriendlyUnitCount = tplanObj4[index8].FriendlyUnitCount - 1;
                  int[] numArray3 = numArray2;
                  int[] numArray4 = numArray3;
                  let mut index10: i32 =  Number2;
                  let mut index11: i32 =  index10;
                  let mut num7: i32 =  numArray3[index10] + 50;
                  numArray4[index11] = num7;
                  num1 = 1;
                }
              }
            }
            if (num1 == 1)
              break;
          }
        }
      }
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
      {
        let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
        if (aiPlanNr > 0 && self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && !self.game.HandyFunctionsObj.HasUnitNavySF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) && self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].PreDef == -1 && self.HexContinent[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] != self.HexContinent[self.TPlanObj[aiPlanNr].FromArea.X, self.TPlanObj[aiPlanNr].FromArea.Y])
        {
          let mut num8: i32 =  0;
          if (self.TPlanObj[aiPlanNr].SeaTarget > 0 & self.TPlanObj[aiPlanNr].SeaTarget <= self.SACount)
          {
            if (self.HexContinent[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] != self.HexContinent[self.SAObj[self.TPlanObj[aiPlanNr].SeaTarget].X, self.SAObj[self.TPlanObj[aiPlanNr].SeaTarget].Y])
              num8 = 1;
          }
          else
            num8 = 1;
          if (num8 == 1)
            self.game.Data.UnitObj[unr].AIPlanNr = self.GetClosestBackPlan(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y);
        }
      }
      let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
      {
        let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
        if (aiPlanNr > 0 && self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && !self.game.HandyFunctionsObj.HasUnitNavySF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) && self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[unr].PreDef == -1 && self.TPlanObj[aiPlanNr].Type == 40 && self.game.Data.UnitObj[unr].AIUnitGoal != 4)
        {
          if (self.getfrontplan(self.TPlanObj[aiPlanNr].FromArea.X, self.TPlanObj[aiPlanNr].FromArea.Y) > -1 && !(self.TPlanObj[aiPlanNr].SeaTarget > 0 & self.TPlanObj[aiPlanNr].SeaStand == 7))
          {
            self.game.Data.UnitObj[unr].AIPlanNr = self.getfrontplan(self.TPlanObj[aiPlanNr].FromArea.X, self.TPlanObj[aiPlanNr].FromArea.Y);
            self.AddLog("Switched unit " + self.game.Data.UnitObj[unr].Name + " from backplan to frontplan, since combat is near");
          }
          if (self.TPlanObj[aiPlanNr].SeaTarget > 0 && self.HexContinent[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] != self.HexContinent[self.TPlanObj[aiPlanNr].FromArea.X, self.TPlanObj[aiPlanNr].FromArea.Y] && self.getfrontplan(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y) > -1)
          {
            self.game.Data.UnitObj[unr].AIPlanNr = self.getfrontplan(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y);
            self.AddLog("Switched unit " + self.game.Data.UnitObj[unr].Name + " from backplan to frontplan, since combat is near");
          }
        }
      }
    }

    pub fn InitTacticalHQ()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList = SimpleList::new();
      SimpleList UL = SimpleList::new();
      if ( self.game.Data.RuleVar[(int) byte.MaxValue] == 1.0)
        return;
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut Number1: i32 =  1; Number1 <= tplanCount1; Number1 += 1)
      {
        if (self.TPlanObj[Number1].Type == 20 && self.TPlanObj[Number1].HQ > -1)
        {
          let mut hq: i32 =  self.TPlanObj[Number1].HQ;
          let mut x: i32 =  self.game.Data.UnitObj[hq].X;
          let mut y: i32 =  self.game.Data.UnitObj[hq].Y;
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 99, (int) Math.Round( self.game.Data.RuleVar[3]), x, y, 0);
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          Number2: i32;
          num1: i32;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (self.game.Data.UnitObj[index].HQ == hq & self.game.Data.UnitObj[Number1].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].X > -1 & self.game.Data.UnitObj[index].PreDef == -1 && self.HexSA[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y] == self.GetAreaNr(self.TPlanObj[Number1].FromArea))
            {
              let mut num2: i32 =  self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y];
              if ( num2 >  self.game.Data.RuleVar[3] * 2.0)
                num2 = (int) Math.Round( (self.game.Data.RuleVar[3] * 2f));
              Number2 += num2;
              num1 += 1;
            }
          }
          Number2 = num1 != 0 ? (int) Math.Round( Number2 /  num1) : 0;
          if ( Number2 >  self.game.Data.RuleVar[52])
          {
            self.AddLog("SET TAC.HQ of Plan #" + Conversion.Str( Number1) + " to NONE because of really bad supply currently.. avg=" + Conversion.Str( Number2) + "ap need.");
            self.TPlanObj[Number1].HQ = -1;
          }
        }
      }
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut Number: i32 =  1; Number <= tplanCount2; Number += 1)
      {
        if (self.TPlanObj[Number].Type == 20 && self.TPlanObj[Number].HQ == -1)
        {
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (self.game.Data.UnitObj[index].AIPlanNr == Number & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].IsHQ)
            {
              self.TPlanObj[Number].HQ = index;
              self.AddLog("*** Gave plan " + Conversion.Str( Number) + " the following HQ: " + self.game.Data.UnitObj[index].Name + " from its own troops.");
              break;
            }
          }
        }
      }
      let mut tplanCount3: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount3; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 20 & self.NeedHQ(index1) && self.TPlanObj[index1].HQ == -1)
        {
          let mut num: i32 =  0;
          let mut tplanCount4: i32 =  self.TPlanCount;
          for (let mut Number: i32 =  1; Number <= tplanCount4; Number += 1)
          {
            if (self.TPlanObj[Number].Type == 20)
            {
              self.AreaDistance(self.GetAreaNr(self.TPlanObj[index1].FromArea), self.GetAreaNr(self.TPlanObj[Number].FromArea));
              if (index1 == Number)
              {
                let mut unitCounter: i32 =  self.game.Data.UnitCounter;
                for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
                {
                  if (self.game.Data.UnitObj[index2].AIPlanNr == Number & self.TPlanObj[Number].HQ != index2 && self.game.Data.UnitObj[index2].IsHQ & self.game.Data.UnitObj[index2].Regime == self.game.Data.Turn)
                  {
                    self.TPlanObj[index1].HQ = index2;
                    self.game.Data.UnitObj[index2].AIPlanNr = index1;
                    self.AddLog("*** Gave plan " + Conversion.Str( index1) + " the following HQ: " + self.game.Data.UnitObj[index2].Name + " from plan # " + Conversion.Str( Number));
                    tplanObj1: Vec<AIPlanClass> = self.TPlanObj;
                    aiPlanClassArray1: Vec<AIPlanClass> = tplanObj1;
                    let mut index3: i32 =  index1;
                    let mut index4: i32 =  index3;
                    aiPlanClassArray1[index4].FriendlyUnitCount = tplanObj1[index3].FriendlyUnitCount + 1;
                    tplanObj2: Vec<AIPlanClass> = self.TPlanObj;
                    aiPlanClassArray2: Vec<AIPlanClass> = tplanObj2;
                    let mut index5: i32 =  Number;
                    let mut index6: i32 =  index5;
                    aiPlanClassArray2[index6].FriendlyUnitCount = tplanObj2[index5].FriendlyUnitCount - 1;
                    num = 1;
                    break;
                  }
                }
                if (num == 1)
                  break;
              }
            }
          }
        }
      }
      let mut tplanCount5: i32 =  self.TPlanCount;
      for (let mut index7: i32 =  1; index7 <= tplanCount5; index7 += 1)
      {
        if (self.TPlanObj[index7].Type == 20 && self.TPlanObj[index7].HQ == -1 & self.NeedHQ(index7) &&  self.game.Data.RuleVar[47] <=  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts |  self.game.Data.RuleVar[863] > 0.0)
        {
          let mut num: i32 =  1;
          let mut tplanCount6: i32 =  self.TPlanCount;
          for (let mut index8: i32 =  1; index8 <= tplanCount6; index8 += 1)
          {
            if (index8 != index7 && self.TPlanObj[index8].Type == 20 | self.TPlanObj[index8].Type == 30 && self.TPlanObj[index8].FromArea.X == self.TPlanObj[index7].FromArea.X && self.TPlanObj[index8].FromArea.Y == self.TPlanObj[index7].FromArea.Y && self.TPlanObj[index8].HQ > -1 &&  self.AverageDistanceUnits(index7, self.game.Data.UnitObj[self.TPlanObj[index8].HQ].X, self.game.Data.UnitObj[self.TPlanObj[index8].HQ].Y) <=  self.game.Data.RuleVar[191])
              num = 0;
          }
          if (self.SAObj[self.GetAreaNr(self.TPlanObj[index7].FromArea)].LandReservePlan == 0)
            num = 0;
          if (self.SAObj[self.GetAreaNr(self.TPlanObj[index7].FromArea)].LandReservePlan > 0)
          {
            let mut landReservePlan: i32 =  self.SAObj[self.GetAreaNr(self.TPlanObj[index7].FromArea)].LandReservePlan;
            let mut x1: i32 =  self.TPlanObj[landReservePlan].FromArea.X;
            let mut y1: i32 =  self.TPlanObj[landReservePlan].FromArea.Y;
            if (self.TPlanObj[landReservePlan].HQ > -1)
            {
              let mut x2: i32 =  self.TPlanObj[index7].FromArea.X;
              let mut y2: i32 =  self.TPlanObj[index7].FromArea.Y;
              self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 99, (int) Math.Round( self.game.Data.RuleVar[3]), x1, y1, 0);
              if ( self.game.EditObj.TempValue[0].Value[x2, y2] >  self.game.Data.RuleVar[52])
                num = 0;
            }
            else
              num = 0;
          }
          if (num == 1)
          {
            let mut unitCounter: i32 =  self.game.Data.UnitCounter;
            for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
            {
              if (self.game.Data.UnitObj[tid].AIPlanNr == index7 & self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn)
                UL.Add(tid, -1);
            }
            Coordinate coordinate = self.SetMatrixHQ(UL, onlysanr: self.GetAreaNr(self.TPlanObj[index7].FromArea));
            x: i32;
            y: i32;
            if (coordinate.onmap)
            {
              x = coordinate.x;
              y = coordinate.y;
            }
            else
            {
              x = self.TPlanObj[index7].FromArea.X;
              y = self.TPlanObj[index7].FromArea.Y;
            }
            if (self.TPlanObj[index7].Stand == 3)
            {
              let mut neighbourForRetreater: i32 =  self.GetBestNeighbourForRetreater(self.GetAreaNr(self.TPlanObj[index7].FromArea));
              if (neighbourForRetreater > 0)
              {
                x = self.SAObj[neighbourForRetreater].X;
                y = self.SAObj[neighbourForRetreater].Y;
              }
            }
            if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 15)
            {
              self.game.ProcessingObj.NewUnit(x, y, 0, true, self.game.Data.Turn);
              self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = index7;
              self.TPlanObj[index7].HQ = self.game.Data.UnitCounter;
              tplanObj: Vec<AIPlanClass> = self.TPlanObj;
              aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
              let mut index9: i32 =  index7;
              let mut index10: i32 =  index9;
              aiPlanClassArray[index10].FriendlyUnitCount = tplanObj[index9].FriendlyUnitCount + 1;
              self.AddLog("*** Gave plan " + Conversion.Str( index7) + " a new HQ: " + self.game.Data.UnitObj[self.game.Data.UnitCounter].Name + ", created freshly.");
            }
          }
        }
      }
    }

    pub fn AverageDistanceUnits(plannr: i32, x: i32, y: i32) -> i32
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (self.game.Data.UnitObj[index].AIPlanNr == plannr & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
        {
          num1 += self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y, 0, x, y, 0);
          num2 += 1;
        }
      }
      if (num2 == 0)
        num2 = 1;
      return (int) Math.Round(Conversion.Int( num1 /  num2));
    }

    pub fn AverageDistanceUnitsInAP(plannr: i32, x: i32, y: i32, bool onlyifinownarea = false) -> i32
    {
      if (self.TempAvgUnits[plannr] > -1)
        return self.TempAvgUnits[plannr];
      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[3]), x, y, 0);
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (self.game.Data.UnitObj[index].X > -1 & self.game.Data.UnitObj[index].PreDef == -1 && self.game.Data.UnitObj[index].AIPlanNr == plannr & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
        {
          let mut num3: i32 =  1;
          if (onlyifinownarea && self.HexSA[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y] != self.GetSANr(self.TPlanObj[plannr].FromArea))
            num3 = 0;
          if (num3 == 1)
          {
            let mut num4: i32 =  (int) Math.Round( self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative);
            if (num4 > 250)
              num4 = 250;
            num1 += num4;
            num2 += 1;
          }
        }
      }
      if (num2 == 0)
        num2 = 1;
      return (int) Math.Round(Conversion.Int( num1 /  num2));
    }

    pub fn InitLandReserveMetaHQ()
    {
      int[] numArray = new int[self.game.Data.UnitCounter + 1];
      self.AddLog("");
      self.AddLog("LANDRESERVE HQ ASSIGNING");
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        numArray[index] = -1;
        if (self.game.Data.UnitObj[index].PreDef == -1 & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].IsHQ)
          self.game.Data.UnitObj[index].HQ = -1;
      }
      let mut num: i32 =  1;
      let mut Number: i32 =  -1;
      let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter2; index += 1)
        numArray[index] = -1;
      while (num == 1)
      {
        Number += 1;
        num = 0;
        SimpleList simpleList1 = SimpleList::new();
        let mut unitCounter3: i32 =  self.game.Data.UnitCounter;
        for (let mut tid: i32 =  0; tid <= unitCounter3; tid += 1)
        {
          if (self.game.Data.UnitObj[tid].PreDef == -1 & self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn && self.game.Data.UnitObj[tid].X > -1 && self.game.Data.UnitObj[tid].IsHQ & self.game.Data.UnitObj[tid].HQ == -1 && self.game.Data.UnitObj[tid].AIPlanNr > 0 && self.TPlanObj[self.game.Data.UnitObj[tid].AIPlanNr].Type == 30 && numArray[tid] == -1)
          {
            let mut closestEnemyDistance: i32 =  self.GetClosestEnemyDistance(self.game.Data.UnitObj[tid].X, self.game.Data.UnitObj[tid].Y);
            simpleList1.Add(tid, closestEnemyDistance);
          }
        }
        if (simpleList1.Counter > -1)
        {
          simpleList1.Sort();
          numArray[simpleList1.Id[simpleList1.Counter]] = Number;
          if (Number == 0)
            self.TPlanObj[self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].AIPlanNr].MetaChainNr = 0;
          self.AddLog("Set " + self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].Name + " to temp=" + Conversion.Str( Number));
          num = 1;
          if (Number > 0)
          {
            self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 99, (int) Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].X, self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].Y, 0, NoAPPenalties: true);
            SimpleList simpleList2 = SimpleList::new();
            let mut unitCounter4: i32 =  self.game.Data.UnitCounter;
            for (let mut tid: i32 =  0; tid <= unitCounter4; tid += 1)
            {
              if (self.game.Data.UnitObj[tid].X > -1 & self.game.Data.UnitObj[tid].PreDef == -1 && numArray[tid] > -1 & numArray[tid] < Number)
                simpleList2.Add(tid, self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[tid].X, self.game.Data.UnitObj[tid].Y]);
            }
            if (simpleList2.Counter > -1)
            {
              simpleList2.Sort();
              let mut index: i32 =  simpleList2.Id[0];
              self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].HQ = index;
              self.TPlanObj[self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].AIPlanNr].MetaChainNr = self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr].MetaChainNr + 1;
              self.AddLog("Assigned " + self.game.Data.UnitObj[simpleList1.Id[simpleList1.Counter]].Name + " to => " + self.game.Data.UnitObj[index].Name);
            }
          }
        }
      }
      self.AddLog("");
    }

    pub fn InitResearch()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList = SimpleList::new();
      if ( self.game.Data.RuleVar[226] <= 0.0)
        return;
      self.AddLog("");
      self.AddLog("RESEARCH");
      let mut resPts1: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts;
      if ( resPts1 >  self.game.Data.RuleVar[181] &  VBMath.Rnd() > 0.5 |  resPts1 >  self.game.Data.RuleVar[181] * 5.0)
      {
        let mut Number: i32 =  (int) Math.Round( ( resPts1 - self.game.Data.RuleVar[181]));
        RegimeClass[] regimeObj1 = self.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray1 = regimeObj1;
        let mut turn1: i32 =  self.game.Data.Turn;
        let mut index1: i32 =  turn1;
        regimeClassArray1[index1].AISavedPP = regimeObj1[turn1].AISavedPP + Number;
        RegimeClass[] regimeObj2 = self.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray2 = regimeObj2;
        let mut turn2: i32 =  self.game.Data.Turn;
        let mut index2: i32 =  turn2;
        regimeClassArray2[index2].ResPts = regimeObj2[turn2].ResPts - Number;
        self.AddLog("Added " + Conversion.Str( Number) + " pp to savedpp of regime. which is now: " + Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP));
      }
      self.AddLog("Saved pp: " + Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP) + ", Normal pp: " + Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts));
      float Number1 = 1000f;
      let mut researchCounter1: i32 =  self.game.Data.ResearchCounter;
      for (let mut index: i32 =  0; index <= researchCounter1; index += 1)
      {
        if (self.game.Data.RegimeObj[self.game.Data.Turn].ResField[index])
        {
          let mut sfTypePic: i32 =  self.game.Data.ResearchObj[index].SFTypePic;
          if (sfTypePic > -1 && self.game.Data.SFTypeObj[sfTypePic].Cap <= 0 && self.game.Data.SFTypeObj[sfTypePic].Theater == 1)
          {
            float num1 = 0.0f;
            let mut num2: i32 =  0;
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut attackx: i32 =  0; attackx <= mapWidth; attackx += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut attacky: i32 =  0; attacky <= mapHeight; attacky += 1)
              {
                if (!(self.game.Data.MapObj[0].HexObj[attackx, attacky].Regime > -1 & self.game.Data.SFTypeObj[sfTypePic].Theater != 1) && self.game.Data.SFTypeObj[sfTypePic].Theater == 1 & self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitCounter > -1 && self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitList[0]].Regime != self.game.Data.Turn)
                {
                  float num3 = self.AverageCombatPerform(-1, sfTypePic, attackx, attacky, onlysametheater: true);
                  float num4 = self.AverageCombatPerform(-1, sfTypePic, attackx, attacky, true, true);
                  if ( num3 >  num4)
                    num1 +=  (( num3 * 3.0 +  num4) / 4.0);
                  else
                    num1 +=  (( num4 * 3.0 +  num3) / 4.0);
                  num2 += 1;
                }
              }
            }
            if (num2 > 0 &  num1 > 0.0 && 1.0 / ( num1 /  num2) <  Number1)
              Number1 =  (1.0 / ( num1 /  num2));
          }
        }
      }
      self.AddLog("bestseamod= " + Conversion.Str( Number1));
      let mut researchCounter2: i32 =  self.game.Data.ResearchCounter;
      for (let mut tid: i32 =  0; tid <= researchCounter2; tid += 1)
      {
        if (!self.game.Data.RegimeObj[self.game.Data.Turn].ResField[tid])
        {
          let mut num5: i32 =  1;
          if (self.game.Data.ResearchObj[tid].PreReq > -1 && !self.game.Data.RegimeObj[self.game.Data.Turn].ResField[self.game.Data.ResearchObj[tid].PreReq])
            num5 = 0;
          if (self.game.Data.ResearchObj[tid].PreReq2 > -1 && !self.game.Data.RegimeObj[self.game.Data.Turn].ResField[self.game.Data.ResearchObj[tid].PreReq2])
            num5 = 0;
          if (num5 == 1)
          {
            let mut num6: i32 =  self.game.Data.ResearchObj[tid].PointCost[self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.game.Data.Turn].People].PeopleGroup];
            let mut sfTypePic: i32 =  self.game.Data.ResearchObj[tid].SFTypePic;
            if (sfTypePic > -1 & num6 > -1 & num6 < 9998 && self.game.Data.SFTypeObj[sfTypePic].StaffPts <= 0 && self.game.Data.SFTypeObj[sfTypePic].Cap <= 0 && !(self.game.Data.SFTypeObj[sfTypePic].Theater == 1 &  self.game.Data.RuleVar[227] == 0.0))
            {
              float num7 = 0.0f;
              let mut num8: i32 =  0;
              let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut index3: i32 =  0; index3 <= mapWidth; index3 += 1)
              {
                let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
                {
                  if (self.game.Data.MapObj[0].HexObj[index3, index4].Regime > -1 & self.game.Data.SFTypeObj[sfTypePic].Theater != 1)
                  {
                    if (self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index3, index4].Regime) && self.GetHexForceLandStrength(index3, index4, true) > 0)
                    {
                      float num9 = self.AverageCombatPerform(-1, sfTypePic, index3, index4, onlysametheater: true);
                      float num10 = self.AverageCombatPerform(-1, sfTypePic, index3, index4, true, true);
                      if ( num9 >  num10)
                        num7 +=  (( num9 * 3.0 +  num10) / 4.0);
                      else
                        num7 +=  (( num10 * 3.0 +  num9) / 4.0);
                      num8 += 1;
                    }
                  }
                  else if (self.game.Data.SFTypeObj[sfTypePic].Theater == 1 & self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index3, index4].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[index3, index4].UnitCounter > -1 && self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index3, index4].UnitList[0]].Regime != self.game.Data.Turn)
                  {
                    float num11 = self.AverageCombatPerform(-1, sfTypePic, index3, index4, onlysametheater: true);
                    float num12 = self.AverageCombatPerform(-1, sfTypePic, index3, index4, true, true);
                    if ( num11 >  num12)
                      num7 +=  (( num11 * 3.0 +  num12) / 4.0);
                    else
                      num7 +=  (( num12 * 3.0 +  num11) / 4.0);
                    num8 += 1;
                  }
                }
              }
              if (num8 > 0 &  num7 > 0.0)
                num6 = (int) Math.Round( ( num6 *  (1.0 / ( num7 /  num8))));
              let mut num13: i32 =  self.GetPowerPointPercentUpgradeableToo(sfTypePic);
              if (num13 > 25)
                num13 = 25;
              if (num13 < 1)
                num13 = 1;
              let mut Number2: i32 =  num6;
              let mut num14: i32 =  (int) Math.Round( (100 * num6) * 0.1 +  (100 * num6) * 0.9 * ( (25 - num13 + 1) / 25.0));
              let mut Number3: i32 =  (int) Math.Round( self.GetPowerPointPercentUpgradeableToo(sfTypePic) / 10.0);
              if (Number3 > 7)
                Number3 = 7;
              if (Number3 < 1)
                Number3 = 1;
              let mut num15: i32 =  (int) Math.Round( num14 /  Number3);
              if ( self.GetFriendlyAirRatio() <  self.game.Data.RuleVar[258] &  self.game.Data.Round >  self.game.Data.RuleVar[259] && self.game.Data.SFTypeObj[sfTypePic].AIRoleScore[12] > 0)
                num15 = (int) Math.Round( num15 / 4.0);
              if (self.game.Data.SFTypeObj[sfTypePic].Theater == 1)
              {
                if (1.0 / ( num7 /  num8) <=  Number1 * 0.9)
                {
                  simpleList.Add(tid, num15);
                  self.AddLog(self.game.Data.SFTypeObj[sfTypePic].Name + " .. weight =" + Conversion.Str( num15) + ", previous weight= " + Conversion.Str( Number2) + ",  powerpointpercentmod=" + Conversion.Str( Number3));
                }
              }
              else
              {
                simpleList.Add(tid, num15);
                self.AddLog(self.game.Data.SFTypeObj[sfTypePic].Name + " .. weight =" + Conversion.Str( num15) + ", previous weight= " + Conversion.Str( Number2) + ",  powerpointpercentmod=" + Conversion.Str( Number3));
              }
            }
          }
        }
      }
      if (simpleList.Counter <= -1)
        return;
      simpleList.Sort();
      let mut counter: i32 =  simpleList.Counter;
      for (let mut index5: i32 =  0; index5 <= counter; index5 += 1)
      {
        if (index5 < 9 &&  self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP >=  self.game.Data.ResearchObj[simpleList.Id[index5]].PointCost[self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.game.Data.Turn].People].PeopleGroup] *  self.game.Data.ResCostMod && !(self.game.Data.SFTypeObj[self.game.Data.ResearchObj[simpleList.Id[index5]].SFTypePic].Theater == 1 &  Number1 == 9999.0))
        {
          RegimeClass[] regimeObj3 = self.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray3 = regimeObj3;
          let mut turn3: i32 =  self.game.Data.Turn;
          let mut index6: i32 =  turn3;
          regimeClassArray3[index6].ResPts = regimeObj3[turn3].ResPts + self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP;
          let mut resPts2: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts;
          self.game.ProcessingObj.BuyResearch(self.game.Data.RegimeObj[self.game.Data.Turn].People, self.game.Data.Turn, simpleList.Id[index5]);
          self.AddLog("Bought researchfield: " + self.game.Data.ResearchObj[simpleList.Id[index5]].Name);
          if (self.game.Data.SFTypeObj[self.game.Data.ResearchObj[simpleList.Id[index5]].SFTypePic].Theater == 1)
            Number1 = 9999f;
          let mut resPts3: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts;
          RegimeClass[] regimeObj4 = self.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray4 = regimeObj4;
          let mut turn4: i32 =  self.game.Data.Turn;
          let mut index7: i32 =  turn4;
          regimeClassArray4[index7].AISavedPP = regimeObj4[turn4].AISavedPP - (resPts2 - resPts3);
          if (0 > self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP)
            self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP = 0;
          RegimeClass[] regimeObj5 = self.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray5 = regimeObj5;
          let mut turn5: i32 =  self.game.Data.Turn;
          let mut index8: i32 =  turn5;
          regimeClassArray5[index8].ResPts = regimeObj5[turn5].ResPts - self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP;
          if (0 > self.game.Data.RegimeObj[self.game.Data.Turn].ResPts)
            self.game.Data.RegimeObj[self.game.Data.Turn].ResPts = 0;
          let mut num16: i32 =  simpleList.Id[index5];
          let mut itemTypeCounter: i32 =  self.game.Data.ItemTypeCounter;
          for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
          {
            if (self.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[0] == num16 | self.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[1] == num16 | self.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[2] == num16 | self.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[3] == num16 | self.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[4] == num16 && self.game.Data.ItemTypeObj[itemtypenr].Blocks > -1)
            {
              let mut blocks: i32 =  self.game.Data.ItemTypeObj[itemtypenr].Blocks;
              let mut locCounter: i32 =  self.game.Data.LocCounter;
              for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
              {
                if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime == self.game.Data.Turn)
                {
                  let mut index9: i32 =  0;
                  do
                  {
                    if (self.game.Data.LocObj[locnr].Production[index9] == self.game.Data.ItemTypeObj[itemtypenr].Blocks && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, itemtypenr).result)
                    {
                      self.game.Data.LocObj[locnr].Production[index9] = itemtypenr;
                      num17: i32;
                      num17 += 1;
                    }
                    index9 += 1;
                  }
                  while (index9 <= 3);
                }
              }
            }
          }
        }
      }
    }

    pub fn GetPowerPointPercent(sftype: i32) -> i32
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (self.game.Data.UnitObj[index1].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index1].PreDef == -1)
        {
          let mut sfCount: i32 =  self.game.Data.UnitObj[index1].SFCount;
          for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[index1].SFList[index2];
            let mut type: i32 =  self.game.Data.SFObj[sf].Type;
            if (type == sftype)
              num1 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
            num2 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
          }
        }
      }
      let mut num3: i32 =  num1 * 100;
      if (num2 < 1)
        num2 = 1;
      return (int) Math.Round( num3 /  num2);
    }

    pub fn GetPowerPointPercentUpgradeableToo(sftype: i32) -> i32
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (self.game.Data.UnitObj[index1].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index1].PreDef == -1)
        {
          let mut sfCount: i32 =  self.game.Data.UnitObj[index1].SFCount;
          for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[index1].SFList[index2];
            let mut type: i32 =  self.game.Data.SFObj[sf].Type;
            if (self.game.Data.SFTypeObj[type].UpgradeToo == sftype)
              num1 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
            num2 += self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty;
          }
        }
      }
      let mut num3: i32 =  num1 * 100;
      if (num2 < 1)
        num2 = 1;
      return (int) Math.Round( num3 /  num2);
    }

    pub fn GetlandUnitsUnderHQ(unr: i32) -> i32
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num: i32;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index].HQ == unr && self.game.Data.UnitObj[index].PreDef == -1 & !self.game.Data.UnitObj[index].IsHQ && self.game.HandyFunctionsObj.HasUnitlandSF(unr))
          num += 1;
      }
      return num;
    }

    pub fn InitLandReserves()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      SimpleList simpleList1 = SimpleList::new();
      let mut saCount1: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount1; index += 1)
        self.SAObj[index].LandReservePlan = 0;
      simpleList1 = SimpleList::new();
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 =  0;
        SimpleList simpleList2 = SimpleList::new();
        let mut saCount2: i32 =  self.SACount;
        for (let mut index1: i32 =  1; index1 <= saCount2; index1 += 1)
        {
          if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[self.SAObj[index1].X, self.SAObj[index1].Y].Regime, self.game.Data.Turn) && self.SAObj[index1].ConstitutantCount == 0 & self.SAObj[index1].LandReservePlan == 0)
          {
            let mut num3: i32 =  9999;
            let mut num4: i32 =  0;
            let mut tplanCount: i32 =  self.TPlanCount;
            for (let mut index2: i32 =  1; index2 <= tplanCount; index2 += 1)
            {
              if (self.TPlanObj[index2].Type == 30 && self.HexOA[self.TPlanObj[index2].FromArea.X, self.TPlanObj[index2].FromArea.Y] == self.HexOA[self.SAObj[index1].X, self.SAObj[index1].Y])
              {
                self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 99, (int) Math.Round( self.game.Data.RuleVar[3]), self.TPlanObj[index2].FromArea.X, self.TPlanObj[index2].FromArea.Y, 0, istransfer: true, BridgeAP: 80);
                let mut num5: i32 =  self.game.EditObj.TempValue[0].Value[self.SAObj[index1].X, self.SAObj[index1].Y];
                if (num5 < num3)
                {
                  num4 = index2;
                  num3 = num5;
                }
              }
            }
            if (num4 > 0 &  num3 <=  self.game.Data.RuleVar[52])
            {
              self.SAObj[index1].LandReservePlan = num4;
            }
            else
            {
              num2 = 1;
              let mut num6: i32 =  1000;
              let mut num7: i32 =  self.GetClosestFrontlineDistance2(self.SAObj[index1].X, self.SAObj[index1].Y);
              if (num7 > 999)
                num7 = 0;
              let mut tweight: i32 =  num6 + num7;
              if (self.game.HandyFunctionsObj.IsHexAirfield(self.SAObj[index1].X, self.SAObj[index1].Y, 0))
                tweight += 200;
              if (self.game.HandyFunctionsObj.IsHexPort(self.SAObj[index1].X, self.SAObj[index1].Y, 0))
                tweight += 100 * self.SAObj[index1].SeaNeighbourCount;
              simpleList2.Add(index1, tweight, index1);
            }
          }
        }
        if (num2 == 1)
        {
          simpleList2.Sort();
          let mut Number: i32 =  simpleList2.Data1[simpleList2.Counter];
          self.AddtPlan();
          self.TPlanObj[self.TPlanCount].FromArea = self.SAObj[Number];
          self.TPlanObj[self.TPlanCount].TooArea = (SAClass) null;
          self.TPlanObj[self.TPlanCount].Type = 30;
          self.AddLog("created new PLANLANDRESERVE " + Conversion.Str( self.TPlanCount) + " at area# " + Conversion.Str( Number));
          num1 = 1;
        }
      }
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut Number1: i32 =  1; Number1 <= tplanCount1; Number1 += 1)
      {
        if (self.TPlanObj[Number1].Type == 30)
        {
          SimpleList simpleList3 = SimpleList::new();
          let mut saCount3: i32 =  self.SACount;
          for (let mut index: i32 =  1; index <= saCount3; index += 1)
          {
            if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[self.SAObj[index].X, self.SAObj[index].Y].Regime, self.game.Data.Turn) && self.SAObj[index].ConstitutantCount == 0 & self.SAObj[index].LandReservePlan == Number1)
            {
              let mut num8: i32 =  1000;
              let mut num9: i32 =  self.GetClosestFrontlineDistance2(self.SAObj[index].X, self.SAObj[index].Y);
              if (num9 > 999)
                num9 = 0;
              let mut num10: i32 =  num8 + num9;
              if (self.game.HandyFunctionsObj.IsHexAirfield(self.SAObj[index].X, self.SAObj[index].Y, 0))
                num10 += 500;
              if (self.game.HandyFunctionsObj.IsHexPort(self.SAObj[index].X, self.SAObj[index].Y, 0))
                num10 += 100 * self.SAObj[index].SeaNeighbourCount;
              if (self.game.Data.MapObj[0].HexObj[self.SAObj[index].X, self.SAObj[index].Y].Location > -1)
                num10 += self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.SAObj[index].X, self.SAObj[index].Y].Location].Type].MaxProd;
              let mut tweight: i32 =  num10 + self.game.Data.MapObj[0].HexObj[self.SAObj[index].X, self.SAObj[index].Y].VP * 100;
              simpleList3.Add(index, tweight, index);
            }
          }
          if (simpleList3.Counter > -1)
          {
            simpleList3.Sort();
            let mut Number2: i32 =  simpleList3.Data1[simpleList3.Counter];
            self.AddLog("Assigned Existing PLANLANDRESERVE " + Conversion.Str( Number1) + " the HQ AREA: " + Conversion.Str( Number2));
            self.TPlanObj[Number1].FromArea = self.SAObj[Number2];
          }
        }
      }
      let mut num11: i32 =  -1;
      let mut y1: i32 =  -1;
      let mut num12: i32 =  -1;
      SimpleList simpleList4 = SimpleList::new();
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut tid: i32 =  1; tid <= tplanCount2; tid += 1)
      {
        if (self.TPlanObj[tid].Type == 30)
        {
          self.TPlanObj[tid].ProdPts = 0;
          let mut saCount4: i32 =  self.SACount;
          for (let mut index3: i32 =  1; index3 <= saCount4; index3 += 1)
          {
            if (self.SAObj[index3].LandReservePlan == tid && self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Location > -1 && self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Regime == self.game.Data.Turn)
            {
              let mut type: i32 =  self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Location].Type;
              if (!self.game.Data.LocTypeObj[type].NoHQ)
              {
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index4: i32 =  tid;
                let mut index5: i32 =  index4;
                aiPlanClassArray[index5].ProdPts = tplanObj[index4].ProdPts + self.game.Data.LocTypeObj[type].MaxProd;
                if (self.game.Data.LocTypeObj[type].MaxProd > num12)
                {
                  num12 = self.game.Data.LocTypeObj[type].MaxProd;
                  num11 = self.SAObj[index3].X;
                  y1 = self.SAObj[index3].Y;
                }
              }
            }
          }
          if (self.TPlanObj[tid].HQ == -1 & self.TPlanObj[tid].ProdPts > 0)
          {
            self.SetMatrixEnemyFront(self.game.Data.Turn, true);
            simpleList4.Add(tid, self.TPlanObj[tid].ProdPts);
          }
        }
      }
      if (simpleList4.Counter <= -1)
        return;
      let mut counter: i32 =  simpleList4.Counter;
      for (let mut index6: i32 =  0; index6 <= counter; index6 += 1)
      {
        let mut x1: i32 =  self.TPlanObj[simpleList4.Id[index6]].FromArea.X;
        let mut y2: i32 =  self.TPlanObj[simpleList4.Id[index6]].FromArea.Y;
        SimpleList simpleList5 = SimpleList::new();
        self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, (int) Math.Round( self.game.Data.RuleVar[51]), x1, y2, 0, SeaBlock: true, BlockAllSea: true);
        let mut unitCounter: i32 =  self.game.Data.UnitCounter;
        for (let mut index7: i32 =  0; index7 <= unitCounter; index7 += 1)
        {
          if (self.game.Data.UnitObj[index7].AIPlanNr > 0 & self.game.Data.UnitObj[index7].IsHQ & self.game.Data.UnitObj[index7].Regime == self.game.Data.Turn && self.TPlanObj[self.game.Data.UnitObj[index7].AIPlanNr].Type != 30 &&  self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index7].X, self.game.Data.UnitObj[index7].Y] <=  self.game.Data.RuleVar[53])
          {
            let mut num13: i32 =  0;
            let mut num14: i32 =  0;
            let mut saCount5: i32 =  self.SACount;
            for (let mut index8: i32 =  1; index8 <= saCount5; index8 += 1)
            {
              if (self.SAObj[index8].LandReservePlan == simpleList4.Id[index6] && self.game.Data.MapObj[0].HexObj[self.SAObj[index8].X, self.SAObj[index8].Y].Location > -1 && self.game.Data.MapObj[0].HexObj[self.SAObj[index8].X, self.SAObj[index8].Y].Regime == self.game.Data.Turn && self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.SAObj[index8].X, self.SAObj[index8].Y].Location].Type].MaxProd > 0)
              {
                if (self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.SAObj[index8].X, self.SAObj[index8].Y].Location].HQ == index7)
                {
                  num13 += 1;
                  if (self.game.Data.Round == 1)
                    num13 += 1;
                }
                num14 += 1;
              }
            }
            let mut num15: i32 =  0;
            if (num13 > 0)
              num15 = (int) Math.Round( (-2 * self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index7].X, self.game.Data.UnitObj[index7].Y]) * ( num13 /  num14));
            let mut num16: i32 =  num15 - 150 * num13 + 10 * self.GetlandUnitsUnderHQ(index7) - 5 * self.GetClosestFrontlineDistance2(self.game.Data.UnitObj[index7].X, self.game.Data.UnitObj[index7].Y);
            let mut num17: i32 =  1;
            if (!self.game.HandyFunctionsObj.IsHexAirfield(self.game.Data.UnitObj[index7].X, self.game.Data.UnitObj[index7].Y, 0))
              num17 = 0;
            if (num17 == 1)
              simpleList5.Add(index7, num16 + self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index7].X, self.game.Data.UnitObj[index7].Y], self.game.Data.UnitObj[index7].AIPlanNr);
          }
        }
        if (simpleList5.Counter > -1)
        {
          simpleList5.Sort();
          self.TPlanObj[simpleList4.Id[index6]].HQ = simpleList5.Id[0];
          self.AddLog("Set HQ for LANDRESERVE to " + self.game.Data.UnitObj[simpleList5.Id[0]].Name);
          if (self.TPlanObj[simpleList5.Data1[0]].HQ == simpleList5.Id[0])
            self.TPlanObj[simpleList5.Data1[0]].HQ = -1;
          self.game.Data.UnitObj[simpleList5.Id[0]].AIPlanNr = simpleList4.Id[index6];
          tplanObj1: Vec<AIPlanClass> = self.TPlanObj;
          aiPlanClassArray1: Vec<AIPlanClass> = tplanObj1;
          int[] data1 = simpleList5.Data1;
          int[] numArray2 = data1;
          let mut index9: i32 =  0;
          let mut index10: i32 =  index9;
          let mut index11: i32 =  numArray2[index10];
          aiPlanClassArray1[index11].FriendlyUnitCount = tplanObj1[data1[index9]].FriendlyUnitCount - 1;
          tplanObj2: Vec<AIPlanClass> = self.TPlanObj;
          aiPlanClassArray2: Vec<AIPlanClass> = tplanObj2;
          int[] id = simpleList4.Id;
          int[] numArray3 = id;
          let mut index12: i32 =  index6;
          let mut index13: i32 =  index12;
          let mut index14: i32 =  numArray3[index13];
          aiPlanClassArray2[index14].FriendlyUnitCount = tplanObj2[id[index12]].FriendlyUnitCount + 1;
        }
        else
        {
          x2: i32;
          if ( self.game.Data.RuleVar[863] != 0.0)
          {
            RegimeClass[] regimeObj = self.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray = regimeObj;
            let mut turn: i32 =  self.game.Data.Turn;
            let mut index15: i32 =  turn;
            regimeClassArray[index15].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[47]));
            x2 = x1;
            y1 = y2;
          }
          else if ( self.game.Data.RuleVar[47] >  self.game.Data.RegimeObj[self.game.Data.Turn].ResPts)
          {
            RegimeClass[] regimeObj = self.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray = regimeObj;
            let mut turn: i32 =  self.game.Data.Turn;
            let mut index16: i32 =  turn;
            regimeClassArray[index16].AISavedPP = (int) Math.Round( ( regimeObj[turn].AISavedPP - self.game.Data.RuleVar[47]));
            if (self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP < 0)
              self.game.Data.RegimeObj[self.game.Data.Turn].AISavedPP = 0;
            x2 = x1;
            y1 = y2;
          }
          else
            x2 = -1;
          if (x2 > -1)
          {
            self.game.ProcessingObj.NewUnit(x2, y1, 0, true, self.game.Data.Turn);
            self.game.Data.UnitObj[self.game.Data.UnitCounter].Name = self.game.HandyFunctionsObj.GetHexName(x2, y1, 0) + " HQ";
            self.game.Data.UnitObj[self.game.Data.UnitCounter].AIPlanNr = simpleList4.Id[index6];
            self.TPlanObj[simpleList4.Id[index6]].HQ = self.game.Data.UnitCounter;
            self.AddLog("Set NEWLY CREATED HQ for LANDRESERVE to " + self.game.Data.UnitObj[self.game.Data.UnitCounter].Name);
            tplanObj: Vec<AIPlanClass> = self.TPlanObj;
            aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
            int[] id = simpleList4.Id;
            int[] numArray4 = id;
            let mut index17: i32 =  index6;
            let mut index18: i32 =  index17;
            let mut index19: i32 =  numArray4[index18];
            aiPlanClassArray[index19].FriendlyUnitCount = tplanObj[id[index17]].FriendlyUnitCount + 1;
            RegimeClass[] regimeObj = self.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray = regimeObj;
            let mut turn: i32 =  self.game.Data.Turn;
            let mut index20: i32 =  turn;
            regimeClassArray[index20].ResPts = (int) Math.Round( ( regimeObj[turn].ResPts + self.game.Data.RuleVar[47]));
          }
        }
      }
    }

    pub fn GetLandForcesNearHex(bool friendly, dist: i32, x: i32, y: i32) -> i32
    {
      let mut landForcesNearHex: i32 =  0;
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          let mut num: i32 =  0;
          if (friendly & self.game.Data.MapObj[0].HexObj[index1, index2].Regime == self.game.Data.Turn)
            num = 1;
          if (!friendly & self.game.Data.MapObj[0].HexObj[index1, index2].Regime > -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[index1, index2].Regime] == 0)
            num = 1;
          if (num == 1 && self.game.HandyFunctionsObj.Distance(x, y, 0, index1, index2, 0) <= dist)
            landForcesNearHex += self.GetHexForceLandStrength(index1, index2);
        }
      }
      return landForcesNearHex;
    }

    pub fn InitPlanLevelAnalysis()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      float aiConservative = self.game.Data.RegimeObj[self.game.Data.Turn].AIConservative;
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut plannr: i32 =  1; plannr <= tplanCount; plannr += 1)
      {
        if (self.TPlanObj[plannr].Type == 20)
        {
          let mut num1: i32 =  (int) Math.Round(Conversion.Int( (int) Math.Round( ( self.GetRealForceInArea(self.GetAreaNr(self.TPlanObj[plannr].FromArea), plannr, false) + self.TPlanObj[plannr].WeightFriendlyForce)) / 2.0));
          let mut num2: i32 =  (int) Math.Round( self.TPlanObj[plannr].WeightEnemyForceUnMod);
          if ( self.TPlanObj[plannr].WeightEnemyForce <  num2)
            num2 = (int) Math.Round( self.TPlanObj[plannr].WeightEnemyForce);
          let mut index: i32 =  self.game.Data.MapObj[0].HexObj[self.TPlanObj[plannr].TooArea.X, self.TPlanObj[plannr].TooArea.Y].Regime;
          let mut num3: i32 =  -1;
          if (index < 0)
          {
            index = 0;
            num3 = 1;
          }
          if (num3 == -1 & self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[index] > 0)
            self.TPlanObj[plannr].Stand = 2;
          else if ( num2 *  aiConservative <  num1)
            self.TPlanObj[plannr].Stand = 1;
          else if ( num2 *  aiConservative / 2.0 >  num1)
          {
            if ( num2 *  aiConservative / 4.0 >  num1)
            {
              let mut num4: i32 =  1;
              let mut areaNr: i32 =  self.GetAreaNr(self.TPlanObj[plannr].FromArea);
              if (self.SAObj[areaNr].LandReservePlan > 0 && self.GetAreaNr(self.TPlanObj[self.SAObj[areaNr].LandReservePlan].FromArea) == areaNr)
                num4 = 0;
              if (num4 == 1 && self.GetFriendlyAreaNeighbours(areaNr, true) < 1)
                num4 = 0;
              if (num4 == 1 && self.SAObj[areaNr].aivp > self.GetAverageAIVP())
                num4 = 0;
              if ( self.TPlanObj[plannr].WeightFriendlyForce * 2.0 >  self.TPlanObj[plannr].WeightEnemyForce)
                num4 = 1;
              self.TPlanObj[plannr].Stand = num4 != 1 ? 2 : 3;
            }
            else
              self.TPlanObj[plannr].Stand = 2;
          }
          else
            self.TPlanObj[plannr].Stand = 2;
        }
        else if (self.TPlanObj[plannr].Type == 40)
        {
          if (self.TPlanObj[plannr].FromArea.SeaNeighbourCount > 0)
          {
            if (self.TPlanObj[plannr].SeaStand == 0 | self.TPlanObj[plannr].SeaStand == 5 | self.TPlanObj[plannr].SeaStand == 4)
              self.TPlanObj[plannr].SeaStand =  self.TPlanObj[plannr].FriendlyNavy / 2.0 <  self.TPlanObj[plannr].EnemyNavy ? (self.TPlanObj[plannr].FriendlyNavy < self.TPlanObj[plannr].EnemyNavy ? ( self.TPlanObj[plannr].FriendlyNavy <  self.TPlanObj[plannr].EnemyNavy / 4.0 ? 4 : 5) : 6) : 7;
            else if (self.TPlanObj[plannr].SeaStand == 7)
              self.TPlanObj[plannr].SeaStand = self.TPlanObj[plannr].FriendlyNavy < self.TPlanObj[plannr].EnemyNavy ? ( self.TPlanObj[plannr].FriendlyNavy <  self.TPlanObj[plannr].EnemyNavy / 2.0 ? ( self.TPlanObj[plannr].FriendlyNavy <  self.TPlanObj[plannr].EnemyNavy / 8.0 ? 4 : 5) : 6) : 7;
            else if (self.TPlanObj[plannr].SeaStand == 6)
              self.TPlanObj[plannr].SeaStand =  self.TPlanObj[plannr].FriendlyNavy / 2.0 <  self.TPlanObj[plannr].EnemyNavy ? ( self.TPlanObj[plannr].FriendlyNavy <  self.TPlanObj[plannr].EnemyNavy / 2.0 ? ( self.TPlanObj[plannr].FriendlyNavy <  self.TPlanObj[plannr].EnemyNavy / 4.0 ? 4 : 5) : 6) : 7;
            if (self.getfrontplan(self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y) > 0)
            {
              if (self.TPlanObj[plannr].SeaStand == 4)
                self.TPlanObj[plannr].SeaStand = 5;
              else if (self.TPlanObj[plannr].SeaStand == 5)
                self.TPlanObj[plannr].SeaStand = 6;
            }
            if (self.TPlanObj[plannr].FriendlyNavy + self.TPlanObj[plannr].FriendlyAir <= 0)
              ;
          }
          else
            self.TPlanObj[plannr].Stand = 2;
          if ( self.game.Data.RuleVar[252] > 0.0)
          {
            let mut closestFrontline: i32 =  self.GetClosestFrontline(self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y);
            if (closestFrontline > 0)
            {
              let mut num: i32 =  self.AreaDistance(self.GetAreaNr(self.TPlanObj[plannr].FromArea), self.GetAreaNr(self.TPlanObj[closestFrontline].FromArea), true);
              if (num > 0 &  num <=  self.game.Data.RuleVar[252])
              {
                if (self.TPlanObj[closestFrontline].Stand != 1)
                  self.TPlanObj[plannr].AssemblyArea = 1;
                else if (self.TPlanObj[plannr].FromArea.SeaNeighbourCount < 1)
                {
                  let mut unitCounter: i32 =  self.game.Data.UnitCounter;
                  for (let mut index: i32 =  0; index <= unitCounter; index += 1)
                  {
                    if (self.game.Data.UnitObj[index].AIPlanNr == plannr && self.game.Data.UnitObj[index].AIUnitGoal == 1 | self.game.Data.UnitObj[index].AIUnitGoal == 2 && self.game.Data.Turn == self.game.Data.UnitObj[index].Regime & self.game.Data.UnitObj[index].PreDef == -1)
                    {
                      self.game.Data.UnitObj[index].AIPlanNr = -1;
                      self.game.Data.UnitObj[index].AIReserve = false;
                    }
                  }
                }
              }
            }
          }
          if ( self.game.Data.RuleVar[256] > 0.0 && self.game.Data.MapObj[0].HexObj[self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y].Location > -1 && self.game.HandyFunctionsObj.CanLocProduce(self.game.Data.MapObj[0].HexObj[self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y].Location, self.game.Data.Turn) &&  self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y].Location].Type].MaxProd >=  self.game.Data.RuleVar[256])
            self.TPlanObj[plannr].AssemblyArea = 1;
        }
      }
    }

    pub fn GetAverageAIVP() -> i32
    {
      if (self.SACount <= 0)
        return 0;
      let mut saCount: i32 =  self.SACount;
      num: i32;
      for (let mut index: i32 =  1; index <= saCount; index += 1)
        num += self.SAObj[index].aivp;
      let mut averageAivp: i32 =  (int) Math.Round( num /  self.SACount);
      if (0 > averageAivp)
        averageAivp = 0;
      return averageAivp;
    }

    pub fn InitTPlanForceBalance()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      int[] numArray2 = new int[self.SACount + 1];
      int[] numArray3 = new int[self.SACount + 1];
      int[] numArray4 = new int[self.SACount + 1];
      int[] numArray5 = new int[self.SACount + 1];
      let mut saCount: i32 =  self.SACount;
      for (let mut index1: i32 =  1; index1 <= saCount; index1 += 1)
      {
        let mut tplanCount: i32 =  self.TPlanCount;
        for (let mut index2: i32 =  1; index2 <= tplanCount; index2 += 1)
        {
          if (self.TPlanObj[index2].Type == 20 && self.TPlanObj[index2].TooArea.X == self.SAObj[index1].X & self.TPlanObj[index2].TooArea.Y == self.SAObj[index1].Y)
          {
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index3: i32 =  0; index3 <= mapWidth; index3 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
              {
                if (self.HexSA[index3, index4] > 0 & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index3, index4].LandscapeType].IsSea && self.SAObj[self.HexSA[index3, index4]].X == self.TPlanObj[index2].FromArea.X & self.SAObj[self.HexSA[index3, index4]].Y == self.TPlanObj[index2].FromArea.Y && self.HexPlan[index3, index4] == index2)
                {
                  int[] numArray6 = numArray4;
                  int[] numArray7 = numArray6;
                  let mut index5: i32 =  index1;
                  let mut index6: i32 =  index5;
                  let mut num: i32 =  numArray6[index5] + 1;
                  numArray7[index6] = num;
                }
              }
            }
          }
        }
        if (numArray4[index1] == 0)
          numArray4[index1] = 1;
        let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index7: i32 =  0; index7 <= mapWidth1; index7 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
          {
            if (self.HexSA[index7, index8] == index1 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index7, index8].LandscapeType].IsSea)
            {
              let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index7, index8].UnitCounter;
              for (let mut index9: i32 =  0; index9 <= unitCounter; index9 += 1)
              {
                int[] numArray8 = numArray2;
                int[] numArray9 = numArray8;
                let mut index10: i32 =  index1;
                let mut index11: i32 =  index10;
                let mut num1: i32 =  numArray8[index10] + self.GetForceLandStrength(self.game.Data.MapObj[0].HexObj[index7, index8].UnitList[index9]);
                numArray9[index11] = num1;
                int[] numArray10 = numArray3;
                int[] numArray11 = numArray10;
                let mut index12: i32 =  index1;
                let mut index13: i32 =  index12;
                let mut num2: i32 =  numArray10[index12] + self.GetForceLandStrength(self.game.Data.MapObj[0].HexObj[index7, index8].UnitList[index9], true);
                numArray11[index13] = num2;
                int[] numArray12 = numArray5;
                int[] numArray13 = numArray12;
                let mut index14: i32 =  index1;
                let mut index15: i32 =  index14;
                let mut num3: i32 =  numArray12[index14] + 1;
                numArray13[index15] = num3;
              }
            }
          }
        }
      }
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
      {
        let mut num4: i32 =  0;
        let mut num5: i32 =  0;
        float num6 = 0.0f;
        float num7 = 0.0f;
        let mut unitCounter: i32 =  self.game.Data.UnitCounter;
        for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
        {
          if (self.game.Data.UnitObj[unr].AIPlanNr == index & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && !self.game.Data.UnitObj[unr].IsHQ && self.game.Data.UnitObj[unr].AIUnitGoal != 4)
          {
            num4 += self.GetForceLandStrength(unr);
            num5 += 1;
          }
        }
        if (self.TPlanObj[index].Type == 20)
        {
          let mut areaNr: i32 =  self.GetAreaNr(self.TPlanObj[index].TooArea);
          num6 =  numArray2[areaNr] * ( self.TPlanObj[index].FrontSize /  numArray4[areaNr]);
          num7 =  numArray3[areaNr] * ( self.TPlanObj[index].FrontSize /  numArray4[areaNr]);
          if (self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime > -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime] > 0)
          {
            num6 *= self.game.Data.RuleVar[246];
            num7 *= self.game.Data.RuleVar[246];
          }
          self.TPlanObj[index].EnemyUnitCount = numArray5[areaNr];
        }
        if ( num6 > 99999.0)
          num6 = 99999f;
        if ( num7 > 99999.0)
          num7 = 99999f;
        self.TPlanObj[index].WeightFriendlyForce =  num4;
        self.TPlanObj[index].FriendlyUnitCount = num5;
        self.TPlanObj[index].WeightEnemyForce = num6;
        self.TPlanObj[index].WeightEnemyForceUnMod = num7;
      }
    }

    pub fn InitTPlanForceBalanceNavy()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      int[] numArray2 = new int[self.SACount + 1];
      int[] numArray3 = new int[self.SACount + 1];
      int[] numArray4 = new int[self.SACount + 1];
      int[] numArray5 = new int[self.SACount + 1];
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount1; index1 += 1)
      {
        if (self.TPlanObj[index1].Type == 40)
        {
          let mut navalTarget: i32 =  self.GetNavalTarget(index1);
          if (navalTarget > 0)
            self.TPlanObj[index1].SeaTarget = navalTarget;
          self.TPlanObj[index1].FriendlyNavy = self.GetRealNavalForceInArea(-1, index1, true, true);
          self.TPlanObj[index1].EnemyNavy = self.GetRealNavalForceInArea(self.GetAreaNr(self.TPlanObj[index1].FromArea), index1, true, false);
          if (navalTarget > 0)
          {
            tplanObj: Vec<AIPlanClass> = self.TPlanObj;
            aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
            let mut index2: i32 =  index1;
            let mut index3: i32 =  index2;
            aiPlanClassArray[index3].EnemyNavy = tplanObj[index2].EnemyNavy + self.GetRealNavalForceInArea(navalTarget, index1, true, false);
          }
          let mut tplanCount2: i32 =  self.TPlanCount;
          for (let mut index4: i32 =  1; index4 <= tplanCount2; index4 += 1)
          {
            if (self.TPlanObj[index4].Type != 40)
              ;
          }
        }
      }
      let mut tplanCount3: i32 =  self.TPlanCount;
      for (let mut index5: i32 =  1; index5 <= tplanCount3; index5 += 1)
      {
        if (self.TPlanObj[index5].Type == 40)
        {
          let mut tplanCount4: i32 =  self.TPlanCount;
          for (let mut plannr: i32 =  1; plannr <= tplanCount4; plannr += 1)
          {
            if (self.TPlanObj[plannr].Type == 40 && plannr != index5 && self.TPlanObj[index5].SeaTarget == self.TPlanObj[plannr].SeaTarget && self.TPlanObj[index5].SeaTarget > 0)
            {
              tplanObj: Vec<AIPlanClass> = self.TPlanObj;
              aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
              let mut index6: i32 =  index5;
              let mut index7: i32 =  index6;
              aiPlanClassArray[index7].FriendlyNavy = tplanObj[index6].FriendlyNavy + self.GetRealNavalForceInArea(-1, plannr, true, true);
            }
          }
        }
      }
    }

    pub fn InitEmergencyUnitSwitch()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.AddLog("");
      self.AddLog("EMERGENCY SWITCHES:");
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        self.game.Data.UnitObj[index].AICutoff = 0;
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index].AIPlanNr > 0)
        {
          let mut aiPlanNr: i32 =  self.game.Data.UnitObj[index].AIPlanNr;
          if (self.TPlanObj[aiPlanNr].Type == 20 && self.game.Data.UnitObj[index].SupplyInReq > 0 && self.game.Data.UnitObj[index].SupplyIn == 0 && self.SAObj[self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)].LandReservePlan > 0 && self.HexSA[self.game.Data.UnitObj[index].X, self.game.Data.UnitObj[index].Y] != self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea))
          {
            self.game.Data.UnitObj[index].AICutoff = 1;
            self.AddLog("Cutten off unit:" + self.game.Data.UnitObj[index].Name);
          }
        }
      }
    }

    pub fn InitUnits()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.AddLog("");
      self.AddLog("Consistency in planning. Assign units back to last plan if possible:");
      let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        self.game.Data.UnitObj[index].AIPlanNr = 0;
        if (self.game.Data.UnitObj[index].PreDef == -1 & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn)
          self.game.Data.UnitObj[index].AIPlanNr = 0;
      }
      let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter2; index1 += 1)
      {
        if (self.game.Data.UnitObj[index1].PreDef == -1 & self.game.Data.UnitObj[index1].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index1].AIPlanRef != null)
        {
          self.AddLog("evaluating: " + self.game.Data.UnitObj[index1].Name);
          self.AddLog("FromArea:#" + Conversion.Str( self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.FromArea)));
          if (self.game.Data.UnitObj[index1].AIPlanRef.TooArea != null)
            self.AddLog("TooArea:#" + Conversion.Str( self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea)));
          let mut tplanCount1: i32 =  self.TPlanCount;
          for (let mut Number: i32 =  1; Number <= tplanCount1; Number += 1)
          {
            if (self.TPlanObj[Number].Type == self.game.Data.UnitObj[index1].AIPlanRef.Type)
            {
              if (self.TPlanObj[Number].Type == 20 && self.GetAreaNr(self.TPlanObj[Number].FromArea) == self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.FromArea) && self.GetAreaNr(self.TPlanObj[Number].TooArea) == self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea))
              {
                self.game.Data.UnitObj[index1].AIPlanNr = Number;
                self.TPlanObj[Number].HQ = self.game.Data.UnitObj[index1].AIPlanRef.HQ;
                self.AddLog(self.game.Data.UnitObj[index1].Name + " to LANDFRONT plan# " + Conversion.Str( Number));
              }
              if (self.TPlanObj[Number].Type == 40 && self.GetAreaNr(self.TPlanObj[Number].FromArea) == self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.FromArea))
              {
                self.game.Data.UnitObj[index1].AIPlanNr = Number;
                self.TPlanObj[Number].CurrentBackRoad = self.game.Data.UnitObj[index1].AIPlanRef.CurrentBackRoad;
                self.TPlanObj[Number].HQ = self.game.Data.UnitObj[index1].AIPlanRef.HQ;
                self.TPlanObj[Number].SeaStand = self.game.Data.UnitObj[index1].AIPlanRef.SeaStand;
                self.TPlanObj[Number].SeaTarget = self.game.Data.UnitObj[index1].AIPlanRef.SeaTarget;
                self.AddLog(self.game.Data.UnitObj[index1].Name + " to BACK plan# " + Conversion.Str( Number));
              }
            }
          }
          let mut tplanCount2: i32 =  self.TPlanCount;
          for (let mut index2: i32 =  1; index2 <= tplanCount2; index2 += 1)
          {
            if (self.game.Data.UnitObj[index1].AIUnitGoal == 4 & self.game.Data.UnitObj[index1].AIPlanNr < 1 && self.game.Data.UnitObj[index1].AIPlanRef.Type == 20 | self.game.Data.UnitObj[index1].AIPlanRef.Type == 50 && self.AreaDistance(self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea), self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.FromArea)) == 1)
            {
              let mut x1: i32 =  self.game.Data.UnitObj[index1].X;
              let mut y1: i32 =  self.game.Data.UnitObj[index1].Y;
              let mut x2: i32 =  self.SAObj[self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea)].X;
              let mut y2: i32 =  self.SAObj[self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea)].Y;
              if (!(x1 == x2 & y1 == y2))
              {
                self.AddtPlan();
                self.TPlanObj[self.TPlanCount].Type = 50;
                self.TPlanObj[self.TPlanCount].FromArea = self.SAObj[self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.FromArea)];
                self.TPlanObj[self.TPlanCount].TooArea = self.SAObj[self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea)];
                self.game.Data.UnitObj[index1].AIPlanNr = self.TPlanCount;
              }
            }
          }
          if (self.game.Data.UnitObj[index1].AIPlanNr == 0)
          {
            let mut tplanCount3: i32 =  self.TPlanCount;
            for (let mut Number: i32 =  1; Number <= tplanCount3; Number += 1)
            {
              if (self.TPlanObj[Number].Type == self.game.Data.UnitObj[index1].AIPlanRef.Type && self.TPlanObj[Number].Type == 20 && self.GetAreaNr(self.TPlanObj[Number].FromArea) == self.HexSA[self.game.Data.UnitObj[index1].X, self.game.Data.UnitObj[index1].Y] && self.GetAreaNr(self.TPlanObj[Number].TooArea) == self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.TooArea))
              {
                self.game.Data.UnitObj[index1].AIPlanNr = Number;
                if (self.TPlanObj[Number].HQ == -1)
                  self.TPlanObj[Number].HQ = self.game.Data.UnitObj[index1].AIPlanRef.HQ;
                self.AddLog(self.game.Data.UnitObj[index1].Name + " to almost same plan# " + Conversion.Str( Number));
              }
            }
          }
          if (self.game.Data.UnitObj[index1].AIPlanNr == 0 && self.game.Data.UnitObj[index1].AIPlanRef.Type != 20 && self.game.Data.UnitObj[index1].AIPlanRef.Type == 30 && self.WhichCurrentAreaIsThis(ref self.game.Data.UnitObj[index1].AIPlanRef.FromArea) > 0)
          {
            self.AddtPlan();
            self.TPlanObj[self.TPlanCount] = self.game.Data.UnitObj[index1].AIPlanRef;
            self.TPlanObj[self.TPlanCount].FromArea = self.SAObj[self.WhichCurrentAreaIsThis(ref self.TPlanObj[self.TPlanCount].FromArea)];
            self.game.Data.UnitObj[index1].AIPlanNr = self.TPlanCount;
            self.AddLog("Initiated LANDRESERVEPLAN from memory by " + self.game.Data.UnitObj[index1].Name + " to plan# " + Conversion.Str( self.TPlanCount));
          }
        }
      }
      let mut unitCounter3: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter3; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn)
        {
          let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
          if (aiPlanNr > 0 && self.TPlanObj[aiPlanNr].Type == 40 & self.TPlanObj[aiPlanNr].SeaTarget > 0 && self.game.Data.UnitObj[unr].X > -1 && !self.game.HandyFunctionsObj.HasUnitNavySF(unr) & !self.game.HandyFunctionsObj.HasUnitAirSF(unr) && self.game.HandyFunctionsObj.HasUnitlandSF(unr) && self.HexSA[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] != self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea))
          {
            let mut tplanCount: i32 =  self.TPlanCount;
            for (let mut index: i32 =  1; index <= tplanCount; index += 1)
            {
              if (self.TPlanObj[index].Type == 20 && self.GetAreaNr(self.TPlanObj[index].FromArea) == self.HexSA[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] && self.GetAreaNr(self.TPlanObj[index].TooArea) == self.TPlanObj[aiPlanNr].SeaTarget)
              {
                self.game.Data.UnitObj[unr].AIPlanNr = index;
                break;
              }
            }
          }
        }
      }
      let mut unitCounter4: i32 =  self.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter4; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unr].AIPlanNr == 0)
        {
          if (self.game.HandyFunctionsObj.HasUnitAirSF(unr))
          {
            let mut closestBackPlan: i32 =  self.GetClosestBackPlan(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y);
            self.game.Data.UnitObj[unr].AIPlanNr = closestBackPlan;
          }
          else if (self.game.HandyFunctionsObj.HasUnitNavySF(unr))
          {
            let mut closestBackPlan: i32 =  self.GetClosestBackPlan(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y);
            self.game.Data.UnitObj[unr].AIPlanNr = closestBackPlan;
          }
          else
          {
            let mut num: i32 =  self.GetClosestFrontline(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y);
            if (num < 1)
              num = self.GetClosestBackPlan(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y);
            self.game.Data.UnitObj[unr].AIPlanNr = num;
          }
        }
      }
      let mut unitCounter5: i32 =  self.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter5; index += 1)
      {
        if (self.game.Data.UnitObj[index].PreDef == -1 & self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].AIPlanNr > 0 & self.game.Data.UnitObj[index].X > -1 && self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr].Type == 20)
          self.game.Data.UnitObj[index].AIReserve = false;
      }
    }

    pub fn BlowBridges()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if ( self.game.Data.RuleVar[245] < 1.0)
        return;
      let mut tplanCount1: i32 =  self.TPlanCount;
      for (let mut plnr: i32 =  1; plnr <= tplanCount1; plnr += 1)
      {
        if (self.TPlanObj[plnr].Type == 20 && self.TPlanObj[plnr].Stand == 2 && self.TPlanObj[plnr].RiverLine > 0)
        {
          let mut num1: i32 =  1;
          let mut tplanCount2: i32 =  self.TPlanCount;
          for (let mut index: i32 =  1; index <= tplanCount2; index += 1)
          {
            if (self.TPlanObj[index].Type == 20 && self.TPlanObj[index].Stand == 1 && self.GetAreaNr(self.TPlanObj[index].TooArea) == self.GetAreaNr(self.TPlanObj[plnr].TooArea))
              num1 = 0;
          }
          if (num1 == 1 &&  self.GivePercentBehindRiver(plnr) >= 0.75)
          {
            let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
              {
                if (self.HexSA[index1, index2] == self.GetAreaNr(self.TPlanObj[plnr].TooArea) && self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1)
                  numArray[index1, index2] = 1;
              }
            }
            let mut num2: i32 =  1;
            Coordinate coordinate;
            while (num2 == 1)
            {
              num2 = 0;
              let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut index3: i32 =  0; index3 <= mapWidth2; index3 += 1)
              {
                let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
                {
                  if (numArray[index3, index4] == 1)
                  {
                    let mut tfacing: i32 =  1;
                    do
                    {
                      coordinate = self.game.HandyFunctionsObj.HexNeighbour(index3, index4, 0, tfacing);
                      if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[plnr].FromArea) | self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[plnr].TooArea) && self.game.Data.MapObj[0].HexObj[index3, index4].RiverType[tfacing - 1] == -1 & self.game.HandyFunctionsObj.MoveApCostPreview2(index3, index4, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, index3, index4, 0, coordinate.x, coordinate.y, 0, false).x < 9999 && numArray[coordinate.x, coordinate.y] == 0)
                      {
                        numArray[coordinate.x, coordinate.y] = 1;
                        num2 = 1;
                      }
                      tfacing += 1;
                    }
                    while (tfacing <= 6);
                  }
                }
              }
            }
            let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index5: i32 =  0; index5 <= mapWidth3; index5 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
              {
                if (numArray[index5, index6] == 0)
                {
                  let mut tfacing: i32 =  1;
                  do
                  {
                    coordinate = self.game.HandyFunctionsObj.HexNeighbour(index5, index6, 0, tfacing);
                    if (coordinate.onmap && numArray[coordinate.x, coordinate.y] == 1 && self.game.Data.MapObj[0].HexObj[index5, index6].Regime == self.game.Data.Turn && self.game.Data.MapObj[0].HexObj[index5, index6].RiverType[tfacing - 1] > -1 && self.game.Data.MapObj[0].HexObj[index5, index6].Bridge[tfacing - 1])
                    {
                      self.game.Data.MapObj[0].HexObj[index5, index6].Bridge[tfacing - 1] = false;
                      let mut num3: i32 =  self.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, 0, index5, index6, 0);
                      self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Bridge[num3 - 1] = false;
                    }
                    tfacing += 1;
                  }
                  while (tfacing <= 6);
                }
              }
            }
          }
        }
      }
    }

    pub fn setrivermatrix(plnr: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut index1: i32 =  plnr;
      Coordinate coordinate;
      if (self.TPlanObj[index1].Type == 20 && self.TPlanObj[index1].Stand == 2 && self.TPlanObj[index1].FromArea.ConstitutantCount == 0)
      {
        let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth1; index2 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            if (self.HexSA[index2, index3] == self.GetAreaNr(self.TPlanObj[index1].TooArea) && self.game.Data.MapObj[0].HexObj[index2, index3].UnitCounter > -1)
              numArray[index2, index3] = 1;
          }
        }
        let mut num: i32 =  1;
        while (num == 1)
        {
          num = 0;
          let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index4: i32 =  0; index4 <= mapWidth2; index4 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
            {
              if (numArray[index4, index5] == 1)
              {
                let mut tfacing: i32 =  1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbour(index4, index5, 0, tfacing);
                  if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].FromArea) | self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].TooArea) && self.game.Data.MapObj[0].HexObj[index4, index5].RiverType[tfacing - 1] == -1 & self.game.HandyFunctionsObj.MoveApCostPreview2(index4, index5, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, index4, index5, 0, coordinate.x, coordinate.y, 0, false).x < 9999 && numArray[coordinate.x, coordinate.y] == 0)
                  {
                    numArray[coordinate.x, coordinate.y] = 1;
                    num = 1;
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
        }
      }
      let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index6: i32 =  0; index6 <= mapWidth3; index6 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
          self.Matrix1[index6, index7] = 0;
      }
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index8: i32 =  0; index8 <= mapWidth4; index8 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index9: i32 =  0; index9 <= mapHeight; index9 += 1)
        {
          if (self.HexSA[index8, index9] == self.GetAreaNr(self.TPlanObj[index1].FromArea) && numArray[index8, index9] == 0)
          {
            let mut num3: i32 =  0;
            let mut tfacing: i32 =  1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbour(index8, index9, 0, tfacing);
              if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].TooArea) | self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].FromArea) && numArray[coordinate.x, coordinate.y] > 0)
                num3 = 1;
              tfacing += 1;
            }
            while (tfacing <= 6);
            if (num3 == 1)
            {
              num2 += 1;
              num1 += self.game.HandyFunctionsObj.Distance(index8, index9, 0, self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y, 0);
            }
          }
        }
      }
      if (num2 <= 0)
        return;
      let mut num4: i32 =  (int) Math.Round( num1 /  num2);
      let mut mapWidth5: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index10: i32 =  0; index10 <= mapWidth5; index10 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index11: i32 =  0; index11 <= mapHeight; index11 += 1)
        {
          if (self.HexSA[index10, index11] == self.GetAreaNr(self.TPlanObj[index1].FromArea) && numArray[index10, index11] == 0)
          {
            let mut num5: i32 =  0;
            let mut num6: i32 =  0;
            let mut tfacing: i32 =  1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbour(index10, index11, 0, tfacing);
              if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].TooArea) | self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].FromArea) && numArray[coordinate.x, coordinate.y] > 0)
              {
                num5 = 1;
                if (self.game.Data.MapObj[0].HexObj[index10, index11].Bridge[tfacing - 1])
                  num6 += 1;
              }
              tfacing += 1;
            }
            while (tfacing <= 6);
            if (num5 == 1)
            {
              let mut num7: i32 =  self.game.HandyFunctionsObj.Distance(index10, index11, 0, self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y, 0);
              if (num7 == 0)
                num7 = 1;
              self.Matrix1[index10, index11] = (int) Math.Round( self.game.Data.RuleVar[152] * ( num4 /  num7));
              if (num6 > 0)
                self.Matrix1[index10, index11] = self.Matrix1[index10, index11] * (num6 * 2);
            }
          }
        }
      }
    }

    pub float GivePercentBehindRiver(plnr: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut index1: i32 =  plnr;
      if (self.TPlanObj[index1].Type == 20 && self.TPlanObj[index1].Stand == 2 && self.TPlanObj[index1].FromArea.ConstitutantCount == 0)
      {
        let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth1; index2 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            if (self.HexSA[index2, index3] == self.GetAreaNr(self.TPlanObj[index1].TooArea) && self.game.Data.MapObj[0].HexObj[index2, index3].UnitCounter > -1)
              numArray[index2, index3] = 1;
          }
        }
        let mut num: i32 =  1;
        while (num == 1)
        {
          num = 0;
          let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index4: i32 =  0; index4 <= mapWidth2; index4 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
            {
              if (numArray[index4, index5] == 1)
              {
                let mut tfacing: i32 =  1;
                do
                {
                  Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(index4, index5, 0, tfacing);
                  if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].FromArea) | self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].TooArea) && self.game.Data.MapObj[0].HexObj[index4, index5].RiverType[tfacing - 1] == -1 & self.game.HandyFunctionsObj.MoveApCostPreview2(index4, index5, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, index4, index5, 0, coordinate.x, coordinate.y, 0, false).x < 9999 && numArray[coordinate.x, coordinate.y] == 0)
                  {
                    numArray[coordinate.x, coordinate.y] = 1;
                    num = 1;
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
        }
      }
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].AIPlanNr == index1 && self.game.Data.UnitObj[unr].X > -1)
        {
          if (numArray[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] == 1)
            num1 += self.game.HandyFunctionsObj.GetPower(unr, self.game.Data.Turn);
          else
            num2 += self.game.HandyFunctionsObj.GetPower(unr, self.game.Data.Turn);
        }
      }
      return num1 + num2 > 0 ?  num2 /  (num1 + num2) : 0.0f;
    }

    pub fn InitRiverLine()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount; index1 += 1)
      {
        self.TPlanObj[index1].RiverLine = 0;
        if (self.TPlanObj[index1].Type == 20 && self.TPlanObj[index1].Stand == 2 && self.TPlanObj[index1].FromArea.ConstitutantCount == 0)
        {
          let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index2: i32 =  0; index2 <= mapWidth1; index2 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            {
              if (self.HexSA[index2, index3] == self.GetAreaNr(self.TPlanObj[index1].TooArea) && self.game.Data.MapObj[0].HexObj[index2, index3].UnitCounter > -1)
                numArray[index2, index3] = 1;
            }
          }
          let mut num: i32 =  1;
          while (num == 1)
          {
            num = 0;
            let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index4: i32 =  0; index4 <= mapWidth2; index4 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
              {
                if (numArray[index4, index5] == 1)
                {
                  let mut tfacing: i32 =  1;
                  do
                  {
                    Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(index4, index5, 0, tfacing);
                    if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].FromArea) | self.HexSA[coordinate.x, coordinate.y] == self.GetAreaNr(self.TPlanObj[index1].TooArea) && self.game.Data.MapObj[0].HexObj[index4, index5].RiverType[tfacing - 1] == -1 & self.game.HandyFunctionsObj.MoveApCostPreview2(index4, index5, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[0]), 0, index4, index5, 0, coordinate.x, coordinate.y, 0, false).x < 9999 && numArray[coordinate.x, coordinate.y] == 0)
                    {
                      numArray[coordinate.x, coordinate.y] = 1;
                      num = 1;
                    }
                    tfacing += 1;
                  }
                  while (tfacing <= 6);
                }
              }
            }
          }
          if (numArray[self.TPlanObj[index1].FromArea.X, self.TPlanObj[index1].FromArea.Y] == 0)
            self.TPlanObj[index1].RiverLine = 1;
        }
      }
    }

    pub fn InitPlanFrontline()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut index1: i32 =  1; index1 <= tplanCount; index1 += 1)
      {
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (self.HexSA[cx, cy] == self.GetSANr(self.TPlanObj[index1].FromArea))
            {
              if (self.TPlanObj[index1].Type == 20)
              {
                let mut tfacing: i32 =  1;
                do
                {
                  Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                  if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] == self.GetSANr(self.TPlanObj[index1].TooArea) && self.HexPlan[cx, cy] <= 0)
                  {
                    self.HexPlan[cx, cy] = index1;
                    tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                    aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                    let mut index2: i32 =  index1;
                    let mut index3: i32 =  index2;
                    aiPlanClassArray[index3].FrontSize = tplanObj[index2].FrontSize + 1;
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
              if (self.TPlanObj[index1].Type == 40)
              {
                self.HexBackPlan[cx, cy] = index1;
                tplanObj: Vec<AIPlanClass> = self.TPlanObj;
                aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
                let mut index4: i32 =  index1;
                let mut index5: i32 =  index4;
                aiPlanClassArray[index5].FrontSize = tplanObj[index4].FrontSize + 1;
              }
            }
          }
        }
      }
    }

    pub fn GetMostUsedHQ(plannr: i32) -> i32
    {
      SimpleList simpleList = SimpleList::new();
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (self.game.Data.UnitObj[index1].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index1].X > -1 && self.game.Data.UnitObj[index1].AIPlanNr == plannr)
        {
          let mut hq: i32 =  self.game.Data.UnitObj[index1].HQ;
          if (hq > -1 && self.game.Data.UnitObj[hq].AIPlanNr == plannr)
          {
            let mut nr: i32 =  simpleList.FindNr(hq);
            if (nr == -1)
            {
              simpleList.Add(hq, 1);
            }
            else
            {
              int[] weight = simpleList.Weight;
              int[] numArray = weight;
              let mut index2: i32 =  nr;
              let mut index3: i32 =  index2;
              let mut num: i32 =  weight[index2] + 1;
              numArray[index3] = num;
            }
          }
        }
      }
      if (simpleList.Counter <= -1)
        return -1;
      simpleList.Sort();
      return simpleList.Id[simpleList.Counter];
    }

    pub fn InitTPlanAPCost()
    {
      let mut tplanCount1: i32 =  self.TPlanCount;
      index1: i32;
      x1: i32;
      y1: i32;
      for (let mut plannr: i32 =  1; plannr <= tplanCount1; plannr += 1)
      {
        if (self.TPlanObj[plannr].Type == 20 & self.TPlanObj[plannr].FriendlyUnitCount > 0 && self.TPlanObj[plannr].TooArea.ConstitutantCount < 1)
        {
          self.TPlanObj[plannr].CurrentAPCost = 0;
          self.TPlanObj[plannr].PossibleAPCost = 0;
          self.TPlanObj[plannr].AverageUnitAPCost = 0;
          index1 = self.GetMostUsedHQ(plannr);
          x2: i32;
          y2: i32;
          if (index1 > -1)
          {
            if (self.game.Data.UnitObj[index1].HQ > -1 & self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].Type != 30)
              index1 = self.game.Data.UnitObj[index1].HQ;
            if (self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].Type == 30)
            {
              x2 = self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].FromArea.X;
              y2 = self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].FromArea.Y;
              x1 = self.TPlanObj[plannr].FromArea.X;
              y1 = self.TPlanObj[plannr].FromArea.Y;
            }
            else
            {
              x2 = self.TPlanObj[plannr].FromArea.X;
              y2 = self.TPlanObj[plannr].FromArea.Y;
              x1 = self.TPlanObj[plannr].FromArea.X;
              y1 = self.TPlanObj[plannr].FromArea.Y;
            }
          }
          else
          {
            x2 = self.TPlanObj[plannr].FromArea.X;
            y2 = self.TPlanObj[plannr].FromArea.Y;
            x1 = self.TPlanObj[plannr].FromArea.X;
            y1 = self.TPlanObj[plannr].FromArea.Y;
          }
          let mut num1: i32 =  self.game.HandyFunctionsObj.Distance(x2, y2, 0, x1, y1, 0);
          if (num1 < 8)
            num1 = 8;
          let mut MaxDistance: i32 =  num1 * 2;
          if (!(x2 == x1 & y2 == y1))
          {
            self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x2, y2, 0, dontenterenemy: false, NoAPPenalties: true, SeaBlock: true, BlockAllSea: true, MaxDistance: MaxDistance);
            tplanObj: Vec<AIPlanClass> = self.TPlanObj;
            aiPlanClassArray: Vec<AIPlanClass> = tplanObj;
            let mut index2: i32 =  plannr;
            let mut index3: i32 =  index2;
            aiPlanClassArray[index3].CurrentAPCost = tplanObj[index2].CurrentAPCost + self.game.EditObj.TempValue[0].Value[x1, y1];
          }
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x1, y1, 0, dontenterenemy: false, NoAPPenalties: true, SeaBlock: true, BlockAllSea: true, MaxDistance: MaxDistance);
          tplanObj1: Vec<AIPlanClass> = self.TPlanObj;
          aiPlanClassArray1: Vec<AIPlanClass> = tplanObj1;
          let mut index4: i32 =  plannr;
          let mut index5: i32 =  index4;
          aiPlanClassArray1[index5].CurrentAPCost = tplanObj1[index4].CurrentAPCost + self.game.EditObj.TempValue[0].Value[self.TPlanObj[plannr].TooArea.X, self.TPlanObj[plannr].TooArea.Y];
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          num2: i32;
          num3: i32;
          for (let mut index6: i32 =  0; index6 <= unitCounter; index6 += 1)
          {
            if (self.game.Data.UnitObj[index6].AIPlanNr == plannr && self.game.Data.UnitObj[index6].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index6].X > -1 && self.HexSA[self.game.Data.UnitObj[index6].X, self.game.Data.UnitObj[index6].Y] == self.GetAreaNr(self.TPlanObj[plannr].FromArea))
            {
              num2 += self.game.EditObj.TempValue[0].Value[self.game.Data.UnitObj[index6].X, self.game.Data.UnitObj[index6].Y];
              num3 += 1;
            }
          }
          if (num3 < 1)
            num3 = 1;
          self.TPlanObj[plannr].AverageUnitAPCost = (int) Math.Round(Conversion.Int( num2 /  num3));
          if (!(x2 == x1 & y2 == y1))
          {
            self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x2, y2, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true, MaxDistance: MaxDistance);
            tplanObj2: Vec<AIPlanClass> = self.TPlanObj;
            aiPlanClassArray2: Vec<AIPlanClass> = tplanObj2;
            let mut index7: i32 =  plannr;
            let mut index8: i32 =  index7;
            aiPlanClassArray2[index8].PossibleAPCost = tplanObj2[index7].PossibleAPCost + self.game.EditObj.TempValue[0].Value[x1, y1];
          }
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x1, y1, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true, MaxDistance: MaxDistance);
          tplanObj3: Vec<AIPlanClass> = self.TPlanObj;
          aiPlanClassArray3: Vec<AIPlanClass> = tplanObj3;
          let mut index9: i32 =  plannr;
          let mut index10: i32 =  index9;
          aiPlanClassArray3[index10].PossibleAPCost = tplanObj3[index9].PossibleAPCost + self.game.EditObj.TempValue[0].Value[self.TPlanObj[plannr].TooArea.X, self.TPlanObj[plannr].TooArea.Y];
        }
      }
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut tplanCount2: i32 =  self.TPlanCount;
      for (let mut index11: i32 =  1; index11 <= tplanCount2; index11 += 1)
      {
        if (self.TPlanObj[index11].Type == 40 & self.TPlanObj[index11].FriendlyUnitCount > 0)
        {
          let mut x3: i32 =  self.TPlanObj[index11].FromArea.X;
          let mut y3: i32 =  self.TPlanObj[index11].FromArea.Y;
          let mut num4: i32 =  0;
          let mut index12: i32 =  0;
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x3, y3, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true);
          let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index13: i32 =  0; index13 <= mapWidth; index13 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
              numArray[index13, index14] = self.game.EditObj.TempValue[0].Value[index13, index14];
          }
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x3, y3, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true, MaxDistance: 20);
          if (self.TPlanObj[index11].CurrentBackRoad > 0)
          {
            let mut x4: i32 =  self.SAObj[self.TPlanObj[index11].CurrentBackRoad].X;
            let mut y4: i32 =  self.SAObj[self.TPlanObj[index11].CurrentBackRoad].Y;
            if (self.game.EditObj.TempValue[0].Value[x4, y4] >= numArray[x4, y4])
              self.TPlanObj[index11].CurrentBackRoad = 0;
          }
          let mut neighbourCount: i32 =  self.TPlanObj[index11].FromArea.NeighbourCount;
          for (let mut index15: i32 =  1; index15 <= neighbourCount; index15 += 1)
          {
            index1 = self.TPlanObj[index11].FromArea.Neighbour[index15];
            if (self.GetAreaNr(self.TPlanObj[index11].FromArea) != index1 && self.HexOA[self.SAObj[index1].X, self.SAObj[index1].Y] > 0)
            {
              x1 = self.SAObj[index1].X;
              y1 = self.SAObj[index1].Y;
              let mut num5: i32 =  1;
              let mut tplanCount3: i32 =  self.TPlanCount;
              for (let mut index16: i32 =  1; index16 <= tplanCount3; index16 += 1)
              {
                if (self.TPlanObj[index16].Type == 40 & index16 != index11 && self.TPlanObj[index16].CurrentBackRoad == self.GetAreaNr(self.TPlanObj[index11].FromArea) && index1 == self.GetAreaNr(self.TPlanObj[index16].FromArea))
                  num5 = 0;
              }
              if (num5 == 1 && self.game.EditObj.TempValue[0].Value[x1, y1] < numArray[x1, y1])
              {
                let mut num6: i32 =  numArray[x1, y1] - self.game.EditObj.TempValue[0].Value[x1, y1];
                if (num6 > num4)
                {
                  num4 = num6;
                  index12 = index1;
                }
              }
            }
          }
          let mut num7: i32 =  0;
          if (num4 > 0)
          {
            num7 = 1;
            x1 = self.SAObj[index12].X;
            y1 = self.SAObj[index12].Y;
            index1 = index12;
          }
          if (num7 == 1)
          {
            if (self.TPlanObj[index11].CurrentBackRoad > 0)
            {
              let mut x5: i32 =  self.SAObj[self.TPlanObj[index11].CurrentBackRoad].X;
              let mut y5: i32 =  self.SAObj[self.TPlanObj[index11].CurrentBackRoad].Y;
              if (self.game.EditObj.TempValue[0].Value[x5, y5] < numArray[x5, y5])
              {
                self.TPlanObj[index11].PossibleAPCost = self.game.EditObj.TempValue[0].Value[x5, y5];
                self.TPlanObj[index11].CurrentAPCost = numArray[x5, y5];
              }
              else
              {
                self.TPlanObj[index11].PossibleAPCost = self.game.EditObj.TempValue[0].Value[x1, y1];
                self.TPlanObj[index11].CurrentAPCost = numArray[x1, y1];
                self.TPlanObj[index11].CurrentBackRoad = index1;
              }
            }
            else
            {
              self.TPlanObj[index11].PossibleAPCost = self.game.EditObj.TempValue[0].Value[x1, y1];
              self.TPlanObj[index11].CurrentAPCost = numArray[x1, y1];
              self.TPlanObj[index11].CurrentBackRoad = index1;
            }
          }
          else
          {
            self.TPlanObj[index11].CurrentAPCost = 0;
            self.TPlanObj[index11].PossibleAPCost = 0;
          }
        }
      }
    }

    pub fn getDistanceClosestUnit(fromarea: i32, towardsarea: i32) -> i32
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut Right: i32 =  0;
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          objArray[index1, index2] =  -1;
          if (self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 && self.HexSA[index1, index2] == fromarea)
            objArray[index1, index2] =  0;
        }
      }
      for (let mut index: i32 =  1; index == 1 & Right < 99; Right += 1)
      {
        index = 0;
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(objArray[cx, cy],  Right, false))
            {
              if (self.HexSA[cx, cy] == towardsarea)
                return Right;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                {
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                  index = 1;
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      return 999;
    }

    pub fn InitTPlanStrategicImportance()
    {
      self.AverageFuzzyVP();
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount; index += 1)
      {
        if (self.TPlanObj[index].Type == 20)
        {
          let mut fuzzyvp1: i32 =  self.TPlanObj[index].FromArea.fuzzyvp;
          let mut fuzzyvp2: i32 =  self.TPlanObj[index].TooArea.fuzzyvp;
          self.TPlanObj[index].WeightStrategic = fuzzyvp1 + fuzzyvp2;
          if (self.SAObj[self.GetAreaNr(self.TPlanObj[index].TooArea)].ConstitutantCount > 0)
          {
            if (!self.IsAreaEmpty(self.GetAreaNr(self.TPlanObj[index].TooArea)))
            {
              let mut num: i32 =  (int) Math.Round(Math.Pow( self.getDistanceClosestUnit(self.GetAreaNr(self.TPlanObj[index].TooArea), self.GetAreaNr(self.TPlanObj[index].FromArea)), 2.0));
              self.TPlanObj[index].WeightStrategic = (int) Math.Round( self.TPlanObj[index].WeightStrategic /  num);
              if (self.TPlanObj[index].WeightStrategic < 1)
                self.TPlanObj[index].WeightStrategic = 1;
              if (self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime == -1 & self.TPlanObj[index].TooArea.aivp == 0)
                self.TPlanObj[index].WeightStrategic = (int) Math.Round( self.TPlanObj[index].WeightStrategic / 3.0);
              else if (self.TPlanObj[index].EnemyUnitCount == 0 & self.TPlanObj[index].TooArea.aivp == 0)
                self.TPlanObj[index].WeightStrategic = (int) Math.Round( self.TPlanObj[index].WeightStrategic / 3.0);
              else if (self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime] > 0)
                self.TPlanObj[index].WeightStrategic = (int) Math.Round( ( self.TPlanObj[index].WeightStrategic * self.game.Data.RuleVar[246]));
              else
                self.TPlanObj[index].WeightStrategic *= 3;
            }
            else
              self.TPlanObj[index].WeightStrategic = 0;
          }
          else if (self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime != -1)
          {
            if (self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime] > 0)
              self.TPlanObj[index].WeightStrategic = (int) Math.Round( ( self.TPlanObj[index].WeightStrategic * self.game.Data.RuleVar[246]));
            else if (self.GetFriendlyAreaNeighbours(self.GetAreaNr(self.TPlanObj[index].TooArea), false) == 0)
              self.TPlanObj[index].WeightStrategic = (int) Math.Round( self.TPlanObj[index].WeightStrategic * 1.25);
          }
        }
        else
        {
          self.TPlanObj[index].WeightStrategic = self.SAObj[self.GetAreaNr(self.TPlanObj[index].FromArea)].aivp + self.SAObj[self.GetAreaNr(self.TPlanObj[index].FromArea)].fuzzyvp;
          if (1 > self.TPlanObj[index].WeightStrategic)
            self.TPlanObj[index].WeightStrategic = 1;
        }
        if (self.TPlanObj[index].Type == 20 && self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime != -1 && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[self.game.Data.MapObj[0].HexObj[self.TPlanObj[index].TooArea.X, self.TPlanObj[index].TooArea.Y].Regime] == 0)
        {
          Random random = new Random(self.game.Data.GameID);
          float num1 =  random.Next(1, 100) / 100f;
          float num2 =  random.Next(1, 100) / 100f;
          float num3 =  random.Next(1, 100) / 100f;
          if ( num1 < 0.25)
            self.TPlanObj[index].WeightStrategic = (int) Math.Round( ( self.TPlanObj[index].WeightStrategic * (num2 * 3f)));
          else if ( num1 > 0.75)
            self.TPlanObj[index].WeightStrategic = (int) Math.Round( ( self.TPlanObj[index].WeightStrategic / (num2 * 3f)));
          if (1 > self.TPlanObj[index].WeightStrategic)
            self.TPlanObj[index].WeightStrategic = 1;
        }
      }
    }

    pub IsAreaEmpty: bool(areanr: i32)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSA[index1, index2] == areanr && (self.game.Data.MapObj[0].HexObj[index1, index2].Location > -1 || self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1))
            return false;
        }
      }
      return true;
    }

    pub fn InitTPlans()
    {
      let mut saCount1: i32 =  self.SACount;
      for (let mut index1: i32 =  1; index1 <= saCount1; index1 += 1)
      {
        if (self.HexOA[self.SAObj[index1].X, self.SAObj[index1].Y] > 0)
        {
          let mut neighbourCount: i32 =  self.SAObj[index1].NeighbourCount;
          for (let mut index2: i32 =  1; index2 <= neighbourCount; index2 += 1)
          {
            let mut index3: i32 =  self.SAObj[index1].Neighbour[index2];
            if (self.HexOA[self.SAObj[index3].X, self.SAObj[index3].Y] == 0)
            {
              let mut regime: i32 =  self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Regime;
              let mut num: i32 =  1;
              if (regime > -1)
              {
                if (self.game.Data.RegimeObj[regime].Sleep & self.game.Data.RegimeObj[regime].DipBlock &  self.game.Data.RuleVar[263] == 0.0)
                  num = 0;
                if (self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regime] == 0 & num == 0)
                  num = 1;
              }
              if (num == 1)
              {
                self.AddtPlan();
                self.TPlanObj[self.TPlanCount].Type = 20;
                self.TPlanObj[self.TPlanCount].FromArea = self.SAObj[index1].Clone();
                self.TPlanObj[self.TPlanCount].TooArea = self.SAObj[index3].Clone();
              }
            }
          }
        }
      }
      let mut saCount2: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount2; index += 1)
      {
        if (self.HexOA[self.SAObj[index].X, self.SAObj[index].Y] > 0 & self.game.Data.MapObj[0].HexObj[self.SAObj[index].X, self.SAObj[index].Y].Regime == self.game.Data.Turn && self.AIVP[self.SAObj[index].X, self.SAObj[index].Y] > 0)
        {
          self.AddtPlan();
          self.TPlanObj[self.TPlanCount].Type = 40;
          self.TPlanObj[self.TPlanCount].FromArea = self.SAObj[index].Clone();
        }
      }
    }

    pub fn InitDeclareWar()
    {
      int[] numArray1 = new int[self.game.Data.RegimeCounter + 1];
      int[] numArray2 = new int[self.game.Data.RegimeCounter + 1];
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
        {
          let mut regime1: i32 =  self.game.Data.MapObj[0].HexObj[cx, cy].Regime;
          if (regime1 > -1)
          {
            let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[cx, cy].UnitCounter;
            for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
            {
              let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[cx, cy].UnitList[index1];
              int[] numArray3 = numArray1;
              int[] numArray4 = numArray3;
              let mut regime2: i32 =  self.game.Data.UnitObj[unit].Regime;
              let mut index2: i32 =  regime2;
              let mut num1: i32 =  numArray3[regime2] + self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unit);
              numArray4[index2] = num1;
              let mut regimeCounter: i32 =  self.game.Data.RegimeCounter;
              for (let mut index3: i32 =  0; index3 <= regimeCounter; index3 += 1)
              {
                if (self.game.Data.RegimeObj[index3].RegimeRel[self.game.Data.UnitObj[unit].Regime] == 2 && index3 != self.game.Data.UnitObj[unit].Regime)
                {
                  int[] numArray5 = numArray1;
                  int[] numArray6 = numArray5;
                  let mut index4: i32 =  index3;
                  let mut index5: i32 =  index4;
                  let mut num2: i32 =  numArray5[index4] + self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unit);
                  numArray6[index5] = num2;
                }
              }
            }
            if (regime1 != self.game.Data.Turn)
            {
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == self.game.Data.Turn)
                {
                  int[] numArray7 = numArray2;
                  int[] numArray8 = numArray7;
                  let mut index6: i32 =  regime1;
                  let mut index7: i32 =  index6;
                  let mut num: i32 =  numArray7[index6] + 1;
                  numArray8[index7] = num;
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      let mut num3: i32 =  0;
      let mut regimeCounter1: i32 =  self.game.Data.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter1; index += 1)
      {
        if (index != self.game.Data.Turn && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[index] == 0)
          num3 += numArray1[index];
      }
      let mut onregnr1: i32 =  -1;
      let mut num4: i32 =  0;
      let mut num5: i32 =  0;
      if (numArray1[self.game.Data.Turn] <= num3)
        return;
      let mut regimeCounter2: i32 =  self.game.Data.RegimeCounter;
      num6: i32;
      for (let mut index: i32 =  0; index <= regimeCounter2; index += 1)
      {
        if (!self.game.Data.RegimeObj[index].DipBlock & !self.game.Data.RegimeObj[index].Sleep && index != self.game.Data.Turn & self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[index] == 1)
        {
          num6 = 0;
          if (numArray1[index] + num3 < numArray1[self.game.Data.Turn])
          {
            let mut num7: i32 =  (int) Math.Round( numArray2[index] * ( numArray1[self.game.Data.Turn] /  numArray1[index]));
            num6 = (int) Math.Round( num7 * 0.5 +  num7 *  VBMath.Rnd() * 0.5);
            if (!self.game.Data.RegimeObj[index].AI)
              num6 *= 2;
          }
          if (numArray2[index] > 0)
            num5 = 1;
          if (num6 > num4)
          {
            num4 = num6;
            onregnr1 = index;
          }
        }
      }
      if (num5 == 1 &&  self.game.Data.RuleVar[903] > 100.0 *  VBMath.Rnd() &&  self.game.Data.RuleVar[903] > 100.0 *  VBMath.Rnd())
        num5 = 0;
      if (onregnr1 == -1 & num5 == 0)
      {
        self.game.HandyFunctionsObj.SetInitialXY(self.game.Data.Turn);
        let mut selectX: i32 =  self.game.SelectX;
        let mut selectY: i32 =  self.game.SelectY;
        let mut regimeCounter3: i32 =  self.game.Data.RegimeCounter;
        for (let mut regnr: i32 =  0; regnr <= regimeCounter3; regnr += 1)
        {
          if (!self.game.Data.RegimeObj[regnr].DipBlock & !self.game.Data.RegimeObj[regnr].Sleep && regnr != self.game.Data.Turn & self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regnr] == 1)
          {
            if (numArray1[regnr] + num3 < numArray1[self.game.Data.Turn])
            {
              self.game.HandyFunctionsObj.SetInitialXY(regnr);
              num6 = (int) Math.Round( ( (int) Math.Round(100.0 - Math.Sqrt( self.game.HandyFunctionsObj.Distance(selectX, selectY, 0, self.game.SelectX, self.game.SelectY, 0))) - VBMath.Rnd() * 5f));
              if (!self.game.Data.RegimeObj[regnr].AI)
                num6 += 4;
            }
            if (num6 > num4)
            {
              num4 = num6;
              onregnr1 = regnr;
            }
          }
        }
      }
      if (onregnr1 <= -1 ||  self.game.Data.RuleVar[903] <= 100.0 *  VBMath.Rnd())
        return;
      self.game.ProcessingObj.DeclareWar(self.game.Data.Turn, onregnr1);
      let mut regimeCounter4: i32 =  self.game.Data.RegimeCounter;
      for (let mut onregnr2: i32 =  0; onregnr2 <= regimeCounter4; onregnr2 += 1)
      {
        if (onregnr2 != self.game.Data.Turn & onregnr2 != onregnr1 && self.game.Data.RegimeObj[onregnr2].RegimeRel[onregnr1] == 2)
          self.game.ProcessingObj.DeclareWar(self.game.Data.Turn, onregnr2);
      }
    }

    pub fn InitFindOA()
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight1: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight1; index2 += 1)
        {
          if (self.HexOA[index1, index2] == 0 & self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index1, index2].Regime, self.game.Data.Turn))
          {
            this += 1.OACount;
            self.HexOA[index1, index2] = self.OACount;
            let mut num: i32 =  1;
            while (num > 0)
            {
              num = 0;
              let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
              {
                let mut mapHeight2: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut cy: i32 =  0; cy <= mapHeight2; cy += 1)
                {
                  if (self.HexOA[cx, cy] == 0 & self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[cx, cy].Regime, self.game.Data.Turn))
                  {
                    let mut tfacing: i32 =  1;
                    do
                    {
                      Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                      if (coordinate.onmap && self.HexOA[coordinate.x, coordinate.y] > 0)
                      {
                        self.HexOA[cx, cy] = self.HexOA[coordinate.x, coordinate.y];
                        num += 1;
                      }
                      tfacing += 1;
                    }
                    while (tfacing <= 6);
                  }
                }
              }
            }
          }
        }
      }
      self.AddLog("Operational Areas found for this regime: " + Conversion.Str( self.OACount));
    }

    pub fn InitFindContinent()
    {
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.HexContinent.GetUpperBound(0) < self.game.Data.MapObj[0].MapWidth)
        self.HexContinent = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (self.HexContinent.GetUpperBound(1) < self.game.Data.MapObj[0].MapHeight)
        self.HexContinent = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight1: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight1; index2 += 1)
        {
          if (self.HexContinent[index1, index2] == 0 & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea)
          {
            this += 1.ContinentCount;
            self.HexContinent[index1, index2] = self.ContinentCount;
            let mut num: i32 =  1;
            while (num > 0)
            {
              num = 0;
              let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
              {
                let mut mapHeight2: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut cy: i32 =  0; cy <= mapHeight2; cy += 1)
                {
                  if (self.HexContinent[cx, cy] == 0 & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
                  {
                    let mut tfacing: i32 =  1;
                    do
                    {
                      Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                      if (coordinate.onmap && self.HexContinent[coordinate.x, coordinate.y] > 0)
                      {
                        self.HexContinent[cx, cy] = self.HexContinent[coordinate.x, coordinate.y];
                        num += 1;
                      }
                      tfacing += 1;
                    }
                    while (tfacing <= 6);
                  }
                }
              }
            }
          }
        }
      }
      self.AddLog("Continents found on this map: " + Conversion.Str( self.ContinentCount));
    }

    pub fn InitGetSeaSA()
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut num: i32 =  1;
      while (num > 0)
      {
        num = 0;
        let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
            numArray[index1, index2] = 0;
        }
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (self.HexSeaSA[cx, cy] == 0 & self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
            {
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] > 0 & self.game.HandyFunctionsObj.IsHexPort(coordinate.x, coordinate.y, 0) | self.HexSeaSA[coordinate.x, coordinate.y] > 0)
                {
                  if (self.HexSA[coordinate.x, coordinate.y] > 0)
                  {
                    numArray[cx, cy] = self.HexSA[coordinate.x, coordinate.y];
                    if (self.HexSeaSA[coordinate.x, coordinate.y] == 0)
                      numArray[coordinate.x, coordinate.y] = self.HexSA[coordinate.x, coordinate.y];
                  }
                  else
                    numArray[cx, cy] = self.HexSeaSA[coordinate.x, coordinate.y];
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
        let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index3: i32 =  0; index3 <= mapWidth3; index3 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
          {
            if (numArray[index3, index4] > 0)
            {
              self.HexSeaSA[index3, index4] = numArray[index3, index4];
              num += 1;
            }
          }
        }
      }
    }

    pub JoinedNeighbour: bool(area1: i32, area2: i32)
    {
      let mut neighbourCount1: i32 =  self.SAObj[area1].NeighbourCount;
      for (let mut index1: i32 =  1; index1 <= neighbourCount1; index1 += 1)
      {
        let mut nr: i32 =  self.SAObj[area1].Neighbour[index1];
        let mut neighbourCount2: i32 =  self.SAObj[area2].NeighbourCount;
        for (let mut index2: i32 =  1; index2 <= neighbourCount2; index2 += 1)
        {
          if (self.SAObj[self.SAObj[area2].Neighbour[index2]].IsNeighbour(nr))
            return true;
        }
      }
      return false;
    }

    pub fn InitGetSA()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSA[index1, index2] == 0 & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea & self.AIVP[index1, index2] > 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].AIBlock < 1)
          {
            this += 1.SACount;
            self.HexSA[index1, index2] = self.SACount;
            self.HexSAWithoutTemp[index1, index2] = self.SACount;
            self.SAObj = (SAClass[]) Utils.CopyArray((Array) self.SAObj, (Array) new SAClass[self.SACount + 1]);
            self.SAObj[self.SACount] = SAClass::new();
            self.SAObj[self.SACount].X = index1;
            self.SAObj[self.SACount].Y = index2;
            self.SAObj[self.SACount].Size = 1;
            self.SAObj[self.SACount].aivp = self.AIVP[index1, index2];
          }
        }
      }
      let mut num1: i32 =  1;
      Coordinate coordinate;
      while (num1 > 0)
      {
        num1 = 0;
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index3: i32 =  0; index3 <= mapWidth2; index3 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
            numArray1[index3, index4] = 0;
        }
        let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth3; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (self.HexSAWithoutTemp[cx, cy] > 0)
            {
              let mut tfacing: i32 =  1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea & self.HexSAWithoutTemp[coordinate.x, coordinate.y] == 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1)
                  numArray1[coordinate.x, coordinate.y] = self.HexSAWithoutTemp[cx, cy];
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
        let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index5: i32 =  0; index5 <= mapWidth4; index5 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
          {
            if (numArray1[index5, index6] > 0)
            {
              self.HexSAWithoutTemp[index5, index6] = numArray1[index5, index6];
              num1 += 1;
            }
          }
        }
      }
      let mut num2: i32 =  1;
      while (num2 > 0)
      {
        num2 = 0;
        let mut mapWidth5: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index7: i32 =  0; index7 <= mapWidth5; index7 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
            numArray1[index7, index8] = 0;
        }
        let mut mapWidth6: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth6; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (self.HexSA[cx, cy] > 0)
            {
              let mut tfacing: i32 =  1;
              do
              {
                coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea & self.HexSA[coordinate.x, coordinate.y] == 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1 && self.game.Data.MapObj[0].HexObj[cx, cy].Regime == self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime)
                  numArray1[coordinate.x, coordinate.y] = self.HexSA[cx, cy];
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
        let mut mapWidth7: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index9: i32 =  0; index9 <= mapWidth7; index9 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index10: i32 =  0; index10 <= mapHeight; index10 += 1)
          {
            if (numArray1[index9, index10] > 0)
            {
              self.HexSA[index9, index10] = numArray1[index9, index10];
              num2 += 1;
              saObj: Vec<SAClass> = self.SAObj;
              saClassArray: Vec<SAClass> = saObj;
              numArray2: Vec<i32> = numArray1;
              numArray3: Vec<i32> = numArray2;
              let mut index11: i32 =  index9;
              let mut index12: i32 =  index11;
              let mut index13: i32 =  index10;
              let mut index14: i32 =  index13;
              let mut index15: i32 =  numArray3[index12, index14];
              saClassArray[index15].Size = saObj[numArray2[index11, index13]].Size + 1;
            }
          }
        }
      }
      let mut num3: i32 =  1;
      while (num3 == 1)
      {
        num3 = 0;
        let mut mapWidth8: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index16: i32 =  0; index16 <= mapWidth8; index16 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index17: i32 =  0; index17 <= mapHeight; index17 += 1)
          {
            if (self.HexSA[index16, index17] == 0 & !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index16, index17].LandscapeType].IsSea && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index16, index17].LandscapeType].AIBlock < 1 && num3 == 0)
            {
              this += 1.SACount;
              self.HexSA[index16, index17] = self.SACount;
              self.SAObj = (SAClass[]) Utils.CopyArray((Array) self.SAObj, (Array) new SAClass[self.SACount + 1]);
              self.SAObj[self.SACount] = SAClass::new();
              self.SAObj[self.SACount].X = index16;
              self.SAObj[self.SACount].Y = index17;
              self.SAObj[self.SACount].Size = 1;
              self.SAObj[self.SACount].aivp = 0;
              self.SAObj[self.SACount].AddConstitutant(self.HexSAWithoutTemp[index16, index17]);
              num3 = 1;
            }
          }
        }
        let mut num4: i32 =  1;
        while (num4 > 0)
        {
          num4 = 0;
          let mut mapWidth9: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index18: i32 =  0; index18 <= mapWidth9; index18 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index19: i32 =  0; index19 <= mapHeight; index19 += 1)
              numArray1[index18, index19] = 0;
          }
          let mut mapWidth10: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut cx: i32 =  0; cx <= mapWidth10; cx += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
            {
              if (self.HexSA[cx, cy] > 0)
              {
                let mut tfacing: i32 =  1;
                do
                {
                  coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                  if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea & self.HexSA[coordinate.x, coordinate.y] == 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1 && self.game.Data.MapObj[0].HexObj[cx, cy].Regime == self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime)
                    numArray1[coordinate.x, coordinate.y] = self.HexSA[cx, cy];
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
          let mut mapWidth11: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index20: i32 =  0; index20 <= mapWidth11; index20 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index21: i32 =  0; index21 <= mapHeight; index21 += 1)
            {
              if (numArray1[index20, index21] > 0)
              {
                self.HexSA[index20, index21] = numArray1[index20, index21];
                self.SAObj[self.HexSA[index20, index21]].AddConstitutant(self.HexSAWithoutTemp[index20, index21]);
                num4 += 1;
                saObj: Vec<SAClass> = self.SAObj;
                saClassArray: Vec<SAClass> = saObj;
                numArray4: Vec<i32> = numArray1;
                numArray5: Vec<i32> = numArray4;
                let mut index22: i32 =  index20;
                let mut index23: i32 =  index22;
                let mut index24: i32 =  index21;
                let mut index25: i32 =  index24;
                let mut index26: i32 =  numArray5[index23, index25];
                saClassArray[index26].Size = saObj[numArray4[index22, index24]].Size + 1;
              }
            }
          }
        }
      }
    }

    pub fn InitSARelations()
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      int[] numArray2 = new int[self.SACount + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      Coordinate coordinate;
      for (let mut cx: i32 =  0; cx <= mapWidth1; cx += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
        {
          if (self.HexSA[cx, cy] > 0)
          {
            let mut tfacing: i32 =  1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
              if (coordinate.onmap && self.HexSA[coordinate.x, coordinate.y] > 0 & self.HexSA[coordinate.x, coordinate.y] != self.HexSA[cx, cy] && !self.SAObj[self.HexSA[cx, cy]].IsNeighbour(self.HexSA[coordinate.x, coordinate.y]))
                self.SAObj[self.HexSA[cx, cy]].AddNeighbour(self.HexSA[coordinate.x, coordinate.y]);
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
        }
      }
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
        {
          if (self.HexSeaSA[cx, cy] > 0)
          {
            let mut tfacing: i32 =  1;
            do
            {
              coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
              if (coordinate.onmap && self.HexSeaSA[coordinate.x, coordinate.y] > 0 & self.HexSeaSA[coordinate.x, coordinate.y] != self.HexSeaSA[cx, cy] && !self.SAObj[self.HexSeaSA[cx, cy]].IsSeaNeighbour(self.HexSeaSA[coordinate.x, coordinate.y]))
                self.SAObj[self.HexSeaSA[cx, cy]].AddSeaNeighbour(self.HexSeaSA[coordinate.x, coordinate.y]);
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
        }
      }
      let mut saCount1: i32 =  self.SACount;
      for (let mut nr: i32 =  1; nr <= saCount1; nr += 1)
      {
        if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.SAObj[nr].X, self.SAObj[nr].Y].LandscapeType].IsSea)
        {
          let mut num1: i32 =  0;
          let mut saCount2: i32 =  self.SACount;
          for (let mut nr2: i32 =  1; nr2 <= saCount2; nr2 += 1)
          {
            if (nr != nr2 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.SAObj[nr].X, self.SAObj[nr].Y].LandscapeType].IsSea)
            {
              let mut num2: i32 =  self.AreaDistance2(nr, nr2, MaxDistance: 3);
              if (num2 > 0)
              {
                let mut num3: i32 =  (int) Math.Round(Conversion.Int( self.SAObj[nr2].aivp /  num2));
                if (num3 > num1)
                  num1 = num3;
              }
            }
          }
          self.SAObj[nr].fuzzyvp = self.SAObj[nr].aivp + num1;
          if (self.SAObj[nr].aivp == 0 & self.game.Data.MapObj[0].HexObj[self.SAObj[nr].X, self.SAObj[nr].Y].Regime == -1)
            self.SAObj[nr].fuzzyvp = 0;
        }
      }
      self.AddLog("SubAreas found on whole map: " + Conversion.Str( self.SACount));
      let mut saCount3: i32 =  self.SACount;
      for (let mut Number1: i32 =  1; Number1 <= saCount3; Number1 += 1)
      {
        self.AddLog(" ");
        self.AddLog("SubArea #" + Conversion.Str( Number1) + ": ");
        self.AddLog(self.game.HandyFunctionsObj.GetHexName(self.SAObj[Number1].X, self.SAObj[Number1].Y, 0) + "(" + Conversion.Str( self.SAObj[Number1].X) + "," + Conversion.Str( self.SAObj[Number1].Y) + "), size: " + Conversion.Str( self.SAObj[Number1].Size) + ", aivp/fuzzyvp: " + Conversion.Str( self.SAObj[Number1].aivp) + "/" + Conversion.Str( self.SAObj[Number1].fuzzyvp) + ", Bordering SA's: " + Conversion.Str( self.SAObj[Number1].NeighbourCount));
        s1: String = "Neighbours: ";
        let mut neighbourCount: i32 =  self.SAObj[Number1].NeighbourCount;
        for (let mut index: i32 =  1; index <= neighbourCount; index += 1)
        {
          let mut Number2: i32 =  self.SAObj[Number1].Neighbour[index];
          s1 = s1 + self.game.HandyFunctionsObj.GetHexName(self.SAObj[Number2].X, self.SAObj[Number2].Y, 0) + "(#" + Conversion.Str( Number2) + ")";
          if (index < self.SAObj[Number1].NeighbourCount)
            s1 += ", ";
        }
        self.AddLog(s1);
        s2: String = "SeaNeighbours: ";
        let mut seaNeighbourCount: i32 =  self.SAObj[Number1].SeaNeighbourCount;
        for (let mut index: i32 =  1; index <= seaNeighbourCount; index += 1)
        {
          let mut Number3: i32 =  self.SAObj[Number1].SeaNeighbour[index];
          s2 = s2 + self.game.HandyFunctionsObj.GetHexName(self.SAObj[Number3].X, self.SAObj[Number3].Y, 0) + "(#" + Conversion.Str( Number3) + ")";
          if (index < self.SAObj[Number1].SeaNeighbourCount)
            s2 += ", ";
        }
        self.AddLog(s2);
        if (self.SAObj[Number1].ConstitutantCount > 0)
        {
          s3: String = "Is Temporary Area. Constitutants:";
          let mut constitutantCount: i32 =  self.SAObj[Number1].ConstitutantCount;
          for (let mut index: i32 =  1; index <= constitutantCount; index += 1)
          {
            let mut Number4: i32 =  self.SAObj[Number1].Constitutant[index];
            if (Number4 > 0)
              s3 = s3 + self.game.HandyFunctionsObj.GetHexName(self.SAObj[Number4].X, self.SAObj[Number4].Y, 0) + "(#" + Conversion.Str( Number4) + ")";
            else
              s3 += "No VP Area";
            if (index < self.SAObj[Number1].ConstitutantCount)
              s3 += ", ";
          }
          self.AddLog(s3);
        }
      }
    }

    pub fn InitAIVP()
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].VP > 0 && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea)
            self.game.Data.MapObj[0].HexObj[index1, index2].VP = 0;
          self.AIVP[index1, index2] = 0;
          self.AIVP[index1, index2] = self.game.Data.MapObj[0].HexObj[index1, index2].VP;
          if (self.game.Data.MapObj[0].HexObj[index1, index2].Location > -1)
          {
            let mut type: i32 =  self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[index1, index2].Location].Type;
            if (self.game.Data.LocTypeObj[type].OnDestructLT == -1)
            {
              let mut num1: i32 =  (int) Math.Round( Conversion.Int( self.game.Data.LocTypeObj[type].MaxProd / self.game.Data.RuleVar[201]));
              if (num1 == 0)
                num1 = 1;
              if (self.game.Data.LocTypeObj[type].MaxProd < 1)
                num1 = 0;
              aivp: Vec<i32> = self.AIVP;
              numArray: Vec<i32> = aivp;
              let mut index3: i32 =  index1;
              let mut index4: i32 =  index3;
              let mut index5: i32 =  index2;
              let mut index6: i32 =  index5;
              let mut num2: i32 =  aivp[index3, index5] + num1;
              numArray[index4, index6] = num2;
            }
          }
          aivp1: Vec<i32> = self.AIVP;
          numArray1: Vec<i32> = aivp1;
          let mut index7: i32 =  index1;
          let mut index8: i32 =  index7;
          let mut index9: i32 =  index2;
          let mut index10: i32 =  index9;
          let mut num: i32 =  aivp1[index7, index9] + self.game.Data.RegimeObj[self.game.Data.Turn].AIVP[0].Value[index1, index2];
          numArray1[index8, index10] = num;
          let mut regime: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].Regime;
          if (regime > -1 & self.AIVP[index1, index2] > 0 && self.game.Data.Turn != regime && self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regime] == 0 &&  self.game.Data.RuleVar[264] == 0.0 && self.game.Data.RegimeObj[regime].AI)
          {
            self.AIVP[index1, index2] = (int) Math.Round( self.AIVP[index1, index2] * 0.5);
            if (1 > self.AIVP[index1, index2])
              self.AIVP[index1, index2] = 1;
            if ((self.game.Data.GameID + regime + self.game.Data.Turn) % 2 == 0)
            {
              self.AIVP[index1, index2] = (int) Math.Round( self.AIVP[index1, index2] * 0.5);
              if (1 > self.AIVP[index1, index2])
                self.AIVP[index1, index2] = 1;
            }
            else if ((self.game.Data.GameID + regime + self.game.Data.Turn) % 3 == 0)
            {
              self.AIVP[index1, index2] = (int) Math.Round( self.AIVP[index1, index2] * 0.1);
              if (1 > self.AIVP[index1, index2])
                self.AIVP[index1, index2] = 1;
            }
          }
        }
      }
    }

    pub float GetEntrenchMod(unr: i32)
    {
      float entrenchMod = 0.0f;
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut entrenchPower: i32 =  self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].EntrenchPower;
        float num = entrenchPower <= 0 ? 0.0f :  self.game.Data.SFObj[sf].CurrentEntrench /  entrenchPower;
        if ( num >  entrenchMod)
          entrenchMod = num;
      }
      if ( entrenchMod < 1.0)
        entrenchMod = 1f;
      return entrenchMod;
    }

    pub float GetEntrenchMod(x: i32, y: i32)
    {
      if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter < 1)
        return 1f;
      float num = 0.0f;
      let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitList[index];
        num += self.GetEntrenchMod(unit);
      }
      float entrenchMod = num /  (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter + 1);
      if ( entrenchMod < 1.0)
        entrenchMod = 1f;
      return entrenchMod;
    }

    pub fn PlanEngineerNeedScore(plnr: i32) -> i32
    {
      num1: i32;
      if (self.TPlanObj[plnr].Type == 40)
        num1 = self.TPlanObj[plnr].CurrentAPCost - self.TPlanObj[plnr].PossibleAPCost;
      else if (self.TPlanObj[plnr].Type == 20 & (self.TPlanObj[plnr].Stand == 1 | self.TPlanObj[plnr].EnemyUnitCount == 0))
      {
        let mut num2: i32 =  self.TPlanObj[plnr].CurrentAPCost - self.TPlanObj[plnr].PossibleAPCost;
        float num3 =  self.TPlanObj[plnr].AverageUnitAPCost / self.game.Data.RuleVar[51];
        if ( num3 < 1.0)
          num3 = 1f;
        num1 = (int) Math.Round( Conversion.Int( num2 * num3));
        if (self.TPlanObj[plnr].FromArea.ConstitutantCount > 0)
          num1 = 0;
        if (self.TPlanObj[plnr].TooArea.ConstitutantCount > 0)
          num1 = (int) Math.Round( num1 / 2.0);
      }
      else
        num1 = self.TPlanObj[plnr].Type != 50 ? 0 : 9999;
      return num1;
    }

    pub float GetPercentCuttenOff(plannr: i32)
    {
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].AIPlanNr == plannr & self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn)
        {
          num1 += self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr);
          if (self.game.Data.UnitObj[unr].AICutoff > 0)
            num2 += self.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr);
        }
      }
      return num1 == 0 ? 0.0f :  num2 /  num1;
    }

    pub Coordinate SetMatrixHQ(SimpleList UL, let mut hqnr: i32 =  -1, let mut onlysanr: i32 =  -1)
    {
      self.Matrix2 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      Coordinate coordinate = Coordinate::new();
      bool flag = true;
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          self.Matrix2[index1, index2] = 0;
      }
      aiPlanNr: i32;
      if (hqnr > -1)
        aiPlanNr = self.game.Data.UnitObj[hqnr].AIPlanNr;
      if (aiPlanNr > 0 && self.TPlanObj[aiPlanNr].Type == 30)
      {
        flag = false;
        self.Matrix2[self.TPlanObj[aiPlanNr].FromArea.X, self.TPlanObj[aiPlanNr].FromArea.Y] = (int) Math.Round( self.game.Data.RuleVar[152]);
      }
      if (flag && UL.Counter > -1)
      {
        let mut counter: i32 =  UL.Counter;
        for (let mut index3: i32 =  0; index3 <= counter; index3 += 1)
        {
          num1: i32;
          if (aiPlanNr > 0)
          {
            num1 = Information.IsNothing( self.TPlanObj[aiPlanNr].TooArea) ? self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[UL.Id[index3]].X, self.game.Data.UnitObj[UL.Id[index3]].Y, 0, self.TPlanObj[aiPlanNr].FromArea.X, self.TPlanObj[aiPlanNr].FromArea.Y, 0) : self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[UL.Id[index3]].X, self.game.Data.UnitObj[UL.Id[index3]].Y, 0, self.TPlanObj[aiPlanNr].TooArea.X, self.TPlanObj[aiPlanNr].TooArea.Y, 0);
            if ( num1 <=  self.game.Data.RuleVar[191])
              num1 = 1;
          }
          self.SetMatrix1(self.game.Data.UnitObj[UL.Id[index3]].X, self.game.Data.UnitObj[UL.Id[index3]].Y, onlythroughfriendlyhex: true);
          let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index4: i32 =  0; index4 <= mapWidth2; index4 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
            {
              num2: i32;
              num2 += 10 * self.game.HandyFunctionsObj.GetPowerPtsAbsolute(UL.Id[index3], true);
              let mut num3: i32 =  (int) Math.Round( ( self.Matrix1[index4, index5] * ( (10 * self.game.HandyFunctionsObj.GetPowerPtsAbsolute(UL.Id[index3], true)) / self.game.Data.RuleVar[152])));
              if (aiPlanNr > 0)
                num3 = (int) Math.Round( num3 /  num1);
              matrix2: Vec<i32> = self.Matrix2;
              numArray: Vec<i32> = matrix2;
              let mut index6: i32 =  index4;
              let mut index7: i32 =  index6;
              let mut index8: i32 =  index5;
              let mut index9: i32 =  index8;
              let mut num4: i32 =  matrix2[index6, index8] + num3;
              numArray[index7, index9] = num4;
            }
          }
        }
      }
      if (hqnr > -1)
      {
        let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut x1: i32 =  0; x1 <= mapWidth3; x1 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[x1, y1].Regime == -1 && self.AIVP[x1, y1] > 0 & self.HexSA[x1, y1] > 0 && self.IsAreaNeighbour(self.HexSA[x1, y1], self.HexSA[self.game.Data.UnitObj[hqnr].X, self.game.Data.UnitObj[hqnr].Y]))
            {
              let mut num5: i32 =  (int) Math.Round( ( self.AIVP[x1, y1] * self.game.Data.RuleVar[152]));
              let mut num6: i32 =  (int) Math.Round( self.game.HandyFunctionsObj.Distance(x1, y1, 0, self.game.Data.UnitObj[hqnr].X, self.game.Data.UnitObj[hqnr].Y, 0) / 2.0);
              if (num6 < 1)
                num6 = 1;
              let mut num7: i32 =  (int) Math.Round( num5 /  num6);
              self.Matrix2[x1, y1] = num7;
            }
          }
        }
      }
      self.SetMatrixEnemyFront(self.game.Data.Turn);
      let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index10: i32 =  0; index10 <= mapWidth4; index10 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index11: i32 =  0; index11 <= mapHeight; index11 += 1)
        {
          if (hqnr > -1)
          {
            if (self.game.Data.UnitObj[hqnr].AIPlanNr > -1)
            {
              if (self.TPlanObj[self.game.Data.UnitObj[hqnr].AIPlanNr].Type == 20)
              {
                if (self.TPlanObj[self.game.Data.UnitObj[hqnr].AIPlanNr].Stand == 1)
                {
                  if ( self.Matrix1[index10, index11] >= 0.9 *  self.game.Data.RuleVar[152])
                    self.Matrix2[index10, index11] = (int) Math.Round( self.Matrix2[index10, index11] * 0.5);
                }
                else if (self.TPlanObj[self.game.Data.UnitObj[hqnr].AIPlanNr].Stand == 2)
                {
                  if ( self.Matrix1[index10, index11] >= 0.8 *  self.game.Data.RuleVar[152])
                    self.Matrix2[index10, index11] = (int) Math.Round( self.Matrix2[index10, index11] * 0.4);
                }
                else if ( self.Matrix1[index10, index11] >= 0.7 *  self.game.Data.RuleVar[152])
                  self.Matrix2[index10, index11] = (int) Math.Round( self.Matrix2[index10, index11] * 0.3);
              }
              else if (self.TPlanObj[self.game.Data.UnitObj[hqnr].AIPlanNr].Type == 30)
              {
                if ( self.Matrix1[index10, index11] >= 0.7 *  self.game.Data.RuleVar[152])
                  self.Matrix2[index10, index11] = (int) Math.Round( self.Matrix2[index10, index11] * 0.3);
              }
              else if ( self.Matrix1[index10, index11] >= 0.9 *  self.game.Data.RuleVar[152])
                self.Matrix2[index10, index11] = (int) Math.Round( self.Matrix2[index10, index11] * 0.5);
            }
            else if ( self.Matrix1[index10, index11] >= 0.9 *  self.game.Data.RuleVar[152])
              self.Matrix2[index10, index11] = (int) Math.Round( self.Matrix2[index10, index11] * 0.5);
          }
        }
      }
      let mut mapWidth5: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index12: i32 =  0; index12 <= mapWidth5; index12 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index13: i32 =  0; index13 <= mapHeight; index13 += 1)
        {
          if (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index12, index13].Regime, self.game.Data.Turn) & self.game.Data.MapObj[0].HexObj[index12, index13].Regime != -1)
            self.Matrix2[index12, index13] = 0;
          if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index12, index13].LandscapeType].AIBlock == -1)
            self.Matrix2[index12, index13] = 0;
        }
      }
      if (onlysanr > 0)
      {
        let mut mapWidth6: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index14: i32 =  0; index14 <= mapWidth6; index14 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index15: i32 =  0; index15 <= mapHeight; index15 += 1)
          {
            if (self.HexSA[index14, index15] > 0 && self.HexSA[index14, index15] != onlysanr)
              self.Matrix2[index14, index15] = 0;
          }
        }
      }
      let mut num8: i32 =  0;
      let mut num9: i32 =  -1;
      let mut num10: i32 =  -1;
      let mut mapWidth7: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index16: i32 =  0; index16 <= mapWidth7; index16 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index17: i32 =  0; index17 <= mapHeight; index17 += 1)
        {
          if (self.Matrix2[index16, index17] > num8)
          {
            num8 = self.Matrix2[index16, index17];
            num9 = index16;
            num10 = index17;
          }
        }
      }
      if (num9 > -1)
      {
        coordinate.onmap = true;
        coordinate.x = num9;
        coordinate.y = num10;
        return coordinate;
      }
      coordinate.onmap = false;
      return coordinate;
    }

    pub NeedHQ: bool(nr: i32) => self.TPlanObj[nr].FriendlyUnitCount > 2;

    pub fn WhichCurrentAreaIsThis(ref SAClass Area) -> i32
    {
      if (Information.IsNothing( Area))
        return 0;
      let mut saCount1: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount1; index += 1)
      {
        if (self.SAObj[index].X == Area.X && self.SAObj[index].Y == Area.Y)
          return index;
      }
      num1: i32;
      if (Area.ConstitutantCount > 0)
      {
        let mut saCount2: i32 =  self.SACount;
        for (let mut index1: i32 =  1; index1 <= saCount2; index1 += 1)
        {
          let mut num2: i32 =  0;
          if (self.SAObj[index1].ConstitutantCount > 0)
          {
            let mut constitutantCount1: i32 =  self.SAObj[index1].ConstitutantCount;
            for (let mut index2: i32 =  1; index2 <= constitutantCount1; index2 += 1)
            {
              let mut constitutantCount2: i32 =  Area.ConstitutantCount;
              for (let mut index3: i32 =  1; index3 <= constitutantCount2; index3 += 1)
              {
                if (self.SAObj[index1].Constitutant[index2] == Area.Constitutant[index3])
                  num2 += 1;
              }
            }
          }
          num3: i32;
          if (num2 > num3)
          {
            num3 = num2;
            num1 = index1;
          }
        }
      }
      if (num1 > 0)
        return num1;
      return self.HexSA[Area.X, Area.Y] > 0 ? self.HexSA[Area.X, Area.Y] : 0;
    }

    pub Coordinate ClosestFriendlyHex(tx: i32, ty: i32, ref SimpleList SL)
    {
      Coordinate coordinate1 = Coordinate::new();
      let mut tfacing: i32 =  1;
      do
      {
        Coordinate coordinate2 = self.game.HandyFunctionsObj.HexNeighbour(tx, ty, 0, tfacing);
        if (coordinate2.onmap && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[tx, ty].Regime, self.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].Regime) && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea && SL.FindNr(-1, coordinate2.x, coordinate2.y) == -1)
          return coordinate2;
        tfacing += 1;
      }
      while (tfacing <= 6);
      coordinate1.x = -1;
      coordinate1.onmap = false;
      return coordinate1;
    }

    pub Coordinate ClosestUnFriendlyHex(tx: i32, ty: i32, ref SimpleList SL)
    {
      Coordinate coordinate1 = Coordinate::new();
      let mut tfacing: i32 =  1;
      do
      {
        Coordinate coordinate2 = self.game.HandyFunctionsObj.HexNeighbour(tx, ty, 0, tfacing);
        if (coordinate2.onmap && !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[tx, ty].Regime, self.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].Regime) && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea && SL.FindNr(-1, coordinate2.x, coordinate2.y) == -1)
          return coordinate2;
        tfacing += 1;
      }
      while (tfacing <= 6);
      coordinate1.x = -1;
      coordinate1.onmap = false;
      return coordinate1;
    }

    pub object BestMatrix2(tx: i32, ty: i32, dist: i32)
    {
      Coordinate coordinate1 = Coordinate::new();
      let mut num1: i32 =  tx - (dist + 1);
      let mut num2: i32 =  tx + (dist + 1);
      num3: i32;
      num4: i32;
      num5: i32;
      for (let mut index: i32 =  num1; index <= num2; index += 1)
      {
        let mut x2: i32 =  index;
        if (self.game.Data.MapObj[0].MapLoop & x2 < 0)
          x2 = self.game.Data.MapObj[0].MapWidth + x2 + 1;
        if (self.game.Data.MapObj[0].MapLoop & x2 > self.game.Data.MapObj[0].MapWidth)
          x2 = x2 - self.game.Data.MapObj[0].MapWidth - 1;
        let mut num6: i32 =  ty - (dist + 1);
        let mut num7: i32 =  ty + (dist + 1);
        for (let mut y2: i32 =  num6; y2 <= num7; y2 += 1)
        {
          Coordinate coordinate2;
          if (x2 > -1 & y2 > -1 && x2 <= self.game.Data.MapObj[0].MapWidth & y2 <= self.game.Data.MapObj[0].MapHeight && self.game.HandyFunctionsObj.Distance(tx, ty, 0, x2, y2, 0) <= dist && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea && !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[x2, y2].Regime, self.game.Data.MapObj[0].HexObj[tx, ty].Regime) && self.Matrix2[x2, y2] > num3)
          {
            num3 = self.Matrix2[x2, y2];
            num4 = x2;
            num5 = y2;
          }
        }
      }
      if (num3 > 0)
      {
        coordinate1.x = num4;
        coordinate1.y = num5;
        coordinate1.onmap = true;
        return  coordinate1;
      }
      coordinate1.x = -1;
      coordinate1.onmap = false;
      return  coordinate1;
    }

    pub void SetMatrix1(
      x: i32,
      y: i32,
      bool subtractformovedunit = false,
      let mut unitnr: i32 =  -1,
      bool onlyinplanarea = false,
      bool onlythroughfriendlyhex = false,
      let mut hq: i32 =  -1,
      let mut MaxDist: i32 =  9999)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.Matrix1 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          self.Matrix1[index1, index2] = 0;
      }
      if (hq > -1)
        self.game.HandyFunctionsObj.MakeSupplyLayer(hq, true);
      self.Matrix1[x, y] = (int) Math.Round( self.game.Data.RuleVar[152]);
      numArray1[x, y] = 1;
      let mut num1: i32 =  0;
      let mut num2: i32 =  1;
      unit: i32;
      while (num2 == 1)
      {
        num2 = 0;
        num1 += 1;
        if (num1 < MaxDist)
        {
          let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
            {
              if (numArray1[cx, cy] == num1)
              {
                let mut tfacing: i32 =  1;
                do
                {
                  Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                  if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea && numArray1[coordinate.x, coordinate.y] == 0)
                  {
                    numArray1[coordinate.x, coordinate.y] = num1 + 1;
                    num2 = 1;
                    self.Matrix1[coordinate.x, coordinate.y] =  self.Matrix1[cx, cy] <=  self.game.Data.RuleVar[152] * 0.25 ? self.Matrix1[cx, cy] - 1 : (int) Math.Round(Conversion.Int( self.Matrix1[cx, cy] * 0.95));
                    if (subtractformovedunit)
                    {
                      let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter;
                      for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                      {
                        unit = self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[index3];
                        if (unit != unitnr && self.UnitMovePhase[unit] == 1)
                        {
                          let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unit].AIPlanNr;
                          if (aiPlanNr > 0)
                          {
                            num2 = (int) Math.Round( self.TPlanObj[aiPlanNr].WeightFriendlyForce);
                            if (num2 == 0)
                              num2 = 1;
                            let mut num3: i32 =  (int) Math.Round(Conversion.Int( self.GetForceLandStrength(unit) /  num2 * ( self.game.Data.RuleVar[152] * 0.1)));
                            matrix1: Vec<i32> = self.Matrix1;
                            numArray2: Vec<i32> = matrix1;
                            let mut x1: i32 =  coordinate.x;
                            let mut index4: i32 =  x1;
                            let mut y1: i32 =  coordinate.y;
                            let mut index5: i32 =  y1;
                            let mut num4: i32 =  matrix1[x1, y1] - num3;
                            numArray2[index4, index5] = num4;
                            if (unitnr > -1 && self.game.Data.UnitObj[unitnr].HQ > -1)
                            {
                              num2 = self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[unitnr].X, self.game.Data.UnitObj[unitnr].Y, 0, self.game.Data.UnitObj[self.game.Data.UnitObj[unitnr].HQ].X, self.game.Data.UnitObj[self.game.Data.UnitObj[unitnr].HQ].Y, 0);
                              if ( num2 >  self.game.Data.RuleVar[191])
                                num2 = (int) Math.Round( (self.game.Data.RuleVar[191] -  num2));
                              if (num2 == 0)
                                num2 = 1;
                              self.Matrix1[coordinate.x, coordinate.y] = (int) Math.Round( self.Matrix1[coordinate.x, coordinate.y] * (1.0 /  num2));
                            }
                            if (0 > self.Matrix1[coordinate.x, coordinate.y])
                              self.Matrix1[coordinate.x, coordinate.y] = 0;
                          }
                        }
                      }
                    }
                    if (onlyinplanarea & unitnr > -1)
                    {
                      let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unitnr].AIPlanNr;
                      if (aiPlanNr > 0 && self.TPlanObj[aiPlanNr].Type != 30 & self.TPlanObj[aiPlanNr].Type != 40 && self.HexSA[coordinate.x, coordinate.y] != self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea) && self.HexSA[coordinate.x, coordinate.y] != self.GetAreaNr(self.TPlanObj[aiPlanNr].TooArea))
                        self.Matrix1[coordinate.x, coordinate.y] = (int) Math.Round( self.Matrix1[coordinate.x, coordinate.y] / 2.0);
                    }
                    if (onlythroughfriendlyhex && !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime, self.game.Data.MapObj[0].HexObj[x, y].Regime))
                      self.Matrix1[coordinate.x, coordinate.y] = 0;
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
        }
      }
      if (hq <= -1)
        return;
      float num5 = self.game.HandyFunctionsObj.UnitSupplyUse(unit) >= 1 ?  self.game.Data.UnitObj[unit].Supply /  self.game.HandyFunctionsObj.UnitSupplyUse(unit) : 1f;
      if (1.0 >  num5)
        num5 = 1f;
      if ( num5 > 9.0)
        num5 = 9f;
      let mut num6: i32 =  (int) Math.Round( num5 *  num5 * 3.0);
      if (1 > num6)
        num6 = 1;
      if (num6 > 50)
        num6 = 50;
      let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index6: i32 =  0; index6 <= mapWidth3; index6 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
        {
          let mut num7: i32 =  self.Matrix1[index6, index7];
          let mut num8: i32 =  (int) Math.Round( ( num7 -  num7 * ( self.game.EditObj.TempSup[0].Value[index6, index7] / ( num6 * self.game.Data.RuleVar[3]))));
          self.Matrix1[index6, index7] = num8;
          if (0 > self.Matrix1[index6, index7])
            self.Matrix1[index6, index7] = 0;
        }
      }
    }

    pub fn SetNavalMatrix1(x: i32, y: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.Matrix1 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          self.Matrix1[index1, index2] = 0;
      }
      if (x == -1)
        return;
      self.Matrix1[x, y] = (int) Math.Round( self.game.Data.RuleVar[152]);
      numArray[x, y] = 1;
      let mut num1: i32 =  0;
      let mut num2: i32 =  1;
      while (num2 == 1)
      {
        num2 = 0;
        num1 += 1;
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (numArray[cx, cy] == num1)
            {
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea | self.game.HandyFunctionsObj.IsHexPort(coordinate.x, coordinate.y, 0))
                {
                  let mut num3: i32 =  1;
                  if (self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter > -1 && self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[0]].Regime != self.game.Data.Turn)
                    num3 = 0;
                  if (numArray[coordinate.x, coordinate.y] == 0 & num3 == 1)
                  {
                    numArray[coordinate.x, coordinate.y] = num1 + 1;
                    num2 = 1;
                    self.Matrix1[coordinate.x, coordinate.y] =  self.Matrix1[cx, cy] <=  self.game.Data.RuleVar[152] * 0.25 ? self.Matrix1[cx, cy] - 1 : (int) Math.Round(Conversion.Int( self.Matrix1[cx, cy] * 0.95));
                  }
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
    }

    pub Coordinate GetAirSupportCoord(unr: i32, plannr: i32)
    {
      long[,] numArray1 = new long[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      long[,] numArray2 = new long[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      long[,] numArray3 = new long[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      long[,] numArray4 = new long[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.game.HandyFunctionsObj.MakeMovePrediction(unr, self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0, ismove: true);
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 =  0; x <= mapWidth1; x += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 =  0; y <= mapHeight; y += 1)
        {
          if (self.HexBackPlan[x, y] == plannr)
          {
            if (self.game.HandyFunctionsObj.GetLowestAp(unr) >= self.game.EditObj.TempValue[0].Value[x, y])
              numArray3[x, y] = 5000L;
          }
          else if (self.HexSA[x, y] > 0 && self.SAObj[self.HexSA[x, y]].IsNeighbour(self.GetAreaNr(self.TPlanObj[plannr].FromArea)) && self.game.HandyFunctionsObj.GetLowestAp(unr) >= self.game.EditObj.TempValue[0].Value[x, y])
            numArray3[x, y] = 5000L;
          if (numArray3[x, y] > 0L)
          {
            if (self.game.HandyFunctionsObj.IsHexAirfield(x, y, 0))
            {
              float fieldStackModifier = self.game.HandyFunctionsObj.GetAirFieldStackModifier(x, y);
              if ( fieldStackModifier < 0.66)
                numArray3[x, y] = 0L;
              else if ( fieldStackModifier < 0.8)
                numArray3[x, y] = (long) Math.Round( numArray3[x, y] / 10.0);
              else if ( fieldStackModifier < 1.0)
                numArray3[x, y] = (long) Math.Round( numArray3[x, y] / 3.0);
            }
            let mut closestFrontline: i32 =  self.GetClosestFrontline(x, y);
            if (closestFrontline > 0)
            {
              if (self.TPlanObj[closestFrontline].Stand == 2 | self.TPlanObj[closestFrontline].Stand == 3)
              {
                let mut closestEnemyDistance: i32 =  self.GetClosestEnemyDistance(x, y, true);
                if (closestEnemyDistance > 0 & closestEnemyDistance < 4)
                  numArray3[x, y] = (long) Math.Round( numArray3[x, y] / 2.0);
                if (closestEnemyDistance > 0 & closestEnemyDistance < 3)
                  numArray3[x, y] = (long) Math.Round( numArray3[x, y] / 3.0);
              }
              if (self.TPlanObj[closestFrontline].Stand == 1)
              {
                let mut closestEnemyDistance: i32 =  self.GetClosestEnemyDistance(x, y, true);
                if (closestEnemyDistance > 0 & closestEnemyDistance < 4)
                  numArray3[x, y] = (long) Math.Round( numArray3[x, y] / 2.0);
                if (closestEnemyDistance > 0 & closestEnemyDistance < 3)
                  numArray3[x, y] = (long) Math.Round( numArray3[x, y] / 2.0);
              }
            }
          }
        }
      }
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth2; index1 += 1)
      {
        let mut mapHeight1: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight1; index2 += 1)
        {
          if (numArray3[index1, index2] > 0L)
          {
            let mut lowestAirAp: i32 =  self.game.HandyFunctionsObj.GetLowestAirAp(unr);
            let mut num1: i32 =  lowestAirAp >= 100 ? 0 : 100 - lowestAirAp;
            if (num1 > 0)
            {
              self.game.HandyFunctionsObj.MakeMovePrediction(unr, index1, index2, 0, OneHexFurther: true, ClearSea: true, attack: ((uint) num1 > 0U));
              let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
              for (let mut index3: i32 =  0; index3 <= mapWidth3; index3 += 1)
              {
                let mut mapHeight2: i32 =  self.game.Data.MapObj[0].MapHeight;
                for (let mut index4: i32 =  0; index4 <= mapHeight2; index4 += 1)
                {
                  if (self.game.Data.MapObj[0].HexObj[index3, index4].Regime != -1)
                  {
                    if (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index3, index4].Regime, self.game.Data.Turn) && self.game.EditObj.TempValue[0].Value[index3, index4] <= 100)
                    {
                      if (self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index3, index4].Regime))
                      {
                        if ( self.game.HandyFunctionsObj.Distance(index1, index2, 0, index3, index4, 0) >  self.game.Data.RuleVar[223] / 3.0 &&  self.game.HandyFunctionsObj.Distance(index1, index2, 0, index3, index4, 0) <=  self.game.Data.RuleVar[223] && -(-1 < self.AreaDistance(self.HexSA[index3, index4], self.GetAreaNr(self.TPlanObj[plannr].FromArea)) ? 1 : 0) <= 1)
                        {
                          long[,] numArray5 = numArray4;
                          long[,] numArray6 = numArray5;
                          let mut index5: i32 =  index1;
                          let mut index6: i32 =  index5;
                          let mut index7: i32 =  index2;
                          let mut index8: i32 =  index7;
                          long num2 = (long) Math.Round( numArray5[index5, index7] +  self.GetHexForceLandStrength(index3, index4, true) / 2.0);
                          numArray6[index6, index8] = num2;
                        }
                      }
                      else if (!self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index3, index4].Regime].DipBlock &&  self.game.HandyFunctionsObj.Distance(index1, index2, 0, index3, index4, 0) >  self.game.Data.RuleVar[223] / 3.0 &&  self.game.HandyFunctionsObj.Distance(index1, index2, 0, index3, index4, 0) <=  self.game.Data.RuleVar[223] && -(-1 < self.AreaDistance(self.HexSA[index3, index4], self.GetAreaNr(self.TPlanObj[plannr].FromArea)) ? 1 : 0) <= 1)
                      {
                        long[,] numArray7 = numArray4;
                        long[,] numArray8 = numArray7;
                        let mut index9: i32 =  index1;
                        let mut index10: i32 =  index9;
                        let mut index11: i32 =  index2;
                        let mut index12: i32 =  index11;
                        long num3 = (long) Math.Round( numArray7[index9, index11] +  self.GetHexForceLandStrength(index3, index4, true) / 2.0);
                        numArray8[index10, index12] = num3;
                      }
                    }
                    if (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index3, index4].Regime, self.game.Data.Turn) || self.game.EditObj.TempValue[0].Value[index3, index4] > 100 ||  self.game.HandyFunctionsObj.Distance(index1, index2, 0, index3, index4, 0) >  self.game.Data.RuleVar[223] / 2.0)
                      ;
                  }
                }
              }
              if (self.GetHexForceAirStrength(index1, index2, true) > 0)
              {
                let mut num4: i32 =  (int) Math.Round(Math.Sqrt(Math.Sqrt(Math.Sqrt( self.GetHexForceAirStrength(index1, index2, true)))));
                numArray4[index1, index2] = (long) Math.Round( numArray4[index1, index2] /  num4);
              }
              long[,] numArray9 = numArray3;
              long[,] numArray10 = numArray9;
              let mut index13: i32 =  index1;
              let mut index14: i32 =  index13;
              let mut index15: i32 =  index2;
              let mut index16: i32 =  index15;
              long num5 = (long) Math.Round( numArray9[index13, index15] + Math.Sqrt( numArray4[index1, index2]));
              numArray10[index14, index16] = num5;
            }
          }
        }
      }
      let mut num6: i32 =  -1;
      let mut num7: i32 =  0;
      let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
      num8: i32;
      for (let mut index17: i32 =  0; index17 <= mapWidth4; index17 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index18: i32 =  0; index18 <= mapHeight; index18 += 1)
        {
          if (numArray3[index17, index18] > (long) num7)
          {
            num6 = index17;
            num8 = index18;
            num7 = (int) numArray3[index17, index18];
          }
        }
      }
      Coordinate airSupportCoord;
      if (num7 > 0)
      {
        airSupportCoord.x = num6;
        airSupportCoord.y = num8;
        airSupportCoord.onmap = true;
      }
      else
      {
        airSupportCoord.x = -1;
        airSupportCoord.y = -1;
        airSupportCoord.onmap = false;
      }
      return airSupportCoord;
    }

    pub Coordinate GetArtilleryCoord(unr: i32, plannr: i32)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      object[,] objArray = new object[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.game.HandyFunctionsObj.MakeMovePrediction(unr, self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0);
      let mut maxArtRange: i32 =  self.game.HandyFunctionsObj.GetMaxArtRange(unr, 0);
      Coordinate artilleryCoord;
      artilleryCoord.onmap = false;
      if (maxArtRange == 0)
        return artilleryCoord;
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (!Information.IsNothing( self.TPlanObj[plannr].TooArea) && self.GetAreaNr(self.TPlanObj[plannr].TooArea) == self.HexSA[index1, index2] && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[index1, index2].Regime) && self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1)
          {
            objArray[index1, index2] =  1;
            let mut tfacing: i32 =  1;
            do
            {
              artilleryCoord = self.game.HandyFunctionsObj.HexNeighbour(index1, index2, 0, tfacing);
              if (artilleryCoord.onmap)
                objArray[artilleryCoord.x, artilleryCoord.y] =  1;
              tfacing += 1;
            }
            while (tfacing <= 6);
            numArray3[index1, index2] = self.GetHexForceLandStrength(index1, index2, true);
          }
        }
      }
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -99999;
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      num3: i32;
      for (let mut x1: i32 =  0; x1 <= mapWidth2; x1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
        {
          if (numArray3[x1, y1] > 0)
          {
            let mut num4: i32 =  x1 - 5;
            let mut num5: i32 =  x1 + 5;
            for (let mut index3: i32 =  num4; index3 <= num5; index3 += 1)
            {
              let mut index4: i32 =  index3;
              if (self.game.Data.MapObj[0].MapLoop & index4 < 0)
                index4 = self.game.Data.MapObj[0].MapWidth + index4 + 1;
              if (self.game.Data.MapObj[0].MapLoop & index4 > self.game.Data.MapObj[0].MapWidth)
                index4 = index4 - self.game.Data.MapObj[0].MapWidth - 1;
              let mut num6: i32 =  y1 - 5;
              let mut num7: i32 =  y1 + 5;
              for (let mut index5: i32 =  num6; index5 <= num7; index5 += 1)
              {
                if (index4 >= 0 & index5 >= 0 && index4 <= self.game.Data.MapObj[0].MapWidth & index5 < self.game.Data.MapObj[0].MapHeight && Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(objArray[index4, index5],  0, false),  self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[index4, index5].Regime, self.game.Data.Turn))) && self.game.HandyFunctionsObj.Distance(x1, y1, 0, index4, index5, 0) <= maxArtRange)
                {
                  let mut num8: i32 =  (int) Math.Round( numArray3[x1, y1] - Math.Pow( self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0, index4, index5, 0), 2.0)) + self.GetHexForceLandStrength(index4, index5, true) * 12;
                  if (num8 > num2)
                  {
                    num1 = index4;
                    num3 = index5;
                    num2 = num8;
                  }
                }
              }
            }
          }
        }
      }
      if (num2 > -99999)
      {
        artilleryCoord.x = num1;
        artilleryCoord.y = num3;
        artilleryCoord.onmap = true;
      }
      else
      {
        artilleryCoord.x = -1;
        artilleryCoord.y = -1;
        artilleryCoord.onmap = false;
      }
      return artilleryCoord;
    }

    pub fn GetNavalTarget(plnr: i32) -> i32
    {
      self.SetNavalMatrix1(self.TPlanObj[plnr].FromArea.X, self.TPlanObj[plnr].FromArea.Y);
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut areaNr: i32 =  self.GetAreaNr(self.TPlanObj[plnr].FromArea);
      let mut seaNeighbourCount: i32 =  self.SAObj[areaNr].SeaNeighbourCount;
      for (let mut index1: i32 =  1; index1 <= seaNeighbourCount; index1 += 1)
      {
        let mut index2: i32 =  self.SAObj[areaNr].SeaNeighbour[index1];
        let mut x: i32 =  self.SAObj[index2].X;
        let mut y: i32 =  self.SAObj[index2].Y;
        if (self.game.Data.MapObj[0].HexObj[x, y].Regime == -1 | self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[x, y].Regime) && self.Matrix1[x, y] > num1 & self.AIVP[x, y] > 0)
        {
          num1 = self.Matrix1[x, y];
          num2 = index2;
        }
      }
      return num2 > 0 ? num2 : 0;
    }

    pub Coordinate GetNavalWarCoord(unr: i32, plannr: i32)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      Coordinate navalWarCoord1;
      navalWarCoord1.onmap = false;
      if (self.TPlanObj[plannr].Type != 40)
      {
        Coordinate navalWarCoord2;
        return navalWarCoord2;
      }
      self.TPlanObj[plannr].FriendlyNavy = self.GetRealNavalForceInArea(self.GetAreaNr(self.TPlanObj[plannr].FromArea), plannr, false, true);
      self.TPlanObj[plannr].EnemyNavy = self.GetRealNavalForceInArea(self.GetAreaNr(self.TPlanObj[plannr].FromArea), plannr, false, false);
      let mut num1: i32 =  4;
      if (self.TPlanObj[plannr].SeaStand == 4)
      {
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 10)
          num1 = 4;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 9)
          num1 = 4;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 8)
          num1 = 4;
      }
      else if (self.TPlanObj[plannr].SeaStand == 5)
      {
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 10)
          num1 = 5;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 9)
          num1 = 8;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 8)
          num1 = 4;
      }
      else if (self.TPlanObj[plannr].SeaStand == 6)
      {
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 10)
          num1 = 5;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 9)
          num1 = 5;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 8)
          num1 = 4;
      }
      else if (self.TPlanObj[plannr].SeaStand == 7)
      {
        if (self.game.Data.UnitObj[unr].AIUnitGoal == 10)
          num1 = 5;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 9)
          num1 = 5;
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 8)
          num1 = 5;
      }
      if (num1 == 4)
      {
        navalWarCoord1.x = self.TPlanObj[plannr].FromArea.X;
        navalWarCoord1.y = self.TPlanObj[plannr].FromArea.Y;
        navalWarCoord1.onmap = true;
      }
      else if (num1 == 8 | num1 == 5)
      {
        if ( self.game.HandyFunctionsObj.UnitSupplyStore(unr) * 0.4 >  self.game.Data.UnitObj[unr].Supply & self.game.Data.UnitObj[unr].PassengerCounter == -1)
        {
          navalWarCoord1.x = self.TPlanObj[plannr].FromArea.X;
          navalWarCoord1.y = self.TPlanObj[plannr].FromArea.Y;
          navalWarCoord1.onmap = true;
        }
        else if ( self.game.HandyFunctionsObj.UnitSupplyStore(unr) * 0.8 >  self.game.Data.UnitObj[unr].Supply & self.game.Data.UnitObj[unr].X == self.TPlanObj[plannr].FromArea.X & self.game.Data.UnitObj[unr].Y == self.TPlanObj[plannr].FromArea.Y)
        {
          navalWarCoord1.x = self.TPlanObj[plannr].FromArea.X;
          navalWarCoord1.y = self.TPlanObj[plannr].FromArea.Y;
          navalWarCoord1.onmap = true;
        }
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 8 & self.game.Data.UnitObj[unr].PassengerCounter == -1)
        {
          navalWarCoord1.x = self.TPlanObj[plannr].FromArea.X;
          navalWarCoord1.y = self.TPlanObj[plannr].FromArea.Y;
          navalWarCoord1.onmap = true;
        }
        else if (self.game.Data.UnitObj[unr].AIUnitGoal == 8 & self.game.Data.UnitObj[unr].PassengerCounter > -1)
        {
          SimpleList simpleList = SimpleList::new();
          let mut tid: i32 =  0;
          let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
            {
              if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].CanAmph && self.HexSA[cx, cy] == self.TPlanObj[plannr].SeaTarget)
              {
                let mut tfacing: i32 =  1;
                do
                {
                  navalWarCoord1 = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                  if (navalWarCoord1.onmap && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[navalWarCoord1.x, navalWarCoord1.y].LandscapeType].IsSea)
                  {
                    tid += 1;
                    let mut seaTarget: i32 =  self.TPlanObj[plannr].SeaTarget;
                    let mut num2: i32 =  20;
                    if (navalWarCoord1.x == self.SAObj[seaTarget].X & navalWarCoord1.y == self.SAObj[seaTarget].Y)
                      num2 = 0;
                    if (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[navalWarCoord1.x, navalWarCoord1.y].Regime))
                      num2 += 10 * self.GetHexForceLandStrength(navalWarCoord1.x, navalWarCoord1.y);
                    let mut num3: i32 =  (int) Math.Round( num2 +  num2 * 0.1 * ( self.game.HandyFunctionsObj.Distance(navalWarCoord1.x, navalWarCoord1.y, 0, self.SAObj[seaTarget].X, self.SAObj[seaTarget].Y, 0) / 2.0));
                    let mut tweight: i32 =  (int) Math.Round( num3 +  num3 * 0.1 * ( self.game.HandyFunctionsObj.Distance(navalWarCoord1.x, navalWarCoord1.y, 0, self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0) / 4.0));
                    simpleList.Add(tid, tweight, navalWarCoord1.x, navalWarCoord1.y);
                  }
                  tfacing += 1;
                }
                while (tfacing <= 6);
              }
            }
          }
          if (simpleList.Counter > -1)
          {
            simpleList.Sort();
            navalWarCoord1.x = simpleList.Data1[0];
            navalWarCoord1.y = simpleList.Data2[0];
            navalWarCoord1.onmap = true;
          }
          else
          {
            navalWarCoord1.x = self.game.Data.UnitObj[unr].X;
            navalWarCoord1.y = self.game.Data.UnitObj[unr].Y;
            navalWarCoord1.onmap = true;
          }
        }
        else
        {
          SimpleList simpleList = SimpleList::new();
          let mut tid1: i32 =  0;
          num4: i32;
          if (self.game.Data.UnitObj[unr].AIUnitGoal != 8)
          {
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
              {
                if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType].IsSea && self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[0]].Regime))
                {
                  tid1 += 1;
                  let mut tweight: i32 =  self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0, index1, index2, 0);
                  num4 = 0;
                  if (self.GetAreaNr(self.TPlanObj[plannr].FromArea) == self.HexSeaSA[index1, index2])
                    num4 = 1;
                  if (self.TPlanObj[plannr].SeaTarget > 0 && self.TPlanObj[plannr].SeaTarget == self.HexSeaSA[index1, index2])
                    num4 = 1;
                  if (num4 == 0)
                    tweight *= 3;
                  simpleList.Add(tid1, tweight, index1, index2);
                }
              }
            }
          }
          if (simpleList.Counter == -1 & self.TPlanObj[plannr].SeaTarget > 0 | self.game.Data.UnitObj[unr].AIUnitGoal == 8 & num1 == 5 && self.TPlanObj[plannr].SeaTarget <= self.SACount)
          {
            let mut x: i32 =  self.SAObj[self.TPlanObj[plannr].SeaTarget].X;
            let mut y: i32 =  self.SAObj[self.TPlanObj[plannr].SeaTarget].Y;
            let mut tweight: i32 =  self.game.HandyFunctionsObj.Distance(self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, 0, x, y, 0);
            let mut tid2: i32 =  tid1 + 1;
            simpleList.Add(tid2, tweight, x, y);
          }
          simpleList.Sort();
          let mut num5: i32 =  -1;
          let mut num6: i32 =  -1;
          let mut num7: i32 =  0;
          if (simpleList.Counter > -1)
          {
            self.SetNavalMatrix1(simpleList.Data1[0], simpleList.Data2[0]);
            let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
            for (let mut index3: i32 =  0; index3 <= mapWidth; index3 += 1)
            {
              let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
              for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
              {
                if (self.HexSeaSA[index3, index4] == self.HexSeaSA[self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y])
                  num4 = 1;
                if (num1 == 5 & self.HexSeaSA[index3, index4] == self.TPlanObj[plannr].SeaTarget)
                  num4 = 1;
                if (self.Matrix1[index3, index4] > num7 & num4 == 1 && self.Matrix1[index3, index4] > num7)
                {
                  num7 = self.Matrix1[index3, index4];
                  num5 = index3;
                  num6 = index4;
                }
              }
            }
          }
          if (num7 > 0)
          {
            navalWarCoord1.x = num5;
            navalWarCoord1.y = num6;
            navalWarCoord1.onmap = true;
          }
        }
      }
      if (navalWarCoord1.onmap)
      {
        self.game.Data.UnitObj[unr].AINavtargetX = navalWarCoord1.x;
        self.game.Data.UnitObj[unr].AINavtargetY = navalWarCoord1.y;
      }
      else
      {
        self.game.Data.UnitObj[unr].AINavtargetX = -1;
        self.game.Data.UnitObj[unr].AINavtargetY = -1;
      }
      return navalWarCoord1;
    }

    pub Coordinate GetReserveCoord(plannr: i32, resnr: i32)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      Coordinate reserveCoord;
      if (resnr <= 1)
      {
        reserveCoord.x = self.TPlanObj[plannr].FromArea.X;
        reserveCoord.y = self.TPlanObj[plannr].FromArea.Y;
        reserveCoord.onmap = true;
      }
      else
      {
        CoordList coordList = CoordList::new();
        coordList.AddCoord(self.TPlanObj[plannr].FromArea.X, self.TPlanObj[plannr].FromArea.Y, 0);
        let mut num1: i32 =  resnr;
        for (let mut index1: i32 =  2; index1 <= num1; index1 += 1)
        {
          SimpleList simpleList = SimpleList::new();
          let mut tid: i32 =  0;
          let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
          for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
          {
            let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
            for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            {
              if (self.HexSA[index2, index3] == self.GetAreaNr(self.TPlanObj[plannr].FromArea))
              {
                let mut num2: i32 =  0;
                let mut tfacing: i32 =  1;
                do
                {
                  Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(index2, index3, 0, tfacing);
                  if (coordinate.onmap && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                    num2 = 1;
                  tfacing += 1;
                }
                while (tfacing <= 6);
                if (num2 == 1)
                {
                  let mut num3: i32 =  999;
                  let mut counter: i32 =  coordList.counter;
                  tweight: i32;
                  for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
                  {
                    tweight = self.game.HandyFunctionsObj.Distance(index2, index3, 0, coordList.coord[index4].x, coordList.coord[index4].y, 0);
                    if (tweight < num3)
                      num3 = tweight;
                  }
                  if (tweight > 0)
                  {
                    tid += 1;
                    simpleList.Add(tid, tweight, index2, index3);
                  }
                }
              }
            }
          }
          if (simpleList.Counter > -1)
          {
            simpleList.Sort();
            reserveCoord.x = simpleList.Data1[simpleList.Counter];
            reserveCoord.y = simpleList.Data2[simpleList.Counter];
            reserveCoord.onmap = true;
            coordList.AddCoord(reserveCoord.x, reserveCoord.y, 0);
          }
        }
        if (!reserveCoord.onmap)
        {
          reserveCoord.x = self.TPlanObj[plannr].FromArea.X;
          reserveCoord.y = self.TPlanObj[plannr].FromArea.Y;
          reserveCoord.onmap = true;
        }
      }
      return reserveCoord;
    }

    pub Coordinate GetEngineerCoord(engcount: i32, plannr: i32)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      engcount = 1;
      CoordList coordList1 = CoordList::new();
      CoordList coordList2 = CoordList::new();
      x1: i32;
      y1: i32;
      if (self.TPlanObj[plannr].Type == 20 | self.TPlanObj[plannr].Type == 50)
      {
        x1 = self.TPlanObj[plannr].TooArea.X;
        y1 = self.TPlanObj[plannr].TooArea.Y;
      }
      else if (self.TPlanObj[plannr].Type == 40)
      {
        if (self.TPlanObj[plannr].CurrentBackRoad < 1)
        {
          let mut x2: i32 =  self.TPlanObj[plannr].FromArea.X;
          let mut y2: i32 =  self.TPlanObj[plannr].FromArea.Y;
          Coordinate engineerCoord;
          engineerCoord.x = x2;
          engineerCoord.y = y2;
          engineerCoord.onmap = true;
          return engineerCoord;
        }
        x1 = self.SAObj[self.TPlanObj[plannr].CurrentBackRoad].X;
        y1 = self.SAObj[self.TPlanObj[plannr].CurrentBackRoad].Y;
      }
      x3: i32;
      y3: i32;
      x4: i32;
      y4: i32;
      if (self.TPlanObj[plannr].Type == 20 | self.TPlanObj[plannr].Type == 50)
      {
        let mut index: i32 =  self.GetMostUsedHQ(plannr);
        if (index > -1)
        {
          if (self.game.Data.UnitObj[index].HQ > -1 & self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr].Type != 30)
            index = self.game.Data.UnitObj[index].HQ;
          if (self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr].Type == 30)
          {
            x3 = self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr].FromArea.X;
            y3 = self.TPlanObj[self.game.Data.UnitObj[index].AIPlanNr].FromArea.Y;
            x4 = self.TPlanObj[plannr].FromArea.X;
            y4 = self.TPlanObj[plannr].FromArea.Y;
          }
          else
          {
            x3 = self.TPlanObj[plannr].FromArea.X;
            y3 = self.TPlanObj[plannr].FromArea.Y;
            x4 = self.TPlanObj[plannr].FromArea.X;
            y4 = self.TPlanObj[plannr].FromArea.Y;
          }
        }
        else
        {
          x3 = self.TPlanObj[plannr].FromArea.X;
          y3 = self.TPlanObj[plannr].FromArea.Y;
          x4 = self.TPlanObj[plannr].FromArea.X;
          y4 = self.TPlanObj[plannr].FromArea.Y;
        }
      }
      else if (self.TPlanObj[plannr].Type == 40)
      {
        x3 = self.TPlanObj[plannr].FromArea.X;
        y3 = self.TPlanObj[plannr].FromArea.Y;
        x4 = self.TPlanObj[plannr].FromArea.X;
        y4 = self.TPlanObj[plannr].FromArea.Y;
      }
      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 100, x4, y4, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true);
      let mut num1: i32 =  1;
      let mut x5: i32 =  x1;
      let mut y5: i32 =  y1;
      coordList1.AddCoord(x5, y5, 0);
      numArray1[x5, y5] = self.game.EditObj.TempValue[0].Value[x5, y5];
      Coordinate coordinate1;
      while (num1 == 1)
      {
        coordinate1 = self.game.EditObj.TempCameFrom[0].Value[x5, y5];
        if (coordinate1.onmap)
        {
          x5 = coordinate1.x;
          y5 = coordinate1.y;
          coordList1.AddCoord(coordinate1.x, coordinate1.y, 0);
          numArray1[x5, y5] = self.game.EditObj.TempValue[0].Value[x5, y5];
        }
        else
          num1 = 0;
      }
      let mut counter1: i32 =  coordList1.counter;
      let mut num2: i32 =  -1;
      if (!(x3 == x4 & y3 == y4))
      {
        self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x3, y3, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true);
        let mut num3: i32 =  1;
        let mut index1: i32 =  x4;
        let mut index2: i32 =  y4;
        while (num3 == 1)
        {
          coordinate1 = self.game.EditObj.TempCameFrom[0].Value[index1, index2];
          if (coordinate1.onmap)
          {
            index1 = coordinate1.x;
            index2 = coordinate1.y;
            coordList1.AddCoord(coordinate1.x, coordinate1.y, 0);
            if (num2 == -1)
              num2 = self.game.EditObj.TempValue[0].Value[index1, index2];
            numArray1[index1, index2] = self.game.EditObj.TempValue[0].Value[index1, index2];
          }
          else
            num3 = 0;
        }
        let mut num4: i32 =  counter1;
        for (let mut index3: i32 =  0; index3 <= num4; index3 += 1)
        {
          numArray4: Vec<i32> = numArray1;
          numArray5: Vec<i32> = numArray4;
          let mut x6: i32 =  coordList1.coord[index3].x;
          let mut index4: i32 =  x6;
          let mut y6: i32 =  coordList1.coord[index3].y;
          let mut index5: i32 =  y6;
          let mut num5: i32 =  numArray4[x6, y6] + num2;
          numArray5[index4, index5] = num5;
        }
      }
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
          numArray1[index6, index7] = self.game.EditObj.TempValue[0].Value[index6, index7];
      }
      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x4, y4, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true);
      let mut num6: i32 =  1;
      let mut x7: i32 =  x1;
      let mut y7: i32 =  y1;
      coordList2.AddCoord(x7, y7, 0);
      numArray2[x7, y7] = self.game.EditObj.TempValue[0].Value[x7, y7];
      while (num6 == 1)
      {
        coordinate1 = self.game.EditObj.TempCameFrom[0].Value[x7, y7];
        if (coordinate1.onmap)
        {
          x7 = coordinate1.x;
          y7 = coordinate1.y;
          coordList2.AddCoord(coordinate1.x, coordinate1.y, 0);
          numArray2[x7, y7] = self.game.EditObj.TempValue[0].Value[x7, y7];
        }
        else
          num6 = 0;
      }
      let mut counter2: i32 =  coordList2.counter;
      let mut num7: i32 =  -1;
      if (!(x3 == x4 & y3 == y4))
      {
        self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x3, y3, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true);
        let mut num8: i32 =  1;
        let mut index8: i32 =  x4;
        let mut index9: i32 =  y4;
        while (num8 == 1)
        {
          coordinate1 = self.game.EditObj.TempCameFrom[0].Value[index8, index9];
          if (coordinate1.onmap)
          {
            index8 = coordinate1.x;
            index9 = coordinate1.y;
            coordList2.AddCoord(coordinate1.x, coordinate1.y, 0);
            if (num7 == -1)
              num7 = self.game.EditObj.TempValue[0].Value[index8, index9];
            numArray2[index8, index9] = self.game.EditObj.TempValue[0].Value[index8, index9];
          }
          else
            num8 = 0;
        }
        let mut num9: i32 =  counter2;
        for (let mut index10: i32 =  0; index10 <= num9; index10 += 1)
        {
          numArray6: Vec<i32> = numArray2;
          numArray7: Vec<i32> = numArray6;
          let mut x8: i32 =  coordList1.coord[index10].x;
          let mut index11: i32 =  x8;
          let mut y8: i32 =  coordList1.coord[index10].y;
          let mut index12: i32 =  y8;
          let mut num10: i32 =  numArray6[x8, y8] + num7;
          numArray7[index11, index12] = num10;
        }
      }
      let mut num11: i32 =  0;
      counter3: i32;
      for (counter3 = coordList2.counter; counter3 >= 0 && counter3 != 0; counter3 += -1)
      {
        Coordinate coordinate2 = self.game.HandyFunctionsObj.MoveApCostPreview2(coordList2.coord[counter3].x, coordList2.coord[counter3].y, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, coordList2.coord[counter3].x, coordList2.coord[counter3].y, 0, coordList2.coord[counter3 - 1].x, coordList2.coord[counter3 - 1].y, 0, false, NoAPPenalties: true, BlockAllSea: true);
        let mut x9: i32 =  coordinate2.x;
        coordinate2 = self.game.HandyFunctionsObj.MoveApCostPreview2(coordList2.coord[counter3].x, coordList2.coord[counter3].y, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, coordList2.coord[counter3].x, coordList2.coord[counter3].y, 0, coordList2.coord[counter3 - 1].x, coordList2.coord[counter3 - 1].y, 0, false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true);
        let mut x10: i32 =  coordinate2.x;
        if (x9 > x10)
          num11 += 1;
        if (num11 == engcount)
          break;
      }
      if (num11 == engcount)
      {
        Coordinate engineerCoord;
        engineerCoord.x = coordList2.coord[counter3].x;
        engineerCoord.y = coordList2.coord[counter3].y;
        engineerCoord.onmap = true;
        engineerCoord.data1 = self.game.HandyFunctionsObj.HexFacing(engineerCoord.x, engineerCoord.y, 0, coordList2.coord[counter3 - 1].x, coordList2.coord[counter3 - 1].y, 0);
        return engineerCoord;
      }
      Coordinate engineerCoord1;
      engineerCoord1.onmap = false;
      return engineerCoord1;
    }

    pub Coordinate GetEngineerCoordOLDBACKUP(engcount: i32, plannr: i32)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      engcount = 1;
      CoordList coordList1 = CoordList::new();
      CoordList coordList2 = CoordList::new();
      x1: i32;
      y1: i32;
      if (self.TPlanObj[plannr].Type == 20)
      {
        x1 = self.TPlanObj[plannr].TooArea.X;
        y1 = self.TPlanObj[plannr].TooArea.Y;
      }
      let mut index1: i32 =  self.GetMostUsedHQ(plannr);
      x2: i32;
      y2: i32;
      x3: i32;
      y3: i32;
      if (index1 > -1)
      {
        if (self.game.Data.UnitObj[index1].HQ > -1 & self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].Type != 30)
          index1 = self.game.Data.UnitObj[index1].HQ;
        if (self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].Type == 30)
        {
          x2 = self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].FromArea.X;
          y2 = self.TPlanObj[self.game.Data.UnitObj[index1].AIPlanNr].FromArea.Y;
          x3 = self.TPlanObj[plannr].FromArea.X;
          y3 = self.TPlanObj[plannr].FromArea.Y;
        }
        else
        {
          x2 = self.TPlanObj[plannr].FromArea.X;
          y2 = self.TPlanObj[plannr].FromArea.Y;
          x3 = self.TPlanObj[plannr].FromArea.X;
          y3 = self.TPlanObj[plannr].FromArea.Y;
        }
      }
      else
      {
        x2 = self.TPlanObj[plannr].FromArea.X;
        y2 = self.TPlanObj[plannr].FromArea.Y;
        x3 = self.TPlanObj[plannr].FromArea.X;
        y3 = self.TPlanObj[plannr].FromArea.Y;
      }
      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x3, y3, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true);
      let mut num1: i32 =  1;
      let mut x4: i32 =  x1;
      let mut y4: i32 =  y1;
      coordList1.AddCoord(x4, y4, 0);
      numArray1[x4, y4] = self.game.EditObj.TempValue[0].Value[x4, y4];
      Coordinate coordinate1;
      while (num1 == 1)
      {
        coordinate1 = self.game.EditObj.TempCameFrom[0].Value[x4, y4];
        if (coordinate1.onmap)
        {
          x4 = coordinate1.x;
          y4 = coordinate1.y;
          coordList1.AddCoord(coordinate1.x, coordinate1.y, 0);
          numArray1[x4, y4] = self.game.EditObj.TempValue[0].Value[x4, y4];
        }
        else
          num1 = 0;
      }
      let mut counter1: i32 =  coordList1.counter;
      let mut num2: i32 =  -1;
      if (!(x2 == x3 & y2 == y3))
      {
        self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x2, y2, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true);
        let mut num3: i32 =  1;
        let mut index2: i32 =  x3;
        let mut index3: i32 =  y3;
        while (num3 == 1)
        {
          coordinate1 = self.game.EditObj.TempCameFrom[0].Value[index2, index3];
          if (coordinate1.onmap)
          {
            index2 = coordinate1.x;
            index3 = coordinate1.y;
            coordList1.AddCoord(coordinate1.x, coordinate1.y, 0);
            if (num2 == -1)
              num2 = self.game.EditObj.TempValue[0].Value[index2, index3];
            numArray1[index2, index3] = self.game.EditObj.TempValue[0].Value[index2, index3];
          }
          else
            num3 = 0;
        }
        let mut num4: i32 =  counter1;
        for (let mut index4: i32 =  0; index4 <= num4; index4 += 1)
        {
          numArray4: Vec<i32> = numArray1;
          numArray5: Vec<i32> = numArray4;
          let mut x5: i32 =  coordList1.coord[index4].x;
          let mut index5: i32 =  x5;
          let mut y5: i32 =  coordList1.coord[index4].y;
          let mut index6: i32 =  y5;
          let mut num5: i32 =  numArray4[x5, y5] + num2;
          numArray5[index5, index6] = num5;
        }
      }
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index7: i32 =  0; index7 <= mapWidth; index7 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
          numArray1[index7, index8] = self.game.EditObj.TempValue[0].Value[index7, index8];
      }
      self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x3, y3, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true);
      let mut num6: i32 =  1;
      let mut x6: i32 =  x1;
      let mut y6: i32 =  y1;
      coordList2.AddCoord(x6, y6, 0);
      numArray2[x6, y6] = self.game.EditObj.TempValue[0].Value[x6, y6];
      while (num6 == 1)
      {
        coordinate1 = self.game.EditObj.TempCameFrom[0].Value[x6, y6];
        if (coordinate1.onmap)
        {
          x6 = coordinate1.x;
          y6 = coordinate1.y;
          coordList2.AddCoord(coordinate1.x, coordinate1.y, 0);
          numArray2[x6, y6] = self.game.EditObj.TempValue[0].Value[x6, y6];
        }
        else
          num6 = 0;
      }
      let mut counter2: i32 =  coordList2.counter;
      let mut num7: i32 =  -1;
      if (!(x2 == x3 & y2 == y3))
      {
        self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, 200, x2, y2, 0, dontenterenemy: false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true);
        let mut num8: i32 =  1;
        let mut index9: i32 =  x3;
        let mut index10: i32 =  y3;
        while (num8 == 1)
        {
          coordinate1 = self.game.EditObj.TempCameFrom[0].Value[index9, index10];
          if (coordinate1.onmap)
          {
            index9 = coordinate1.x;
            index10 = coordinate1.y;
            coordList2.AddCoord(coordinate1.x, coordinate1.y, 0);
            if (num7 == -1)
              num7 = self.game.EditObj.TempValue[0].Value[index9, index10];
            numArray2[index9, index10] = self.game.EditObj.TempValue[0].Value[index9, index10];
          }
          else
            num8 = 0;
        }
        let mut num9: i32 =  counter2;
        for (let mut index11: i32 =  0; index11 <= num9; index11 += 1)
        {
          numArray6: Vec<i32> = numArray2;
          numArray7: Vec<i32> = numArray6;
          let mut x7: i32 =  coordList1.coord[index11].x;
          let mut index12: i32 =  x7;
          let mut y7: i32 =  coordList1.coord[index11].y;
          let mut index13: i32 =  y7;
          let mut num10: i32 =  numArray6[x7, y7] + num7;
          numArray7[index12, index13] = num10;
        }
      }
      let mut num11: i32 =  0;
      let mut num12: i32 =  -1;
      counter3: i32;
      for (counter3 = coordList2.counter; counter3 >= 0; counter3 += -1)
      {
        num12 += 1;
        if (numArray1[coordList2.coord[counter3].x, coordList2.coord[counter3].y] > numArray2[coordList2.coord[counter3].x, coordList2.coord[counter3].y])
          num11 += 1;
        if (num11 == engcount)
          break;
      }
      if (num11 == engcount)
      {
        index14: i32;
        for (index14 = counter3 - 1; index14 >= 0; index14 += -1)
        {
          Coordinate coordinate2 = self.game.HandyFunctionsObj.MoveApCostPreview2(coordList2.coord[index14].x, coordList2.coord[index14].y, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, coordList2.coord[index14].x, coordList2.coord[index14].y, 0, coordList2.coord[index14 - 1].x, coordList2.coord[index14 - 1].y, 0, false, NoAPPenalties: true, BlockAllSea: true);
          let mut x8: i32 =  coordinate2.x;
          coordinate2 = self.game.HandyFunctionsObj.MoveApCostPreview2(coordList2.coord[index14].x, coordList2.coord[index14].y, self.game.Data.Turn, (int) Math.Round( self.game.Data.RuleVar[99]), 0, coordList2.coord[index14].x, coordList2.coord[index14].y, 0, coordList2.coord[index14 - 1].x, coordList2.coord[index14 - 1].y, 0, false, NoAPPenalties: true, BlockAllSea: true, EngineerTest: true);
          let mut x9: i32 =  coordinate2.x;
          if (x8 > x9)
            break;
        }
        Coordinate engineerCoordOldbackup;
        engineerCoordOldbackup.x = coordList2.coord[index14].x;
        engineerCoordOldbackup.y = coordList2.coord[index14].y;
        engineerCoordOldbackup.onmap = true;
        engineerCoordOldbackup.data1 = self.game.HandyFunctionsObj.HexFacing(engineerCoordOldbackup.x, engineerCoordOldbackup.y, 0, coordList2.coord[index14 - 1].x, coordList2.coord[index14 - 1].y, 0);
        return engineerCoordOldbackup;
      }
      Coordinate engineerCoordOldbackup1;
      engineerCoordOldbackup1.onmap = false;
      return engineerCoordOldbackup1;
    }

    pub Coordinate GetEscapeCoord(x: i32, y: i32, areanr: i32)
    {
      numArray1: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray2: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray3: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      numArray4: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          numArray4[index1, index2] = 0;
          if (self.HexSA[index1, index2] == areanr)
          {
            numArray4[index1, index2] = (int) Math.Round( self.game.Data.RuleVar[152]);
            numArray1[index1, index2] = 1;
          }
        }
      }
      let mut num1: i32 =  0;
      let mut num2: i32 =  1;
      Coordinate escapeCoord;
      while (num2 == 1)
      {
        num2 = 0;
        num1 += 1;
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (numArray1[cx, cy] == num1)
            {
              let mut tfacing: i32 =  1;
              do
              {
                escapeCoord = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (escapeCoord.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[escapeCoord.x, escapeCoord.y].LandscapeType].IsSea && numArray1[escapeCoord.x, escapeCoord.y] == 0 && !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[escapeCoord.x, escapeCoord.y].Regime, self.game.Data.Turn) && self.GetHexForceLandStrength(escapeCoord.x, escapeCoord.y) == 0)
                {
                  numArray1[escapeCoord.x, escapeCoord.y] = num1 + 1;
                  num2 = 1;
                  numArray4[escapeCoord.x, escapeCoord.y] = (int) Math.Round(Conversion.Int( numArray4[cx, cy] * 0.95));
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      let mut mapWidth3: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index3: i32 =  0; index3 <= mapWidth3; index3 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
        {
          numArray1[index3, index4] = 0;
          numArray2[index3, index4] = numArray4[index3, index4];
          numArray4[index3, index4] = 0;
        }
      }
      numArray4[x, y] = (int) Math.Round( self.game.Data.RuleVar[152]);
      numArray1[x, y] = 1;
      let mut num3: i32 =  0;
      let mut num4: i32 =  1;
      while (num4 == 1)
      {
        num4 = 0;
        num3 += 1;
        let mut mapWidth4: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth4; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (numArray1[cx, cy] == num3)
            {
              let mut tfacing: i32 =  1;
              do
              {
                escapeCoord = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (escapeCoord.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[escapeCoord.x, escapeCoord.y].LandscapeType].IsSea && numArray1[escapeCoord.x, escapeCoord.y] == 0 && numArray2[escapeCoord.x, escapeCoord.y] > 0)
                {
                  numArray1[escapeCoord.x, escapeCoord.y] = num3 + 1;
                  num4 = 1;
                  numArray4[escapeCoord.x, escapeCoord.y] = (int) Math.Round(Conversion.Int( numArray4[cx, cy] * 0.95));
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      escapeCoord = Coordinate::new();
      x = -1;
      y = -1;
      let mut num5: i32 =  0;
      let mut mapWidth5: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index5: i32 =  0; index5 <= mapWidth5; index5 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
        {
          if (self.HexSA[index5, index6] == areanr && numArray4[index5, index6] > num5)
          {
            num5 = numArray4[index5, index6];
            x = index5;
            y = index6;
          }
        }
      }
      if (num5 > 0)
      {
        escapeCoord.onmap = true;
        escapeCoord.x = x;
        escapeCoord.y = y;
      }
      else
        escapeCoord.onmap = false;
      return escapeCoord;
    }

    pub fn SetMatrixEnemyFront(notregnr: i32, bool emptyhexisdanger = false)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.Matrix1 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 =  0; x <= mapWidth1; x += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 =  0; y <= mapHeight; y += 1)
        {
          self.Matrix1[x, y] = 0;
          if (self.game.Data.MapObj[0].HexObj[x, y].Regime > -1 & !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[0].HexObj[x, y].Regime, notregnr))
          {
            if (self.game.Data.MapObj[0].HexObj[x, y].UnitCounter > -1)
            {
              numArray[x, y] = 1;
              self.Matrix1[x, y] = (int) Math.Round( (self.game.Data.RuleVar[152] * ( self.GetHexForceLandStrength(x, y) / self.game.Data.RuleVar[183])));
              if ( self.Matrix1[x, y] >  self.game.Data.RuleVar[152])
                self.Matrix1[x, y] = (int) Math.Round( self.game.Data.RuleVar[152]);
            }
            if (emptyhexisdanger)
              self.Matrix1[x, y] = (int) Math.Round( self.game.Data.RuleVar[152]);
          }
        }
      }
      let mut num1: i32 =  0;
      let mut num2: i32 =  1;
      while (num2 == 1)
      {
        num2 = 0;
        num1 += 1;
        let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth2; cx += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (numArray[cx, cy] == num1)
            {
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea && numArray[coordinate.x, coordinate.y] == 0)
                {
                  numArray[coordinate.x, coordinate.y] = num1 + 1;
                  num2 = 1;
                  self.Matrix1[coordinate.x, coordinate.y] = (int) Math.Round(Conversion.Int( self.Matrix1[cx, cy] * 0.95));
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
    }

    pub fn GetAreaNr(SAClass tempSA) -> i32
    {
      try
      {
        let mut saCount: i32 =  self.SACount;
        for (let mut areaNr: i32 =  1; areaNr <= saCount; areaNr += 1)
        {
          if (self.SAObj[areaNr].X == tempSA.X & self.SAObj[areaNr].Y == tempSA.Y)
            return areaNr;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return 0;
    }

    pub IsAreaNeighbour: bool(area1: i32, area2: i32)
    {
      let mut neighbourCount: i32 =  self.SAObj[area1].NeighbourCount;
      for (let mut index: i32 =  1; index <= neighbourCount; index += 1)
      {
        if (self.SAObj[area1].Neighbour[index] == area2)
          return true;
      }
      return false;
    }

    pub GetForceLandStrength: i32(
      unr: i32,
      bool withoutmods = false,
      bool asattack = false,
      let mut attackx: i32 =  -1,
      let mut attacky: i32 =  -1)
    {
      if (self.game.Data.UnitObj[unr].SFCount <= -1)
        return 0;
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      forceLandStrength: i32;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
        let mut powerPts: i32 =  self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].PowerPts;
        let mut num1: i32 =  self.game.Data.SFObj[sf].Qty * powerPts;
        let mut regime: i32 =  self.game.Data.UnitObj[unr].Regime;
        if (self.game.Data.SFTypeObj[type].Theater == 0)
        {
          if (!asattack)
          {
            if (!withoutmods)
            {
              let mut num2: i32 =  (int) Math.Round( num1 * 0.5 +  num1 * 0.5 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
              let mut num3: i32 =  (int) Math.Round( num2 * 0.5 +  num2 * 0.5 * ( (self.game.Data.SFObj[sf].DefMod + 100) / 100.0));
              let mut num4: i32 =  (int) Math.Round( num3 * 0.1 +  num3 * 0.9 * ( self.game.Data.UnitObj[unr].SupplyConsume / 100.0));
              if (self.game.Data.Turn != self.game.Data.UnitObj[unr].Regime)
                num4 = (int) Math.Round( num4 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
              let mut num5: i32 =  (int) Math.Round( num4 * (1.0 +  self.game.Data.SFObj[sf].CurrentEntrench / 100.0));
              if (self.game.Data.UnitObj[unr].X != -1)
                num5 = (int) Math.Round( ( num5 * self.game.Data.SFTypeObj[type].CombatModDef[self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y].LandscapeType]));
              num1 = (int) Math.Round( ( num5 * self.game.Data.PeopleObj[self.game.Data.SFObj[sf].People].BattleForMod[self.game.Data.PeopleObj[self.game.Data.RegimeObj[regime].People].PeopleGroup]));
            }
          }
          else
          {
            let mut num6: i32 =  (int) Math.Round( num1 * 0.5 +  num1 * 0.5 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            let mut num7: i32 =  (int) Math.Round( num6 * 0.1 +  num6 * 0.9 * ( (self.game.Data.SFObj[sf].OffMod + 100) / 100.0));
            num1 = (int) Math.Round( num7 * 0.1 +  num7 * 0.9 * ( self.game.Data.UnitObj[unr].SupplyConsume / 100.0));
            if (self.game.Data.Turn != self.game.Data.UnitObj[unr].Regime)
              num1 = (int) Math.Round( num1 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            if (attackx > -1)
            {
              num1 = (int) Math.Round( ( num1 * self.AverageCombatPerform(sf, type, attackx, attacky)));
              if (self.game.Data.UnitObj[unr].X != -1)
                num1 = (int) Math.Round( ( num1 * self.game.Data.SFTypeObj[type].CombatModAtt[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType]));
            }
          }
          let mut Number: i32 =  (int) Math.Round( num1 * 0.5 + 1.5 *  num1 * ( self.game.Data.SFObj[sf].Xp / 100.0));
          forceLandStrength += Conversion.Int(Number);
        }
      }
      return forceLandStrength;
    }

    pub GetForceAirStrength: i32(
      unr: i32,
      bool withoutmods = false,
      bool asattack = false,
      let mut attackx: i32 =  -1,
      let mut attacky: i32 =  -1)
    {
      if (self.game.Data.UnitObj[unr].SFCount <= -1)
        return 0;
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      forceAirStrength: i32;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
        let mut powerPts: i32 =  self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].PowerPts;
        let mut theater: i32 =  self.game.Data.SFTypeObj[type].Theater;
        let mut num1: i32 =  self.game.Data.SFObj[sf].Qty * powerPts;
        if (theater == 2)
        {
          if (!asattack)
          {
            if (!withoutmods)
            {
              let mut num2: i32 =  (int) Math.Round( num1 * 0.5 +  num1 * 0.5 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
              num1 = (int) Math.Round( num2 * 0.5 +  num2 * 0.5 * ( (self.game.Data.SFObj[sf].DefMod + 100) / 100.0));
              if (self.game.Data.Turn != self.game.Data.UnitObj[unr].Regime)
                num1 = (int) Math.Round( num1 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            }
          }
          else
          {
            let mut num3: i32 =  (int) Math.Round( num1 * 0.5 +  num1 * 0.5 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            num1 = (int) Math.Round( num3 * 0.1 +  num3 * 0.9 * ( (self.game.Data.SFObj[sf].OffMod + 100) / 100.0));
            if (self.game.Data.Turn != self.game.Data.UnitObj[unr].Regime)
              num1 = (int) Math.Round( num1 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            if (attackx > -1)
              num1 = (int) Math.Round( ( (int) Math.Round( ( num1 * self.AverageCombatPerform(sf, type, attackx, attacky))) * self.game.Data.SFTypeObj[type].CombatModAtt[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType]));
          }
          let mut Number: i32 =  (int) Math.Round( num1 * 0.5 + 1.5 *  num1 * ( self.game.Data.SFObj[sf].Xp / 100.0));
          forceAirStrength += Conversion.Int(Number);
        }
      }
      return forceAirStrength;
    }

    pub GetForceNavalStrength: i32(
      unr: i32,
      bool withoutmods = false,
      bool asattack = false,
      let mut attackx: i32 =  -1,
      let mut attacky: i32 =  -1)
    {
      if (self.game.Data.UnitObj[unr].SFCount <= -1)
        return 0;
      let mut sfCount: i32 =  self.game.Data.UnitObj[unr].SFCount;
      forceNavalStrength: i32;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  self.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 =  self.game.Data.SFObj[sf].Type;
        let mut num1: i32 =  self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].PowerPts;
        if (self.game.Data.SFTypeObj[type].AIRoleScore[18] < 1 && self.game.Data.SFTypeObj[type].AIRoleScore[19] < 1 && self.game.Data.SFTypeObj[type].AIRoleScore[17] > 0)
          num1 = 0;
        let mut theater: i32 =  self.game.Data.SFTypeObj[type].Theater;
        let mut num2: i32 =  self.game.Data.SFObj[sf].Qty * num1;
        if (theater == 1)
        {
          if (!asattack)
          {
            if (!withoutmods)
            {
              let mut num3: i32 =  (int) Math.Round( num2 * 0.5 +  num2 * 0.5 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
              num2 = (int) Math.Round( num3 * 0.5 +  num3 * 0.5 * ( (self.game.Data.SFObj[sf].DefMod + 100) / 100.0));
              if (self.game.Data.Turn != self.game.Data.UnitObj[unr].Regime)
                num2 = (int) Math.Round( num2 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            }
          }
          else
          {
            let mut num4: i32 =  (int) Math.Round( num2 * 0.1 +  num2 * 0.9 * ( (self.game.Data.SFObj[sf].OffMod + 100) / 100.0));
            num2 = (int) Math.Round( num4 * 0.5 +  num4 * 0.5 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            if (self.game.Data.Turn != self.game.Data.UnitObj[unr].Regime)
              num2 = (int) Math.Round( num2 * ( self.game.Data.SFObj[sf].Rdn / 100.0));
            if (attackx > -1)
              num2 = (int) Math.Round( ( (int) Math.Round( ( num2 * self.AverageCombatPerform(sf, type, attackx, attacky))) * self.game.Data.SFTypeObj[type].CombatModAtt[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType]));
          }
          let mut Number: i32 =  (int) Math.Round( num2 * 0.5 + 1.5 *  num2 * ( self.game.Data.SFObj[sf].Xp / 100.0));
          forceNavalStrength += Conversion.Int(Number);
        }
      }
      return forceNavalStrength;
    }

    pub FindBestSuitedItemType: i32(
      unr: i32,
      role: i32,
      let mut prodpts: i32 =  -1,
      let mut locnr: i32 =  -1,
      bool randomeffect = false,
      let mut rangy: i32 =  5)
    {
      SimpleList simpleList1 = SimpleList::new();
      int[] numArray1 = new int[self.game.Data.LandscapeTypeCounter + 1];
      if (role == -1)
        return -1;
      let mut aiPlanNr: i32 =  self.game.Data.UnitObj[unr].AIPlanNr;
      if (aiPlanNr < 1)
        return -1;
      let mut itemTypeCounter: i32 =  self.game.Data.ItemTypeCounter;
      for (let mut index: i32 =  0; index <= itemTypeCounter; index += 1)
      {
        let mut isSfType: i32 =  self.game.Data.ItemTypeObj[index].IsSFType;
        if (isSfType > -1 && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, index, self.game.Data.RegimeObj[self.game.Data.Turn].People).result)
        {
          let mut tweight: i32 =  self.game.Data.SFTypeObj[isSfType].AIRoleScore[role];
          if (tweight > 0)
          {
            if (prodpts > -1 && self.game.Data.ItemTypeObj[index].ProdWeight > prodpts)
              tweight = (int) Math.Round( tweight / (0.5 +  self.game.Data.ItemTypeObj[index].ProdWeight /  prodpts));
            if (tweight > 0)
              simpleList1.Add(isSfType, tweight, index);
          }
        }
      }
      if (simpleList1.Counter == -1)
        return -1;
      let mut x1: i32 =  self.game.Data.UnitObj[unr].X;
      let mut y1: i32 =  self.game.Data.UnitObj[unr].Y;
      if (x1 == -1)
        return -1;
      let mut counter: i32 =  simpleList1.Counter;
      for (let mut index1: i32 =  0; index1 <= counter; index1 += 1)
      {
        let mut num1: i32 =  simpleList1.Weight[index1];
        let mut num2: i32 =  0;
        let mut landscapeTypeCounter1: i32 =  self.game.Data.LandscapeTypeCounter;
        num3: i32;
        for (let mut index2: i32 =  0; index2 <= landscapeTypeCounter1; index2 += 1)
        {
          numArray1[index2] = 0;
          num3 = 0;
        }
        let mut num4: i32 =  x1 - rangy;
        let mut num5: i32 =  x1 + rangy;
        for (let mut index3: i32 =  num4; index3 <= num5; index3 += 1)
        {
          let mut x2: i32 =  index3;
          if (self.game.Data.MapObj[0].MapLoop & x2 < 0)
            x2 = self.game.Data.MapObj[0].MapWidth + x2 + 1;
          if (self.game.Data.MapObj[0].MapLoop & x2 > self.game.Data.MapObj[0].MapWidth)
            x2 = x2 - self.game.Data.MapObj[0].MapWidth - 1;
          let mut num6: i32 =  y1 - 5;
          let mut num7: i32 =  y1 + 5;
          for (let mut y2: i32 =  num6; y2 <= num7; y2 += 1)
          {
            if (x2 > -1 & y2 > -1 && x2 <= self.game.Data.MapObj[0].MapWidth & y2 <= self.game.Data.MapObj[0].MapHeight)
            {
              if (self.TPlanObj[aiPlanNr].Type == 20)
              {
                if (self.HexSA[x2, y2] == self.GetAreaNr(self.TPlanObj[aiPlanNr].TooArea))
                {
                  if (self.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter > -1)
                    num2 += self.GetHexForceLandStrength(x2, y2, true);
                  if (self.TPlanObj[aiPlanNr].Stand == 1)
                  {
                    num3 += 1;
                    int[] numArray2 = numArray1;
                    int[] numArray3 = numArray2;
                    let mut landscapeType: i32 =  self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType;
                    let mut index4: i32 =  landscapeType;
                    let mut num8: i32 =  numArray2[landscapeType] + 1;
                    numArray3[index4] = num8;
                  }
                }
                if (self.HexSA[x2, y2] == self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea))
                {
                  if (self.TPlanObj[aiPlanNr].Stand == 2)
                  {
                    num3 += 1;
                    int[] numArray4 = numArray1;
                    int[] numArray5 = numArray4;
                    let mut landscapeType: i32 =  self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType;
                    let mut index5: i32 =  landscapeType;
                    let mut num9: i32 =  numArray4[landscapeType] + 1;
                    numArray5[index5] = num9;
                  }
                  if (self.TPlanObj[aiPlanNr].Stand == 3)
                  {
                    num3 += 1;
                    int[] numArray6 = numArray1;
                    int[] numArray7 = numArray6;
                    let mut landscapeType: i32 =  self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType;
                    let mut index6: i32 =  landscapeType;
                    let mut num10: i32 =  numArray6[landscapeType] + 1;
                    numArray7[index6] = num10;
                  }
                }
              }
              if (self.TPlanObj[aiPlanNr].Type == 40)
              {
                if (self.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter > -1)
                  num2 += self.GetHexForceLandStrength(x2, y2, true);
                num3 += 1;
                int[] numArray8 = numArray1;
                int[] numArray9 = numArray8;
                let mut landscapeType: i32 =  self.game.Data.MapObj[0].HexObj[x2, y2].LandscapeType;
                let mut index7: i32 =  landscapeType;
                let mut num11: i32 =  numArray8[landscapeType] + 1;
                numArray9[index7] = num11;
              }
            }
          }
        }
        float num12 = 0.0f;
        let mut landscapeTypeCounter2: i32 =  self.game.Data.LandscapeTypeCounter;
        for (let mut index8: i32 =  0; index8 <= landscapeTypeCounter2; index8 += 1)
        {
          if (numArray1[index8] > 0)
          {
            if (self.TPlanObj[aiPlanNr].Stand == 2)
              num12 = num12 +  ( self.game.Data.LandscapeTypeObj[index8].DefBonus[self.game.Data.SFTypeObj[simpleList1.Id[index1]].UnitGroup] / 100.0 * ( numArray1[index8] /  num3)) + self.game.Data.SFTypeObj[simpleList1.Id[index1]].CombatModDef[index8] * ( numArray1[index8] /  num3);
            else if (self.TPlanObj[aiPlanNr].Stand == 1 | self.TPlanObj[aiPlanNr].Type == 40)
              num12 += self.game.Data.SFTypeObj[simpleList1.Id[index1]].CombatModAtt[index8] * ( numArray1[index8] /  num3);
          }
        }
        if (num2 > 0)
        {
          SimpleList simpleList2 = SimpleList::new();
          let mut tid: i32 =  0;
          let mut num13: i32 =  x1 - rangy;
          let mut num14: i32 =  x1 + rangy;
          for (let mut index9: i32 =  num13; index9 <= num14; index9 += 1)
          {
            let mut attackx: i32 =  index9;
            if (self.game.Data.MapObj[0].MapLoop & attackx < 0)
              attackx = self.game.Data.MapObj[0].MapWidth + attackx + 1;
            if (self.game.Data.MapObj[0].MapLoop & attackx > self.game.Data.MapObj[0].MapWidth)
              attackx = attackx - self.game.Data.MapObj[0].MapWidth - 1;
            let mut num15: i32 =  y1 - rangy;
            let mut num16: i32 =  y1 + rangy;
            float a;
            for (let mut attacky: i32 =  num15; attacky <= num16; attacky += 1)
            {
              if (attackx > -1 & attacky > -1 && attackx <= self.game.Data.MapObj[0].MapWidth & attacky <= self.game.Data.MapObj[0].MapHeight && !(attackx == x1 & attacky == y1))
              {
                if (self.TPlanObj[aiPlanNr].Type == 20)
                {
                  if (self.HexSA[attackx, attacky] == self.GetAreaNr(self.TPlanObj[aiPlanNr].TooArea) && self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitCounter > -1)
                  {
                    if (self.TPlanObj[aiPlanNr].Stand == 2)
                      a = self.AverageCombatPerform(-1, simpleList1.Id[index1], attackx, attacky, true) + self.AverageCombatPerform(-1, simpleList1.Id[index1], attackx, attacky) / 2f;
                    else if (self.TPlanObj[aiPlanNr].Stand == 1)
                      a = self.AverageCombatPerform(-1, simpleList1.Id[index1], attackx, attacky) + self.AverageCombatPerform(-1, simpleList1.Id[index1], attackx, attacky, true) / 2f;
                    if ( a > 10.0)
                      a = 10f;
                    if ( a < 0.1)
                      a = 0.1f;
                    tid += 1;
                    simpleList2.Add(tid, (int) Math.Round( (a * 10f)));
                  }
                }
                else if (self.TPlanObj[aiPlanNr].Type == 40 && -(-1 < self.AreaDistance(self.HexSA[attackx, attacky], self.GetAreaNr(self.TPlanObj[aiPlanNr].FromArea)) ? 1 : 0) <= 1 && self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitCounter > -1)
                {
                  a = self.AverageCombatPerform(-1, simpleList1.Id[index1], attackx, attacky) + self.AverageCombatPerform(-1, simpleList1.Id[index1], attackx, attacky, true) / 2f;
                  if ( a > 10.0)
                    a = 10f;
                  if ( a < 0.1)
                    a = 0.1f;
                  tid += 1;
                  simpleList2.Add(tid, (int) Math.Round( a));
                }
              }
            }
          }
          if (simpleList2.Counter > -1)
          {
            simpleList2.Sort();
            num1 = simpleList2.Counter <= 0 ? (int) Math.Round( num1 * ( (simpleList2.Weight[0] + simpleList2.Weight[simpleList2.Counter]) / 2.0)) : (int) Math.Round( num1 * ( (simpleList2.Weight[(int) Math.Round( (Conversion.Int(VBMath.Rnd() *  simpleList2.Counter) + 1f))] + simpleList2.Weight[(int) Math.Round( (Conversion.Int(VBMath.Rnd() *  simpleList2.Counter) + 1f))] + simpleList2.Weight[0] + simpleList2.Weight[simpleList2.Counter]) / 4.0));
          }
        }
        else
          num1 = simpleList1.Weight[index1];
        let mut num17: i32 =  (int) Math.Round( ( num1 * num12));
        if (randomeffect)
          num17 = (int) Math.Round( num17 * 0.5 +  num17 *  VBMath.Rnd());
        simpleList1.Weight[index1] = num17;
      }
      simpleList1.Sort();
      return simpleList1.Data1[simpleList1.Counter];
    }

    pub fn MakeCombatMatrix()
    {
      bool flag = false;
      if (flag)
      {
        self.LogCounter = -1;
        self.AddLog("COMBATMATRIX");
      }
      self.CombatMatrix = new float[self.game.Data.SFTypeCounter + 1, self.game.Data.SFTypeCounter + 1];
      let mut sfTypeCounter1: i32 =  self.game.Data.SFTypeCounter;
      for (let mut index1: i32 =  0; index1 <= sfTypeCounter1; index1 += 1)
      {
        if (flag)
          self.AddLog("********* " + self.game.Data.SFTypeObj[index1].Name + " VERSUS: ");
        let mut sfTypeCounter2: i32 =  self.game.Data.SFTypeCounter;
        for (let mut index2: i32 =  0; index2 <= sfTypeCounter2; index2 += 1)
        {
          let mut num1: i32 =  self.game.Data.SFTypeObj[index1].AttackPower[self.game.Data.SFTypeObj[index2].UnitGroup] * self.game.Data.SFTypeObj[index1].Attacks;
          let mut num2: i32 =  self.game.Data.SFTypeObj[index2].AttackPowerDef[self.game.Data.SFTypeObj[index1].UnitGroup] * self.game.Data.SFTypeObj[index2].Attacks;
          let mut num3: i32 =  self.game.Data.SFTypeObj[index1].HitPoints[self.game.Data.SFTypeObj[index2].UnitGroup];
          let mut num4: i32 =  self.game.Data.SFTypeObj[index2].HitPointsDef[self.game.Data.SFTypeObj[index1].UnitGroup];
          if (self.game.Data.SFTypeObj[index2].BackBench & self.game.Data.SFTypeObj[index1].Theater == 0)
            num4 *= 2;
          if (self.game.Data.SFTypeObj[index1].BackBench & self.game.Data.SFTypeObj[index2].Theater == 0)
            num3 *= 4;
          let mut num5: i32 =  self.game.Data.SFTypeObj[index1].PowerPts;
          let mut num6: i32 =  self.game.Data.SFTypeObj[index2].PowerPts;
          if (num6 == 0)
            num6 = 1;
          if (num5 == 0)
            num5 = 1;
          float num7 = 1f;
          float num8 = 1f;
          if (num5 > num6)
          {
            num2 = (int) Math.Round( num2 * ( num5 /  num6));
            num4 = (int) Math.Round( num4 * ( num5 /  num6));
            num8 *=  num5 /  num6;
          }
          else if (num6 > num5)
          {
            num1 = (int) Math.Round( num1 * ( num6 /  num5));
            num3 = (int) Math.Round( num3 * ( num6 /  num5));
            num7 *=  num6 /  num5;
          }
          if ( num7 *  self.game.Data.SFTypeObj[index1].Attacks >  num8 *  self.game.Data.SFTypeObj[index2].MaxAttacked)
            num1 = (int) Math.Round( ( num1 *  ( num8 *  self.game.Data.SFTypeObj[index2].MaxAttacked / ( num7 *  self.game.Data.SFTypeObj[index1].Attacks))));
          if ( num8 *  self.game.Data.SFTypeObj[index2].Attacks >  num7 *  self.game.Data.SFTypeObj[index1].MaxAttacked)
            num2 = (int) Math.Round( ( num2 *  ( num7 *  self.game.Data.SFTypeObj[index1].MaxAttacked / ( num8 *  self.game.Data.SFTypeObj[index2].Attacks))));
          float num9 =  num1 /  num4;
          float num10 =  num2 /  num3;
          float Number =  num10 <= 0.0 ? 10f : num9 / num10;
          if ( Number > 5.0)
            Number =  (5.0 + Math.Sqrt( Number - 4.0));
          if ( Number < 0.1)
            Number = 0.1f;
          self.CombatMatrix[index1, index2] = Number;
          if (self.game.Data.SFTypeObj[index1].Theater == self.game.Data.SFTypeObj[index2].Theater |  num9 > 0.0 && flag)
            self.AddLog(self.game.Data.SFTypeObj[index2].Name + " = " + Conversion.Str( Number));
        }
      }
    }

    pub float AverageCombatPerform(
      sfnrxxx: i32,
      typ: i32,
      attackx: i32,
      attacky: i32,
      bool defend = false,
      bool onlysametheater = false)
    {
      int[] numArray1 = new int[self.game.Data.SFTypeCounter + 1];
      float[] numArray2 = new float[self.game.Data.SFTypeCounter + 1];
      float[] numArray3 = new float[self.game.Data.SFTypeCounter + 1];
      if (self.CombatMatrix.GetUpperBound(0) == self.game.Data.SFTypeCounter)
      {
        let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitCounter;
        num1: i32;
        num2: i32;
        for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
        {
          let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitList[index1];
          let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount;
          for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[index2];
            let mut num3: i32 =  self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].PowerPts;
            if (!onlysametheater | self.game.Data.SFTypeObj[typ].Theater == self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater)
            {
              float num4 = !defend ? self.CombatMatrix[typ, self.game.Data.SFObj[sf].Type] * self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].CombatModAtt[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType] : self.CombatMatrix[self.game.Data.SFObj[sf].Type, typ] * self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].CombatModDef[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType];
              num1 = (int) Math.Round( ( num1 + num4 *  num3));
              num2 += num3;
            }
          }
        }
        return num2 <= 0 ? 1f :  num1 /  num2;
      }
      float num5;
      if (!defend)
      {
        let mut sfTypeCounter1: i32 =  self.game.Data.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter1; index += 1)
        {
          float num6 =  self.game.Data.SFTypeObj[typ].AttackPower[self.game.Data.SFTypeObj[index].UnitGroup] * self.game.Data.SFTypeObj[typ].CombatModAtt[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType] *  self.game.Data.SFTypeObj[typ].Attacks;
          float num7 =  Conversion.Int( self.game.Data.SFTypeObj[typ].KillPercent / 100.0 * 10.0 * 100.0 * ( num6 /  self.game.Data.SFTypeObj[index].DefPower));
          if ( num7 > 9999.0)
            num7 = 9999f;
          float num8 = self.game.Data.SFTypeObj[typ].PowerPts <= 0 ? num7 : num7 * ( self.game.Data.SFTypeObj[index].PowerPts /  self.game.Data.SFTypeObj[typ].PowerPts);
          numArray2[index] = num8;
          float num9 =  self.game.Data.SFTypeObj[index].AttackPowerDef[self.game.Data.SFTypeObj[typ].UnitGroup] * self.game.Data.SFTypeObj[typ].CombatModDef[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType] *  self.game.Data.SFTypeObj[index].Attacks;
          float num10 =  Conversion.Int( self.game.Data.SFTypeObj[index].KillPercent / 100.0 * 10.0 * 100.0 * ( num9 /  self.game.Data.SFTypeObj[typ].DefPower));
          if (self.game.Data.SFTypeObj[typ].BackBench)
            num10 /= self.game.Data.RuleVar[222];
          if (self.game.Data.SFTypeObj[typ].ArtRange > 1)
            num10 /= 3f;
          if ( num10 > 9999.0)
            num10 = 9999f;
          numArray3[index] = num10;
        }
        let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitCounter;
        num11: i32;
        for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
        {
          let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitList[index3];
          let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount;
          for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[index4];
            let mut num12: i32 =  self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].PowerPts;
            if (self.game.Data.SFTypeObj[typ].Theater == self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater)
              num12 *= 10;
            if (!onlysametheater | self.game.Data.SFTypeObj[typ].Theater == self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater)
            {
              int[] numArray4 = numArray1;
              int[] numArray5 = numArray4;
              let mut type: i32 =  self.game.Data.SFObj[sf].Type;
              let mut index5: i32 =  type;
              let mut num13: i32 =  numArray4[type] + num12;
              numArray5[index5] = num13;
              num11 += num12;
            }
          }
        }
        let mut sfTypeCounter2: i32 =  self.game.Data.SFTypeCounter;
        float num14;
        float num15;
        for (let mut index: i32 =  0; index <= sfTypeCounter2; index += 1)
        {
          if (numArray1[index] > 0)
          {
            num14 += numArray2[index] * ( numArray1[index] /  num11);
            num15 += numArray3[index] * ( numArray1[index] /  num11);
          }
        }
        if ( num14 == 0.0)
          num14 = 1f;
        if ( num15 == 0.0)
          num15 = 1f;
        num5 = num14 / num15;
      }
      else
      {
        let mut sfTypeCounter3: i32 =  self.game.Data.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter3; index += 1)
        {
          float num16 =  self.game.Data.SFTypeObj[typ].AttackPowerDef[self.game.Data.SFTypeObj[index].UnitGroup] * self.game.Data.SFTypeObj[typ].CombatModDef[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType] *  self.game.Data.SFTypeObj[typ].Attacks;
          float num17 =  Conversion.Int( self.game.Data.SFTypeObj[typ].KillPercent / 100.0 * 10.0 * 100.0 * ( num16 /  self.game.Data.SFTypeObj[index].DefPower));
          if ( num17 > 9999.0)
            num17 = 9999f;
          if (self.game.Data.SFTypeObj[typ].PowerPts > 0)
            num17 *=  self.game.Data.SFTypeObj[index].PowerPts /  self.game.Data.SFTypeObj[typ].PowerPts;
          numArray2[index] = num17;
          float num18 =  self.game.Data.SFTypeObj[index].AttackPower[self.game.Data.SFTypeObj[typ].UnitGroup] * self.game.Data.SFTypeObj[typ].CombatModAtt[self.game.Data.MapObj[0].HexObj[attackx, attacky].LandscapeType] *  self.game.Data.SFTypeObj[index].Attacks;
          float num19 =  Conversion.Int( self.game.Data.SFTypeObj[index].KillPercent / 100.0 * 10.0 * 100.0 * ( num18 /  self.game.Data.SFTypeObj[typ].DefPower));
          if (self.game.Data.SFTypeObj[typ].BackBench)
            num19 /= self.game.Data.RuleVar[222];
          if (self.game.Data.SFTypeObj[typ].ArtRange > 1)
            num19 /= 3f;
          if ( num19 > 9999.0)
            num19 = 9999f;
          numArray3[index] = num19;
        }
        let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitCounter;
        num20: i32;
        for (let mut index6: i32 =  0; index6 <= unitCounter; index6 += 1)
        {
          let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[attackx, attacky].UnitList[index6];
          let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount;
          for (let mut index7: i32 =  0; index7 <= sfCount; index7 += 1)
          {
            let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[index7];
            let mut num21: i32 =  self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].PowerPts;
            if (self.game.Data.SFTypeObj[typ].Theater == self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater)
              num21 *= 10;
            if (!onlysametheater | self.game.Data.SFTypeObj[typ].Theater == self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater)
            {
              int[] numArray6 = numArray1;
              int[] numArray7 = numArray6;
              let mut type: i32 =  self.game.Data.SFObj[sf].Type;
              let mut index8: i32 =  type;
              let mut num22: i32 =  numArray6[type] + num21;
              numArray7[index8] = num22;
              num20 += num21;
            }
          }
        }
        let mut sfTypeCounter4: i32 =  self.game.Data.SFTypeCounter;
        float num23;
        float num24;
        for (let mut index: i32 =  0; index <= sfTypeCounter4; index += 1)
        {
          if (numArray1[index] > 0)
          {
            num23 += numArray2[index] * ( numArray1[index] /  num20);
            num24 += numArray3[index] * ( numArray1[index] /  num20);
          }
        }
        if ( num23 == 0.0)
          num23 = 1f;
        if ( num24 == 0.0)
          num24 = 1f;
        num5 = num23 / num24;
      }
      return num5;
    }

    pub GetHexForceLandStrength: i32(
      x: i32,
      y: i32,
      bool withoutmods = false,
      bool asattack = false,
      let mut attackx: i32 =  -1,
      let mut attacky: i32 =  -1)
    {
      let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
      forceLandStrength: i32;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
        forceLandStrength += self.GetForceLandStrength(self.game.Data.MapObj[0].HexObj[x, y].UnitList[index], withoutmods, asattack, attackx, attacky);
      return forceLandStrength;
    }

    pub GetHexForceAirStrength: i32(
      x: i32,
      y: i32,
      bool withoutmods = false,
      bool asattack = false,
      let mut attackx: i32 =  -1,
      let mut attacky: i32 =  -1)
    {
      let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
      forceAirStrength: i32;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
        forceAirStrength += self.GetForceAirStrength(self.game.Data.MapObj[0].HexObj[x, y].UnitList[index], withoutmods, asattack, attackx, attacky);
      return forceAirStrength;
    }

    pub fn GetClosestFrontline(x: i32, y: i32) -> i32
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (x == -1)
        return 0;
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          numArray[index1, index2] = 0;
      }
      let mut closestFrontline: i32 =  0;
      let mut num1: i32 =  9999;
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x2: i32 =  0; x2 <= mapWidth2; x2 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y2: i32 =  0; y2 <= mapHeight; y2 += 1)
        {
          if (self.HexPlan[x2, y2] > 0 & self.HexOA[x, y] == self.HexOA[x2, y2] && self.TPlanObj[self.HexPlan[x2, y2]].Type == 20)
          {
            let mut num2: i32 =  self.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0);
            if (num1 > num2)
            {
              num1 = num2;
              closestFrontline = self.HexPlan[x2, y2];
            }
          }
        }
      }
      return closestFrontline;
    }

    pub fn GetClosestBackPlan(x: i32, y: i32) -> i32
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      if (x == -1)
        return 0;
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          numArray[index1, index2] = 0;
      }
      let mut closestBackPlan: i32 =  0;
      let mut num1: i32 =  9999;
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x2: i32 =  0; x2 <= mapWidth2; x2 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y2: i32 =  0; y2 <= mapHeight; y2 += 1)
        {
          if (self.HexBackPlan[x2, y2] > 0)
          {
            let mut num2: i32 =  self.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0);
            if (num1 > num2)
            {
              num1 = num2;
              closestBackPlan = self.HexBackPlan[x2, y2];
            }
          }
        }
      }
      return closestBackPlan;
    }

    pub fn GetClosestFrontlineDistance2(x: i32, y: i32) -> i32
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          numArray[index1, index2] = 0;
      }
      let mut num1: i32 =  0;
      let mut frontlineDistance2: i32 =  9999;
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x2: i32 =  0; x2 <= mapWidth2; x2 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y2: i32 =  0; y2 <= mapHeight; y2 += 1)
        {
          if (self.HexPlan[x2, y2] > 0 && self.TPlanObj[self.HexPlan[x2, y2]].Type == 20)
          {
            let mut num2: i32 =  self.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0);
            if (frontlineDistance2 > num2)
            {
              frontlineDistance2 = num2;
              num1 = self.HexPlan[x2, y2];
            }
          }
        }
      }
      return frontlineDistance2;
    }

    pub fn GetClosestEnemyDistance(x: i32, y: i32, bool enemyunit = false) -> i32
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          numArray[index1, index2] = 0;
      }
      let mut num1: i32 =  0;
      let mut closestEnemyDistance: i32 =  9999;
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x2: i32 =  0; x2 <= mapWidth2; x2 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y2: i32 =  0; y2 <= mapHeight; y2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[x2, y2].Regime > -1 && self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[x2, y2].Regime))
          {
            let mut num2: i32 =  self.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0);
            if (enemyunit && self.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter < 0)
              num2 = 99999;
            if (closestEnemyDistance > num2)
            {
              closestEnemyDistance = num2;
              num1 = self.HexPlan[x2, y2];
            }
          }
        }
      }
      return closestEnemyDistance;
    }

    pub fn GetClosestFrontlineDistance(sanr: i32, x: i32, y: i32, bool withunit = false) -> i32
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          numArray[index1, index2] = 0;
      }
      let mut num1: i32 =  0;
      let mut frontlineDistance: i32 =  9999;
      let mut mapWidth2: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut x2: i32 =  0; x2 <= mapWidth2; x2 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut y2: i32 =  0; y2 <= mapHeight; y2 += 1)
        {
          if (self.HexSA[x2, y2] == sanr)
          {
            let mut num2: i32 =  1;
            if (self.game.Data.MapObj[0].HexObj[x2, y2].UnitCounter < 0 & withunit)
              num2 = 0;
            if (num2 == 1)
            {
              let mut num3: i32 =  self.game.HandyFunctionsObj.Distance(x, y, 0, x2, y2, 0);
              if (frontlineDistance > num3)
              {
                frontlineDistance = num3;
                num1 = self.HexSA[x2, y2];
              }
            }
          }
        }
      }
      return frontlineDistance;
    }

    pub fn GetSANr(SAClass TempArea) -> i32
    {
      let mut saCount: i32 =  self.SACount;
      for (let mut saNr: i32 =  1; saNr <= saCount; saNr += 1)
      {
        if (self.SAObj[saNr].X == TempArea.X & self.SAObj[saNr].Y == TempArea.Y)
          return saNr;
      }
      return 0;
    }

    pub fn AverageFuzzyVP() -> i32
    {
      let mut saCount: i32 =  self.SACount;
      num: i32;
      for (let mut index: i32 =  1; index <= saCount; index += 1)
        num += self.SAObj[index].fuzzyvp;
      return (int) Math.Round(Conversion.Int( num /  self.SACount));
    }

    pub fn GetFriendlyAreaNeighbours(areanr: i32, bool withoutenemies) -> i32
    {
      let mut index1: i32 =  areanr;
      let mut neighbourCount: i32 =  self.SAObj[index1].NeighbourCount;
      friendlyAreaNeighbours: i32;
      for (let mut index2: i32 =  1; index2 <= neighbourCount; index2 += 1)
      {
        let mut areanr1: i32 =  self.SAObj[index1].Neighbour[index2];
        if (!self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.MapObj[0].HexObj[self.SAObj[index1].X, self.SAObj[index1].Y].Regime, self.game.Data.MapObj[0].HexObj[self.SAObj[areanr1].X, self.SAObj[areanr1].Y].Regime))
        {
          let mut num: i32 =  1;
          if (withoutenemies && self.GetFriendlyAreaNeighbours(areanr1, false) < self.SAObj[areanr1].NeighbourCount)
            num = 0;
          if (num == 1)
            friendlyAreaNeighbours += 1;
        }
      }
      return friendlyAreaNeighbours;
    }

    pub fn GetBestNeighbourForRetreater(areanr: i32) -> i32
    {
      let mut neighbourCount: i32 =  self.SAObj[areanr].NeighbourCount;
      for (let mut index: i32 =  1; index <= neighbourCount; index += 1)
      {
        let mut areanr1: i32 =  self.SAObj[areanr].Neighbour[index];
        if (!self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.MapObj[0].HexObj[self.SAObj[areanr].X, self.SAObj[areanr].Y].Regime, self.game.Data.MapObj[0].HexObj[self.SAObj[areanr1].X, self.SAObj[areanr1].Y].Regime))
        {
          if (self.GetFriendlyAreaNeighbours(areanr1, false) == self.SAObj[areanr1].NeighbourCount)
            return areanr1;
        }
      }
      return -1;
    }

    pub fn AreaDistance(nr: i32, nr2: i32, bool onlyfriendly = false, let mut MaxDistance: i32 =  999) -> i32
    {
      int[] numArray = new int[self.SACount + 1];
      let mut saCount1: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount1; index += 1)
        numArray[index] = -1;
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      numArray[nr] = 0;
      while (num1 == 1 & num2 < MaxDistance)
      {
        num1 = 0;
        num2 += 1;
        let mut saCount2: i32 =  self.SACount;
        for (let mut index: i32 =  1; index <= saCount2; index += 1)
        {
          if (numArray[index] == num2 - 1)
          {
            let mut saCount3: i32 =  self.SACount;
            for (let mut nr1: i32 =  1; nr1 <= saCount3; nr1 += 1)
            {
              if (self.SAObj[index].IsNeighbour(nr1) && !onlyfriendly | self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[self.SAObj[nr1].X, self.SAObj[nr1].Y].Regime) && numArray[nr1] == -1)
              {
                numArray[nr1] = num2;
                if (nr1 == nr2)
                  return num2;
                num1 = 1;
              }
            }
          }
        }
      }
      return numArray[nr2];
    }

    pub fn AreaDistance2(nr: i32, nr2: i32, bool onlyfriendly = false, let mut MaxDistance: i32 =  999) -> i32
    {
      int[] numArray = new int[self.SACount + 1];
      let mut saCount1: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount1; index += 1)
        numArray[index] = -1;
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      numArray[nr] = 0;
      while (num1 == 1 & num2 < MaxDistance)
      {
        num1 = 0;
        num2 += 1;
        let mut saCount2: i32 =  self.SACount;
        for (let mut index1: i32 =  1; index1 <= saCount2; index1 += 1)
        {
          if (numArray[index1] == num2 - 1)
          {
            let mut neighbourCount: i32 =  self.SAObj[index1].NeighbourCount;
            for (let mut index2: i32 =  1; index2 <= neighbourCount; index2 += 1)
            {
              let mut index3: i32 =  self.SAObj[index1].Neighbour[index2];
              if (index3 > 0 && !onlyfriendly | self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.MapObj[0].HexObj[self.SAObj[index3].X, self.SAObj[index3].Y].Regime) && numArray[index3] == -1)
              {
                numArray[index3] = num2;
                if (index3 == nr2)
                  return num2;
                num1 = 1;
              }
            }
          }
        }
      }
      return numArray[nr2];
    }

    pub fn AreaDistanceIncludingSea(nr: i32, nr2: i32) -> i32
    {
      int[] numArray = new int[self.SACount + 1];
      let mut saCount1: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount1; index += 1)
        numArray[index] = -1;
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      numArray[nr] = 0;
      while (num1 == 1)
      {
        num1 = 0;
        num2 += 1;
        let mut saCount2: i32 =  self.SACount;
        for (let mut index: i32 =  1; index <= saCount2; index += 1)
        {
          if (numArray[index] == num2 - 1)
          {
            let mut saCount3: i32 =  self.SACount;
            for (let mut nr1: i32 =  1; nr1 <= saCount3; nr1 += 1)
            {
              if (self.SAObj[index].IsNeighbour(nr1) | self.SAObj[index].IsSeaNeighbour(nr1) && numArray[nr1] == -1)
              {
                numArray[nr1] = num2;
                if (nr1 == nr2)
                  return num2;
                num1 = 1;
              }
            }
          }
        }
      }
      return numArray[nr2];
    }

    pub fn AreaDistanceOnlySea(nr: i32, nr2: i32) -> i32
    {
      int[] numArray = new int[self.SACount + 1];
      let mut saCount1: i32 =  self.SACount;
      for (let mut index: i32 =  1; index <= saCount1; index += 1)
        numArray[index] = -1;
      let mut num1: i32 =  1;
      let mut num2: i32 =  0;
      numArray[nr] = 0;
      while (num1 == 1)
      {
        num1 = 0;
        num2 += 1;
        let mut saCount2: i32 =  self.SACount;
        for (let mut index: i32 =  1; index <= saCount2; index += 1)
        {
          if (numArray[index] == num2 - 1)
          {
            let mut saCount3: i32 =  self.SACount;
            for (let mut nr1: i32 =  1; nr1 <= saCount3; nr1 += 1)
            {
              if (self.SAObj[index].IsSeaNeighbour(nr1) && numArray[nr1] == -1)
              {
                numArray[nr1] = num2;
                if (nr1 == nr2)
                  return num2;
                num1 = 1;
              }
            }
          }
        }
      }
      return numArray[nr2];
    }

    pub fn AddtPlan()
    {
      this += 1.TPlanCount;
      self.TPlanObj = (AIPlanClass[]) Utils.CopyArray((Array) self.TPlanObj, (Array) new AIPlanClass[self.TPlanCount + 1]);
      self.TPlanObj[self.TPlanCount] = AIPlanClass::new();
    }

    pub fn Removetplan(nr: i32)
    {
      if (nr < self.TPlanCount)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.TPlanCount - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          self.TPlanObj[index] = self.TPlanObj[index + 1];
      }
      --self.TPlanCount;
      self.TPlanObj = (AIPlanClass[]) Utils.CopyArray((Array) self.TPlanObj, (Array) new AIPlanClass[self.TPlanCount + 1]);
    }

    pub fn getclosestplan(x: i32, y: i32, plantype: i32) -> i32
    {
      num: i32;
      for (; num < 99; num += 1)
      {
        let mut nr: i32 =  self.HexSA[x, y];
        let mut tplanCount1: i32 =  self.TPlanCount;
        for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
        {
          if (self.TPlanObj[index].Type == plantype && self.GetAreaNr(self.TPlanObj[index].FromArea) == nr)
            return index;
        }
        let mut saCount: i32 =  self.SACount;
        for (let mut nr2: i32 =  0; nr2 <= saCount; nr2 += 1)
        {
          if (self.AreaDistance(nr, nr2) == num)
          {
            let mut tplanCount2: i32 =  self.TPlanCount;
            for (let mut index: i32 =  1; index <= tplanCount2; index += 1)
            {
              if (self.TPlanObj[index].Type == plantype && self.GetAreaNr(self.TPlanObj[index].FromArea) == nr2)
                return index;
            }
          }
        }
      }
      return -1;
    }

    pub fn getclosestplansea(x: i32, y: i32, plantype: i32) -> i32
    {
      num: i32;
      for (; num < 99; num += 1)
      {
        let mut nr: i32 =  self.HexSA[x, y];
        let mut tplanCount1: i32 =  self.TPlanCount;
        for (let mut index: i32 =  1; index <= tplanCount1; index += 1)
        {
          if (self.TPlanObj[index].Type == plantype && self.GetAreaNr(self.TPlanObj[index].FromArea) == nr)
            return index;
        }
        let mut saCount: i32 =  self.SACount;
        for (let mut nr2: i32 =  0; nr2 <= saCount; nr2 += 1)
        {
          if (self.AreaDistanceOnlySea(nr, nr2) == num)
          {
            let mut tplanCount2: i32 =  self.TPlanCount;
            for (let mut index: i32 =  1; index <= tplanCount2; index += 1)
            {
              if (self.TPlanObj[index].Type == plantype && self.GetAreaNr(self.TPlanObj[index].FromArea) == nr2)
                return index;
            }
          }
        }
      }
      return -1;
    }

    pub fn getfrontplan(x: i32, y: i32) -> i32
    {
      let mut num1: i32 =  20;
      let mut num2: i32 =  self.HexSA[x, y];
      let mut tplanCount: i32 =  self.TPlanCount;
      for (let mut index: i32 =  1; index <= tplanCount; index += 1)
      {
        if (self.TPlanObj[index].Type == num1 && self.GetAreaNr(self.TPlanObj[index].FromArea) == num2)
          return index;
      }
      return -1;
    }

    pub fn GetRealForceInArea(areanr: i32, plannr: i32, bool withoutmods) -> i32
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      realForceInArea: i32;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSA[index1, index2] == areanr)
          {
            let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
            for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
            {
              let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3];
              if (self.game.Data.UnitObj[unit].AIPlanNr == plannr)
                realForceInArea += self.GetForceLandStrength(unit, withoutmods);
            }
          }
        }
      }
      return realForceInArea;
    }

    pub fn GetRealForceInArea2(areanr: i32, bool withoutmods) -> i32
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      realForceInArea2: i32;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSA[index1, index2] == areanr)
          {
            let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
            for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
            {
              let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3];
              realForceInArea2 += self.GetForceLandStrength(unit, withoutmods);
            }
          }
        }
      }
      return realForceInArea2;
    }

    pub fn GetRealNavalForceInArea(seaareanr: i32, plannr: i32, bool withoutmods, bool friendly) -> i32
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      navalForceInArea: i32;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSeaSA[index1, index2] == seaareanr | seaareanr == -1)
          {
            let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
            for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
            {
              let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3];
              if (friendly)
              {
                if (self.game.Data.UnitObj[unit].Regime == self.game.Data.Turn && self.game.Data.UnitObj[unit].AIPlanNr == plannr)
                  navalForceInArea += self.GetForceNavalStrength(unit, withoutmods);
              }
              else if (self.game.HandyFunctionsObj.IsHostileNotSelf(self.game.Data.Turn, self.game.Data.UnitObj[unit].Regime))
                navalForceInArea += self.GetForceNavalStrength(unit, withoutmods);
            }
          }
        }
      }
      return navalForceInArea;
    }

    pub fn Screenshot(typ: i32, fileextension: String)
    {
      FileStream fileStream = new FileStream(self.game.AppPath + "logs/screenshot_typ" + Strings.Trim(Conversion.Str( typ)) + "_pl" + Strings.Trim(Conversion.Str( self.game.Data.Turn)) + fileextension + ".bmp", FileMode.Create);
      bitmap: Bitmap = new Bitmap(self.game.Data.MapObj[0].MapWidth * 40 + 80, self.game.Data.MapObj[0].MapHeight * 32 + 68, PixelFormat.Format24bppRgb);
      bitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics = Graphics.FromImage((Image) bitmap);
      if (typ == 1)
      {
        DrawMod.DrawText(ref graphics, "SA", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot1(ref graphics);
      }
      if (typ == 2)
      {
        DrawMod.DrawText(ref graphics, "Plan Frontlines", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot2(ref graphics);
      }
      if (typ == 3)
      {
        DrawMod.DrawText(ref graphics, "Matrix1", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot3(ref graphics);
      }
      if (typ == 4)
      {
        DrawMod.DrawText(ref graphics, "Matrix2", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot4(ref graphics);
      }
      if (typ == 5)
      {
        DrawMod.DrawText(ref graphics, "Continents", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot5(ref graphics);
      }
      if (typ == 6)
      {
        DrawMod.DrawText(ref graphics, "Operational Areas", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot6(ref graphics);
      }
      if (typ == 7)
      {
        DrawMod.DrawText(ref graphics, "Sea SA", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, bitmap.Height - 15);
        self.Screenshotgrid(ref graphics);
        self.Screenshot7(ref graphics);
      }
      graphics.Dispose();
      bitmap.Save((Stream) fileStream, ImageFormat.Bmp);
      fileStream.Close();
    }

    pub fn Screenshotgrid(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          customBitmapObj: CustomBitmapClass = self.game.CustomBitmapObj;
          let mut cx: i32 =  index1;
          let mut cy: i32 =  index2;
          bitmap: Bitmap = (Bitmap) null;
          ref local1: Bitmap = ref bitmap;
          bool flag = false;
          ref bool local2 = ref flag;
          objBitmap: Bitmap = customBitmapObj.DrawHex(cx, cy, 0, gBitmap: (ref local1), tFromMapPopup: (ref local2));
          if (index1 == 0 | index1 % 2 == 0)
          {
            DrawMod.DrawScaled(ref g, ref objBitmap, index1 * 40, index2 * 32, 40, 32);
            DrawMod.DrawRectangle(ref g, index1 * 40, index2 * 32, 40, 32, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
          }
          else
          {
            DrawMod.DrawScaled(ref g, ref objBitmap, index1 * 40, index2 * 32 + 16, 40, 32);
            DrawMod.DrawRectangle(ref g, index1 * 40, index2 * 32 + 16, 40, 32, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
          }
        }
      }
    }

    pub fn Screenshot5(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexContinent[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexContinent[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
            else
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexContinent[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
          }
        }
      }
    }

    pub fn Screenshot6(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexOA[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexOA[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
            else
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexOA[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
          }
        }
      }
    }

    pub fn Screenshot7(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSeaSA[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexSeaSA[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
            else
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexSeaSA[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
          }
        }
      }
    }

    pub fn Screenshot1(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexSA[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
            {
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexSA[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexSAWithoutTemp[index1, index2]), Font::new("Times New Roman", 11f, FontStyle.Italic, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 17);
            }
            else
            {
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexSA[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexSAWithoutTemp[index1, index2]), Font::new("Times New Roman", 11f, FontStyle.Italic, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 17 + 16);
            }
          }
        }
      }
    }

    pub fn Screenshot2(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.HexPlan[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexPlan[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
            else
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.HexPlan[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
          }
        }
      }
    }

    pub fn Screenshot3(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.Matrix1[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.Matrix1[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
            else
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.Matrix1[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
          }
        }
      }
    }

    pub fn Screenshot4(ref Graphics g)
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.Matrix2[index1, index2] > 0)
          {
            if (index1 == 0 | index1 % 2 == 0)
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.Matrix2[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2);
            else
              DrawMod.DrawTextOutline(ref g, Conversion.Str( self.Matrix2[index1, index2]), Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), index1 * 40 + 2, index2 * 32 + 2 + 16);
          }
        }
      }
    }

    pub fn CloseAI()
    {
      self.CratesCheck();
      self.WriteLog();
      self.WriteLog2();
    }

    pub fn CratesCheck()
    {
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].CardUponConquest > -1)
          {
            let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
            for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
            {
              if (self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3]].Regime == self.game.Data.Turn && self.game.Data.MapObj[0].HexObj[index1, index2].CardUponConquest > -1)
              {
                self.game.EditObj.AreaX = index1;
                self.game.EditObj.AreaY = index2;
                self.game.EditObj.AreaMap = 0;
                self.game.EditObj.DoCardSlot = self.game.Data.MapObj[0].HexObj[index1, index2].CardUponConquest;
                self.game.ProcessingObj.PlayCard(self.game.EditObj.DoCardSlot);
                let mut num: i32 =  0;
                let mut locCounter: i32 =  self.game.Data.LocCounter;
                for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
                {
                  if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime == self.game.Data.Turn)
                  {
                    let mut index4: i32 =  0;
                    do
                    {
                      if (self.game.Data.LocObj[locnr].Production[index4] > -1 && !self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, self.game.Data.LocObj[locnr].Production[index4]).result)
                      {
                        num += 1;
                        self.game.Data.LocObj[locnr].Production[index4] = -1;
                        self.game.Data.LocObj[locnr].ProdPointRemainder[index4] = 0;
                        self.game.Data.LocObj[locnr].ProdPercent[index4] = 0;
                      }
                      index4 += 1;
                    }
                    while (index4 <= 3);
                  }
                }
                self.game.EditObj.DoCardSlot = -1;
                self.game.Data.MapObj[0].HexObj[index1, index2].CardUponConquest = -1;
                break;
              }
            }
          }
        }
      }
    }

    pub fn AddLog2(s: String)
    {
      this += 1.LogCounter2;
      self.LogTxt2 = (string[]) Utils.CopyArray((Array) self.LogTxt2, (Array) new string[self.LogCounter2 + 1]);
      self.LogTxt2[self.LogCounter2] = s;
    }

    pub fn WriteLog2()
    {
      let mut num1: i32 =  self.game.HandyFunctionsObj.CheckDiskSpace(Strings.Left(self.game.AppPath, Strings.InStr(self.game.AppPath, ":")));
      if (num1 > 0 & num1 < 50)
      {
        let mut num2: i32 =  (int) Interaction.MsgBox( "Not of space left to write to disk.");
      }
      else
      {
        StreamWriter text = File.CreateText(self.game.AppPath + "logs/AItimer.txt");
        let mut logCounter2: i32 =  self.LogCounter2;
        for (let mut index: i32 =  0; index <= logCounter2; index += 1)
          text.WriteLine(self.LogTxt2[index]);
        text.Close();
      }
    }

    pub fn AddLog(s: String)
    {
      this += 1.LogCounter;
      self.LogTxt = (string[]) Utils.CopyArray((Array) self.LogTxt, (Array) new string[self.LogCounter + 1]);
      self.LogTxt[self.LogCounter] = s;
    }

    pub fn WriteLog()
    {
      let mut num1: i32 =  self.game.HandyFunctionsObj.CheckDiskSpace(Strings.Left(self.game.AppPath, Strings.InStr(self.game.AppPath, ":")));
      if (num1 > 0 & num1 < 50)
      {
        let mut num2: i32 =  (int) Interaction.MsgBox( "Not of space left to write to disk.");
      }
      else
      {
        StreamWriter text = File.CreateText(self.game.AppPath + "logs/AIlog_" + Conversion.Str( self.game.Data.Turn) + ".txt");
        let mut logCounter: i32 =  self.LogCounter;
        for (let mut index: i32 =  0; index <= logCounter; index += 1)
          text.WriteLine(self.LogTxt[index]);
        text.Close();
      }
    }

    pub object GetAAonHex(x: i32, y: i32, versusattacker: i32)
    {
      Coordinate target = Coordinate::new();
      let mut aaonHex: i32 =  0;
      let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (Math.Abs(index1 - x) < 5 & Math.Abs(index2 - y) < 5)
          {
            let mut unitCounter: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
            for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
            {
              let mut unit: i32 =  self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3];
              target.x = index1;
              target.y = index2;
              target.onmap = true;
              if (self.game.HandyFunctionsObj.CanUnitAA(unit, target, versusattacker))
              {
                let mut sfCount: i32 =  self.game.Data.UnitObj[unit].SFCount;
                for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
                {
                  let mut sf: i32 =  self.game.Data.UnitObj[unit].SFList[index4];
                  let mut type: i32 =  self.game.Data.SFObj[sf].Type;
                  num: i32;
                  if (self.game.Data.SFTypeObj[type].AIRoleScore[12] > 0)
                    num = (int) Math.Round( (self.game.Data.SFTypeObj[type].PowerPts * self.game.Data.SFObj[sf].Qty) * ( self.game.Data.SFTypeObj[type].AIRoleScore[12] / 100.0));
                  aaonHex += num;
                }
              }
            }
          }
        }
      }
      return  aaonHex;
    }

    pub fn GetMeRandomUnit() -> i32
    {
      SimpleList simpleList = SimpleList::new();
      let mut unitCounter: i32 =  self.game.Data.UnitCounter;
      for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
      {
        if (self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn && self.game.Data.UnitObj[tid].X > -1 & self.game.Data.UnitObj[tid].SupplyConsume >= 100 & self.game.Data.UnitObj[tid].PreDef == -1)
          simpleList.Add(tid, 1);
      }
      return simpleList.Counter > -1 ? simpleList.Id[(int) Math.Round( (VBMath.Rnd() *  (simpleList.Counter + 1)))] : -1;
    }

    pub fn ExecResourceComplient()
    {
      int[] numArray1 = new int[500];
      if (!self.game.Data.RegimeObj[self.game.Data.Turn].AI)
        return;
      self.game.ProcessingObj.LocationProductionPrognosis();
      let mut index1: i32 =  0;
      do
      {
        let mut num1: i32 =  0;
        do
        {
          let mut num2: i32 =  0;
          let mut num3: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].TempRegimeSlotPredict[index1];
          if (num3 < 0 & self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[index1] != num3)
          {
            num4: i32;
            do
            {
              num2 += 1;
              self.game.ProcessingObj.LocationProductionPrognosis();
              num4 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[index1] + self.game.Data.RegimeObj[self.game.Data.Turn].TempRegimeSlotPredict[index1];
              if (num4 < 0 & self.game.Data.RegimeObj[self.game.Data.Turn].TempRegimeSlotPredict[index1] != 0)
              {
                SimpleList simpleList1 = SimpleList::new();
                let mut locCounter1: i32 =  self.game.Data.LocCounter;
                for (let mut tid: i32 =  0; tid <= locCounter1; tid += 1)
                {
                  if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tid].X, self.game.Data.LocObj[tid].Y].Regime == self.game.Data.Turn)
                  {
                    let mut tdata1: i32 =  0;
                    do
                    {
                      if (self.game.Data.LocObj[tid].ProdPercent[tdata1] > 0)
                      {
                        let mut tdata2: i32 =  self.game.Data.LocObj[tid].Production[tdata1];
                        if (tdata2 > -1)
                        {
                          let mut index2: i32 =  0;
                          do
                          {
                            if (self.game.Data.ItemTypeObj[tdata2].RegimeSlotsCost[index2] == index1 & self.game.Data.ItemTypeObj[tdata2].RegimeSlotsCostQty[index2] > 0)
                            {
                              let mut tweight: i32 =  (int) Math.Round(100.0 /  self.game.Data.ItemTypeObj[tdata2].ProdWeight *  self.game.Data.ItemTypeObj[tdata2].RegimeSlotsCostQty[index2] * 100.0);
                              if (self.game.Data.ItemTypeObj[tdata2].IsSFType > -1 && self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tdata2].IsSFType].Theater == 1)
                                tweight = (int) Math.Round( Conversion.Int( (int) Math.Round( tweight / 2.0) * VBMath.Rnd()));
                              simpleList1.Add(tid, tweight, tdata1, tdata2, CheckExistence: false);
                            }
                            index2 += 1;
                          }
                          while (index2 <= 4);
                        }
                      }
                      tdata1 += 1;
                    }
                    while (tdata1 <= 3);
                  }
                }
                simpleList1.ReverseSort();
                let mut num5: i32 =  0;
                let mut index3: i32 =  simpleList1.Data2[0];
                let mut index4: i32 =  simpleList1.Data1[0];
                let mut locnr: i32 =  simpleList1.Id[0];
                int[] numArray2 = new int[500];
                let mut index5: i32 =  0;
                do
                {
                  if (self.game.Data.ItemTypeObj[index3].RegimeSlotsCost[index5] > -1)
                    numArray2[self.game.Data.ItemTypeObj[index3].RegimeSlotsCost[index5]] = self.game.Data.ItemTypeObj[index3].RegimeSlotsCostQty[index5];
                  index5 += 1;
                }
                while (index5 <= 4);
                if (num5 == 0)
                {
                  SimpleList simpleList2 = SimpleList::new();
                  switch (num1)
                  {
                    case 0:
                      let mut locCounter2: i32 =  self.game.Data.LocCounter;
                      for (let mut index6: i32 =  0; index6 <= locCounter2; index6 += 1)
                      {
                        if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index6].X, self.game.Data.LocObj[index6].Y].Regime == self.game.Data.Turn)
                        {
                          let mut index7: i32 =  0;
                          do
                          {
                            if (self.game.Data.LocObj[index6].ProdPercent[index7] > 0)
                            {
                              let mut tid: i32 =  self.game.Data.LocObj[index6].Production[index7];
                              if (tid > -1)
                              {
                                let mut num6: i32 =  1;
                                let mut index8: i32 =  0;
                                do
                                {
                                  if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCost[index8] > -1)
                                  {
                                    if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCost[index8] == index1 & self.game.Data.ItemTypeObj[tid].RegimeSlotsCostQty[index8] >= numArray2[index1])
                                      num6 = 0;
                                    if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCostQty[index8] >= numArray2[index1])
                                      num6 = 0;
                                  }
                                  index8 += 1;
                                }
                                while (index8 <= 4);
                                if (num6 == 1 && !(self.game.Data.ItemTypeObj[index3].IsSFType > -1 & self.game.Data.ItemTypeObj[tid].IsSFType == -1) && !(self.game.Data.ItemTypeObj[index3].IsRegimeSlot > -1 & self.game.Data.ItemTypeObj[tid].IsRegimeSlot == -1) && !(self.game.Data.ItemTypeObj[index3].IsSupply & !self.game.Data.ItemTypeObj[tid].IsSupply) && !(self.game.Data.ItemTypeObj[index3].IsResPt & !self.game.Data.ItemTypeObj[tid].IsResPt) && self.game.Data.ItemTypeObj[tid].IsSFType > -1 & self.game.Data.ItemTypeObj[index3].IsSFType > -1 && self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[5] <= 0 && self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[1] <= 0)
                                  simpleList2.Add(tid, 3000 + (int) Math.Round( (VBMath.Rnd() * 1000f)));
                              }
                            }
                            index7 += 1;
                          }
                          while (index7 <= 3);
                        }
                      }
                      break;
                    case 1:
                      let mut num7: i32 =  0;
                      do
                      {
                        let mut meRandomUnit: i32 =  self.GetMeRandomUnit();
                        if (meRandomUnit > -1)
                        {
                          let mut tid: i32 =  self.FindBestSuitedItemType(meRandomUnit, 6, 10000, locnr, true);
                          if (tid == 0)
                            tid = 0;
                          if (tid > -1)
                          {
                            let mut num8: i32 =  1;
                            let mut index9: i32 =  0;
                            do
                            {
                              if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCost[index9] > -1)
                              {
                                if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCost[index9] == index1 & self.game.Data.ItemTypeObj[tid].RegimeSlotsCostQty[index9] >= numArray2[index1])
                                  num8 = 0;
                                if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCostQty[index9] >= numArray2[index1])
                                  num8 = 0;
                              }
                              index9 += 1;
                            }
                            while (index9 <= 4);
                            if (num8 == 1)
                            {
                              if (self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[5] > 0)
                                simpleList2.Add(tid, 4000 - self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[6] * 10);
                              else if (self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[1] > 0)
                                simpleList2.Add(tid, 4000 - self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[6] * 10);
                              else
                                simpleList2.Add(tid, 2000 - self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[6] * 10);
                            }
                          }
                        }
                        num7 += 1;
                      }
                      while (num7 <= 10);
                      break;
                    default:
                      let mut locCounter3: i32 =  self.game.Data.LocCounter;
                      for (let mut index10: i32 =  0; index10 <= locCounter3; index10 += 1)
                      {
                        if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index10].X, self.game.Data.LocObj[index10].Y].Regime == self.game.Data.Turn)
                        {
                          let mut index11: i32 =  0;
                          do
                          {
                            if (self.game.Data.LocObj[index10].ProdPercent[index11] > 0)
                            {
                              let mut tid: i32 =  self.game.Data.LocObj[index10].Production[index11];
                              if (tid > -1)
                              {
                                let mut num9: i32 =  1;
                                let mut index12: i32 =  0;
                                do
                                {
                                  if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCost[index12] > -1)
                                  {
                                    if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCost[index12] == index1 & self.game.Data.ItemTypeObj[tid].RegimeSlotsCostQty[index12] >= numArray2[index1])
                                      num9 = 0;
                                    if (self.game.Data.ItemTypeObj[tid].RegimeSlotsCostQty[index12] >= numArray2[index1])
                                      num9 = 0;
                                  }
                                  index12 += 1;
                                }
                                while (index12 <= 4);
                                if (num9 == 1)
                                {
                                  if (self.game.Data.ItemTypeObj[index3].IsSFType > -1 & self.game.Data.ItemTypeObj[tid].IsSFType == -1)
                                    simpleList2.Add(tid, 99999);
                                  else if (self.game.Data.ItemTypeObj[index3].IsRegimeSlot > -1 & self.game.Data.ItemTypeObj[tid].IsRegimeSlot == -1)
                                    simpleList2.Add(tid, 99999);
                                  else if (self.game.Data.ItemTypeObj[index3].IsSupply & !self.game.Data.ItemTypeObj[tid].IsSupply)
                                    simpleList2.Add(tid, 99999);
                                  else if (self.game.Data.ItemTypeObj[index3].IsResPt & !self.game.Data.ItemTypeObj[tid].IsResPt)
                                    simpleList2.Add(tid, 99999);
                                  else if (self.game.Data.ItemTypeObj[tid].IsSFType > -1 & self.game.Data.ItemTypeObj[index3].IsSFType > -1)
                                  {
                                    if (self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[5] <= 0 && self.game.Data.SFTypeObj[self.game.Data.ItemTypeObj[tid].IsSFType].AIRoleScore[1] <= 0)
                                      simpleList2.Add(tid, 3000 + (int) Math.Round( (VBMath.Rnd() * 1000f)));
                                  }
                                  else
                                    simpleList2.Add(tid, 5000 + (int) Math.Round( (VBMath.Rnd() * 1000f)));
                                }
                              }
                            }
                            index11 += 1;
                          }
                          while (index11 <= 3);
                        }
                      }
                      break;
                  }
                  simpleList2.Sort();
                  if (simpleList2.Counter > -1)
                  {
                    let mut counter: i32 =  simpleList2.Counter;
                    for (let mut index13: i32 =  0; index13 <= counter; index13 += 1)
                    {
                      let mut itemtypenr: i32 =  simpleList2.Id[index13];
                      if (self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, itemtypenr).result)
                      {
                        self.game.Data.LocObj[locnr].Production[index4] = itemtypenr;
                        break;
                      }
                    }
                  }
                }
              }
            }
            while (num4 < 0 & num2 < 999);
          }
          num1 += 1;
        }
        while (num1 <= 2);
        index1 += 1;
      }
      while (index1 <= 499);
    }

    ~AIClass() => base.Finalize();
  }
