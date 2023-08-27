// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIWorld
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;

namespace WindowsApplication1
{
  pub class AIWorld
  {
    pub ai: DC2AIClass;
    pub AIMatrix owner;
    pub AIMatrix frontline;
    pub AIMatrix troops;
    pub AIMatrix offtroops;
    pub AIMatrix move;
    pub AIMatrix friendlyBottleneck;
    pub AIMatrix vp;
    pub AIMatrix friendlySupply;
    pub AIMatrix friendlySupplyRoute;
    pub AIMatrix enemySupply;
    pub startFriendlyTroops: i32;
    pub startEnemyTroops: i32;
    pub startFriendlyVP: i32;
    pub startEnemyVP: i32;
    pub halfScore: Vec<i32>;
    pub float AbsoluteFriendlyMod;
    pub float FriendlyMod;
    pub InitialPercentageInSupply: i32;
    pub AIFrontList frontList;
    pub run: i32;

    pub int UnitOffensiveModifier(int unr)
    {
      let mut sfCount: i32 =  this.ai.game.Data.UnitObj[unr].SFCount;
      int num1;
      int num2;
      for (let mut index: i32 =  0; index <= sfCount; index += 1)
      {
        let mut sf: i32 =  this.ai.game.Data.UnitObj[unr].SFList[index];
        if (this.ai.game.Data.SFTypeObj[this.ai.game.Data.SFObj[sf].Type].AIRoleScore[10] > 0)
        {
          num1 += this.ai.game.Data.SFObj[sf].Qty;
          num2 += this.ai.game.Data.SFObj[sf].Qty;
        }
        else
          num1 += this.ai.game.Data.SFObj[sf].Qty;
      }
      return (int) Math.Round( ( num2 /  num1 * 100f));
    }

    pub AIWorld(
      ref tai: DC2AIClass,
      AIFrontList tfrontList,
      float tFriendlyMod = 1f,
      float tAbsoluteFriendlyMod = 0.0f,
      bool ModifyForUnitRatio = false)
    {
      this.halfScore = new int[100];
      this.run = 0;
      this.ai = tai;
      this.frontList = tfrontList;
      this.frontline = this.ai.frontMatrix.Clone();
      this.SetOwner();
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      if (this.ai.VAR_STRATEGIC_WEAKNESS_AT_BOTTLENECK)
      {
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (this.owner.Value[index1, index2] == 2 && this.ai.map.HexObj[index1, index2].UnitCounter > -1)
            {
              num3 += 1;
              if (this.ai.enemyBoostMatrix.Value[index1, index2] > num2)
                num2 = this.ai.enemyBoostMatrix.Value[index1, index2];
              num1 += this.ai.enemyBoostMatrix.Value[index1, index2];
            }
          }
        }
        if (num3 > 0)
          num1 = (int) Math.Round( num1 /  num3);
      }
      this.ai.SetTroopsAndAPMatrix(ref this.troops, ref this.move, this.ai.map.MapWidth, this.ai.map.MapHeight, 0, 0, ref this.frontList, tfrontArea: this.frontline);
      this.FriendlyMod = tFriendlyMod;
      this.AbsoluteFriendlyMod = tAbsoluteFriendlyMod;
      this.offtroops = new AIMatrix(ref this.ai);
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      for (let mut x: i32 =  0; x <= mapWidth1; x += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut y: i32 =  0; y <= mapHeight; y += 1)
        {
          if (this.ai.CustomCalls.HasCustumCalls())
          {
            numArray1: Vec<i32> = this.troops.Value;
            numArray2: Vec<i32> = numArray1;
            let mut index3: i32 =  x;
            let mut index4: i32 =  index3;
            let mut index5: i32 =  y;
            let mut index6: i32 =  index5;
            let mut num4: i32 =  numArray1[index3, index5] + this.ai.CustomCalls.CustomRuleWorld_ExtraTroopsOnHex(x, y, this.troops.Value[x, y]);
            numArray2[index4, index6] = num4;
          }
          if (this.owner.Value[x, y] == 1)
            this.troops.Value[x, y] = (int) Math.Round( ( this.troops.Value[x, y] * this.FriendlyMod));
          if (this.owner.Value[x, y] == 1 & ModifyForUnitRatio & this.frontline.Value[x, y] > 0)
            this.troops.Value[x, y] = (int) Math.Round( this.troops.Value[x, y] * Math.Max(0.2,  Math.Min(1f, this.frontList.Front[this.frontList.GetFrontNr(this.frontline.Value[x, y])].UnitCountRatio)));
          if (this.owner.Value[x, y] == 2)
          {
            let mut num5: i32 =  this.troops.Value[x, y];
            if (this.ai.VAR_STRATEGIC_WEAKNESS_AT_BOTTLENECK & num5 > 0)
            {
              let mut num6: i32 =  this.ai.enemyBoostMatrix.Value[x, y] - num1;
              if (num6 >= num1 * 9)
                num5 *= 2;
              else if (num6 >= num1 * 7)
                num5 = (int) Math.Round( num5 * 1.75);
              else if (num6 >= num1 * 5)
                num5 = (int) Math.Round( num5 * 1.5);
              else if (num6 >= num1 * 3)
                num5 = (int) Math.Round( num5 * 1.25);
              else if (num6 < 0)
              {
                if (num6 <= 0)
                  num5 = (int) Math.Round( num5 * 0.75);
              }
              this.troops.Value[x, y] = num5;
            }
          }
          if (this.owner.Value[x, y] == 2)
          {
            let mut num7: i32 =  0;
            let mut num8: i32 =  0;
            let mut unitCounter: i32 =  this.ai.map.HexObj[x, y].UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            {
              let mut unit: i32 =  this.ai.map.HexObj[x, y].UnitList[index];
              if (this.ai.game.HandyFunctionsObj.HasUnitlandSF(unit))
              {
                num7 += this.UnitOffensiveModifier(unit);
                num8 += 1;
              }
            }
            if (num8 > 0 & num7 > 0)
            {
              let mut num9: i32 =  (int) Math.Round( num7 /  num8);
              this.offtroops.Value[x, y] = num9;
            }
          }
        }
      }
      this.vp = this.ai.SetVPMatrix();
      this.CellularAutomatonSetFriendlySupply();
      this.CellularAutomatonSetFriendlySupplyRoute();
      this.CellularAutomatonSetEnemySupply();
      this.friendlyBottleneck = this.ai.SetFriendlyBottleNeckMatrix((AIFront) null, this.friendlySupply, this.owner, false, 2);
      this.SetStartValues();
      let mut index7: i32 =  0;
      do
      {
        this.halfScore[index7] = -999999;
        index7 += 1;
      }
      while (index7 <= 99);
    }

    pub AIWorld()
    {
      this.halfScore = new int[100];
      this.run = 0;
    }

    pub AIWorld Clone()
    {
      AIWorld aiWorld = AIWorld::new();
      aiWorld.ai = this.ai;
      this.frontline = this.ai.frontMatrix.Clone();
      aiWorld.owner = this.owner.Clone();
      aiWorld.frontline = this.frontline.Clone();
      aiWorld.troops = this.troops.Clone();
      aiWorld.move = this.move.Clone();
      aiWorld.friendlySupply = this.friendlySupply.Clone();
      aiWorld.friendlySupplyRoute = this.friendlySupplyRoute.Clone();
      aiWorld.enemySupply = this.enemySupply.Clone();
      aiWorld.startEnemyTroops = this.startEnemyTroops;
      aiWorld.startFriendlyTroops = this.startFriendlyTroops;
      aiWorld.startEnemyVP = this.startEnemyVP;
      aiWorld.startFriendlyVP = this.startFriendlyVP;
      aiWorld.vp = this.vp;
      let mut index: i32 =  0;
      do
      {
        aiWorld.halfScore[index] = this.halfScore[index];
        index += 1;
      }
      while (index <= 99);
      return aiWorld;
    }

    pub fn SetHalfScore(int tround, AIFrontList tPlan) => this.halfScore[tround] = this.GetScore("", false, tPlan, true);

    pub int SetStartValues()
    {
      let mut num1: i32 =  0;
      let mut mapWidth: i32 =  this.ai.map.MapWidth;
      int num2;
      int val2;
      int num3;
      int num4;
      int num5;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (this.owner.Value[index1, index2] == 1)
          {
            if (this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2] > 0)
              num2 += this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2];
            if (this.troops.Value[index1, index2] > 0)
              val2 += this.troops.Value[index1, index2];
            if (this.friendlySupply.Value[index1, index2] <= this.ai.VAR_SUPPLY_MAXIMUM_RANGE)
              num3 += this.troops.Value[index1, index2];
          }
          else if (this.owner.Value[index1, index2] == 2)
          {
            if (this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2] > 0)
              num4 += this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2];
            if (this.troops.Value[index1, index2] > 0)
              num5 += this.troops.Value[index1, index2];
          }
        }
      }
      this.InitialPercentageInSupply = (int) Math.Round( (num3 * 100) /  Math.Max(1, val2));
      if (num5 == 0)
        num5 = 1;
      if (num4 == 0)
        num4 = 1;
      this.startFriendlyTroops = val2;
      this.startEnemyTroops = num5;
      this.startFriendlyVP = num2;
      this.startEnemyVP = num4;
      return num1;
    }

    pub int GetScore(logFileName: String, bool DoLog, AIFrontList plan, bool halfscoreCalc)
    {
      let mut mapWidth: i32 =  this.ai.map.MapWidth;
      int num1;
      int num2;
      int num3;
      int num4;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (this.owner.Value[index1, index2] == 1)
          {
            if (this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2] > 0)
              num1 += this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2];
            if (this.troops.Value[index1, index2] > 0)
              num2 += this.troops.Value[index1, index2];
          }
          else if (this.owner.Value[index1, index2] == 2)
          {
            if (this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2] > 0)
              num3 += this.ai.map.HexObj[index1, index2].VP + this.ai.game.Data.RegimeObj[this.ai.GetGameDataTurn()].AIVP[0].Value[index1, index2];
            if (this.troops.Value[index1, index2] > 0)
              num4 += this.troops.Value[index1, index2];
          }
        }
      }
      if (num4 == 0)
        num4 = 1;
      if (num3 == 0)
        num3 = 1;
      float num5 =  this.startFriendlyTroops /  this.startEnemyTroops;
      float num6 =  num2 /  num4;
      let mut num7: i32 =  (int) Math.Round(( num6 -  num5) * 100.0);
      float num8 =  this.startFriendlyVP /  this.startEnemyVP;
      float num9 =  num1 /  num3;
      let mut num10: i32 =  (int) Math.Round(( num9 -  num8) * 100.0) * 1;
      let mut score: i32 =  num7 + num10;
      if (!halfscoreCalc)
      {
        let mut num11: i32 =  1;
        let mut index: i32 =  0;
        do
        {
          if (this.halfScore[index] > -999999)
          {
            num11 += 1;
            score += this.halfScore[index];
          }
          index += 1;
        }
        while (index <= 99);
        score = (int) Math.Round( score /  num11);
      }
      if (DoLog)
      {
        this.ai.ClearLog();
        this.ai.AddLog("StartFriendlyTroops = " + this.startFriendlyTroops.ToString());
        this.ai.AddLog("StartEnemyTroops = " + this.startEnemyTroops.ToString());
        this.ai.AddLog("EndFriendlyTroops = " + num2.ToString());
        this.ai.AddLog("EndEnemyTroops = " + num4.ToString());
        this.ai.AddLog("Initial Ratio = " + num5.ToString() + ", End Ratio = " + num6.ToString());
        this.ai.AddLog("------------");
        this.ai.AddLog("Score1 = " + num7.ToString());
        this.ai.AddLog("");
        this.ai.AddLog("StartFriendlyVP = " + this.startFriendlyVP.ToString());
        this.ai.AddLog("StartEnemyVP = " + this.startEnemyVP.ToString());
        this.ai.AddLog("EndFriendlyVP = " + num1.ToString());
        this.ai.AddLog("EndEnemyVP = " + num3.ToString());
        this.ai.AddLog("Initial Ratio = " + num8.ToString() + ", End Ratio = " + num9.ToString());
        this.ai.AddLog("------------");
        this.ai.AddLog("Score2 = " + num10.ToString());
        this.ai.AddLog("");
        if (!halfscoreCalc)
        {
          let mut index: i32 =  0;
          do
          {
            if (this.halfScore[index] > -999999)
              this.ai.AddLog("Round " + index.ToString() + " score: " + this.halfScore[index].ToString());
            index += 1;
          }
          while (index <= 99);
        }
        this.ai.AddLog("");
        this.ai.AddLog("Final Score = " + score.ToString());
        this.ai.AddLog("");
        this.ai.AddLog("---------------------");
        this.ai.AddLog("FRONTS:");
        this.ai.AddLog("");
        let mut counter1: i32 =  plan.Counter;
        for (let mut index3: i32 =  0; index3 <= counter1; index3 += 1)
        {
          str1: String = "";
          if (plan.Front[index3].FrontType == 1)
            str1 = " , type: FRONTLINE";
          if (plan.Front[index3].FrontType == 2)
            str1 = " , type: RESERVE";
          if (plan.Front[index3].FrontType == 1)
          {
            str1 = str1 + "\r\n" + "STANCE: ";
            if (plan.Front[index3].StartStance == 3)
              str1 += "ATTACK".to_owned();
            if (plan.Front[index3].StartStance == 2)
              str1 += "HOLD".to_owned();
            if (plan.Front[index3].StartStance == 1)
              str1 += "RETREAT".to_owned();
          }
          str2: String = str1 + ", AVG.STRENGTH: " + plan.Front[index3].AverageStrength.ToString();
          if (plan.Front[index3].FrontType == 2)
            str2 = str2 + "\r\n" + "TARGET-FRONT: " + plan.Front[index3].TargetFrontID.ToString() + ", DISTANCE: " + plan.Front[index3].Distance.ToString();
          this.ai.AddLog("FRONT " + plan.Front[index3].FrontID.ToString() + str2);
          this.ai.AddLog("----------------------------------");
          let mut counter2: i32 =  plan.Front[index3].units.counter;
          for (let mut index4: i32 =  0; index4 <= counter2; index4 += 1)
            this.ai.AddLog(this.ai.game.Data.UnitObj[plan.Front[index3].units.unr[index4]].Name);
          this.ai.AddLog("");
        }
        this.ai.WriteLog("score=" + score.ToString() + "_" + logFileName);
      }
      return score;
    }

    pub fn SetOwner() => this.owner = this.ai.SetOwnerMatrix(0, 0, this.ai.map.MapWidth, this.ai.map.MapHeight);

    pub fn CellularAutomatonSetFriendlySupply()
    {
      this.friendlySupply = new AIMatrix(ref this.ai);
      this.friendlySupply.SetAllValuesTo(9999);
      let mut index: i32 =  0;
      do
      {
        if (this.ai.VAR_SUPPLY_ACTIVE[this.ai.GetGameDataTurn(), index])
          this.friendlySupply.Value[this.ai.VAR_SUPPLY_X[this.ai.GetGameDataTurn(), index], this.ai.VAR_SUPPLY_Y[this.ai.GetGameDataTurn(), index]] = 0;
        index += 1;
      }
      while (index <= 3);
      this.friendlySupply.ExpandAsSimplifiedSupplyMatrix(this.ai.VAR_SUPPLY_FRIENDLY_MOVETYPE, ref this.owner, 1, (AICoordinateMatrix) null);
    }

    pub fn CellularAutomatonSetFriendlySupplyRoute()
    {
      this.friendlySupplyRoute = new AIMatrix(ref this.ai);
      this.friendlySupplyRoute.SetAllValuesTo(9999);
      let mut index: i32 =  0;
      do
      {
        if (this.ai.VAR_SUPPLY_ACTIVE[this.ai.GetGameDataTurn(), index])
          this.friendlySupplyRoute.Value[this.ai.VAR_SUPPLY_X[this.ai.GetGameDataTurn(), index], this.ai.VAR_SUPPLY_Y[this.ai.GetGameDataTurn(), index]] = 0;
        index += 1;
      }
      while (index <= 3);
      this.friendlySupplyRoute.ExpandAsSimplifiedSupplyRouteMatrix(this.ai.VAR_SUPPLY_FRIENDLY_MOVETYPE, ref this.owner, 1);
    }

    pub fn CellularAutomatonSetEnemySupply()
    {
      this.enemySupply = new AIMatrix(ref this.ai);
      this.enemySupply.SetAllValuesTo(9999);
      let mut regimeCounter: i32 =  this.ai.game.Data.RegimeCounter;
      for (let mut reg1: i32 =  0; reg1 <= regimeCounter; reg1 += 1)
      {
        if (!this.ai.game.HandyFunctionsObj.IsAlliedOrSelf(reg1, this.ai.game.Data.Turn))
        {
          let mut index: i32 =  0;
          do
          {
            if (this.ai.VAR_SUPPLY_ACTIVE[reg1, index])
              this.enemySupply.Value[this.ai.VAR_SUPPLY_X[reg1, index], this.ai.VAR_SUPPLY_Y[reg1, index]] = 0;
            index += 1;
          }
          while (index <= 3);
        }
      }
      this.enemySupply.ExpandAsSimplifiedSupplyMatrix(this.ai.VAR_SUPPLY_ENEMY_MOVETYPE, ref this.owner, 2, (AICoordinateMatrix) null);
    }

    pub fn CellularAutomatonEnemyNextTurn() => this.CellularAutomatonSetEnemySupply();

    pub fn CellularAutomatonFriendlyNextTurn()
    {
      this.CellularAutomatonSetFriendlySupply();
      this.CellularAutomatonSetFriendlySupplyRoute();
    }

    pub fn CellularAutomatonEnemyForces()
    {
      if (this.ai.VAR_ENEMYMOVE_PROGNOSIS_MODE <= 1)
      {
        this.CellularAutomatonEnemySpreadOut(1);
        if (this.run <= 1)
          this.CellularAutomatonEnemyAttack(0.2f);
        this.CellularAutomatonEnemySpreadOut(2);
        this.CellularAutomatonEnemyAttack(1f /  this.run);
        if (this.ai.VAR_ENEMYMOVE_PROGNOSIS_MODE == 0)
        {
          if (this.run <= 2)
            this.CellularAutomatonEnemySpreadOut(2);
          if (this.run <= 2)
            this.CellularAutomatonEnemyAttack(0.3f /  this.run);
        }
        else
          this.CellularAutomatonEnemySpreadOut(1);
        this.CellularAutomatonEnemySpreadOut(2);
      }
      if (this.ai.VAR_ENEMYMOVE_PROGNOSIS_MODE == 0)
        this.CellularAutomatonMoveEnemyTowardsFront(3, this.ai.VAR_FRONTLINE_DEPTH + 1);
      if (this.ai.VAR_ENEMYMOVE_PROGNOSIS_MODE != 1)
        return;
      this.CellularAutomatonMoveEnemyTowardsFront(1, this.ai.VAR_FRONTLINE_DEPTH + 1);
    }

    pub fn CellularAutomatonStartXIterations(int iterations)
    {
      let mut num: i32 =  iterations;
      for (let mut iteration: i32 =  1; iteration <= num; iteration += 1)
      {
        this += 1.run;
        if (this.run > 1)
        {
          this.frontline.ExpandUniquesValuesForSameRegime(2);
          this.frontline.RemoveValuesByMask(this.owner, 2);
        }
        this.CellularAutomatonFriendlyForces(this.frontList, iteration);
        this.CellularAutomatonEnemyNextTurn();
        this.CellularAutomatonEnemyForces();
        this.CellularAutomatonFriendlyNextTurn();
        this.CellularAutomatonSetStats();
      }
    }

    pub fn CellularAutomatonSetStats()
    {
      let mut counter: i32 =  this.frontList.Counter;
      for (let mut index1: i32 =  0; index1 <= counter; index1 += 1)
      {
        AIFront aiFront = this.frontList.Front[index1];
        let mut num1: i32 =  0;
        let mut num2: i32 =  0;
        let mut num3: i32 =  0;
        let mut num4: i32 =  0;
        let mut d: i32 =  0;
        let mut num5: i32 =  0;
        let mut num6: i32 =  0;
        let mut num7: i32 =  0;
        if (aiFront.FrontID == 375)
          num6 = num6;
        if (aiFront.FrontType == 1)
        {
          let mut mapWidth: i32 =  this.ai.map.MapWidth;
          for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
          {
            let mut mapHeight: i32 =  this.ai.map.MapHeight;
            for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            {
              if (this.frontline.Value[index2, index3] == aiFront.FrontID & this.owner.Value[index2, index3] == 1)
              {
                num6 += this.troops.Value[index2, index3];
                if (this.friendlySupply.Value[index2, index3] > (int) Math.Round( (this.ai.VAR_SUPPLY_50PERCENT_RANGE + this.ai.VAR_SUPPLY_25PERCENT_RANGE) / 2.0) && this.ai.game.Data.MapObj[0].HexObj[index2, index3].Regime == this.ai.game.Data.Turn)
                  num7 += this.troops.Value[index2, index3];
              }
              if (this.ai.frontMatrix.Value[index2, index3] == aiFront.FrontID & (this.troops.Value[index2, index3] > 0 | this.ai.frontlinesMatrix.Value[index2, index3] == 1))
              {
                if (this.ai.VAR_MATRIX_RETREAT.Value[index2, index3] <= 25)
                  d += 1;
                else if (this.ai.VAR_MATRIX_RETREAT.Value[index2, index3] <= 50)
                  num5 += 1;
              }
              if (this.ai.frontMatrix.Value[index2, index3] == aiFront.FrontID)
              {
                num4 += 1;
                num3 += this.ai.VAR_MATRIX_RETREAT.Value[index2, index3];
              }
              if (this.ai.frontMatrix.Value[index2, index3] == aiFront.FrontID & this.ai.game.Data.MapObj[0].HexObj[index2, index3].Regime == this.ai.game.Data.Turn)
                num1 = num1 + 1 + this.ai.game.Data.MapObj[0].HexObj[index2, index3].VP;
              if (this.frontline.Value[index2, index3] == aiFront.FrontID & this.owner.Value[index2, index3] == 1 & this.ai.game.Data.MapObj[0].HexObj[index2, index3].Regime == this.ai.game.Data.Turn)
                num2 = num2 + 1 + this.ai.game.Data.MapObj[0].HexObj[index2, index3].VP;
            }
          }
        }
        let mut num8: i32 =  num7 <= 0 ? 0 : (int) Math.Round( (num7 * 100) /  num6);
        if (this.ai.game.Data.Turn == 4 & aiFront.FrontID == 2)
          num8 = num8;
        if ( num6 >  aiFront.OrigPower * 0.1)
        {
          aiFront.StatAvgPercentageOutOfSupply = (int) Math.Round( (aiFront.StatLastPercentageOutOfSupply * (aiFront.StatIterationCount + 1) + num8) /  (aiFront.StatIterationCount + 1));
          aiFront.StatLastPercentageOutOfSupply = num8;
          aiFront += 1.StatIterationCount;
        }
        if (num1 > 0)
          aiFront.statsHexLeftPercentage = (int) Math.Round(Math.Ceiling( (100 * num2) /  num1));
        if (this.run == 1)
        {
          aiFront.statLastPowerPercentageRun1 = (int) Math.Round( ((num6 + 1) * 100) /  (aiFront.OrigPower + 1));
          if (aiFront.statLastPowerPercentageRun1 > 100)
            aiFront.statLastPowerPercentageRun1 = 100;
        }
        if (num4 > 0)
        {
          let mut num9: i32 =  (int) Math.Round( ((int) Math.Round( num3 /  num4) + aiFront.retreatAverageScore) / 2.0);
          if (d > 0)
            num9 = (int) Math.Round( num9 / Math.Sqrt( d));
          if (num5 > 3)
            num9 = (int) Math.Round( num9 / Math.Sqrt(Math.Sqrt( (num5 - 3))));
          if (num9 >= 100 & this.run == 1 | num9 > 0 & num9 < 100 & this.run > 1)
            num6 = (int) Math.Round( (num6 + (int) Math.Round( aiFront.OrigPower / 8.0)) * (100.0 /  num9));
        }
        aiFront.StatLastPowerPercentageFullRun = (int) Math.Round( ((num6 + 1) * 100) /  (aiFront.OrigPower + 1));
        if (aiFront.StatLastPowerPercentageFullRun > 100)
          aiFront.StatLastPowerPercentageFullRun = 100;
      }
    }

    pub fn CellularAutomatonFriendlyForces(AIFrontList frontList, int iteration)
    {
      this.CellularAutomatonMoveTowardsFront(3, this.ai.VAR_FRONTLINE_DEPTH);
      let mut counter: i32 =  frontList.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        AIFront front = frontList.Front[index];
        if (front.FrontID == 1421)
          front.FrontID = front.FrontID;
        if (front.FrontType == 1)
        {
          let mut num: i32 =  front.Stance != 2 ? (!(front.Stance == 3 &  front.OrigAverageStrength >= 3.5) ? (front.Stance != 3 ? 1 : 3) : 4) : 2;
          if (front.Stance == 3)
          {
            if (front.FrontID < 1000000 | this.run > 1)
              this.CellularAutomatonAttack(front, Math.Max(1, (int) Math.Round( num / 2.0)));
            this.CellularAutomatonSpreadOut(front, num + 2);
          }
          else if (front.Stance == 2 | iteration > 1)
          {
            if (front.FrontID < 1000000 | this.run > 1 && num >= 2)
              this.CellularAutomatonAttack(front, 1);
            this.CellularAutomatonSpreadOut(front, num + 2);
          }
          else if (front.Stance == 1 | front.Stance == 4)
            this.CellularAutomatonFallBack(front, 4);
        }
        else if (front.FrontType != 2)
          ;
      }
    }

    pub fn CellularAutomatonAttack(AIFront front, int MAX_STEPS)
    {
      int num1;
      int num2;
      do
      {
        num1 = 0;
        AIMatrix aiMatrix = new AIMatrix(ref this.ai);
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (this.frontline.Value[index1, index2] == front.FrontID | this.run > 1 && this.friendlySupply.Value[index1, index2] <= this.ai.VAR_SUPPLY_50PERCENT_RANGE && this.troops.Value[index1, index2] > 0 & aiMatrix.Value[index1, index2] == 0)
            {
              let mut x: i32 =  -1;
              let mut y: i32 =  -1;
              let mut num3: i32 =  -1;
              let mut attackPower: i32 =  0;
              let mut index3: i32 =  0;
              int totalPowerOn1;
              do
              {
                Coordinate coordinate = this.ai.TempHexNeighbour[index1, index2, index3];
                if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2 && this.ai.game.Data.RegimeObj[this.ai.game.Data.Turn].RegimeRel[this.ai.map.HexObj[coordinate.x, coordinate.y].Regime] == 0 && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1)
                {
                  let mut totalPowerOn2: i32 =  this.CellularAutomatonGetTotalPowerOn(coordinate.x, coordinate.y, front, aiMatrix, false);
                  if (totalPowerOn2 > attackPower)
                  {
                    x = coordinate.x;
                    y = coordinate.y;
                    attackPower = totalPowerOn2;
                    totalPowerOn1 = this.CellularAutomatonGetTotalPowerOn(coordinate.x, coordinate.y, front, aiMatrix, true);
                    num3 = this.CellularAutomatonGetBestRiver(coordinate.x, coordinate.y, aiMatrix);
                  }
                }
                index3 += 1;
              }
              while (index3 <= 5);
              if (x > -1)
              {
                float minimumAttackAdvantage = this.CellularAutomatonGetMinimumAttackAdvantage(front.Stance, this.ai.map.HexObj[x, y].LandscapeType, num3);
                float num4 =  attackPower /  this.ModifyPower(this.troops.Value[x, y], this.enemySupply.Value[x, y], true, (AIFront) null, false);
                if ( num4 >=  minimumAttackAdvantage)
                {
                  let mut lossForAttacker: i32 =  this.CellularAutomatonGetLossForAttacker(front.Stance, attackPower, this.ModifyPower(this.troops.Value[x, y], this.enemySupply.Value[x, y], true, (AIFront) null, false), this.ai.map.HexObj[x, y].LandscapeType, num3, totalPowerOn1);
                  let mut lossForDefender: i32 =  this.CellularAutomatonGetLossForDefender(front.Stance, attackPower, this.ModifyPower(this.troops.Value[x, y], this.enemySupply.Value[x, y], true, (AIFront) null, false));
                  this.CellularAutomatonModifyTroopsOn(x, y, front, lossForAttacker, aiMatrix);
                  numArray1: Vec<i32> = this.troops.Value;
                  numArray2: Vec<i32> = numArray1;
                  let mut index4: i32 =  x;
                  let mut index5: i32 =  index4;
                  let mut index6: i32 =  y;
                  let mut index7: i32 =  index6;
                  let mut num5: i32 =  numArray1[index4, index6] - lossForDefender;
                  numArray2[index5, index7] = num5;
                  float num6 = 10f;
                  if (front.Stance == 3)
                    num6 = 3f;
                  aiMatrix.Value[x, y] = 1;
                  if (front.Stance == 2)
                    num6 = 6f;
                  if ( num4 >=  num6)
                  {
                    this.CellularAutomatonRetreatEnemyTroops(x, y, front);
                    this.owner.Value[x, y] = 1;
                    this.CellularAutomatonMoveIn(x, y, front, aiMatrix);
                    aiMatrix.Value[x, y] = 1;
                    this.frontline.Value[x, y] = front.FrontID;
                    this.friendlySupply.Value[x, y] = this.friendlySupply.Value[index1, index2];
                  }
                  num1 = 1;
                }
              }
            }
          }
        }
        num2 += 1;
      }
      while (num1 > 0 & num2 < MAX_STEPS);
    }

    pub fn CellularAutomatonEnemyAttack(float attackForceModifier)
    {
      let mut num1: i32 =  3;
      let mut num2: i32 =  2;
      if (this.ai.VAR_ENEMYMOVE_PROGNOSIS_MODE == 1)
      {
        num2 = 1;
        num1 = 2;
      }
      if (this.ai.VAR_ENEMYMOVE_PROGNOSIS_MODE == 2)
        return;
      AIMatrix aiMatrix = new AIMatrix(ref this.ai);
      int num3;
      int num4;
      do
      {
        num3 = 0;
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (index1 == 10 & index2 == 17)
              index1 = index1;
            if (this.owner.Value[index1, index2] == 2 && this.ai.game.Data.RegimeObj[this.ai.game.Data.Turn].RegimeRel[this.ai.map.HexObj[index1, index2].Regime] == 0 | this.ai.map.HexObj[index1, index2].Regime == this.ai.game.Data.Turn && this.enemySupply.Value[index1, index2] < 999 && this.troops.Value[index1, index2] > 10 & aiMatrix.Value[index1, index2] == 0 & (num4 < num2 | this.offtroops.Value[index1, index2] > 25))
            {
              let mut num5: i32 =  -1;
              let mut num6: i32 =  -1;
              let mut num7: i32 =  0;
              let mut index3: i32 =  0;
              Coordinate coordinate;
              do
              {
                coordinate = this.ai.TempHexNeighbour[index1, index2, index3];
                if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1 && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1)
                {
                  let mut num8: i32 =  this.friendlyBottleneck.Value[coordinate.x, coordinate.y] + this.vp.Value[coordinate.x, coordinate.y] * 2;
                  if (num8 > num7)
                  {
                    num5 = coordinate.x;
                    num6 = coordinate.y;
                    num7 = num8;
                  }
                }
                index3 += 1;
              }
              while (index3 <= 5);
              let mut x: i32 =  -1;
              let mut y: i32 =  -1;
              let mut num9: i32 =  -9999;
              let mut num10: i32 =  -1;
              let mut index4: i32 =  0;
              int totalEnemyPowerOn1;
              do
              {
                coordinate = this.ai.TempHexNeighbour[index1, index2, index4];
                if (coordinate.x == 31 & coordinate.y == 30)
                  x = x;
                if (coordinate.onmap && aiMatrix.Value[coordinate.x, coordinate.y] < 1 & this.owner.Value[coordinate.x, coordinate.y] == 1 && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1)
                {
                  let mut totalEnemyPowerOn2: i32 =  this.CellularAutomatonGetTotalEnemyPowerOn(coordinate.x, coordinate.y, aiMatrix);
                  int num11;
                  if (this.troops.Value[coordinate.x, coordinate.y] < 1)
                  {
                    num11 = totalEnemyPowerOn2 * 25;
                    if (coordinate.x > 50 & coordinate.y < 7)
                      coordinate.x = coordinate.x;
                  }
                  else
                    num11 = (int) Math.Round( totalEnemyPowerOn2 / 2.0 +  totalEnemyPowerOn2 / 2.0 * Math.Max(0.2, Math.Min(5.0,  totalEnemyPowerOn2 /  this.troops.Value[coordinate.x, coordinate.y])));
                  let mut num12: i32 =  num11 + (int) Math.Round( (this.troops.Value[index1, index2] * 1) * (Math.Sqrt( this.friendlyBottleneck.Value[coordinate.x, coordinate.y]) / 50.0));
                  if (num5 == coordinate.x & num6 == coordinate.y)
                    num12 *= 2;
                  let mut num13: i32 =  num12 + this.vp.Value[index1, index2] * Math.Max(3, this.vp.Value[index1, index2]);
                  if (num13 > num9)
                  {
                    x = coordinate.x;
                    y = coordinate.y;
                    num9 = num13;
                    totalEnemyPowerOn1 = this.CellularAutomatonGetTotalEnemyPowerOn(coordinate.x, coordinate.y, aiMatrix);
                    num10 = this.CellularAutomatonGetBestRiver(coordinate.x, coordinate.y, aiMatrix);
                  }
                }
                index4 += 1;
              }
              while (index4 <= 5);
              if (x > -1)
              {
                let mut totalEnemyPowerOn3: i32 =  this.CellularAutomatonGetTotalEnemyPowerOn(x, y, aiMatrix);
                float minimumAttackAdvantage = this.CellularAutomatonGetMinimumAttackAdvantage(3, this.ai.map.HexObj[x, y].LandscapeType, num10);
                if (totalEnemyPowerOn1 > 10 &  totalEnemyPowerOn1 >  this.ModifyPower(this.troops.Value[x, y], this.friendlySupply.Value[x, y], true, (AIFront) null, true) *  minimumAttackAdvantage)
                {
                  let mut lossForAttacker: i32 =  this.CellularAutomatonGetLossForAttacker(3, totalEnemyPowerOn3, this.ModifyPower(this.troops.Value[x, y], this.friendlySupply.Value[x, y], true, (AIFront) null, true), this.ai.map.HexObj[x, y].LandscapeType, num10);
                  let mut lossForDefender: i32 =  this.CellularAutomatonGetLossForDefender(3, totalEnemyPowerOn3, this.ModifyPower(this.troops.Value[x, y], this.friendlySupply.Value[x, y], true, (AIFront) null, true));
                  let mut attLoss: i32 =  (int) Math.Round( ( lossForAttacker * attackForceModifier));
                  let mut num14: i32 =  (int) Math.Round( ( lossForDefender * attackForceModifier));
                  this.CellularAutomatonModifyEnemyTroopsOn(x, y, attLoss, aiMatrix);
                  let mut num15: i32 =  (int) Math.Round( this.troops.Value[x, y] / 2.0);
                  numArray1: Vec<i32> = this.troops.Value;
                  numArray2: Vec<i32> = numArray1;
                  let mut index5: i32 =  x;
                  let mut index6: i32 =  index5;
                  let mut index7: i32 =  y;
                  let mut index8: i32 =  index7;
                  let mut num16: i32 =  numArray1[index5, index7] - num14;
                  numArray2[index6, index8] = num16;
                  if (this.CellularAutomatonGetTotalEnemyPowerOn(coordinate.x, coordinate.y, aiMatrix) > this.troops.Value[x, y] * 3 && num14 * 3 >= this.troops.Value[x, y])
                  {
                    if (num15 > this.troops.Value[x, y])
                      this.troops.Value[x, y] = num15;
                    this.CellularAutomatonRetreatFriendlyTroops(x, y, this.frontline);
                    this.owner.Value[x, y] = 2;
                    this.CellularAutomatonEnemyMoveIn(x, y, aiMatrix);
                    aiMatrix.Value[x, y] = 1;
                    this.frontline.Value[x, y] = 0;
                    this.enemySupply.Value[x, y] = this.enemySupply.Value[index1, index2];
                    num3 = 1;
                  }
                }
                aiMatrix.Value[x, y] = 1;
              }
            }
          }
        }
        num4 += 1;
      }
      while (num3 > 0 & num4 < num1);
    }

    pub int CellularAutomatonRetreatEnemyTroops(int x, int y, AIFront front)
    {
      let mut index1: i32 =  0;
      int num1;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index1];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2)
          num1 += 1;
        index1 += 1;
      }
      while (index1 <= 5);
      if (num1 > 0)
      {
        let mut num2: i32 =  (int) Math.Round( this.troops.Value[x, y] /  num1);
        let mut index2: i32 =  0;
        do
        {
          Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index2];
          if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2 && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1)
          {
            this.offtroops.Value[coordinate.x, coordinate.y] = (int) Math.Round( this.offtroops.Value[coordinate.x, coordinate.y] /  (this.troops.Value[coordinate.x, coordinate.y] + num2));
            numArray1: Vec<i32> = this.troops.Value;
            numArray2: Vec<i32> = numArray1;
            let mut x1: i32 =  coordinate.x;
            let mut index3: i32 =  x1;
            let mut y1: i32 =  coordinate.y;
            let mut index4: i32 =  y1;
            let mut num3: i32 =  numArray1[x1, y1] + num2;
            numArray2[index3, index4] = num3;
          }
          index2 += 1;
        }
        while (index2 <= 5);
      }
      this.troops.Value[x, y] = 0;
      this.offtroops.Value[x, y] = 0;
      int num4;
      return num4;
    }

    pub int CellularAutomatonRetreatFriendlyTroops(int x, int y, AIMatrix frontlines)
    {
      let mut index1: i32 =  0;
      Coordinate coordinate;
      int num1;
      do
      {
        coordinate = this.ai.TempHexNeighbour[x, y, index1];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1 && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[coordinate.x, coordinate.y].LandscapeType].AIBlock < 1)
          num1 += 1;
        index1 += 1;
      }
      while (index1 <= 5);
      if (num1 > 0)
      {
        let mut num2: i32 =  (int) Math.Round( this.troops.Value[x, y] /  num1);
        let mut index2: i32 =  0;
        do
        {
          coordinate = this.ai.TempHexNeighbour[x, y, index2];
          if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1)
          {
            numArray1: Vec<i32> = this.troops.Value;
            numArray2: Vec<i32> = numArray1;
            let mut x1: i32 =  coordinate.x;
            let mut index3: i32 =  x1;
            let mut y1: i32 =  coordinate.y;
            let mut index4: i32 =  y1;
            let mut num3: i32 =  numArray1[x1, y1] + num2;
            numArray2[index3, index4] = num3;
            if (frontlines.Value[coordinate.x, coordinate.y] == 0)
              frontlines.Value[coordinate.x, coordinate.y] = frontlines.Value[x, y];
          }
          index2 += 1;
        }
        while (index2 <= 5);
      }
      this.troops.Value[x, y] = 0;
      this.offtroops.Value[x, y] = 0;
      int num4;
      return num4;
    }

    pub int CellularAutomatonGetTotalPowerOn(
      int x,
      int y,
      AIFront front,
      AIMatrix blocked,
      bool getRealScore)
    {
      let mut index: i32 =  0;
      int totalPowerOn;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1 && this.frontline.Value[coordinate.x, coordinate.y] == front.FrontID && blocked.Value[coordinate.x, coordinate.y] == 0)
        {
          let mut power: i32 =  this.troops.Value[coordinate.x, coordinate.y];
          let mut num: i32 =  !getRealScore ? this.ModifyPower(power, this.friendlySupply.Value[coordinate.x, coordinate.y], false, front, true) : this.ModifyPower(power, this.friendlySupply.Value[coordinate.x, coordinate.y], false, (AIFront) null, true);
          totalPowerOn += num;
        }
        index += 1;
      }
      while (index <= 5);
      return totalPowerOn;
    }

    pub int CellularAutomatonGetBestRiver(int x, int y, AIMatrix blocked)
    {
      let mut num: i32 =  0;
      let mut bestRiver: i32 =  -1;
      let mut index: i32 =  0;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] != this.owner.Value[x, y] && blocked.Value[coordinate.x, coordinate.y] == 0 && this.ai.map.HexObj[x, y].RiverType[index] > -1)
        {
          let mut tempDefenseBonus: i32 =  this.ai.game.Data.RiverTypeObj[this.ai.map.HexObj[x, y].RiverType[index]].TempDefenseBonus;
          if (tempDefenseBonus > num)
          {
            num = tempDefenseBonus;
            bestRiver = this.ai.map.HexObj[x, y].RiverType[index];
          }
        }
        index += 1;
      }
      while (index <= 5);
      return bestRiver;
    }

    pub int CellularAutomatonGetTotalEnemyPowerOn(int x, int y, AIMatrix blocked)
    {
      let mut index: i32 =  0;
      int totalEnemyPowerOn;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2 && blocked.Value[coordinate.x, coordinate.y] == 0)
        {
          let mut num: i32 =  this.ModifyPower(this.troops.Value[coordinate.x, coordinate.y], this.enemySupply.Value[coordinate.x, coordinate.y], false, (AIFront) null, false);
          if (this.offtroops.Value[coordinate.x, coordinate.y] > 0)
            num += (int) Math.Round( num * ( this.offtroops.Value[coordinate.x, coordinate.y] / 100.0));
          totalEnemyPowerOn += num;
        }
        index += 1;
      }
      while (index <= 5);
      return totalEnemyPowerOn;
    }

    pub int ModifyPower(
      int power,
      int supply,
      bool IsDefender,
      AIFront front,
      bool isFriendly)
    {
      if (IsDefender)
      {
        if (supply > this.ai.VAR_SUPPLY_MAXIMUM_RANGE)
          power = (int) Math.Round( power * 0.5);
        else if (supply > this.ai.VAR_SUPPLY_25PERCENT_RANGE)
          power = (int) Math.Round( power * 0.75);
        else if (supply > this.ai.VAR_SUPPLY_50PERCENT_RANGE)
          power = (int) Math.Round( power * 0.9);
        else if (supply > this.ai.VAR_SUPPLY_75PERCENT_RANGE)
          power *= 1;
      }
      else
      {
        if (supply > this.ai.VAR_SUPPLY_MAXIMUM_RANGE)
          power = (int) Math.Round( power * 0.4);
        else if (supply > this.ai.VAR_SUPPLY_25PERCENT_RANGE)
          power = (int) Math.Round( power * 0.6);
        else if (supply > this.ai.VAR_SUPPLY_50PERCENT_RANGE)
          power = (int) Math.Round( power * 0.8);
        else if (supply > this.ai.VAR_SUPPLY_75PERCENT_RANGE)
          power = (int) Math.Round( power * 0.9);
        if (!Information.IsNothing( front))
          power += (int) Math.Round( power * ( front.OffensiveModifier / 100.0));
      }
      if (IsDefender)
        power = isFriendly ? (int) Math.Round( ( power * this.ai.VAR_DEFENSIVE_WORLD_MODIFIER_FRIENDLY)) : (int) Math.Round( ( power * this.ai.VAR_DEFENSIVE_WORLD_MODIFIER_ENEMY));
      return power;
    }

    pub fn CellularAutomatonMoveIn(int x, int y, AIFront front, AIMatrix blocked)
    {
      let mut index1: i32 =  0;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index1];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1 && this.frontline.Value[coordinate.x, coordinate.y] == front.FrontID && blocked.Value[coordinate.x, coordinate.y] == 0)
        {
          let mut ofEnemyNeighbours: i32 =  this.CellularAutomatonGetNumberOfEnemyNeighbours(coordinate.x, coordinate.y);
          let mut num1: i32 =  (int) Math.Round( this.troops.Value[coordinate.x, coordinate.y] /  (ofEnemyNeighbours + 1));
          numArray1: Vec<i32> = this.troops.Value;
          numArray2: Vec<i32> = numArray1;
          let mut index2: i32 =  x;
          let mut index3: i32 =  index2;
          let mut index4: i32 =  y;
          let mut index5: i32 =  index4;
          let mut num2: i32 =  numArray1[index2, index4] + num1;
          numArray2[index3, index5] = num2;
          numArray3: Vec<i32> = this.troops.Value;
          numArray4: Vec<i32> = numArray3;
          let mut x1: i32 =  coordinate.x;
          let mut index6: i32 =  x1;
          let mut y1: i32 =  coordinate.y;
          let mut index7: i32 =  y1;
          let mut num3: i32 =  numArray3[x1, y1] - num1;
          numArray4[index6, index7] = num3;
        }
        index1 += 1;
      }
      while (index1 <= 5);
    }

    pub fn CellularAutomatonEnemyMoveIn(int x, int y, AIMatrix blocked)
    {
      let mut index1: i32 =  0;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index1];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2 && blocked.Value[coordinate.x, coordinate.y] == 0)
        {
          let mut friendlyNeighbours: i32 =  this.CellularAutomatonGetNumberOfFriendlyNeighbours(coordinate.x, coordinate.y);
          let mut num1: i32 =  (int) Math.Round( this.troops.Value[coordinate.x, coordinate.y] /  (friendlyNeighbours + 1));
          this.offtroops.Value[x, y] = (int) Math.Round( (this.offtroops.Value[x, y] * this.troops.Value[x, y] + this.offtroops.Value[coordinate.x, coordinate.y] * num1) /  (num1 + this.troops.Value[x, y]));
          numArray1: Vec<i32> = this.troops.Value;
          numArray2: Vec<i32> = numArray1;
          let mut index2: i32 =  x;
          let mut index3: i32 =  index2;
          let mut index4: i32 =  y;
          let mut index5: i32 =  index4;
          let mut num2: i32 =  numArray1[index2, index4] + num1;
          numArray2[index3, index5] = num2;
          numArray3: Vec<i32> = this.troops.Value;
          numArray4: Vec<i32> = numArray3;
          let mut x1: i32 =  coordinate.x;
          let mut index6: i32 =  x1;
          let mut y1: i32 =  coordinate.y;
          let mut index7: i32 =  y1;
          let mut num3: i32 =  numArray3[x1, y1] - num1;
          numArray4[index6, index7] = num3;
          if (this.troops.Value[coordinate.x, coordinate.y] < 1)
            this.offtroops.Value[coordinate.x, coordinate.y] = 0;
        }
        index1 += 1;
      }
      while (index1 <= 5);
    }

    pub int CellularAutomatonGetNumberOfEnemyNeighbours(int x, int y)
    {
      let mut ofEnemyNeighbours: i32 =  0;
      let mut index: i32 =  0;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2)
          ofEnemyNeighbours += 1;
        index += 1;
      }
      while (index <= 5);
      return ofEnemyNeighbours;
    }

    pub int CellularAutomatonGetNumberOfFriendlyNeighbours(int x, int y)
    {
      let mut friendlyNeighbours: i32 =  0;
      let mut index: i32 =  0;
      do
      {
        Coordinate coordinate = this.ai.TempHexNeighbour[x, y, index];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1)
          friendlyNeighbours += 1;
        index += 1;
      }
      while (index <= 5);
      return friendlyNeighbours;
    }

    pub void CellularAutomatonModifyTroopsOn(
      int x,
      int y,
      AIFront front,
      int attLoss,
      AIMatrix Blocked)
    {
      let mut index1: i32 =  0;
      Coordinate coordinate;
      int num1;
      do
      {
        coordinate = this.ai.TempHexNeighbour[x, y, index1];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1 && Blocked.Value[coordinate.x, coordinate.y] == 0)
          num1 += 1;
        index1 += 1;
      }
      while (index1 <= 5);
      let mut num2: i32 =  (int) Math.Round( attLoss /  num1);
      let mut index2: i32 =  0;
      do
      {
        coordinate = this.ai.TempHexNeighbour[x, y, index2];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 1 && Blocked.Value[coordinate.x, coordinate.y] == 0)
        {
          numArray1: Vec<i32> = this.troops.Value;
          numArray2: Vec<i32> = numArray1;
          let mut x1: i32 =  coordinate.x;
          let mut index3: i32 =  x1;
          let mut y1: i32 =  coordinate.y;
          let mut index4: i32 =  y1;
          let mut num3: i32 =  numArray1[x1, y1] - num2;
          numArray2[index3, index4] = num3;
        }
        index2 += 1;
      }
      while (index2 <= 5);
    }

    pub fn CellularAutomatonModifyEnemyTroopsOn(int x, int y, int attLoss, AIMatrix Blocked)
    {
      let mut index1: i32 =  0;
      Coordinate coordinate;
      int num1;
      do
      {
        coordinate = this.ai.TempHexNeighbour[x, y, index1];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2 && Blocked.Value[coordinate.x, coordinate.y] == 0)
          num1 += 1;
        index1 += 1;
      }
      while (index1 <= 5);
      let mut num2: i32 =  (int) Math.Round(Conversion.Int( attLoss /  num1));
      let mut index2: i32 =  0;
      do
      {
        coordinate = this.ai.TempHexNeighbour[x, y, index2];
        if (coordinate.onmap && this.owner.Value[coordinate.x, coordinate.y] == 2 && Blocked.Value[coordinate.x, coordinate.y] == 0)
        {
          numArray1: Vec<i32> = this.troops.Value;
          numArray2: Vec<i32> = numArray1;
          let mut x1: i32 =  coordinate.x;
          let mut index3: i32 =  x1;
          let mut y1: i32 =  coordinate.y;
          let mut index4: i32 =  y1;
          let mut num3: i32 =  numArray1[x1, y1] - num2;
          numArray2[index3, index4] = num3;
        }
        index2 += 1;
      }
      while (index2 <= 5);
    }

    pub float CellularAutomatonGetMinimumAttackAdvantage(int stance, int ltnr, int riv)
    {
      float minimumAttackAdvantage = 5f;
      switch (stance)
      {
        case 1:
          minimumAttackAdvantage = 5f;
          break;
        case 2:
          minimumAttackAdvantage = 3.5f;
          break;
        case 3:
          minimumAttackAdvantage = 2f;
          break;
      }
      if (this.ai.game.Data.LandscapeTypeObj[ltnr].TempDefenseBonus > 0)
        minimumAttackAdvantage +=  ( minimumAttackAdvantage * 0.33 * ( this.ai.game.Data.LandscapeTypeObj[ltnr].TempDefenseBonus / 100.0));
      if (riv > -1 && this.ai.game.Data.RiverTypeObj[riv].TempDefenseBonus > 0)
        minimumAttackAdvantage +=  ( minimumAttackAdvantage * 0.33 * ( this.ai.game.Data.RiverTypeObj[riv].TempDefenseBonus / 100.0));
      return minimumAttackAdvantage;
    }

    pub int CellularAutomatonGetLossForAttacker(
      int stance,
      int attackPower,
      int defendPower,
      int ltnr,
      int rivtype,
      let mut realAttackerPower: i32 =  -1)
    {
      float num;
      switch (stance)
      {
        case 1:
          num =  (0.06 / Math.Min(5.0,  attackPower /  defendPower));
          break;
        case 2:
          num =  (0.06 / Math.Min(5.0,  attackPower /  defendPower));
          break;
        case 3:
          num =  (0.06 / Math.Min(5.0,  attackPower /  defendPower));
          break;
      }
      if (this.ai.game.Data.LandscapeTypeObj[ltnr].TempDefenseBonus > 0)
        num += num * ( this.ai.game.Data.LandscapeTypeObj[ltnr].TempDefenseBonus / 100f);
      if (rivtype > -1 && this.ai.game.Data.RiverTypeObj[rivtype].TempDefenseBonus > 0)
        num += num * ( this.ai.game.Data.RiverTypeObj[rivtype].TempDefenseBonus / 100f);
      int lossForAttacker;
      if (realAttackerPower > -1)
      {
        lossForAttacker = (int) Math.Round( ( realAttackerPower * num)) + 1;
        if (lossForAttacker > realAttackerPower)
          lossForAttacker = realAttackerPower;
      }
      else
      {
        lossForAttacker = (int) Math.Round( ( attackPower * num)) + 1;
        if (lossForAttacker > attackPower)
          lossForAttacker = attackPower;
      }
      return lossForAttacker;
    }

    pub int CellularAutomatonGetLossForDefender(int stance, int attackPower, int defendPower)
    {
      let mut num1: i32 =  attackPower;
      if (attackPower > this.ai.VAR_HEX_STACK_REGULAR * 4)
        attackPower = (int) Math.Round( (this.ai.VAR_HEX_STACK_REGULAR * 4) + 4.0 * Math.Sqrt( (attackPower - this.ai.VAR_HEX_STACK_REGULAR * 4)));
      if (defendPower > this.ai.VAR_HEX_STACK_REGULAR * 4)
        defendPower = (int) Math.Round( (this.ai.VAR_HEX_STACK_REGULAR * 4) + Math.Sqrt( (defendPower - this.ai.VAR_HEX_STACK_REGULAR * 4)));
      float num2;
      switch (stance)
      {
        case 1:
          num2 =  (0.012 * Math.Min(5.0,  attackPower /  defendPower));
          break;
        case 2:
          num2 =  (0.012 * Math.Min(5.0,  attackPower /  defendPower));
          break;
        case 3:
          num2 =  (0.012 * Math.Min(5.0,  attackPower /  defendPower));
          break;
      }
      let mut lossForDefender: i32 =  (int) Math.Round( ( num1 * num2)) + 1;
      if (lossForDefender > defendPower)
        lossForDefender = defendPower;
      return lossForDefender;
    }

    pub fn CellularAutomatonReserveDelivery(AIFront front, int iteration)
    {
      let mut num1: i32 =  4;
      if (front.tempDelivered || front.Distance >= iteration * num1)
        return;
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      int num2;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (this.frontline.Value[index1, index2] == front.TargetFrontID)
            num2 += 1;
        }
      }
      let mut counter: i32 =  front.units.counter;
      int num3;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(front.units.AIid[index]);
        if (unitByAiid > -1)
          num3 += this.ai.game.Data.UnitObj[unitByAiid].TempUnitPower;
      }
      float Number =  (int) Math.Round( ( num3 * this.FriendlyMod)) /  num2;
      let mut num4: i32 =  (int) Math.Round( Conversion.Int(Number));
      float num5 = Number -  num4;
      let mut mapWidth2: i32 =  this.ai.map.MapWidth;
      for (let mut index3: i32 =  0; index3 <= mapWidth2; index3 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
        {
          if (this.frontline.Value[index3, index4] == front.TargetFrontID)
          {
            numArray1: Vec<i32> = this.troops.Value;
            numArray2: Vec<i32> = numArray1;
            let mut index5: i32 =  index3;
            let mut index6: i32 =  index5;
            let mut index7: i32 =  index4;
            let mut index8: i32 =  index7;
            let mut num6: i32 =  numArray1[index5, index7] + num4;
            numArray2[index6, index8] = num6;
            if ( VBMath.Rnd() <  num5)
            {
              numArray3: Vec<i32> = this.troops.Value;
              numArray4: Vec<i32> = numArray3;
              let mut index9: i32 =  index3;
              let mut index10: i32 =  index9;
              let mut index11: i32 =  index4;
              let mut index12: i32 =  index11;
              let mut num7: i32 =  numArray3[index9, index11] + 1;
              numArray4[index10, index12] = num7;
            }
          }
        }
      }
      front.tempDelivered = true;
    }

    pub fn CellularAutomatonGiveEnemyReinforcements()
    {
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      int num1;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (this.owner.Value[index1, index2] == 2 & this.troops.Value[index1, index2] > 0 && this.enemySupply.Value[index1, index2] < this.ai.VAR_SUPPLY_50PERCENT_RANGE)
            num1 += 1;
        }
      }
      float Number =  (int) Math.Round( this.ai.VAR_REINFORCEMENTS_ENEMY /  num1);
      let mut num2: i32 =  (int) Math.Round( Conversion.Int(Number));
      float num3 = Number -  num2;
      let mut mapWidth2: i32 =  this.ai.map.MapWidth;
      for (let mut index3: i32 =  0; index3 <= mapWidth2; index3 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
        {
          if (this.owner.Value[index3, index4] == 2 & this.troops.Value[index3, index4] > 0 && this.enemySupply.Value[index3, index4] < this.ai.VAR_SUPPLY_50PERCENT_RANGE)
          {
            numArray1: Vec<i32> = this.troops.Value;
            numArray2: Vec<i32> = numArray1;
            let mut index5: i32 =  index3;
            let mut index6: i32 =  index5;
            let mut index7: i32 =  index4;
            let mut index8: i32 =  index7;
            let mut num4: i32 =  numArray1[index5, index7] + num2;
            numArray2[index6, index8] = num4;
            if ( VBMath.Rnd() <  num3)
            {
              numArray3: Vec<i32> = this.troops.Value;
              numArray4: Vec<i32> = numArray3;
              let mut index9: i32 =  index3;
              let mut index10: i32 =  index9;
              let mut index11: i32 =  index4;
              let mut index12: i32 =  index11;
              let mut num5: i32 =  numArray3[index9, index11] + 1;
              numArray4[index10, index12] = num5;
            }
          }
        }
      }
    }

    pub fn CellularAutomatonGiveFriendlyReinforcements()
    {
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      int num1;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (this.frontline.Value[index1, index2] > 0 && this.owner.Value[index1, index2] == 1 && this.friendlySupply.Value[index1, index2] < this.ai.VAR_SUPPLY_50PERCENT_RANGE)
            num1 += 1;
        }
      }
      float Number =  this.ai.VAR_REINFORCEMENTS_FRIENDLY /  num1;
      let mut num2: i32 =  (int) Math.Round( Conversion.Int(Number));
      float num3 = Number -  num2;
      let mut mapWidth2: i32 =  this.ai.map.MapWidth;
      for (let mut index3: i32 =  0; index3 <= mapWidth2; index3 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
        {
          if (this.frontline.Value[index3, index4] > 0 && this.owner.Value[index3, index4] == 1 && this.friendlySupply.Value[index3, index4] < this.ai.VAR_SUPPLY_50PERCENT_RANGE)
          {
            numArray1: Vec<i32> = this.troops.Value;
            numArray2: Vec<i32> = numArray1;
            let mut index5: i32 =  index3;
            let mut index6: i32 =  index5;
            let mut index7: i32 =  index4;
            let mut index8: i32 =  index7;
            let mut num4: i32 =  numArray1[index5, index7] + num2;
            numArray2[index6, index8] = num4;
            if ( VBMath.Rnd() <  num3)
            {
              numArray3: Vec<i32> = this.troops.Value;
              numArray4: Vec<i32> = numArray3;
              let mut index9: i32 =  index3;
              let mut index10: i32 =  index9;
              let mut index11: i32 =  index4;
              let mut index12: i32 =  index11;
              let mut num5: i32 =  numArray3[index9, index11] + 1;
              numArray4[index10, index12] = num5;
            }
          }
        }
      }
    }

    pub fn CellularAutomatonRemoveFrontsWithoutFrontline(ref AIFrontList frontList)
    {
      let mut counter: i32 =  frontList.Counter;
      for (let mut index1: i32 =  0; index1 <= counter; index1 += 1)
      {
        if (frontList.Front[index1].FrontType == 1)
        {
          let mut num: i32 =  0;
          let mut mapWidth1: i32 =  this.ai.map.MapWidth;
          for (let mut index2: i32 =  0; index2 <= mapWidth1; index2 += 1)
          {
            let mut mapHeight: i32 =  this.ai.map.MapHeight;
            for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            {
              if (this.frontline.Value[index2, index3] == frontList.Front[index1].FrontID)
              {
                let mut index4: i32 =  0;
                do
                {
                  Coordinate coordinate = this.ai.TempHexNeighbour[index2, index3, index4];
                  if (this.owner.Value[coordinate.x, coordinate.y] == 2)
                    num = 1;
                  index4 += 1;
                }
                while (index4 <= 5);
                if (num == 1)
                  break;
              }
            }
            if (num == 1)
              break;
          }
          if (num == 0)
          {
            let mut mapWidth2: i32 =  this.ai.map.MapWidth;
            for (let mut index5: i32 =  0; index5 <= mapWidth2; index5 += 1)
            {
              let mut mapHeight: i32 =  this.ai.map.MapHeight;
              for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
              {
                if (this.frontline.Value[index5, index6] == frontList.Front[index1].FrontID)
                  this.frontline.Value[index5, index6] = 0;
              }
            }
          }
        }
      }
    }

    pub fn CellularAutomatonMoveTowardsFront(int range, int mindist)
    {
      AIMatrix aiMatrix1 = new AIMatrix(ref this.ai);
      AIMatrix aiMatrix2 = this.owner.Clone();
      aiMatrix2.SetAllValuesNotValueXTo(0, 2);
      aiMatrix2.ExpandAndAddValueForAnyRegime(9999);
      AIMatrix aiMatrix3 = new AIMatrix(ref this.ai);
      int num1;
      int num2;
      do
      {
        num1 = 0;
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut x1: i32 =  0; x1 <= mapWidth; x1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
          {
            if (this.frontline.Value[x1, y1] == 0 | aiMatrix2.Value[x1, y1] > 3 + mindist && this.troops.Value[x1, y1] > 0 & this.owner.Value[x1, y1] == 1)
            {
              let mut num3: i32 =  999999;
              let mut num4: i32 =  -1;
              let mut num5: i32 =  -1;
              let mut num6: i32 =  x1 - (range + 1);
              let mut num7: i32 =  x1 + (range + 1);
              for (let mut index1: i32 =  num6; index1 <= num7; index1 += 1)
              {
                let mut num8: i32 =  y1 - (range + 1);
                let mut num9: i32 =  y1 + (range + 1);
                for (let mut index2: i32 =  num8; index2 <= num9; index2 += 1)
                {
                  let mut x2: i32 =  index1;
                  let mut y2: i32 =  index2;
                  if (this.ai.map.MapLoop)
                  {
                    if (x2 < 0)
                      x2 = this.ai.map.MapWidth + 1 - x2;
                    if (x2 > this.ai.map.MapWidth)
                      x2 -= this.ai.map.MapWidth + 1;
                  }
                  if (x2 >= 0 & y2 >= 0 & x2 <= this.ai.map.MapWidth & y2 <= this.ai.map.MapHeight && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[x2, y2].LandscapeType].AIBlock < 1)
                  {
                    let mut num10: i32 =  this.ai.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                    if (num10 > 0 & num10 <= range && this.owner.Value[x2, y2] == 1 && aiMatrix2.Value[x2, y2] < num3)
                    {
                      num3 = aiMatrix2.Value[x2, y2];
                      num4 = x2;
                      num5 = y2;
                    }
                  }
                }
              }
              if (num4 > -1)
              {
                let mut num11: i32 =  this.troops.Value[x1, y1];
                numArray1: Vec<i32> = this.troops.Value;
                numArray2: Vec<i32> = numArray1;
                let mut index3: i32 =  x1;
                let mut index4: i32 =  index3;
                let mut index5: i32 =  y1;
                let mut index6: i32 =  index5;
                let mut num12: i32 =  numArray1[index3, index5] - num11;
                numArray2[index4, index6] = num12;
                numArray3: Vec<i32> = aiMatrix3.Value;
                numArray4: Vec<i32> = numArray3;
                let mut index7: i32 =  num4;
                let mut index8: i32 =  index7;
                let mut index9: i32 =  num5;
                let mut index10: i32 =  index9;
                let mut num13: i32 =  numArray3[index7, index9] + num11;
                numArray4[index8, index10] = num13;
                num1 = 1;
              }
            }
          }
        }
        num2 += 1;
      }
      while (num1 > 0 & num2 < 5);
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      for (let mut index11: i32 =  0; index11 <= mapWidth1; index11 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index12: i32 =  0; index12 <= mapHeight; index12 += 1)
        {
          numArray5: Vec<i32> = this.troops.Value;
          numArray6: Vec<i32> = numArray5;
          let mut index13: i32 =  index11;
          let mut index14: i32 =  index13;
          let mut index15: i32 =  index12;
          let mut index16: i32 =  index15;
          let mut num14: i32 =  numArray5[index13, index15] + aiMatrix3.Value[index11, index12];
          numArray6[index14, index16] = num14;
        }
      }
    }

    pub fn CellularAutomatonMoveEnemyTowardsFront(int range, int mindist)
    {
      AIMatrix aiMatrix1 = new AIMatrix(ref this.ai);
      AIMatrix aiMatrix2 = this.owner.Clone();
      aiMatrix2.SetAllValuesNotValueXTo(0, 1);
      aiMatrix2.ExpandAndAddValueForAnyRegime(25);
      AIMatrix aiMatrix3 = new AIMatrix(ref this.ai);
      int num1;
      int num2;
      do
      {
        num1 = 0;
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut x1: i32 =  0; x1 <= mapWidth; x1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
          {
            if (aiMatrix2.Value[x1, y1] > mindist + 1 && this.troops.Value[x1, y1] > 0 & this.owner.Value[x1, y1] == 2)
            {
              let mut num3: i32 =  999999;
              let mut index1: i32 =  -1;
              let mut index2: i32 =  -1;
              let mut num4: i32 =  x1 - (range + 1);
              let mut num5: i32 =  x1 + (range + 1);
              for (let mut index3: i32 =  num4; index3 <= num5; index3 += 1)
              {
                let mut num6: i32 =  y1 - (range + 1);
                let mut num7: i32 =  y1 + (range + 1);
                for (let mut index4: i32 =  num6; index4 <= num7; index4 += 1)
                {
                  let mut x2: i32 =  index3;
                  let mut y2: i32 =  index4;
                  if (this.ai.map.MapLoop)
                  {
                    if (x2 < 0)
                      x2 = this.ai.map.MapWidth + 1 - x2;
                    if (x2 > this.ai.map.MapWidth)
                      x2 -= this.ai.map.MapWidth + 1;
                  }
                  if (x2 >= 0 & y2 >= 0 & x2 <= this.ai.map.MapWidth & y2 <= this.ai.map.MapHeight && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[x2, y2].LandscapeType].AIBlock < 1)
                  {
                    let mut num8: i32 =  this.ai.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                    if (num8 > 0 & num8 <= range && this.owner.Value[x2, y2] == 2 && aiMatrix2.Value[x2, y2] * 100 + this.vp.Value[x2, y2] < num3)
                    {
                      num3 = aiMatrix2.Value[x2, y2] * 100 + this.vp.Value[x2, y2];
                      index1 = x2;
                      index2 = y2;
                    }
                  }
                }
              }
              if (index1 > -1)
              {
                let mut num9: i32 =  this.troops.Value[x1, y1];
                this.offtroops.Value[index1, index2] = (int) Math.Round( (this.offtroops.Value[index1, index2] * this.troops.Value[index1, index2]) +  (this.offtroops.Value[x1, y1] * num9) /  (this.offtroops.Value[index1, index2] + num9));
                numArray1: Vec<i32> = this.troops.Value;
                numArray2: Vec<i32> = numArray1;
                let mut index5: i32 =  x1;
                let mut index6: i32 =  index5;
                let mut index7: i32 =  y1;
                let mut index8: i32 =  index7;
                let mut num10: i32 =  numArray1[index5, index7] - num9;
                numArray2[index6, index8] = num10;
                numArray3: Vec<i32> = aiMatrix3.Value;
                numArray4: Vec<i32> = numArray3;
                let mut index9: i32 =  index1;
                let mut index10: i32 =  index9;
                let mut index11: i32 =  index2;
                let mut index12: i32 =  index11;
                let mut num11: i32 =  numArray3[index9, index11] + num9;
                numArray4[index10, index12] = num11;
                if (this.troops.Value[x1, y1] < 1)
                  this.offtroops.Value[x1, y1] = 0;
                num1 = 1;
              }
            }
          }
        }
        num2 += 1;
      }
      while (num1 > 0 & num2 < 5);
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      for (let mut index13: i32 =  0; index13 <= mapWidth1; index13 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
        {
          numArray5: Vec<i32> = this.troops.Value;
          numArray6: Vec<i32> = numArray5;
          let mut index15: i32 =  index13;
          let mut index16: i32 =  index15;
          let mut index17: i32 =  index14;
          let mut index18: i32 =  index17;
          let mut num12: i32 =  numArray5[index15, index17] + aiMatrix3.Value[index13, index14];
          numArray6[index16, index18] = num12;
        }
      }
    }

    pub fn CellularAutomatonSpreadOut(AIFront front, int range)
    {
      AIMatrix aiMatrix1 = new AIMatrix(ref this.ai);
      float num1 = 0.2f;
      let mut num2: i32 =  1;
      AIMatrix aiMatrix2 = this.troops.Clone();
      aiMatrix2.RemoveValuesByNotMask(this.owner, 2);
      AIMatrix troops = aiMatrix2.AverageValuesForSameRegime(2, this.owner, OnlyOwnerX: 2);
      troops.AddValue(this.troops, 1);
      troops.SetValueXToValueY(0, 1);
      troops.RemoveValuesByNotMask(this.owner, 2);
      AIMatrix enemy = this.ai.SetEnemyPressureMatrix(this.owner, troops, this.frontline, front.FrontID);
      AIMatrix aiMatrix3 = this.ai.SetRatioInPercentage(ref this.troops, ref enemy, this.frontline, front.FrontID);
      aiMatrix3.RemoveValueByPercentage(this.friendlyBottleneck);
      aiMatrix3.RemoveValueByPercentage(this.friendlyBottleneck);
      AIMatrix aiMatrix4 = new AIMatrix(ref this.ai);
      let mut num3: i32 =  front.units.counter + 1;
      if (this.ai.game.Data.Product >= 6)
        num3 *= 2;
      if (num3 < 0)
        num3 = 0;
      int num4;
      int num5;
      do
      {
        num4 = 0;
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut x1: i32 =  0; x1 <= mapWidth; x1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
          {
            if (x1 == 54 & y1 == 13)
              x1 = x1;
            if (this.frontline.Value[x1, y1] == front.FrontID & this.owner.Value[x1, y1] == 1 && this.troops.Value[x1, y1] > 0)
            {
              let mut num6: i32 =  999999;
              let mut index1: i32 =  -1;
              let mut index2: i32 =  -1;
              let mut num7: i32 =  x1 - (range + 1);
              let mut num8: i32 =  x1 + (range + 1);
              for (let mut index3: i32 =  num7; index3 <= num8; index3 += 1)
              {
                let mut num9: i32 =  y1 - (range + 1);
                let mut num10: i32 =  y1 + (range + 1);
                for (let mut index4: i32 =  num9; index4 <= num10; index4 += 1)
                {
                  let mut x2: i32 =  index3;
                  let mut y2: i32 =  index4;
                  if (this.ai.map.MapLoop)
                  {
                    if (x2 < 0)
                      x2 = this.ai.map.MapWidth + 1 - x2;
                    if (x2 > this.ai.map.MapWidth)
                      x2 -= this.ai.map.MapWidth + 1;
                  }
                  if (x2 >= 0 & y2 >= 0 & x2 <= this.ai.map.MapWidth & y2 <= this.ai.map.MapHeight && this.frontline.Value[x2, y2] == front.FrontID | this.ai.game.Data.Product < 6 && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[x2, y2].LandscapeType].AIBlock < 1)
                  {
                    let mut num11: i32 =  this.ai.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                    int num12;
                    if (num11 > 0 & num11 <= range)
                    {
                      if (this.owner.Value[x2, y2] == 1)
                      {
                        if (enemy.Value[x2, y2] > 0 & aiMatrix3.Value[x2, y2] - this.friendlyBottleneck.Value[x2, y2] < num6 && this.friendlySupply.Value[x2, y2] < this.ai.VAR_SUPPLY_25PERCENT_RANGE)
                        {
                          if (num3 > 0 | this.troops.Value[x2, y2] + aiMatrix4.Value[x2, y2] > 0)
                          {
                            num6 = aiMatrix3.Value[x2, y2] - this.friendlyBottleneck.Value[x2, y2];
                            index1 = x2;
                            index2 = y2;
                          }
                          else
                            num12 = x2;
                        }
                      }
                      else
                        num12 = x2;
                    }
                  }
                }
              }
              if (index1 > -1 && num6 + aiMatrix4.Value[index1, index2] < aiMatrix3.Value[x1, y1])
              {
                if (this.troops.Value[index1, index2] + aiMatrix4.Value[index1, index2] < 1)
                  --num3;
                let mut num13: i32 =  (int) Math.Round( (num1 *  this.troops.Value[x1, y1]));
                if (num13 < num2)
                  num13 = this.troops.Value[x1, y1] < num2 ? this.troops.Value[x1, y1] : num2;
                if (this.frontline.Value[index1, index2] == 0)
                  this.frontline.Value[index1, index2] = front.FrontID;
                numArray1: Vec<i32> = this.troops.Value;
                numArray2: Vec<i32> = numArray1;
                let mut index5: i32 =  x1;
                let mut index6: i32 =  index5;
                let mut index7: i32 =  y1;
                let mut index8: i32 =  index7;
                let mut num14: i32 =  numArray1[index5, index7] - num13;
                numArray2[index6, index8] = num14;
                numArray3: Vec<i32> = aiMatrix4.Value;
                numArray4: Vec<i32> = numArray3;
                let mut index9: i32 =  index1;
                let mut index10: i32 =  index9;
                let mut index11: i32 =  index2;
                let mut index12: i32 =  index11;
                let mut num15: i32 =  numArray3[index9, index11] + num13;
                numArray4[index10, index12] = num15;
                aiMatrix3.Value[x1, y1] = enemy.Value[x1, y1] != 0 ? (int) Math.Round(100.0 * ( this.troops.Value[x1, y1] /  enemy.Value[x1, y1])) : 999;
                aiMatrix3.Value[index1, index2] = enemy.Value[index1, index2] != 0 ? (int) Math.Round(100.0 * ( (this.troops.Value[index1, index2] + aiMatrix4.Value[index1, index2]) /  enemy.Value[index1, index2])) : 999;
                num4 = 1;
              }
            }
          }
        }
        num5 += 1;
      }
      while (num4 > 0 & num5 < 5);
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      for (let mut index13: i32 =  0; index13 <= mapWidth1; index13 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
        {
          numArray5: Vec<i32> = this.troops.Value;
          numArray6: Vec<i32> = numArray5;
          let mut index15: i32 =  index13;
          let mut index16: i32 =  index15;
          let mut index17: i32 =  index14;
          let mut index18: i32 =  index17;
          let mut num16: i32 =  numArray5[index15, index17] + aiMatrix4.Value[index13, index14];
          numArray6[index16, index18] = num16;
        }
      }
    }

    pub fn CellularAutomatonFallBack(AIFront front, int MAX_NUMBER_OF_STEPS)
    {
      AIMatrix aiMatrix1 = new AIMatrix(ref this.ai);
      let mut num1: i32 =  99999;
      AIMatrix enemy = this.ai.SetEnemyPressureMatrix(this.owner, this.troops, this.frontline, front.FrontID);
      AIMatrix aiMatrix2 = this.ai.SetRatioInPercentage(ref this.troops, ref enemy, this.frontline, front.FrontID);
      AIMatrix aiMatrix3 = new AIMatrix(ref this.ai);
      int num2;
      int num3;
      do
      {
        num2 = 0;
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut x1: i32 =  0; x1 <= mapWidth; x1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
          {
            if (this.frontline.Value[x1, y1] == front.FrontID & this.owner.Value[x1, y1] == 1 && this.troops.Value[x1, y1] > 0 & aiMatrix2.Value[x1, y1] < num1)
            {
              let mut num4: i32 =  this.friendlySupplyRoute.Value[x1, y1];
              let mut index1: i32 =  -1;
              let mut index2: i32 =  -1;
              let mut num5: i32 =  x1 - MAX_NUMBER_OF_STEPS;
              let mut num6: i32 =  x1 + MAX_NUMBER_OF_STEPS;
              for (let mut x2: i32 =  num5; x2 <= num6; x2 += 1)
              {
                let mut num7: i32 =  y1 - MAX_NUMBER_OF_STEPS;
                let mut num8: i32 =  y1 + MAX_NUMBER_OF_STEPS;
                for (let mut y2: i32 =  num7; y2 <= num8; y2 += 1)
                {
                  if (x2 >= 0 & y2 >= 0 & x2 <= this.ai.map.MapWidth & y2 <= this.ai.map.MapHeight && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[x2, y2].LandscapeType].AIBlock < 1)
                  {
                    let mut num9: i32 =  this.ai.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                    if (num9 > 0 & num9 <= MAX_NUMBER_OF_STEPS && this.owner.Value[x2, y2] == 1 && this.friendlySupplyRoute.Value[x2, y2] + aiMatrix3.Value[x2, y2] < num4 & !(x2 == x1 & y2 == y1))
                    {
                      num4 = this.friendlySupplyRoute.Value[x2, y2] + aiMatrix3.Value[x2, y2];
                      index1 = x2;
                      index2 = y2;
                    }
                  }
                }
              }
              if (index1 > -1)
              {
                let mut num10: i32 =  this.troops.Value[x1, y1];
                numArray1: Vec<i32> = this.troops.Value;
                numArray2: Vec<i32> = numArray1;
                let mut index3: i32 =  x1;
                let mut index4: i32 =  index3;
                let mut index5: i32 =  y1;
                let mut index6: i32 =  index5;
                let mut num11: i32 =  numArray1[index3, index5] - num10;
                numArray2[index4, index6] = num11;
                if (0 > this.troops.Value[x1, y1])
                  this.troops.Value[x1, y1] = 0;
                numArray3: Vec<i32> = aiMatrix3.Value;
                numArray4: Vec<i32> = numArray3;
                let mut index7: i32 =  index1;
                let mut index8: i32 =  index7;
                let mut index9: i32 =  index2;
                let mut index10: i32 =  index9;
                let mut num12: i32 =  numArray3[index7, index9] + num10;
                numArray4[index8, index10] = num12;
                aiMatrix2.Value[x1, y1] = enemy.Value[x1, y1] != 0 ? (int) Math.Round(100.0 * ( this.troops.Value[x1, y1] /  enemy.Value[x1, y1])) : 999;
                aiMatrix2.Value[index1, index2] = enemy.Value[index1, index2] != 0 ? (int) Math.Round(100.0 * ( (this.troops.Value[index1, index2] + num10) /  enemy.Value[index1, index2])) : 999;
                num2 = 1;
              }
            }
          }
        }
        num3 += 1;
      }
      while (num2 > 0 & num3 < MAX_NUMBER_OF_STEPS);
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      for (let mut index11: i32 =  0; index11 <= mapWidth1; index11 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index12: i32 =  0; index12 <= mapHeight; index12 += 1)
        {
          if (0 > this.troops.Value[index11, index12])
            this.troops.Value[index11, index12] = 0;
          numArray5: Vec<i32> = this.troops.Value;
          numArray6: Vec<i32> = numArray5;
          let mut index13: i32 =  index11;
          let mut index14: i32 =  index13;
          let mut index15: i32 =  index12;
          let mut index16: i32 =  index15;
          let mut num13: i32 =  numArray5[index13, index15] + aiMatrix3.Value[index11, index12];
          numArray6[index14, index16] = num13;
        }
      }
    }

    pub fn CellularAutomatonEnemySpreadOut(int range)
    {
      AIMatrix aiMatrix1 = new AIMatrix(ref this.ai);
      float num1 = 0.2f;
      let mut num2: i32 =  5;
      AIMatrix friendly = this.ai.SetFriendlyPressureMatrix(this.owner, this.troops);
      AIMatrix aiMatrix2 = this.ai.SetRatioForEnemyInPercentage(ref this.troops, ref friendly, ref this.owner);
      AIMatrix aiMatrix3 = new AIMatrix(ref this.ai);
      int num3;
      int num4;
      do
      {
        num3 = 0;
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut x1: i32 =  0; x1 <= mapWidth; x1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
          {
            if (this.owner.Value[x1, y1] == 2 && this.troops.Value[x1, y1] > 0)
            {
              let mut num5: i32 =  999999;
              let mut index1: i32 =  -1;
              let mut index2: i32 =  -1;
              let mut num6: i32 =  x1 - (range + 1);
              let mut num7: i32 =  x1 + (range + 1);
              for (let mut index3: i32 =  num6; index3 <= num7; index3 += 1)
              {
                let mut num8: i32 =  y1 - (range + 1);
                let mut num9: i32 =  y1 + (range + 1);
                for (let mut index4: i32 =  num8; index4 <= num9; index4 += 1)
                {
                  let mut x2: i32 =  index3;
                  let mut y2: i32 =  index4;
                  if (this.ai.map.MapLoop)
                  {
                    if (x2 < 0)
                      x2 = this.ai.map.MapWidth + 1 - x2;
                    if (x2 > this.ai.map.MapWidth)
                      x2 -= this.ai.map.MapWidth + 1;
                  }
                  if (x2 >= 0 & y2 >= 0 & x2 <= this.ai.map.MapWidth & y2 <= this.ai.map.MapHeight && this.ai.game.Data.LandscapeTypeObj[this.ai.map.HexObj[x2, y2].LandscapeType].AIBlock < 1)
                  {
                    let mut num10: i32 =  this.ai.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                    if (num10 > 0 & num10 <= range && this.owner.Value[x2, y2] == 2 & aiMatrix2.Value[x2, y2] > 0)
                    {
                      let mut num11: i32 =  this.friendlyBottleneck.Value[x2, y2] - this.friendlyBottleneck.Value[x1, y1];
                      if (num11 > 10)
                        num11 = 10;
                      if (num11 < -10)
                        num11 = -10;
                      float num12 = 10f;
                      if (num11 > 0)
                        num12 +=  num11 / 1.25f;
                      if (num11 < 0)
                        num12 -=  Math.Abs(num11) / 1.25f;
                      if ( (aiMatrix2.Value[x2, y2] * 100) /  num12 <  num5)
                      {
                        num5 = (int) Math.Round( ( (aiMatrix2.Value[x2, y2] * 100) / num12));
                        index1 = x2;
                        index2 = y2;
                      }
                    }
                  }
                }
              }
              if (index1 > -1 && aiMatrix3.Value[index1, index2] + (int) Math.Round( (num1 *  aiMatrix2.Value[x1, y1])) < aiMatrix2.Value[x1, y1] - (int) Math.Round( (num1 *  aiMatrix2.Value[x1, y1])))
              {
                let mut num13: i32 =  (int) Math.Round( (num1 *  this.troops.Value[x1, y1]));
                if (num13 < num2)
                  num13 = this.troops.Value[x1, y1] < num2 ? this.troops.Value[x1, y1] : num2;
                this.offtroops.Value[index1, index2] = (int) Math.Round( (this.offtroops.Value[index1, index2] * this.troops.Value[index1, index2] + this.offtroops.Value[x1, y1] * this.troops.Value[x1, y1]) /  (this.troops.Value[x1, y1] + this.troops.Value[index1, index2]));
                numArray1: Vec<i32> = this.troops.Value;
                numArray2: Vec<i32> = numArray1;
                let mut index5: i32 =  x1;
                let mut index6: i32 =  index5;
                let mut index7: i32 =  y1;
                let mut index8: i32 =  index7;
                let mut num14: i32 =  numArray1[index5, index7] - num13;
                numArray2[index6, index8] = num14;
                if (this.troops.Value[x1, y1] < 1)
                  this.offtroops.Value[x1, y1] = 0;
                numArray3: Vec<i32> = aiMatrix3.Value;
                numArray4: Vec<i32> = numArray3;
                let mut index9: i32 =  index1;
                let mut index10: i32 =  index9;
                let mut index11: i32 =  index2;
                let mut index12: i32 =  index11;
                let mut num15: i32 =  numArray3[index9, index11] + num13;
                numArray4[index10, index12] = num15;
                aiMatrix2.Value[x1, y1] = friendly.Value[x1, y1] != 0 ? (int) Math.Round(100.0 * ( this.troops.Value[x1, y1] /  friendly.Value[x1, y1])) : 999;
                aiMatrix2.Value[index1, index2] = friendly.Value[index1, index2] != 0 ? (int) Math.Round(100.0 * ( (this.troops.Value[index1, index2] + aiMatrix3.Value[index1, index2]) /  friendly.Value[index1, index2])) : 999;
                num3 = 1;
              }
            }
          }
        }
        num4 += 1;
      }
      while (num3 > 0 & num4 < 20);
      let mut mapWidth1: i32 =  this.ai.map.MapWidth;
      for (let mut index13: i32 =  0; index13 <= mapWidth1; index13 += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
        {
          numArray5: Vec<i32> = this.troops.Value;
          numArray6: Vec<i32> = numArray5;
          let mut index15: i32 =  index13;
          let mut index16: i32 =  index15;
          let mut index17: i32 =  index14;
          let mut index18: i32 =  index17;
          let mut num16: i32 =  numArray5[index15, index17] + aiMatrix3.Value[index13, index14];
          numArray6[index16, index18] = num16;
        }
      }
    }
  }
}
