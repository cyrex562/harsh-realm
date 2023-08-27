// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIFront
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class AIFront
  {
    pub ai: DC2AIClass;
    pub FrontID: i32;
    pub AIUnitList strictHQs;
    pub AIUnitList units;
    pub AIUnitList artUnits;
    pub AIUnitList orgUnits;
    pub FrontType: i32;
    pub AIUnitList removelist;
    pub Stance: i32;
    pub Direction: i32;
    pub float OrigAverageStrength;
    pub float AverageStrength;
    pub addedFictivePower: i32;
    pub Strength: i32;
    pub OrigPower: i32;
    pub PowerToReserve: i32;
    pub TargetFrontID: i32;
    pub Distance: i32;
    pub StartStance: i32;
    pub OffensiveZone: i32;
    pub DefensiveZone: i32;
    pub OffensiveZone2: i32;
    pub enemyPower: i32;
    pub tempReservesLeaving: i32;
    pub tempDelivered: bool;
    pub Importance: i32;
    pub tempCreatedFromFrontID: i32;
    pub temp1: i32;
    pub OffensiveModifier: i32;
    pub TopOperation: bool;
    pub StatLastPercentageOutOfSupply: i32;
    pub StatAvgPercentageOutOfSupply: i32;
    pub statLastPowerPercentageRun1: i32;
    pub StatLastPowerPercentageFullRun: i32;
    pub StatIterationCount: i32;
    pub statsHexLeftPercentage: i32;
    pub Coordinate[] Coords;
    pub coordCount: i32;
    pub float UnitCountRatio;
    pub targetX: i32;
    pub targetY: i32;
    pub enemyAvgSupply: i32;
    pub FrontHexes: i32;
    pub RealRetreat: bool;
    pub bridgeCount: i32;
    pub vpScoreAveragePercent: i32;
    pub retreatAverageScore: i32;
    pub ThreatPercentage: i32;
    pub OpportunityPercentage: i32;
    pub tempEnemyRegime: i32;
    pub tempEnemyRegime2: i32;
    pub hasSupplySource: bool;
    pub tempOrigAutomatonPower: i32;
    pub receivedJoin: i32;
    pub strengthModifier: i32;

    pub AIFront Clone()
    {
      AIFront aiFront = new AIFront(ref this.ai, this.FrontType);
      aiFront.Direction = this.Direction;
      aiFront.FrontID = this.FrontID;
      aiFront.Stance = this.Stance;
      aiFront.enemyAvgSupply = this.enemyAvgSupply;
      aiFront.StartStance = this.StartStance;
      aiFront.OffensiveModifier = this.OffensiveModifier;
      aiFront.Strength = this.Strength;
      aiFront.enemyPower = this.enemyPower;
      aiFront.AverageStrength = this.AverageStrength;
      aiFront.FrontType = this.FrontType;
      aiFront.RealRetreat = this.RealRetreat;
      aiFront.TargetFrontID = this.TargetFrontID;
      aiFront.Distance = this.Distance;
      aiFront.temp1 = this.temp1;
      aiFront.OrigAverageStrength = this.OrigAverageStrength;
      aiFront.StatAvgPercentageOutOfSupply = this.StatAvgPercentageOutOfSupply;
      aiFront.StatLastPercentageOutOfSupply = this.StatLastPercentageOutOfSupply;
      aiFront.StatIterationCount = this.StatIterationCount;
      aiFront.statsHexLeftPercentage = this.statsHexLeftPercentage;
      aiFront.Importance = this.Importance;
      aiFront.UnitCountRatio = this.UnitCountRatio;
      aiFront.TopOperation = this.TopOperation;
      aiFront.OffensiveZone = this.OffensiveZone;
      aiFront.OffensiveZone2 = this.OffensiveZone2;
      aiFront.FrontHexes = this.FrontHexes;
      aiFront.hasSupplySource = this.hasSupplySource;
      aiFront.DefensiveZone = this.DefensiveZone;
      aiFront.addedFictivePower = this.addedFictivePower;
      aiFront.tempCreatedFromFrontID = this.tempCreatedFromFrontID;
      aiFront.units = AIUnitList::new();
      aiFront.artUnits = AIUnitList::new();
      aiFront.orgUnits = AIUnitList::new();
      aiFront.strictHQs = AIUnitList::new();
      let mut counter1: i32 =  this.units.counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
        aiFront.units.add(this.units.unr[index], this.units.AIid[index]);
      let mut counter2: i32 =  this.artUnits.counter;
      for (let mut index: i32 =  0; index <= counter2; index += 1)
        aiFront.artUnits.add(this.artUnits.unr[index], this.artUnits.AIid[index]);
      let mut counter3: i32 =  this.orgUnits.counter;
      for (let mut index: i32 =  0; index <= counter3; index += 1)
        aiFront.orgUnits.add(this.orgUnits.unr[index], this.orgUnits.AIid[index]);
      let mut counter4: i32 =  this.strictHQs.counter;
      for (let mut index: i32 =  0; index <= counter4; index += 1)
        aiFront.strictHQs.add(this.strictHQs.unr[index], this.strictHQs.AIid[index]);
      aiFront.removelist = (AIUnitList) null;
      aiFront.coordCount = this.coordCount;
      if (this.coordCount == -1)
        aiFront.Coords = new Coordinate[1];
      aiFront.Coords = new Coordinate[this.coordCount + 1];
      let mut coordCount: i32 =  this.coordCount;
      for (let mut index: i32 =  0; index <= coordCount; index += 1)
        aiFront.Coords[index] = this.Coords[index];
      aiFront.targetX = this.targetX;
      aiFront.targetY = this.targetY;
      aiFront.bridgeCount = this.bridgeCount;
      aiFront.vpScoreAveragePercent = this.vpScoreAveragePercent;
      aiFront.retreatAverageScore = this.retreatAverageScore;
      aiFront.ThreatPercentage = this.ThreatPercentage;
      aiFront.OpportunityPercentage = this.OpportunityPercentage;
      aiFront.tempEnemyRegime = this.tempEnemyRegime;
      aiFront.tempEnemyRegime2 = this.tempEnemyRegime2;
      aiFront.strengthModifier = this.strengthModifier;
      aiFront.receivedJoin = this.receivedJoin;
      return aiFront;
    }

    pub AIFront()
    {
      this.Coords = new Coordinate[1];
      this.tempEnemyRegime = -1;
      this.tempEnemyRegime2 = -1;
      this.hasSupplySource = false;
      this.tempOrigAutomatonPower = 0;
      this.receivedJoin = 0;
      this.TargetFrontID = -1;
      this.units = AIUnitList::new();
      this.artUnits = AIUnitList::new();
      this.orgUnits = AIUnitList::new();
      this.strictHQs = AIUnitList::new();
      this.OffensiveZone = -1;
      this.OffensiveZone2 = -1;
      this.addedFictivePower = 0;
      this.DefensiveZone = -1;
      this.coordCount = -1;
      this.targetX = -1;
      this.enemyPower = 0;
      this.targetY = -1;
      this.temp1 = 0;
      this.bridgeCount = 0;
      this.vpScoreAveragePercent = 50;
      this.retreatAverageScore = 100;
      this.tempEnemyRegime = -1;
      this.tempEnemyRegime2 = -1;
      this.enemyAvgSupply = 0;
      this.strengthModifier = 100;
      this.receivedJoin = 0;
    }

    pub SimpleList GetNeighbourFrontList()
    {
      SimpleList neighbourFrontList = SimpleList::new();
      let mut mapWidth: i32 =  this.ai.map.MapWidth;
      for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
        {
          if (this.ai.frontMatrix.Value[cx, cy] == this.FrontID)
          {
            let mut tfacing: i32 =  1;
            do
            {
              Coordinate coordinate = this.ai.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
              if (coordinate.onmap && this.ai.frontMatrix.Value[coordinate.x, coordinate.y] != this.FrontID & this.ai.frontMatrix.Value[coordinate.x, coordinate.y] > 0 && neighbourFrontList.FindNr(this.ai.frontMatrix.Value[coordinate.x, coordinate.y]) == -1)
                neighbourFrontList.Add(this.ai.frontMatrix.Value[coordinate.x, coordinate.y], 0);
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
        }
      }
      return neighbourFrontList;
    }

    pub HasFriendlyZeroBorder: bool()
    {
      SimpleList simpleList = SimpleList::new();
      let mut mapWidth: i32 =  this.ai.map.MapWidth;
      for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
      {
        let mut mapHeight: i32 =  this.ai.map.MapHeight;
        for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
        {
          if (this.ai.frontMatrix.Value[cx, cy] == this.FrontID)
          {
            let mut tfacing: i32 =  1;
            do
            {
              Coordinate coordinate = this.ai.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
              if (coordinate.onmap && this.ai.frontMatrix.Value[coordinate.x, coordinate.y] == 0 && this.ai.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == this.ai.game.Data.Turn && this.ai.friendlySupplyMatrix.Value[coordinate.x, coordinate.y] < this.ai.VAR_SUPPLY_MAXIMUM_RANGE)
                return true;
              tfacing += 1;
            }
            while (tfacing <= 6);
          }
        }
      }
      return false;
    }

    pub fn CopyUnitsFromAIFront(AIFront tempFront)
    {
      this.units = AIUnitList::new();
      let mut counter1: i32 =  tempFront.units.counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
        this.units.add(tempFront.units.unr[index], tempFront.units.AIid[index]);
      let mut counter2: i32 =  tempFront.artUnits.counter;
      for (let mut index: i32 =  0; index <= counter2; index += 1)
        this.artUnits.add(tempFront.artUnits.unr[index], tempFront.artUnits.AIid[index]);
      let mut counter3: i32 =  tempFront.orgUnits.counter;
      for (let mut index: i32 =  0; index <= counter3; index += 1)
        this.orgUnits.add(tempFront.orgUnits.unr[index], tempFront.orgUnits.AIid[index]);
    }

    pub AIFront(ref tai: DC2AIClass, int tFrontType)
    {
      this.Coords = new Coordinate[1];
      this.tempEnemyRegime = -1;
      this.tempEnemyRegime2 = -1;
      this.hasSupplySource = false;
      this.tempOrigAutomatonPower = 0;
      this.receivedJoin = 0;
      this.units = AIUnitList::new();
      this.artUnits = AIUnitList::new();
      this.orgUnits = AIUnitList::new();
      this.removelist = AIUnitList::new();
      this.strictHQs = AIUnitList::new();
      this.FrontType = tFrontType;
      this.ai = tai;
      this.TargetFrontID = -1;
      this.coordCount = -1;
      this.targetX = -1;
      this.targetY = -1;
      this.OffensiveZone = -1;
      this.OffensiveZone2 = -1;
      this.DefensiveZone = -1;
    }

    pub fn AddUnit(int unr) => this.units.add(unr, this.ai.game.Data.UnitObj[unr].AIid);

    pub fn AddCoord(int x, int y)
    {
      this += 1.coordCount;
      this.Coords = (Coordinate[]) Utils.CopyArray((Array) this.Coords, (Array) new Coordinate[this.coordCount + 1]);
      this.Coords[this.coordCount].x = x;
      this.Coords[this.coordCount].y = y;
    }

    pub HasCoord: bool(int x, int y)
    {
      let mut coordCount: i32 =  this.coordCount;
      for (let mut index: i32 =  0; index <= coordCount; index += 1)
      {
        if (this.Coords[index].x == x & this.Coords[index].y == y)
          return true;
      }
      return false;
    }

    pub fn RemoveCoord(int x, int y)
    {
      let mut num1: i32 =  -1;
      let mut coordCount: i32 =  this.coordCount;
      for (let mut index: i32 =  0; index <= coordCount; index += 1)
      {
        if (this.Coords[index].x == x & this.Coords[index].y == y)
        {
          num1 = index;
          break;
        }
      }
      if (num1 <= -1)
        return;
      let mut num2: i32 =  num1;
      let mut num3: i32 =  this.coordCount - 1;
      for (let mut index: i32 =  num2; index <= num3; index += 1)
        this.Coords[index] = this.Coords[index + 1];
      --this.coordCount;
      this.Coords = (Coordinate[]) Utils.CopyArray((Array) this.Coords, (Array) new Coordinate[this.coordCount + 1]);
    }

    pub fn AddArtUnit(int unr) => this.artUnits.add(unr, this.ai.game.Data.UnitObj[unr].AIid);

    pub fn AddOrgUnit(int unr) => this.orgUnits.add(unr, this.ai.game.Data.UnitObj[unr].AIid);

    pub fn RemoveUnitAIid(int AIid) => this.units.removeAiId(AIid);

    pub fn RemoveArtUnitAIid(int AIid) => this.artUnits.removeAiId(AIid);

    pub fn RemoveOrgUnitAIid(int AIid) => this.orgUnits.removeAiId(AIid);

    pub int GetPowerUnderCorps(int hq)
    {
      let mut powerUnderCorps: i32 =  0;
      let mut counter1: i32 =  this.units.counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        if (this.ai.game.Data.UnitObj[this.units.unr[index]].HQ == hq)
          powerUnderCorps += this.ai.game.Data.UnitObj[this.units.unr[index]].TempUnitPower;
      }
      let mut counter2: i32 =  this.artUnits.counter;
      for (let mut index: i32 =  0; index <= counter2; index += 1)
      {
        if (this.ai.game.Data.UnitObj[this.artUnits.unr[index]].HQ == hq)
          powerUnderCorps += this.ai.game.Data.UnitObj[this.artUnits.unr[index]].TempUnitPower;
      }
      return powerUnderCorps;
    }

    pub int GetPowerUnderFront(bool Absolute = false)
    {
      let mut powerUnderFront: i32 =  0;
      if (Absolute)
      {
        let mut counter1: i32 =  this.units.counter;
        for (let mut index: i32 =  0; index <= counter1; index += 1)
          powerUnderFront += this.ai.game.Data.UnitObj[this.units.unr[index]].TempUnitPowerAbs;
        let mut counter2: i32 =  this.artUnits.counter;
        for (let mut index: i32 =  0; index <= counter2; index += 1)
          powerUnderFront += this.ai.game.Data.UnitObj[this.artUnits.unr[index]].TempUnitPowerAbs;
      }
      else
      {
        let mut counter3: i32 =  this.units.counter;
        for (let mut index: i32 =  0; index <= counter3; index += 1)
          powerUnderFront += this.ai.game.Data.UnitObj[this.units.unr[index]].TempUnitPower;
        let mut counter4: i32 =  this.artUnits.counter;
        for (let mut index: i32 =  0; index <= counter4; index += 1)
          powerUnderFront += this.ai.game.Data.UnitObj[this.artUnits.unr[index]].TempUnitPower;
      }
      return powerUnderFront;
    }

    pub int GetPowerUnderHis(int his)
    {
      let mut powerUnderHis: i32 =  0;
      let mut counter1: i32 =  this.units.counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        if (this.ai.game.Data.UnitObj[this.units.unr[index]].Historical == his)
          powerUnderHis = powerUnderHis + this.ai.game.Data.UnitObj[this.units.unr[index]].TempUnitPower + 1;
      }
      let mut counter2: i32 =  this.artUnits.counter;
      for (let mut index: i32 =  0; index <= counter2; index += 1)
      {
        if (this.ai.game.Data.UnitObj[this.artUnits.unr[index]].Historical == his)
          powerUnderHis = powerUnderHis + this.ai.game.Data.UnitObj[this.artUnits.unr[index]].TempUnitPower + 1;
      }
      return powerUnderHis;
    }

    pub int GetTransferableHisUnitsInUnitList()
    {
      let mut hisUnitsInUnitList: i32 =  0;
      let mut num1: i32 =  0;
      if (DrawMod.TGame.Data.Product >= 6)
        num1 = -1;
      bool[] flagArray = new bool[this.units.counter + 1];
      let mut counter1: i32 =  this.units.counter;
      for (let mut index1: i32 =  0; index1 <= counter1; index1 += 1)
      {
        if (this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical > -1 & !flagArray[index1] && this.ai.game.Data.UnitObj[this.units.unr[index1]].TempCategory == 1 && this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical) > num1)
        {
          hisUnitsInUnitList += 1;
          flagArray[index1] = true;
          let mut num2: i32 =  index1 + 1;
          let mut counter2: i32 =  this.units.counter;
          for (let mut index2: i32 =  num2; index2 <= counter2; index2 += 1)
          {
            if (this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical == this.ai.game.Data.UnitObj[this.units.unr[index2]].Historical)
              flagArray[index2] = true;
          }
        }
      }
      return hisUnitsInUnitList;
    }

    pub HasTopUnits: bool()
    {
      let mut counter: i32 =  this.units.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.ai.game.Data.UnitObj[this.units.unr[index]].TempTopUnit)
          return true;
      }
      return false;
    }

    pub int GetUnitAIIDToSplitOff(
      AIMatrix frontMatrix,
      int targetFrontID,
      AIFront targetFront,
      bool GetPowerPts = false)
    {
      let mut id: i32 =  -1;
      let mut num1: i32 =  999999999;
      let mut num2: i32 =  0;
      bool[] flagArray = new bool[this.units.counter + 1];
      let mut counter1: i32 =  this.units.counter;
      int num3;
      int num4;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        if (this.ai.game.Data.UnitObj[this.units.unr[index]].Historical > -1 & !flagArray[index])
        {
          bool flag = false;
          if (this.ai.VAR_USE_STRICT_HQFRONT)
          {
            let mut hq: i32 =  this.ai.game.Data.UnitObj[this.units.unr[index]].HQ;
            if (hq > -1 && this.strictHQs.CheckIfPresentUnr(hq) & targetFront.strictHQs.CheckIfPresentUnr(hq))
              flag = true;
          }
          else
            flag = true;
          if (flag & this.ai.game.Data.UnitObj[this.units.unr[index]].TempCategory == 1 & !this.ai.game.Data.UnitObj[this.units.unr[index]].TempTopUnit && this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index]].Historical) > 0 && this.ai.game.Data.UnitObj[this.units.unr[index]].Historical != this.FrontID & !this.ai.game.Data.UnitObj[this.units.unr[index]].AIReserve)
          {
            if (this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index]].Historical) <= 1)
              num3 += 1;
            else
              num4 += 1;
          }
        }
      }
      let mut num5: i32 =  1;
      do
      {
        let mut counter2: i32 =  this.units.counter;
        for (let mut index1: i32 =  0; index1 <= counter2; index1 += 1)
        {
          if (this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical > -1 & !flagArray[index1])
          {
            bool flag = false;
            if (this.ai.VAR_USE_STRICT_HQFRONT)
            {
              let mut hq: i32 =  this.ai.game.Data.UnitObj[this.units.unr[index1]].HQ;
              if (hq > -1 && this.strictHQs.CheckIfPresentUnr(hq) & targetFront.strictHQs.CheckIfPresentUnr(hq))
                flag = true;
            }
            else
              flag = true;
            let mut x: i32 =  this.ai.game.Data.UnitObj[this.units.unr[index1]].X;
            let mut y: i32 =  this.ai.game.Data.UnitObj[this.units.unr[index1]].Y;
            if (flag & (num5 == 2 | frontMatrix.Value[x, y] == targetFrontID | targetFrontID == -1) && this.ai.game.Data.UnitObj[this.units.unr[index1]].TempCategory == 1 & !this.ai.game.Data.UnitObj[this.units.unr[index1]].TempTopUnit && this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical) > 0 && this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical != this.FrontID & !this.ai.game.Data.UnitObj[this.units.unr[index1]].AIReserve)
            {
              num2 += 1;
              if (num3 >= num4 | num4 == 1)
              {
                if (this.units.AIid[index1] + 100000 * this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical) < num1)
                {
                  num1 = this.units.AIid[index1] + 1000000 * this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical);
                  id = this.units.AIid[index1];
                }
              }
              else if (this.units.AIid[index1] - 100 * this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical) < num1)
              {
                num1 = this.units.AIid[index1] - 100 * this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical);
                id = this.units.AIid[index1];
              }
              flagArray[index1] = true;
              let mut num6: i32 =  index1 + 1;
              let mut counter3: i32 =  this.units.counter;
              for (let mut index2: i32 =  num6; index2 <= counter3; index2 += 1)
              {
                if (this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical == this.ai.game.Data.UnitObj[this.units.unr[index2]].Historical)
                  flagArray[index2] = true;
              }
            }
          }
        }
        if (id <= -1)
          num5 += 1;
        else
          break;
      }
      while (num5 <= 2);
      if (!GetPowerPts)
        return id;
      if (id <= -1)
        return 0;
      let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(id);
      let mut unitCounter: i32 =  this.ai.game.Data.UnitCounter;
      int unitAiidToSplitOff;
      for (let mut index: i32 =  0; index <= unitCounter; index += 1)
      {
        if (this.ai.game.Data.UnitObj[index].Historical == this.ai.game.Data.UnitObj[unitByAiid].Historical)
          unitAiidToSplitOff += this.ai.game.Data.UnitObj[index].TempUnitPowerAbs;
      }
      return unitAiidToSplitOff;
    }

    pub int GetUnitsInTargetFrontIDTerritory(AIMatrix frontMatrix, int targetFrontID)
    {
      bool[] flagArray = new bool[this.units.counter + 1];
      let mut counter1: i32 =  this.units.counter;
      int frontIdTerritory;
      for (let mut index1: i32 =  0; index1 <= counter1; index1 += 1)
      {
        if (this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical > -1 & !flagArray[index1])
        {
          let mut x: i32 =  this.ai.game.Data.UnitObj[this.units.unr[index1]].X;
          let mut y: i32 =  this.ai.game.Data.UnitObj[this.units.unr[index1]].Y;
          if (frontMatrix.Value[x, y] == targetFrontID | targetFrontID == -1 && this.ai.game.Data.UnitObj[this.units.unr[index1]].TempCategory == 1 & !this.ai.game.Data.UnitObj[this.units.unr[index1]].TempTopUnit && this.ai.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical) > 0 && this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical != this.FrontID && !this.ai.game.Data.UnitObj[this.units.unr[index1]].AIReserve)
          {
            frontIdTerritory += 1;
            flagArray[index1] = true;
            let mut num: i32 =  index1 + 1;
            let mut counter2: i32 =  this.units.counter;
            for (let mut index2: i32 =  num; index2 <= counter2; index2 += 1)
            {
              if (this.ai.game.Data.UnitObj[this.units.unr[index1]].Historical == this.ai.game.Data.UnitObj[this.units.unr[index2]].Historical)
                flagArray[index2] = true;
            }
          }
        }
      }
      return frontIdTerritory;
    }

    pub Coordinate GetAverageFrontCoordinate(let mut withZoneNumber: i32 =  -1)
    {
      let mut num1: i32 =  0;
      int num2;
      int num3;
      if (this.FrontType != 2)
      {
        let mut mapWidth: i32 =  this.ai.map.MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.ai.map.MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (this.ai.frontMatrix.Value[index1, index2] == this.FrontID & (withZoneNumber == -1 | this.ai.VAR_MATRIX_ZONES.Value[index1, index2] == withZoneNumber))
            {
              num1 += 1;
              num2 += index1;
              num3 += index2;
            }
          }
        }
      }
      Coordinate averageFrontCoordinate = Coordinate::new();
      if (num1 > 0)
      {
        averageFrontCoordinate.onmap = true;
        averageFrontCoordinate.x = (int) Math.Round( num2 /  num1);
        averageFrontCoordinate.y = (int) Math.Round( num3 /  num1);
      }
      else
      {
        let mut counter1: i32 =  this.units.counter;
        for (let mut index: i32 =  0; index <= counter1; index += 1)
        {
          let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(this.units.AIid[index]);
          num2 += this.ai.game.Data.UnitObj[unitByAiid].X;
          num3 += this.ai.game.Data.UnitObj[unitByAiid].Y;
          num1 += 1;
        }
        let mut counter2: i32 =  this.artUnits.counter;
        for (let mut index: i32 =  0; index <= counter2; index += 1)
        {
          let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(this.artUnits.AIid[index]);
          num2 += this.ai.game.Data.UnitObj[unitByAiid].X;
          num3 += this.ai.game.Data.UnitObj[unitByAiid].Y;
          num1 += 1;
        }
        let mut counter3: i32 =  this.orgUnits.counter;
        for (let mut index: i32 =  0; index <= counter3; index += 1)
        {
          let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(this.orgUnits.AIid[index]);
          num2 += this.ai.game.Data.UnitObj[unitByAiid].X;
          num3 += this.ai.game.Data.UnitObj[unitByAiid].Y;
          num1 += 1;
        }
        if (num1 > 0)
        {
          averageFrontCoordinate.onmap = true;
          averageFrontCoordinate.x = (int) Math.Round( num2 /  num1);
          averageFrontCoordinate.y = (int) Math.Round( num3 /  num1);
        }
        else
          averageFrontCoordinate.onmap = false;
      }
      return averageFrontCoordinate;
    }

    pub Coordinate GetAverageUnitsCoordinate(int hqNrNecc, int SSHQNrNecc)
    {
      let mut num1: i32 =  0;
      Coordinate averageUnitsCoordinate = Coordinate::new();
      let mut counter1: i32 =  this.units.counter;
      int num2;
      int num3;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(this.units.AIid[index]);
        if (hqNrNecc < 0 | this.ai.game.Data.UnitObj[unitByAiid].HQ == hqNrNecc && SSHQNrNecc < 0 | this.ai.game.Data.UnitObj[unitByAiid].AISubStrictGroup == SSHQNrNecc)
        {
          num2 += this.ai.game.Data.UnitObj[unitByAiid].X;
          num3 += this.ai.game.Data.UnitObj[unitByAiid].Y;
          num1 += 1;
        }
      }
      let mut counter2: i32 =  this.artUnits.counter;
      for (let mut index: i32 =  0; index <= counter2; index += 1)
      {
        let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(this.artUnits.AIid[index]);
        if (hqNrNecc < 0 | this.ai.game.Data.UnitObj[unitByAiid].HQ == hqNrNecc && SSHQNrNecc < 0 | this.ai.game.Data.UnitObj[unitByAiid].AISubStrictGroup == SSHQNrNecc)
        {
          num2 += this.ai.game.Data.UnitObj[unitByAiid].X;
          num3 += this.ai.game.Data.UnitObj[unitByAiid].Y;
          num1 += 1;
        }
      }
      let mut counter3: i32 =  this.orgUnits.counter;
      for (let mut index: i32 =  0; index <= counter3; index += 1)
      {
        let mut unitByAiid: i32 =  this.ai.game.HandyFunctionsObj.GetUnitByAIid(this.orgUnits.AIid[index]);
        if (hqNrNecc < 0 | this.ai.game.Data.UnitObj[unitByAiid].HQ == hqNrNecc && SSHQNrNecc < 0 | this.ai.game.Data.UnitObj[unitByAiid].AISubStrictGroup == SSHQNrNecc)
        {
          num2 += this.ai.game.Data.UnitObj[unitByAiid].X;
          num3 += this.ai.game.Data.UnitObj[unitByAiid].Y;
          num1 += 1;
        }
      }
      if (num1 > 0)
      {
        averageUnitsCoordinate.onmap = true;
        averageUnitsCoordinate.x = (int) Math.Round( num2 /  num1);
        averageUnitsCoordinate.y = (int) Math.Round( num3 /  num1);
      }
      else
        averageUnitsCoordinate.onmap = false;
      return averageUnitsCoordinate;
    }
  }
}
