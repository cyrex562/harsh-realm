// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;

namespace WindowsApplication1
{
  pub class CombatClass
  {
    pub game: GameClass;
    pub previewMode: bool;
    pub CombatStep: i32;
    pub CombatRound: i32;
    pub DetailCounter: i32;
    pub DetailString: Vec<String>;
    pub RepText: Vec<String>;
    pub RepTitle: Vec<String>;
    pub RepFrom: Vec<i32>;
    pub RepRound: Vec<i32>;
    pub RepType: Vec<i32>;
    pub string[,,] RepCom;
    pub RepCounter: i32;
    pub BattleCounter: i32;
    pub object[] BattleString;
    pub AllDetailCounter: i32;
    pub AllDetailString: Vec<String>;
    pub CoordList HexList;
    pub UCounter: i32;
    pub BattleUnit[] UList;
    pub EventCaused: bool;
    pub ICounter: i32;
    pub Individual[] IList;
    pub BattleEnded: i32;
    pub CombatType: i32;
    pub CombatType2: i32;
    pub IDcounter: i32;
    pub const LANDATTACK: i32 =  1;
    pub const LANDSURPRISE: i32 =  11;
    pub const AIRRECON: i32 =  13;
    pub const SEAATTACK: i32 =  2;
    pub const ARTATTACK: i32 =  3;
    pub const AIRSTRIKE: i32 =  5;
    pub const SEAARTATTACK: i32 =  4;
    pub const BOMB: i32 =  6;
    pub const PARADROP: i32 =  9;
    pub const AMPH: i32 =  10;
    pub const REBEL: i32 =  12;
    pub const AIRSUPPLY: i32 =  14;
    pub const AIRLIFT: i32 =  15;
    pub const AIRBRIDGE: i32 =  16;
    pub InterceptFire: bool;
    pub Coordinate CombatTarget;
    pub float AttackCrowding;
    pub float AttackCrowdingAir;
    pub float AttackCrowdingArt;
    pub float CrowdingAttMod;
    pub float CrowdingAttAirMod;
    pub float CrowdingAttArtMod;
    pub float CrowdingDefMod;
    pub CrowdingDefCount: i32;
    pub AttackerRegime: i32;
    pub DefenderRegime: i32;
    pub TargetX: i32;
    pub TargetY: i32;
    pub TargetMap: i32;
    pub float ConcentricBonus;
    pub float bestConcentricBonus;
    pub AttackerDice: i32;
    pub DefenderDice: i32;
    pub AntiStrucDam: i32;
    pub initialStrucDam: i32;
    pub Coordinate TempTarget;
    pub TempType: i32;
    pub UnitList TempUnits;
    pub Tempattacktype: i32;
    pub MapMatrix2[] TempValue3;
    pub initialRecon: i32;
    pub initialReconDef: i32;
    pub NewBattleStack: i32;
    pub NewBattleStackArt: i32;
    pub NewBattleStackAir: i32;
    pub se1carryPointsTotal: i32;
    pub se1carryPointsDelivered: i32;
    pub se1damagePercentage: i32;
    pub dontUseSfx: bool;
    pub somePartialAttackPresent: bool;
    pub CustomCombatCalls customCombatObj;
    pub allowHistoryOwnRegime: bool;

    pub CombatClass(tgame: GameClass, bool tPreviewMode = false)
    {
      this.previewMode = false;
      this.DetailString = new string[1];
      this.RepText = new string[1];
      this.RepTitle = new string[1];
      this.RepFrom = new int[1];
      this.RepRound = new int[1];
      this.RepType = new int[1];
      this.RepCom = new string[61, 6, 1];
      this.BattleString = new object[1];
      this.AllDetailString = new string[1];
      this.UList = new BattleUnit[1];
      this.IList = new Individual[1];
      this.InterceptFire = false;
      this.TempValue3 = new MapMatrix2[1];
      this.dontUseSfx = false;
      this.somePartialAttackPresent = false;
      this.allowHistoryOwnRegime = false;
      this.CombatStep = 0;
      this.CombatRound = 0;
      this.UCounter = -1;
      this.ICounter = -1;
      this.BattleCounter = -1;
      this.DetailCounter = -1;
      this.BattleEnded = 0;
      this.game = tgame;
      this.previewMode = tPreviewMode;
      this += 1.game.Data.CombatLogId;
      this.TempValue3 = new MapMatrix2[tgame.Data.MapCounter + 1];
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut index: i32 =  0; index <= mapCounter; index += 1)
        this.TempValue3[index] = new MapMatrix2(this.game.Data.MapObj[index].MapWidth, this.game.Data.MapObj[index].MapHeight);
      this.customCombatObj = CustomCombatCalls::new();
    }

    pub OrderResult Init(
      Coordinate Target,
      Type: i32,
      UnitList Units,
      attacktype: i32,
      bool tDontUseSfx = false,
      bool tallowHistoryOwnRegime = false)
    {
      OrderResult orderResult = OrderResult::new();
      Neighbours neighbour = Neighbours::new();
      int[] numArray1 = new int[this.game.Data.RegimeCounter + 1];
      this.dontUseSfx = tDontUseSfx;
      this.allowHistoryOwnRegime = tallowHistoryOwnRegime;
      this.game.EditObj.CombatOneSentence = "";
      this.game.EditObj.CombatOneSentenceCustom = "";
      if (this.game.Data.UncertaintyOn)
      {
        this.AttackerDice = DrawMod.RandyNumber.Next(1, 7);
        this.DefenderDice = DrawMod.RandyNumber.Next(1, 7);
      }
      this.TempTarget = Target;
      this.TempType = Type;
      this.TempUnits = Units;
      this.Tempattacktype = attacktype;
      orderResult.OK = true;
      this.game.EditObj.TempValue = new MapMatrix2[this.game.Data.MapCounter + 1];
      let mut mapCounter1: i32 =  this.game.Data.MapCounter;
      for (let mut index: i32 =  0; index <= mapCounter1; index += 1)
        this.game.EditObj.TempValue[index] = new MapMatrix2(this.game.Data.MapObj[index].MapWidth, this.game.Data.MapObj[index].MapHeight);
      let mut mapCounter2: i32 =  this.game.Data.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter2; index1 += 1)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            this.TempValue3[index1].Value[index2, index3] = this.game.EditObj.TempValue[index1].Value[index2, index3];
        }
      }
      if (orderResult.OK)
      {
        if (this.game.Data.MapObj[0].HexObj[Target.x, Target.y].Location > -1)
          this.initialStrucDam = this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[Target.x, Target.y].Location].StructuralPts;
        this.CombatTarget = Target;
        this.HexList = CoordList::new();
        this.HexList.AddCoord(Target.x, Target.y, Target.map);
        this.TargetX = Target.x;
        this.TargetY = Target.y;
        this.TargetMap = Target.map;
        this.DefenderRegime = -1;
        this.InterceptFire = false;
        this.CombatType2 = 0;
        if (attacktype == 2)
          this.CombatType = 1;
        if (attacktype == 12)
          this.CombatType = 2;
        if (attacktype == 11)
          this.CombatType = 3;
        if (attacktype == 13)
          this.CombatType = 4;
        if (attacktype == 14)
          this.CombatType = 5;
        if (attacktype == 15)
          this.CombatType = 6;
        if (attacktype == 19)
          this.CombatType = 9;
        if (attacktype == 21)
          this.CombatType = 10;
        if (attacktype == 1)
          this.CombatType = 11;
        if (attacktype == 31)
          this.CombatType = 12;
        if (attacktype == 33)
          this.CombatType = 13;
        if (attacktype == 55)
        {
          this.CombatType = 13;
          this.CombatType2 = 16;
        }
        if (attacktype == 40)
          this.CombatType = 14;
        if (attacktype == 42)
          this.CombatType = 15;
        if (attacktype == 18)
          this.CombatType = 10;
        if (attacktype == 99 &  this.game.Data.RuleVar[428] > 0.0)
        {
          this.CombatType = 3;
          this.InterceptFire = true;
        }
        this.AttackCrowding = this.game.Data.RuleVar[30];
        this.AttackCrowdingAir = this.game.Data.RuleVar[833];
        this.AttackCrowdingArt = this.game.Data.RuleVar[834];
        this.game.EditObj.TargetX = this.TargetX;
        this.game.EditObj.TargetY = this.TargetY;
        if (this.CombatType == 12 | this.CombatType == 10 | this.CombatType == 1 | this.CombatType == 9)
          this.AttackCrowding =  this.game.HandyFunctionsObj.maxAttackStack();
        let mut counter1: i32 =  Units.counter;
        Coordinate coordinate;
        for (let mut index: i32 =  0; index <= counter1; index += 1)
        {
          if (this.CombatType == 9 & Units.data[index] == 1)
          {
            this.game.Data.MapObj[this.game.Data.UnitObj[Units.unr[index]].Map].HexObj[this.game.Data.UnitObj[Units.unr[index]].X, this.game.Data.UnitObj[Units.unr[index]].Y].RemoveUnitFromList(Units.unr[index]);
            this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].AddUnitToList(Units.unr[index]);
            this.game.Data.UnitObj[Units.unr[index]].X = Target.x;
            this.game.Data.UnitObj[Units.unr[index]].Y = Target.y;
          }
          if (this.CombatType == 9 & Units.data[index] == 1 | this.CombatType == 10 | this.CombatType == 12)
          {
            coordinate = Coordinate::new();
            coordinate.x = -1;
            coordinate.y = -1;
            coordinate.map = -1;
            this.AttackerRegime = this.game.Data.UnitObj[Units.unr[index]].Regime;
            coordinate.onmap = false;
            this.AddUnitToUnits(Units.unr[index], 1, coordinate, coordinate, Target, IsParadropper: true);
          }
          else
          {
            coordinate = Coordinate::new();
            coordinate.onmap = true;
            coordinate.x = this.game.Data.UnitObj[Units.unr[index]].X;
            coordinate.y = this.game.Data.UnitObj[Units.unr[index]].Y;
            coordinate.map = this.game.Data.UnitObj[Units.unr[index]].Map;
            this.AttackerRegime = this.game.Data.UnitObj[Units.unr[index]].Regime;
            this.AddUnitToUnits(Units.unr[index], 1, coordinate, coordinate, Target);
          }
          if (this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 11)
          {
            let mut num: i32 =  this.game.HandyFunctionsObj.HexFacing(Target.x, Target.y, Target.map, this.game.Data.UnitObj[Units.unr[index]].X, this.game.Data.UnitObj[Units.unr[index]].Y, this.game.Data.UnitObj[Units.unr[index]].Map);
            if (num > 0)
              neighbour.data[num - 1] = 1;
          }
          if (!this.HexList.Exists(this.game.Data.UnitObj[Units.unr[index]].X, this.game.Data.UnitObj[Units.unr[index]].Y, this.game.Data.UnitObj[Units.unr[index]].Map))
            this.HexList.AddCoord(this.game.Data.UnitObj[Units.unr[index]].X, this.game.Data.UnitObj[Units.unr[index]].Y, this.game.Data.UnitObj[Units.unr[index]].Map);
        }
        let mut unitCounter1: i32 =  this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].UnitCounter;
        Coordinate ttoo;
        for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
        {
          let mut unit: i32 =  this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].UnitList[index];
          if (this.game.Data.UnitObj[unit].Regime != this.AttackerRegime & (this.game.Data.RegimeObj[this.AttackerRegime].RegimeRel[this.game.Data.UnitObj[unit].Regime] <= 1 |  this.game.Data.RuleVar[342] == 1.0) && this.InterceptFire & unit == this.game.EditObj.OrderUnit | !this.InterceptFire)
          {
            if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].LandscapeType].IsSea)
              coordinate = this.game.HandyFunctionsObj.FindLandRetreat(unit, Target, neighbour);
            else if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].LandscapeType].IsSea)
              coordinate = this.game.HandyFunctionsObj.FindSeaRetreat(unit, Target, neighbour);
            this.AddUnitToUnits(unit, 0, coordinate, Target, ttoo);
            if (this.game.Data.UnitObj[unit].Regime != this.AttackerRegime)
              this.DefenderRegime = this.game.Data.UnitObj[unit].Regime;
          }
        }
        if (this.CombatType == 1 &  this.game.Data.RuleVar[428] > 0.0 & this.game.Data.Product >= 6 & !this.previewMode)
        {
          UnitList interceptFireUnits = this.game.HandyFunctionsObj.GetInterceptFireUnits(-1, Target.x, Target.y);
          let mut counter2: i32 =  interceptFireUnits.counter;
          for (let mut index4: i32 =  0; index4 <= counter2; index4 += 1)
          {
            let mut tunr: i32 =  interceptFireUnits.unr[index4];
            let mut num: i32 =  1;
            let mut unitCounter2: i32 =  this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].UnitCounter;
            for (let mut index5: i32 =  0; index5 <= unitCounter2; index5 += 1)
            {
              if (this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].UnitList[index5] == tunr)
                num = 0;
            }
            if (num == 1)
            {
              coordinate = Coordinate::new();
              coordinate.onmap = true;
              coordinate.x = this.game.Data.UnitObj[tunr].X;
              coordinate.y = this.game.Data.UnitObj[tunr].Y;
              coordinate.map = this.game.Data.UnitObj[tunr].Map;
              if (coordinate.x == -1)
                index4 = index4;
              this.AddUnitToUnits(tunr, 0, coordinate, coordinate, Target, isSupportInterceptFire: true);
              if (!this.HexList.Exists(this.game.Data.UnitObj[tunr].X, this.game.Data.UnitObj[tunr].Y, this.game.Data.UnitObj[tunr].Map))
                this.HexList.AddCoord(this.game.Data.UnitObj[tunr].X, this.game.Data.UnitObj[tunr].Y, this.game.Data.UnitObj[tunr].Map);
            }
          }
        }
        if (!this.previewMode && this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 9 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15)
        {
          let mut num1: i32 =  0;
          let mut index6: i32 =  -1;
          if (this.game.Data.MapObj[Target.map].HexObj[Target.x, Target.y].Regime == this.AttackerRegime | this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[Target.x, Target.y].Regime == -1)
          {
            num1 = 1;
            let mut unitCounter3: i32 =  this.game.Data.UnitCounter;
            for (let mut unr: i32 =  0; unr <= unitCounter3; unr += 1)
            {
              if (this.game.Data.UnitObj[unr].Regime != this.AttackerRegime && this.game.HandyFunctionsObj.CanUnitIntercept(unr, Target, this.AttackerRegime))
              {
                bool flag = true;
                if ( this.game.Data.RuleVar[407] > 0.0 && !this.CheckEnoughFuelAmmo(unr, false))
                  flag = false;
                if (flag && !(this.game.Data.UnitObj[unr].X == this.TargetX & this.game.Data.UnitObj[unr].Y == this.TargetY))
                {
                  int[] numArray2 = numArray1;
                  int[] numArray3 = numArray2;
                  let mut regime: i32 =  this.game.Data.UnitObj[unr].Regime;
                  let mut index7: i32 =  regime;
                  let mut num2: i32 =  numArray2[regime] + this.game.HandyFunctionsObj.GetInterdictPower(unr);
                  numArray3[index7] = num2;
                }
              }
            }
            let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
            for (let mut index8: i32 =  0; index8 <= regimeCounter; index8 += 1)
            {
              num3: i32;
              if (numArray1[index8] > num3)
              {
                index6 = index8;
                num3 = numArray1[index8];
              }
            }
            if (index6 > -1)
              this.DefenderRegime = index6;
          }
          if (this.DefenderRegime == -1)
            this.DefenderRegime = this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Regime;
          int[] numArray4 = new int[500];
          int[] numArray5 = new int[500];
          Random random = Random::new();
          float num4 =  random.Next(0, 100);
          let mut unitCounter4: i32 =  this.game.Data.UnitCounter;
          for (let mut index9: i32 =  0; index9 <= unitCounter4; index9 += 1)
          {
            if (this.game.Data.UnitObj[index9].Regime != this.AttackerRegime)
            {
              float num5 =  random.Next(0, 100);
              let mut num6: i32 =  1;
              let mut num7: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[index9].X, this.game.Data.UnitObj[index9].Y, 0, this.TargetX, this.TargetY, this.TargetMap);
              if (num7 == 0)
                num7 = num7;
              if (this.game.HandyFunctionsObj.CanUnitIntercept(index9, Target, this.AttackerRegime))
              {
                if (num7 > 0 &  this.game.Data.RuleVar[837] > 0.0)
                {
                  let mut num8: i32 =  this.game.EditObj.TempValue[this.TargetMap].Value[this.TargetX, this.TargetY];
                  let mut num9: i32 =  (int) Math.Round( ( this.game.HandyFunctionsObj.GetLowestAirAp(index9) * this.game.Data.RuleVar[147]));
                  let mut num10: i32 =  num8 <= num9 ? (int) Math.Round( ( (int) Math.Round( (int) Math.Round( (this.game.Data.RuleVar[837] - this.game.Data.RuleVar[838])) * ( num8 /  num9)) + this.game.Data.RuleVar[838])) : (int) Math.Round( this.game.Data.RuleVar[837]);
                  if ( num5 <  num10)
                    num6 = 0;
                }
                if (num6 == 1 && !(this.game.Data.UnitObj[index9].Map == this.TargetMap & this.game.Data.UnitObj[index9].X == this.TargetX & this.game.Data.UnitObj[index9].Y == this.TargetY))
                {
                  let mut num11: i32 =  1;
                  if (num1 == 1 & this.game.Data.UnitObj[index9].Regime != index6)
                  {
                    num11 = 0;
                    if (this.game.Data.RegimeObj[index6].RegimeRel[this.game.Data.UnitObj[index9].Regime] == 2)
                      num11 = 1;
                  }
                  if (num11 == 1)
                  {
                    if ( this.game.Data.RuleVar[407] > 0.0)
                    {
                      if ( this.game.Data.RuleVar[407] > 0.0)
                      {
                        this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[index9].X, this.game.Data.UnitObj[index9].Y, 0, this.TargetX, this.TargetY, 0);
                        if (!this.CheckEnoughFuelAmmo(index9, false))
                          num11 = 0;
                      }
                    }
                    else
                    {
                      let mut index10: i32 =  0;
                      do
                      {
                        numArray4[index10] = numArray5[index10];
                        index10 += 1;
                      }
                      while (index10 <= 499);
                      let mut sfCount: i32 =  this.game.Data.UnitObj[index9].SFCount;
                      for (let mut index11: i32 =  0; index11 <= sfCount; index11 += 1)
                      {
                        let mut sf: i32 =  this.game.Data.UnitObj[index9].SFList[index11];
                        let mut type: i32 =  this.game.Data.SFObj[sf].Type;
                        if (this.game.Data.SFTypeObj[type].Theater == 2 && !this.game.Data.SFTypeObj[type].BackBench)
                        {
                          let mut fuelRegimeVar: i32 =  this.game.Data.SFTypeObj[type].FuelRegimeVar;
                          let mut currentSlot: i32 =  this.game.Data.SFTypeObj[type].FuelRegimeVar;
                          if ( this.game.Data.RuleVar[949] > 0.0)
                            currentSlot = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot, this.game.Data.UnitObj[index9].RealX(ref this.game), this.game.Data.UnitObj[index9].RealY(ref this.game));
                          let mut index12: i32 =  currentSlot;
                          if (index12 > -1)
                          {
                            let mut num12: i32 =  this.game.Data.SFTypeObj[type].FuelForAttackDef * 2 * this.game.Data.SFObj[sf].Qty;
                            int[] numArray6 = numArray4;
                            int[] numArray7 = numArray6;
                            let mut index13: i32 =  index12;
                            let mut index14: i32 =  index13;
                            let mut num13: i32 =  numArray6[index13] + num12;
                            numArray7[index14] = num13;
                            if (numArray4[index12] > this.game.Data.RegimeObj[this.game.Data.UnitObj[index9].Regime].RegimeSlot[index12])
                              num11 = 0;
                          }
                        }
                      }
                      if (num11 == 1)
                      {
                        let mut index15: i32 =  0;
                        do
                        {
                          numArray5[index15] = numArray4[index15];
                          index15 += 1;
                        }
                        while (index15 <= 499);
                      }
                    }
                    if (num11 == 1)
                    {
                      coordinate = Coordinate::new();
                      coordinate.onmap = true;
                      coordinate.x = this.game.Data.UnitObj[index9].X;
                      coordinate.y = this.game.Data.UnitObj[index9].Y;
                      coordinate.map = this.game.Data.UnitObj[index9].Map;
                      Coordinate tfrom = coordinate;
                      if (this.game.Data.UnitObj[index9].X == this.TargetX & this.game.Data.UnitObj[index9].Y == this.TargetY & this.game.Data.UnitObj[index9].Map == this.TargetMap)
                        coordinate.onmap = false;
                      if (!this.HexList.Exists(this.game.Data.UnitObj[index9].X, this.game.Data.UnitObj[index9].Y, this.game.Data.UnitObj[index9].Map))
                        this.HexList.AddCoord(this.game.Data.UnitObj[index9].X, this.game.Data.UnitObj[index9].Y, this.game.Data.UnitObj[index9].Map);
                      if (this.DefenderRegime == -1)
                        this.DefenderRegime = this.game.Data.UnitObj[index9].Regime;
                      let mut num14: i32 =  0;
                      if (this.DefenderRegime == this.game.Data.UnitObj[index9].Regime)
                        num14 = 1;
                      if (this.game.Data.RegimeObj[this.DefenderRegime].RegimeRel[this.game.Data.UnitObj[index9].Regime] == 2)
                        num14 = 1;
                      if (num14 == 1)
                        this.AddUnitToUnits(index9, 0, coordinate, tfrom, ttoo, true);
                    }
                  }
                }
              }
            }
          }
          let mut unitCounter5: i32 =  this.game.Data.UnitCounter;
          for (let mut index16: i32 =  0; index16 <= unitCounter5; index16 += 1)
          {
            if (this.game.Data.UnitObj[index16].Regime == this.DefenderRegime && !(this.game.Data.UnitObj[index16].X == this.TargetX & this.game.Data.UnitObj[index16].Y == this.TargetY & this.game.Data.UnitObj[index16].Map == this.TargetMap) && this.game.HandyFunctionsObj.CanUnitAA(index16, Target, this.AttackerRegime))
            {
              bool flag = true;
              if ( this.game.Data.RuleVar[407] > 0.0 && !this.CheckEnoughFuelAmmo(index16, true))
                flag = false;
              if (flag)
              {
                coordinate = Coordinate::new();
                coordinate.onmap = true;
                coordinate.x = this.game.Data.UnitObj[index16].X;
                coordinate.y = this.game.Data.UnitObj[index16].Y;
                coordinate.map = this.game.Data.UnitObj[index16].Map;
                this.HexList.AddCoord(this.game.Data.UnitObj[index16].X, this.game.Data.UnitObj[index16].Y, this.game.Data.UnitObj[index16].Map);
                this.AddUnitToUnits(index16, 0, coordinate, coordinate, ttoo, isAA: true);
              }
            }
          }
        }
        if (this.DefenderRegime == -1)
          this.DefenderRegime = this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Regime;
        if (!this.previewMode)
        {
          let mut ucounter: i32 =  this.UCounter;
          for (let mut index: i32 =  0; index <= ucounter; index += 1)
          {
            let mut unr: i32 =  this.UList[index].UNr;
            this.game.HandyFunctionsObj.RemoveZOCPts(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.HandyFunctionsObj.GetUnitZOC(unr), this.game.Data.UnitObj[unr].Regime);
          }
        }
        if (!this.previewMode)
        {
          this += 1.game.Data.StepNr;
          if (!this.game.EditObj.CombatSim)
          {
            if (this.CombatType == 12)
            {
              if (!this.game.Data.RegimeObj[this.AttackerRegime].AI)
              {
                this.game.Data.MapObj[0].HexObj[Target.x, Target.y].set_ReconPts(this.AttackerRegime, 999);
                this.game.Data.MapObj[0].HexObj[Target.x, Target.y].set_SeeNow(this.AttackerRegime, 1);
                this.game.Data.MapObj[0].HexObj[Target.x, Target.y].set_LastLT(this.AttackerRegime, this.game.Data.MapObj[0].HexObj[Target.x, Target.y].LandscapeType);
                this.game.Data.MapObj[0].HexObj[Target.x, Target.y].set_LastSpr(this.AttackerRegime, this.game.Data.MapObj[0].HexObj[Target.x, Target.y].SpriteNr);
                this.game.Data.MapObj[0].HexObj[Target.x, Target.y].set_LastReg(this.AttackerRegime, this.game.Data.MapObj[0].HexObj[Target.x, Target.y].Regime);
                this.game.HandyFunctionsObj.HistoryAddHex(Target.x, Target.y, 0, this.AttackerRegime, 1, 1, infostring: "Friendly Rebels appear", allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
                this += 1.game.Data.StepNr;
              }
              this.game.HandyFunctionsObj.HistoryAddHex(Target.x, Target.y, Target.map, -1, 3, neighbour: neighbour, attacktype: attacktype, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
            }
            else if (this.InterceptFire)
              this.game.HandyFunctionsObj.HistoryAddHex(Target.x, Target.y, Target.map, this.DefenderRegime, 3, neighbour: neighbour, attacktype: attacktype, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
            else
              this.game.HandyFunctionsObj.HistoryAddHex(Target.x, Target.y, Target.map, this.AttackerRegime, 3, neighbour: neighbour, attacktype: attacktype, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
          }
        }
        bool flag1 = false;
        if (this.AttackerRegime > -1)
          flag1 = this.game.Data.RegimeObj[this.AttackerRegime].AI;
        if (this.CombatType == 1)
        {
          if (this.game.HandyFunctionsObj.GetHQsInUnitList() > 1)
          {
            handyFunctionsObj: HandyFunctionsclass = this.game.HandyFunctionsObj;
            ref Neighbours local1 = ref neighbour;
            bool flag2 = true;
            ref bool local2 = ref flag2;
            let mut num: i32 =  flag1 ? 1 : 0;
            this.ConcentricBonus = handyFunctionsObj.GetConcentricBonus(ref local1, ref local2, num != 0);
          }
          else
          {
            handyFunctionsObj: HandyFunctionsclass = this.game.HandyFunctionsObj;
            ref Neighbours local3 = ref neighbour;
            bool flag3 = false;
            ref bool local4 = ref flag3;
            let mut num: i32 =  flag1 ? 1 : 0;
            this.ConcentricBonus = handyFunctionsObj.GetConcentricBonus(ref local3, ref local4, num != 0);
          }
        }
        else
          this.ConcentricBonus = 1f;
        if ( this.ConcentricBonus >  this.bestConcentricBonus)
          this.bestConcentricBonus = this.ConcentricBonus;
        this.AddToI();
        if (!this.previewMode)
          this.PreBattleStuff();
        if ( this.game.Data.RuleVar[407] > 0.0)
          this.InitLis();
        if ( this.game.Data.RuleVar[434] > 0.0)
          this.InitSimplifiedSupply();
      }
      this.initialRecon = this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].get_ReconPts(this.AttackerRegime);
      if (this.game.EventRelatedObj.Helper_AirEnabled())
      {
        let mut num: i32 =  0;
        let mut icounter: i32 =  this.ICounter;
        for (let mut index: i32 =  0; index <= icounter; index += 1)
        {
          if (this.IList[index].IAttacker == 1 && this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 2)
            num += this.game.Data.SFTypeObj[this.IList[index].ISFType].ReconPts;
        }
        if (this.CombatType2 == 16)
          this.initialRecon = (int) Math.Round(Math.Ceiling( num / 2.0));
        else if (this.CombatType == 13)
          this.initialRecon = Convert.ToInt32(Math.Ceiling(new Decimal(num * 2)));
        else if (this.CombatType == 5)
          this.initialRecon = (int) Math.Round(Math.Ceiling( num / 2.0));
      }
      if (!this.previewMode && this.customCombatObj.HasCustumCalls())
      {
        CustomCombatCalls customCombatObj = this.customCombatObj;
        combatClass: CombatClass = this;
        ref local: CombatClass = ref combatClass;
        let mut game: GameClass = this.game;
        customCombatObj.PreCombatCall(ref local, game);
      }
      this.CombatReconEndOfRound();
      return orderResult;
    }

    pub fn PayForSimplifiedSupplyRules()
    {
      if (this.CombatRound < 1)
        this.CombatRound = 1;
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index1: i32 =  0; index1 <= ucounter; index1 += 1)
      {
        float a1 = 0.0f;
        float a2 = 0.0f;
        let mut icounter: i32 =  this.ICounter;
        for (let mut index2: i32 =  0; index2 <= icounter; index2 += 1)
        {
          if (this.IList[index2].IUlistNr == index1)
          {
            let mut isfType: i32 =  this.IList[index2].ISFType;
            float num1 = 0.0f;
            float num2 = 0.0f;
            let mut num3: i32 =  this.CombatRound;
            if (this.IList[index2].IRetreated > 0)
              num3 = this.IList[index2].IRetreat;
            let mut num4: i32 =  10;
            if (this.game.Data.SFTypeObj[this.IList[index2].ISFType].EndCombatRound > 0 && this.game.Data.SFTypeObj[this.IList[index2].ISFType].EndCombatRound < this.CombatRound)
              num4 = this.game.Data.SFTypeObj[this.IList[index2].ISFType].EndCombatRound - this.game.Data.SFTypeObj[this.IList[index2].ISFType].StartCombatRound;
            if (this.UList[index1].Uattacker == 1)
            {
              if (this.game.Data.SFTypeObj[isfType].SupplyForAttack > 0)
                num1 +=  this.game.Data.SFTypeObj[isfType].SupplyForAttack /  num4 *  num3;
              if (this.game.Data.SFTypeObj[isfType].FuelForAttack > 0)
              {
                float num5 = 0.0f;
                float val1;
                if (this.UList[index1].UApSpend > 0)
                {
                  float num6 =  Math.Max(25, this.UList[index1].UApMoveCost) /  this.UList[index1].UApSpend;
                  if ( num6 > 1.0)
                    num6 = 1f;
                  float num7 = 1f - num6;
                  if ( num7 < 0.0)
                    num7 = 0.0f;
                  float num8 = VBMath.Rnd();
                  val1 = num5 +  (this.game.Data.SFTypeObj[isfType].FuelForAttack * num3) * num6 +  (this.game.Data.SFTypeObj[isfType].FuelForAttackDef * num3) * num7 * num8 +  ( (this.game.Data.SFTypeObj[isfType].FuelForAttack * num3) *  num7 * (1.0 -  num8));
                }
                else
                  val1 = num5 +  (this.game.Data.SFTypeObj[isfType].FuelForAttackDef * num3);
                float val2 =  this.game.Data.SFTypeObj[isfType].FuelForMove * ( this.UList[index1].UApMoveCost / 10f);
                float num9 = Math.Max(val1, val2);
                num2 += num9;
              }
            }
            else if (this.IList[index2].ILastAttackDone > 0)
            {
              if (this.game.Data.SFTypeObj[isfType].SupplyForAttackDef > 0)
                num1 +=  this.game.Data.SFTypeObj[isfType].SupplyForAttackDef /  num4 *  num3;
              if (this.game.Data.SFTypeObj[isfType].FuelForAttackDef > 0)
                num2 +=  (this.game.Data.SFTypeObj[isfType].FuelForAttackDef * num3);
            }
            float num10;
            float num11;
            if (this.IList[index2].IAttacker == 0)
            {
              num10 = num1 *  (( this.IList[index2].ILisAmmoMod - 0.330000013113022) / 0.660000026226044);
              num11 = num2 *  (( this.IList[index2].ILisFuelMod - 0.330000013113022) / 0.660000026226044);
            }
            else
            {
              num10 = num1 * this.IList[index2].ILisAmmoMod;
              num11 = num2 * this.IList[index2].ILisFuelMod;
            }
            if (this.IList[index2].IleftOutOfPartialAttack)
              num11 = 0.0f;
            if (this.UList[index1].UDefIntercept)
              num11 = 0.0f;
            a1 += num10;
            a2 += num11;
          }
        }
        if (this.UList[index1].Uattacker == 0 & this.InterceptFire)
          a1 = 0.0f;
        if (this.InterceptFire &  this.game.Data.RuleVar[436] > 0.0)
        {
          a1 =  (int) Math.Round(Math.Ceiling( a1 *  this.game.Data.RuleVar[436]));
          a2 = 0.0f;
        }
        if (this.UList[index1].Uattacker == 1)
        {
          if (this.game.Data.RegimeObj[this.AttackerRegime].AI)
          {
            a1 =  (int) Math.Round( a1 * (1.0 -  ((this.game.Data.RegimeObj[this.AttackerRegime].AIHelpCombat + 10) * 2) / 100.0));
            a2 =  (int) Math.Round( a2 * (1.0 -  ((this.game.Data.RegimeObj[this.AttackerRegime].AIHelpCombat + 10) * 2) / 100.0));
          }
        }
        else if (this.game.Data.RegimeObj[this.DefenderRegime].AI)
        {
          a1 =  (int) Math.Round( a1 * (1.0 -  ((this.game.Data.RegimeObj[this.DefenderRegime].AIHelpCombat + 10) * 2) / 100.0));
          a2 =  (int) Math.Round( a2 * (1.0 -  ((this.game.Data.RegimeObj[this.DefenderRegime].AIHelpCombat + 10) * 2) / 100.0));
        }
        UnitClass[] unitObj1 = this.game.Data.UnitObj;
        UnitClass[] unitClassArray1 = unitObj1;
        let mut unr1: i32 =  this.UList[index1].UNr;
        let mut index3: i32 =  unr1;
        unitClassArray1[index3].Supply = unitObj1[unr1].Supply - (int) Math.Round( a1);
        if (0 > this.game.Data.UnitObj[this.UList[index1].UNr].Supply)
          this.game.Data.UnitObj[this.UList[index1].UNr].Supply = 0;
        if ( this.game.Data.RuleVar[435] > 0.0)
        {
          if (this.CombatType == 3 | this.CombatType == 4)
            a2 = 0.0f;
          UnitClass[] unitObj2 = this.game.Data.UnitObj;
          UnitClass[] unitClassArray2 = unitObj2;
          let mut unr2: i32 =  this.UList[index1].UNr;
          let mut index4: i32 =  unr2;
          unitClassArray2[index4].Fuel = unitObj2[unr2].Fuel - (int) Math.Round( a2);
          UnitClass[] unitObj3 = this.game.Data.UnitObj;
          UnitClass[] unitClassArray3 = unitObj3;
          let mut unr3: i32 =  this.UList[index1].UNr;
          let mut index5: i32 =  unr3;
          unitClassArray3[index5].FuelUsedMove = unitObj3[unr3].FuelUsedMove + (int) Math.Round( a2);
          if (0 > this.game.Data.UnitObj[this.UList[index1].UNr].Fuel)
            this.game.Data.UnitObj[this.UList[index1].UNr].Fuel = 0;
        }
      }
    }

    pub fn PayForLis()
    {
      let mut index1: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 2;
      let mut index2: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 3;
      let mut index3: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 4;
      let mut index4: i32 =  17;
      let mut index5: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 0;
      let mut index6: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 1;
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index7: i32 =  0; index7 <= ucounter; index7 += 1)
      {
        SimpleList SL = SimpleList::new();
        let mut combatRound: i32 =  this.CombatRound;
        let mut num1: i32 =  this.CombatType == 3 ? (!(this.UList[index7].URetreatMode == 2 | this.UList[index7].URetreatMode == 3) ? combatRound - 2 : this.UList[index7].URetreat) : combatRound - 2;
        let mut num2: i32 =  0;
        if (this.CombatType == 5 | this.CombatType == 13)
        {
          let mut unr: i32 =  this.UList[index7].UNr;
          num2 = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0, this.TargetX, this.TargetY, 0, 99);
        }
        if (num1 < 1)
          num1 = 1;
        let mut icounter: i32 =  this.ICounter;
        for (let mut index8: i32 =  0; index8 <= icounter; index8 += 1)
        {
          if (this.IList[index8].IUlistNr == index7)
          {
            let mut isfType: i32 =  this.IList[index8].ISFType;
            let mut num3: i32 =  num1;
            if (num2 > 0)
            {
              let mut num4: i32 =  (int) Math.Round(Math.Ceiling( (num2 * this.game.Data.SFTypeObj[isfType].AirAPRule) / 10.0));
              if (num4 > num3)
                num3 = num4;
            }
            if (this.CombatType2 == 16 | this.CombatType == 13 | this.CombatType == 5 & this.IList[index8].IAttacker == 0)
            {
              if (this.game.Data.SFTypeObj[isfType].SFTypeVar[index6] > 0)
                SL.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index5], (int) Math.Round( (this.game.Data.SFTypeObj[isfType].SFTypeVar[index6] * num3) * 0.5));
            }
            else if (this.CombatType != 3 && this.game.Data.SFTypeObj[isfType].SFTypeVar[index6] > 0)
              SL.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index5], this.game.Data.SFTypeObj[isfType].SFTypeVar[index6] * num3);
            if (!(this.game.Data.SFTypeObj[this.IList[index8].ISFType].ArtRange > 0 &  this.IList[index8].ILisAmmoMod < 1.0))
            {
              if (this.game.Data.SFTypeObj[this.IList[index8].ISFType].Theater == 2)
              {
                if (this.IList[index8].RoundsOfLandAttack > 0 && this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] > 0)
                {
                  let mut tweight: i32 =  this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] * this.IList[index8].RoundsOfLandAttack;
                  SL.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index1], tweight);
                }
                if (this.IList[index8].RoundsOfAirAttack > 0 && this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] > 0)
                {
                  let mut tweight: i32 =  this.game.Data.SFTypeObj[isfType].SFTypeVar[index4] * this.IList[index8].RoundsOfAirAttack;
                  SL.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index1], tweight);
                }
              }
              else if (this.UList[index7].Uattacker == 1)
              {
                if (this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] > 0)
                  SL.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index1], this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] * num1);
              }
              else if (this.game.Data.SFTypeObj[isfType].SFTypeVar[index3] > 0)
                SL.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index1], this.game.Data.SFTypeObj[isfType].SFTypeVar[index3] * num1);
            }
            else
              index7 = index7;
            index7 = index7;
          }
        }
        let mut counter: i32 =  SL.Counter;
        for (let mut index9: i32 =  0; index9 <= counter; index9 += 1)
        {
          let mut num5: i32 =  SL.Weight[index9];
          let mut num6: i32 =  (int) Math.Round(Math.Ceiling( SL.Weight[index9] / 10.0));
          let mut num7: i32 =  num5 - num6 * 10;
          if (num7 > 0 && DrawMod.RandyNumber.Next(0, 10) <= num7)
            num6 += 1;
          SL.Weight[index9] = num6;
        }
        this.game.Data.UnitObj[this.UList[index7].UNr].items.list.RemoveWeight(ref SL);
        this.game.Data.UnitObj[this.UList[index7].UNr].items.list.removeWeight0orLower();
      }
    }

    pub fn InitLis()
    {
      let mut index1: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 2;
      let mut index2: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 3;
      let mut index3: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 4;
      let mut index4: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 0;
      let mut index5: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 1;
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index6: i32 =  0; index6 <= ucounter; index6 += 1)
      {
        let mut num1: i32 =  0;
        if (this.CombatType == 5 | this.CombatType == 13)
          num1 = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index6].UNr].X, this.game.Data.UnitObj[this.UList[index6].UNr].Y, 0, this.TargetX, this.TargetY, 0, 99);
        SimpleList simpleList1 = this.game.Data.UnitObj[this.UList[index6].UNr].items.list.Clone();
        SimpleList simpleList2 = SimpleList::new();
        let mut icounter1: i32 =  this.ICounter;
        for (let mut index7: i32 =  0; index7 <= icounter1; index7 += 1)
        {
          if (this.IList[index7].IUlistNr == index6)
          {
            let mut isfType: i32 =  this.IList[index7].ISFType;
            float num2 = 1f;
            let mut num3: i32 =  this.game.Data.SFObj[this.IList[index7].ISFNr].Ap;
            if (this.game.Data.SFTypeObj[this.IList[index7].ISFType].EndCombatRound > 0)
              num3 = this.game.Data.SFTypeObj[this.IList[index7].ISFType].EndCombatRound * 10;
            if (num3 < 100)
              num2 =  num3 / 100f;
            float num4 = num2;
            if (this.UList[index6].UDefIntercept && num1 > 0)
            {
              let mut val1: i32 =  (int) Math.Round(Math.Ceiling( (num1 * this.game.Data.SFTypeObj[isfType].AirAPRule) / 10.0));
              let mut num5: i32 =  this.CombatType != 13 ? Math.Max(val1, 10) : Math.Max(val1, 3);
              num4 =  ( num4 *  num5 / 10.0);
            }
            if (this.CombatType2 == 16 | this.CombatType == 13 | this.CombatType == 5 & this.IList[index7].IAttacker == 0)
            {
              if (this.game.Data.SFTypeObj[isfType].SFTypeVar[index5] > 0)
                simpleList2.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index4], (int) Math.Round( this.game.Data.SFTypeObj[isfType].SFTypeVar[index5] *  num4 * 0.5));
            }
            else if (this.CombatType != 3 && this.game.Data.SFTypeObj[isfType].SFTypeVar[index5] > 0 && !(this.CombatType == 3 | this.CombatType == 4))
              simpleList2.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index4], (int) Math.Round( ( this.game.Data.SFTypeObj[isfType].SFTypeVar[index5] * num2)));
            if (this.UList[index6].Uattacker == 1)
            {
              if (this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] > 0)
                simpleList2.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index1], (int) Math.Round(Math.Ceiling( this.game.Data.SFTypeObj[isfType].SFTypeVar[index2] *  num2)));
            }
            else if (this.game.Data.SFTypeObj[isfType].SFTypeVar[index3] > 0)
              simpleList2.AddWeight(this.game.Data.SFTypeObj[isfType].SFTypeVar[index1], (int) Math.Round(Math.Ceiling( this.game.Data.SFTypeObj[isfType].SFTypeVar[index3] *  num2)));
          }
        }
        let mut icounter2: i32 =  this.ICounter;
        for (let mut index8: i32 =  0; index8 <= icounter2; index8 += 1)
        {
          if (this.IList[index8].IUlistNr == index6)
          {
            let mut isfType: i32 =  this.IList[index8].ISFType;
            let mut tid1: i32 =  this.game.Data.SFTypeObj[isfType].SFTypeVar[index1];
            let mut weight1: i32 =  simpleList1.FindWeight(tid1);
            let mut weight2: i32 =  simpleList2.FindWeight(tid1);
            float num6 = (this.IList[index8].IAttacker != 1 ? this.game.Data.SFTypeObj[isfType].SFTypeVar[index3] : this.game.Data.SFTypeObj[isfType].SFTypeVar[index2]) <= 0 ? 1f : (weight2 <= 0 ? 1f : Math.Min(1f,  weight1 /  weight2));
            let mut tid2: i32 =  this.game.Data.SFTypeObj[isfType].SFTypeVar[index4];
            let mut weight3: i32 =  simpleList1.FindWeight(tid2);
            let mut weight4: i32 =  simpleList2.FindWeight(tid2);
            float num7 = this.game.Data.SFTypeObj[isfType].SFTypeVar[index5] <= 0 ? 1f : (weight4 <= 0 ? 1f : Math.Min(1f,  weight3 /  weight4));
            if (this.CombatType == 3 | this.CombatType == 4)
              num7 = 1f;
            this.IList[index8].ILisAmmoMod = num6;
            this.IList[index8].ILisFuelMod = num7;
          }
        }
      }
    }

    pub CheckEnoughFuelAmmo: bool(unr: i32, bool forAA)
    {
      let mut num1: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0, this.TargetX, this.TargetY, 0);
      let mut val2: i32 =  10;
      if (this.CombatType == 13)
        val2 = 3;
      let mut index1: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 2;
      let mut num2: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 3;
      let mut index2: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 4;
      let mut index3: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 0;
      let mut index4: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 1;
      SimpleList simpleList1 = this.game.Data.UnitObj[unr].items.list.Clone();
      SimpleList simpleList2 = SimpleList::new();
      let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index5: i32 =  0; index5 <= sfCount; index5 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index5];
        let mut type: i32 =  this.game.Data.SFObj[sf].Type;
        float num3 = 1f;
        float num4 = 1f;
        if (val2 < 10)
        {
          num4 =  val2 / 10f;
          num3 =  val2 / 10f;
        }
        if (num1 > 0)
        {
          let mut num5: i32 =  Math.Max((int) Math.Round(Math.Ceiling( (num1 * this.game.Data.SFTypeObj[type].AirAPRule) / 10.0)), val2);
          num4 =  ( num4 *  num5 / 10.0);
        }
        Math.Min(this.game.Data.SFObj[sf].Ap, this.game.Data.SFObj[sf].Rdn);
        if (this.CombatType2 == 16 | (this.CombatType == 13 | this.CombatType == 5) & !forAA && this.game.Data.SFTypeObj[type].SFTypeVar[index4] > 0)
          simpleList2.AddWeight(this.game.Data.SFTypeObj[type].SFTypeVar[index3], (int) Math.Round( this.game.Data.SFTypeObj[type].SFTypeVar[index4] *  num4 * 0.5));
        if (this.game.Data.SFTypeObj[type].SFTypeVar[index2] > 0)
          simpleList2.AddWeight(this.game.Data.SFTypeObj[type].SFTypeVar[index1], (int) Math.Round( ( this.game.Data.SFTypeObj[type].SFTypeVar[index2] * num3)));
      }
      let mut num6: i32 =  10;
      let mut weight1: i32 =  simpleList1.FindWeight(10);
      let mut weight2: i32 =  simpleList2.FindWeight(10);
      float num7 = weight2 <= 0 ? 1f : Math.Min(1f,  weight1 /  weight2);
      num6 = 1;
      let mut num8: i32 =  simpleList1.FindWeight(1) + simpleList1.FindWeight(15);
      let mut num9: i32 =  simpleList2.FindWeight(1) + simpleList1.FindWeight(15);
      float num10 = num9 <= 0 ? 1f : Math.Min(1f,  num8 /  num9);
      if (this.CombatType == 3 | this.CombatType == 4)
        num10 = 1f;
      return  num7 >= 1.0 &  num10 >= 1.0;
    }

    pub fn InitSimplifiedSupply()
    {
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index1: i32 =  0; index1 <= ucounter; index1 += 1)
      {
        float num1 = 0.0f;
        float num2 = 0.0f;
        float num3 = 0.0f;
        float num4 = 0.0f;
        let mut icounter1: i32 =  this.ICounter;
        for (let mut index2: i32 =  0; index2 <= icounter1; index2 += 1)
        {
          if (this.IList[index2].IUlistNr == index1)
          {
            bool flag = false;
            if (this.IList[index2].IleftOutOfPartialAttack)
              flag = true;
            if (this.UList[index1].UDefIntercept)
              flag = true;
            if (this.CombatType == 3 | this.CombatType == 4)
              flag = true;
            if (this.InterceptFire &  this.game.Data.RuleVar[436] > 0.0)
              flag = true;
            let mut isfType: i32 =  this.IList[index2].ISFType;
            let mut num5: i32 =  10;
            if (this.UList[index1].Uattacker == 1 && (int) Math.Round(Math.Floor( this.UList[index1].UMaxApToSpend / 10.0)) < num5)
              num5 = (int) Math.Round(Math.Floor( this.UList[index1].UMaxApToSpend / 10.0));
            if (this.InterceptFire | this.CombatType == 11)
              num5 = 2;
            if (this.UList[index1].USupportInterceptFire)
              num5 = 2;
            if (this.game.Data.SFTypeObj[isfType].EndCombatRound > 0 & this.game.Data.SFTypeObj[isfType].EndCombatRound < num5)
              num5 = this.game.Data.SFTypeObj[isfType].EndCombatRound - this.game.Data.SFTypeObj[isfType].StartCombatRound;
            if (this.UList[index1].Uattacker == 0)
              num5 *= 2;
            if (this.UList[index1].Uattacker == 1)
              num5 = (int) Math.Round( num5 * 1.25);
            if (this.UList[index1].Uattacker == 1)
            {
              if (this.game.Data.SFTypeObj[isfType].FuelForAttack > 0 & !flag)
                num1 +=  (this.game.Data.SFTypeObj[isfType].FuelForAttack * num5);
              if (this.game.Data.SFTypeObj[isfType].SupplyForAttack > 0)
                num2 +=  this.game.Data.SFTypeObj[isfType].SupplyForAttack / 10f *  num5;
            }
            else
            {
              if (this.game.Data.SFTypeObj[isfType].FuelForAttackDef > 0 & !flag)
                num1 +=  this.game.Data.SFTypeObj[isfType].FuelForAttackDef *  Math.Ceiling( num5 / 2.0);
              if (this.game.Data.SFTypeObj[isfType].SupplyForAttackDef > 0)
                num2 +=  this.game.Data.SFTypeObj[isfType].SupplyForAttackDef / 10f *  num5;
            }
            let mut num6: i32 =  10;
            if (this.UList[index1].Uattacker == 1)
            {
              if (this.game.Data.SFTypeObj[isfType].FuelForAttack > 0 & !flag)
                num3 +=  (this.game.Data.SFTypeObj[isfType].FuelForAttack * num6);
              if (this.game.Data.SFTypeObj[isfType].SupplyForAttack > 0)
                num4 +=  this.game.Data.SFTypeObj[isfType].SupplyForAttack / 10f *  num6;
            }
            else
            {
              if (this.game.Data.SFTypeObj[isfType].FuelForAttackDef > 0 & !flag)
                num3 +=  (this.game.Data.SFTypeObj[isfType].FuelForAttackDef * num6);
              if (this.game.Data.SFTypeObj[isfType].SupplyForAttackDef > 0)
                num4 +=  this.game.Data.SFTypeObj[isfType].SupplyForAttackDef / 10f *  num6;
            }
          }
        }
        if ( this.game.Data.RuleVar[436] > 0.0 & this.InterceptFire)
        {
          num2 =  (int) Math.Round(Math.Ceiling( num2 *  this.game.Data.RuleVar[436]));
          num4 =  (int) Math.Round(Math.Ceiling( num4 *  this.game.Data.RuleVar[436]));
        }
        if (this.UList[index1].Uattacker == 1)
        {
          if (this.game.Data.RegimeObj[this.AttackerRegime].AI)
          {
            num2 =  (int) Math.Round( num2 * (1.0 -  ((this.game.Data.RegimeObj[this.AttackerRegime].AIHelpCombat + 10) * 2) / 100.0));
            num4 =  (int) Math.Round( num4 * (1.0 -  ((this.game.Data.RegimeObj[this.AttackerRegime].AIHelpCombat + 10) * 2) / 100.0));
            num1 =  (int) Math.Round( num1 * (1.0 -  ((this.game.Data.RegimeObj[this.AttackerRegime].AIHelpCombat + 10) * 2) / 100.0));
            num3 =  (int) Math.Round( num3 * (1.0 -  ((this.game.Data.RegimeObj[this.AttackerRegime].AIHelpCombat + 10) * 2) / 100.0));
          }
        }
        else if (this.game.Data.RegimeObj[this.DefenderRegime].AI)
        {
          num2 =  (int) Math.Round( num2 * (1.0 -  ((this.game.Data.RegimeObj[this.DefenderRegime].AIHelpCombat + 10) * 2) / 100.0));
          num4 =  (int) Math.Round( num4 * (1.0 -  ((this.game.Data.RegimeObj[this.DefenderRegime].AIHelpCombat + 10) * 2) / 100.0));
          num1 =  (int) Math.Round( num1 * (1.0 -  ((this.game.Data.RegimeObj[this.DefenderRegime].AIHelpCombat + 10) * 2) / 100.0));
          num3 =  (int) Math.Round( num3 * (1.0 -  ((this.game.Data.RegimeObj[this.DefenderRegime].AIHelpCombat + 10) * 2) / 100.0));
        }
        float num7 = 1f;
        float num8 = 1f;
        if ( num1 > 0.0)
        {
          num7 =  this.game.Data.UnitObj[this.UList[index1].UNr].Fuel / num1;
          if ( num7 <  (this.game.Data.RuleVar[439] / 100f))
            num7 =  this.game.Data.UnitObj[this.UList[index1].UNr].Fuel / num3;
        }
        if ( num2 > 0.0)
        {
          num8 =  this.game.Data.UnitObj[this.UList[index1].UNr].Supply / num2;
          if ( num8 <  (this.game.Data.RuleVar[439] / 100f))
            num8 =  this.game.Data.UnitObj[this.UList[index1].UNr].Supply / num4;
        }
        if ( num7 <= 1.0)
          ;
        if ( num8 > 1.0)
          num8 = 1f;
        if ( num7 > 1.0)
          num7 = 1f;
        if ( this.game.Data.RuleVar[435] < 1.0)
          num7 = 0.0f;
        let mut icounter2: i32 =  this.ICounter;
        for (let mut index3: i32 =  0; index3 <= icounter2; index3 += 1)
        {
          if (this.IList[index3].IUlistNr == index1)
          {
            if (this.IList[index3].IAttacker == 1)
            {
              this.IList[index3].ILisAmmoMod = num8 + this.game.Data.SFTypeObj[this.IList[index3].ISFType].OutOfSupplyAttack * (1f - num8);
              this.IList[index3].ILisFuelMod = num7 + this.game.Data.SFTypeObj[this.IList[index3].ISFType].OutOfFuelAttack * (1f - num7);
              if (this.game.Data.SFTypeObj[this.IList[index3].ISFType].SupplyForAttack < 1)
                this.IList[index3].ILisAmmoMod = 1f;
              if (this.game.Data.SFTypeObj[this.IList[index3].ISFType].FuelForAttack < 1)
                this.IList[index3].ILisFuelMod = 1f;
            }
            else if (!this.InterceptFire)
            {
              this.IList[index3].ILisAmmoMod = num8 + this.game.Data.SFTypeObj[this.IList[index3].ISFType].OutOfSupplyDefense * (1f - num8);
              this.IList[index3].ILisFuelMod = num7 + this.game.Data.SFTypeObj[this.IList[index3].ISFType].OutOfFuelDefense * (1f - num7);
              this.IList[index3].ILisAmmoMod = Math.Min(1f, 0.34f + this.IList[index3].ILisAmmoMod * 0.67f);
              this.IList[index3].ILisFuelMod = Math.Min(1f, 0.34f + this.IList[index3].ILisFuelMod * 0.67f);
              if (this.game.Data.SFTypeObj[this.IList[index3].ISFType].SupplyForAttackDef < 1)
                this.IList[index3].ILisAmmoMod = 1f;
              if (this.game.Data.SFTypeObj[this.IList[index3].ISFType].FuelForAttackDef < 1)
                this.IList[index3].ILisFuelMod = 1f;
            }
            else
            {
              this.IList[index3].ILisAmmoMod = 1f;
              this.IList[index3].ILisFuelMod = 1f;
            }
          }
        }
      }
    }

    pub fn CheckSeaAttackBreakOff()
    {
      if (this.CombatType != 2 || this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea)
        return;
      let mut num: i32 =  0;
      let mut icounter: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter; index += 1)
      {
        if (this.IList[index].IAttacker == 0 & this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 1)
          num += 1;
      }
      if (num != 0)
        return;
      this.BattleEnded = 1;
    }

    pub fn DoRound()
    {
      this.CheckSeaAttackBreakOff();
      if (this.BattleEnded > 0)
        return;
      if (this.game.EditObj.CombatSim)
        VBMath.Randomize();
      this += 1.CombatRound;
      if (this.CombatRound == 1)
        this.ThrowInitiative();
      if (this.CombatRound == 1)
        this.SortOnInitiative();
      let mut index1: i32 =  0;
      let mut index2: i32 =  1;
      string[,] matrx1 = new string[61, 6];
      let mut ucounter1: i32 =  this.UCounter;
      txt: String;
      str1: String;
      for (let mut fromy: i32 =  0; fromy <= ucounter1; fromy += 1)
      {
        if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.UList[fromy].UNr].Regime, this.game.Data.Turn))
        {
          matrx1[1, index1] = "Move cost";
          matrx1[1, index2] = Strings.Trim(Conversion.Str( this.UList[fromy].UApMoveCost));
          matrx1[2, index1] = "AP spend so far";
          matrx1[2, index2] = Strings.Trim(Conversion.Str( this.UList[fromy].UApSpend));
          matrx1[3, index1] = "Max AP to spend";
          matrx1[3, index2] = Strings.Trim(Conversion.Str( this.UList[fromy].UMaxApToSpend));
          matrx1[4, index1] = "Retreat started in round";
          matrx1[4, index2] = Strings.Trim(Conversion.Str( this.UList[fromy].URetreat));
          matrx1[5, index1] = "Broken".to_owned();
          matrx1[5, index2] = Strings.Trim(Conversion.Str( this.UList[fromy].UBreaks));
          matrx1[6, index1] = "Retreat".to_owned();
          matrx1[6, index2] = !(this.UList[fromy].URetreatMode > 0 & this.UList[fromy].URetreatMode <= 2) ? (this.UList[fromy].URetreatMode != 5 ? (this.UList[fromy].URetreatMode <= 0 ? "-" : "Retreating") : "Panicking") : "Orderly/Out of AP";
          this.AddReport(2, "Stats at start of round", txt, fromy, this.CombatRound, matrx1);
          str1 = "";
        }
      }
      string[,] matrx2 = new string[61, 6];
      let mut icounter1: i32 =  this.ICounter;
      for (let mut index3: i32 =  0; index3 <= icounter1; index3 += 1)
      {
        if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.IList[index3].IUnr].Regime, this.game.Data.Turn))
        {
          matrx2[1, index1] = "Readiness".to_owned();
          matrx2[1, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IRdn));
          matrx2[2, index1] = "Morale".to_owned();
          matrx2[2, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IMor));
          matrx2[3, index1] = "Experience".to_owned();
          matrx2[3, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IXp));
          matrx2[4, index1] = "Entrenchment".to_owned();
          matrx2[4, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IEntrench));
          matrx2[5, index1] = "AA out of hex";
          matrx2[5, index2] = Strings.Trim(Conversion.Str( (1 == this.IList[index3].IAA)));
          matrx2[6, index1] = "Breakthrough".to_owned();
          matrx2[6, index2] = Strings.Trim(Conversion.Str( (this.IList[index3].IBreakTrough == 1)));
          matrx2[7, index1] = "Broken".to_owned();
          matrx2[7, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IBroken));
          matrx2[8, index1] = "Capitulated".to_owned();
          matrx2[8, index2] = Strings.Trim(Conversion.Str( this.IList[index3].ICapitulate));
          matrx2[9, index1] = "Killed".to_owned();
          matrx2[9, index2] = Strings.Trim(Conversion.Str( (this.IList[index3].IKilled > 0)));
          matrx2[10, index1] = "Retreat completed";
          matrx2[10, index2] = Strings.Trim(Conversion.Str( (this.IList[index3].IRetreated > 0)));
          matrx2[11, index1] = "Retreat started in round";
          matrx2[11, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IRetreat));
          matrx2[12, index1] = "Retreat Mode";
          matrx2[12, index2] = this.IList[index3].IRetreatMode != 1 ? (this.IList[index3].IRetreatMode != 2 ? (this.IList[index3].IRetreatMode != 3 ? (this.IList[index3].IRetreatMode <= 0 ? "-" : "Retreating") : "Panicking") : "Orderly/Out of AP") : "Forced by combat result";
          matrx2[13, index1] = "Attack Count";
          matrx2[13, index2] = Strings.Trim(Conversion.Str( this.IList[index3].AttackCount));
          matrx2[14, index1] = "Attacked Count";
          matrx2[14, index2] = Strings.Trim(Conversion.Str( this.IList[index3].AttackedCount));
          matrx2[15, index1] = "Last Attack Done";
          matrx2[15, index2] = Strings.Trim(Conversion.Str( this.IList[index3].ILastAttackDone));
          matrx2[16, index1] = "Last Attacked";
          matrx2[16, index2] = Strings.Trim(Conversion.Str( this.IList[index3].ILastAttacked));
          matrx2[17, index1] = "Last Hit";
          matrx2[17, index2] = Strings.Trim(Conversion.Str( this.IList[index3].ILastHit));
          matrx2[18, index1] = "Last Targeted";
          matrx2[18, index2] = Strings.Trim(Conversion.Str( this.IList[index3].ILastTargeted));
          matrx2[19, index1] = "PreventPoints given";
          matrx2[19, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IPreventPointsGiven));
          matrx2[20, index1] = "PreventPoints used";
          matrx2[20, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IPreventPointsUsed));
          if ( this.game.Data.RuleVar[431] > 0.0)
          {
            matrx2[21, index1] = "Cover Points";
            matrx2[21, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IcoverPoints));
            matrx2[22, index1] = "Visible From Round";
            matrx2[22, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IvisibleFromRound));
          }
          if ( this.game.Data.RuleVar[493] > 0.0)
          {
            matrx2[23, index1] = "Left behind from partial att.";
            matrx2[23, index2] = Strings.Trim(Conversion.Str( this.IList[index3].IleftOutOfPartialAttack));
          }
          if (this.IList[index3].IAttacker == 1)
            index3 = index3;
          this.AddReport(2, "Stats at start of round", txt, index3 + 10000, this.CombatRound, matrx2);
        }
        str1 = "";
        str2: String = -(this.game.TempCombat.IList[index3].IBroken ? 1 : 0) <= 0 ? (this.game.TempCombat.IList[index3].IKilled <= 0 ? ((uint) this.game.TempCombat.IList[index3].IRetreated <= 0U ? (!(this.game.TempCombat.IList[index3].IRetreat > 0 & this.game.TempCombat.IList[index3].IRetreatMode == 3) ? (this.game.TempCombat.IList[index3].IRetreat <= 0 ? (this.game.TempCombat.IList[index3].IBreakTrough <= 0 ? "" : "!") : "RTR-ING") : "PANICK") : "RTR-ED") : (!this.game.TempCombat.IList[index3].ICapitulate ? "DEAD" : "CAPITULATE")) : "BROKEN".to_owned();
        this.IList[index3].IHistoricState = (string[]) Utils.CopyArray((Array) this.IList[index3].IHistoricState, (Array) new string[this.CombatRound + 1]);
        this.game.TempCombat.IList[index3].IHistoricState[this.CombatRound] = str2;
      }
      this.AddDetail("COMBATROUND = " + Conversion.Str( this.CombatRound));
      if (this.customCombatObj.HasCustumCalls())
      {
        CustomCombatCalls customCombatObj = this.customCombatObj;
        combatClass: CombatClass = this;
        ref local: CombatClass = ref combatClass;
        let mut game: GameClass = this.game;
        let mut combatRound: i32 =  this.CombatRound;
        customCombatObj.StartCombatRound(ref local, game, combatRound);
      }
      this.ClearDetail();
      if (this.CombatRound == 1)
        this.CheckBreakthrough();
      this.CheckCapitulation(false);
      this.SetConcentricBonus();
      this.DoSteps();
      this.CheckAutoDestroy();
      this.PlayRelevantSound();
      this.CheckBreakthrough();
      this.CheckSafeRetreat();
      if (this.game.Data.Product == 6)
      {
        this.CheckPanicUnitRetreat();
        this.CheckCapitulation(true);
        this.CheckOrderlyUnitRetreat();
        this.CheckBreak();
      }
      else
      {
        this.CheckOrderlyUnitRetreat();
        this.CheckPanicUnitRetreat();
        this.CheckBreak();
        this.CheckCapitulation(true);
      }
      this.CheckOutOfAP();
      this.CheckVictory();
      this.CombatReconEndOfRound();
      this.FinishStats();
      let mut icounter2: i32 =  this.ICounter;
      for (let mut index4: i32 =  0; index4 <= icounter2; index4 += 1)
      {
        if (this.IList[index4].AttackCount > 0)
        {
          Individual[] ilist = this.IList;
          Individual[] individualArray = ilist;
          let mut index5: i32 =  index4;
          let mut index6: i32 =  index5;
          individualArray[index6].RoundsOfAttack = ilist[index5].RoundsOfAttack + 1;
        }
        if (this.IList[index4].AttackCountAir > 0)
        {
          Individual[] ilist = this.IList;
          Individual[] individualArray = ilist;
          let mut index7: i32 =  index4;
          let mut index8: i32 =  index7;
          individualArray[index8].RoundsOfAirAttack = ilist[index7].RoundsOfAirAttack + 1;
        }
        if (this.IList[index4].AttackCountLand > 0)
        {
          Individual[] ilist = this.IList;
          Individual[] individualArray = ilist;
          let mut index9: i32 =  index4;
          let mut index10: i32 =  index9;
          individualArray[index10].RoundsOfLandAttack = ilist[index9].RoundsOfLandAttack + 1;
        }
      }
      let mut index11: i32 =  0;
      let mut index12: i32 =  1;
      string[,] matrx3 = new string[61, 6];
      let mut icounter3: i32 =  this.ICounter;
      for (let mut index13: i32 =  0; index13 <= icounter3; index13 += 1)
      {
        if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.IList[index13].IUnr].Regime, this.game.Data.Turn))
        {
          matrx3[1, index11] = "Readiness".to_owned();
          matrx3[1, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IRdn));
          matrx3[2, index11] = "Morale".to_owned();
          matrx3[2, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IMor));
          matrx3[3, index11] = "Experience".to_owned();
          matrx3[3, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IXp));
          matrx3[4, index11] = "Entrenchment".to_owned();
          matrx3[4, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IEntrench));
          matrx3[5, index11] = "AA out of hex";
          matrx3[5, index12] = Strings.Trim(Conversion.Str( (1 == this.IList[index13].IAA)));
          matrx3[6, index11] = "Breakthrough".to_owned();
          matrx3[6, index12] = Strings.Trim(Conversion.Str( (this.IList[index13].IBreakTrough == 1)));
          matrx3[7, index11] = "Broken".to_owned();
          matrx3[7, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IBroken));
          matrx3[8, index11] = "Capitulated".to_owned();
          matrx3[8, index12] = Strings.Trim(Conversion.Str( this.IList[index13].ICapitulate));
          matrx3[9, index11] = "Killed".to_owned();
          matrx3[9, index12] = Strings.Trim(Conversion.Str( (this.IList[index13].IKilled > 0)));
          matrx3[10, index11] = "Retreat completed";
          matrx3[10, index12] = Strings.Trim(Conversion.Str( (this.IList[index13].IRetreated > 0)));
          matrx3[11, index11] = "Retreat started in round";
          matrx3[11, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IRetreat));
          matrx3[12, index11] = "Retreat Mode";
          matrx3[12, index12] = this.IList[index13].IRetreatMode > 2 ? (this.IList[index13].IRetreatMode != 3 ? (this.IList[index13].IRetreatMode <= 0 ? "-" : "Retreating") : "Panicking") : "Orderly/Out of AP";
          matrx3[13, index11] = "Attack Count";
          matrx3[13, index12] = Strings.Trim(Conversion.Str( this.IList[index13].AttackCount));
          matrx3[14, index11] = "Attacked Count";
          matrx3[14, index12] = Strings.Trim(Conversion.Str( this.IList[index13].AttackedCount));
          matrx3[15, index11] = "Last Attack Done";
          matrx3[15, index12] = Strings.Trim(Conversion.Str( this.IList[index13].ILastAttackDone));
          matrx3[16, index11] = "Last Attacked";
          matrx3[16, index12] = Strings.Trim(Conversion.Str( this.IList[index13].ILastAttacked));
          matrx3[17, index11] = "Last Hit";
          matrx3[17, index12] = Strings.Trim(Conversion.Str( this.IList[index13].ILastHit));
          matrx3[18, index11] = "Last Targeted";
          matrx3[18, index12] = Strings.Trim(Conversion.Str( this.IList[index13].ILastTargeted));
          matrx3[19, index11] = "PreventPoints given";
          matrx3[19, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IPreventPointsGiven));
          matrx3[20, index11] = "PreventPoints used";
          matrx3[20, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IPreventPointsUsed));
          if ( this.game.Data.RuleVar[431] > 0.0)
          {
            matrx3[21, index11] = "Cover Points";
            matrx3[21, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IcoverPoints));
            matrx3[22, index11] = "Visible From Round";
            matrx3[22, index12] = Strings.Trim(Conversion.Str( this.IList[index13].IvisibleFromRound));
          }
          if (index13 > 44)
            index13 = index13;
          this.AddReport(2, "Stats at end of round", txt, index13 + 10000, this.CombatRound, matrx3);
        }
        str1 = "";
        str3: String = -(this.game.TempCombat.IList[index13].IBroken ? 1 : 0) <= 0 ? (this.game.TempCombat.IList[index13].IKilled <= 0 ? ((uint) this.game.TempCombat.IList[index13].IRetreated <= 0U ? (!(this.game.TempCombat.IList[index13].IRetreat > 0 & this.game.TempCombat.IList[index13].IRetreatMode == 3) ? (this.game.TempCombat.IList[index13].IRetreat <= 0 ? (this.game.TempCombat.IList[index13].IBreakTrough <= 0 ? "" : "!") : "RTR-ING") : "PANICKING") : "RTR-ED") : (!this.game.TempCombat.IList[index13].ICapitulate ? "DEAD" : "CAPITULATE")) : "BROKEN".to_owned();
        this.IList[index13].IHistoricState2 = (string[]) Utils.CopyArray((Array) this.IList[index13].IHistoricState2, (Array) new string[this.CombatRound + 1]);
        this.game.TempCombat.IList[index13].IHistoricState2[this.CombatRound] = str3;
      }
      string[,] matrx4 = new string[61, 6];
      let mut ucounter2: i32 =  this.UCounter;
      for (let mut fromy: i32 =  0; fromy <= ucounter2; fromy += 1)
      {
        if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.UList[fromy].UNr].Regime, this.game.Data.Turn))
        {
          matrx4[1, index11] = "Move cost";
          matrx4[1, index12] = Strings.Trim(Conversion.Str( this.UList[fromy].UApMoveCost));
          matrx4[2, index11] = "AP spend so far";
          matrx4[2, index12] = Strings.Trim(Conversion.Str( this.UList[fromy].UApSpend));
          matrx4[3, index11] = "Max AP to spend";
          matrx4[3, index12] = Strings.Trim(Conversion.Str( this.UList[fromy].UMaxApToSpend));
          matrx4[4, index11] = "Retreat started in round";
          matrx4[4, index12] = Strings.Trim(Conversion.Str( this.UList[fromy].URetreat));
          matrx4[5, index11] = "Broken".to_owned();
          matrx4[5, index12] = Strings.Trim(Conversion.Str( this.UList[fromy].UBreaks));
          matrx4[6, index11] = "Retreat".to_owned();
          matrx4[6, index12] = !(this.UList[fromy].URetreatMode > 0 & this.UList[fromy].URetreatMode <= 2) ? (this.UList[fromy].URetreatMode != 5 ? (this.UList[fromy].URetreatMode <= 0 ? "-" : "Retreating") : "Panicking") : "Orderly/Out of AP";
          this.AddReport(2, "Stats at end of round", txt, fromy, this.CombatRound, matrx4);
          str1 = "";
        }
      }
    }

    pub fn PlayRelevantSound()
    {
      if (this.previewMode || this.dontUseSfx || this.ICounter < 0)
        return;
      let mut num1: i32 =  0;
      let mut icounter1: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IDammageDone > 0)
        {
          let mut isfType: i32 =  this.IList[index].ISFType;
          if (Strings.Len(this.game.Data.SFTypeObj[isfType].BattleWAV) > 0)
            num1 += this.game.Data.SFTypeObj[isfType].PowerPts;
        }
      }
      let mut index1: i32 =  -1;
      let mut num2: i32 =  0;
      let mut num3: i32 =  (int) Math.Round( VBMath.Rnd() *  this.ICounter + 1.0);
      while (index1 == -1 & num1 > 0)
      {
        num2 += 1;
        let mut num4: i32 =  num3;
        let mut icounter2: i32 =  this.ICounter;
        for (let mut index2: i32 =  num4; index2 <= icounter2; index2 += 1)
        {
          if (this.IList[index2].IDammageDone > 0 && this.IList[index2].IAttacker == 1 & (this.CombatRound + 1) % 2 == 0 | this.IList[index2].IAttacker == 0 & (this.CombatRound + 1) % 2 > 0)
          {
            let mut isfType: i32 =  this.IList[index2].ISFType;
            if (Strings.Len(this.game.Data.SFTypeObj[isfType].BattleWAV) > 0 &&  this.game.Data.SFTypeObj[isfType].PowerPts >  VBMath.Rnd() *  num1)
            {
              index1 = isfType;
              break;
            }
          }
        }
        num3 = 0;
        if (num2 > 999)
          index1 = -2;
      }
      if (index1 < 0)
      {
        let mut icounter3: i32 =  this.ICounter;
        for (let mut index3: i32 =  0; index3 <= icounter3; index3 += 1)
        {
          if (this.IList[index3].IAttacker == 1 & (this.CombatRound + 1) % 2 == 0)
          {
            let mut isfType: i32 =  this.IList[index3].ISFType;
            if (Strings.Len(this.game.Data.SFTypeObj[isfType].BattleWAV) > 0)
              num1 += this.game.Data.SFTypeObj[isfType].PowerPts;
          }
        }
        index1 = -1;
        let mut num5: i32 =  0;
        let mut num6: i32 =  (int) Math.Round( VBMath.Rnd() *  this.ICounter + 1.0);
        while (index1 == -1)
        {
          num5 += 1;
          let mut num7: i32 =  num6;
          let mut icounter4: i32 =  this.ICounter;
          for (let mut index4: i32 =  num7; index4 <= icounter4; index4 += 1)
          {
            if (this.IList[index4].IAttacker == 1 & (this.CombatRound + 1) % 2 == 0)
            {
              let mut isfType: i32 =  this.IList[index4].ISFType;
              if (Strings.Len(this.game.Data.SFTypeObj[isfType].BattleWAV) > 0 &&  this.game.Data.SFTypeObj[isfType].PowerPts >  VBMath.Rnd() *  num1)
              {
                index1 = isfType;
                break;
              }
            }
          }
          num6 = 0;
          if (num5 > 999)
            index1 = -2;
        }
        if (this.CombatType == 13)
          index1 = -1;
      }
      if (index1 <= -1 || !(Strings.Len(this.game.Data.SFTypeObj[index1].BattleWAV) > 0 & !this.dontUseSfx) || !(!this.game.AIRunning & !this.game.Data.RegimeObj[this.game.Data.Turn].AI))
        return;
      SoundMod.PlayAWave(this.game.AppPath + "sound/" + this.game.Data.SFTypeObj[index1].BattleWAV, ref this.game.EditObj);
    }

    pub fn CombatReconEndOfRound()
    {
      if ( this.game.Data.RuleVar[431] < 1.0)
        return;
      let mut num1: i32 =  0;
      if ((this.CombatType == 1 | this.CombatType == 10 | this.CombatType == 11 | this.CombatType == 13 | this.CombatType == 5) & this.CombatRound > 0)
      {
        if (this.AttackerRegime == this.game.Data.Turn && this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].MaxRecon > this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].get_ReconPts(this.AttackerRegime))
          this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].set_ReconPts(this.AttackerRegime, this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].MaxRecon);
        HexClass[,] hexObj = this.game.Data.MapObj[0].HexObj;
        HexClass[,] hexClassArray = hexObj;
        let mut targetX: i32 =  this.TargetX;
        let mut index1: i32 =  targetX;
        let mut targetY: i32 =  this.TargetY;
        let mut index2: i32 =  targetY;
        HexClass hexClass = hexClassArray[index1, index2];
        let mut attackerRegime: i32 =  this.AttackerRegime;
        let mut Index: i32 =  attackerRegime;
        let mut num2: i32 =  hexObj[targetX, targetY].get_ReconPts(attackerRegime) + this.initialRecon;
        hexClass.set_ReconPts(Index, num2);
        if (1 > this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].get_ReconPts(this.AttackerRegime))
          this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].set_ReconPts(this.AttackerRegime, 1);
        if (this.AttackerRegime == this.game.Data.Turn && this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].get_ReconPts(this.AttackerRegime) > this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].MaxRecon)
          this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].MaxRecon = this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].get_ReconPts(this.AttackerRegime);
      }
      let mut icounter: i32 =  this.ICounter;
      for (let mut inr: i32 =  0; inr <= icounter; inr += 1)
      {
        if (this.IList[inr].IAttacker == 0)
          num1 = num1;
        if ( this.GetEffectiveReconOnHexOfTargettedIndividual(inr) >=  this.IList[inr].IcoverPoints)
        {
          if (this.IList[inr].IUnr == 202)
            inr = inr;
          if (this.IList[inr].IvisibleFromRound > this.CombatRound)
          {
            this.IList[inr].IvisibleFromRound = this.CombatRound;
            if (!this.previewMode)
              this.UList[this.IList[inr].IUlistNr].USetToSpottedAtEnd = true;
          }
          else
            this.UList[this.IList[inr].IUlistNr].USetToSpottedAtEnd = true;
        }
      }
    }

    pub fn FinishStats()
    {
      this.AddDetail("");
      this.AddDetail("DEFENDER INDIVIDUALS:");
      let mut icounter1: i32 =  this.ICounter;
      for (let mut Number: i32 =  0; Number <= icounter1; Number += 1)
      {
        if (this.IList[Number].IAttacker == 0 && this.IList[Number].IRetreated == 0 & this.IList[Number].IKilled == 0)
          this.AddDetail(Conversion.Str( Number) + ") " + this.game.Data.SFTypeObj[this.IList[Number].ISFType].Name + " Breakthrough = " + Conversion.Str( this.IList[Number].IBreakTrough) + ", AttackCount = " + Conversion.Str( this.IList[Number].AttackCount) + ", Attacked = " + Conversion.Str( this.IList[Number].AttackedCount) + ", LastHit = " + Conversion.Str( this.IList[Number].ILastHit) + ", LastTargeted = " + Conversion.Str( this.IList[Number].ILastTargeted));
      }
      this.AddDetail("");
      this.AddDetail("ATTACKER INDIVIDUALS:");
      let mut icounter2: i32 =  this.ICounter;
      for (let mut Number: i32 =  0; Number <= icounter2; Number += 1)
      {
        if (this.IList[Number].IAttacker == 1 && this.IList[Number].IRetreated == 0 & this.IList[Number].IKilled == 0)
          this.AddDetail(Conversion.Str( Number) + ") " + this.game.Data.SFTypeObj[this.IList[Number].ISFType].Name + " Breakthrough = " + Conversion.Str( this.IList[Number].IBreakTrough) + ", AttackCount = " + Conversion.Str( this.IList[Number].AttackCount) + ", Attacked = " + Conversion.Str( this.IList[Number].AttackedCount) + ", LastHit = " + Conversion.Str( this.IList[Number].ILastHit) + ", LastTargeted = " + Conversion.Str( this.IList[Number].ILastTargeted));
      }
      this.AddDetail("");
    }

    pub fn DoBattle()
    {
      while (this.BattleEnded == 0)
        this.DoRound();
    }

    pub fn CheckAutoDestroy()
    {
      let mut icounter: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter; index += 1)
      {
        if (this.IList[index].AttackCount > 0 | this.game.Data.SFTypeObj[this.IList[index].ISFType].AntiStrucPts > 0 && this.IList[index].IAttacker == 1 && this.game.Data.SFTypeObj[this.IList[index].ISFType].AutoDestroy && this.CombatRound >= 1)
          this.IList[index].IKilled = this.CombatRound;
        if (this.IList[index].AttackCount > 0 && this.IList[index].IAttacker == 0 && this.game.Data.SFTypeObj[this.IList[index].ISFType].AutoDestroy2 && this.CombatRound >= 1)
          this.IList[index].IKilled = this.CombatRound;
      }
    }

    pub GetPercentChange: String(float newval, float oldval)
    {
      if ( newval >  oldval)
        return "+" + Strings.Trim(Conversion.Str( (int) Math.Round( newval * 100.0 /  oldval - 100.0))) + "%";
      return  newval ==  oldval ? "0%" : "-" + Strings.Trim(Conversion.Str( (int) Math.Round(100.0 -  newval * 100.0 /  oldval))) + "%";
    }

    pub fn GetEffectiveReconOnHexOfTargettedIndividual(inr: i32) -> i32
    {
      let mut x: i32 =  this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].X;
      let mut y: i32 =  this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].Y;
      Index: i32;
      if (this.IList[inr].IAttacker == 1)
      {
        Index = this.DefenderRegime;
        if (this.CombatType == 1 | this.CombatType == 11 || this.CombatType == 2 || this.IList[inr].IleftOutOfPartialAttack & this.IList[inr].ILastAttackDone > 0)
          return 9999;
      }
      else
        Index = this.AttackerRegime;
      if (Index == -1)
        return 0;
      let mut rawRecon: i32 =  this.game.Data.MapObj[0].HexObj[x, y].get_ReconPts(Index);
      if ( this.game.Data.RuleVar[419] > 0.0)
        rawRecon = this.game.HandyFunctionsObj.GetEffectiveRecon(rawRecon);
      return rawRecon;
    }

    pub fn GetUnmodifiedReconOnHexOfTargettedIndividual(inr: i32) -> i32
    {
      let mut x: i32 =  this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].X;
      let mut y: i32 =  this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].Y;
      let mut Index: i32 =  this.IList[inr].IAttacker != 1 ? this.AttackerRegime : this.DefenderRegime;
      return Index == -1 ? 0 : this.game.Data.MapObj[0].HexObj[x, y].get_ReconPts(Index);
    }

    pub fn GetIndividualHide(inr: i32) -> i32
    {
      let mut individualHide: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].X, this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].Y].LandscapeType].HidePts + this.game.Data.SFTypeObj[this.IList[inr].ISFType].HidePts;
      if ( this.game.Data.RuleVar[419] > 0.0)
      {
        let mut num1: i32 =  individualHide;
        let mut unr: i32 =  this.UList[this.IList[inr].IUlistNr].UNr;
        let mut num2: i32 =  num1;
        let mut averageXp: i32 =  this.game.HandyFunctionsObj.GetAverageXp(unr);
        let mut num3: i32 =   averageXp <=  this.game.Data.RuleVar[423] ? (int) Math.Round(Math.Ceiling(0.6 *  num2)) + (int) Math.Round(Math.Floor(0.4 *  num2 *  averageXp /  this.game.Data.RuleVar[423])) : num2 + (int) Math.Round(Math.Floor(0.4 *  num2 *  Math.Min(1f,  (( averageXp -  this.game.Data.RuleVar[423]) / (80.0 -  this.game.Data.RuleVar[423])))));
        float num4 =  this.game.HandyFunctionsObj.GetHexStackPts(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0) / this.game.Data.RuleVar[30];
        float num5 = 1f;
        if ( num4 < 0.333)
          num5 =  (1.39999997615814 - 0.4 * ( num4 / 0.333));
        else if ( num4 > 1.0)
        {
          num5 =  (1.0 - 0.5 * ( num4 - 1.0));
          if ( num5 < 0.0)
            num5 = 0.0f;
        }
        individualHide = (int) Math.Round( ( (int) Math.Round( ( num3 * num5)) + this.game.Data.RuleVar[9]));
        if (this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].Spotted)
          individualHide = (int) Math.Round( individualHide / 2.0);
        if (this.game.Data.UnitObj[this.UList[this.IList[inr].IUlistNr].UNr].Identified)
          individualHide = (int) Math.Round( individualHide / 2.0);
      }
      return individualHide;
    }

    pub fn GetMaxAttacked(defnr: i32, attnr: i32) -> i32
    {
      if (this.game.Data.Product != 6)
        return this.game.Data.SFTypeObj[this.IList[defnr].ISFType].MaxAttacked;
      let mut maxAttacked: i32 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].MaxAttacked;
      if ( this.ConcentricBonus > 1.0 & this.IList[defnr].IAttacker == 0)
        maxAttacked = (int) Math.Round( ( maxAttacked * this.ConcentricBonus));
      if (this.IList[defnr].IRdn < 100)
        maxAttacked = (int) Math.Round( maxAttacked * Math.Min(3.0, 100.0 /  Math.Max(1, this.IList[defnr].IRdn)));
      if (this.IList[defnr].IRetreat > 0)
        maxAttacked *= 2;
      if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0 | this.game.Data.SFTypeObj[this.IList[attnr].ISFType].directRange > 0)
        maxAttacked *= 3;
      return maxAttacked;
    }

    pub fn DoActualAttack(attnr: i32, defnr: i32, bool counterattack = false)
    {
      string[,] matrx = new string[61, 6];
      bool flag1 = !(!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime, this.game.Data.Turn));
      bool flag2 = !(!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime, this.game.Data.Turn));
      if (this.game.Data.FOWOn && this.game.Data.Product >= 6 &  this.game.Data.RuleVar[431] > 0.0)
      {
        if ( this.IList[defnr].IcoverPoints <=  this.GetEffectiveReconOnHexOfTargettedIndividual(defnr))
          flag1 = false;
        if ( this.IList[attnr].IcoverPoints <=  this.GetEffectiveReconOnHexOfTargettedIndividual(attnr))
          flag2 = false;
      }
      bool flag3 = flag1;
      bool flag4 = flag2;
      if (this.previewMode)
      {
        flag1 = false;
        flag2 = false;
        if (this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime == this.game.Data.Turn)
          flag4 = true;
        else
          flag3 = true;
      }
      str1: String;
      if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AutoDestroy & this.IList[attnr].IAttacker == 1 && this.CombatRound >= 2 & this.IList[attnr].IKilled == 0)
      {
        this.IList[attnr].IKilled = this.CombatRound;
        str1 += "Attacker auto-destructs.";
      }
      else if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AutoDestroy2 & this.IList[attnr].IAttacker == 0 && this.CombatRound >= 2 & this.IList[attnr].IKilled == 0)
      {
        this.IList[attnr].IKilled = this.CombatRound;
        str1 += "Attacker auto-destructs.";
      }
      else
      {
        let mut unitGroup1: i32 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].UnitGroup;
        this.AddDetail((this.IList[attnr].IAttacker != 1 ? "DEFENDER: " : "ATTACKER: ") + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " attacks " + this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name);
        float num1;
        if (this.IList[attnr].IAttacker == 1)
        {
          if (this.CombatType == 3 | this.CombatType == 4)
          {
            if ( this.game.Data.RuleVar[419] > 0.0)
            {
              num1 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPower[unitGroup1];
              if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].directRange > 0)
              {
                if (this.IList[attnr].IAttacker == 1 && this.IList[attnr].IdirectFire)
                  num1 =  Math.Max(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPower[unitGroup1], this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPowerDef[unitGroup1]);
                if (this.IList[attnr].IAttacker == 0 && !Information.IsNothing( this.IList[attnr].IdirectFireDef) && this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
                  num1 =  Math.Max(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPower[unitGroup1], this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPowerDef[unitGroup1]);
              }
            }
            else
              num1 = Conversions.ToSingle(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackArt[unitGroup1]);
          }
          else
            num1 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPower[unitGroup1];
        }
        else
          num1 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPowerDef[unitGroup1];
        float Number1 = this.IList[defnr].IAttacker != 1 ?  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].HitPointsDef[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup] : (this.game.Data.Product < 6 ?  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].HitPoints[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup] : (counterattack ?  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].HitPointsDef[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup] :  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].HitPoints[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup]));
        let mut landscapeType1: i32 =  this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].LandscapeType;
        s1: String = "BEFORE MOD: Attval=" + Conversion.Str( num1) + ", DefVal= " + Conversion.Str( Number1);
        str2: String = "attack=" + Strings.Trim(Conversion.Str( num1)) + " VS defense = " + Strings.Trim(Conversion.Str( Number1));
        num2: i32;
        let mut index1: i32 =  num2 + 1;
        matrx[index1, 0] = "Start Att Score";
        matrx[index1, 1] = "";
        matrx[index1, 2] = Strings.Trim(Conversion.Str( num1));
        num3: i32;
        let mut index2: i32 =  num3 + 1;
        matrx[index2, 3] = "Start Def Score";
        matrx[index2, 4] = "";
        matrx[index2, 5] = Strings.Trim(Conversion.Str( Number1));
        this.AddDetail(s1);
        float powerPts1 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].PowerPts;
        float powerPts2 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].PowerPts;
        index3: i32;
        if ( this.game.Data.RuleVar[407] < 1.0 &  this.game.Data.RuleVar[434] < 1.0)
        {
          if ( this.game.Data.RuleVar[435] > 0.0)
          {
            if (((!this.game.EditObj.CombatSim ? 1 : 0) & 0) != 0)
            {
              index1 += 1;
              matrx[index1, 0] = "Fuel".to_owned();
              float oldval = num1;
              this.IList[attnr].IAttackMod = 1f;
              if (this.IList[attnr].AttackCount == 0 && this.IList[defnr].AttackedCount < this.GetMaxAttacked(defnr, attnr))
              {
                this.IList[attnr].IAttackMod = 1f;
                index3 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelForAttack;
                let mut fuelRegimeVar1: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelRegimeVar;
                num4: i32;
                if (index3 > 0)
                {
                  if (this.IList[attnr].IAttacker == 0)
                  {
                    if (!(this.CombatType == 3 | this.CombatType == 4) && !(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0 & (this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13)))
                    {
                      index3 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelForAttackDef;
                      num4 = this.DefenderRegime;
                      if (this.game.Data.UnitObj[this.IList[attnr].IUnr].Fuel >= index3)
                      {
                        UnitClass[] unitObj1 = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray1 = unitObj1;
                        let mut iunr1: i32 =  this.IList[attnr].IUnr;
                        let mut index4: i32 =  iunr1;
                        unitClassArray1[index4].Fuel = unitObj1[iunr1].Fuel - index3;
                        UnitClass[] unitObj2 = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray2 = unitObj2;
                        let mut iunr2: i32 =  this.IList[attnr].IUnr;
                        let mut index5: i32 =  iunr2;
                        unitClassArray2[index5].FuelUsedDef = unitObj2[iunr2].FuelUsedDef + index3;
                        this.AddDetail("Defender: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " consumes " + Conversion.Str( index3) + " " + this.game.Data.RegimeSlotName[fuelRegimeVar1]);
                      }
                      else
                      {
                        this.AddDetail("Defender: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " out of fuel!!!");
                        this.IList[attnr].IAttackMod = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].OutOfFuelDefense;
                      }
                    }
                  }
                  else
                  {
                    index3 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelForAttack;
                    let mut fuelRegimeVar2: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelRegimeVar;
                    num4 = this.AttackerRegime;
                    if (this.game.Data.UnitObj[this.IList[attnr].IUnr].Fuel >= index3)
                    {
                      UnitClass[] unitObj3 = this.game.Data.UnitObj;
                      UnitClass[] unitClassArray3 = unitObj3;
                      let mut iunr3: i32 =  this.IList[attnr].IUnr;
                      let mut index6: i32 =  iunr3;
                      unitClassArray3[index6].Fuel = unitObj3[iunr3].Fuel - index3;
                      UnitClass[] unitObj4 = this.game.Data.UnitObj;
                      UnitClass[] unitClassArray4 = unitObj4;
                      let mut iunr4: i32 =  this.IList[attnr].IUnr;
                      let mut index7: i32 =  iunr4;
                      unitClassArray4[index7].FuelUsedAtt = unitObj4[iunr4].FuelUsedAtt + index3;
                      this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " consumes " + Conversion.Str( index3) + " " + this.game.Data.RegimeSlotName[fuelRegimeVar2]);
                    }
                    else
                    {
                      this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " out of fuel!!!");
                      this.IList[attnr].IAttackMod = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].OutOfFuelAttack;
                    }
                  }
                }
              }
              num1 *= this.IList[attnr].IAttackMod;
              this.AddDetail("Attval after fuel modifier: " + Conversion.Str( num1));
              matrx[index1, 1] = this.GetPercentChange(num1, oldval);
              matrx[index1, 2] = Conversions.ToString(num1);
              if (flag1)
              {
                matrx[index1, 1] = "?";
                matrx[index1, 2] = "?";
              }
            }
          }
          else if (!this.game.EditObj.CombatSim)
          {
            index1 += 1;
            matrx[index1, 0] = "Fuel".to_owned();
            float oldval = num1;
            this.IList[attnr].IAttackMod = 1f;
            if (this.IList[attnr].AttackCount == 0 && this.IList[defnr].AttackedCount < this.GetMaxAttacked(defnr, attnr))
            {
              this.IList[attnr].IAttackMod = 1f;
              index3 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelForAttack;
              let mut currentSlot1: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelRegimeVar;
              if ( this.game.Data.RuleVar[949] > 0.0)
                currentSlot1 = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot1, this.game.Data.UnitObj[this.IList[attnr].IUnr].RealX(ref this.game), this.game.Data.UnitObj[this.IList[attnr].IUnr].RealY(ref this.game));
              let mut index8: i32 =  currentSlot1;
              if (index8 > -1 & index3 > 0)
              {
                if (this.IList[attnr].IAttacker == 0)
                {
                  if (!(this.CombatType == 3 | this.CombatType == 4) && !(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0 & (this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13)))
                  {
                    index3 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelForAttackDef;
                    let mut defenderRegime: i32 =  this.DefenderRegime;
                    if (this.game.Data.RegimeObj[defenderRegime].RegimeSlot[index8] >= index3)
                    {
                      int[] regimeSlot = this.game.Data.RegimeObj[defenderRegime].RegimeSlot;
                      int[] numArray = regimeSlot;
                      let mut index9: i32 =  index8;
                      let mut index10: i32 =  index9;
                      let mut num5: i32 =  regimeSlot[index9] - index3;
                      numArray[index10] = num5;
                      UnitClass[] unitObj = this.game.Data.UnitObj;
                      UnitClass[] unitClassArray = unitObj;
                      let mut iunr: i32 =  this.IList[attnr].IUnr;
                      let mut index11: i32 =  iunr;
                      unitClassArray[index11].FuelUsedDef = unitObj[iunr].FuelUsedDef + index3;
                      this.AddDetail("Defender: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " consumes " + Conversion.Str( index3) + " " + this.game.Data.RegimeSlotName[index8]);
                    }
                    else
                    {
                      this.AddDetail("Defender: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " out of fuel!!!");
                      this.IList[attnr].IAttackMod = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].OutOfFuelDefense;
                    }
                  }
                }
                else
                {
                  index3 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelForAttack;
                  let mut currentSlot2: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FuelRegimeVar;
                  if ( this.game.Data.RuleVar[949] > 0.0)
                    currentSlot2 = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot2, this.game.Data.UnitObj[this.IList[attnr].IUnr].RealX(ref this.game), this.game.Data.UnitObj[this.IList[attnr].IUnr].RealY(ref this.game));
                  let mut index12: i32 =  currentSlot2;
                  let mut attackerRegime: i32 =  this.AttackerRegime;
                  if (this.game.Data.RegimeObj[attackerRegime].RegimeSlot[index12] >= index3)
                  {
                    int[] regimeSlot = this.game.Data.RegimeObj[attackerRegime].RegimeSlot;
                    int[] numArray = regimeSlot;
                    let mut index13: i32 =  index12;
                    let mut index14: i32 =  index13;
                    let mut num6: i32 =  regimeSlot[index13] - index3;
                    numArray[index14] = num6;
                    UnitClass[] unitObj = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray = unitObj;
                    let mut iunr: i32 =  this.IList[attnr].IUnr;
                    let mut index15: i32 =  iunr;
                    unitClassArray[index15].FuelUsedAtt = unitObj[iunr].FuelUsedAtt + index3;
                    this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " consumes " + Conversion.Str( index3) + " " + this.game.Data.RegimeSlotName[index12]);
                  }
                  else
                  {
                    this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + " out of fuel!!!");
                    this.IList[attnr].IAttackMod = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].OutOfFuelAttack;
                  }
                }
              }
            }
            num1 *= this.IList[attnr].IAttackMod;
            this.AddDetail("Attval after fuel modifier: " + Conversion.Str( num1));
            matrx[index1, 1] = this.GetPercentChange(num1, oldval);
            matrx[index1, 2] = Conversions.ToString(num1);
            if (flag1)
            {
              matrx[index1, 1] = "?";
              matrx[index1, 2] = "?";
            }
          }
        }
        if (this.IList[attnr].AttackCount + this.IList[attnr].ICounterAttacks == 0)
        {
          index3 = this.IList[attnr].ISFType;
          if (this.game.Data.SFTypeObj[index3].StockpileUsedPerRound > 0)
          {
            if (this.game.Data.UnitObj[this.IList[attnr].IUnr].StockpileCurrent >= this.game.Data.SFTypeObj[index3].StockpileUsedPerRound)
            {
              this.IList[attnr].Istockpile = true;
              UnitClass[] unitObj = this.game.Data.UnitObj;
              UnitClass[] unitClassArray = unitObj;
              let mut iunr: i32 =  this.IList[attnr].IUnr;
              let mut index16: i32 =  iunr;
              unitClassArray[index16].StockpileCurrent = unitObj[iunr].StockpileCurrent - this.game.Data.SFTypeObj[index3].StockpileUsedPerRound;
            }
            else
              this.IList[attnr].Istockpile = false;
          }
          else
            this.IList[attnr].Istockpile = true;
        }
        if (this.game.Data.SFTypeObj[index3].StockpileUsedPerRound > 0 && !this.IList[attnr].Istockpile)
        {
          index1 += 1;
          matrx[index1, 0] = "Stockpile depl.";
          float oldval = num1;
          num1 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].StockpileDepletedMod;
          this.AddDetail("Attval after stockpile depleted modifier: " + Conversion.Str( num1));
          matrx[index1, 1] = this.GetPercentChange(num1, oldval);
          matrx[index1, 2] = Conversions.ToString(num1);
          if (flag1)
          {
            matrx[index1, 1] = "?";
            matrx[index1, 2] = "?";
          }
        }
        if (this.game.Data.UncertaintyOn & !this.previewMode)
        {
          index1 += 1;
          matrx[index1, 0] = "Uncertainty effect";
          float oldval = num1;
          let mut udice1: i32 =  this.UList[this.IList[attnr].IUlistNr].UDice;
          let mut udice2: i32 =  this.UList[this.IList[defnr].IUlistNr].UDice;
          num7: i32;
          num8: i32;
          if (this.IList[attnr].IAttacker == 1)
          {
            num7 = udice1 + this.AttackerDice;
            num8 = udice2 + this.DefenderDice;
          }
          else
          {
            num7 = udice1 + this.DefenderDice;
            num8 = udice2 + this.AttackerDice;
          }
          let mut num9: i32 =  num7 - num8;
          if (num9 > 0)
            num1 *=  (1.0 +  num9 / 10.0);
          this.AddDetail("Attval after uncertainty effect: " + Conversion.Str( num1));
          matrx[index1, 1] = this.GetPercentChange(num1, oldval);
          matrx[index1, 2] = Conversions.ToString(num1);
          if (flag1)
          {
            matrx[index1, 1] = "?";
            matrx[index1, 2] = "?";
          }
        }
        if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[470] > 0.0)
        {
          bool flag5 = false;
          if (this.CombatType == 3 | this.CombatType == 4)
            flag5 = true;
          if (this.InterceptFire)
            flag5 = true;
          if (this.UList[this.IList[attnr].IUlistNr].USupportInterceptFire)
            flag5 = true;
          index1 += 1;
          matrx[index1, 0] = "Night penalty";
          float oldval = num1;
          if (!flag5 &&  this.game.Data.RuleVar[470] > 0.0)
          {
            float num10 = 0.2f +  this.IList[attnr].IXp / 150f;
            if ( num10 < 0.25)
              num10 = 0.25f;
            if ( num10 > 0.66)
              num10 = 0.66f;
            num1 *= num10;
          }
          this.AddDetail("Attval after Night penalty: " + Conversion.Str( num1));
          matrx[index1, 1] = this.GetPercentChange(num1, oldval);
          matrx[index1, 2] = Conversions.ToString(num1);
          if (flag1)
          {
            matrx[index1, 1] = "?";
            matrx[index1, 2] = "?";
          }
        }
        if ( this.game.Data.RuleVar[419] > 0.0)
        {
          if (this.IList[attnr].IAttacker == 1)
          {
            index1 += 1;
            matrx[index1, 0] = "Direct Fire Mod";
            float oldval = num1;
            if (this.IList[attnr].IdirectFire)
              num1 =  ( num1 *  this.IList[attnr].IdirectMod / 100.0);
            this.AddDetail("Attval after Direct Fire Mod: " + Conversion.Str( num1));
            matrx[index1, 1] = this.GetPercentChange(num1, oldval);
            matrx[index1, 2] = Conversions.ToString(num1);
            if (flag1)
            {
              matrx[index1, 1] = "?";
              matrx[index1, 2] = "?";
            }
          }
          if (this.IList[attnr].IAttacker == 0 & (!this.InterceptFire & this.CombatType == 3 | this.UList[this.IList[attnr].IUlistNr].USupportInterceptFire))
          {
            index1 += 1;
            matrx[index1, 0] = "Direct Fire Mod";
            float oldval = num1;
            if (!Information.IsNothing( this.IList[attnr].IdirectFireDef) && this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
              num1 =  ( num1 *  this.IList[attnr].IdirectModDef[this.IList[defnr].IUlistNr] / 100.0);
            this.AddDetail("Attval after Direct Fire Mod: " + Conversion.Str( num1));
            matrx[index1, 1] = this.GetPercentChange(num1, oldval);
            matrx[index1, 2] = Conversions.ToString(num1);
            if (flag1)
            {
              matrx[index1, 1] = "?";
              matrx[index1, 2] = "?";
            }
          }
          if ( this.game.Data.RuleVar[430] > 0.0)
          {
            if (this.CombatType == 3)
            {
              index1 += 1;
              matrx[index1, 0] = "Indirect Fire LOS Bonus";
              float oldval = num1;
              if (!(this.IList[attnr].IAttacker == 0 & this.InterceptFire) & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0)
              {
                if (!Information.IsNothing( this.UList[this.IList[attnr].IUlistNr].ULos))
                {
                  num1 +=  ( this.game.Data.RuleVar[430] *  num1 *  this.UList[this.IList[attnr].IUlistNr].ULos[this.IList[defnr].IUlistNr] / 100.0);
                  let mut num11: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y].HeightLevel - this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.IList[attnr].IUnr].X, this.game.Data.UnitObj[this.IList[attnr].IUnr].Y].HeightLevel;
                  if (num11 < 0)
                    num1 +=  ( this.game.Data.RuleVar[430] *  num1 *  this.UList[this.IList[attnr].IUlistNr].ULos[this.IList[defnr].IUlistNr] / 100.0 * ( Math.Abs(num11) *  this.game.Data.RuleVar[425] / 100.0));
                }
                this.AddDetail("Attval after Indirect Fire LOS Bonus: " + Conversion.Str( num1));
                matrx[index1, 1] = this.GetPercentChange(num1, oldval);
                matrx[index1, 2] = Conversions.ToString(num1);
                if (flag1)
                {
                  matrx[index1, 1] = "?";
                  matrx[index1, 2] = "?";
                }
              }
            }
            if (this.CombatType == 1)
            {
              index1 += 1;
              matrx[index1, 0] = "Indirect Fire LOS Bonus";
              float oldval = num1;
              if (this.UList[this.IList[attnr].IUlistNr].USupportInterceptFire & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0)
              {
                num1 +=  ( this.game.Data.RuleVar[430] *  num1 *  this.UList[this.IList[attnr].IUlistNr].ULos[this.IList[defnr].IUlistNr] / 100.0);
                let mut num12: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y].HeightLevel - this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.IList[attnr].IUnr].X, this.game.Data.UnitObj[this.IList[attnr].IUnr].Y].HeightLevel;
                if (num12 < 0)
                  num1 +=  ( this.game.Data.RuleVar[430] *  num1 *  this.UList[this.IList[attnr].IUlistNr].ULos[this.IList[defnr].IUlistNr] / 100.0 * ( Math.Abs(num12) *  this.game.Data.RuleVar[425] / 100.0));
              }
              this.AddDetail("Attval after Indirect Fire LOS Bonus: " + Conversion.Str( num1));
              matrx[index1, 1] = this.GetPercentChange(num1, oldval);
              matrx[index1, 2] = Conversions.ToString(num1);
              if (flag1)
              {
                matrx[index1, 1] = "?";
                matrx[index1, 2] = "?";
              }
            }
          }
        }
        let mut index17: i32 =  index1 + 1;
        matrx[index17, 0] = "Weather".to_owned();
        float oldval1 = num1;
        float num13 = num1 * ( this.game.Data.UnitTypePenalty[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup] / 100f);
        this.AddDetail("Attval after global combat modifier (wheater): " + Conversion.Str( num13));
        matrx[index17, 1] = this.GetPercentChange(num13, oldval1);
        matrx[index17, 2] = Conversions.ToString(num13);
        if (flag1)
        {
          matrx[index17, 1] = "?";
          matrx[index17, 2] = "?";
        }
        combatClass: CombatClass;
        if (!this.previewMode)
        {
          index17 += 1;
          matrx[index17, 0] = "Attack startup";
          float oldval2 = num13;
          let mut num14: i32 =  100;
          if (this.customCombatObj.HasCustumCalls())
          {
            str3: String = "";
            CustomCombatCalls customCombatObj = this.customCombatObj;
            combatClass = this;
            ref local1: CombatClass = ref combatClass;
            let mut game: GameClass = this.game;
            let mut attnr1: i32 =  attnr;
            let mut defnr1: i32 =  defnr;
            let mut num15: i32 =  counterattack ? 1 : 0;
            ref local2: String = ref str3;
            num14 = customCombatObj.IndividualCombatCall_HasNoEarlyCombatRoundPenalties(ref local1, game, attnr1, defnr1, num15 != 0, ref local2);
            if (str3.Length > 0)
              str2 = str2 + "\r\n" + str3;
          }
          if (this.game.Data.Product >= 6 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].EndCombatRound > 0)
            num14 = 0;
          if (num14 <= 100 & !this.previewMode && this.CombatType == 1 | this.CombatType == 10 | this.CombatType == 12 && this.IList[attnr].IAttacker == 1 & !counterattack | this.IList[attnr].IAttacker == 0 & counterattack && Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectLessEqual(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackArt[unitGroup1],  0, false),  !this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench)))
          {
            float num16 =  ( this.game.Data.SFTypeObj[this.IList[attnr].ISFType].FirstRoundPenaltyMod *  num14 / 100.0);
            if (this.IList[attnr].IAttacker == 0 & counterattack)
              num16 =  ( this.game.Data.SFTypeObj[this.IList[defnr].ISFType].FirstRoundPenaltyMod *  num14 / 100.0);
            if (this.CombatRound == 1)
              num13 =  ( num16 *  num13 *  this.game.Data.RuleVar[316] + (1.0 -  num16) *  num13);
            if (this.CombatRound == 2)
              num13 =  ( num16 *  num13 *  this.game.Data.RuleVar[317] + (1.0 -  num16) *  num13);
            this.AddDetail("Attval after 1st+2nd round attacker penalty: " + Conversion.Str( num13));
          }
          matrx[index17, 1] = this.GetPercentChange(num13, oldval2);
          matrx[index17, 2] = Conversions.ToString(num13);
          if (flag1)
          {
            matrx[index17, 1] = "?";
            matrx[index17, 2] = "?";
          }
        }
        if (counterattack)
        {
          index17 += 1;
          matrx[index17, 0] = "Counterattack".to_owned();
          float oldval3 = num13;
          if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0)
          {
            if (this.IList[attnr].IAttacker == 1)
              num13 *= this.game.Data.RuleVar[101];
            else
              num13 *= this.game.Data.RuleVar[102];
          }
          else if (this.IList[attnr].IAttacker == 1)
            num13 *= this.game.Data.RuleVar[103];
          else
            num13 *= this.game.Data.RuleVar[104];
          if (this.game.Data.Product >= 6 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Attacks * 2 < this.IList[attnr].ICounterAttacks)
          {
            let mut num17: i32 =  (int) Math.Round( (this.IList[attnr].ICounterAttacks - this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Attacks * 2));
            for (let mut index18: i32 =  1; index18 <= num17; index18 += 1)
              num13 *= 0.66f;
          }
          matrx[index17, 1] = this.GetPercentChange(num13, oldval3);
          matrx[index17, 2] = Conversions.ToString(num13);
          if (flag1)
          {
            matrx[index17, 1] = "?";
            matrx[index17, 2] = "?";
          }
          this.AddDetail("Attval after Counter Attack penalty: " + Conversion.Str( num13));
        }
        if (this.IList[attnr].IAA == 1)
        {
          index17 += 1;
          matrx[index17, 0] = "AA distance";
          float oldval4 = num13;
          num13 *= this.game.Data.RuleVar[105];
          this.AddDetail("Supporting AA fire gets penalty because outside own hex battle. mod the attval to: " + Conversion.Str( num13));
          matrx[index17, 1] = this.GetPercentChange(num13, oldval4);
          matrx[index17, 2] = Conversions.ToString(num13);
          if (flag1)
          {
            matrx[index17, 1] = "?";
            matrx[index17, 2] = "?";
          }
        }
        let mut index19: i32 =  index17 + 1;
        matrx[index19, 0] = "Max attacked";
        float oldval5 = num13;
        if (this.game.Data.Product == 7 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].MaxAttacked == 0)
          this.game.Data.SFTypeObj[this.IList[defnr].ISFType].MaxAttacked = 3;
        if (this.IList[defnr].AttackedCount > this.GetMaxAttacked(defnr, attnr))
        {
          bool flag6 = false;
          if (this.game.Data.Product >= 6)
          {
            if (this.IList[attnr].IAttacker == 1)
            {
              if (this.IList[attnr].IdirectFire)
                flag6 = true;
            }
            else if (!Information.IsNothing( this.IList[attnr].IdirectFireDef) && this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
              flag6 = true;
          }
          if (!(this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater < 2 & (this.CombatType == 5 | this.CombatType == 3 | this.CombatType == 4)) | flag6 && flag6 | !(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater < 2 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2 & (this.CombatType == 14 | this.CombatType == 15 | this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13)))
          {
            float num18 =  this.IList[defnr].AttackedCount /  this.GetMaxAttacked(defnr, attnr);
            if ( num18 >  this.game.Data.RuleVar[300])
              num18 = this.game.Data.RuleVar[300];
            if ( num18 <= 0.0)
              num18 = 1f;
            num13 /= num18;
          }
        }
        matrx[index19, 1] = this.GetPercentChange(num13, oldval5);
        matrx[index19, 2] = Conversions.ToString(num13);
        if (flag1)
        {
          matrx[index19, 1] = "?";
          matrx[index19, 2] = "?";
        }
        this.AddDetail("Target has been attacked more than its maxattacked.. mod the attval to: " + Conversion.Str( num13));
        if (this.customCombatObj.HasCustumCalls())
        {
          if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater != 2)
          {
            index19 += 1;
            matrx[index19, 0] = "Landscape".to_owned();
            float oldval6 = num13;
            str4: String = "";
            if (this.IList[attnr].IAttacker == 0)
            {
              let mut num19: i32 =  (int) Math.Round( (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModDef[landscapeType1] * 100f));
              CustomCombatCalls customCombatObj = this.customCombatObj;
              combatClass = this;
              ref local3: CombatClass = ref combatClass;
              let mut game: GameClass = this.game;
              let mut attnr2: i32 =  attnr;
              let mut defnr2: i32 =  defnr;
              let mut num20: i32 =  counterattack ? 1 : 0;
              ref local4: String = ref str4;
              let mut landscapeDefMod: i32 =  num19;
              let mut num21: i32 =  customCombatObj.IndividualCombatCall_RiverTypeAndLandscapeTypeModifier(ref local3, game, attnr2, defnr2, num20 != 0, ref local4, 0, 0, landscapeDefMod, -1);
              num13 =  ( num13 *  num21 / 100.0);
            }
            else if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType > -1 & (this.CombatType == 3 | this.CombatType == 4))
            {
              let mut num22: i32 =  (int) Math.Round( (this.game.Data.SFTypeObj[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType].CombatModAtt[landscapeType1] * 100f));
              CustomCombatCalls customCombatObj = this.customCombatObj;
              combatClass = this;
              ref local5: CombatClass = ref combatClass;
              let mut game: GameClass = this.game;
              let mut attnr3: i32 =  attnr;
              let mut defnr3: i32 =  defnr;
              let mut num23: i32 =  counterattack ? 1 : 0;
              ref local6: String = ref str4;
              let mut landscapeAttMod: i32 =  num22;
              let mut num24: i32 =  customCombatObj.IndividualCombatCall_RiverTypeAndLandscapeTypeModifier(ref local5, game, attnr3, defnr3, num23 != 0, ref local6, 0, landscapeAttMod, 0, -1);
              num13 =  ( num13 *  num24 / 100.0);
            }
            else
            {
              let mut num25: i32 =  (int) Math.Round( (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModAtt[landscapeType1] * 100f));
              CustomCombatCalls customCombatObj = this.customCombatObj;
              combatClass = this;
              ref local7: CombatClass = ref combatClass;
              let mut game: GameClass = this.game;
              let mut attnr4: i32 =  attnr;
              let mut defnr4: i32 =  defnr;
              let mut num26: i32 =  counterattack ? 1 : 0;
              ref local8: String = ref str4;
              let mut landscapeAttMod: i32 =  num25;
              let mut num27: i32 =  customCombatObj.IndividualCombatCall_RiverTypeAndLandscapeTypeModifier(ref local7, game, attnr4, defnr4, num26 != 0, ref local8, 0, landscapeAttMod, 0, -1);
              num13 =  ( num13 *  num27 / 100.0);
            }
            if (this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Type].PictureLT > -1)
            {
              let mut pictureLt: i32 =  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Type].PictureLT;
              if (this.IList[attnr].IAttacker == 0)
                num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModDef[pictureLt];
              else
                num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModAtt[pictureLt];
            }
            matrx[index19, 1] = this.GetPercentChange(num13, oldval6);
            if ( num13 <  oldval6 & this.IList[attnr].IAttacker == 1)
              num13 = num13;
            matrx[index19, 2] = Conversions.ToString(num13);
            if (flag1)
            {
              matrx[index19, 1] = "?";
              matrx[index19, 2] = "?";
            }
            if (str4.Length > 0)
              str2 = str2 + "\r\n" + str4;
          }
        }
        else if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater != 2)
        {
          index19 += 1;
          matrx[index19, 0] = "Landscape".to_owned();
          float oldval7 = num13;
          bool flag7 = false;
          if (this.game.Data.Product >= 6)
          {
            if (this.IList[attnr].IdirectFire)
              flag7 = true;
            if (this.IList[attnr].IAttacker == 0 && !Information.IsNothing( this.IList[attnr].IdirectFireDef) && this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
              flag7 = true;
            if (!(this.CombatType == 3 | this.CombatType == 4))
              flag7 = false;
          }
          if (flag7)
          {
            let mut landscapeType2: i32 =  this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.game.Data.UnitObj[this.UList[this.IList[defnr].IUlistNr].UNr].X, this.game.Data.UnitObj[this.UList[this.IList[defnr].IUlistNr].UNr].Y].LandscapeType;
            if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType > -1)
              num13 *= this.game.Data.SFTypeObj[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType].CombatModAtt[landscapeType2];
            else
              num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModAtt[landscapeType2];
          }
          else
          {
            if (this.IList[attnr].IAttacker == 0)
              num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModDef[landscapeType1];
            else if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType > -1 & (this.CombatType == 3 | this.CombatType == 4))
              num13 *= this.game.Data.SFTypeObj[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType].CombatModAtt[landscapeType1];
            else
              num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModAtt[landscapeType1];
            if (this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Type].PictureLT > -1)
            {
              let mut pictureLt: i32 =  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Type].PictureLT;
              if (this.IList[attnr].IAttacker == 0)
                num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModDef[pictureLt];
              else
                num13 *= this.game.Data.SFTypeObj[this.IList[attnr].ISFType].CombatModAtt[pictureLt];
            }
          }
          matrx[index19, 1] = this.GetPercentChange(num13, oldval7);
          matrx[index19, 2] = Conversions.ToString(num13);
          if (flag1)
          {
            matrx[index19, 1] = "?";
            matrx[index19, 2] = "?";
          }
        }
        this.AddDetail("Attval after CombatMod for Landscapetype: " + Conversion.Str( num13));
        if (this.IList[attnr].IAttacker == 0)
          index3 = index3;
        if (!this.previewMode | this.UList[this.IList[attnr].IUlistNr].previewInfoLevel >= 2)
        {
          index19 += 1;
          matrx[index19, 0] = "HQ".to_owned();
          float oldval8 = num13;
          if (!this.game.Data.UnitObj[this.IList[attnr].IUnr].IsHQ)
          {
            if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0)
            {
              num13 *= this.UList[this.IList[attnr].IUlistNr].UStaffMod;
              this.AddDetail("Attval after CombatMod for Staff (" + Conversion.Str( this.UList[this.IList[attnr].IUlistNr].UStaffMod) + "): " + Conversion.Str( num13));
            }
            else
              this.AddDetail("Attval NO staff modifier because not land theater.");
          }
          else
            this.AddDetail("Attval NO staff modifier because is HQ.");
          matrx[index19, 1] = this.GetPercentChange(num13, oldval8);
          matrx[index19, 2] = Conversions.ToString(num13);
          if (flag1)
          {
            matrx[index19, 1] = "?";
            matrx[index19, 2] = "?";
          }
        }
        let mut index20: i32 =  index19 + 1;
        matrx[index20, 0] = "Readiness".to_owned();
        float oldval9 = num13;
        let mut unitGroup2: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup;
        float num28 = this.IList[attnr].IAttacker != 1 ?  ( num13 * (1.0 -  this.game.Data.RuleVar[107]) +  num13 *  this.game.Data.RuleVar[107] * ( this.IList[attnr].IRdn / 100.0)) :  ( num13 * (1.0 -  this.game.Data.RuleVar[106]) +  num13 *  this.game.Data.RuleVar[106] * ( this.IList[attnr].IRdn / 100.0));
        this.AddDetail("Attval after Rdn mod: " + Conversion.Str( num28));
        matrx[index20, 1] = this.GetPercentChange(num28, oldval9);
        matrx[index20, 2] = Conversions.ToString(num28);
        if (flag1)
        {
          matrx[index20, 1] = "?";
          matrx[index20, 2] = "?";
        }
        if ( this.game.Data.RuleVar[482] > 0.0 & this.game.Data.Product >= 6 && this.UList[this.IList[attnr].IUlistNr].Uattacker == 1 & !(this.game.EditObj.attackTypeOption == 0 | this.game.EditObj.attackTypeOption == 3))
        {
          index20 += 1;
          matrx[index20, 0] = "Attack Mode";
          float oldval10 = num28;
          if (this.game.EditObj.attackTypeOption == 1)
            num28 *= 0.33f;
          else if (this.game.EditObj.attackTypeOption == 2)
            num28 *= 0.5f;
          else if (this.game.EditObj.attackTypeOption == 4)
            num28 *= 1.66f;
          this.AddDetail("Attval after Attack Mode mod: " + Conversion.Str( num28));
          matrx[index20, 1] = this.GetPercentChange(num28, oldval10);
          matrx[index20, 2] = Conversions.ToString(num28);
          if (flag1)
          {
            matrx[index20, 1] = "?";
            matrx[index20, 2] = "?";
          }
        }
        if ( this.game.Data.RuleVar[407] > 0.0 |  this.game.Data.RuleVar[434] > 0.0 && !this.previewMode | !flag3)
        {
          index21: i32;
          if ( this.IList[attnr].ILisAmmoMod < 1.0)
          {
            index21 = index20 + 1;
            matrx[index21, 0] = "Ammunition Mod";
            float oldval11 = num28;
            num28 *= this.IList[attnr].ILisAmmoMod;
            this.AddDetail("Attval after Ammunition mod: " + Conversion.Str( num28));
            matrx[index21, 1] = this.GetPercentChange(num28, oldval11);
            matrx[index21, 2] = Conversions.ToString(num28);
            if (flag1)
            {
              matrx[index21, 1] = "?";
              matrx[index21, 2] = "?";
            }
          }
          else
          {
            index21 = index20 + 1;
            matrx[index21, 0] = "Ammunition Mod";
            float oldval12 = num28;
            matrx[index21, 1] = this.GetPercentChange(num28, oldval12);
            matrx[index21, 2] = Conversions.ToString(num28);
            if (flag1)
            {
              matrx[index21, 1] = "?";
              matrx[index21, 2] = "?";
            }
          }
          if ( this.IList[attnr].ILisFuelMod < 1.0)
          {
            index20 = index21 + 1;
            matrx[index20, 0] = "Fuel Mod";
            float oldval13 = num28;
            num28 *= this.IList[attnr].ILisFuelMod;
            this.AddDetail("Attval after Fuel mod: " + Conversion.Str( num28));
            matrx[index20, 1] = this.GetPercentChange(num28, oldval13);
            matrx[index20, 2] = Conversions.ToString(num28);
            if (flag1)
            {
              matrx[index20, 1] = "?";
              matrx[index20, 2] = "?";
            }
          }
          else
          {
            index20 = index21 + 1;
            matrx[index20, 0] = "Fuel Mod";
            float oldval14 = num28;
            matrx[index20, 1] = this.GetPercentChange(num28, oldval14);
            matrx[index20, 2] = Conversions.ToString(num28);
            if (flag1)
            {
              matrx[index20, 1] = "?";
              matrx[index20, 2] = "?";
            }
          }
        }
        let mut index22: i32 =  index20 + 1;
        matrx[index22, 0] = "Airfield Stack";
        float oldval15 = num28;
        unitGroup2 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup;
        if ( this.game.HandyFunctionsObj.GetAirFieldStackModifier(this.game.Data.UnitObj[this.IList[attnr].IUnr].X, this.game.Data.UnitObj[this.IList[attnr].IUnr].Y) < 1.0)
          num28 =  (0.33 *  num28 + 0.66 *  num28 *  this.game.HandyFunctionsObj.GetAirFieldStackModifier(this.game.Data.UnitObj[this.IList[attnr].IUnr].X, this.game.Data.UnitObj[this.IList[attnr].IUnr].Y));
        this.AddDetail("Attval after airfield stack: " + Conversion.Str( num28));
        matrx[index22, 1] = this.GetPercentChange(num28, oldval15);
        matrx[index22, 2] = Conversions.ToString(num28);
        if (flag1)
        {
          matrx[index22, 1] = "?";
          matrx[index22, 2] = "?";
        }
        if (!this.previewMode | this.UList[this.IList[attnr].IUlistNr].previewInfoLevel >= 2)
        {
          index22 += 1;
          matrx[index22, 0] = "Special".to_owned();
          float oldval16 = num28;
          if (this.IList[attnr].IAttacker == 1)
          {
            index3 = this.game.Data.SFObj[this.IList[attnr].ISFNr].OffMod + 100;
            if (0 > index3)
              index3 = 0;
            if (index3 > 999)
              index3 = 999;
            num28 *=  index3 / 100f;
          }
          else
          {
            index3 = this.game.Data.SFObj[this.IList[attnr].ISFNr].DefMod + 100;
            if (0 > index3)
              index3 = 0;
            if (index3 > 999)
              index3 = 999;
            num28 *=  index3 / 100f;
          }
          matrx[index22, 1] = this.GetPercentChange(num28, oldval16);
          matrx[index22, 2] = Conversions.ToString(num28);
          if (flag1)
          {
            matrx[index22, 1] = "?";
            matrx[index22, 2] = "?";
          }
          this.AddDetail("Attval after AttackMod / DefensiveMod mod: " + Conversion.Str( num28));
        }
        if (!this.previewMode | !flag3)
        {
          index22 += 1;
          matrx[index22, 0] = "Supply".to_owned();
          float oldval17 = num28;
          num28 =  ( num28 * (1.0 -  this.game.Data.RuleVar[130]) +  num28 *  this.game.Data.RuleVar[130] * ( this.game.Data.UnitObj[this.IList[attnr].IUnr].SupplyConsume / 100.0));
          this.AddDetail("attval after SupplyConsume mod: " + Conversion.Str( num28));
          matrx[index22, 1] = this.GetPercentChange(num28, oldval17);
          matrx[index22, 2] = Conversions.ToString(num28);
          if (flag1)
          {
            matrx[index22, 1] = "?";
            matrx[index22, 2] = "?";
          }
        }
        let mut index23: i32 =  index22 + 1;
        matrx[index23, 0] = "People".to_owned();
        float oldval18 = num28;
        let mut people1: i32 =  this.game.Data.SFObj[this.IList[attnr].ISFNr].People;
        let mut people2: i32 =  this.game.Data.SFObj[this.IList[defnr].ISFNr].People;
        let mut people3: i32 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime].People;
        float num29 = num28 * this.game.Data.PeopleObj[people1].BattleForMod[this.game.Data.PeopleObj[people3].PeopleGroup];
        this.AddDetail("Attval after Battle For People X mod: " + Conversion.Str( num29));
        matrx[index23, 1] = this.GetPercentChange(num29, oldval18);
        matrx[index23, 2] = Conversions.ToString(num29);
        if (flag1)
        {
          matrx[index23, 1] = "?";
          matrx[index23, 2] = "?";
        }
        let mut index24: i32 =  index23 + 1;
        matrx[index24, 0] = "VS People";
        float oldval19 = num29;
        float num30 = num29 * this.game.Data.PeopleObj[people1].BattleVSMod[this.game.Data.PeopleObj[people2].PeopleGroup];
        this.AddDetail("Attval after Battle Versus People Y mod: " + Conversion.Str( num30));
        matrx[index24, 1] = this.GetPercentChange(num30, oldval19);
        matrx[index24, 2] = Conversions.ToString(num30);
        if (flag1)
        {
          matrx[index24, 1] = "?";
          matrx[index24, 2] = "?";
        }
        let mut index25: i32 =  index24 + 1;
        matrx[index25, 0] = "Experience".to_owned();
        float oldval20 = num30;
        float num31;
        if ( this.game.Data.RuleVar[877] < 1.0)
        {
          num31 = num30 +  ( num30 * 1.0 * ( this.IList[attnr].IXp / 100.0));
        }
        else
        {
          index3 = (int) Math.Round( (100f - this.game.Data.RuleVar[877]));
          num31 = num30 +  ( num30 * 1.0 * ( index3 / 100.0 * ( this.IList[attnr].IXp / 100.0)));
        }
        this.AddDetail("Attval after XP mod: " + Conversion.Str( num31));
        matrx[index25, 1] = this.GetPercentChange(num31, oldval20);
        matrx[index25, 2] = Conversions.ToString(num31);
        if (flag1)
        {
          matrx[index25, 1] = "?";
          matrx[index25, 2] = "?";
        }
        if (this.IList[attnr].IAttacker == 1)
        {
          index25 += 1;
          matrx[index25, 0] = "Concentric".to_owned();
          float oldval21 = num31;
          num31 *= this.ConcentricBonus;
          this.AddDetail("Attval after Concentric Bonus (" + Conversion.Str( this.ConcentricBonus) + "): " + Conversion.Str( num31));
          matrx[index25, 1] = this.GetPercentChange(num31, oldval21);
          matrx[index25, 2] = Conversions.ToString(num31);
          if (flag1)
          {
            matrx[index25, 1] = "?";
            matrx[index25, 2] = "?";
          }
        }
        if (this.IList[attnr].IAttacker == 0)
        {
          if (this.CombatType == 11 & !counterattack)
          {
            index25 += 1;
            matrx[index25, 0] = "Surprise".to_owned();
            float oldval22 = num31;
            num31 *= this.game.Data.RuleVar[108];
            if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[470] > 0.0)
              num31 *= this.game.Data.RuleVar[108];
            this.AddDetail("Attval after LANDSURPRISE mod: " + Conversion.Str( num31));
            matrx[index25, 1] = this.GetPercentChange(num31, oldval22);
            matrx[index25, 2] = Conversions.ToString(num31);
            if (flag1)
            {
              matrx[index25, 1] = "?";
              matrx[index25, 2] = "?";
            }
          }
          if (this.CombatType == 9 & !counterattack && this.IList[defnr].IParadropper)
          {
            index25 += 1;
            matrx[index25, 0] = "Paradrop".to_owned();
            float oldval23 = num31;
            num31 *= this.game.Data.RuleVar[109];
            this.AddDetail("attval after paradropper mod: " + Conversion.Str( num31));
            matrx[index25, 1] = this.GetPercentChange(num31, oldval23);
            matrx[index25, 2] = Conversions.ToString(num31);
            if (flag1)
            {
              matrx[index25, 1] = "?";
              matrx[index25, 2] = "?";
            }
          }
          if (this.CombatType == 10 & !counterattack)
          {
            index25 += 1;
            matrx[index25, 0] = "Amphibic".to_owned();
            float oldval24 = num31;
            num31 *= this.game.Data.RuleVar[110];
            this.AddDetail("Attval after amph mod: " + Conversion.Str( num31));
            matrx[index25, 1] = this.GetPercentChange(num31, oldval24);
            matrx[index25, 2] = Conversions.ToString(num31);
            if (flag1)
            {
              matrx[index25, 1] = "?";
              matrx[index25, 2] = "?";
            }
          }
        }
        else if (this.CombatType == 12 & !counterattack)
        {
          index25 += 1;
          matrx[index25, 0] = "Rebel".to_owned();
          float oldval25 = num31;
          num31 *= this.game.Data.RuleVar[111];
          this.AddDetail("Attval after REBEL mod: " + Conversion.Str( num31));
          matrx[index25, 1] = this.GetPercentChange(num31, oldval25);
          matrx[index25, 2] = Conversions.ToString(num31);
          if (flag1)
          {
            matrx[index25, 1] = "?";
            matrx[index25, 2] = "?";
          }
        }
        if (this.IList[defnr].IRetreat > 0)
        {
          if (this.IList[defnr].IRetreatMode == 2)
          {
            index25 += 1;
            matrx[index25, 0] = "Retreating target";
            float oldval26 = num31;
            if (this.IList[defnr].IAttacker == 0)
            {
              num31 *= this.game.Data.RuleVar[113];
              this.AddDetail("Attval * x due to defender target is orderly retreating: " + Conversion.Str( num31));
            }
            else
            {
              num31 *= this.game.Data.RuleVar[112];
              this.AddDetail("Attval * x due to attacker target is orderly retreating: " + Conversion.Str( num31));
            }
            matrx[index25, 1] = this.GetPercentChange(num31, oldval26);
            matrx[index25, 2] = Conversions.ToString(num31);
            if (flag1)
            {
              matrx[index25, 1] = "?";
              matrx[index25, 2] = "?";
            }
          }
          if (this.IList[defnr].IRetreatMode == 3)
          {
            index25 += 1;
            matrx[index25, 0] = "Panicking target";
            float oldval27 = num31;
            if (this.IList[defnr].IAttacker == 0)
            {
              num31 *= this.game.Data.RuleVar[115];
              this.AddDetail("Attval * x due to defender target is panic retreating: " + Conversion.Str( num31));
            }
            else
            {
              num31 *= this.game.Data.RuleVar[114];
              this.AddDetail("Attval * x due to attacker target is panic retreating: " + Conversion.Str( num31));
            }
            matrx[index25, 1] = this.GetPercentChange(num31, oldval27);
            matrx[index25, 2] = Conversions.ToString(num31);
            if (flag1)
            {
              matrx[index25, 1] = "?";
              matrx[index25, 2] = "?";
            }
          }
        }
        bool flag8 = true;
        if (this.game.Data.Product >= 6)
        {
          if (this.CombatType == 3 | this.CombatType == 4)
            flag8 = false;
          if (this.InterceptFire)
            flag8 = false;
          if (this.UList[this.IList[attnr].IUlistNr].USupportInterceptFire)
            flag8 = false;
        }
        if (flag8)
        {
          index25 += 1;
          matrx[index25, 0] = "Divisional".to_owned();
          if (this.game.Data.Product >= 6)
            matrx[index25, 0] = "Multi-Unit";
          float oldval28 = num31;
          if (!this.previewMode | this.UList[this.IList[attnr].IUlistNr].previewInfoLevel >= 2)
          {
            index3 = this.game.HandyFunctionsObj.GetDivBonus(this.IList[attnr].IUnr);
            if (index3 > 0)
              num31 += num31 * ( index3 / 100f);
          }
          this.AddDetail("Attval, due to " + Conversion.Str( index3) + "% div bonus: " + Conversion.Str( num31));
          matrx[index25, 1] = this.GetPercentChange(num31, oldval28);
          matrx[index25, 2] = Conversions.ToString(num31);
          if (flag1)
          {
            matrx[index25, 1] = "?";
            matrx[index25, 2] = "?";
          }
        }
        let mut num32: i32 =  0;
        let mut index26: i32 =  index25 + 1;
        matrx[index26, 0] = "AI Bonus";
        float oldval29 = num31;
        if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime].AI)
        {
          if ( this.game.Data.RuleVar[992] > 0.0)
            num32 = (int) Math.Round( this.game.Data.RuleVar[992]);
          let mut num33: i32 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime].AIHelpCombat + num32;
          if (this.customCombatObj.HasCustumCalls())
          {
            CustomCombatCalls customCombatObj = this.customCombatObj;
            combatClass = this;
            ref local: CombatClass = ref combatClass;
            let mut forNr: i32 =  attnr;
            let mut againstNr: i32 =  defnr;
            let mut tHelpCombat: i32 =  num33;
            num33 = customCombatObj.AIHelpCombatChanger(ref local, forNr, againstNr, tHelpCombat);
          }
          if (num33 > 0 && num33 > 0)
            num31 += num31 * ( num33 / 100f);
        }
        this.AddDetail("Attval, due to AI Combat Bonus " + Conversion.Str( num31));
        matrx[index26, 1] = this.GetPercentChange(num31, oldval29);
        matrx[index26, 2] = Conversions.ToString(num31);
        if (flag1)
        {
          matrx[index26, 1] = "?";
          matrx[index26, 2] = "?";
        }
        if (this.game.Data.Product == 7 &  this.game.Data.RuleVar[475] > 0.0)
        {
          index26 += 1;
          matrx[index26, 0] = "Battlegroup Reduced Org.";
          float oldval30 = num31;
          if (this.game.HandyFunctionsObj.CheckIsBattlegroup(this.IList[attnr].IUnr))
          {
            bool flag9 = true;
            if (this.customCombatObj.HasCustumCalls())
            {
              CustomCombatCalls customCombatObj = this.customCombatObj;
              combatClass = this;
              ref local: CombatClass = ref combatClass;
              let mut game: GameClass = this.game;
              let mut attnr5: i32 =  attnr;
              let mut defnr5: i32 =  defnr;
              flag9 = customCombatObj.IndividualCombatCall_SmallSizeBGmodifierApplies(ref local, game, attnr5, defnr5);
            }
            if (flag9)
            {
              num31 *= 0.85f;
              this.AddDetail("Attval after penalty for BG reduced organisation " + Conversion.Str( num31));
            }
          }
          matrx[index26, 1] = this.GetPercentChange(num31, oldval30);
          matrx[index26, 2] = Conversions.ToString(num31);
          if (flag1)
          {
            matrx[index26, 1] = "?";
            matrx[index26, 2] = "?";
          }
        }
        let mut index27: i32 =  index2 + 1;
        matrx[index27, 3] = "Readiness".to_owned();
        float oldval31 = Number1;
        let mut unitGroup3: i32 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].UnitGroup;
        float num34 =  ( Number1 * (1.0 -  this.game.Data.RuleVar[116]) +  Number1 *  this.game.Data.RuleVar[116] * ( this.IList[defnr].IRdn / 100.0));
        if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0)
          num34 =  (int) Math.Round(( oldval31 +  oldval31 +  num34) / 3.0);
        this.AddDetail("Defval after Rdn mod: " + Conversion.Str( num34));
        matrx[index27, 4] = this.GetPercentChange(num34, oldval31);
        matrx[index27, 5] = Conversions.ToString(num34);
        if (flag2)
        {
          matrx[index27, 4] = "?";
          matrx[index27, 5] = "?";
        }
        if ( this.game.Data.RuleVar[482] > 0.0 & this.game.Data.Product >= 6 && this.UList[this.IList[defnr].IUlistNr].Uattacker == 1 & !(this.game.EditObj.attackTypeOption == 0 | this.game.EditObj.attackTypeOption == 3))
        {
          index27 += 1;
          matrx[index27, 3] = "Attack Mode";
          float oldval32 = num34;
          if (this.game.EditObj.attackTypeOption == 1)
            num34 *= 1.5f;
          else if (this.game.EditObj.attackTypeOption == 2)
            num34 *= 1.66f;
          else if (this.game.EditObj.attackTypeOption == 4)
            num34 *= 0.5f;
          this.AddDetail("Defval after Attack Mode mod: " + Conversion.Str( num34));
          matrx[index27, 4] = this.GetPercentChange(num34, oldval32);
          matrx[index27, 5] = Conversions.ToString(num34);
          if (flag2)
          {
            matrx[index27, 4] = "?";
            matrx[index27, 5] = "?";
          }
        }
        if (this.game.Data.UncertaintyOn & !this.previewMode)
        {
          index27 += 1;
          matrx[index27, 3] = "Uncertainty effect";
          float oldval33 = num34;
          let mut udice3: i32 =  this.UList[this.IList[attnr].IUlistNr].UDice;
          let mut udice4: i32 =  this.UList[this.IList[defnr].IUlistNr].UDice;
          num35: i32;
          num36: i32;
          if (this.IList[attnr].IAttacker == 1)
          {
            num35 = udice3 + this.AttackerDice;
            num36 = udice4 + this.DefenderDice;
          }
          else
          {
            num35 = udice3 + this.DefenderDice;
            num36 = udice4 + this.AttackerDice;
          }
          let mut num37: i32 =  num36 - num35;
          if (num37 > 0)
            num34 *=  (1.0 +  num37 / 10.0);
          this.AddDetail("Attval after uncertainty effect: " + Conversion.Str( num34));
          matrx[index27, 4] = this.GetPercentChange(num34, oldval33);
          matrx[index27, 5] = Conversions.ToString(num34);
          if (flag2)
          {
            matrx[index27, 4] = "?";
            matrx[index27, 5] = "?";
          }
        }
        if (!this.previewMode | !flag4)
        {
          index27 += 1;
          matrx[index27, 3] = "Supply".to_owned();
          float oldval34 = num34;
          num34 =  ( num34 * (1.0 -  this.game.Data.RuleVar[130]) +  num34 *  this.game.Data.RuleVar[130] * ( this.game.Data.UnitObj[this.IList[defnr].IUnr].SupplyConsume / 100.0));
          this.AddDetail("Defval after SupplyConsume mod: " + Conversion.Str( num34));
          matrx[index27, 4] = this.GetPercentChange(num34, oldval34);
          matrx[index27, 5] = Conversions.ToString(num34);
          if (flag2)
          {
            matrx[index27, 4] = "?";
            matrx[index27, 5] = "?";
          }
        }
        if (this.game.Data.Product >= 6)
        {
          if (!(this.CombatType == 11 | this.InterceptFire) & !this.UList[this.IList[attnr].IUlistNr].USupportInterceptFire)
          {
            index27 += 1;
            matrx[index27, 3] = "Entrenchment".to_owned();
            float oldval35 = num34;
            if (this.IList[defnr].IAttacker == 0 & this.CombatType != 12)
            {
              let mut ientrench: i32 =  this.IList[defnr].IEntrench;
              num34 *=  (1.0 +  ientrench / 100.0);
              this.AddDetail("Defval after Entrench mod for defending hex: " + Conversion.Str( num34));
            }
            else if (this.IList[defnr].IAttacker == 1 & (this.CombatType == 4 | this.CombatType == 3))
            {
              num34 *=  (1.0 +  this.IList[defnr].IEntrench / 100.0);
              this.AddDetail("Defval after Entrench mod for defending hex: " + Conversion.Str( num34));
            }
            matrx[index27, 4] = this.GetPercentChange(num34, oldval35);
            matrx[index27, 5] = Conversions.ToString(num34);
            if (flag2)
            {
              matrx[index27, 4] = "?";
              matrx[index27, 5] = "?";
            }
            if (this.IList[defnr].IAttacker == 0 & this.CombatType != 12)
            {
              let mut ientrench: i32 =  this.IList[defnr].IEntrench;
              if (this.customCombatObj.HasCustumCalls() & ientrench > 0)
              {
                let mut oldval36: i32 =  (int) Math.Round( num34);
                CustomCombatCalls customCombatObj = this.customCombatObj;
                combatClass = this;
                ref local: CombatClass = ref combatClass;
                let mut indNr: i32 =  defnr;
                let mut x: i32 =  this.CombatTarget.x;
                let mut y: i32 =  this.CombatTarget.y;
                let mut entr: i32 =  ientrench;
                let mut num38: i32 =  customCombatObj.EntrenchModifier(ref local, indNr, x, y, entr) - this.IList[defnr].IEntrench;
                num34 += num34 * ( num38 / 100f);
                if ( num34 !=  oldval36)
                {
                  index27 += 1;
                  matrx[index27, 3] = "Extra Entrenchment";
                  matrx[index27, 4] = this.GetPercentChange(num34,  oldval36);
                  matrx[index27, 5] = Conversions.ToString(num34);
                  if (flag2)
                  {
                    matrx[index27, 4] = "?";
                    matrx[index27, 5] = "?";
                  }
                }
              }
            }
          }
        }
        else
        {
          index27 += 1;
          matrx[index27, 3] = "Entrenchment".to_owned();
          float oldval37 = num34;
          if (this.IList[defnr].IAttacker == 0 & this.CombatType != 12)
          {
            num34 *=  (1.0 +  this.IList[defnr].IEntrench / 100.0);
            this.AddDetail("Defval after Entrench mod for defending hex: " + Conversion.Str( num34));
          }
          matrx[index27, 4] = this.GetPercentChange(num34, oldval37);
          matrx[index27, 5] = Conversions.ToString(num34);
          if (flag2)
          {
            matrx[index27, 4] = "?";
            matrx[index27, 5] = "?";
          }
        }
        let mut index28: i32 =  index27 + 1;
        matrx[index28, 3] = "Experience".to_owned();
        float oldval38 = num34;
        float num39;
        if ( this.game.Data.RuleVar[876] < 1.0)
        {
          num39 = num34 +  ( num34 * 1.0 * ( this.IList[defnr].IXp / 100.0));
        }
        else
        {
          index3 = (int) Math.Round( (100f - this.game.Data.RuleVar[877]));
          num39 = num34 +  ( num34 * 1.0 * ( index3 / 100.0 * ( this.IList[defnr].IXp / 100.0)));
        }
        this.AddDetail("Defval after xp mod: " + Conversion.Str( num39));
        matrx[index28, 4] = this.GetPercentChange(num39, oldval38);
        matrx[index28, 5] = Conversions.ToString(num39);
        if (flag2)
        {
          matrx[index28, 4] = "?";
          matrx[index28, 5] = "?";
        }
        if ( this.game.Data.RuleVar[431] > 0.0)
        {
          index28 += 1;
          matrx[index28, 3] = "Defender is Hidden";
          float oldval39 = num39;
          if ( this.GetEffectiveReconOnHexOfTargettedIndividual(defnr) <  this.IList[defnr].IcoverPoints | this.previewMode & this.UList[this.IList[defnr].IUlistNr].previewInfoLevel < 2 & this.GetEffectiveReconOnHexOfTargettedIndividual(defnr) < this.IList[defnr].previewCoverPoints)
          {
            float num40 = 2f;
            num39 *= num40;
            if (this.previewMode && (this.CombatType == 3 | this.CombatType == 4) & this.game.Data.Product != 7)
            {
              if (this.IList[defnr].IAttacker == 0 & this.CrowdingDefCount > 0)
              {
                if (this.CrowdingDefCount < DrawMod.RandyNumber.Next(0, (int) Math.Round( (this.game.Data.RuleVar[30] * 2f))))
                  num39 *= this.game.Data.RuleVar[30] * 2f /  this.CrowdingDefCount;
              }
              else if (this.IList[defnr].IAttacker == 1 & this.NewBattleStack > 0 && this.NewBattleStack < DrawMod.RandyNumber.Next(0, (int) Math.Round( (this.AttackCrowding * 2f))))
                num39 *= this.AttackCrowding * 2f /  this.NewBattleStack;
            }
            this.AddDetail("Defval after HP bonus for being Hidden: " + Conversion.Str( num39));
          }
          matrx[index28, 4] = this.GetPercentChange(num39, oldval39);
          matrx[index28, 5] = Conversions.ToString(num39);
          if (flag2)
          {
            matrx[index28, 4] = "?";
            matrx[index28, 5] = "?";
          }
        }
        if (this.game.AllowHeightMap & ( this.game.Data.RuleVar[424] > 0.0 |  this.game.Data.RuleVar[425] > 0.0))
        {
          index3 = 0;
          if (this.IList[attnr].IAttacker == 1 & (this.IList[attnr].IdirectFire | !(this.CombatType == 3 | this.CombatType == 4)))
            index3 = 1;
          if (this.IList[attnr].IAttacker == 0)
          {
            if (!(this.CombatType == 3 | this.CombatType == 4))
              index3 = !(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0) ? 1 : 0;
            else if ( this.game.Data.RuleVar[419] > 0.0)
            {
              if (!Information.IsNothing( this.IList[attnr].IdirectFireDef) && this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
                index3 = 1;
            }
            else
              index3 = 0;
          }
          if (index3 == 1)
          {
            let mut num41: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y].HeightLevel - this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.IList[attnr].IUnr].X, this.game.Data.UnitObj[this.IList[attnr].IUnr].Y].HeightLevel;
            if (num41 > 0)
            {
              index28 += 1;
              matrx[index28, 3] = "Defender has highground";
              float oldval40 = num39;
              num39 += num39 *  ( num41 *  this.game.Data.RuleVar[425] / 100.0);
              this.AddDetail("Defval after Defender Highground bonus " + Conversion.Str( num39));
              matrx[index28, 4] = this.GetPercentChange(num39, oldval40);
              matrx[index28, 5] = Conversions.ToString(num39);
              if (flag2)
              {
                matrx[index28, 4] = "?";
                matrx[index28, 5] = "?";
              }
            }
            else if (num41 < 0)
            {
              index28 += 1;
              matrx[index28, 3] = "Attacker has highground";
              float oldval41 = num39;
              num39 += num39 *  ( num41 *  this.game.Data.RuleVar[424] / 100.0);
              this.AddDetail("Defval after Attacker Highground penalty " + Conversion.Str( num39));
              matrx[index28, 4] = this.GetPercentChange(num39, oldval41);
              matrx[index28, 5] = Conversions.ToString(num39);
              if (flag2)
              {
                matrx[index28, 4] = "?";
                matrx[index28, 5] = "?";
              }
            }
          }
        }
        if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[431] > 0.0)
        {
          bool flag10 = false;
          bool flag11 = false;
          if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench)
          {
            if (this.IList[attnr].IBreakTrough > 0 & !this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench)
            {
              if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].ArtRange > 0 & !(this.CombatType == 3 | this.CombatType == 4))
              {
                index28 += 1;
                matrx[index28, 3] = "Very close combat";
                float oldval42 = num39;
                num39 *= 0.33f;
                this.AddDetail("Defval after art-VERY close combat penalty " + Conversion.Str( num39));
                matrx[index28, 4] = this.GetPercentChange(num39, oldval42);
                matrx[index28, 5] = Conversions.ToString(num39);
                if (flag2)
                {
                  matrx[index28, 4] = "?";
                  matrx[index28, 5] = "?";
                }
                flag11 = true;
              }
            }
            else if (!this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].ArtRange > 0 & !(this.CombatType == 3 | this.CombatType == 4))
            {
              index28 += 1;
              matrx[index28, 3] = "Close combat";
              float oldval43 = num39;
              num39 *= 0.66f;
              this.AddDetail("Defval after art-close combat penalty " + Conversion.Str( num39));
              matrx[index28, 4] = this.GetPercentChange(num39, oldval43);
              matrx[index28, 5] = Conversions.ToString(num39);
              if (flag2)
              {
                matrx[index28, 4] = "?";
                matrx[index28, 5] = "?";
              }
              flag10 = true;
            }
          }
          if (((flag2 ? 1 : 0) & 0) != 0)
          {
            if (!flag11)
            {
              index28 += 1;
              matrx[index28, 3] = "Very close combat";
              float oldval44 = num39;
              matrx[index28, 4] = this.GetPercentChange(num39, oldval44);
              matrx[index28, 5] = Conversions.ToString(num39);
              if (flag2)
              {
                matrx[index28, 4] = "?";
                matrx[index28, 5] = "?";
              }
            }
            if (!flag10)
            {
              index28 += 1;
              matrx[index28, 3] = "Close combat";
              float oldval45 = num39;
              matrx[index28, 4] = this.GetPercentChange(num39, oldval45);
              matrx[index28, 5] = Conversions.ToString(num39);
              if (flag2)
              {
                matrx[index28, 4] = "?";
                matrx[index28, 5] = "?";
              }
            }
          }
        }
        if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[484] > 0.0)
        {
          index28 += 1;
          matrx[index28, 3] = "March Mode";
          float oldval46 = num39;
          if (this.game.Data.UnitObj[this.IList[defnr].IUnr].moveMode == 1)
          {
            if (this.CombatRound == 1)
              num39 *= 0.05f;
            if (this.CombatRound == 2)
              num39 *= 0.15f;
            if (this.CombatRound == 3)
              num39 *= 0.3f;
            if (this.CombatRound == 4)
              num39 *= 0.5f;
            if (this.CombatRound == 5)
              num39 *= 0.75f;
          }
          this.AddDetail("Defval after March Mode penalty " + Conversion.Str( num39));
          matrx[index28, 4] = this.GetPercentChange(num39, oldval46);
          matrx[index28, 5] = Conversions.ToString(num39);
          if (flag2)
          {
            matrx[index28, 4] = "?";
            matrx[index28, 5] = "?";
          }
          index26 += 1;
          matrx[index26, 0] = "March Mode";
          float oldval47 = num31;
          if (this.game.Data.UnitObj[this.IList[attnr].IUnr].moveMode == 1)
          {
            if (this.CombatRound == 1)
              num31 *= 0.1f;
            if (this.CombatRound == 2)
              num31 *= 0.3f;
            if (this.CombatRound == 3)
              num31 *= 0.5f;
            if (this.CombatRound == 4)
              num31 *= 0.7f;
            if (this.CombatRound == 5)
              num31 *= 0.9f;
          }
          this.AddDetail("Attval after March Mode penalty: " + Conversion.Str( num31));
          matrx[index26, 1] = this.GetPercentChange(num31, oldval47);
          matrx[index26, 2] = Conversions.ToString(num31);
          if (flag1)
          {
            matrx[index26, 1] = "?";
            matrx[index26, 2] = "?";
          }
        }
        let mut index29: i32 =  index28 + 1;
        matrx[index29, 3] = "AI Bonus";
        float oldval48 = num39;
        let mut num42: i32 =  0;
        if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime].AI)
        {
          if ( this.game.Data.RuleVar[992] > 0.0)
            num42 = (int) Math.Round( this.game.Data.RuleVar[992]);
          let mut num43: i32 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime].AIHelpCombat + num42;
          if (this.customCombatObj.HasCustumCalls())
          {
            CustomCombatCalls customCombatObj = this.customCombatObj;
            combatClass = this;
            ref local: CombatClass = ref combatClass;
            let mut forNr: i32 =  defnr;
            let mut againstNr: i32 =  attnr;
            let mut tHelpCombat: i32 =  num43;
            num43 = customCombatObj.AIHelpCombatChanger(ref local, forNr, againstNr, tHelpCombat);
          }
          if (num43 > 0)
          {
            num39 += num39 * ( num43 / 100f);
            this.AddDetail("Defval, due to " + Conversion.Str( index3) + " AI Combat Bonus " + Conversion.Str( num39));
          }
        }
        matrx[index29, 4] = this.GetPercentChange(num39, oldval48);
        matrx[index29, 5] = Conversions.ToString(num39);
        if (flag2)
        {
          matrx[index29, 4] = "?";
          matrx[index29, 5] = "?";
        }
        if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[475] > 0.0)
        {
          index29 += 1;
          matrx[index29, 3] = "Battlegroup small size";
          float oldval49 = num39;
          if (this.game.HandyFunctionsObj.CheckIsBattlegroup(this.IList[defnr].IUnr) &&  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.IList[defnr].IUnr) <  this.game.Data.RuleVar[476])
          {
            bool flag12 = true;
            if (this.customCombatObj.HasCustumCalls())
            {
              CustomCombatCalls customCombatObj = this.customCombatObj;
              combatClass = this;
              ref local: CombatClass = ref combatClass;
              let mut game: GameClass = this.game;
              let mut attnr6: i32 =  attnr;
              let mut defnr6: i32 =  defnr;
              flag12 = customCombatObj.IndividualCombatCall_SmallSizeBGmodifierApplies(ref local, game, attnr6, defnr6);
            }
            if (flag12)
            {
              num39 = num39 *  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.IList[defnr].IUnr) / this.game.Data.RuleVar[476];
              this.AddDetail("Defval after penalty for small battlegroup size " + Conversion.Str( num39));
            }
          }
          matrx[index29, 4] = this.GetPercentChange(num39, oldval49);
          matrx[index29, 5] = Conversions.ToString(num39);
          if (flag2)
          {
            matrx[index29, 4] = "?";
            matrx[index29, 5] = "?";
          }
        }
        let mut index30: i32 =  index29 + 1;
        matrx[index30, 3] = "River".to_owned();
        float oldval50 = num39;
        if (this.IList[defnr].IAttacker != 0 && !this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench & !(this.CombatType == 3 | this.CombatType == 4) && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0 && this.IList[defnr].IBreakTrough == 0 & this.game.Data.UnitObj[this.IList[defnr].IUnr].Map == this.CombatTarget.map && this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y, this.game.Data.UnitObj[this.IList[defnr].IUnr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map) == 1)
        {
          let mut index31: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y, this.game.Data.UnitObj[this.IList[defnr].IUnr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map) - 1;
          let mut index32: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y].RiverType[index31];
          bool flag13 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.IList[defnr].IUnr].X, this.game.Data.UnitObj[this.IList[defnr].IUnr].Y].Bridge[index31];
          if (index32 > -1)
          {
            if (this.customCombatObj.HasCustumCalls())
            {
              float num44 = this.game.Data.RiverTypeObj[index32].AttackPenalty[unitGroup3];
              if (flag13)
                num44 *= this.game.Data.RuleVar[5];
              let mut num45: i32 =  (int) Math.Round((1.0 -  num44) * 100.0);
              str5: String = "";
              CustomCombatCalls customCombatObj = this.customCombatObj;
              combatClass = this;
              ref local9: CombatClass = ref combatClass;
              let mut game: GameClass = this.game;
              let mut attnr7: i32 =  attnr;
              let mut defnr7: i32 =  defnr;
              let mut num46: i32 =  counterattack ? 1 : 0;
              ref local10: String = ref str5;
              let mut riverHpMod: i32 =  num45;
              let mut riverType: i32 =  index32;
              let mut num47: i32 =  customCombatObj.IndividualCombatCall_RiverTypeAndLandscapeTypeModifier(ref local9, game, attnr7, defnr7, num46 != 0, ref local10, riverHpMod, 0, 0, riverType);
              num39 =  ( num39 *  num47 / 100.0);
              this.AddDetail("Defval after attack over river/bridge mod: " + Conversion.Str( num39));
              if (str5.Length > 0)
                str2 = str2 + "\r\n" + str5;
            }
            else
            {
              float num48 = this.game.Data.RiverTypeObj[index32].AttackPenalty[unitGroup3];
              if (flag13)
                num48 *= this.game.Data.RuleVar[5];
              num39 *= 1f - num48;
              this.AddDetail("Defval after attack over river/bridge mod: " + Conversion.Str( num39));
            }
          }
        }
        matrx[index30, 4] = this.GetPercentChange(num39, oldval50);
        matrx[index30, 5] = Conversions.ToString(num39);
        if (flag2)
        {
          matrx[index30, 4] = "?";
          matrx[index30, 5] = "?";
        }
        let mut index33: i32 =  index30 + 1;
        matrx[index33, 3] = "Overstack".to_owned();
        float oldval51 = num39;
        if (this.IList[defnr].IAttacker == 0 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0)
        {
          num39 *= this.CrowdingDefMod;
          this.AddDetail("defval after Crowding def Mod: " + Conversion.Str( num39));
        }
        if (this.IList[defnr].IAttacker == 1 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0 && !(this.CombatType == 3 | this.CombatType == 4))
        {
          num39 *= this.CrowdingAttMod;
          this.AddDetail("defval after Crowding att Mod: " + Conversion.Str( num39));
        }
        let mut num49: i32 =  this.IList[defnr].IAttacker == 1 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2 ? 1 : 0;
        matrx[index33, 4] = this.GetPercentChange(num39, oldval51);
        matrx[index33, 5] = Conversions.ToString(num39);
        if (flag2)
        {
          matrx[index33, 4] = "?";
          matrx[index33, 5] = "?";
        }
        if ( this.game.Data.RuleVar[835] > 0.0)
        {
          index33 += 1;
          matrx[index33, 3] = "Airbase surprise";
          oldval51 = num39;
          float num50 = this.game.Data.RuleVar[835];
          if (this.CombatRound > 1)
            num50 -=  (this.CombatRound - 1) * this.game.Data.RuleVar[836];
          if (this.game.Data.Product == 7)
            num50 *= 0.66f;
          if ( num50 > 0.0)
          {
            float Number2 =  (1.0 -  num50 / 100.0);
            if (this.IList[defnr].IAttacker == 0 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2 && this.game.Data.UnitObj[this.IList[defnr].IUnr].X == this.TargetX & this.game.Data.UnitObj[this.IList[defnr].IUnr].Y == this.TargetY)
            {
              num39 *= Number2;
              this.AddDetail("defval after penalty (" + Conversion.Str( Number2) + ") for airfield under attack: " + Conversion.Str( num39));
            }
          }
        }
        matrx[index33, 4] = this.GetPercentChange(num39, oldval51);
        matrx[index33, 5] = Conversions.ToString(num39);
        if (flag2)
        {
          matrx[index33, 4] = "?";
          matrx[index33, 5] = "?";
        }
        if (this.game.Data.Product == 7)
        {
          if ( this.game.Data.RuleVar[835] > 0.0)
          {
            index26 += 1;
            matrx[index26, 0] = "Airbase surprise";
            oldval51 = num31;
            float num51 = this.game.Data.RuleVar[835];
            if (this.CombatRound > 1)
              num51 -=  (this.CombatRound - 1) * this.game.Data.RuleVar[836];
            if ( num51 > 0.0)
            {
              float Number3 =  (1.0 -  num51 / 100.0);
              if (this.IList[attnr].IAttacker == 0 & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 2 && this.game.Data.UnitObj[this.IList[attnr].IUnr].X == this.TargetX & this.game.Data.UnitObj[this.IList[attnr].IUnr].Y == this.TargetY)
              {
                num31 *= Number3;
                this.AddDetail("attval after penalty (" + Conversion.Str( Number3) + ") for airfield under attack: " + Conversion.Str( num31));
              }
            }
          }
          matrx[index26, 1] = this.GetPercentChange(num31, oldval51);
          matrx[index26, 2] = Conversions.ToString(num31);
          if (flag2)
          {
            matrx[index26, 1] = "?";
            matrx[index26, 2] = "?";
          }
        }
        let mut index34: i32 =  index26 + 1;
        matrx[index34, 0] = "Overstack".to_owned();
        float oldval52 = num31;
        if (this.IList[attnr].IAttacker == 0 & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0)
        {
          float Number4 = this.CrowdingDefMod;
          if ( Number4 < 1.0)
            Number4 =  ((1.0 +  Number4 +  Number4 +  Number4 * (1.0 /  Number4 - 1.0)) / (3.0 + (1.0 /  Number4 - 1.0)));
          num31 *= Number4;
          this.AddDetail("attval after Crowding def Mod(x%) (" + Conversion.Str( Number4) + "): " + Conversion.Str( num31));
        }
        if (this.IList[attnr].IAttacker == 1 & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0)
        {
          float Number5 = !(this.CombatType == 3 | this.CombatType == 4) ? this.CrowdingAttMod : this.CrowdingAttArtMod;
          if ( Number5 < 1.0)
            Number5 =  ((1.0 +  Number5 +  Number5 +  Number5 * (1.0 /  Number5 - 1.0)) / (3.0 + (1.0 /  Number5 - 1.0)));
          num31 *= Number5;
          if (this.CombatType == 3 | this.CombatType == 4)
          {
            matrx[index34, 0] = "Overstack Art";
            this.AddDetail("attval after Crowding att ART Mod (x%) (" + Conversion.Str( Number5) + "): " + Conversion.Str( num31));
          }
          else
            this.AddDetail("attval after Crowding att Mod (x%) (" + Conversion.Str( Number5) + "): " + Conversion.Str( num31));
        }
        if (this.IList[attnr].IAttacker == 1 & this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 2 & this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater != 2)
        {
          float Number6 = this.CrowdingAttAirMod;
          if ( Number6 < 1.0)
            Number6 =  ((1.0 +  Number6 +  Number6 +  Number6 * (1.0 /  Number6 - 1.0)) / (3.0 + (1.0 /  Number6 - 1.0)));
          num31 *= Number6;
          matrx[index34, 0] = "Overstack Air";
          this.AddDetail("attval after Crowding att AIR Mod (x%) (" + Conversion.Str( Number6) + "): " + Conversion.Str( num31));
        }
        matrx[index34, 1] = this.GetPercentChange(num31, oldval52);
        matrx[index34, 2] = Conversions.ToString(num31);
        if (flag1)
        {
          matrx[index34, 1] = "?";
          matrx[index34, 2] = "?";
        }
        bool flag14 = true;
        if (this.game.Data.Product >= 6)
        {
          if (this.CombatType == 3 | this.CombatType == 4)
            flag14 = false;
          if (this.InterceptFire)
            flag14 = false;
          if (this.UList[this.IList[attnr].IUlistNr].USupportInterceptFire)
            flag14 = false;
        }
        if (flag14)
        {
          index33 += 1;
          matrx[index33, 3] = "Divisional".to_owned();
          if (this.game.Data.Product >= 6)
            matrx[index33, 3] = "Multi-Unit";
          float oldval53 = num39;
          index3 = this.game.HandyFunctionsObj.GetDivBonus(this.IList[defnr].IUnr);
          if (index3 > 0)
          {
            num39 += num39 * ( index3 / 100f);
            this.AddDetail("Defval, due to " + Conversion.Str( index3) + "% div bonus: " + Conversion.Str( num39));
          }
          matrx[index33, 4] = this.GetPercentChange(num39, oldval53);
          matrx[index33, 5] = Conversions.ToString(num39);
          if (flag2)
          {
            matrx[index33, 4] = "?";
            matrx[index33, 5] = "?";
          }
        }
        if (this.customCombatObj.HasCustumCalls())
        {
          let mut num52: i32 =  this.customCombatObj.NumberOfMods();
          for (index3 = 1; index3 <= num52; index3 += 1)
          {
            float oldval54 = num31;
            str6: String = "";
            CustomCombatCalls customCombatObj = this.customCombatObj;
            let mut modNr: i32 =  index3;
            combatClass = this;
            ref local11: CombatClass = ref combatClass;
            let mut game: GameClass = this.game;
            let mut attnr8: i32 =  attnr;
            let mut defnr8: i32 =  defnr;
            let mut num53: i32 =  counterattack ? 1 : 0;
            double attval =  num31;
            ref local12: String = ref str6;
            num31 = customCombatObj.IndividualCombatCall_AttValModder(modNr, ref local11, game, attnr8, defnr8, num53 != 0,  attval, ref local12);
            if ( num31 !=  oldval54)
            {
              index34 += 1;
              matrx[index34, 0] = this.customCombatObj.GetModName(index3);
              if ( num31 >  oldval54)
                num31 = num31;
              matrx[index34, 1] = this.GetPercentChange(num31, oldval54);
              matrx[index34, 2] = Conversions.ToString(num31);
              if (flag1)
              {
                matrx[index34, 1] = "?";
                matrx[index34, 2] = "?";
              }
            }
            if (str6.Length > 0)
              str2 = str2 + "\r\n" + str6;
          }
        }
        if (this.customCombatObj.HasCustumCalls())
        {
          let mut num54: i32 =  this.customCombatObj.NumberOfMods();
          for (index3 = 1; index3 <= num54; index3 += 1)
          {
            float oldval55 = num39;
            str7: String = "";
            CustomCombatCalls customCombatObj = this.customCombatObj;
            let mut modNr: i32 =  index3;
            combatClass = this;
            ref local13: CombatClass = ref combatClass;
            let mut game: GameClass = this.game;
            let mut attnr2: i32 =  attnr;
            let mut defnr9: i32 =  defnr;
            let mut num55: i32 =  counterattack ? 1 : 0;
            double defval =  num39;
            ref local14: String = ref str7;
            num39 = customCombatObj.IndividualCombatCall_DefValModder(modNr, ref local13, game, attnr2, defnr9, num55 != 0,  defval, ref local14);
            if ( num39 !=  oldval55)
            {
              index33 += 1;
              matrx[index33, 3] = this.customCombatObj.GetModName(index3);
              matrx[index33, 4] = this.GetPercentChange(num39, oldval55);
              matrx[index33, 5] = Conversions.ToString(num39);
              if (flag2)
              {
                matrx[index33, 4] = "?";
                matrx[index33, 5] = "?";
              }
            }
            if (str7.Length > 0)
              str2 = str2 + "\r\n" + str7;
          }
        }
        let mut index35: i32 =  index34 + 1;
        if (index35 >= 31)
          index3 = index3;
        matrx[index35, 0] = "After mods";
        matrx[index35, 1] = "";
        matrx[index35, 2] = Conversions.ToString(num31);
        let mut index36: i32 =  index33 + 1;
        if (index36 >= 31)
          index3 = index3;
        matrx[index36, 3] = "After mods";
        matrx[index36, 4] = "";
        matrx[index36, 5] = Conversions.ToString(num39);
        if ( num31 == 0.0)
          num31 = num31;
        s2: String = "AFTER ALL MODS: Attval=" + Conversion.Str( num31) + ", DefVal= " + Conversion.Str( num39);
        this.AddDetail(s2);
        txt: String;
        if ( this.game.Data.RuleVar[431] < 1.0)
          txt = str2 + "\r\n" + "After modifications: Attack score=" + Strings.Trim(Conversion.Str( Math.Round( num31, 2))) + " VS Defensive score=" + Strings.Trim(Conversion.Str( Math.Round( num39, 2)));
        else if (!this.game.Data.FOWOn)
        {
          if (this.IList[attnr].IvisibleFromRound >= this.CombatRound & this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime != this.game.Data.Turn)
            txt = "Attack score = " + Strings.Trim(Conversion.Str( Math.Round( num31, 2))) + " [Hidden] VS Defensive score=" + Strings.Trim(Conversion.Str( Math.Round( num39, 2)));
          else if (this.IList[defnr].IvisibleFromRound >= this.CombatRound & this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime != this.game.Data.Turn)
            txt = "Attack score=" + Strings.Trim(Conversion.Str( Math.Round( num31, 2))) + " VS Defensive score=" + Strings.Trim(Conversion.Str( Math.Round( num39, 2))) + " [Hidden]";
          else
            txt = str2 + "\r\n" + "After modifications: Attack score=" + Strings.Trim(Conversion.Str( Math.Round( num31, 2))) + " VS Defensive score=" + Strings.Trim(Conversion.Str( Math.Round( num39, 2)));
        }
        else if (this.IList[attnr].IvisibleFromRound >= this.CombatRound & this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime != this.game.Data.Turn)
          txt = "Attack score = ? VS Defensive score=" + Strings.Trim(Conversion.Str( Math.Round( num39, 2)));
        else if (this.IList[defnr].IvisibleFromRound >= this.CombatRound & this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime != this.game.Data.Turn)
          txt = "Attack score=" + Strings.Trim(Conversion.Str( Math.Round( num31, 2))) + " VS Defensive score=?";
        else
          txt = str2 + "\r\n" + "After modifications: Attack score=" + Strings.Trim(Conversion.Str( Math.Round( num31, 2))) + " VS Defensive score=" + Strings.Trim(Conversion.Str( Math.Round( num39, 2)));
        str8: String = txt;
        float num56;
        if ( powerPts1 >  powerPts2)
        {
          num56 = powerPts2 / powerPts1;
          if ( num56 <  this.game.Data.RuleVar[117])
            num56 = this.game.Data.RuleVar[117];
        }
        else if ( powerPts2 >  powerPts1)
        {
          num56 = powerPts2 / powerPts1;
          if ( num56 <  this.game.Data.RuleVar[117])
            num56 = this.game.Data.RuleVar[117];
        }
        else
          num56 = 1f;
        if ( num56 >  this.game.Data.RuleVar[118])
          num56 = this.game.Data.RuleVar[118];
        float num57 =  Conversion.Int( DrawMod.RandyNumber.Next(0, 10000) / 10000.0 *  num31);
        float num58 =  Conversion.Int( DrawMod.RandyNumber.Next(0, 10000) / 10000.0 *  num39);
        let mut num59: i32 =  0;
        if ( num57 >=  num58)
          num59 = 1;
        let mut num60: i32 =  0;
        if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].DepletingHitpointRule > 0 & num59 == 1 && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].PreventCounter == -1 & !this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench &&  this.IList[defnr].IHp -  num57 > 0.0)
          num59 = 0;
        str9: String = "";
        str10: String;
        if (this.game.Data.Product >= 6 && num59 == 0 && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].DepletingHitpointRule > 0)
        {
          bool flag15;
          if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench & this.IList[attnr].IBreakTrough > 0)
          {
            num57 *= 10f;
            flag15 = true;
          }
          str11: String;
          if ( this.IList[defnr].IHp -  num57 <= 0.0 & (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench |  num57 >=  num58) | flag15 &  num57 >=  num58)
          {
            if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench & this.IList[attnr].IBreakTrough < 1)
            {
              num59 = 1;
              num60 = 3;
              this.IList[defnr].IHp = 0;
              str10 = str11 + " : SHIELDING DEPLETED";
              txt = txt + "\r\n" + "Result of attack is a RETREAT.";
              str9 = str9 + "\r\n" + "Result of attack is a RETREAT.";
            }
            else if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench)
            {
              this.IList[defnr].IHp = 0;
              num59 = 1;
              num60 = 1;
              str10 = str11 + " : CLOSE DISTANCE KILL";
              txt = txt + "\r\n" + "Result of attack is a KILL.";
              str9 = str9 + "\r\n" + "Result of attack is a KILL.";
            }
            else
            {
              this.IList[defnr].IHp = 0;
              if ( num57 >=  num58)
              {
                num59 = 1;
                num60 = 1;
                str10 = str11 + " : SHIELDING DEPLETED";
                txt = txt + "\r\n" + "Result of attack is a KILL.";
                str9 = str9 + "\r\n" + "Result of attack is a KILL.";
              }
              else
              {
                num59 = 1;
                num60 = 3;
                str10 = str11 + " : SHIELDING DEPLETED";
                txt = txt + "\r\n" + "Result of attack is a RETREAT.";
                str9 = str9 + "\r\n" + "Result of attack is a RETREAT.";
              }
            }
          }
          else if (this.IList[defnr].IHp > 0)
          {
            if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench |  num57 >=  num58)
            {
              this.IList[defnr].IHp = (int) Math.Round( ( this.IList[defnr].IHp - num57));
              num59 = 0;
              str10 = str11 + " : SHIELDING ABSORBED HIT";
              txt = txt + "\r\n" + "Shielding down to " + this.IList[defnr].IHp.ToString() + ".";
              str9 = str9 + "\r\n" + "Shielding down to " + this.IList[defnr].IHp.ToString() + ".";
            }
            else
            {
              str10 = str11 + " : SHIELDING INTACT";
              txt = txt + "\r\n" + "Shielding has not been drained.";
              str9 = str9 + "\r\n" + "Shielding has not been drained.";
            }
          }
        }
        if (this.game.Data.Product == 6 & this.CombatType == 1 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].EP > 0 & !counterattack & this.IList[attnr].AttackCount == 0 && this.IList[defnr].IEntrench > 0 &&  this.UList[this.IList[attnr].IUlistNr].UepEpMaxMod > 0.0 & this.IList[defnr].IEntrench > 0)
        {
          index3 = (int) Math.Round(100.0 *  this.UList[this.IList[attnr].IUlistNr].UepEpMaxMod / 100.0);
          if (index3 > 100)
            index3 = 100;
          if (!this.previewMode)
          {
            this.game.Data.SFObj[this.IList[attnr].ISFNr].EP = Math.Min(this.game.Data.SFObj[this.IList[attnr].ISFNr].EP - 1, (int) Math.Round( this.game.Data.SFObj[this.IList[attnr].ISFNr].EP / 2.0));
            if (0 > this.game.Data.SFObj[this.IList[attnr].ISFNr].EP)
              this.game.Data.SFObj[this.IList[attnr].ISFNr].EP = 0;
          }
          if (num59 == 1)
            index3 *= 2;
          if (index3 > this.IList[defnr].IEntrench)
            index3 = this.IList[defnr].IEntrench;
          if (index3 > 0)
            this.IList[defnr].IEntrench -= index3;
          txt = txt + "\r\n" + "Sapper action, using EP, reduces enemy entrenchment by -" + index3.ToString() + ".";
          if (this.IList[defnr].IEntrench <= 0)
            txt += " Enemy entrenchment fully depleted.";
        }
        if (num59 == 1)
        {
          let mut num61: i32 =  (int) Math.Round( (Conversion.Int(VBMath.Rnd() * 100f) + 1f));
          let mut killPercent: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].KillPercent;
          if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType > -1 & (this.CombatType == 3 | this.CombatType == 4))
            killPercent = this.game.Data.SFTypeObj[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType].KillPercent;
          if (killPercent >= num61)
            num60 = 1;
          if (num60 == 0)
          {
            let mut num62: i32 =  !(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType > -1 & (this.CombatType == 3 | this.CombatType == 4)) ? this.game.Data.SFTypeObj[this.IList[attnr].ISFType].RetreatPercent : this.game.Data.SFTypeObj[this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtSFType].RetreatPercent;
            if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0 & !(this.CombatType == 3 | this.CombatType == 4) &  this.game.Data.RuleVar[895] > 0.0)
            {
              num62 = (int) Math.Round( ( num62 * this.game.Data.RuleVar[895]));
              txt = txt + "\r\n" + "Chance for retreat score was modified by " + this.game.Data.RuleVar[895].ToString() + " due to using artillery in a frontal attack.";
            }
            if (killPercent + num62 >= num61)
              num60 = 3;
            if (num60 == 0)
              num60 = 4;
          }
          if (num60 == 1 && this.IList[defnr].IAttacker == 0 &&  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].KilltoRetreatChance >  VBMath.Rnd() * 100.0)
          {
            s2 += ", Succesfull Kill to Retreat by defender ";
            txt = txt + "\r\n" + "Defender manages to avoid kill result and turn it into retreat result.";
            num60 = 3;
          }
          if (num60 == 3 && this.IList[defnr].IAttacker == 0 && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2 && this.game.Data.UnitObj[this.IList[defnr].IUnr].X == this.TargetX)
          {
            let mut num63: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].Y == this.TargetY & this.game.Data.UnitObj[this.IList[defnr].IUnr].Map == this.TargetMap ? 1 : 0;
          }
          if (num60 == 3 & this.IList[defnr].IRetreatMode == 1 && this.IList[defnr].IRetreat == this.CombatRound)
          {
            s2 = ", Because sub was already forced retreating retreat target is changed to pinned.";
            txt = txt + "\r\n" + "Retreat result is handled as pinned result because target was already retreating.";
            num60 = 4;
          }
          if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].SlotNumber > -1 && this.IList[attnr].IAttacker == 1)
          {
            int[] areaCode = this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].AreaCode;
            int[] numArray = areaCode;
            let mut slotNumber: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].SlotNumber;
            let mut index37: i32 =  slotNumber;
            let mut num64: i32 =  areaCode[slotNumber] + 1;
            numArray[index37] = num64;
          }
          if (!this.previewMode && this.customCombatObj.HasCustumCalls())
          {
            str12: String = "";
            CustomCombatCalls customCombatObj = this.customCombatObj;
            let mut result: i32 =  num60;
            combatClass = this;
            ref local15: CombatClass = ref combatClass;
            let mut game: GameClass = this.game;
            let mut attnr9: i32 =  attnr;
            let mut defnr10: i32 =  defnr;
            let mut num65: i32 =  counterattack ? 1 : 0;
            double attval =  num57;
            double defval =  num58;
            ref local16: String = ref str12;
            num60 = customCombatObj.IndividualCombatCall_ResultModifier(result, ref local15, game, attnr9, defnr10, num65 != 0,  attval,  defval, ref local16);
            if (str12.Length > 1)
              txt = txt + "\r\n" + str12;
            if (num60 < 1)
              num59 = 0;
            if (num60 > 0)
              num59 = 1;
          }
          let mut num66: i32 =  0;
          if ( this.game.Data.RuleVar[431] > 0.0)
          {
            let mut landscapeTypeCounter: i32 =  this.game.Data.LandscapeTypeCounter;
            for (let mut index38: i32 =  0; index38 <= landscapeTypeCounter; index38 += 1)
            {
              if (this.game.Data.LandscapeTypeObj[index38].HidePts > num66)
                num66 = this.game.Data.LandscapeTypeObj[index38].HidePts;
            }
            if (num66 < 50)
              num66 = 50;
            let mut num67: i32 =  (int) Math.Round(800.0 /  (int) Math.Round( (this.game.Data.RuleVar[56] - this.game.Data.RuleVar[55])));
            if (this.CombatType == 3)
            {
              if (this.IList[attnr].IAttacker == 1)
              {
                if (this.IList[attnr].IdirectFire)
                {
                  float num68 =  (int) Math.Round( ( Math.Max(10, num66 - this.GetIndividualHide(attnr)) /  num67 /  Math.Max(this.IList[defnr].AttackedCount, 1)) *  this.UList[this.IList[defnr].IUlistNr].ULos[this.IList[attnr].IUlistNr] / 100.0);
                  Individual[] ilist = this.IList;
                  Individual[] individualArray = ilist;
                  let mut index39: i32 =  attnr;
                  let mut index40: i32 =  index39;
                  individualArray[index40].IcoverPoints = ilist[index39].IcoverPoints - num68;
                }
                else
                {
                  float num69 =  this.GetUnmodifiedReconOnHexOfTargettedIndividual(attnr) /  num67;
                  Individual[] ilist = this.IList;
                  Individual[] individualArray = ilist;
                  let mut index41: i32 =  attnr;
                  let mut index42: i32 =  index41;
                  individualArray[index42].IcoverPoints = ilist[index41].IcoverPoints - num69;
                }
              }
              else if (this.IList[attnr].IAttacker == 0)
              {
                if (this.IList[attnr].IdirectFire)
                {
                  if (Information.IsNothing( this.IList[attnr].IdirectFireDef))
                  {
                    if (this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
                    {
                      float num70 =  Math.Max(10, num66 - this.GetIndividualHide(attnr)) /  num67 /  Math.Max(this.IList[defnr].AttackedCount, 1);
                      float num71 =  (int) Math.Round( (index3 * this.UList[this.IList[defnr].IUlistNr].ULos[this.IList[attnr].IUlistNr]) / 100.0);
                      Individual[] ilist = this.IList;
                      Individual[] individualArray = ilist;
                      let mut index43: i32 =  attnr;
                      let mut index44: i32 =  index43;
                      individualArray[index44].IcoverPoints = ilist[index43].IcoverPoints - num71;
                    }
                    else
                    {
                      float num72 =  this.GetUnmodifiedReconOnHexOfTargettedIndividual(attnr) /  num67;
                      Individual[] ilist = this.IList;
                      Individual[] individualArray = ilist;
                      let mut index45: i32 =  attnr;
                      let mut index46: i32 =  index45;
                      individualArray[index46].IcoverPoints = ilist[index45].IcoverPoints - num72;
                    }
                  }
                }
                else
                {
                  float targettedIndividual =  this.GetUnmodifiedReconOnHexOfTargettedIndividual(attnr);
                  Individual[] ilist = this.IList;
                  Individual[] individualArray = ilist;
                  let mut index47: i32 =  attnr;
                  let mut index48: i32 =  index47;
                  individualArray[index48].IcoverPoints = ilist[index47].IcoverPoints - targettedIndividual;
                }
              }
              else
              {
                float num73 =  Math.Max(10, num66 - this.GetIndividualHide(attnr)) /  num67 /  Math.Max(this.IList[defnr].AttackedCount, 1);
                Individual[] ilist = this.IList;
                Individual[] individualArray = ilist;
                let mut index49: i32 =  attnr;
                let mut index50: i32 =  index49;
                individualArray[index50].IcoverPoints = ilist[index49].IcoverPoints - num73;
              }
              if ( this.IList[attnr].IcoverPoints < 0.0)
                this.IList[attnr].IcoverPoints = 0.0f;
            }
          }
          if (this.game.Data.Product >= 6 && this.IList[defnr].AttackedCount == 0)
          {
            if ( this.game.Data.RuleVar[467] > 0.0 &&  this.IList[defnr].ILisAmmoMod < 1.0)
            {
              index3 = (int) Math.Round( this.game.Data.RuleVar[467]) - (int) Math.Round(Math.Ceiling( this.IList[defnr].ILisAmmoMod *  this.game.Data.RuleVar[467]));
              this.ReduceMor(defnr, index3);
            }
            if ( this.game.Data.RuleVar[468] > 0.0 &&  this.IList[defnr].ILisAmmoMod < 1.0)
            {
              index3 = (int) Math.Round( this.game.Data.RuleVar[468]) - (int) Math.Round(Math.Ceiling( this.IList[defnr].ILisAmmoMod *  this.game.Data.RuleVar[468]));
              if (this.CombatType == 3 | this.CombatType == 4)
                index3 = (int) Math.Round( index3 / 2.0);
              this.ReduceAbsRdn(defnr, index3);
            }
          }
          if (num59 == 1)
          {
            if ( this.game.Data.RuleVar[438] > 0.0 &&  this.IList[defnr].ILisAmmoMod < 1.0)
            {
              index3 = (int) Math.Round( this.game.Data.RuleVar[438]) - (int) Math.Round(Math.Ceiling( this.IList[defnr].ILisAmmoMod *  this.game.Data.RuleVar[438]));
              if (this.IList[defnr].IAttacker == 1)
                index3 *= 2;
              this.ReduceMor(defnr, index3);
            }
            if ( this.game.Data.RuleVar[431] > 0.0)
            {
              if (this.CombatType == 3)
              {
                if (this.IList[attnr].IAttacker == 1)
                {
                  if (this.IList[attnr].IdirectFire)
                    this.IList[defnr].IcoverPoints = 0.0f;
                  else if (DrawMod.RandyNumber.Next(0, (int) Math.Round( this.game.Data.RuleVar[56])) < this.GetEffectiveReconOnHexOfTargettedIndividual(defnr))
                    this.IList[defnr].IcoverPoints = 0.0f;
                }
                else if ( this.game.Data.RuleVar[419] > 0.0)
                {
                  if (!Information.IsNothing( this.IList[attnr].IdirectFireDef))
                  {
                    if (this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
                      this.IList[defnr].IcoverPoints = 0.0f;
                    else if (DrawMod.RandyNumber.Next(0, (int) Math.Round( this.game.Data.RuleVar[56])) < this.GetEffectiveReconOnHexOfTargettedIndividual(defnr))
                      this.IList[defnr].IcoverPoints = 0.0f;
                  }
                }
                else if (DrawMod.RandyNumber.Next(0, (int) Math.Round( this.game.Data.RuleVar[56])) < this.GetEffectiveReconOnHexOfTargettedIndividual(defnr))
                  this.IList[defnr].IcoverPoints = 0.0f;
              }
              else
                this.IList[defnr].IcoverPoints = 0.0f;
            }
            switch (num60)
            {
              case 1:
                this.IList[defnr].IKilled = this.CombatRound;
                this.IList[defnr].IRetreat = 0;
                this.IList[defnr].IRetreatMode = 0;
                this.AddXp(attnr, (int) Math.Round( this.game.Data.RuleVar[119] *  num56 * ( this.game.Data.UnitObj[this.IList[defnr].IUnr].SupplyConsume / 100.0)));
                s2 = ", Results in KILL.";
                str10 += " : KILL";
                txt = txt + "\r\n" + "Result of attack is a KILL.";
                Individual[] ilist1 = this.IList;
                Individual[] individualArray1 = ilist1;
                let mut index51: i32 =  attnr;
                let mut index52: i32 =  index51;
                individualArray1[index52].ItotalKills = ilist1[index51].ItotalKills + 1;
                Individual[] ilist2 = this.IList;
                Individual[] individualArray2 = ilist2;
                let mut index53: i32 =  attnr;
                let mut index54: i32 =  index53;
                individualArray2[index54].ItotalKillsPowerPoints = ilist2[index53].ItotalKillsPowerPoints + this.game.Data.SFTypeObj[this.IList[defnr].ISFType].PowerPts;
                if (!counterattack)
                {
                  Individual[] ilist3 = this.IList;
                  Individual[] individualArray3 = ilist3;
                  let mut index55: i32 =  attnr;
                  let mut index56: i32 =  index55;
                  individualArray3[index56].IDammageDone = ilist3[index55].IDammageDone + this.game.Data.SFTypeObj[this.IList[defnr].ISFType].PowerPts;
                }
                if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].KillIsRegVar > -1)
                {
                  int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime].RegimeSlot;
                  int[] numArray = regimeSlot;
                  let mut killIsRegVar: i32 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].KillIsRegVar;
                  let mut index57: i32 =  killIsRegVar;
                  let mut num74: i32 =  regimeSlot[killIsRegVar] + 1;
                  numArray[index57] = num74;
                  break;
                }
                break;
              case 3:
                let mut num75: i32 =  1;
                str10 += " : RETREAT";
                if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea & this.CombatType == 2)
                  num75 = 0;
                if (this.IList[defnr].IRetreat > 0)
                  num75 = 0;
                if (num75 == 1)
                {
                  this.IList[defnr].IRetreat = this.CombatRound;
                  this.IList[defnr].IRetreatMode = 1;
                  s2 = ", Results in RETREAT";
                  txt = txt + "\r\n" + "Result of attack is a forced RETREAT.";
                }
                else
                {
                  s2 = ", Results in RETREAT score.. which is handled as mor/rdn loss. Because retreat is not allowed with this type of combat.";
                  txt = txt + "\r\n" + "Result of attack is the penalties of a RETREAT.";
                }
                this.AddXp(attnr, (int) Math.Round( this.game.Data.RuleVar[120] *  num56 * ( this.game.Data.UnitObj[this.IList[defnr].IUnr].SupplyConsume / 100.0)));
                this.ReduceRdn(defnr, (int) Math.Round( this.game.Data.RuleVar[122]));
                this.ReduceMor(defnr, (int) Math.Round( this.game.Data.RuleVar[124]));
                if ( this.game.Data.RuleVar[431] > 0.0 & this.game.Data.Product >= 6)
                  this.ReduceEntr_AdvancedCombatRecon(attnr, defnr, (int) Math.Round( this.game.Data.RuleVar[126]));
                else
                  this.ReduceEntr(defnr, (int) Math.Round( this.game.Data.RuleVar[126]));
                if (!counterattack)
                {
                  Individual[] ilist4 = this.IList;
                  Individual[] individualArray4 = ilist4;
                  let mut index58: i32 =  attnr;
                  let mut index59: i32 =  index58;
                  individualArray4[index59].IDammageDone = ilist4[index58].IDammageDone + this.game.Data.SFTypeObj[this.IList[defnr].ISFType].PowerPts;
                  break;
                }
                break;
              case 4:
                s2 = ", Results in PINNED ";
                str10 += " : PINNED";
                txt = txt + "\r\n" + "Result of attack is a PINNED hit.";
                this.AddXp(attnr, (int) Math.Round( this.game.Data.RuleVar[121] *  num56 * ( this.game.Data.UnitObj[this.IList[defnr].IUnr].SupplyConsume / 100.0)));
                this.ReduceRdn(defnr, (int) Math.Round( this.game.Data.RuleVar[123]));
                if ( this.game.Data.RuleVar[431] > 0.0 & this.game.Data.Product >= 6)
                  this.ReduceEntr_AdvancedCombatRecon(attnr, defnr, (int) Math.Round( this.game.Data.RuleVar[(int) sbyte.MaxValue]));
                else
                  this.ReduceEntr(defnr, (int) Math.Round( this.game.Data.RuleVar[(int) sbyte.MaxValue]));
                this.ReduceMor(defnr, (int) Math.Round( this.game.Data.RuleVar[125]));
                break;
            }
            if (num60 == 1 | num60 == 3 && this.game.Data.UnitObj[this.IList[defnr].IUnr].SupplyConsume >= (int) Math.Round( (VBMath.Rnd() * 100f)) &&  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ChanceOnDeathIfMakeHit > 0.0 && this.IList[attnr].IAttacker == 1 &&  VBMath.Rnd() <  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ChanceOnDeathIfMakeHit)
            {
              this.IList[attnr].IKilled = this.CombatRound;
              this.IList[attnr].IRetreat = 0;
              this.IList[attnr].IRetreatMode = 0;
              str13: String = txt + "\r\n";
              str10 += " : ACCIDENT";
              txt = str13 + "However due to enemy last second enemy counteractions and/or accidents the attacker gets KILLED as well in the combat.";
            }
            if (!counterattack)
              this.ReduceAbsRdn(attnr, this.game.Data.SFTypeObj[this.IList[attnr].ISFType].RdnLossPerAttack);
            if (!counterattack)
              this.IList[attnr].ISuccesfullAttack = this.CombatRound;
            if (this.game.Data.Product == 6)
              this.IList[defnr].ILastHit = this.CombatRound;
            else if (!counterattack)
              this.IList[defnr].ILastHit = this.CombatRound;
            Individual[] ilist5 = this.IList;
            Individual[] individualArray5 = ilist5;
            let mut index60: i32 =  attnr;
            let mut index61: i32 =  index60;
            individualArray5[index61].ItotalHits = ilist5[index60].ItotalHits + 1;
          }
          else
            s2 = ", Extra attack after target is already forced retreating results in MISS.";
        }
        else
        {
          if (!counterattack)
            this.ReduceAbsRdn(attnr, this.game.Data.SFTypeObj[this.IList[attnr].ISFType].RdnLossPerAttack);
          s2 = ", Results in MISS.";
          txt = txt + "\r\n" + "Result of attack is a MISS.";
        }
        if (this.IList[attnr].IAttacker == 1 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AntiStrucPts > 0 & this.IList[attnr].ILastAttackDone < this.CombatRound && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].Regime, this.AttackerRegime))
        {
          if (this.IList[attnr].Istockpile)
          {
            let mut antiStrucPts: i32 =  this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AntiStrucPts;
            let mut num76: i32 =  (int) Math.Round(Conversion.Int( (int) Math.Round(0.33 *  antiStrucPts + 0.66 *  antiStrucPts *  this.game.HandyFunctionsObj.GetAirFieldStackModifier(this.game.Data.UnitObj[this.IList[attnr].IUnr].X, this.game.Data.UnitObj[this.IList[attnr].IUnr].Y)) * ( this.IList[attnr].IRdn / 100.0)));
            if (num76 == 0 &&  VBMath.Rnd() <  this.IList[attnr].IRdn / 100.0)
              num76 = 1;
            let mut Number7: i32 =  (int) Math.Round( (int) Math.Round( VBMath.Rnd() * ( num76 + 0.5)) /  (this.IList[attnr].AttackedCount + 1));
            if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 2 && this.CombatType != 6)
              Number7 = (int) Math.Round( ( Number7 * this.game.Data.RuleVar[128]));
            if (!counterattack)
              this.AntiStrucDam += Number7;
            if (!counterattack & Number7 > 0)
            {
              s2 = s2 + ", +" + Conversion.Str( Number7) + " AntiStruc Dam";
              txt = txt + "\r\n" + "Result of attack is " + Strings.Trim(Conversion.Str( Number7)) + " anti structural damage." + "\r\n" + "Total anti-structural damage now: " + Strings.Trim(Conversion.Str( this.AntiStrucDam));
            }
          }
          else
          {
            s2 = s2 + ", +" + Conversion.Str( 0) + " AntiStruc Dam because no stockpile available.";
            txt = txt + "\r\n" + "Result of attack is " + Strings.Trim(Conversion.Str( 0)) + " anti structural damage because no stockpile available." + "\r\n" + "Total anti-structural damage now: " + Strings.Trim(Conversion.Str( this.AntiStrucDam));
          }
        }
        if (!counterattack)
          this.IList[attnr].ILastAttackDone = this.CombatRound;
        if (!counterattack && !this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench | this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater > 0)
          this.IList[defnr].ILastTargeted = this.CombatRound;
        if (!counterattack)
        {
          Individual[] ilist6 = this.IList;
          Individual[] individualArray6 = ilist6;
          let mut index62: i32 =  attnr;
          let mut index63: i32 =  index62;
          individualArray6[index63].AttackCount = ilist6[index62].AttackCount + 1;
          if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2 &&  num57 > 0.0)
          {
            Individual[] ilist7 = this.IList;
            Individual[] individualArray7 = ilist7;
            let mut index64: i32 =  attnr;
            let mut index65: i32 =  index64;
            individualArray7[index65].AttackCountAir = ilist7[index64].AttackCountAir + 1;
          }
          if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0 &&  num57 > 0.0)
          {
            Individual[] ilist8 = this.IList;
            Individual[] individualArray8 = ilist8;
            let mut index66: i32 =  attnr;
            let mut index67: i32 =  index66;
            individualArray8[index67].AttackCountLand = ilist8[index66].AttackCountLand + 1;
          }
        }
        else
        {
          Individual[] ilist = this.IList;
          Individual[] individualArray = ilist;
          let mut index68: i32 =  attnr;
          let mut index69: i32 =  index68;
          individualArray[index69].ICounterAttacks = ilist[index68].ICounterAttacks + 1;
        }
        Individual[] ilist9 = this.IList;
        Individual[] individualArray9 = ilist9;
        let mut index70: i32 =  defnr;
        let mut index71: i32 =  index70;
        individualArray9[index71].AttackedCount = ilist9[index70].AttackedCount + 1;
        if (!counterattack)
          this.IList[defnr].ILastAttacked = this.CombatRound;
        if (this.customCombatObj.HasCustumCalls() && this.IList[attnr].AttackCount == 1 & !counterattack)
        {
          str14: String = "";
          CustomCombatCalls customCombatObj = this.customCombatObj;
          combatClass = this;
          ref local17: CombatClass = ref combatClass;
          let mut game: GameClass = this.game;
          let mut attnr10: i32 =  attnr;
          let mut defnr11: i32 =  defnr;
          ref local18: String = ref str14;
          if (customCombatObj.IndividualCombatCall_FirstAttackOfRound(ref local17, game, attnr10, defnr11, ref local18) > 0 & str14.Length > 0)
            txt = txt + "\r\n" + str14;
        }
        let mut landscapeType3: i32 =  this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].LandscapeType;
        unitGroup2 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].UnitGroup;
        this.AddDetail(s2);
        if ( this.game.Data.RuleVar[431] < 1.0)
        {
          str15: String = str10;
          title1: String;
          if (counterattack)
            title1 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[attnr].IID)) + ") counter-attacks ==> " + this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[defnr].IID)) + ")" + str15;
          else
            title1 = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[attnr].IID)) + ") attacks ==> " + this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[defnr].IID)) + ")" + str15;
          this.AddReport(1, title1, txt, attnr + 10000, this.CombatRound, matrx);
          title2: String;
          if (counterattack)
            title2 = this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[defnr].IID)) + ") <== is counter-attacked by " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[attnr].IID)) + ")" + str15;
          else
            title2 = this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[defnr].IID)) + ") <== is attacked by " + this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[attnr].IID)) + ")" + str15;
          this.AddReport(1, title2, txt, defnr + 10000, this.CombatRound, matrx);
        }
        else
        {
          str16: String = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[attnr].IID)) + ")";
          str17: String = this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[defnr].IID)) + ")";
          if (this.IList[attnr].IvisibleFromRound > this.CombatRound && this.game.Data.UnitObj[this.IList[attnr].IUnr].Regime != this.game.Data.Turn)
          {
            if (this.game.Data.FOWOn)
            {
              str16 = "Unknown enemy (?)";
              let mut index72: i32 =  0;
              do
              {
                matrx[index72, 0] = "";
                matrx[index72, 1] = "";
                matrx[index72, 2] = "";
                index72 += 1;
              }
              while (index72 <= 60);
              txt = str8 + "\r\n" + str9;
            }
            else
              str16 += " [Hidden]";
          }
          if (this.IList[defnr].IvisibleFromRound > this.CombatRound)
          {
            if (this.game.Data.UnitObj[this.IList[defnr].IUnr].Regime != this.game.Data.Turn)
            {
              if (this.game.Data.FOWOn)
              {
                str17 = "Unknown enemy (?)";
                let mut index73: i32 =  0;
                do
                {
                  matrx[index73, 3] = "";
                  matrx[index73, 4] = "";
                  matrx[index73, 5] = "";
                  index73 += 1;
                }
                while (index73 <= 60);
                txt = str8 + "\r\n" + str9;
              }
              else
                str17 += " [Hidden]";
            }
          }
          else
            txt = txt;
          str18: String = str10;
          this.AddReport(1, !counterattack ? str16 + " attacks ==> " + str17 + str18 : " counter-attacks ==> " + str17 + str18, txt, attnr + 10000, this.CombatRound, matrx);
          this.AddReport(1, !counterattack ? str17 + " <== is attacked by " + str16 + str18 : str17 + " <== is counter-attacked by " + str16 + str18, txt, defnr + 10000, this.CombatRound, matrx);
        }
      }
    }

    pub fn CheckCapitulation(bool afterStepsCheck)
    {
      if (afterStepsCheck && this.game.Data.Product >= 6 && this.CombatType == 3 | this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 9 | this.CombatType == 10 | this.CombatType == 11 | this.CombatType == 12)
      {
        let mut icounter: i32 =  this.ICounter;
        for (let mut index: i32 =  0; index <= icounter; index += 1)
        {
          if (this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0 && this.game.Data.SFTypeObj[this.IList[index].ISFType].EndCombatRound > 0 && this.game.Data.SFTypeObj[this.IList[index].ISFType].EndCombatRound >= this.CombatRound)
          {
            this.IList[index].IRetreat = this.CombatRound;
            this.IList[index].IRetreatMode = 2;
            this.AddReport(0, "Special Retreat", "This trooptype always retreat from this combat round onwards.", index + 10000, this.CombatRound);
          }
        }
      }
      isfType: i32;
      num1: i32;
      num2: i32;
      if (this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 9 | this.CombatType == 10 | this.CombatType == 11 | this.CombatType == 12)
      {
        let mut icounter: i32 =  this.ICounter;
        for (let mut index1: i32 =  0; index1 <= icounter; index1 += 1)
        {
          if (this.IList[index1].IAttacker == 0 & (this.IList[index1].IRetreat > 0 | this.IList[index1].IRetreated > 0) && this.IList[index1].IKilled == 0 & !this.IList[index1].ISurrenderTestDone)
          {
            let mut num3: i32 =  0;
            if (this.IList[index1].IUnr == 124)
              index1 = index1;
            if ( this.IList[index1].IMor <=  this.game.Data.RuleVar[37] &&  this.IList[index1].IRdn <  this.game.Data.RuleVar[301] & this.IList[index1].IRetreat == this.CombatRound)
            {
              this.IList[index1].ISurrenderTestDone = true;
              if ( VBMath.Rnd() >  this.IList[index1].IRdn /  this.game.Data.RuleVar[301])
                num3 = 1;
              else if (!(this.game.Data.Product >= 6 & this.IList[index1].IRetreatMode != 3))
                this.AddReport(0, "Capitulation test", "Morale is lower then " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[37])) + " and readiness is lower then " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[301])) + ". Readiness is " + this.IList[index1].IRdn.ToString() + ". Capitulate test must thus be done. Chance to capitulate is " + Strings.Trim(Conversion.Str( (int) Math.Round(100.0 - 100.0 * ( this.IList[index1].IRdn /  this.game.Data.RuleVar[301])))) + "%. Individual survives test.", index1 + 10000, this.CombatRound);
            }
            if (this.game.Data.Product >= 6 & this.IList[index1].IRetreatMode != 3)
              num3 = 0;
            if (num3 == 1)
            {
              this.AddReport(0, "Capitulation test : CAPITULATION", "Morale is lower then " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[37])) + " and readiness is lower then " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[301])) + ". Readiness is " + this.IList[index1].IRdn.ToString() + " Thus had " + Strings.Trim(Conversion.Str( (int) Math.Round(100.0 - 100.0 * ( this.IList[index1].IRdn /  this.game.Data.RuleVar[301])))) + "% chance to capitulate. Individual capitulates.", index1 + 10000, this.CombatRound);
              this.IList[index1].IKilled = 1;
              this.IList[index1].ICapitulate = true;
              if (this.game.Data.SFTypeObj[this.IList[index1].ISFType].KillIsRegVar > -1)
              {
                int[] regimeSlot = this.game.Data.RegimeObj[this.AttackerRegime].RegimeSlot;
                int[] numArray = regimeSlot;
                let mut killIsRegVar: i32 =  this.game.Data.SFTypeObj[this.IList[index1].ISFType].KillIsRegVar;
                let mut index2: i32 =  killIsRegVar;
                let mut num4: i32 =  regimeSlot[killIsRegVar] + 1;
                numArray[index2] = num4;
              }
              this.AddDetail("A defender capitulates..");
              isfType = this.IList[index1].ISFType;
              if (this.IList[index1].IAttacker == 1)
              {
                num1 = this.AttackerRegime;
                num2 = this.DefenderRegime;
              }
              else
              {
                num1 = this.DefenderRegime;
                num2 = this.AttackerRegime;
              }
            }
          }
        }
      }
      if (this.CombatRound <= 30 || !(this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 9 | this.CombatType == 10 | this.CombatType == 11 | this.CombatType == 12))
        return;
      let mut icounter1: i32 =  this.ICounter;
      for (let mut index3: i32 =  0; index3 <= icounter1; index3 += 1)
      {
        if (this.IList[index3].IAttacker == 1 && this.IList[index3].IKilled == 0)
        {
          this.IList[index3].IKilled = 1;
          this.IList[index3].ICapitulate = true;
          if (this.game.Data.SFTypeObj[this.IList[index3].ISFType].KillIsRegVar > -1)
          {
            int[] regimeSlot = this.game.Data.RegimeObj[this.DefenderRegime].RegimeSlot;
            int[] numArray = regimeSlot;
            let mut killIsRegVar: i32 =  this.game.Data.SFTypeObj[this.IList[index3].ISFType].KillIsRegVar;
            let mut index4: i32 =  killIsRegVar;
            let mut num5: i32 =  regimeSlot[killIsRegVar] + 1;
            numArray[index4] = num5;
          }
          this.AddDetail("An attacker capitulates..");
          isfType = this.IList[index3].ISFType;
          if (this.IList[index3].IAttacker == 1)
          {
            num1 = this.AttackerRegime;
            num2 = this.DefenderRegime;
          }
          else
          {
            num1 = this.DefenderRegime;
            num2 = this.AttackerRegime;
          }
        }
      }
    }

    pub fn FindISlot(iId: i32) -> i32
    {
      let mut icounter: i32 =  this.ICounter;
      for (let mut islot: i32 =  0; islot <= icounter; islot += 1)
      {
        if (this.IList[islot].IID == iId)
          return islot;
      }
      return -1;
    }

    pub fn FindUSlot(unr: i32) -> i32
    {
      let mut ucounter: i32 =  this.UCounter;
      for (let mut uslot: i32 =  0; uslot <= ucounter; uslot += 1)
      {
        if (this.UList[uslot].UNr == unr)
          return uslot;
      }
      return -1;
    }

    pub fn FindPreventer(defnr: i32, attnr: i32) -> i32
    {
      if (defnr == -1)
        return -1;
      let mut isfType1: i32 =  this.IList[defnr].ISFType;
      let mut unitGroup1: i32 =  this.game.Data.SFTypeObj[isfType1].UnitGroup;
      let mut isfType2: i32 =  this.IList[attnr].ISFType;
      let mut unitGroup2: i32 =  this.game.Data.SFTypeObj[isfType2].UnitGroup;
      let mut iattacker: i32 =  this.IList[defnr].IAttacker;
      let mut preventer: i32 =  -1;
      let mut num1: i32 =  -1;
      let mut index1: i32 =  -1;
      if (this.game.Data.Product != 7 && this.game.Data.SFTypeObj[isfType2].ArtRange > 0 ||  this.game.Data.RuleVar[493] > 0.0 && this.IList[defnr].IleftOutOfPartialAttack)
        return defnr;
      let mut num2: i32 =  (int) Math.Round( Conversion.Int(VBMath.Rnd() *  this.ICounter)) + 1;
      let mut index2: i32 =  num2;
      let mut num3: i32 =  0;
      while (num3 != 1)
      {
        index2 += 1;
        if (index2 > this.ICounter)
          index2 = 0;
        if (index2 == num2)
          num3 = 1;
        let mut isfType3: i32 =  this.IList[index2].ISFType;
        if (this.IList[index2].IAttacker == iattacker & (this.IList[index2].IRetreatMode == 0 | this.IList[defnr].IRetreatMode > 0 & this.IList[index2].IRetreated == 0) & this.IList[index2].IKilled == 0)
        {
          let mut isfType4: i32 =  this.IList[index2].ISFType;
          let mut unitGroup3: i32 =  this.game.Data.SFTypeObj[isfType4].UnitGroup;
          if (this.game.Data.UnitObj[this.IList[index2].IUnr].X == this.game.Data.UnitObj[this.IList[defnr].IUnr].X & this.game.Data.UnitObj[this.IList[index2].IUnr].Y == this.game.Data.UnitObj[this.IList[defnr].IUnr].Y | this.game.Data.SFTypeObj[this.IList[index2].ISFType].Theater >= 2 && this.IList[index2].AttackedCount < this.game.Data.SFTypeObj[isfType4].MaxAttacked | this.game.Data.SFTypeObj[isfType4].MaxPreventPointsUsed >= 9999)
          {
            let mut preventCounter: i32 =  this.game.Data.SFTypeObj[isfType4].PreventCounter;
            for (let mut index3: i32 =  0; index3 <= preventCounter; index3 += 1)
            {
              if (this.game.Data.SFTypeObj[isfType4].PreventHitOn[index3] == unitGroup1 | this.game.Data.SFTypeObj[isfType4].PreventHitOn[index3] == -1 && this.game.Data.SFTypeObj[isfType4].MaxPreventPointsUsed >= 9999 | 0 < this.game.Data.SFTypeObj[isfType1].MaxPreventPointsGiven - this.IList[defnr].IPreventPointsGiven && 0 < this.game.Data.SFTypeObj[isfType4].MaxPreventPointsUsed - this.IList[index2].IPreventPointsUsed && this.game.Data.SFTypeObj[isfType4].PreventHitFrom[index3] == unitGroup2 | this.game.Data.SFTypeObj[isfType4].PreventHitFrom[index3] == -1)
              {
                let mut num4: i32 =  (int) Math.Round( (100f * VBMath.Rnd()));
                if (this.game.Data.SFTypeObj[isfType4].PreventPriority[index3] * 1000 + num4 > num1)
                {
                  preventer = index2;
                  num1 = num4 + this.game.Data.SFTypeObj[isfType4].PreventPriority[index3] * 1000;
                  index1 = index3;
                }
              }
            }
          }
        }
      }
      if (preventer > -1)
      {
        if (preventer > -1 & this.game.Data.SFTypeObj[this.IList[preventer].ISFType].PreventChance[index1] > 0)
        {
          Individual[] ilist1 = this.IList;
          Individual[] individualArray1 = ilist1;
          let mut index4: i32 =  defnr;
          let mut index5: i32 =  index4;
          individualArray1[index5].IPreventPointsGiven = ilist1[index4].IPreventPointsGiven + this.game.Data.SFTypeObj[this.IList[preventer].ISFType].PreventPoints[index1];
          Individual[] ilist2 = this.IList;
          Individual[] individualArray2 = ilist2;
          let mut index6: i32 =  preventer;
          let mut index7: i32 =  index6;
          individualArray2[index7].IPreventPointsUsed = ilist2[index6].IPreventPointsUsed + this.game.Data.SFTypeObj[this.IList[preventer].ISFType].PreventPoints[index1];
          let mut Number1: i32 =  (int) Math.Round( (VBMath.Rnd() * 100f));
          let mut Number2: i32 =  this.game.Data.SFTypeObj[this.IList[preventer].ISFType].PreventChance[index1];
          str1: String = this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[defnr].IID)) + ")";
          str2: String = this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[attnr].IID)) + ")";
          str3: String = this.game.Data.SFTypeObj[this.IList[preventer].ISFType].Name + "(" + Strings.Trim(Conversion.Str( this.IList[preventer].IID)) + ")";
          str4: String = "Roll required to be lower or equal then " + Conversion.Str( Number2) + ". roll was " + Conversion.Str( Number1) + ". PreventPointsGiven to " + str1 + ": " + Conversion.Str( this.IList[defnr].IPreventPointsGiven) + ". PreventPointsUsed for " + str3 + ": " + Conversion.Str( this.IList[preventer].IPreventPointsUsed) + ". ";
          if (Number1 < Number2)
          {
            this.AddDetail("Prevent SUCCES" + this.game.Data.SFTypeObj[this.IList[preventer].ISFType].Name);
            this.AddReport(0, "Prevent Received SUCCES", "Individual " + str4 + str3 + " prevents hit on " + str1 + " by " + str2, defnr + 10000, this.CombatRound);
            this.AddReport(0, "Prevent Given SUCCES", "Individual " + str4 + str3 + " prevents a hit on " + str1 + " by " + str2, preventer + 10000, this.CombatRound);
            this.AddReport(0, "Target avoids hit by prevent SUCCES", "Individual " + str4 + str1 + " is prevented from being hit by " + str3, attnr + 10000, this.CombatRound);
          }
          else
          {
            this.AddDetail("Hit was almost prevented by " + this.game.Data.SFTypeObj[this.IList[preventer].ISFType].Name);
            this.AddReport(0, "Prevent Received FAILED", "Individual " + str4 + str3 + " tried to prevent hit on " + str1 + " by " + str2, defnr + 10000, this.CombatRound);
            this.AddReport(0, "Prevent Given FAILED", "Individual " + str4 + str3 + " tried to prevent a hit on " + str1 + " by " + str2 + ", but failed to do succesfully do so.", preventer + 10000, this.CombatRound);
            this.AddReport(0, "Target avoids hit by prevent FAILED ", "Individual " + str4 + str1 + " was attempted to be prevented from being hit by " + str3, attnr + 10000, this.CombatRound);
            preventer = defnr;
          }
        }
        else
          preventer = defnr;
      }
      else
        preventer = defnr;
      return preventer;
    }

    pub fn DoSteps()
    {
      if (this.previewMode)
        VBMath.Randomize( (int) Math.Round( (this.game.Data.GameID * this.TargetX) /  (this.TargetY + 1)));
      let mut icounter1: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        this.IList[index].AttackCount = 0;
        this.IList[index].AttackCountAir = 0;
        this.IList[index].AttackCountLand = 0;
        this.IList[index].AttackedCount = 0;
        this.IList[index].ICounterAttacks = 0;
        this.IList[index].IPreventPointsGiven = 0;
        this.IList[index].IPreventPointsUsed = 0;
      }
      let mut ucounter1: i32 =  this.UCounter;
      for (let mut index: i32 =  0; index <= ucounter1; index += 1)
        this.UList[index].UParticipated = 0;
      this.CrowdingAttMod = 1f;
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      if (this.CombatType == 12 | this.CombatType == 1 | this.CombatType == 10 | this.CombatType == 9)
      {
        let mut icounter2: i32 =  this.ICounter;
        for (let mut inr: i32 =  0; inr <= icounter2; inr += 1)
        {
          if (this.TestAttack(inr) && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 0 && this.IList[inr].IAttacker == 1)
          {
            num1 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
            if (this.UList[this.IList[inr].IUlistNr].UApMoveCost < this.UList[this.IList[inr].IUlistNr].UApSpend)
              num2 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
          }
        }
        if (num2 > 0 & !this.previewMode)
        {
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut index1: i32 =  0; index1 <= regimeCounter; index1 += 1)
          {
            if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.AttackerRegime, index1))
            {
              HexClass[,] hexObj = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj;
              HexClass[,] hexClassArray = hexObj;
              let mut targetX: i32 =  this.TargetX;
              let mut index2: i32 =  targetX;
              let mut targetY: i32 =  this.TargetY;
              let mut index3: i32 =  targetY;
              HexClass hexClass = hexClassArray[index2, index3];
              let mut Index1: i32 =  index1;
              let mut Index2: i32 =  Index1;
              let mut num3: i32 =  (int) Math.Round( hexObj[targetX, targetY].get_BattlePenalty(Index1) + Math.Max(1.0,  this.game.Data.RuleVar[325] * Math.Min( num2 / 100.0, 1.0)));
              hexClass.set_BattlePenalty(Index2, num3);
              if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].get_BattlePenalty(index1) > 10.0 *  this.game.Data.RuleVar[325])
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].set_BattlePenalty(index1, (int) Math.Round( (10f * this.game.Data.RuleVar[325])));
            }
          }
        }
        if (num1 == 0)
          num1 = 1;
        if (this.CombatRound == 1)
          this.NewBattleStack = num1;
        this.CrowdingAttMod = this.AttackCrowding /  (num1 + this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].get_BattleStack(this.AttackerRegime));
        if ( this.CrowdingAttMod > 1.0)
          this.CrowdingAttMod = 1f;
      }
      this.CrowdingAttArtMod = 1f;
      let mut num4: i32 =  0;
      let mut num5: i32 =  0;
      if ((this.CombatType == 3 | this.CombatType == 4) &  this.AttackCrowdingArt > 0.0)
      {
        let mut icounter3: i32 =  this.ICounter;
        for (let mut inr: i32 =  0; inr <= icounter3; inr += 1)
        {
          if (this.TestAttack(inr) && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater < 2 && this.IList[inr].IAttacker == 1)
          {
            num4 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
            if (this.UList[this.IList[inr].IUlistNr].UApMoveCost < this.UList[this.IList[inr].IUlistNr].UApSpend)
              num5 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
          }
        }
        if (num4 == 0)
          num4 = 1;
        if (this.CombatRound == 1)
          this.NewBattleStackArt = num4;
        this.CrowdingAttArtMod = this.AttackCrowdingArt /  (num4 + this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].get_BattleStackArt(this.AttackerRegime));
        if ( this.CrowdingAttArtMod > 1.0)
          this.CrowdingAttArtMod = 1f;
      }
      this.CrowdingAttAirMod = 1f;
      let mut num6: i32 =  0;
      let mut num7: i32 =  0;
      if ( this.AttackCrowdingAir > 0.0 && this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 9 | this.CombatType == 13)
      {
        let mut icounter4: i32 =  this.ICounter;
        for (let mut inr: i32 =  0; inr <= icounter4; inr += 1)
        {
          if (this.TestAttack(inr) && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 2 && this.IList[inr].IAttacker == 1)
          {
            num6 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
            if (this.UList[this.IList[inr].IUlistNr].UApMoveCost < this.UList[this.IList[inr].IUlistNr].UApSpend)
              num7 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
          }
        }
        if (this.CombatRound == 1)
          this.NewBattleStackAir = num6;
        if (num6 == 0)
          num6 = 1;
        this.CrowdingAttAirMod = this.AttackCrowdingAir /  (num6 + this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].get_BattleStackAir(this.AttackerRegime));
        if ( this.CrowdingAttAirMod > 1.0)
          this.CrowdingAttAirMod = 1f;
      }
      this.CrowdingDefMod = 1f;
      let mut num8: i32 =  0;
      if (this.CombatType == 12 | this.CombatType == 1 | this.CombatType == 10 | this.CombatType == 9 | this.CombatType == 3 | this.CombatType == 4 | this.CombatType == 5 | this.CombatType == 6)
      {
        let mut icounter5: i32 =  this.ICounter;
        for (let mut inr: i32 =  0; inr <= icounter5; inr += 1)
        {
          if (this.TestAttack(inr) & this.IList[inr].IAA < 1 && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 0 && this.IList[inr].IAttacker == 0)
            num8 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].Frontage;
        }
        this.CrowdingDefCount = num8;
        if (num8 == 0)
          num8 = 1;
        this.CrowdingDefMod = this.game.Data.RuleVar[30] /  num8;
        if ( this.CrowdingDefMod > 1.0)
          this.CrowdingDefMod = 1f;
      }
      let mut num9: i32 =  -1;
      let mut num10: i32 =  1;
      while (num10 > 0)
      {
        num10 = 0;
        num9 += 1;
        this.AddDetail("Attack SubRound = " + Conversion.Str( (num9 + 1)));
        let mut icounter6: i32 =  this.ICounter;
        for (let mut index4: i32 =  0; index4 <= icounter6; index4 += 1)
        {
          let mut num11: i32 =  1;
          let mut Number1: i32 =  0;
          if (this.IList[index4].IAttacker == 0)
            Number1 = Number1;
          while (num11 == 1)
          {
            num11 = this.IList[index4].AttackCount >= this.game.Data.SFTypeObj[this.IList[index4].ISFType].Attacks ? 0 : 1;
            if (this.IList[index4].AttackCount != num9)
              num11 = 0;
            bool flag1 = false;
            if (this.game.Data.SFTypeObj[this.IList[index4].ISFType].Attacks == 0)
              flag1 = true;
            if (this.game.Data.Product >= 6)
            {
              if (this.game.Data.SFTypeObj[this.IList[index4].ISFType].StartCombatRound > 0 && this.CombatRound < this.game.Data.SFTypeObj[this.IList[index4].ISFType].StartCombatRound)
                num11 = 0;
              if (this.game.Data.SFTypeObj[this.IList[index4].ISFType].EndCombatRound > 0 && this.game.Data.SFTypeObj[this.IList[index4].ISFType].EndCombatRound == this.game.Data.SFTypeObj[this.IList[index4].ISFType].StartCombatRound | this.game.Data.SFTypeObj[this.IList[index4].ISFType].EndCombatRound == 1 && this.IList[index4].IAttacker == 0)
                num11 = 0;
            }
            if (this.game.Data.Product >= 7 &&  this.game.Data.RuleVar[407] > 0.0 && this.game.Data.SFTypeObj[this.IList[index4].ISFType].ArtRange > 0 &&  this.IList[index4].ILisAmmoMod < 1.0)
              num11 = 0;
            if (num11 == 1)
            {
              if (this.TestAttack(index4))
              {
                let mut combatType: i32 =  this.CombatType;
                if ( this.game.Data.RuleVar[493] > 0.0 && this.IList[index4].IleftOutOfPartialAttack)
                  this.CombatType = 3;
                let mut index5: i32 =  this.FindPreventer(this.FindOpponent(index4), index4);
                if (index5 > -1)
                {
                  if (this.IList[index5].IleftOutOfPartialAttack)
                    index5 = index5;
                  Number1 += 1;
                  this.DoActualAttack(index4, index5);
                  bool flag2 = false;
                  if ( this.game.Data.RuleVar[431] > 0.0 &&  this.GetEffectiveReconOnHexOfTargettedIndividual(index4) <  this.IList[index4].IcoverPoints)
                    flag2 = true;
                  if (this.game.Data.Product >= 7 &&  this.game.Data.RuleVar[407] > 0.0 && this.game.Data.SFTypeObj[this.IList[index5].ISFType].ArtRange > 0 &&  this.IList[index5].ILisAmmoMod < 1.0)
                    flag2 = true;
                  if (!flag2 & this.IList[index5].IKilled == 0 &  this.game.Data.RuleVar[807] == 0.0 && this.TestTarget(index5, index4))
                  {
                    let mut num12: i32 =  1;
                    if (this.CombatType == 3 | this.CombatType == 4)
                    {
                      num12 = 0;
                      if ( this.game.Data.RuleVar[493] > 0.0 & num12 == 0 && this.IList[index4].IleftOutOfPartialAttack)
                        num12 = 1;
                    }
                    if (this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15 && this.game.Data.SFTypeObj[this.IList[index4].ISFType].Theater < 2)
                      num12 = 0;
                    if (this.IList[index4].AttackedCount >= this.game.Data.SFTypeObj[this.IList[index4].ISFType].MaxAttacked)
                      num12 = 0;
                    let mut unitGroup: i32 =  this.game.Data.SFTypeObj[this.IList[index4].ISFType].UnitGroup;
                    if (this.game.Data.SFTypeObj[this.IList[index5].ISFType].AttackPower[unitGroup] < 1)
                      num12 = 0;
                    if (this.IList[index5].IRetreat > 0)
                      num12 = 0;
                    if (this.IList[index5].AttackedCount > this.game.Data.SFTypeObj[this.IList[index5].ISFType].MaxAttacked &  this.game.Data.RuleVar[925] < 1.0)
                      num12 = 0;
                    if (num12 == 1)
                      this.DoActualAttack(index5, index4, true);
                  }
                  num11 = 1;
                  num10 = 1;
                  if (Number1 == 1)
                  {
                    Individual[] ilist = this.IList;
                    Individual[] individualArray = ilist;
                    let mut index6: i32 =  index4;
                    let mut index7: i32 =  index6;
                    individualArray[index7].IBattleRounds = ilist[index6].IBattleRounds + 1;
                  }
                }
                else
                {
                  flag1 = true;
                  num11 = 0;
                }
                if ( this.game.Data.RuleVar[493] > 0.0 && this.IList[index4].IleftOutOfPartialAttack)
                  this.CombatType = combatType;
              }
              else
                num11 = 0;
            }
            if (flag1 && this.game.Data.SFTypeObj[this.IList[index4].ISFType].AntiStrucPts > 0 && this.IList[index4].IAttacker == 1 && this.IList[index4].ILastAttackDone < this.CombatRound && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].Regime, this.AttackerRegime))
            {
              this.IList[index4].ILastAttackDone = this.CombatRound;
              Individual[] ilist = this.IList;
              Individual[] individualArray = ilist;
              let mut index8: i32 =  index4;
              let mut index9: i32 =  index8;
              individualArray[index9].AttackCount = ilist[index8].AttackCount + 1;
              let mut num13: i32 =  (int) Math.Round(Conversion.Int( this.game.Data.SFTypeObj[this.IList[index4].ISFType].AntiStrucPts * ( this.IList[index4].IRdn / 100.0)));
              let mut num14: i32 =  (int) Math.Round(0.33 *  num13 + 0.66 *  num13 *  this.game.HandyFunctionsObj.GetAirFieldStackModifier(this.game.Data.UnitObj[this.IList[index4].IUnr].X, this.game.Data.UnitObj[this.IList[index4].IUnr].Y));
              if (num14 == 0 &&  VBMath.Rnd() <  this.IList[index4].IRdn / 100.0)
                num14 = 1;
              let mut Number2: i32 =  (int) Math.Round( (int) Math.Round( VBMath.Rnd() * ( num14 + 0.5)) /  (this.IList[index4].AttackedCount + 1));
              if (this.game.Data.SFTypeObj[this.IList[index4].ISFType].Theater == 2 && this.CombatType != 6)
                Number2 = (int) Math.Round( ( Number2 * this.game.Data.RuleVar[128]));
              if (this.game.Data.SFTypeObj[this.IList[index4].ISFType].SlotNumber > -1 && this.IList[index4].IAttacker == 1)
              {
                int[] areaCode = this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].AreaCode;
                int[] numArray = areaCode;
                let mut slotNumber: i32 =  this.game.Data.SFTypeObj[this.IList[index4].ISFType].SlotNumber;
                let mut index10: i32 =  slotNumber;
                let mut num15: i32 =  areaCode[slotNumber] + 1;
                numArray[index10] = num15;
              }
              if ( this.game.Data.RuleVar[435] > 0.0)
              {
                if (((!this.game.EditObj.CombatSim ? 1 : 0) & 0) != 0)
                {
                  Number1 = this.game.Data.SFTypeObj[this.IList[index4].ISFType].FuelForAttack;
                  let mut index11: i32 =  (int) Math.Round( this.game.Data.RuleVar[435]);
                  let mut attackerRegime: i32 =  this.AttackerRegime;
                  if (Number1 > 0)
                  {
                    if (this.game.Data.UnitObj[this.IList[index4].IUnr].Fuel >= Number1)
                    {
                      UnitClass[] unitObj1 = this.game.Data.UnitObj;
                      UnitClass[] unitClassArray1 = unitObj1;
                      let mut iunr1: i32 =  this.IList[index4].IUnr;
                      let mut index12: i32 =  iunr1;
                      unitClassArray1[index12].FuelUsedAtt = unitObj1[iunr1].FuelUsedAtt + Number1;
                      UnitClass[] unitObj2 = this.game.Data.UnitObj;
                      UnitClass[] unitClassArray2 = unitObj2;
                      let mut iunr2: i32 =  this.IList[index4].IUnr;
                      let mut index13: i32 =  iunr2;
                      unitClassArray2[index13].Fuel = unitObj2[iunr2].Fuel - Number1;
                      this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name + " consumes " + Conversion.Str( Number1) + " " + this.game.Data.RegimeSlotName[index11]);
                      this.AddReport(0, "Fuel consumption", "Individual " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name, index4 + 10000, this.CombatRound);
                    }
                    else
                    {
                      this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name + " out of fuel!!!");
                      this.AddReport(0, "Out of fuel", "Individual " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name, index4 + 10000, this.CombatRound);
                      Number2 = (int) Math.Round( ( Number2 * this.game.Data.SFTypeObj[this.IList[index4].ISFType].OutOfFuelAttack));
                    }
                  }
                }
              }
              else if (!this.game.EditObj.CombatSim)
              {
                let mut fuelForAttack: i32 =  this.game.Data.SFTypeObj[this.IList[index4].ISFType].FuelForAttack;
                let mut currentSlot: i32 =  this.game.Data.SFTypeObj[this.IList[index4].ISFType].FuelRegimeVar;
                if ( this.game.Data.RuleVar[949] > 0.0)
                  currentSlot = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot, this.game.Data.UnitObj[this.IList[index4].IUnr].RealX(ref this.game), this.game.Data.UnitObj[this.IList[index4].IUnr].RealY(ref this.game));
                let mut index14: i32 =  currentSlot;
                let mut attackerRegime: i32 =  this.AttackerRegime;
                if (fuelForAttack > 0 & index14 > -1)
                {
                  if (this.game.Data.RegimeObj[attackerRegime].RegimeSlot[index14] >= fuelForAttack)
                  {
                    UnitClass[] unitObj = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray = unitObj;
                    let mut iunr: i32 =  this.IList[index4].IUnr;
                    let mut index15: i32 =  iunr;
                    unitClassArray[index15].FuelUsedAtt = unitObj[iunr].FuelUsedAtt + fuelForAttack;
                    int[] regimeSlot = this.game.Data.RegimeObj[attackerRegime].RegimeSlot;
                    int[] numArray = regimeSlot;
                    let mut index16: i32 =  index14;
                    let mut index17: i32 =  index16;
                    let mut num16: i32 =  regimeSlot[index16] - fuelForAttack;
                    numArray[index17] = num16;
                    this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name + " consumes " + Conversion.Str( fuelForAttack) + " " + this.game.Data.RegimeSlotName[index14]);
                    this.AddReport(0, "Fuel consumption", "Individual " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name, index4 + 10000, this.CombatRound);
                  }
                  else
                  {
                    this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name + " out of fuel!!!");
                    this.AddReport(0, "Out of fuel", "Individual " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name, index4 + 10000, this.CombatRound);
                    Number2 = (int) Math.Round( ( Number2 * this.game.Data.SFTypeObj[this.IList[index4].ISFType].OutOfFuelAttack));
                  }
                }
                Number1 = this.IList[index4].ISFType;
                if (this.game.Data.SFTypeObj[Number1].StockpileUsedPerRound > 0)
                {
                  if (this.game.Data.UnitObj[this.IList[index4].IUnr].StockpileCurrent >= this.game.Data.SFTypeObj[Number1].StockpileUsedPerRound)
                  {
                    this.IList[index4].Istockpile = true;
                    UnitClass[] unitObj = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray = unitObj;
                    let mut iunr: i32 =  this.IList[index4].IUnr;
                    let mut index18: i32 =  iunr;
                    unitClassArray[index18].StockpileCurrent = unitObj[iunr].StockpileCurrent - this.game.Data.SFTypeObj[Number1].StockpileUsedPerRound;
                  }
                  else
                    this.IList[index4].Istockpile = false;
                }
                else
                  this.IList[index4].Istockpile = true;
                if (!this.IList[index4].Istockpile)
                {
                  this.AddDetail("Attacker: " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name + " out of stockpile!!!");
                  this.AddReport(0, "Out of stockpile", "Individual " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name, index4 + 10000, this.CombatRound);
                  Number2 = 0;
                }
              }
              if (this.game.Data.Product >= 7 &&  this.IList[index4].ILisAmmoMod < 1.0)
                Number2 = (int) Math.Round( ( Number2 * this.IList[index4].ILisAmmoMod));
              this.AntiStrucDam += Number2;
              this.AddReport(0, "Anti-structural damage", "Individual " + this.game.Data.SFTypeObj[this.IList[index4].ISFType].Name + " that has no opponent does " + Conversion.Str( Number2) + " damage anyway.\r\nTotal anti-structural damage now: " + Strings.Trim(Conversion.Str( this.AntiStrucDam)), index4 + 10000, this.CombatRound);
              if (this.customCombatObj.HasCustumCalls())
              {
                str: String = "";
                CustomCombatCalls customCombatObj = this.customCombatObj;
                combatClass: CombatClass = this;
                ref local1: CombatClass = ref combatClass;
                let mut game: GameClass = this.game;
                let mut attnr: i32 =  index4;
                ref local2: String = ref str;
                customCombatObj.IndividualCombatCall_FirstAttackOfRound(ref local1, game, attnr, -1, ref local2);
              }
            }
          }
        }
      }
      let mut ucounter2: i32 =  this.UCounter;
      for (let mut index: i32 =  0; index <= ucounter2; index += 1)
        this.UList[index].UParticipated = 1;
    }

    pub void AddUnitToUnits(
      tunr: i32,
      tattacker: i32,
      Coordinate tretreat,
      Coordinate tfrom,
      Coordinate ttoo,
      bool IsInterceptor = false,
      bool IsParadropper = false,
      bool isAA = false,
      bool isSupportInterceptFire = false)
    {
      this += 1.UCounter;
      this.UList = (BattleUnit[]) Utils.CopyArray((Array) this.UList, (Array) new BattleUnit[this.UCounter + 1]);
      this.UList[this.UCounter].UNr = tunr;
      if (this.game.Data.Product == 6)
      {
        Coordinate coordinate = this.game.HandyFunctionsObj.EPandPowerInHex(this.game.Data.UnitObj[tunr].X, this.game.Data.UnitObj[tunr].Y, tunr);
        coordinate.x = (int) Math.Round( ( coordinate.x * this.game.Data.RuleVar[42]));
        if (coordinate.x > 0 & coordinate.data2 > 0)
          this.UList[this.UCounter].UepEpMaxMod =  (int) Math.Round( (100 * coordinate.data2) /  coordinate.x);
      }
      if (this.game.Data.UncertaintyOn)
        this.UList[this.UCounter].UDice = this.game.HandyFunctionsObj.Uncertainty_RollLoadedDice(tunr);
      this.UList[this.UCounter].Uattacker = tattacker;
      this.UList[this.UCounter].URetreat = 0;
      this.UList[this.UCounter].UPanicked = false;
      this.UList[this.UCounter].UCanRetreat.x = tretreat.x;
      this.UList[this.UCounter].UCanRetreat.y = tretreat.y;
      this.UList[this.UCounter].UCanRetreat.map = tretreat.map;
      this.UList[this.UCounter].UCanRetreat.onmap = tretreat.onmap;
      this.UList[this.UCounter].UParadropper = false;
      this.UList[this.UCounter].UDefIntercept = IsInterceptor;
      this.UList[this.UCounter].UinitialRdn = this.game.HandyFunctionsObj.GetAverageRdn(tunr);
      this.game.Data.UnitObj[tunr].DidAttack = true;
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            this.game.EditObj.TempValue[index1].Value[index2, index3] = this.TempValue3[index1].Value[index2, index3];
        }
      }
      if (this.game.Data.UnitObj[tunr].IsHQ)
      {
        this.UList[this.UCounter].UStaffMod = 1f;
        this.UList[this.UCounter].UStaffXpMod = 0.0f;
      }
      else if (this.game.Data.UnitObj[tunr].HQ > -1)
      {
        let mut num1: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(this.game.Data.UnitObj[tunr].HQ);
        let mut num2: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(this.game.Data.UnitObj[tunr].HQ, true);
        let mut num3: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(this.game.Data.UnitObj[tunr].HQ);
        if (num1 > 100)
          num1 = 100;
        if (num3 > 100)
          num3 = 100;
        if (num2 > 100)
          num2 = 100;
        if (this.game.Data.UnitObj[tunr].IsHQ)
        {
          this.UList[this.UCounter].UStaffMod =  (1.0 +  this.game.HandyFunctionsObj.GetStaffCombatMod(tunr) * ( num2 / 100.0));
          this.UList[this.UCounter].UStaffXpMod = this.game.Data.RuleVar[75] * ( num1 / 100f);
        }
        else if (this.game.Data.UnitObj[tunr].HQ > -1)
        {
          this.UList[this.UCounter].UStaffMod =  (1.0 +  this.game.Data.RuleVar[140] * ( num3 / 100.0) * ( this.game.HandyFunctionsObj.Gethqpow(tunr) / 100.0) +  this.game.HandyFunctionsObj.GetStaffCombatMod(this.game.Data.UnitObj[tunr].HQ) * ( num2 / 100.0) * ( this.game.HandyFunctionsObj.Gethqpow(tunr) / 100.0));
          this.UList[this.UCounter].UStaffXpMod =  ( this.game.Data.RuleVar[75] * ( num1 / 100.0) * ( this.game.HandyFunctionsObj.Gethqpow(tunr) / 100.0));
        }
        else
        {
          this.UList[this.UCounter].UStaffMod = 1f;
          this.UList[this.UCounter].UStaffXpMod = 0.0f;
        }
      }
      else
      {
        this.UList[this.UCounter].UStaffMod = 1f;
        this.UList[this.UCounter].UStaffXpMod = 0.0f;
      }
      if (tattacker == 1)
      {
        let mut dist: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[this.UCounter].UNr].X, this.game.Data.UnitObj[this.UList[this.UCounter].UNr].Y, this.game.Data.UnitObj[this.UList[this.UCounter].UNr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map);
        if (this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 11)
        {
          this.UList[this.UCounter].UApMoveCost = this.game.HandyFunctionsObj.MoveApCostPreview(tunr, tfrom.x, tfrom.y, tfrom.x, tfrom.y, tfrom.map, ttoo.x, ttoo.y, ttoo.map, true).x;
          this.UList[this.UCounter].UMaxApToSpend = this.game.HandyFunctionsObj.GetLowestAp(tunr);
        }
        else if (this.CombatType == 3)
          this.UList[this.UCounter].UMaxApToSpend = !this.InterceptFire ? ( this.game.Data.RuleVar[419] <= 0.0 ? this.game.HandyFunctionsObj.GetLowestlandartAp(tunr, dist) : Math.Min(this.game.HandyFunctionsObj.GetLowestlandartAp(tunr, dist), this.game.HandyFunctionsObj.GetLowestlandDirectAp(tunr, dist))) : (int) Math.Round( (10f * this.game.Data.RuleVar[428]));
        else if (this.CombatType == 4)
          this.UList[this.UCounter].UMaxApToSpend = this.game.HandyFunctionsObj.GetLowestseaartAp(tunr, dist);
        else if (this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15)
        {
          bool flag = false;
          if (this.CombatType == 15 & this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Regime == this.AttackerRegime & this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Location > -1)
          {
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Location].Type].IsAirfield)
              flag = true;
            if (this.game.Data.LocObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Location].isAirfield)
              flag = true;
          }
          if (flag)
            this.game.HandyFunctionsObj.MakeMovePrediction(tunr, this.game.Data.UnitObj[tunr].X, this.game.Data.UnitObj[tunr].Y, this.game.Data.UnitObj[tunr].Map, false, PredictAirOnly: true, IsTransfer: true);
          else
            this.game.HandyFunctionsObj.MakeMovePrediction(tunr, this.game.Data.UnitObj[tunr].X, this.game.Data.UnitObj[tunr].Y, this.game.Data.UnitObj[tunr].Map, false, PredictAirOnly: true, attack: true);
          this.UList[this.UCounter].UApMoveCost = this.game.EditObj.TempValue[this.CombatTarget.map].Value[this.CombatTarget.x, this.CombatTarget.y];
          if (this.CombatType == 14 && this.UList[this.UCounter].UApMoveCost < 40)
            this.UList[this.UCounter].UApMoveCost = 40;
          this.UList[this.UCounter].UMaxApToSpend = this.game.HandyFunctionsObj.GetLowestAirAp(tunr);
        }
        else if (this.CombatType == 9 & IsParadropper)
        {
          this.UList[this.UCounter].UParadropper = true;
          this.UList[this.UCounter].UApMoveCost = this.game.HandyFunctionsObj.GetLowestAp(this.UList[this.UCounter].UNr);
          this.UList[this.UCounter].UMaxApToSpend = 9999;
        }
        else if (this.CombatType == 9 & !IsParadropper)
        {
          this.UList[this.UCounter].UParadropper = false;
          this.game.HandyFunctionsObj.MakeMovePrediction(tunr, this.game.Data.UnitObj[tunr].X, this.game.Data.UnitObj[tunr].Y, this.game.Data.UnitObj[tunr].Map, false, attack: true);
          this.UList[this.UCounter].UApMoveCost = this.game.EditObj.TempValue[this.CombatTarget.map].Value[this.CombatTarget.x, this.CombatTarget.y];
          this.UList[this.UCounter].UMaxApToSpend = this.game.HandyFunctionsObj.GetLowestAirAp(tunr);
        }
        else if (this.CombatType == 10)
        {
          this.UList[this.UCounter].UApMoveCost = this.game.HandyFunctionsObj.GetLowestAp(this.UList[this.UCounter].UNr);
          this.UList[this.UCounter].UMaxApToSpend = 9999;
        }
        else if (this.CombatType == 12)
        {
          this.UList[this.UCounter].UApMoveCost = this.game.HandyFunctionsObj.GetLowestAp(this.UList[this.UCounter].UNr);
          this.UList[this.UCounter].UMaxApToSpend = 9999;
        }
        this.UList[this.UCounter].UApSpend = 0;
      }
      else
      {
        this.UList[this.UCounter].UApMoveCost = 0;
        this.UList[this.UCounter].UMaxApToSpend = 9999;
        this.UList[this.UCounter].UApSpend = 0;
        if (IsInterceptor)
        {
          this.UList[this.UCounter].UDefIntercept = true;
          this.AddBiggy("Interceptors come to the rescue of the defenders: " + this.game.Data.UnitObj[tunr].Name);
        }
        if (isSupportInterceptFire)
        {
          this.UList[this.UCounter].UMaxApToSpend = (int) Math.Round( (10f * this.game.Data.RuleVar[428]));
          this.UList[this.UCounter].USupportInterceptFire = true;
          this.AddBiggy("Support Intercept Fire for Defenders: " + this.game.Data.UnitObj[tunr].Name);
        }
        if (isAA)
        {
          this.AddBiggy("AA from a distance: " + this.game.Data.UnitObj[tunr].Name);
          this.UList[this.UCounter].UAA = 1;
        }
      }
      if (this.game.Data.UnitObj[tunr].Regime == this.game.Data.Turn)
        return;
      Coordinate reconMinusHide;
      if (this.game.Data.UnitObj[tunr].Regime == this.game.Data.Turn | !this.game.Data.FOWOn | this.game.Data.Round == 0)
        reconMinusHide.x = 3;
      else
        reconMinusHide = this.game.HandyFunctionsObj.GetReconMinusHide(tunr, this.game.Data.Turn);
      this.UList[this.UCounter].UVisibility = reconMinusHide;
    }

    pub fn AddToI()
    {
      Random random = new Random((int) Math.Round( this.game.Data.GameID /  (this.TargetX + 1) *  (this.TargetY + 1)));
      bool flag1 = false;
      let mut ucounter1: i32 =  this.UCounter;
      for (let mut index: i32 =  0; index <= ucounter1; index += 1)
      {
        let mut unr: i32 =  this.UList[index].UNr;
        if (this.UList[index].Uattacker == 1 &&  this.game.Data.RuleVar[493] > 0.0)
        {
          Coordinate coordinate = this.game.HandyFunctionsObj.MoveApCostPreview(unr, this.UList[index].UCanRetreat.x, this.UList[index].UCanRetreat.y, this.UList[index].UCanRetreat.x, this.UList[index].UCanRetreat.y, 0, this.TargetX, this.TargetY, 0, true);
          if (coordinate.data2 > 0 & coordinate.data2 < 100)
          {
            flag1 = true;
            this.UList[index].UpartialAttack = true;
          }
        }
      }
      let mut ucounter2: i32 =  this.UCounter;
      for (let mut index1: i32 =  0; index1 <= ucounter2; index1 += 1)
      {
        let mut unr: i32 =  this.UList[index1].UNr;
        maxDist: i32;
        if ( this.game.Data.RuleVar[419] > 0.0)
        {
          if ((this.CombatType == 3 | flag1) & this.UList[index1].Uattacker == 1)
          {
            maxDist = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index1].UNr].X, this.game.Data.UnitObj[this.UList[index1].UNr].Y, 0, this.TargetX, this.TargetY, 0, 99);
            this.game.HandyFunctionsObj.RedimTempLosValue(0);
            this.game.HandyFunctionsObj.SetTempLos(this.game.Data.UnitObj[this.UList[index1].UNr].X, this.game.Data.UnitObj[this.UList[index1].UNr].Y, maxDist + 1, false, false);
            this.UList[index1].ULos = new int[this.UCounter + 1];
            let mut ucounter3: i32 =  this.UCounter;
            for (let mut index2: i32 =  0; index2 <= ucounter3; index2 += 1)
              this.UList[index1].ULos[index2] = this.game.EditObj.TempLos[0].Value[this.game.Data.UnitObj[this.UList[index2].UNr].X, this.game.Data.UnitObj[this.UList[index2].UNr].Y];
          }
          if ((this.CombatType == 3 | flag1) & this.UList[index1].Uattacker == 0)
          {
            maxDist = Math.Max(this.game.HandyFunctionsObj.GetMaxArtRange(unr, 0), this.game.HandyFunctionsObj.GetMaxDirectRange(unr, 0)) + 1;
            this.game.HandyFunctionsObj.RedimTempLosValue(0);
            this.game.HandyFunctionsObj.SetTempLos(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, maxDist, false, false);
            this.UList[index1].ULos = new int[this.UCounter + 1];
            let mut ucounter4: i32 =  this.UCounter;
            for (let mut index3: i32 =  0; index3 <= ucounter4; index3 += 1)
            {
              maxDist = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index3].UNr].X, this.game.Data.UnitObj[this.UList[index3].UNr].Y, 0, this.TargetX, this.TargetY, 0, 99);
              this.UList[index1].ULos[index3] = this.game.EditObj.TempLos[0].Value[this.game.Data.UnitObj[this.UList[index3].UNr].X, this.game.Data.UnitObj[this.UList[index3].UNr].Y];
            }
          }
          if (this.CombatType == 1 & this.UList[index1].Uattacker == 0 & this.UList[index1].USupportInterceptFire)
          {
            maxDist = Math.Max(this.game.HandyFunctionsObj.GetMaxArtRange(unr, 0), this.game.HandyFunctionsObj.GetMaxDirectRange(unr, 0)) + 1;
            this.game.HandyFunctionsObj.RedimTempLosValue(0);
            this.game.HandyFunctionsObj.SetTempLos(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, maxDist, false, false);
            this.UList[index1].ULos = new int[this.UCounter + 1];
            let mut ucounter5: i32 =  this.UCounter;
            for (let mut index4: i32 =  0; index4 <= ucounter5; index4 += 1)
            {
              maxDist = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index4].UNr].X, this.game.Data.UnitObj[this.UList[index4].UNr].Y, 0, this.TargetX, this.TargetY, 0, 99);
              this.UList[index1].ULos[index4] = this.game.EditObj.TempLos[0].Value[this.game.Data.UnitObj[this.UList[index4].UNr].X, this.game.Data.UnitObj[this.UList[index4].UNr].Y];
            }
          }
        }
        if (this.game.Data.UnitObj[unr].SFCount > -1)
        {
          let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
          for (let mut i1: i32 =  0; i1 <= sfCount1; i1 += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[i1];
            let mut type: i32 =  this.game.Data.SFObj[sf].Type;
            let mut num1: i32 =  0;
            bool flag2 = false;
            if (this.CombatType == 1 && this.UList[index1].Uattacker == 1 &&  this.game.Data.RuleVar[493] > 0.0 && this.game.HandyFunctionsObj.MoveApCostPreview(unr, this.UList[index1].UCanRetreat.x, this.UList[index1].UCanRetreat.y, this.UList[index1].UCanRetreat.x, this.UList[index1].UCanRetreat.y, 0, this.TargetX, this.TargetY, 0, true, onlySfNr: sf).x > this.game.Data.SFObj[sf].Ap)
            {
              flag2 = true;
              maxDist = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index1].UNr].X, this.game.Data.UnitObj[this.UList[index1].UNr].Y, this.game.Data.UnitObj[this.UList[index1].UNr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map);
              if (maxDist > this.game.Data.SFTypeObj[type].ArtRange)
              {
                num1 = 1;
                if ( this.game.Data.RuleVar[419] > 0.0)
                {
                  let mut num2: i32 =  0;
                  num1 = maxDist <= this.game.Data.SFTypeObj[type].directRange ? num2 : 1;
                }
              }
            }
            if (this.CombatType == 1 | this.CombatType == 10 && this.game.Data.SFTypeObj[type].Theater > 0)
              num1 = 1;
            if (this.CombatType == 9 && this.game.Data.SFTypeObj[type].Theater == 1)
              num1 = 1;
            if (this.CombatType == 2 && this.game.Data.SFTypeObj[type].Theater == 0)
            {
              if (this.UList[index1].Uattacker == 0)
              {
                if (this.game.Data.SFTypeObj[type].ArtRange < 1)
                  num1 = 1;
              }
              else
                num1 = 1;
            }
            if (this.CombatType == 3 | this.CombatType == 4)
            {
              if (this.UList[index1].Uattacker != 0)
                ;
              if (this.UList[index1].Uattacker == 1)
              {
                maxDist = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index1].UNr].X, this.game.Data.UnitObj[this.UList[index1].UNr].Y, this.game.Data.UnitObj[this.UList[index1].UNr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map);
                if (maxDist > this.game.Data.SFTypeObj[type].ArtRange)
                {
                  num1 = 1;
                  if ( this.game.Data.RuleVar[419] > 0.0)
                  {
                    num1 = 0;
                    if (maxDist > this.game.Data.SFTypeObj[type].directRange)
                      num1 = 1;
                  }
                }
              }
              else
              {
                let mut num3: i32 =  this.UList[index1].Uattacker == 0 & !this.InterceptFire & num1 == 0 ? 1 : 0;
              }
            }
            if (this.CombatType == 1 & this.UList[index1].USupportInterceptFire && this.UList[index1].Uattacker == 0)
            {
              maxDist = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index1].UNr].X, this.game.Data.UnitObj[this.UList[index1].UNr].Y, this.game.Data.UnitObj[this.UList[index1].UNr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map);
              if (maxDist > this.game.Data.SFTypeObj[type].ArtRange)
              {
                num1 = 1;
                if ( this.game.Data.RuleVar[419] > 0.0)
                {
                  num1 = 0;
                  if (maxDist > this.game.Data.SFTypeObj[type].directRange)
                    num1 = 1;
                }
              }
            }
            if (this.CombatType == 3 && this.UList[index1].Uattacker == 1 && this.game.Data.SFTypeObj[type].Theater == 1)
              num1 = 1;
            if (this.CombatType == 4)
            {
              if (this.UList[index1].Uattacker == 0 && this.game.Data.SFTypeObj[type].Theater == 1)
                num1 = 1;
              if (this.UList[index1].Uattacker == 1 && this.game.Data.SFTypeObj[type].Theater != 1)
                num1 = 1;
            }
            if (this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15)
            {
              if (this.UList[index1].Uattacker == 1 && this.game.Data.SFTypeObj[type].Theater < 2)
                num1 = 1;
              if (this.UList[index1].Uattacker == 0 && this.game.Data.SFTypeObj[type].Theater == 2 && this.game.Data.UnitObj[unr].X == this.TargetX && this.game.Data.UnitObj[unr].Y == this.TargetY && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea)
              {
                if (this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Location == -1)
                  num1 = 1;
                else if (!this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Location].Type].IsAirfield && !this.game.Data.LocObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].Location].isAirfield)
                  num1 = 1;
              }
            }
            if (this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15 | this.CombatType == 9 && this.UList[index1].Uattacker == 0 && this.UList[index1].UDefIntercept)
            {
              if (this.game.Data.SFTypeObj[type].Theater < 2)
                num1 = 1;
              if (this.game.Data.SFTypeObj[type].BackBench)
                num1 = 1;
            }
            if (this.CombatType == 9 & this.UList[index1].Uattacker == 1 && !this.UList[index1].UParadropper && this.game.Data.SFTypeObj[type].Theater < 2)
              num1 = 1;
            if (this.UList[index1].UAA == 1 && this.game.Data.SFTypeObj[type].AARange < this.game.HandyFunctionsObj.Distance(this.TargetX, this.TargetY, this.TargetMap, this.game.Data.UnitObj[this.UList[index1].UNr].X, this.game.Data.UnitObj[this.UList[index1].UNr].Y, this.game.Data.UnitObj[this.UList[index1].UNr].Map))
              num1 = 1;
            if (num1 == 0)
            {
              let mut num4: i32 =  this.game.Data.SFObj[sf].Qty;
              let mut num5: i32 =  -1;
              averageRdn: i32;
              averageMor: i32;
              averageXp: i32;
              averageEntrench: i32;
              Coordinate reconMinusHide;
              i2: i32;
              if (this.previewMode)
              {
                averageRdn = this.game.HandyFunctionsObj.GetAverageRdn(unr);
                averageMor = this.game.HandyFunctionsObj.GetAverageMor(unr);
                averageXp = this.game.HandyFunctionsObj.GetAverageXp(unr);
                averageEntrench = this.game.HandyFunctionsObj.GetAverageEntrench(unr);
                if (this.game.Data.FOWOn & this.game.Data.UnitObj[unr].Regime != this.game.Data.Turn)
                {
                  if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn | !this.game.Data.FOWOn | this.game.Data.Round == 0)
                    reconMinusHide.x = 3;
                  else
                    reconMinusHide = this.game.HandyFunctionsObj.GetReconMinusHide(unr, this.game.Data.Turn);
                  this.UList[index1].previewInfoLevel = reconMinusHide.x;
                  if (reconMinusHide.x == 2)
                  {
                    this.game.HandyFunctionsObj.RandomizeForUnit(unr, i1);
                    float num6 =  reconMinusHide.y / (this.game.Data.RuleVar[56] - this.game.Data.RuleVar[55]);
                    float num7 =  ((1.0 -  num6) * 2.0);
                    num4 = (int) Math.Round( Conversion.Int((VBMath.Rnd() * num7 + num6) *  num4));
                    if (num4 < 1)
                      num4 = 1;
                    VBMath.Randomize();
                  }
                  else if (reconMinusHide.x == 3)
                  {
                    num4 = num4;
                  }
                  else
                  {
                    num4 = 0;
                    if (i1 == 0)
                    {
                      i2 = 0;
                      let mut num8: i32 =  0;
                      let mut num9: i32 =  0;
                      SimpleList simpleList = SimpleList::new();
                      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                      for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
                      {
                        if (this.game.Data.UnitObj[index5].Regime == this.game.Data.UnitObj[unr].Regime & this.game.Data.UnitObj[index5].PreDef == -1)
                        {
                          let mut sfCount2: i32 =  this.game.Data.UnitObj[index5].SFCount;
                          for (let mut index6: i32 =  0; index6 <= sfCount2; index6 += 1)
                          {
                            i2 = this.game.Data.UnitObj[index5].SFList[index6];
                            if (this.game.Data.SFTypeObj[this.game.Data.SFObj[i2].Type].Attacks > 0)
                            {
                              let mut num10: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[i2].Type].HitPointsDef[0];
                              simpleList.AddWeight(this.game.Data.SFObj[i2].Type, num10 * this.game.Data.SFObj[i2].Qty);
                              num8 += this.game.Data.SFObj[i2].Qty;
                            }
                          }
                          num9 += 1;
                        }
                      }
                      simpleList.ReverseSortHighSpeed();
                      if (simpleList.Counter > -1)
                      {
                        num5 = simpleList.Id[0];
                        if (num9 > 0)
                          num4 = (int) Math.Round( num8 /  (num9 * 2)) + 1;
                      }
                    }
                  }
                }
                else
                {
                  reconMinusHide.x = 3;
                  this.UList[index1].previewInfoLevel = reconMinusHide.x;
                }
              }
              else if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn | !this.game.Data.FOWOn | this.game.Data.Round == 0)
                reconMinusHide.x = 3;
              else
                reconMinusHide = this.game.HandyFunctionsObj.GetReconMinusHide(unr, this.game.Data.Turn);
              let mut num11: i32 =  num4;
              for (let mut index7: i32 =  1; index7 <= num11; index7 += 1)
              {
                this += 1.IDcounter;
                this += 1.ICounter;
                this.IList = (Individual[]) Utils.CopyArray((Array) this.IList, (Array) new Individual[this.ICounter + 1]);
                this.IList[this.ICounter].IBreakTrough = 0;
                let mut landscapeType: i32 =  this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].LandscapeType;
                this.IList[this.ICounter].IInitThrow = 0;
                this.IList[this.ICounter].IKilled = 0;
                this.IList[this.ICounter].ILastAttacked = 0;
                this.IList[this.ICounter].IID = this.IDcounter;
                this.IList[this.ICounter].IUnr = unr;
                this.IList[this.ICounter].IUlistNr = index1;
                this.IList[this.ICounter].IRdn = this.game.Data.SFObj[sf].Rdn;
                this.IList[this.ICounter].IRetreat = 0;
                this.IList[this.ICounter].IRetreated = 0;
                this.IList[this.ICounter].IRetreatMode = 0;
                this.IList[this.ICounter].ISFNr = sf;
                this.IList[this.ICounter].IXp = this.game.Data.SFObj[sf].Xp;
                this.IList[this.ICounter].IMor = this.game.Data.SFObj[sf].Mor;
                this.IList[this.ICounter].IAttacker = this.UList[index1].Uattacker;
                this.IList[this.ICounter].ISFType = this.game.Data.SFObj[sf].Type;
                if (num5 > -1)
                  this.IList[this.ICounter].ISFType = num5;
                this.IList[this.ICounter].IParadropper = this.UList[index1].UParadropper;
                this.IList[this.ICounter].IRdnInitial = this.IList[this.ICounter].IRdn;
                this.IList[this.ICounter].IMorInitial = this.IList[this.ICounter].IMor;
                this.IList[this.ICounter].IXpInitial = this.IList[this.ICounter].IXp;
                this.IList[this.ICounter].IEntrenchInitial = this.game.Data.SFObj[sf].CurrentEntrench;
                this.IList[this.ICounter].IleftOutOfPartialAttack = flag2;
                if (flag2)
                  this.somePartialAttackPresent = true;
                this.IList[this.ICounter].IHistoricInit = new int[50];
                if ( this.game.Data.RuleVar[431] > 0.0)
                {
                  if ( this.game.Data.RuleVar[419] > 0.0)
                  {
                    num12: i32;
                    if (this.CombatType == 3 | this.CombatType == 4)
                    {
                      let mut maxValue: i32 =  (int) Math.Round( this.game.Data.RuleVar[55]) + (int) Math.Round( ((int) Math.Round( this.game.Data.RuleVar[56]) - (int) Math.Round( this.game.Data.RuleVar[55])) / 2.0);
                      num12 = (int) Math.Round( (random.Next((int) Math.Round( this.game.Data.RuleVar[55]), maxValue) + random.Next((int) Math.Round( this.game.Data.RuleVar[55]), maxValue)) / 2.0);
                    }
                    else
                    {
                      let mut maxValue: i32 =  (int) Math.Round( this.game.Data.RuleVar[55]) + (int) Math.Round( ((int) Math.Round( this.game.Data.RuleVar[56]) - (int) Math.Round( this.game.Data.RuleVar[55])) / 2.0);
                      num12 = (int) Math.Round( (random.Next(0, maxValue) + random.Next(0, maxValue)) / 2.0);
                    }
                    let mut num13: i32 =  num12 + this.GetIndividualHide(this.ICounter);
                    if (this.IList[this.ICounter].IAttacker == 0)
                      index1 = index1;
                    this.IList[this.ICounter].IvisibleFromRound = 999999;
                    let mut num14: i32 =  (int) Math.Round( this.game.Data.RuleVar[56]);
                    if (this.IList[this.ICounter].IUnr == 135)
                      index1 = index1;
                    if (num13 > num14)
                      num13 = num14 + (int) Math.Round( (num13 - num14) / 2.0);
                    if (num13 > num14 + 5)
                      num13 = num14 + 5 + (int) Math.Round( (num13 - (num14 + 5)) / 2.0);
                    i2 = this.GetEffectiveReconOnHexOfTargettedIndividual(this.ICounter);
                    let mut num15: i32 =  (int) Math.Round( this.game.Data.RuleVar[55]) + (int) Math.Round( ((int) Math.Round( this.game.Data.RuleVar[56]) - (int) Math.Round( this.game.Data.RuleVar[55])) / 2.0) + this.GetIndividualHide(this.ICounter);
                    if (this.IList[this.ICounter].IUnr == 202)
                      index1 = index1;
                    if (i2 >= num13 & reconMinusHide.x <= 1 & num15 > num13 & num15 >= i2 + 1)
                      num13 = DrawMod.RandyNumber.Next(i2 + 1, num15 + 1);
                    else if (i2 >= num13 & reconMinusHide.x <= 1)
                      num13 = i2 + 1;
                    if (num13 > 90)
                      num13 = 90;
                    this.IList[this.ICounter].IcoverPoints =  num13;
                  }
                  else
                  {
                    this.IList[this.ICounter].IcoverPoints =  random.Next((int) Math.Round( this.game.Data.RuleVar[55]), (int) Math.Round( this.game.Data.RuleVar[55]) + (int) Math.Round( ((int) Math.Round( this.game.Data.RuleVar[56]) - (int) Math.Round( this.game.Data.RuleVar[55])) / 2.0));
                    Individual[] ilist = this.IList;
                    Individual[] individualArray = ilist;
                    let mut icounter: i32 =  this.ICounter;
                    let mut index8: i32 =  icounter;
                    individualArray[index8].IcoverPoints = ilist[icounter].IcoverPoints +  this.GetIndividualHide(this.ICounter);
                    this.IList[this.ICounter].IvisibleFromRound = 999999;
                    if (this.CombatType == 13 | this.CombatType == 5 && this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].Theater == 2)
                    {
                      this.IList[this.ICounter].IcoverPoints = 0.0f;
                      this.IList[this.ICounter].IvisibleFromRound = 0;
                    }
                  }
                }
                else
                {
                  this.IList[this.ICounter].IcoverPoints = 0.0f;
                  this.IList[this.ICounter].IvisibleFromRound = 0;
                }
                if ( this.game.Data.RuleVar[419] > 0.0)
                {
                  if (this.CombatType == 3 & this.UList[index1].Uattacker == 1 | flag2)
                  {
                    if (this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directRange >= maxDist)
                    {
                      this.IList[this.ICounter].IdirectFire = true;
                      this.IList[this.ICounter].IdirectMod =  this.game.EditObj.TempLos[0].Value[this.CombatTarget.x, this.CombatTarget.y];
                      this.IList[this.ICounter].IdirectMod =  ( this.IList[this.ICounter].IdirectMod *  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directModFirstHex / 100.0);
                      let mut num16: i32 =  maxDist;
                      for (let mut index9: i32 =  2; index9 <= num16; index9 += 1)
                        this.IList[this.ICounter].IdirectMod =  ( this.IList[this.ICounter].IdirectMod *  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directModPerHex / 100.0);
                    }
                    else
                    {
                      this.IList[this.ICounter].IdirectFire = false;
                      this.IList[this.ICounter].IdirectMod = 0.0f;
                    }
                  }
                  else if ((this.CombatType == 3 | flag1) & this.UList[index1].Uattacker == 0 & !this.InterceptFire)
                  {
                    this.IList[this.ICounter].IdirectFireDef = new bool[this.UCounter + 1];
                    this.IList[this.ICounter].IdirectModDef = new float[this.UCounter + 1];
                    let mut ucounter6: i32 =  this.UCounter;
                    for (let mut index10: i32 =  0; index10 <= ucounter6; index10 += 1)
                    {
                      if (this.UList[index10].Uattacker == 1)
                      {
                        let mut num17: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index10].UNr].X, this.game.Data.UnitObj[this.UList[index10].UNr].Y, 0, this.CombatTarget.x, this.CombatTarget.y, 0, 99);
                        if (this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directRange >= num17)
                        {
                          this.IList[this.ICounter].IdirectFireDef[index10] = true;
                          this.IList[this.ICounter].IdirectModDef[index10] =  this.game.EditObj.TempLos[0].Value[this.game.Data.UnitObj[this.UList[index10].UNr].X, this.game.Data.UnitObj[this.UList[index10].UNr].Y];
                          this.IList[this.ICounter].IdirectModDef[index10] =  ( this.IList[this.ICounter].IdirectModDef[index10] *  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directModFirstHex / 100.0);
                          let mut num18: i32 =  num17;
                          for (let mut index11: i32 =  2; index11 <= num18; index11 += 1)
                            this.IList[this.ICounter].IdirectModDef[index10] =  ( this.IList[this.ICounter].IdirectModDef[index10] *  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directModPerHex / 100.0);
                        }
                        else
                        {
                          this.IList[this.ICounter].IdirectFireDef[index10] = false;
                          this.IList[this.ICounter].IdirectModDef[index10] = 0.0f;
                        }
                      }
                    }
                  }
                  else if (this.CombatType == 1 & this.UList[index1].Uattacker == 0 & this.UList[index1].USupportInterceptFire)
                  {
                    this.IList[this.ICounter].IdirectFireDef = new bool[this.UCounter + 1];
                    this.IList[this.ICounter].IdirectModDef = new float[this.UCounter + 1];
                    let mut ucounter7: i32 =  this.UCounter;
                    for (let mut index12: i32 =  0; index12 <= ucounter7; index12 += 1)
                    {
                      if (this.UList[index12].Uattacker == 1)
                      {
                        let mut num19: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.UList[index12].UNr].X, this.game.Data.UnitObj[this.UList[index12].UNr].Y, 0, this.CombatTarget.x, this.CombatTarget.y, 0, 99);
                        if (this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directRange >= num19)
                        {
                          this.IList[this.ICounter].IdirectFireDef[index12] = true;
                          this.IList[this.ICounter].IdirectModDef[index12] =  this.game.EditObj.TempLos[0].Value[this.game.Data.UnitObj[this.UList[index12].UNr].X, this.game.Data.UnitObj[this.UList[index12].UNr].Y];
                          this.IList[this.ICounter].IdirectModDef[index12] =  ( this.IList[this.ICounter].IdirectModDef[index12] *  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directModFirstHex / 100.0);
                          let mut num20: i32 =  num19;
                          for (let mut index13: i32 =  2; index13 <= num20; index13 += 1)
                            this.IList[this.ICounter].IdirectModDef[index12] =  ( this.IList[this.ICounter].IdirectModDef[index12] *  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directModPerHex / 100.0);
                        }
                        else
                        {
                          this.IList[this.ICounter].IdirectFireDef[index12] = false;
                          this.IList[this.ICounter].IdirectModDef[index12] = 0.0f;
                        }
                      }
                    }
                  }
                }
                this.IList[this.ICounter].IHp = 0;
                if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].DepletingHitpointRule > 0)
                {
                  float num21 = 1f;
                  if ( this.game.Data.RuleVar[407] > 0.0)
                  {
                    let mut index14: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 2;
                    let mut index15: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 3;
                    let mut index16: i32 =  (int) Math.Round( this.game.Data.RuleVar[407]) + 4;
                    if (index14 > 0 & (index15 > 0 | index16 > 0))
                    {
                      let mut tid: i32 =  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].SFTypeVar[index14];
                      let mut num22: i32 =  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].SFTypeVar[index15];
                      let mut num23: i32 =  this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].SFTypeVar[index16];
                      i2 = this.IList[this.ICounter].IAttacker != 1 ? num23 : num22;
                      let mut weight: i32 =  this.game.Data.UnitObj[this.IList[this.ICounter].IUnr].items.list.FindWeight(tid);
                      if (i2 > 0)
                      {
                        num21 =  weight /  i2;
                        if ( num21 > 1.0)
                          num21 = 1f;
                      }
                      else
                        num21 = 1f;
                    }
                  }
                  this.IList[this.ICounter].IHp = (int) Math.Round( ( this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].HitPoints[0] * num21));
                }
                if (this.UList[index1].UAA == 1)
                  this.IList[this.ICounter].IAA = 1;
                if (this.UList[index1].Uattacker == 0 & this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].Theater == 0)
                  this.IList[this.ICounter].IEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
                else if ((this.CombatType == 3 | this.CombatType == 4) & this.game.Data.Product >= 6)
                  this.IList[this.ICounter].IEntrench = this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].ArtRange <= 0 ? (this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].directRange <= 0 ? 0 : this.game.Data.SFObj[sf].CurrentEntrench) : this.game.Data.SFObj[sf].CurrentEntrench;
                else if (this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].ArtRange < 1)
                  this.IList[this.ICounter].IEntrench = 0;
                else if (this.game.Data.Product >= 6)
                  this.IList[this.ICounter].IEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
                let mut unitGroup: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup;
                if (this.previewMode)
                {
                  this.IList[this.ICounter].previewCoverPoints = (int) Math.Round( this.IList[this.ICounter].IcoverPoints);
                  this.IList[this.ICounter].IcoverPoints = 0.0f;
                  this.IList[this.ICounter].IvisibleFromRound = 0;
                  if (reconMinusHide.x == 2)
                  {
                    let mut num24: i32 =  1;
                    do
                    {
                      if (num24 == 1)
                        i2 = averageRdn;
                      if (num24 == 2)
                        i2 = averageXp;
                      if (num24 == 3)
                        i2 = averageMor;
                      if (num24 == 4)
                        i2 = averageEntrench;
                      this.game.HandyFunctionsObj.RandomizeForUnit(unr, i2);
                      float num25 =  reconMinusHide.y / (this.game.Data.RuleVar[56] - this.game.Data.RuleVar[55]);
                      float num26 =  ((1.0 -  num25) * 2.0);
                      float num27 = VBMath.Rnd() * num26 + num25;
                      i2 = (int) Math.Round( Conversion.Int( i2 * num27));
                      if (i2 < 0)
                        i2 = 0;
                      if (i2 > 100)
                        i2 = 100;
                      if (num24 == 1)
                        this.IList[this.ICounter].IRdn = i2;
                      if (num24 == 2)
                        this.IList[this.ICounter].IXp = i2;
                      if (num24 == 3)
                        this.IList[this.ICounter].IMor = i2;
                      if (num24 == 4)
                        this.IList[this.ICounter].IEntrench = i2;
                      num24 += 1;
                    }
                    while (num24 <= 4);
                  }
                  else if (reconMinusHide.x < 2)
                  {
                    this.IList[this.ICounter].IRdn = 100;
                    this.IList[this.ICounter].IXp = 20;
                    this.IList[this.ICounter].IMor = 70;
                    this.IList[this.ICounter].IEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(this.TargetX, this.TargetY, 0, this.game.Data.SFTypeObj[this.IList[this.ICounter].ISFType].UnitGroup);
                    this.IList[this.ICounter].IcoverPoints = 0.0f;
                    this.IList[this.ICounter].IvisibleFromRound = 0;
                  }
                }
              }
            }
          }
        }
      }
    }

    pub OtherSideStillFighting: bool(seenfromside: i32)
    {
      let mut num: i32 =  0;
      let mut icounter: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter; index += 1)
      {
        if (this.IList[index].IAttacker != seenfromside && this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0 & this.IList[index].IRetreated == 0)
          num += 1;
      }
      return num > 0;
    }

    pub fn CheckOutOfAP()
    {
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut icounter1: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IAttacker == 0)
        {
          num2 += 1;
          if (this.IList[index].IRetreat == 0 && this.IList[index].IKilled == 0)
            num1 += 1;
        }
      }
      if (this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15)
        num1 = 1;
      if (num2 == 0)
        num1 = 1;
      if (this.CombatType == 3 | this.CombatType == 4)
        num1 = 1;
      if (num1 == 0)
        return;
      let mut ucounter: i32 =  this.UCounter;
      for (let mut fromy: i32 =  0; fromy <= ucounter; fromy += 1)
      {
        if (this.UList[fromy].UParticipated > 0)
        {
          BattleUnit[] ulist = this.UList;
          BattleUnit[] battleUnitArray = ulist;
          let mut index1: i32 =  fromy;
          let mut index2: i32 =  index1;
          battleUnitArray[index2].UApSpend = ulist[index1].UApSpend + 10;
        }
        if (this.UList[fromy].URetreat == 0 & this.UList[fromy].Uattacker == 0)
        {
          if (this.CombatType == 9 & this.CombatRound == 5)
          {
            if (this.UList[fromy].UDefIntercept)
            {
              this.AddReport(0, "Unit out of AP", "Unit retreats due to max combatrounds for this engagement type reached.", fromy, this.CombatRound);
              let mut icounter2: i32 =  this.ICounter;
              for (let mut index: i32 =  0; index <= icounter2; index += 1)
              {
                if (this.IList[index].IUnr == this.UList[fromy].UNr)
                {
                  this.IList[index].IRetreat = this.CombatRound;
                  this.IList[index].IRetreated = 1;
                  this.IList[index].IRetreatMode = 2;
                  this.IList[index].IOutOfAp = this.CombatRound;
                }
              }
              this.UList[fromy].URetreat = this.CombatRound;
              this.UList[fromy].URetreatMode = 2;
            }
            let mut icounter3: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter3; index += 1)
            {
              if (this.IList[index].IRetreat == 0 & this.IList[index].IAttacker == 0 && this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 2)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
          }
          if (this.UList[fromy].UApSpend + 10 > this.UList[fromy].UMaxApToSpend)
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to being out of AP.", fromy, this.CombatRound);
            let mut icounter4: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter4; index += 1)
            {
              if (this.IList[index].IUnr == this.UList[fromy].UNr)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 3;
          }
        }
        if (this.UList[fromy].URetreat == 0 & this.UList[fromy].Uattacker == 1)
        {
          if (this.UList[fromy].UApSpend + 10 > this.UList[fromy].UMaxApToSpend)
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to being out of AP.", fromy, this.CombatRound);
            let mut icounter5: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter5; index += 1)
            {
              if (this.IList[index].IUnr == this.UList[fromy].UNr)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 3;
          }
          let mut num3: i32 =  2;
          CustomCombatCalls customCombatObj1 = this.customCombatObj;
          combatClass1: CombatClass = this;
          ref local1: CombatClass = ref combatClass1;
          if (customCombatObj1.AlterCombatLastRound(ref local1) > 0)
          {
            CustomCombatCalls customCombatObj2 = this.customCombatObj;
            combatClass2: CombatClass = this;
            ref local2: CombatClass = ref combatClass2;
            num3 = customCombatObj2.AlterCombatLastRound(ref local2);
          }
          if ((this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15) & this.CombatRound == num3)
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to max combatrounds for this engagement type reached.", fromy, this.CombatRound);
            let mut icounter6: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter6; index += 1)
            {
              if (this.IList[index].IUnr == this.UList[fromy].UNr)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 2;
          }
          if (this.CombatType == 11 & this.CombatRound == 2)
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to max combatrounds for this engagement type reached.", fromy, this.CombatRound);
            let mut icounter7: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter7; index += 1)
            {
              if (this.IList[index].IUnr == this.UList[fromy].UNr)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 4;
          }
          if (this.InterceptFire & this.CombatRound >= (int) Math.Round( (this.game.Data.RuleVar[428] / 2f)))
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to max combatrounds for this engagement type reached.", fromy, this.CombatRound);
            let mut icounter8: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter8; index += 1)
            {
              if (this.IList[index].IUnr == this.UList[fromy].UNr)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 4;
          }
          if (this.CombatType == 9 & this.CombatRound == 5)
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to max combatrounds for this engagement type reached.", fromy, this.CombatRound);
            if (!this.UList[fromy].UParadropper)
            {
              let mut icounter9: i32 =  this.ICounter;
              for (let mut index: i32 =  0; index <= icounter9; index += 1)
              {
                if (this.IList[index].IUnr == this.UList[fromy].UNr)
                {
                  this.IList[index].IRetreat = this.CombatRound;
                  this.IList[index].IRetreated = 1;
                  this.IList[index].IRetreatMode = 2;
                  this.IList[index].IOutOfAp = this.CombatRound;
                }
              }
              this.DoParaDropperCasualties();
              this.UList[fromy].URetreat = this.CombatRound;
              this.UList[fromy].URetreatMode = 2;
            }
          }
        }
        else
        {
          if (this.UList[fromy].USupportInterceptFire & this.CombatRound >= (int) Math.Round( (this.game.Data.RuleVar[428] / 2f)))
          {
            this.AddReport(0, "Unit out of AP", "Unit retreats due to max combatrounds for this engagement type reached.", fromy, this.CombatRound);
            let mut icounter10: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter10; index += 1)
            {
              if (this.IList[index].IUnr == this.UList[fromy].UNr)
              {
                this.IList[index].IRetreat = this.CombatRound;
                this.IList[index].IRetreated = 1;
                this.IList[index].IRetreatMode = 2;
                this.IList[index].IOutOfAp = this.CombatRound;
              }
            }
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 4;
          }
          if (this.CombatType == 9 & this.CombatRound == 5 && !this.UList[fromy].UParadropper)
            this.DoParaDropperCasualties();
        }
      }
    }

    pub fn DoParaDropperCasualties()
    {
      let mut icounter1: i32 =  this.ICounter;
      num1: i32;
      num2: i32;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IAttacker == 1 && !this.IList[index].IParadropper)
        {
          num1 += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
          if (this.IList[index].IKilled > 0)
            num2 += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
        }
      }
      float num3 =  num2 /  num1;
      this.AddBiggy("Paradroppers suffer " + Conversion.Str( Conversion.Int(num3 * 100f)) + "% casualties due to transport losses.");
      if ( num3 <= 0.0)
        return;
      let mut icounter2: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter2; index += 1)
      {
        if (this.IList[index].IAttacker == 1 && this.IList[index].IParadropper &&  VBMath.Rnd() <  num3)
          this.IList[index].IKilled = this.CombatRound;
      }
    }

    pub fn CheckSafeRetreat()
    {
      let mut icounter1: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IRetreat > 0 && this.IList[index].IRetreat + 2 <= this.CombatRound)
          this.IList[index].IRetreated = 1;
      }
      int[] numArray1 = new int[this.UCounter + 1];
      int[] numArray2 = new int[this.UCounter + 1];
      let mut icounter2: i32 =  this.ICounter;
      for (let mut index1: i32 =  0; index1 <= icounter2; index1 += 1)
      {
        if (this.IList[index1].IKilled <= 0 & -(this.IList[index1].IBroken ? 1 : 0) <= 0 & -(this.IList[index1].ICapitulate ? 1 : 0) <= 0)
        {
          int[] numArray3 = numArray1;
          int[] numArray4 = numArray3;
          let mut iulistNr1: i32 =  this.IList[index1].IUlistNr;
          let mut index2: i32 =  iulistNr1;
          let mut num1: i32 =  numArray3[iulistNr1] + 1;
          numArray4[index2] = num1;
          if (this.IList[index1].IRetreated > 0)
          {
            int[] numArray5 = numArray2;
            int[] numArray6 = numArray5;
            let mut iulistNr2: i32 =  this.IList[index1].IUlistNr;
            let mut index3: i32 =  iulistNr2;
            let mut num2: i32 =  numArray5[iulistNr2] + 1;
            numArray6[index3] = num2;
          }
        }
      }
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index: i32 =  0; index <= ucounter; index += 1)
      {
        if (this.UList[index].URetreated < 1 && numArray1[index] == numArray2[index] & numArray1[index] > 0)
          this.UList[index].URetreated = this.CombatRound;
      }
    }

    pub fn PreBattleStuff()
    {
      let mut num1: i32 =  num1;
      if (this.game.EditObj.CombatSim)
        return;
      if ( this.game.Data.RuleVar[435] > 0.0)
      {
        let mut ucounter: i32 =  this.UCounter;
        for (let mut index1: i32 =  0; index1 <= ucounter; index1 += 1)
        {
          let mut num2: i32 =  0;
          let mut icounter: i32 =  this.ICounter;
          num3: i32;
          for (let mut index2: i32 =  0; index2 <= icounter; index2 += 1)
          {
            if (this.IList[index2].IUnr == this.UList[index1].UNr && !(this.CombatType == 12 | this.CombatType == 10) & !this.IList[index2].IParadropper)
            {
              num3 = this.UList[this.IList[index2].IUlistNr].UApMoveCost;
              if (num3 > 999)
                num3 = this.UList[this.IList[index2].IUlistNr].UMaxApToSpend;
              if (num3 > this.game.Data.SFObj[this.IList[index2].ISFNr].Ap)
                num3 = this.game.Data.SFObj[this.IList[index2].ISFNr].Ap;
              let mut isfType: i32 =  this.IList[index2].ISFType;
              if (!this.IList[index2].IleftOutOfPartialAttack && num3 > num2)
                num2 = num3;
            }
          }
          this.UList[index1].UApMoveCost = num3;
        }
      }
      else
      {
        let mut icounter: i32 =  this.ICounter;
        for (let mut index3: i32 =  0; index3 <= icounter; index3 += 1)
        {
          if (index3 == 55)
            index3 = index3;
          if (this.IList[index3].IAttacker == 1 && !(this.CombatType == 12 | this.CombatType == 10) & !this.IList[index3].IParadropper)
          {
            let mut num4: i32 =  this.UList[this.IList[index3].IUlistNr].UApMoveCost;
            if (num4 > 999)
              num4 = this.UList[this.IList[index3].IUlistNr].UMaxApToSpend;
            let mut isfType: i32 =  this.IList[index3].ISFType;
            if (this.game.Data.SFTypeObj[isfType].FuelForMove > 0 & this.game.Data.SFTypeObj[isfType].FuelRegimeVar > -1)
            {
              let mut currentSlot: i32 =  this.game.Data.SFTypeObj[isfType].FuelRegimeVar;
              if ( this.game.Data.RuleVar[435] <= 0.0)
              {
                if ( this.game.Data.RuleVar[949] > 0.0)
                  currentSlot = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot, this.game.Data.UnitObj[this.UList[this.IList[index3].IUlistNr].UNr].RealX(ref this.game), this.game.Data.UnitObj[this.UList[this.IList[index3].IUlistNr].UNr].RealY(ref this.game));
                let mut num5: i32 =  (int) Math.Round( this.game.Data.SFTypeObj[isfType].FuelForMove * ( num4 / 10.0));
                if (this.game.Data.SFTypeObj[isfType].FuelForMove > num5)
                  num5 = this.game.Data.SFTypeObj[isfType].FuelForMove;
                let mut num6: i32 =  num5 * 1;
                if (this.game.Data.RegimeObj[this.AttackerRegime].RegimeSlot[currentSlot] >= num6)
                {
                  int[] regimeSlot = this.game.Data.RegimeObj[this.AttackerRegime].RegimeSlot;
                  int[] numArray1 = regimeSlot;
                  let mut index4: i32 =  currentSlot;
                  let mut index5: i32 =  index4;
                  let mut num7: i32 =  regimeSlot[index4] - num6;
                  numArray1[index5] = num7;
                  UnitClass[] unitObj = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray = unitObj;
                  let mut iunr: i32 =  this.IList[index3].IUnr;
                  let mut index6: i32 =  iunr;
                  unitClassArray[index6].FuelUsedMove = unitObj[iunr].FuelUsedMove + num6;
                  int[] regimeSlotPredict = this.game.Data.RegimeObj[this.AttackerRegime].TempRegimeSlotPredict;
                  int[] numArray2 = regimeSlotPredict;
                  let mut index7: i32 =  currentSlot;
                  let mut index8: i32 =  index7;
                  let mut num8: i32 =  regimeSlotPredict[index7] - num6;
                  numArray2[index8] = num8;
                }
                else
                {
                  let mut num9: i32 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[index3].IUnr].Regime].RegimeSlot[currentSlot];
                  int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[index3].IUnr].Regime].RegimeSlot;
                  int[] numArray3 = regimeSlot;
                  let mut index9: i32 =  currentSlot;
                  let mut index10: i32 =  index9;
                  let mut num10: i32 =  regimeSlot[index9] - num9;
                  numArray3[index10] = num10;
                  int[] regimeSlotPredict = this.game.Data.RegimeObj[this.game.Data.UnitObj[this.IList[index3].IUnr].Regime].TempRegimeSlotPredict;
                  int[] numArray4 = regimeSlotPredict;
                  let mut index11: i32 =  currentSlot;
                  let mut index12: i32 =  index11;
                  let mut num11: i32 =  regimeSlotPredict[index11] - num9;
                  numArray4[index12] = num11;
                  UnitClass[] unitObj = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray = unitObj;
                  let mut iunr: i32 =  this.IList[index3].IUnr;
                  let mut index13: i32 =  iunr;
                  unitClassArray[index13].FuelUsedMove = unitObj[iunr].FuelUsedMove + num9;
                }
              }
            }
          }
        }
      }
    }

    pub fn MoraleTest()
    {
    }

    pub void AddReport(
      type: i32,
      title: String,
      txt: String,
      fromy: i32,
      round: i32,
      string[,] matrx = null)
    {
      if (this.RepCounter > 60000)
        return;
      this += 1.RepCounter;
      if (this.RepTitle.GetUpperBound(0) < this.RepCounter)
      {
        this.RepTitle = (string[]) Utils.CopyArray((Array) this.RepTitle, (Array) new string[this.RepCounter + 1000 + 1]);
        this.RepText = (string[]) Utils.CopyArray((Array) this.RepText, (Array) new string[this.RepCounter + 1000 + 1]);
        this.RepRound = (int[]) Utils.CopyArray((Array) this.RepRound, (Array) new int[this.RepCounter + 1000 + 1]);
        this.RepType = (int[]) Utils.CopyArray((Array) this.RepType, (Array) new int[this.RepCounter + 1000 + 1]);
        this.RepFrom = (int[]) Utils.CopyArray((Array) this.RepFrom, (Array) new int[this.RepCounter + 1000 + 1]);
        this.RepCom = (string[,,]) Utils.CopyArray((Array) this.RepCom, (Array) new string[61, 6, this.RepCounter + 1000 + 1]);
      }
      this.RepTitle[this.RepCounter] = title;
      this.RepText[this.RepCounter] = txt;
      this.RepType[this.RepCounter] = type;
      this.RepRound[this.RepCounter] = round;
      this.RepFrom[this.RepCounter] = fromy;
      if (Information.IsNothing( matrx))
        return;
      let mut index1: i32 =  0;
      do
      {
        let mut index2: i32 =  0;
        do
        {
          this.RepCom[index1, index2, this.RepCounter] = matrx[index1, index2];
          index2 += 1;
        }
        while (index2 <= 5);
        index1 += 1;
      }
      while (index1 <= 60);
    }

    pub fn CheckOrderlyUnitRetreat()
    {
      let mut ucounter: i32 =  this.UCounter;
      for (let mut fromy: i32 =  0; fromy <= ucounter; fromy += 1)
      {
        title: String;
        txt: String;
        if (!this.UList[fromy].UPanicked & this.UList[fromy].URetreat == 0)
        {
          let mut Number: i32 =  this.game.Data.UnitObj[this.UList[fromy].UNr].SODefendPercent;
          if ( this.game.Data.RuleVar[482] > 0.0 & this.game.Data.Product >= 6 && this.UList[fromy].Uattacker == 1)
          {
            if (this.game.EditObj.attackTypeOption == 1)
            {
              Number = 90;
              if (this.CombatRound >= 2)
                Number = 101;
            }
            else if (this.game.EditObj.attackTypeOption == 2)
            {
              Number = 75;
              if (this.CombatRound >= 4)
                Number = 101;
            }
            else if (this.game.EditObj.attackTypeOption == 3 | this.game.EditObj.attackTypeOption == 0)
              Number = 50;
            else if (this.game.EditObj.attackTypeOption == 4)
              Number = 25;
          }
          if (!this.UList[fromy].UCanRetreat.onmap)
            Number = 0;
          if (this.CombatType == 3 | this.CombatType == 4)
            Number = 0;
          if ((this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15) & this.UList[fromy].Uattacker == 0 && this.game.Data.UnitObj[this.UList[fromy].UNr].X == this.TargetX & this.game.Data.UnitObj[this.UList[fromy].UNr].Y == this.TargetY & this.game.Data.UnitObj[this.UList[fromy].UNr].Map == this.TargetMap)
            Number = 0;
          if (this.CombatType == 9 | this.CombatType == 10 | this.CombatType == 12 & this.UList[fromy].Uattacker == 1)
            Number = 0;
          if (this.game.Data.Product > 4)
          {
            let mut num1: i32 =  0;
            let mut num2: i32 =  0;
            let mut num3: i32 =  0;
            let mut num4: i32 =  0;
            bool flag = false;
            let mut icounter: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter; index += 1)
            {
              if (this.IList[index].IAttacker == 1)
              {
                if (this.game.Data.SFTypeObj[this.IList[index].ISFType].DepletingHitpointRule < 1)
                {
                  if (this.IList[index].IRetreat < 1 & this.IList[index].IRetreated < 1)
                    num1 += 1;
                  else
                    num2 += 1;
                }
              }
              else if (this.IList[index].IAttacker == 0 && this.game.Data.SFTypeObj[this.IList[index].ISFType].DepletingHitpointRule < 1)
              {
                if (this.IList[index].IRetreat < 1 & this.IList[index].IRetreated < 1)
                  num3 += 1;
                else
                  num4 += 1;
              }
              if (this.game.Data.SFTypeObj[this.IList[index].ISFType].DepletingHitpointRule > 0 & this.game.Data.SFTypeObj[this.IList[index].ISFType].PreventCounter > -1 && this.IList[index].IUlistNr == fromy)
                flag = true;
            }
            if (flag)
            {
              if (this.UList[fromy].Uattacker == 1)
              {
                if (num1 < 1)
                  Number = 999;
              }
              else if (num3 < 1)
                Number = 999;
            }
          }
          if (Number > 0)
          {
            if (this.UList[fromy].URetreat == 0)
            {
              let mut num5: i32 =  0;
              let mut num6: i32 =  0;
              let mut icounter1: i32 =  this.ICounter;
              for (let mut index: i32 =  0; index <= icounter1; index += 1)
              {
                if (this.IList[index].IUnr == this.UList[fromy].UNr && !this.IList[index].IleftOutOfPartialAttack |  this.game.Data.RuleVar[493] < 1.0)
                {
                  let mut num7: i32 =  0;
                  if (this.IList[index].IRetreated > 0)
                    num7 = 1;
                  if (this.IList[index].IRetreat > 0)
                    num7 = 1;
                  if (this.IList[index].IKilled > 0)
                    num7 = 1;
                  if (num7 == 0)
                    num6 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
                  num5 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
                }
              }
              if (num5 == 0)
                num5 = 1;
              title = "Orderly retreat test";
              txt = "Units standing order is to retreat if less left then " + Strings.Trim(Conversion.Str( Number)) + "%." + "\r\n" + "Current troops left = " + Strings.Trim(Conversion.Str( Conversion.Int( (num6 * 100) /  num5))) + "%.";
              if ( (num6 * 100) /  num5 <  Number)
              {
                title = "Orderly retreat test : UNIT RETREATS";
                txt = txt + "\r\n" + "Results in retreat. ";
                s: String = s + "Unit decides to retreat: " + this.game.Data.UnitObj[this.UList[fromy].UNr].Name;
                this.AddBiggy(s);
                this.UList[fromy].URetreat = this.CombatRound;
                this.UList[fromy].URetreatMode = 1;
                let mut icounter2: i32 =  this.ICounter;
                for (let mut inr: i32 =  0; inr <= icounter2; inr += 1)
                {
                  if (this.IList[inr].IUnr == this.UList[fromy].UNr && this.IList[inr].IRetreated == 0 & this.IList[inr].IRetreat == 0)
                  {
                    this.IList[inr].IRetreat = this.CombatRound;
                    this.IList[inr].IRetreatMode = 2;
                    this.ReduceEntr(inr, (int) Math.Round( this.game.Data.RuleVar[126]));
                    let mut imor: i32 =  this.IList[inr].IMor;
                    this.ReduceMor(inr, (int) Math.Round(Conversion.Int( (100 - Number) / 2.0)));
                    this.AddReport(0, "Morale penalty due to orderly retreat", "Loses morale equal to %=(100-" + Strings.Trim(Conversion.Str( Number)) + ")/2. Results in morale dropping from " + Strings.Trim(Conversion.Str( imor)) + " to " + Strings.Trim(Conversion.Str( this.IList[inr].IMor)) + ".", 10000 + inr, this.CombatRound);
                  }
                }
              }
            }
          }
          else
          {
            title = "Orderly retreat test skipped";
            txt = "No test is done because standing order retreat percentage or type of battle does not allow orderly retreats.";
          }
        }
        else
        {
          title = "Orderly retreat test skipped";
          txt = "No test is done because unit is already retreating or panicking.";
        }
        if (!(!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.UList[fromy].UNr].Regime, this.game.Data.Turn)))
          txt = "details hidden for FOW";
        this.AddReport(1, title, txt, fromy, this.CombatRound);
      }
    }

    pub fn CheckPanicUnitRetreat()
    {
      let mut ucounter: i32 =  this.UCounter;
      for (let mut fromy: i32 =  0; fromy <= ucounter; fromy += 1)
      {
        num1: i32;
        if (!this.UList[fromy].UCanRetreat.onmap)
        {
          num1 = 100 - 2 * this.game.HandyFunctionsObj.GetAverageMor(this.UList[fromy].UNr);
          if (num1 < 0)
            num1 = 0;
        }
        else
          num1 = 100 - this.game.HandyFunctionsObj.GetAverageMor(this.UList[fromy].UNr);
        let mut num2: i32 =  100 - this.game.HandyFunctionsObj.GetAverageBaseMor(this.UList[fromy].UNr);
        if (this.CombatType == 3 | this.CombatType == 4)
          num1 = 0;
        if ((this.CombatType == 5 | this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15) & this.UList[fromy].Uattacker == 0 && this.game.Data.UnitObj[this.UList[fromy].UNr].X == this.TargetX & this.game.Data.UnitObj[this.UList[fromy].UNr].Y == this.TargetY & this.game.Data.UnitObj[this.UList[fromy].UNr].Map == this.TargetMap)
          num1 = 0;
        if ( this.game.Data.RuleVar[996] < 1.0)
          num2 = 0;
        if (num1 > 0 | num2 > 0 && this.UList[fromy].URetreat == 0)
        {
          let mut num3: i32 =  0;
          let mut num4: i32 =  0;
          let mut icounter1: i32 =  this.ICounter;
          for (let mut index: i32 =  0; index <= icounter1; index += 1)
          {
            if (this.IList[index].IUnr == this.UList[fromy].UNr && !this.IList[index].IleftOutOfPartialAttack |  this.game.Data.RuleVar[493] < 1.0)
            {
              let mut num5: i32 =  0;
              if (this.IList[index].IRetreated > 0)
                num5 = 1;
              if (this.IList[index].IRetreat > 0)
                num5 = 1;
              if (this.IList[index].IKilled > 0)
                num5 = 1;
              if (num5 == 0)
                num4 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
              num3 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
            }
          }
          if (num3 == 0)
            num3 = 1;
          if (this.UList[fromy].Uattacker == 0)
            ;
          let mut num6: i32 =  0;
          title: String = "Panic test";
          str1: String = "If % of troops left gets below " + Strings.Trim(Conversion.Str( Math.Round(new Decimal(num1), 1))) + "% then panic test must be made." + "\r\n";
          if ( this.game.Data.RuleVar[996] == 1.0)
            str1 = str1 + "If % of troops left gets below " + Strings.Trim(Conversion.Str( Math.Round(new Decimal(num2), 1))) + "% then a reduced chance panic test must be made." + "\r\n";
          str2: String = str1 + "Current % of troops left is " + Strings.Trim(Conversion.Str( Math.Round( (num4 * 100) /  num3))) + "%.";
          str3: String;
          if ( (num4 * 100) /  num3 <  num1 & (this.game.Data.Product < 6 |  this.game.Data.RuleVar[996] < 1.0))
          {
            num6 = 1;
            str3 = str2 + "\r\n" + "Panic test has to be made. There is a " + Strings.Trim(Conversion.Str( (int) Math.Round( (this.game.Data.RuleVar[70] * 100f)))) + "% chance unit panics.";
            if (num6 == 1 &&  VBMath.Rnd() >=  this.game.Data.RuleVar[70])
              num6 = 0;
          }
          else if ( (num4 * 100) /  num3 <  num2 &  this.game.Data.RuleVar[996] == 1.0 & this.game.Data.Product < 6)
          {
            num6 = 1;
            float num7 =  (50 - Math.Abs(num2 - num1));
            if ( num7 < 20.0)
              num7 = 20f;
            float num8 = num7 / 100f;
            str3 = str2 + "\r\n" + "Reduced Chance Panic test has to be made. There is a " + Strings.Trim(Conversion.Str( (int) Math.Round( this.game.Data.RuleVar[70] *  num8 * 100.0))) + "% chance unit panics.";
            if (num6 == 1 &&  VBMath.Rnd() >=  this.game.Data.RuleVar[70] *  num8)
              num6 = 0;
          }
          else if ( (num4 * 100) /  num3 <  num1 &  this.game.Data.RuleVar[996] == 1.0 & this.game.Data.Product >= 6)
          {
            num6 = 1;
            float num9 =  (30 - (num2 - num1));
            if ( num9 < 10.0)
              num9 = 10f;
            float num10 = num9 / 100f;
            str3 = str2 + "\r\n" + "Panic test has to be made. There is a " + Strings.Trim(Conversion.Str( (int) Math.Round( this.game.Data.RuleVar[70] *  num10 * 100.0))) + "% chance unit panics.";
            if (num6 == 1 &&  VBMath.Rnd() >=  this.game.Data.RuleVar[70] *  num10)
              num6 = 0;
          }
          else
            str3 = str2 + "\r\n" + "Unit does not have to make a panic test.";
          if (num6 == 1 && this.customCombatObj.HasCustumCalls())
          {
            s9: String = "";
            if (this.customCombatObj.UnitCombatCall_AvoidPanic(this.game, this.UList[fromy].UNr, ref s9) > 0 & s9.Length > 0)
            {
              num6 = 0;
              str3 = str3 + "\r\n" + s9;
            }
          }
          txt: String;
          if (num6 == 1)
          {
            s: String = s + "Unit Panics!: " + this.game.Data.UnitObj[this.UList[fromy].UNr].Name;
            str4: String = str3 + "\r\n";
            title = "Panic test : UNIT PANICKS!";
            txt = str4 + "Unit fails panic test and panicks!";
            this.UList[fromy].UPanicked = true;
            this.AddBiggy(s);
            this.UList[fromy].URetreat = this.CombatRound;
            this.UList[fromy].URetreatMode = 5;
            let mut peopleGroup: i32 =  this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.UnitObj[this.UList[fromy].UNr].Regime].People].PeopleGroup;
            let mut icounter2: i32 =  this.ICounter;
            for (let mut inr: i32 =  0; inr <= icounter2; inr += 1)
            {
              if (this.IList[inr].IUnr == this.UList[fromy].UNr && this.IList[inr].IRetreated == 0 & this.IList[inr].IRetreat == 0)
              {
                this.IList[inr].IRetreat = this.CombatRound;
                this.IList[inr].IRetreatMode = 3;
                this.ReduceEntr(inr, (int) Math.Round( this.game.Data.RuleVar[126]));
                this.ReduceMor(inr, (int) Math.Round( this.game.Data.RuleVar[129]));
                let mut num11: i32 =  this.game.Data.PeopleObj[this.game.Data.SFObj[this.IList[inr].ISFNr].People].BaseMorale[peopleGroup];
                if (this.IList[inr].IMor > num11)
                  this.IList[inr].IMor = num11;
              }
            }
          }
          else
            txt = str3 + " Unit does not panic.";
          if (!(!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.UList[fromy].UNr].Regime, this.game.Data.Turn)))
            txt = "Details hidden for FOW";
          this.AddReport(0, title, txt, fromy, this.CombatRound);
        }
      }
    }

    pub fn CheckBreak()
    {
      if ( this.game.Data.RuleVar[354] == 0.0)
        return;
      let mut ucounter: i32 =  this.UCounter;
      for (let mut fromy: i32 =  0; fromy <= ucounter; fromy += 1)
      {
        if (this.UList[fromy].URetreat == this.CombatRound & this.UList[fromy].Uattacker == 0 & !this.game.Data.UnitObj[this.UList[fromy].UNr].IsHQ)
        {
          bool flag = true;
          if (this.game.Data.Product >= 6 && this.UList[fromy].URetreatMode != 5)
          {
            flag = false;
            if (this.game.Data.Product == 6 && this.UList[fromy].URetreatMode > 0)
            {
              let mut num1: i32 =  0;
              let mut icounter: i32 =  this.ICounter;
              for (let mut index: i32 =  0; index <= icounter; index += 1)
              {
                if (this.IList[index].IUlistNr == fromy && this.IList[index].IKilled < 1 & this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 0)
                  num1 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
              }
              if ( num1 <  this.game.Data.RuleVar[476])
              {
                let mut num2: i32 =  (int) Math.Round( ( (100 * num1) / this.game.Data.RuleVar[476]));
                if (DrawMod.RandyNumber.Next(0, 100) > num2)
                  flag = true;
              }
            }
          }
          if (flag)
          {
            let mut num3: i32 =  0;
            let mut num4: i32 =  0;
            let mut icounter1: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter1; index += 1)
            {
              if (this.IList[index].IUlistNr == fromy && this.IList[index].IKilled < 1 & this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 0)
              {
                num3 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
                if (this.game.Data.PeopleObj[this.game.Data.SFObj[this.IList[index].ISFNr].People].BreakAt > -1)
                  num4 += this.game.Data.PeopleObj[this.game.Data.SFObj[this.IList[index].ISFNr].People].BreakAt * this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
              }
            }
            if (num3 > 0)
            {
              title: String = "Breaking test";
              str1: String = "Done in the round retreat starts.";
              let mut Number1: i32 =  (int) Math.Round(Conversion.Int( num4 /  num3));
              let mut num5: i32 =  this.game.HandyFunctionsObj.GetStartPower(this.UList[fromy].UNr);
              let mut unr: i32 =  this.UList[fromy].UNr;
              if (this.game.Data.UnitObj[unr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].ModelMaster > -1 && this.game.Data.UnitObj[unr].HistoricalSubPart > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].SubParts[this.game.Data.UnitObj[unr].HistoricalSubPart] > -1)
              {
                let mut preDef: i32 =  this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].SubParts[this.game.Data.UnitObj[unr].HistoricalSubPart]);
                if (preDef > -1)
                  num5 = this.game.HandyFunctionsObj.GetPowerPtsAbsolute(preDef, true);
              }
              if (this.game.HandyFunctionsObj.CheckIsBattlegroup(this.UList[fromy].UNr))
                num5 = 999999;
              txt: String = str1 + "\r\n" + "Breakpois: i32 at " + Strings.Trim(Conversion.Str( Number1)) + "% left of original size. Current size is " + Strings.Trim(Conversion.Str( (int) Math.Round( (num3 * 100) /  num5))) + "%.";
              if (num5 > 0 && (int) Math.Round( (num3 * 100) /  num5) <= Number1)
              {
                str2: String = txt + "\r\n" + "Unit qualifies for a break-test.";
                let mut num6: i32 =  this.game.HandyFunctionsObj.GetStartPower(this.UList[fromy].UNr);
                if (this.game.HandyFunctionsObj.CheckIsBattlegroup(this.UList[fromy].UNr))
                  num6 = 999999;
                let mut Number2: i32 =  0;
                if (num6 > 0)
                  Number2 = (int) Math.Round( (Number1 - (int) Math.Round( (num3 * 100) /  num6)) * (100.0 /  Number1));
                txt = str2 + "\r\n" + "Chance unit breaks is set at " + Strings.Trim(Conversion.Str( Number2)) + "%.";
                if (this.UList[fromy].URetreatMode == 5)
                {
                  Number2 *= 2;
                  if (Number2 > 100)
                    Number2 = 100;
                  txt = txt + "\r\n" + "Due to unit is in panickin instead of orderly retreating the chance is doubled. Chance is set at " + Strings.Trim(Conversion.Str( Number2)) + "%.";
                }
                if ( VBMath.Rnd() * 100.0 <  Number2)
                {
                  let mut num7: i32 =  0;
                  let mut num8: i32 =  0;
                  let mut icounter2: i32 =  this.ICounter;
                  for (let mut index: i32 =  0; index <= icounter2; index += 1)
                  {
                    if (this.IList[index].IUnr == this.UList[fromy].UNr)
                    {
                      let mut num9: i32 =  0;
                      if (this.IList[index].IKilled > 0)
                        num9 = 1;
                      if (num9 == 0)
                        num8 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
                      num7 += this.game.Data.SFTypeObj[this.IList[index].ISFType].PowerPts;
                    }
                  }
                  if (num8 > 0)
                  {
                    str3: String = txt + "\r\n";
                    title = "Breaking test : UNIT BREAKS";
                    txt = str3 + "Result of random roll is: Unit breaks!";
                    s: String = s + "Unit Breaks: " + this.game.Data.UnitObj[this.UList[fromy].UNr].Name;
                    this.UList[fromy].UBreaks = true;
                    this.AddBiggy(s);
                    this.UList[fromy].URetreat = this.CombatRound;
                    this.UList[fromy].URetreatMode = 5;
                    let mut icounter3: i32 =  this.ICounter;
                    for (let mut inr: i32 =  0; inr <= icounter3; inr += 1)
                    {
                      if (this.IList[inr].IUnr == this.UList[fromy].UNr && this.IList[inr].IKilled == 0)
                      {
                        this.IList[inr].IKilled = this.CombatRound;
                        this.IList[inr].IBroken = true;
                        this.IList[inr].ICapitulate = true;
                        this.IList[inr].IRetreatMode = 3;
                        this.ReduceMor(inr, (int) Math.Round( this.game.Data.RuleVar[129]));
                      }
                    }
                  }
                }
                else
                  txt = txt + "\r\n" + "Result of random roll is: Unit holds!";
              }
              if (!(!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.UList[fromy].UNr].Regime, this.game.Data.Turn)))
                txt = "details hidden for FOW";
              this.AddReport(0, title, txt, fromy, this.CombatRound);
            }
          }
        }
      }
    }

    pub fn FillEditObjHisLoss(let mut forReg: i32 =  -1)
    {
      this.game.EditObj.HisLossCounter = -1;
      this.game.EditObj.HisLossAttacker = new int[1];
      this.game.EditObj.HisLossDEAD = new int[1];
      this.game.EditObj.HisLossOK = new int[1];
      this.game.EditObj.HisLossSFType = new int[1];
      this.game.EditObj.HisRegimeWon = -1;
      if ( this.game.Data.RuleVar[431] < 1.0 | forReg == -1)
      {
        let mut icounter: i32 =  this.ICounter;
        for (let mut index1: i32 =  0; index1 <= icounter; index1 += 1)
        {
          let mut num1: i32 =  -1;
          try
          {
            if (this.game.EditObj.HisLossCounter > -1)
            {
              let mut hisLossCounter: i32 =  this.game.EditObj.HisLossCounter;
              for (let mut index2: i32 =  0; index2 <= hisLossCounter; index2 += 1)
              {
                if (this.game.EditObj.HisLossAttacker[index2] == this.IList[index1].IAttacker & this.game.EditObj.HisLossSFType[index2] == this.IList[index1].ISFType)
                {
                  num1 = index2;
                  break;
                }
              }
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            num1 = -1;
            ProjectData.ClearProjectError();
          }
          if (num1 == -1)
          {
            this += 1.game.EditObj.HisLossCounter;
            if (this.game.EditObj.HisLossCounter == -1)
              this.game.EditObj.HisLossCounter = 0;
            this.game.EditObj.HisLossAttacker = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossAttacker, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossDEAD = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossDEAD, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossOK = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossOK, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossSFType = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossSFType, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossAttacker[this.game.EditObj.HisLossCounter] = this.IList[index1].IAttacker;
            this.game.EditObj.HisLossSFType[this.game.EditObj.HisLossCounter] = this.IList[index1].ISFType;
            num1 = this.game.EditObj.HisLossCounter;
          }
          if (this.IList[index1].IKilled > 0)
          {
            int[] hisLossDead = this.game.EditObj.HisLossDEAD;
            int[] numArray = hisLossDead;
            let mut index3: i32 =  num1;
            let mut index4: i32 =  index3;
            let mut num2: i32 =  hisLossDead[index3] + 1;
            numArray[index4] = num2;
          }
          else
          {
            int[] hisLossOk = this.game.EditObj.HisLossOK;
            int[] numArray = hisLossOk;
            let mut index5: i32 =  num1;
            let mut index6: i32 =  index5;
            let mut num3: i32 =  hisLossOk[index5] + 1;
            numArray[index6] = num3;
          }
        }
      }
      else
      {
        let mut icounter: i32 =  this.ICounter;
        for (let mut index7: i32 =  0; index7 <= icounter; index7 += 1)
        {
          let mut num4: i32 =  this.IList[index7].ISFType;
          if (!(!this.game.Data.FOWOn | this.IList[index7].IvisibleFromRound < this.CombatRound | forReg == this.AttackerRegime & this.IList[index7].IAttacker == 1 | forReg == this.DefenderRegime & this.IList[index7].IAttacker == 0))
          {
            if (!(this.game.Data.Product == 6 & (this.IList[index7].IAttacker == 1 & forReg == this.AttackerRegime | this.IList[index7].IAttacker == 0 & forReg == this.DefenderRegime)))
            {
              if (!(this.InterceptFire & this.IList[index7].IAttacker == 0 & forReg == this.game.Data.Turn & this.DefenderRegime == forReg))
                num4 = -1;
            }
          }
          let mut num5: i32 =  -1;
          try
          {
            if (this.game.EditObj.HisLossCounter > -1)
            {
              let mut hisLossCounter: i32 =  this.game.EditObj.HisLossCounter;
              for (let mut index8: i32 =  0; index8 <= hisLossCounter; index8 += 1)
              {
                if (this.game.EditObj.HisLossAttacker[index8] == this.IList[index7].IAttacker & this.game.EditObj.HisLossSFType[index8] == num4)
                {
                  num5 = index8;
                  break;
                }
              }
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            num5 = -1;
            ProjectData.ClearProjectError();
          }
          if (num5 == -1)
          {
            this += 1.game.EditObj.HisLossCounter;
            if (this.game.EditObj.HisLossCounter == -1)
              this.game.EditObj.HisLossCounter = 0;
            this.game.EditObj.HisLossAttacker = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossAttacker, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossDEAD = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossDEAD, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossOK = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossOK, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossSFType = (int[]) Utils.CopyArray((Array) this.game.EditObj.HisLossSFType, (Array) new int[this.game.EditObj.HisLossCounter + 1]);
            this.game.EditObj.HisLossAttacker[this.game.EditObj.HisLossCounter] = this.IList[index7].IAttacker;
            this.game.EditObj.HisLossSFType[this.game.EditObj.HisLossCounter] = num4;
            num5 = this.game.EditObj.HisLossCounter;
          }
          if (this.IList[index7].IKilled > 0)
          {
            int[] hisLossDead = this.game.EditObj.HisLossDEAD;
            int[] numArray = hisLossDead;
            let mut index9: i32 =  num5;
            let mut index10: i32 =  index9;
            let mut num6: i32 =  hisLossDead[index9] + 1;
            numArray[index10] = num6;
          }
          else
          {
            int[] hisLossOk = this.game.EditObj.HisLossOK;
            int[] numArray = hisLossOk;
            let mut index11: i32 =  num5;
            let mut index12: i32 =  index11;
            let mut num7: i32 =  hisLossOk[index11] + 1;
            numArray[index12] = num7;
          }
        }
      }
    }

    pub fn CheckVictory()
    {
      let mut location1: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].Location;
      num1: i32;
      if (this.game.Data.Product >= 6 &&  this.game.Data.RuleVar[431] > 0.0 && this.CombatType == 3 &&  this.game.Data.RuleVar[55] >  this.game.Data.MapObj[0].HexObj[this.TargetX, this.TargetY].MaxRecon)
      {
        num2: i32;
        num1 = num2 + 1;
      }
      let mut icounter1: i32 =  this.ICounter;
      num3: i32;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IRetreated == 0 & this.IList[index].IKilled == 0 & this.IList[index].IAA <= 0)
        {
          if (this.IList[index].IAttacker == 1)
            num3 += 1;
          else
            num1 += 1;
        }
      }
      if (num1 == 0 & num3 > 0)
      {
        if (this.CombatType == 6 || this.CombatType == 5 &  this.game.Data.RuleVar[341] == 1.0 & this.CombatRound <= 10)
          return;
        if (this.CombatType == 3 | this.CombatType == 4 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].Location > -1 | this.game.Data.Product == 6)
        {
          if (this.game.Data.Product < 6)
            return;
          num1 += 1;
        }
      }
      if (num3 == 0 | num1 == 0)
      {
        let mut dam: i32 =  0 + this.AntiStrucDam;
        if (this.customCombatObj.HasCustumCalls())
        {
          CustomCombatCalls customCombatObj = this.customCombatObj;
          combatClass: CombatClass = this;
          ref local: CombatClass = ref combatClass;
          let mut game: GameClass = this.game;
          let mut antiStrucDam: i32 =  this.AntiStrucDam;
          customCombatObj.StartCombatRound(ref local, game, antiStrucDam);
        }
        if (!this.game.EditObj.CombatSim && dam > 0)
        {
          if (this.customCombatObj.HasCustumCalls())
          {
            CustomCombatCalls customCombatObj = this.customCombatObj;
            combatClass: CombatClass = this;
            ref local: CombatClass = ref combatClass;
            let mut game: GameClass = this.game;
            let mut damPts: i32 =  dam;
            customCombatObj.DoStructuralDamageCall(ref local, game, damPts);
          }
          if (this.CombatType == 6 & this.game.EditObj.OrderBombMode == 1)
            this.game.HandyFunctionsObj.DoStrucPtsDammage(this.TargetX, this.TargetY, this.TargetMap, dam, true, false);
          else if (this.CombatType == 6 & this.game.EditObj.OrderBombMode == 0)
            this.game.HandyFunctionsObj.DoStrucPtsDammage(this.TargetX, this.TargetY, this.TargetMap, dam, both: false);
          else
            this.game.HandyFunctionsObj.DoStrucPtsDammage(this.TargetX, this.TargetY, this.TargetMap, dam);
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].Location == -1 & location1 != -1)
          {
            let mut icounter2: i32 =  this.ICounter;
            for (let mut index: i32 =  0; index <= icounter2; index += 1)
            {
              if (this.IList[index].IAttacker == 0)
              {
                this.IList[index].IEntrench = (int) Math.Round(Conversion.Int( this.IList[index].IEntrench / 3.0));
                if (this.IList[index].IEntrench > this.game.HandyFunctionsObj.GetMaximumEntrench(this.TargetX, this.TargetY, this.game.EditObj.MapSelected, this.game.Data.SFTypeObj[this.IList[index].ISFType].UnitGroup))
                  this.IList[index].IEntrench = this.game.HandyFunctionsObj.GetMaximumEntrench(this.TargetX, this.TargetY, this.game.EditObj.MapSelected, this.game.Data.SFTypeObj[this.IList[index].ISFType].UnitGroup);
              }
            }
            let mut ucounter: i32 =  this.UCounter;
            for (let mut tunr: i32 =  0; tunr <= ucounter; tunr += 1)
            {
              if (this.game.Data.UnitObj[this.UList[tunr].UNr].X == this.TargetX & this.game.Data.UnitObj[this.UList[tunr].UNr].Y == this.TargetY & this.game.Data.UnitObj[this.UList[tunr].UNr].Map == this.TargetMap)
                this.killairandnavy(tunr);
            }
          }
        }
        if (this.CombatType == 6)
        {
          let mut icounter3: i32 =  this.ICounter;
          for (let mut inr: i32 =  0; inr <= icounter3; inr += 1)
          {
            if (this.IList[inr].IAttacker == 1 && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 2 & this.game.Data.SFTypeObj[this.IList[inr].ISFType].AntiStrucPts > 0)
            {
              let mut num4: i32 =  (int) Math.Round(Conversion.Int(160.0 * ( this.UList[this.IList[inr].IUlistNr].UApMoveCost / 100.0)));
              if (num4 > 160)
                num4 = 160;
              let mut pointstot: i32 =  (int) Math.Round(Conversion.Int( (int) Math.Round( (VBMath.Rnd() *  num4)) / 10.0));
              this.AddXp(inr, pointstot);
            }
          }
        }
        if (this.CombatType == 13)
        {
          let mut icounter4: i32 =  this.ICounter;
          for (let mut inr: i32 =  0; inr <= icounter4; inr += 1)
          {
            if (this.IList[inr].IAttacker == 1 && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 2)
            {
              let mut num5: i32 =  (int) Math.Round(Conversion.Int(80.0 * ( this.UList[this.IList[inr].IUlistNr].UApMoveCost / 100.0)));
              if (num5 > 80)
                num5 = 80;
              let mut pointstot: i32 =  (int) Math.Round(Conversion.Int( (int) Math.Round( (VBMath.Rnd() *  num5)) / 10.0));
              this.AddXp(inr, pointstot);
            }
          }
        }
        if (this.CombatType == 9 | this.CombatType == 14 | this.CombatType == 15)
        {
          let mut icounter5: i32 =  this.ICounter;
          for (let mut inr: i32 =  0; inr <= icounter5; inr += 1)
          {
            if (this.IList[inr].IAttacker == 1 && this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 2 & this.game.Data.SFTypeObj[this.IList[inr].ISFType].CarryCap > 0)
            {
              let mut num6: i32 =  (int) Math.Round(Conversion.Int(120.0 * ( this.UList[this.IList[inr].IUlistNr].UApMoveCost / 100.0)));
              if (num6 > 120)
                num6 = 120;
              let mut pointstot: i32 =  (int) Math.Round(Conversion.Int( (int) Math.Round( (VBMath.Rnd() *  num6)) / 10.0));
              this.AddXp(inr, pointstot);
            }
          }
        }
      }
      if ((num3 == 0 | num1 == 0) & this.CombatType == 13 & this.CombatType2 != 16)
      {
        let mut num7: i32 =  0;
        let mut icounter6: i32 =  this.ICounter;
        for (let mut index: i32 =  0; index <= icounter6; index += 1)
        {
          if (this.IList[index].IAttacker == 1 && this.IList[index].IKilled == 0 & (this.IList[index].IRetreatMode == 2 | this.IList[index].IRetreatMode == 0))
            num7 += this.game.Data.SFTypeObj[this.IList[index].ISFType].ReconPts;
        }
        if (num7 > 0)
        {
          this.game.EditObj.CombatOneSentence = "Executed recon mission with " + Strings.Trim(Conversion.Str( num7)) + " recon points.";
          this.game.HandyFunctionsObj.DoAirRecon(this.TargetX, this.TargetY, this.TargetMap, num7, this.UList[0].UNr);
        }
      }
      if ((num3 == 0 | num1 == 0) & this.CombatType2 == 16)
      {
        this.se1carryPointsDelivered = 0;
        this.se1carryPointsTotal = 0;
        this.se1damagePercentage = 0;
        let mut icounter7: i32 =  this.ICounter;
        num8: i32;
        for (let mut index: i32 =  0; index <= icounter7; index += 1)
        {
          if (this.IList[index].IAttacker == 1)
          {
            let mut carryCap: i32 =  this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
            if (this.customCombatObj.HasCustumCalls())
            {
              let mut num9: i32 =  this.customCombatObj.UnitAirBridgeBonus(this.game, this.IList[index].IUnr);
              if (num9 > 0)
                carryCap += (int) Math.Round(Math.Ceiling( (carryCap * num9) / 100.0));
            }
            this.se1carryPointsTotal += carryCap;
            if (this.IList[index].IKilled == 0 & (this.IList[index].IRetreatMode == 2 | this.IList[index].IRetreatMode == 0))
              this.se1carryPointsDelivered += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
            if (this.IList[index].IKilled > 0)
              num8 += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
          }
        }
        if (this.se1carryPointsDelivered < 1)
          this.game.EditObj.CombatOneSentence = "Establishing Air Bridge failed due to enemy fire.";
        else
          this.game.EditObj.CombatOneSentence = "Air Bridge established with " + this.se1carryPointsDelivered.ToString() + " Log.Pts of " + this.se1carryPointsTotal.ToString() + ".";
        if (this.se1carryPointsDelivered > 0)
          this.se1damagePercentage = (int) Math.Round( num8 /  this.se1carryPointsDelivered);
      }
      if ((num3 == 0 | num1 == 0) & this.CombatType == 14)
      {
        let mut num10: i32 =  0;
        let mut icounter8: i32 =  this.ICounter;
        for (let mut index: i32 =  0; index <= icounter8; index += 1)
        {
          if (this.IList[index].IAttacker == 1 && this.IList[index].IKilled == 0 & (this.IList[index].IRetreatMode == 2 | this.IList[index].IRetreatMode == 0))
            num10 += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
        }
        let mut num11: i32 =  (int) Math.Round( Conversion.Int( num10 /  this.game.EditObj.AirSupplyCarry *  this.game.EditObj.AirSupplyPts));
        num12: i32;
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Location > -1)
        {
          let mut location2: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Location;
          num12 = !(this.game.Data.LocTypeObj[this.game.Data.LocObj[location2].Type].IsAirfield | this.game.Data.LocObj[location2].isAirfield) ? (int) Math.Round( num11 * 0.5) : (int) Math.Round( num11 * (0.5 + 0.5 * ( this.game.Data.LocObj[location2].StructuralPts /  this.game.Data.LocTypeObj[this.game.Data.LocObj[location2].Type].StructuralPts)));
        }
        else
          num12 = (int) Math.Round( num11 * 0.5);
        this.game.EditObj.CombatOneSentence = "Executed air supply mission. Delivered:  " + Strings.Trim(Conversion.Str( num12)) + " supply points of " + Strings.Trim(Conversion.Str( this.game.EditObj.AirSupplyPts));
        UnitClass[] unitObj = this.game.Data.UnitObj;
        UnitClass[] unitClassArray = unitObj;
        let mut airSupplyHq: i32 =  this.game.EditObj.AirSupplyHq;
        let mut index1: i32 =  airSupplyHq;
        unitClassArray[index1].Supply = unitObj[airSupplyHq].Supply - this.game.EditObj.AirSupplyPts;
        this.game.HandyFunctionsObj.DisperseAirSupply(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.TargetMap, num12);
      }
      if ((num3 == 0 | num1 == 0) & this.CombatType == 15)
      {
        let mut num13: i32 =  0;
        let mut num14: i32 =  0;
        let mut icounter9: i32 =  this.ICounter;
        for (let mut index: i32 =  0; index <= icounter9; index += 1)
        {
          if (this.IList[index].IAttacker == 1)
          {
            if (!(this.IList[index].IKilled == 0 & (this.IList[index].IRetreatMode == 2 | this.IList[index].IRetreatMode == 0)))
              num13 += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
            num14 += this.game.Data.SFTypeObj[this.IList[index].ISFType].CarryCap;
          }
        }
        float num15 =  num13 /  num14;
        this.game.EditObj.CombatOneSentence = "Executed airlift mission. Lost " + Conversion.Str( Conversion.Int(num15 * 100f)) + "% of passengers.";
        this.game.EditObj.PassengerDammage = num15;
      }
      if (num3 == 0 & num1 > 0)
      {
        if ( this.game.Data.RuleVar[407] > 0.0)
          this.PayForLis();
        if ( this.game.Data.RuleVar[434] > 0.0)
          this.PayForSimplifiedSupplyRules();
        let mut ucounter: i32 =  this.UCounter;
        for (let mut tunr: i32 =  0; tunr <= ucounter; tunr += 1)
        {
          this.ReformUnit(tunr);
          if (this.UList[tunr].Uattacker == 1 & this.UList[tunr].UDead == 0)
          {
            this.AddDetail("Attacking unit retreats");
            this.DoRetreat(tunr);
          }
        }
        this.BattleEnded = 2;
        this.CombatLogBattles(2);
      }
      if (num1 == 0 & num3 > 0)
      {
        this.CombatLogBattles(1);
        if ( this.game.Data.RuleVar[407] > 0.0)
          this.PayForLis();
        if ( this.game.Data.RuleVar[434] > 0.0)
          this.PayForSimplifiedSupplyRules();
        let mut ucounter: i32 =  this.UCounter;
        for (let mut tunr: i32 =  0; tunr <= ucounter; tunr += 1)
        {
          this.ReformUnit(tunr);
          if (this.CombatType == 1 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 9 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 10 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 12 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 2 & this.UList[tunr].Uattacker == 0 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].LandscapeType].IsSea)
            this.killairandnavy(tunr);
          if (this.UList[tunr].Uattacker == 0 & this.UList[tunr].UDead == 0 && !(this.CombatType == 3 | this.CombatType == 4 | this.CombatType == 6 | this.CombatType == 5 | this.CombatType == 13) && !(!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea & this.CombatType == 2))
          {
            this.AddDetail("Defending Unit retreats");
            this.DoRetreat(tunr);
          }
          if (this.UList[tunr].Uattacker == 1 & this.CombatType == 1 & !this.game.EditObj.CombatSim && !this.UList[tunr].UpartialAttack |  this.game.Data.RuleVar[493] < 1.0)
          {
            this.game.Data.UnitObj[this.UList[tunr].UNr].FreeCombatX = this.TargetX;
            this.game.Data.UnitObj[this.UList[tunr].UNr].FreeCombatY = this.TargetY;
            this.game.Data.UnitObj[this.UList[tunr].UNr].FreeCombatMap = this.TargetMap;
          }
        }
        this.BattleEnded = 1;
        if (!this.game.EditObj.CombatSim && this.CombatType == 1 | this.CombatType == 9 | this.CombatType == 10 | this.CombatType == 12 && this.game.HandyFunctionsObj.IsHostileNotSelf(this.AttackerRegime, this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Regime))
        {
          if (this.UList[0].UNr > -1)
            this.game.HandyFunctionsObj.UnitCausesHexOwnershipChange(this.AttackerRegime, this.CombatTarget.x, this.CombatTarget.y, this.game.Data.UnitObj[this.UList[0].UNr].X, this.game.Data.UnitObj[this.UList[0].UNr].Y);
          this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Regime = !( this.game.Data.RuleVar[840] == 1.0 & this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].OrigOwner > -1 & this.game.HandyFunctionsObj.IsAlliedOrSelf(this.AttackerRegime, this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].OrigOwner)) ? (this.game.Data.RegimeObj[this.AttackerRegime].UberRegime != -1 ? this.game.Data.RegimeObj[this.AttackerRegime].UberRegime : this.AttackerRegime) : this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].OrigOwner;
          if (this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].CardUponConquest > -1)
          {
            if (this.game.Data.RegimeObj[this.AttackerRegime].AI)
              this.game.ProcessingObj.PlayCard(this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].CardUponConquest);
            else
              this.game.EditObj.DoCardSlot = this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].CardUponConquest;
          }
          if (this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location > -1)
          {
            this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].HQ = -1;
            if ( this.game.Data.RuleVar[898] > 0.0)
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].StructuralPts = 0;
            let mut index: i32 =  0;
            do
            {
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Production[index] = -1;
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].ProdPercent[index] = 0;
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].ProdPointRemainder[index] = 0;
              index += 1;
            }
            while (index <= 3);
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Type].AutoProd > -1)
            {
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].ProdPercent[0] = 100;
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Production[0] = this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location].Type].AutoProd;
              this.game.HandyFunctionsObj.UpgradeProduction(this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location);
            }
            this.game.ProcessingObj.LocationProductionPrognosis();
          }
          if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[471] > 0.0 && this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2 > -1)
          {
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2].Type].isSupplyBase)
            {
              this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2].supplyBaseMode = 1;
              if (this.game.Data.RegimeObj[this.AttackerRegime].AI)
                this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2].supplyBaseMode = 3;
              if (this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2].supplyBaseFixed == 1)
                this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2].supplyBaseFixed = 0;
            }
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2].Type].isSupplySource)
              this.game.Data.RemoveLoc(this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].Location2);
          }
        }
      }
      if (num3 == 0 & num1 == 0)
      {
        this.CombatLogBattles(3);
        if ( this.game.Data.RuleVar[407] > 0.0)
          this.PayForLis();
        if ( this.game.Data.RuleVar[434] > 0.0)
          this.PayForSimplifiedSupplyRules();
        let mut ucounter: i32 =  this.UCounter;
        for (let mut tunr: i32 =  0; tunr <= ucounter; tunr += 1)
        {
          this.ReformUnit(tunr);
          if (this.CombatType == 1 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 10 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 12 & this.UList[tunr].Uattacker == 0)
            this.killairandnavy(tunr);
          if (this.CombatType == 2 & this.UList[tunr].Uattacker == 0 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.CombatTarget.map].HexObj[this.CombatTarget.x, this.CombatTarget.y].LandscapeType].IsSea)
            this.killairandnavy(tunr);
          if (this.UList[tunr].UDead == 0 && !(!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea & this.CombatType == 2) && this.UList[tunr].Uattacker == 1 | !(this.CombatType == 3 | this.CombatType == 4 | this.CombatType == 6 | this.CombatType == 5 | this.CombatType == 9 | this.CombatType == 13))
          {
            this.AddDetail("Att or Def Unit retreats due to standoff");
            this.DoRetreat(tunr);
          }
        }
        this.BattleEnded = 3;
      }
      if (this.BattleEnded <= 0 || !this.customCombatObj.HasCustumCalls())
        return;
      CustomCombatCalls customCombatObj1 = this.customCombatObj;
      combatClass1: CombatClass = this;
      ref local1: CombatClass = ref combatClass1;
      let mut game1: GameClass = this.game;
      customCombatObj1.EndCombatCall(ref local1, game1);
    }

    pub fn CombatLogBattles(tBattleEnded: i32)
    {
      if ( this.game.Data.RuleVar[955] <= 0.0)
        return;
      num: i32;
      if (tBattleEnded == 1)
        num = this.AttackerRegime;
      if (tBattleEnded == 2)
        num = this.DefenderRegime;
      if (tBattleEnded == 3)
        num = -1;
      let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[955]));
      if (stringListById == -1)
        return;
      this.game.Data.StringListObj[stringListById].AddRow(this.game.Data.StringListObj[stringListById].Length);
      let mut length: i32 =  this.game.Data.StringListObj[stringListById].Length;
      this.game.Data.StringListObj[stringListById].SetItem(length, 0, this.game.Data.CombatLogId.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 1, this.AttackerRegime.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 2, this.TargetX.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 3, this.TargetY.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 4, this.DefenderRegime.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 5, Conversions.ToString(num));
      this.game.Data.StringListObj[stringListById].SetItem(length, 6, this.CombatRound.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 7, this.initialRecon.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 8, this.CombatType.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 9, this.game.Data.Round.ToString());
      this.game.Data.StringListObj[stringListById].SetItem(length, 10, ((int) Math.Round( (this.bestConcentricBonus * 100f))).ToString());
    }

    pub fn SetConcentricBonus()
    {
      Neighbours neighbours = Neighbours::new();
      UnitList UL = UnitList::new();
      this.ConcentricBonus = 1f;
      if (this.CombatType == 1)
      {
        let mut ucounter: i32 =  this.UCounter;
        for (let mut index: i32 =  0; index <= ucounter; index += 1)
        {
          if (this.UList[index].Uattacker == 1 & this.UList[index].URetreat == 0)
          {
            let mut num: i32 =  this.game.HandyFunctionsObj.HexFacing(this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map, this.game.Data.UnitObj[this.UList[index].UNr].X, this.game.Data.UnitObj[this.UList[index].UNr].Y, this.game.Data.UnitObj[this.UList[index].UNr].Map);
            if (num > 0)
              neighbours.data[num - 1] = 1;
            UL.add(this.UList[index].UNr);
          }
        }
      }
      bool flag1 = false;
      if (this.AttackerRegime > -1)
        flag1 = this.game.Data.RegimeObj[this.AttackerRegime].AI;
      if (Operators.ConditionalCompareObjectGreater(this.game.HandyFunctionsObj.GetHQsInUnitList(ref UL),  1, false))
      {
        handyFunctionsObj: HandyFunctionsclass = this.game.HandyFunctionsObj;
        ref Neighbours local1 = ref neighbours;
        bool flag2 = true;
        ref bool local2 = ref flag2;
        let mut num: i32 =  flag1 ? 1 : 0;
        this.ConcentricBonus = handyFunctionsObj.GetConcentricBonus(ref local1, ref local2, num != 0);
      }
      else
      {
        handyFunctionsObj: HandyFunctionsclass = this.game.HandyFunctionsObj;
        ref Neighbours local3 = ref neighbours;
        bool flag3 = false;
        ref bool local4 = ref flag3;
        let mut num: i32 =  flag1 ? 1 : 0;
        this.ConcentricBonus = handyFunctionsObj.GetConcentricBonus(ref local3, ref local4, num != 0);
      }
      if ( this.ConcentricBonus <=  this.bestConcentricBonus)
        return;
      this.bestConcentricBonus = this.ConcentricBonus;
    }

    pub fn EndBattle()
    {
      UnitList UL = UnitList::new();
      this.game.EditObj.se1_map_data3cache_set = false;
      if (this.customCombatObj.HasCustumCalls())
      {
        CustomCombatCalls customCombatObj = this.customCombatObj;
        combatClass: CombatClass = this;
        ref local: CombatClass = ref combatClass;
        let mut game: GameClass = this.game;
        customCombatObj.EndBattleCall(ref local, game);
      }
      bool flag1 = false;
      if (this.BattleEnded > 0)
      {
        if ( this.game.Data.RuleVar[431] < 1.0)
        {
          if (this.game.Data.Product == 6)
          {
            if (!this.allowHistoryOwnRegime)
            {
              if (this.AttackerRegime == this.game.Data.Turn)
                this.FillEditObjHisLoss(this.DefenderRegime);
              else
                this.FillEditObjHisLoss(this.AttackerRegime);
            }
          }
          else
            this.FillEditObjHisLoss();
        }
        this.game.EditObj.HisLossAttReg = this.AttackerRegime;
        this.game.EditObj.HisLossDefReg = this.DefenderRegime;
        if (this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 10 | this.CombatType == 9 | this.CombatType == 12)
        {
          if (this.BattleEnded == 1)
            this.game.EditObj.HisRegimeWon = this.AttackerRegime;
          if (this.BattleEnded == 2)
            this.game.EditObj.HisRegimeWon = this.DefenderRegime;
          if (this.BattleEnded == 3)
            this.game.EditObj.HisRegimeWon = -1;
        }
        else
          this.game.EditObj.HisRegimeWon = -1;
        if ( this.game.Data.RuleVar[485] > 0.0 & (this.CombatType == 3 | this.CombatType == 4))
        {
          let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[485]));
          if (stringListById > -1)
          {
            bool[] flagArray = new bool[this.UCounter + 10 + 1];
            this.game.HandyFunctionsObj.ClearFireListRecent();
            let mut ucounter1: i32 =  this.UCounter;
            for (let mut index1: i32 =  0; index1 <= ucounter1; index1 += 1)
            {
              if (this.UList[index1].Uattacker == 1 & !flagArray[index1] & this.UList[index1].UNr > -1)
              {
                SimpleList simpleList = SimpleList::new();
                bool flag2 = false;
                let mut ucounter2: i32 =  this.UCounter;
                for (let mut index2: i32 =  0; index2 <= ucounter2; index2 += 1)
                {
                  if (this.UList[index2].UNr > -1 && this.game.Data.UnitObj[this.UList[index1].UNr].X == this.game.Data.UnitObj[this.UList[index2].UNr].X && this.game.Data.UnitObj[this.UList[index1].UNr].Y == this.game.Data.UnitObj[this.UList[index2].UNr].Y && !flagArray[index2])
                  {
                    let mut icounter: i32 =  this.game.TempCombat.ICounter;
                    for (let mut index3: i32 =  0; index3 <= icounter; index3 += 1)
                    {
                      if (this.game.TempCombat.IList[index3].IUlistNr == index2)
                      {
                        let mut attackCount: i32 =  this.game.TempCombat.IList[index3].AttackCount;
                        if (attackCount > 0)
                        {
                          let mut reinforcementType: i32 =  this.game.Data.SFTypeObj[this.game.TempCombat.IList[index3].ISFType].ReinforcementType;
                          if (reinforcementType > -1)
                            simpleList.AddWeight(reinforcementType, attackCount);
                          if (this.IList[index3].IvisibleFromRound <= this.CombatRound)
                            flag2 = true;
                        }
                      }
                    }
                    flagArray[index2] = true;
                  }
                }
                if (simpleList.Counter > -1)
                {
                  simpleList.ReverseSortHighSpeed();
                  str1: String = this.game.Data.ReinfName[simpleList.Id[0]];
                  str2: String = simpleList.Counter != 0 ? str1 : str1;
                  this.game.Data.StringListObj[stringListById].AddRowWithData(this.AttackerRegime.ToString(), this.TargetX.ToString(), this.TargetY.ToString(), "2", this.AttackerRegime.ToString(), this.DefenderRegime.ToString(), "1", this.game.Data.Round.ToString(), this.game.Data.Turn.ToString(), str2 + " fire from " + this.game.Data.UnitObj[this.UList[index1].UNr].X.ToString() + "," + this.game.Data.UnitObj[this.UList[index1].UNr].Y.ToString());
                  this.game.Data.StringListObj[stringListById].AddRowWithData(this.AttackerRegime.ToString(), this.game.Data.UnitObj[this.UList[index1].UNr].X.ToString(), this.game.Data.UnitObj[this.UList[index1].UNr].Y.ToString(), "1", this.AttackerRegime.ToString(), this.DefenderRegime.ToString(), "1", this.game.Data.Round.ToString(), this.game.Data.Turn.ToString(), str2 + " fire on " + this.TargetX.ToString() + "," + this.TargetY.ToString());
                  if (flag2)
                  {
                    this.game.Data.StringListObj[stringListById].AddRowWithData(this.DefenderRegime.ToString(), this.TargetX.ToString(), this.TargetY.ToString(), "2", this.AttackerRegime.ToString(), this.DefenderRegime.ToString(), "1", this.game.Data.Round.ToString(), this.game.Data.Turn.ToString(), str2 + " fire from " + this.game.Data.UnitObj[this.UList[index1].UNr].X.ToString() + "," + this.game.Data.UnitObj[this.UList[index1].UNr].Y.ToString());
                    this.game.Data.StringListObj[stringListById].AddRowWithData(this.DefenderRegime.ToString(), this.game.Data.UnitObj[this.UList[index1].UNr].X.ToString(), this.game.Data.UnitObj[this.UList[index1].UNr].Y.ToString(), "1", this.AttackerRegime.ToString(), this.DefenderRegime.ToString(), "1", this.game.Data.Round.ToString(), this.game.Data.Turn.ToString(), str2 + " fire on " + this.TargetX.ToString() + "," + this.TargetY.ToString());
                  }
                  else
                    this.game.Data.StringListObj[stringListById].AddRowWithData(this.DefenderRegime.ToString(), this.TargetX.ToString(), this.TargetY.ToString(), "2", this.AttackerRegime.ToString(), this.DefenderRegime.ToString(), "1", this.game.Data.Round.ToString(), this.game.Data.Turn.ToString(), str2 + " fire from unknown hex");
                }
              }
            }
            this.game.HandyFunctionsObj.ResetFireListCache();
          }
        }
        let mut ucounter3: i32 =  this.UCounter;
        for (let mut index4: i32 =  0; index4 <= ucounter3; index4 += 1)
        {
          let mut unr: i32 =  this.UList[index4].UNr;
          UnitClass[] unitObj1 = this.game.Data.UnitObj;
          UnitClass[] unitClassArray1 = unitObj1;
          let mut index5: i32 =  unr;
          let mut index6: i32 =  index5;
          unitClassArray1[index6].uncertaintyRolls = unitObj1[index5].uncertaintyRolls + this.UList[index4].UDice;
          UnitClass[] unitObj2 = this.game.Data.UnitObj;
          UnitClass[] unitClassArray2 = unitObj2;
          let mut index7: i32 =  unr;
          let mut index8: i32 =  index7;
          unitClassArray2[index8].uncertaintyRollsCount = unitObj2[index7].uncertaintyRollsCount + 1;
        }
        let mut ucounter4: i32 =  this.UCounter;
        for (let mut index: i32 =  0; index <= ucounter4; index += 1)
        {
          if (this.UList[index].UDead == 0)
          {
            let mut unr: i32 =  this.UList[index].UNr;
            for (let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount; sfCount >= 0; sfCount += -1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[sfCount];
              if (this.game.Data.SFObj[sf].Qty < 1)
                this.game.Data.UnitObj[unr].RemoveSF(sf);
            }
          }
        }
        let mut ucounter5: i32 =  this.UCounter;
        for (let mut index: i32 =  0; index <= ucounter5; index += 1)
        {
          if (this.UList[index].UDead == 0 & this.UList[index].Uattacker == 0)
            this.game.HandyFunctionsObj.SetOnlyZOCAround(this.game.Data.UnitObj[this.UList[index].UNr].X, this.game.Data.UnitObj[this.UList[index].UNr].Y, this.game.Data.UnitObj[this.UList[index].UNr].Map, this.game.Data.UnitObj[this.UList[index].UNr].Regime, this.UList[index].UNr);
        }
        let mut ucounter6: i32 =  this.UCounter;
        for (let mut index: i32 =  0; index <= ucounter6; index += 1)
        {
          if (this.UList[index].UDead == 0 & this.UList[index].Uattacker == 1)
          {
            if (!(this.CombatType == 9 & !this.UList[index].UParadropper) & this.CombatType != 13 & this.CombatType != 5 & this.CombatType != 6)
              this.game.HandyFunctionsObj.SetOnlyZOCAround(this.game.Data.UnitObj[this.UList[index].UNr].X, this.game.Data.UnitObj[this.UList[index].UNr].Y, this.game.Data.UnitObj[this.UList[index].UNr].Map, this.AttackerRegime, this.UList[index].UNr);
            if (this.CombatType == 9 & this.UList[index].UParadropper | this.CombatType == 10 | this.CombatType == 12)
              this.game.HandyFunctionsObj.SetOnlyReconAround(this.game.Data.UnitObj[this.UList[index].UNr].X, this.game.Data.UnitObj[this.UList[index].UNr].Y, this.game.Data.UnitObj[this.UList[index].UNr].Map, this.game.Data.UnitObj[this.UList[index].UNr].Regime, this.UList[index].UNr);
          }
        }
        let mut ucounter7: i32 =  this.UCounter;
        for (let mut index: i32 =  0; index <= ucounter7; index += 1)
        {
          if (this.UList[index].UDead == 0)
          {
            this.UList[index].UDead = this.game.HandyFunctionsObj.CheckCargoLoss(this.UList[index].UNr, ref UL, this.AttackerRegime, this.DefenderRegime);
          }
          else
          {
            this.UList[index].UDead = this.game.HandyFunctionsObj.CheckCargoLoss(this.UList[index].UNr, ref UL, this.AttackerRegime, this.DefenderRegime);
            this.UList[index].UDead = 1;
          }
        }
        if (this.game.Data.Turn == this.AttackerRegime)
        {
          let mut index9: i32 =  -1;
          let mut ucounter8: i32 =  this.UCounter;
          for (let mut index10: i32 =  0; index10 <= ucounter8; index10 += 1)
          {
            if (this.UList[index10].Uattacker == 1 & this.UList[index10].UDead == 0)
            {
              index9 = this.UList[index10].UNr;
              this.game.SelectX = this.game.Data.UnitObj[index9].X;
              this.game.SelectY = this.game.Data.UnitObj[index9].Y;
              this.game.EditObj.MapSelected = this.game.Data.UnitObj[index9].Map;
            }
          }
          let mut ucounter9: i32 =  this.UCounter;
          for (let mut index11: i32 =  0; index11 <= ucounter9; index11 += 1)
          {
            if (this.UList[index11].Uattacker == 1 & this.UList[index11].UDead == 0 & this.UList[index11].URetreat == 0)
            {
              index9 = this.UList[index11].UNr;
              this.game.SelectX = this.game.Data.UnitObj[index9].X;
              this.game.SelectY = this.game.Data.UnitObj[index9].Y;
              this.game.EditObj.MapSelected = this.game.Data.UnitObj[index9].Map;
            }
          }
          if (index9 > -1)
          {
            let mut num: i32 =  0;
            this.game.EditObj.UnitSelected = index9;
            while (index9 != this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, 1))
            {
              num += 1;
              if (num > 20)
                break;
            }
          }
        }
        this.game.HandyFunctionsObj.CheckAttachTransportValidity();
        if (this.game.Data.Product == 6)
        {
          let mut ucounter10: i32 =  this.UCounter;
          for (let mut index: i32 =  0; index <= ucounter10; index += 1)
          {
            let mut unr: i32 =  this.UList[index].UNr;
            if (this.game.Data.UnitObj[unr].Regime != this.game.Data.Turn)
            {
              Coordinate reconMinusHide;
              if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn | !this.game.Data.FOWOn | this.game.Data.Round == 0)
                reconMinusHide.x = 3;
              else
                reconMinusHide = this.game.HandyFunctionsObj.GetReconMinusHide(unr, this.game.Data.Turn);
              if (this.UList[index].UVisibility.x < 1 & reconMinusHide.x > 0)
                flag1 = true;
              else if (this.UList[index].UVisibility.x > 0 & reconMinusHide.x < 1)
                flag1 = true;
              else if (this.CombatType == 11)
                flag1 = true;
              else if (this.UList[index].URetreated > 0)
                flag1 = true;
            }
          }
        }
        if (this.CombatType == 11)
        {
          let mut ucounter11: i32 =  this.UCounter;
          for (let mut index: i32 =  0; index <= ucounter11; index += 1)
          {
            if (this.UList[index].Uattacker == 0 & this.UList[index].UDead == 0)
              this.game.Data.UnitObj[this.UList[index].UNr].Spotted = true;
          }
        }
        let mut ucounter12: i32 =  this.UCounter;
        for (let mut index: i32 =  0; index <= ucounter12; index += 1)
        {
          if (this.UList[index].Uattacker == 0 && this.UList[index].USetToSpottedAtEnd)
            this.game.Data.UnitObj[this.UList[index].UNr].Spotted = true;
          if (this.UList[index].Uattacker == 0)
            this.game.ProcessingObj.SpottedAndIdentifiedUpdate(this.AttackerRegime, this.UList[index].UNr, true);
          else if (this.UList[index].Uattacker != 1)
            ;
        }
        for (let mut unitCounter: i32 =  this.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
        {
          if (this.game.Data.UnitObj[unitCounter].PassengerCounter > -1)
          {
            for (let mut passengerCounter: i32 =  this.game.Data.UnitObj[unitCounter].PassengerCounter; passengerCounter >= 0; passengerCounter += -1)
            {
              if (UL.CheckIfPresent(this.game.Data.UnitObj[unitCounter].PassengerList[passengerCounter]))
                this.game.Data.UnitObj[unitCounter].RemovePassenger(this.game.Data.UnitObj[unitCounter].PassengerList[passengerCounter]);
            }
          }
        }
        for (let mut unitCounter: i32 =  this.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
        {
          if (UL.CheckIfPresent(unitCounter))
          {
            this.KillUnit(unitCounter);
            flag1 = true;
          }
          else if (unitCounter == this.game.EditObj.OrderTarget & this.CombatType == 15)
          {
            if ( this.game.EditObj.PassengerDammage > 0.0)
              this.game.HandyFunctionsObj.DoLoseCargo(this.game.EditObj.OrderTarget, this.game.EditObj.PassengerDammage, this.AttackerRegime, this.DefenderRegime);
            if (this.game.Data.UnitObj[this.game.EditObj.OrderTarget].SFCount == -1)
            {
              if (this.customCombatObj.HasCustumCalls())
              {
                CustomCombatCalls customCombatObj = this.customCombatObj;
                combatClass: CombatClass = this;
                ref local: CombatClass = ref combatClass;
                let mut game: GameClass = this.game;
                let mut orderTarget: i32 =  this.game.EditObj.OrderTarget;
                customCombatObj.UnitLost(ref local, game, orderTarget);
              }
              this.game.Data.RemoveUnit(this.game.EditObj.OrderTarget, ref this.game);
            }
            else
            {
              this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[this.game.Data.UnitObj[this.game.EditObj.OrderTarget].X, this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Y].RemoveUnitFromList(this.game.EditObj.OrderTarget);
              this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].AddUnitToList(this.game.EditObj.OrderTarget);
              this.game.Data.UnitObj[this.game.EditObj.OrderTarget].X = this.game.EditObj.TargetX;
              this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Y = this.game.EditObj.TargetY;
              this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Map = this.game.EditObj.TargetMap;
              let mut sfCount1: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderTarget].SFCount;
              for (let mut index: i32 =  0; index <= sfCount1; index += 1)
              {
                let mut sf: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderTarget].SFList[index];
                this.game.Data.SFObj[sf].Ap = 0;
                this.game.Data.SFObj[sf].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(this.game.Data.UnitObj[this.game.EditObj.OrderTarget].X, this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Y, this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Map, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup);
                this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
                this.game.Data.SFObj[sf].EP = 0;
              }
              this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y].RemoveUnitFromList(this.game.EditObj.OrderUnit);
              this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].AddUnitToList(this.game.EditObj.OrderUnit);
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X = this.game.EditObj.TargetX;
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y = this.game.EditObj.TargetY;
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map = this.game.EditObj.TargetMap;
              let mut sfCount2: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].SFCount;
              for (let mut index: i32 =  0; index <= sfCount2; index += 1)
              {
                let mut sf: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].SFList[index];
                this.game.Data.SFObj[sf].Ap = 0;
                this.game.Data.SFObj[sf].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup);
                this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
                this.game.Data.SFObj[sf].EP = 0;
              }
            }
          }
          else
          {
            let mut ucounter13: i32 =  this.UCounter;
            for (let mut index: i32 =  0; index <= ucounter13; index += 1)
            {
              if (this.UList[index].UDead == 1 & this.UList[index].UNr == unitCounter)
              {
                this.KillUnit(unitCounter);
                flag1 = true;
              }
            }
          }
        }
        if (this.CombatType == 11 &&  this.game.Data.RuleVar[431] < 1.0)
        {
          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].MaxRecon = 999;
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
          {
            if (index != this.game.Data.Turn && this.game.HandyFunctionsObj.IsAlliedOrSelf(index, this.game.Data.Turn, true))
              this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].set_ReconPts(index, 999);
          }
        }
        if (!(this.CombatType == 3 | this.CombatType == 4 | this.CombatType == 13) &&  this.game.Data.RuleVar[431] < 1.0)
        {
          let mut num: i32 =  this.CombatRound;
          if (num > 10)
            num = 10;
          HexClass[,] hexObj = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj;
          HexClass[,] hexClassArray = hexObj;
          let mut targetX: i32 =  this.TargetX;
          let mut index12: i32 =  targetX;
          let mut targetY: i32 =  this.TargetY;
          let mut index13: i32 =  targetY;
          hexClassArray[index12, index13].MaxRecon = (int) Math.Round( hexObj[targetX, targetY].MaxRecon + Conversion.Int( this.game.Data.RuleVar[56] * ( num / 10.0)));
          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].set_SeeNow(this.AttackerRegime, 1);
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut index14: i32 =  0; index14 <= regimeCounter; index14 += 1)
          {
            if (index14 != this.game.Data.Turn && this.game.HandyFunctionsObj.IsAlliedOrSelf(index14, this.game.Data.Turn, true) && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].MaxRecon > this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].get_ReconPts(index14))
              this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].set_ReconPts(index14, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].MaxRecon);
          }
        }
        if ( this.game.Data.RuleVar[431] < 1.0)
        {
          if (!this.game.EditObj.CombatSim)
          {
            if (this.CombatType == 12 | this.InterceptFire | this.CombatType2 == 16)
              this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[0].x, this.HexList.coord[0].y, this.HexList.coord[0].map, -1, 4, infostring: this.game.EditObj.CombatOneSentence, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
            else
              this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[0].x, this.HexList.coord[0].y, this.HexList.coord[0].map, this.AttackerRegime, 4, infostring: this.game.EditObj.CombatOneSentence, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
            this.game.EditObj.HisLossCounter = -1;
          }
          if (!this.game.EditObj.CombatSim && this.HexList.counter > -1)
          {
            this += 1.game.Data.StepNr;
            let mut counter: i32 =  this.HexList.counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (this.CombatType == 12 | this.InterceptFire | this.CombatType2 == 16)
                this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[index].x, this.HexList.coord[index].y, this.HexList.coord[index].map, -1, 4, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
              else if (this.CombatType2 == 16)
                this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[index].x, this.HexList.coord[index].y, this.HexList.coord[index].map, -1, 4, attacktype: 55, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
              else
                this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[index].x, this.HexList.coord[index].y, this.HexList.coord[index].map, this.AttackerRegime, 4, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
            }
          }
        }
        else
        {
          if (!this.game.EditObj.CombatSim & this.game.Data.Product != 6 && this.HexList.counter > -1)
            this += 1.game.Data.StepNr;
          let mut num1: i32 =  1;
          do
          {
            let mut onlyForReg: i32 =  -1;
            if (num1 == 1)
            {
              this.FillEditObjHisLoss(this.AttackerRegime);
              onlyForReg = this.AttackerRegime;
            }
            if (num1 == 2)
            {
              this.FillEditObjHisLoss(this.DefenderRegime);
              onlyForReg = this.DefenderRegime;
            }
            this.game.EditObj.HisLossAttReg = this.AttackerRegime;
            this.game.EditObj.HisLossDefReg = this.DefenderRegime;
            if (this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 10 | this.CombatType == 9 | this.CombatType == 12)
            {
              if (this.BattleEnded == 1)
                this.game.EditObj.HisRegimeWon = this.AttackerRegime;
              if (this.BattleEnded == 2)
                this.game.EditObj.HisRegimeWon = this.DefenderRegime;
              if (this.BattleEnded == 3)
                this.game.EditObj.HisRegimeWon = -1;
            }
            else
              this.game.EditObj.HisRegimeWon = -1;
            if (!(num1 == 1 & this.AttackerRegime == this.game.Data.Turn & this.InterceptFire) & !(num1 == 2 & this.DefenderRegime == this.game.Data.Turn & this.InterceptFire) && !this.game.EditObj.CombatSim)
            {
              if (this.CombatType == 12 | this.InterceptFire)
                this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[0].x, this.HexList.coord[0].y, this.HexList.coord[0].map, -1, 4, infostring: this.game.EditObj.CombatOneSentence, onlyForReg: onlyForReg, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
              else if (this.CombatType2 == 16)
                this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[0].x, this.HexList.coord[0].y, this.HexList.coord[0].map, -1, 4, attacktype: 55, infostring: this.game.EditObj.CombatOneSentence, onlyForReg: onlyForReg, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
              else
                this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[0].x, this.HexList.coord[0].y, this.HexList.coord[0].map, this.AttackerRegime, 4, infostring: this.game.EditObj.CombatOneSentence, onlyForReg: onlyForReg, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
              this.game.EditObj.HisLossCounter = -1;
            }
            num1 += 1;
          }
          while (num1 <= 2);
          let mut num2: i32 =  1;
          do
          {
            let mut onlyForReg: i32 =  -1;
            this.game.EditObj.HisLossCounter = -1;
            if (num2 == 1)
              onlyForReg = this.AttackerRegime;
            if (num2 == 2)
              onlyForReg = this.DefenderRegime;
            if (!(num2 == 1 & this.AttackerRegime == this.game.Data.Turn & this.InterceptFire) & !(num2 == 2 & this.DefenderRegime == this.game.Data.Turn & this.InterceptFire) && !this.game.EditObj.CombatSim && this.HexList.counter > -1)
            {
              let mut counter: i32 =  this.HexList.counter;
              for (let mut index: i32 =  0; index <= counter; index += 1)
              {
                if (this.CombatType == 12 | this.InterceptFire)
                  this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[index].x, this.HexList.coord[index].y, this.HexList.coord[index].map, -1, 4, onlyForReg: onlyForReg, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
                else if (this.CombatType == 16)
                  this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[index].x, this.HexList.coord[index].y, this.HexList.coord[index].map, -1, 4, attacktype: 55, onlyForReg: onlyForReg, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
                else
                  this.game.HandyFunctionsObj.HistoryAddHex(this.HexList.coord[index].x, this.HexList.coord[index].y, this.HexList.coord[index].map, this.AttackerRegime, 4, onlyForReg: onlyForReg, allowAddedForCurrentTurn: this.allowHistoryOwnRegime);
              }
            }
            num2 += 1;
          }
          while (num2 <= 2);
        }
        if ( this.game.Data.RuleVar[428] > 0.0)
        {
          this.game.EditObj.tempDidInterceptList = CoordList::new();
          if (this.InterceptFire)
          {
            let mut counter: i32 =  this.HexList.counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (this.game.Data.MapObj[0].HexObj[this.HexList.coord[index].x, this.HexList.coord[index].y].Regime == this.AttackerRegime && this.game.Data.MapObj[0].HexObj[this.HexList.coord[index].x, this.HexList.coord[index].y].get_ReconPts(this.DefenderRegime) > 0)
                this.game.EditObj.tempDidInterceptList.AddCoord(this.HexList.coord[index].x, this.HexList.coord[index].y, 0);
            }
          }
        }
      }
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut map: i32 =  0; map <= mapCounter; map += 1)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[map].MapWidth;
        for (let mut x: i32 =  0; x <= mapWidth; x += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[map].MapHeight;
          for (let mut y: i32 =  0; y <= mapHeight; y += 1)
            this.game.HandyFunctionsObj.DoZOCConquest(x, y, map, this.AttackerRegime);
        }
      }
      let mut icounter1: i32 =  this.ICounter;
      bool flag3;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IAttacker == 0 && this.IList[index].ISFType > -1 && this.game.Data.SFTypeObj[this.IList[index].ISFType].AntiSupply > 0 | this.game.Data.SFTypeObj[this.IList[index].ISFType].AntiSupplySea > 0)
          flag3 = true;
      }
      if (flag3)
        this.game.HandyFunctionsObj.DoAntiInfraDammage(true);
      let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
      for (let mut reg2: i32 =  0; reg2 <= regimeCounter1; reg2 += 1)
      {
        if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.AttackerRegime, reg2) &&  this.game.Data.RuleVar[859] != 1.0 & !this.game.EditObj.CombatSim)
        {
          HexClass[,] hexObj1 = this.game.Data.MapObj[this.CombatTarget.map].HexObj;
          HexClass[,] hexClassArray1 = hexObj1;
          let mut x1: i32 =  this.CombatTarget.x;
          let mut index15: i32 =  x1;
          let mut y1: i32 =  this.CombatTarget.y;
          let mut index16: i32 =  y1;
          HexClass hexClass1 = hexClassArray1[index15, index16];
          let mut Index1: i32 =  reg2;
          let mut Index2: i32 =  Index1;
          let mut num3: i32 =  hexObj1[x1, y1].get_BattleStack(Index1) + this.NewBattleStack;
          hexClass1.set_BattleStack(Index2, num3);
          HexClass[,] hexObj2 = this.game.Data.MapObj[this.CombatTarget.map].HexObj;
          HexClass[,] hexClassArray2 = hexObj2;
          let mut x2: i32 =  this.CombatTarget.x;
          let mut index17: i32 =  x2;
          let mut y2: i32 =  this.CombatTarget.y;
          let mut index18: i32 =  y2;
          HexClass hexClass2 = hexClassArray2[index17, index18];
          let mut Index3: i32 =  reg2;
          let mut Index4: i32 =  Index3;
          let mut num4: i32 =  hexObj2[x2, y2].get_BattleStackArt(Index3) + this.NewBattleStackArt;
          hexClass2.set_BattleStackArt(Index4, num4);
          HexClass[,] hexObj3 = this.game.Data.MapObj[this.CombatTarget.map].HexObj;
          HexClass[,] hexClassArray3 = hexObj3;
          let mut x3: i32 =  this.CombatTarget.x;
          let mut index19: i32 =  x3;
          let mut y3: i32 =  this.CombatTarget.y;
          let mut index20: i32 =  y3;
          HexClass hexClass3 = hexClassArray3[index19, index20];
          let mut Index5: i32 =  reg2;
          let mut Index6: i32 =  Index5;
          let mut num5: i32 =  hexObj3[x3, y3].get_BattleStackAir(Index5) + this.NewBattleStackAir;
          hexClass3.set_BattleStackAir(Index6, num5);
        }
      }
      if ( this.game.Data.RuleVar[843] > 0.0 & !this.game.EditObj.CombatSim)
        this.game.EventRelatedObj.DoCheckSpecificEvent((int) Math.Round( this.game.Data.RuleVar[843]));
      this.game.EditObj.attackTypeOption = 0;
      this.game.EditObj.TempCoordListLastMove = CoordList::new();
      if (flag1 && this.game.Data.Product == 6 &&  this.game.Data.RuleVar[455] > 0.0)
        this.game.HandyFunctionsObj.MakeFuzzyOwner(false, true, this.game.Data.Turn);
      this.WriteToFileLog();
    }

    pub fn ReformUnit(tunr: i32)
    {
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      let mut num4: i32 =  0;
      let mut num5: i32 =  0;
      if (this.game.EditObj.CombatSim)
        return;
      let mut unr: i32 =  this.UList[tunr].UNr;
      let mut index1: i32 =  -1;
      length: i32;
      index2: i32;
      if ( this.game.Data.RuleVar[956] > 0.0)
      {
        index1 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[956]));
        if (index1 > -1)
        {
          this.game.Data.StringListObj[index1].AddRow(this.game.Data.StringListObj[index1].Length);
          length = this.game.Data.StringListObj[index1].Length;
          this.game.Data.StringListObj[index1].SetItem(length, 0, this.game.Data.CombatLogId.ToString());
          if (this.game.Data.UnitObj[unr].Historical > -1)
            this.game.Data.StringListObj[index1].SetItem(length, 1, this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].ID.ToString());
          else
            this.game.Data.StringListObj[index1].SetItem(length, 1, Conversions.ToString(-Conversions.ToDouble(1.ToString())));
          this.game.Data.StringListObj[index1].SetItem(length, 2, this.game.Data.UnitObj[unr].HistoricalSubPart.ToString());
          this.game.Data.StringListObj[index1].SetItem(length, 3, this.game.Data.UnitObj[unr].Regime.ToString());
          this.game.Data.StringListObj[index1].SetItem(length, 4, this.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr).ToString());
          this.game.Data.StringListObj[index1].SetItem(length, 5, this.game.HandyFunctionsObj.GetAverageRdn(unr).ToString());
          StringListClass stringListClass = this.game.Data.StringListObj[index1];
          let mut row: i32 =  length;
          index2 = this.game.HandyFunctionsObj.GetAverageEntrench(unr);
          s: String = index2.ToString();
          stringListClass.SetItem(row, 9, s);
          let mut num6: i32 =  -1;
          if (this.UList[tunr].Uattacker == 1)
            num6 += this.game.HandyFunctionsObj.HexFacing(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.CombatTarget.x, this.CombatTarget.y, this.CombatTarget.map);
          this.game.Data.StringListObj[index1].SetItem(length, 10, num6.ToString());
        }
      }
      if (this.UList[tunr].Uattacker == 1)
      {
        UnitClass[] unitObj = this.game.Data.UnitObj;
        UnitClass[] unitClassArray = unitObj;
        index2 = unr;
        let mut index3: i32 =  index2;
        unitClassArray[index3].offensiveCombat = unitObj[index2].offensiveCombat + this.UList[tunr].UApSpend;
      }
      else
      {
        UnitClass[] unitObj = this.game.Data.UnitObj;
        UnitClass[] unitClassArray = unitObj;
        index2 = unr;
        let mut index4: i32 =  index2;
        unitClassArray[index4].defensiveCombat = unitObj[index2].defensiveCombat + this.UList[tunr].UApSpend;
      }
      let mut num7: i32 =  0;
      let mut num8: i32 =  0;
      let mut num9: i32 =  0;
      let mut num10: i32 =  0;
      let mut averageRdn1: i32 =  this.game.HandyFunctionsObj.GetAverageRdn(unr);
      let mut num11: i32 =  0;
      let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index5: i32 =  0; index5 <= sfCount1; index5 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index5];
        let mut num12: i32 =  0;
        let mut num13: i32 =  0;
        let mut num14: i32 =  0;
        let mut num15: i32 =  0;
        let mut num16: i32 =  0;
        let mut num17: i32 =  0;
        let mut num18: i32 =  0;
        let mut num19: i32 =  0;
        let mut num20: i32 =  0;
        let mut icounter: i32 =  this.ICounter;
        for (let mut inr: i32 =  0; inr <= icounter; inr += 1)
        {
          if (this.IList[inr].IUnr == unr & this.IList[inr].ISFNr == sf)
          {
            num20 = 1;
            num3 += 1;
            num2 += this.IList[inr].ItotalKillsPowerPoints;
            num4 += this.IList[inr].ItotalKills;
            num11 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].PowerPts;
            if (this.IList[inr].IKilled == 0)
            {
              if (this.IList[inr].IAttacker == 1)
                this.IList[inr].IEntrench = !(this.game.Data.SFTypeObj[this.IList[inr].ISFType].ArtRange < 1 | !(this.CombatType == 3 | this.CombatType == 4)) ? this.game.Data.SFObj[this.IList[inr].ISFNr].CurrentEntrench : this.game.HandyFunctionsObj.GetMinimumEntrench(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.SFTypeObj[this.IList[inr].ISFType].UnitGroup);
              let mut num21: i32 =  this.CombatRound;
              if (num21 > 4)
                num21 = 4;
              if (num21 < 2)
                num21 = 2;
              let mut percent: i32 =  num21 * 5;
              if (this.IList[inr].IAttacker == 1 & this.IList[inr].IRetreated == 0)
                this.AddMor(inr, percent);
              num14 += 1;
              num12 += this.IList[inr].IXp;
              num13 += this.IList[inr].IRdn;
              num17 += this.IList[inr].IMor;
              num19 += this.IList[inr].IEntrench;
              num9 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].SupplyCarry;
              num8 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].SupplyCarry;
              num16 = (int) Math.Round( num16 +  this.game.Data.SFObj[this.IList[inr].ISFNr].EP /  this.game.Data.SFObj[this.IList[inr].ISFNr].Qty);
              num15 = (int) Math.Round( num15 +  this.game.Data.SFObj[this.IList[inr].ISFNr].EP /  this.game.Data.SFObj[this.IList[inr].ISFNr].Qty);
              num10 += this.IList[inr].IDammageDone;
            }
            if (this.IList[inr].IKilled > 0)
            {
              num1 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].PowerPts;
              num5 += 1;
              num18 += 1;
              num8 += this.game.Data.SFTypeObj[this.IList[inr].ISFType].SupplyCarry;
              if (this.game.Data.SFObj[this.IList[inr].ISFNr].Qty > 0)
                num16 = (int) Math.Round( num16 +  this.game.Data.SFObj[this.IList[inr].ISFNr].EP /  this.game.Data.SFObj[this.IList[inr].ISFNr].Qty);
              let mut isfType: i32 =  this.IList[inr].ISFType;
              index6: i32;
              index7: i32;
              if (this.IList[inr].IAttacker == 1)
              {
                index6 = this.AttackerRegime;
                index7 = this.DefenderRegime;
              }
              else
              {
                index6 = this.DefenderRegime;
                index7 = this.AttackerRegime;
              }
              if ( this.game.Data.RuleVar[841] == 0.0 & index7 < this.AttackerRegime)
              {
                if (index7 > -1)
                {
                  skills: Vec<i32> = this.game.Data.RegimeObj[index7].SKills;
                  numArray: Vec<i32> = skills;
                  index2 = isfType;
                  let mut index8: i32 =  index2;
                  let mut index9: i32 =  this.game.Data.Round + 1;
                  let mut index10: i32 =  index9;
                  let mut num22: i32 =  skills[index2, index9] + 1;
                  numArray[index8, index10] = num22;
                }
              }
              else if (index7 > -1)
              {
                skills: Vec<i32> = this.game.Data.RegimeObj[index7].SKills;
                numArray: Vec<i32> = skills;
                let mut index11: i32 =  isfType;
                let mut index12: i32 =  index11;
                index2 = this.game.Data.Round;
                let mut index13: i32 =  index2;
                let mut num23: i32 =  skills[index11, index2] + 1;
                numArray[index12, index13] = num23;
              }
              if ( this.game.Data.RuleVar[841] == 0.0 & index6 < this.AttackerRegime)
              {
                sloss: Vec<i32> = this.game.Data.RegimeObj[index6].SLoss;
                numArray: Vec<i32> = sloss;
                let mut index14: i32 =  isfType;
                let mut index15: i32 =  index14;
                index2 = this.game.Data.Round + 1;
                let mut index16: i32 =  index2;
                let mut num24: i32 =  sloss[index14, index2] + 1;
                numArray[index15, index16] = num24;
              }
              else
              {
                sloss: Vec<i32> = this.game.Data.RegimeObj[index6].SLoss;
                numArray: Vec<i32> = sloss;
                let mut index17: i32 =  isfType;
                let mut index18: i32 =  index17;
                index2 = this.game.Data.Round;
                let mut index19: i32 =  index2;
                let mut num25: i32 =  sloss[index17, index2] + 1;
                numArray[index18, index19] = num25;
              }
              if (index6 == this.AttackerRegime)
              {
                sloss: Vec<i32> = this.game.Data.RegimeObj[index6].SLoss;
                numArray: Vec<i32> = sloss;
                let mut index20: i32 =  isfType;
                let mut index21: i32 =  index20;
                index2 = 0;
                let mut index22: i32 =  index2;
                let mut num26: i32 =  sloss[index20, index2] + 1;
                numArray[index21, index22] = num26;
              }
              if (index7 == this.AttackerRegime)
              {
                skills: Vec<i32> = this.game.Data.RegimeObj[index7].SKills;
                numArray: Vec<i32> = skills;
                let mut index23: i32 =  isfType;
                let mut index24: i32 =  index23;
                index2 = 0;
                let mut index25: i32 =  index2;
                let mut num27: i32 =  skills[index23, index2] + 1;
                numArray[index24, index25] = num27;
              }
            }
          }
        }
        if (num14 > 0)
        {
          let mut num28: i32 =  (int) Math.Round(Conversion.Int( num13 /  num14));
          let mut num29: i32 =  (int) Math.Round(Conversion.Int( num12 /  num14));
          let mut num30: i32 =  (int) Math.Round(Conversion.Int( num17 /  num14));
          let mut num31: i32 =  (int) Math.Round(Conversion.Int( num19 /  num14));
          num32: i32;
          if (this.UList[tunr].Uattacker == 1)
          {
            if (this.UList[tunr].UApSpend < this.UList[tunr].UApMoveCost)
              this.UList[tunr].UApSpend = this.UList[tunr].UApMoveCost;
            float Number;
            if (this.InterceptFire)
            {
              num32 = this.game.Data.SFObj[sf].Ap - this.UList[tunr].UApSpend * 2;
              Number =  this.game.Data.RuleVar[436] <= 0.0 ?  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ReadinessLoss * ( this.UList[tunr].UApMoveCost / 100f) :  ( this.game.Data.RuleVar[436] *  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ReadinessLoss * ( this.UList[tunr].UApMoveCost / 100.0));
            }
            else
            {
              num32 = this.game.Data.SFObj[sf].Ap - this.UList[tunr].UApSpend;
              Number =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ReadinessLoss * ( this.UList[tunr].UApMoveCost / 100f);
            }
            let mut num33: i32 =  (int) Math.Round( Conversion.Int(Number));
            float num34 = Number -  num33;
            if ( num34 > 0.0 &&  VBMath.Rnd() <  num34)
              num33 += 1;
            if (this.AttackerRegime > -1 && this.game.Data.RegimeObj[this.AttackerRegime].AI)
            {
              if ( this.game.Data.RuleVar[995] == 1.0)
                num33 = 0;
              else if ( this.game.Data.RuleVar[995] == 1.0)
                num33 = (int) Math.Round( num33 / 2.0);
            }
            num28 -= num33;
          }
          else
            num32 = !this.InterceptFire ? this.game.Data.SFObj[sf].Ap - this.UList[tunr].UApSpend : this.game.Data.SFObj[sf].Ap;
          if (0 > num32)
            num32 = 0;
          if ( this.game.Data.RuleVar[60] >  num28)
            num28 = (int) Math.Round( this.game.Data.RuleVar[60]);
          if (this.UList[tunr].Uattacker == 1 && this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].EndCombatRound > 0 & this.CombatType == 3)
            num32 = 0;
          this.game.Data.SFObj[sf].Qty = num14;
          this.game.Data.SFObj[sf].Rdn = num28;
          if (this.CombatType2 != 16)
            this.game.Data.SFObj[sf].Ap = num32;
          this.game.Data.SFObj[sf].Xp = num29;
          this.game.Data.SFObj[sf].Mor = num30;
          this.game.Data.SFObj[sf].CurrentEntrench = num31;
          if (this.UList[tunr].Uattacker == 1 & this.game.Data.Product >= 6 && !(this.CombatType == 3 | this.CombatType == 4))
            this.game.Data.SFObj[sf].initialEntrench = num31;
          if (num16 > 0)
            this.game.Data.SFObj[sf].EP = num15;
          num7 += 1;
        }
        else if (num18 > 0)
          this.game.Data.SFObj[sf].Qty = 0;
        if (num20 == 0)
        {
          num9 += this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].SupplyCarry * this.game.Data.SFObj[sf].Qty;
          num8 += this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].SupplyCarry * this.game.Data.SFObj[sf].Qty;
        }
      }
      if (this.game.Data.Product >= 6 && 3 == this.CombatType && this.UList[tunr].Uattacker == 0)
      {
        let mut averageRdn2: i32 =  this.game.HandyFunctionsObj.GetAverageRdn(unr);
        let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index26: i32 =  0; index26 <= sfCount2; index26 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index26];
          let mut num35: i32 =  averageRdn1 - averageRdn2;
          if (num35 > 0)
          {
            this.game.Data.SFObj[sf].Ap -= num35 * 4;
            if (this.game.Data.SFObj[sf].Ap < 0)
              this.game.Data.SFObj[sf].Ap = 0;
          }
        }
      }
      if (this.game.Data.Product != 7 &  this.game.Data.RuleVar[497] > 0.0 & num3 > 0)
      {
        float num36 = num2 <= 0 ? 4f :  num1 /  num2;
        let mut num37: i32 =   num36 > 0.25 ? ( num36 > 0.33 ? ( num36 > 0.5 ? ( num36 > 0.66 ? ( num36 >= 1.5 ? ( num36 >= 2.0 ? ( num36 >= 3.0 ? ( num36 >= 4.0 ? 4 : 3) : 2) : 1) : 0) : 1) : 2) : 3) : 4;
        let mut num38: i32 =  0;
        let mut num39: i32 =  num37;
        for (let mut index27: i32 =  1; index27 <= num39; index27 += 1)
          num38 += DrawMod.RandyNumber.Next(1, (int) Math.Round( this.game.Data.RuleVar[497]) + 1);
        let mut num40: i32 =  0;
        if (num3 > 0)
          num40 =  num36 >= 1.0 ? (int) Math.Round( (100 * num5) /  num3) : (int) Math.Round( (100 * num4) /  num3);
        if (num38 > num40)
          num38 = num40;
        if ( num36 > 1.0)
        {
          let mut averageMor: i32 =  this.game.HandyFunctionsObj.GetAverageMor(unr);
          if (DrawMod.RandyNumber.Next(1, 101) <= averageMor)
            num38 = (int) Math.Round( num38 / 2.0);
        }
        if ( num36 >= 1.0)
          num38 = -num38;
        if (this.CombatType == 3 | this.InterceptFire | this.UList[tunr].USupportInterceptFire)
          num38 = (int) Math.Round( num38 / 2.0);
        if (num38 != 0)
        {
          let mut sfCount3: i32 =  this.game.Data.UnitObj[unr].SFCount;
          for (let mut index28: i32 =  0; index28 <= sfCount3; index28 += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index28];
            if (num38 > 0)
            {
              SFClass[] sfObj = this.game.Data.SFObj;
              SFClass[] sfClassArray = sfObj;
              let mut index29: i32 =  sf;
              let mut index30: i32 =  index29;
              sfClassArray[index30].Mor = sfObj[index29].Mor + (int) Math.Round(Math.Ceiling( ((100 - this.game.Data.SFObj[sf].Mor) * num38) / 100.0));
            }
            else
            {
              SFClass[] sfObj = this.game.Data.SFObj;
              SFClass[] sfClassArray = sfObj;
              let mut index31: i32 =  sf;
              let mut index32: i32 =  index31;
              sfClassArray[index32].Mor = sfObj[index31].Mor - (int) Math.Round(Math.Ceiling( (this.game.Data.SFObj[sf].Mor * Math.Abs(num38)) / 100.0));
            }
            if (0 > this.game.Data.SFObj[sf].Mor)
              this.game.Data.SFObj[sf].Mor = 0;
            if (this.game.Data.SFObj[sf].Mor > 100)
              this.game.Data.SFObj[sf].Mor = 100;
          }
          str1: String = "Kills versus losses ratio result in Morale change of ";
          str2: String =  num36 >= 1.0 ? str1 + num38.ToString() + "%" : str1 + "+" + num38.ToString() + "%";
          this.AddBiggy(str2);
          this.AddReport(1, "Kills/Losses ratio morale change", str2, tunr, this.CombatRound);
        }
      }
      if (this.UList[tunr].UAA == 0)
        this.game.Data.UnitObj[unr].Supply = num8 <= 0 ? 0 : (int) Math.Round(Conversion.Int( this.game.Data.UnitObj[unr].Supply * ( num9 /  num8)));
      if (this.game.Data.UnitObj[unr].SFCount > -1)
      {
        let mut num41: i32 =  0;
        let mut sfCount4: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index33: i32 =  0; index33 <= sfCount4; index33 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index33];
          num41 += this.game.Data.SFObj[sf].Qty;
        }
        if (num41 == 0)
        {
          this.AddDetail("We are going to remove unit because no fighters left in it.");
          this.UList[tunr].UDead = 1;
        }
      }
      else if (!(this.CombatType == 3 | this.CombatType == 4 | this.CombatType == 6 | this.CombatType == 5 | this.CombatType == 13) && !(this.BattleEnded != 1 & this.game.Data.UnitObj[unr].IsHQ))
      {
        this.AddDetail("We are going to remove unit because nothing left");
        this.UList[tunr].UDead = 1;
      }
      if (this.UList[tunr].UDead != 1)
        this.game.ProcessingObj.MaxReadinessRule(unr);
      index34: i32;
      if (index1 > -1)
      {
        this.game.Data.StringListObj[index1].SetItem(length, 6, this.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr).ToString());
        StringListClass stringListClass = this.game.Data.StringListObj[index1];
        let mut row: i32 =  length;
        index34 = this.game.HandyFunctionsObj.GetAverageRdn(unr);
        s: String = index34.ToString();
        stringListClass.SetItem(row, 7, s);
        let mut num42: i32 =  0;
        if (this.UList[tunr].UDead == 1)
          num42 = 5;
        if (this.UList[tunr].URetreat > 0 & this.UList[tunr].URetreatMode == 1)
          num42 = 2;
        else if (this.UList[tunr].URetreat > 0)
          num42 = 1;
        if (this.UList[tunr].UPanicked)
          num42 = 3;
        if (this.UList[tunr].UBreaks)
          num42 = 4;
        this.game.Data.StringListObj[index1].SetItem(length, 8, num42.ToString());
      }
      if ( this.game.Data.RuleVar[472] > 0.0 & this.game.Data.Product >= 6)
      {
        let mut averageRdn3: i32 =  this.game.HandyFunctionsObj.GetAverageRdn(unr);
        let mut uinitialRdn: i32 =  this.UList[tunr].UinitialRdn;
        if (averageRdn3 < uinitialRdn)
        {
          let mut num43: i32 =  (int) Math.Round( ((uinitialRdn - averageRdn3) * 100) /  uinitialRdn) * 2;
          if (num43 > 100)
            num43 = 100;
          if (num43 > 0)
          {
            UnitClass[] unitObj = this.game.Data.UnitObj;
            UnitClass[] unitClassArray = unitObj;
            index34 = unr;
            let mut index35: i32 =  index34;
            unitClassArray[index35].apReserve = unitObj[index34].apReserve - (int) Math.Round( (this.game.Data.UnitObj[unr].apReserve * num43) / 100.0);
          }
        }
      }
      if (this.game.Data.Product == 6)
      {
        let mut num44: i32 =  999;
        let mut sfCount5: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index36: i32 =  0; index36 <= sfCount5; index36 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index36];
          if (this.game.Data.SFObj[sf].Ap < num44)
            num44 = this.game.Data.SFObj[sf].Ap;
        }
        if (num44 < 999)
        {
          let mut sfCount6: i32 =  this.game.Data.UnitObj[unr].SFCount;
          for (let mut index37: i32 =  0; index37 <= sfCount6; index37 += 1)
            this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index37]].Ap = num44;
        }
      }
      if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[484] > 0.0 && this.game.Data.UnitObj[unr].moveMode == 1)
        this.game.Data.UnitObj[unr].moveMode = 0;
      if (this.game.Data.Product != 6)
        return;
      let mut num45: i32 =  this.game.HandyFunctionsObj.UnitMaximumFuelReserve(unr);
      if (this.game.Data.UnitObj[unr].Fuel > num45)
        this.game.Data.UnitObj[unr].Fuel = num45;
      let mut num46: i32 =  this.game.HandyFunctionsObj.UnitSupplyStore(unr);
      if (this.game.Data.UnitObj[unr].Supply > num46)
        this.game.Data.UnitObj[unr].Supply = num46;
      if (!( this.game.Data.RuleVar[399] > 0.0 & num1 > 0))
        return;
      let mut num47: i32 =  (int) Math.Round( (100f * this.game.Data.RuleVar[397] *  num1 /  num11));
      let mut num48: i32 =  (int) Math.Round(Math.Floor( num47 / 1000.0));
      let mut num49: i32 =  num47 - num48 * 1000;
      if (DrawMod.RandyNumber.Next(0, 1000) < num49)
        num48 += 1;
      if (num48 > (int) Math.Round( (this.game.Data.RuleVar[396] *  num1)))
        num48 = (int) Math.Round( (this.game.Data.RuleVar[396] *  num1));
      let mut sfCount7: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index38: i32 =  0; index38 <= sfCount7; index38 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index38];
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        index34 = sf;
        let mut index39: i32 =  index34;
        sfClassArray[index39].Vigor = sfObj[index34].Vigor - num48;
        if (this.game.Data.SFObj[sf].Vigor < 1)
          this.game.Data.SFObj[sf].Vigor = 1;
      }
    }

    pub issfthere: bool(sfnr: i32)
    {
      let mut icounter: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter; index += 1)
      {
        if (this.IList[index].ISFNr == sfnr)
          return true;
      }
      return false;
    }

    pub fn killairandnavy(tunr: i32)
    {
      if (this.game.EditObj.CombatSim)
        return;
      let mut unr: i32 =  this.UList[tunr].UNr;
      if (this.UList[tunr].UDefIntercept & this.UList[tunr].UCanRetreat.onmap)
        return;
      let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index1: i32 =  0; index1 <= sfCount1; index1 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index1];
        if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater > 0)
        {
          if (!this.issfthere(sf))
          {
            let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
            for (let mut index2: i32 =  1; index2 <= qty; index2 += 1)
            {
              this += 1.ICounter;
              this.IList = (Individual[]) Utils.CopyArray((Array) this.IList, (Array) new Individual[this.ICounter + 1]);
              this.IList[this.ICounter].IAttacker = 0;
              this.IList[this.ICounter].ISFType = this.game.Data.SFObj[sf].Type;
              this.IList[this.ICounter].ISFNr = sf;
              this.IList[this.ICounter].IKilled = 1;
              this.IList[this.ICounter].IUnr = unr;
              this.IList[this.ICounter].ICapitulate = true;
              let mut isfType: i32 =  this.IList[this.ICounter].ISFType;
              index3: i32;
              index4: i32;
              if (this.IList[this.ICounter].IAttacker == 1)
              {
                index3 = this.AttackerRegime;
                index4 = this.DefenderRegime;
              }
              else
              {
                index3 = this.DefenderRegime;
                index4 = this.AttackerRegime;
              }
              if ( this.game.Data.RuleVar[841] == 0.0 & index4 < this.AttackerRegime)
              {
                skills: Vec<i32> = this.game.Data.RegimeObj[index4].SKills;
                numArray: Vec<i32> = skills;
                let mut index5: i32 =  isfType;
                let mut index6: i32 =  index5;
                let mut index7: i32 =  this.game.Data.Round + 1;
                let mut index8: i32 =  index7;
                let mut num: i32 =  skills[index5, index7] + 1;
                numArray[index6, index8] = num;
              }
              else
              {
                skills: Vec<i32> = this.game.Data.RegimeObj[index4].SKills;
                numArray: Vec<i32> = skills;
                let mut index9: i32 =  isfType;
                let mut index10: i32 =  index9;
                let mut round: i32 =  this.game.Data.Round;
                let mut index11: i32 =  round;
                let mut num: i32 =  skills[index9, round] + 1;
                numArray[index10, index11] = num;
              }
              if ( this.game.Data.RuleVar[841] == 0.0 & index3 < this.AttackerRegime)
              {
                sloss: Vec<i32> = this.game.Data.RegimeObj[index3].SLoss;
                numArray: Vec<i32> = sloss;
                let mut index12: i32 =  isfType;
                let mut index13: i32 =  index12;
                let mut index14: i32 =  this.game.Data.Round + 1;
                let mut index15: i32 =  index14;
                let mut num: i32 =  sloss[index12, index14] + 1;
                numArray[index13, index15] = num;
              }
              else
              {
                sloss: Vec<i32> = this.game.Data.RegimeObj[index3].SLoss;
                numArray: Vec<i32> = sloss;
                let mut index16: i32 =  isfType;
                let mut index17: i32 =  index16;
                let mut round: i32 =  this.game.Data.Round;
                let mut index18: i32 =  round;
                let mut num: i32 =  sloss[index16, round] + 1;
                numArray[index17, index18] = num;
              }
              if (index3 == this.AttackerRegime)
              {
                sloss: Vec<i32> = this.game.Data.RegimeObj[index3].SLoss;
                numArray: Vec<i32> = sloss;
                let mut index19: i32 =  isfType;
                let mut index20: i32 =  index19;
                let mut index21: i32 =  0;
                let mut index22: i32 =  index21;
                let mut num: i32 =  sloss[index19, index21] + 1;
                numArray[index20, index22] = num;
              }
              if (index4 == this.AttackerRegime)
              {
                skills: Vec<i32> = this.game.Data.RegimeObj[index4].SKills;
                numArray: Vec<i32> = skills;
                let mut index23: i32 =  isfType;
                let mut index24: i32 =  index23;
                let mut index25: i32 =  0;
                let mut index26: i32 =  index25;
                let mut num: i32 =  skills[index23, index25] + 1;
                numArray[index24, index26] = num;
              }
            }
          }
          else
          {
            let mut icounter: i32 =  this.ICounter;
            for (let mut index27: i32 =  0; index27 <= icounter; index27 += 1)
            {
              if (this.IList[index27].ISFNr == sf & this.IList[index27].IKilled == 0)
              {
                this.IList[index27].IKilled = 1;
                this.IList[index27].ICapitulate = true;
                let mut isfType: i32 =  this.IList[index27].ISFType;
                index28: i32;
                index29: i32;
                if (this.IList[index27].IAttacker == 1)
                {
                  index28 = this.AttackerRegime;
                  index29 = this.DefenderRegime;
                }
                else
                {
                  index28 = this.DefenderRegime;
                  index29 = this.AttackerRegime;
                }
                if (index29 < this.AttackerRegime &  this.game.Data.RuleVar[841] == 0.0)
                {
                  skills: Vec<i32> = this.game.Data.RegimeObj[index29].SKills;
                  numArray: Vec<i32> = skills;
                  let mut index30: i32 =  isfType;
                  let mut index31: i32 =  index30;
                  let mut index32: i32 =  this.game.Data.Round + 1;
                  let mut index33: i32 =  index32;
                  let mut num: i32 =  skills[index30, index32] + 1;
                  numArray[index31, index33] = num;
                }
                else
                {
                  skills: Vec<i32> = this.game.Data.RegimeObj[index29].SKills;
                  numArray: Vec<i32> = skills;
                  let mut index34: i32 =  isfType;
                  let mut index35: i32 =  index34;
                  let mut round: i32 =  this.game.Data.Round;
                  let mut index36: i32 =  round;
                  let mut num: i32 =  skills[index34, round] + 1;
                  numArray[index35, index36] = num;
                }
                if (index28 < this.AttackerRegime &  this.game.Data.RuleVar[841] == 0.0)
                {
                  sloss: Vec<i32> = this.game.Data.RegimeObj[index28].SLoss;
                  numArray: Vec<i32> = sloss;
                  let mut index37: i32 =  isfType;
                  let mut index38: i32 =  index37;
                  let mut index39: i32 =  this.game.Data.Round + 1;
                  let mut index40: i32 =  index39;
                  let mut num: i32 =  sloss[index37, index39] + 1;
                  numArray[index38, index40] = num;
                }
                else
                {
                  sloss: Vec<i32> = this.game.Data.RegimeObj[index28].SLoss;
                  numArray: Vec<i32> = sloss;
                  let mut index41: i32 =  isfType;
                  let mut index42: i32 =  index41;
                  let mut round: i32 =  this.game.Data.Round;
                  let mut index43: i32 =  round;
                  let mut num: i32 =  sloss[index41, round] + 1;
                  numArray[index42, index43] = num;
                }
                if (index28 == this.AttackerRegime)
                {
                  sloss: Vec<i32> = this.game.Data.RegimeObj[index28].SLoss;
                  numArray: Vec<i32> = sloss;
                  let mut index44: i32 =  isfType;
                  let mut index45: i32 =  index44;
                  let mut index46: i32 =  0;
                  let mut index47: i32 =  index46;
                  let mut num: i32 =  sloss[index44, index46] + 1;
                  numArray[index45, index47] = num;
                }
                if (index29 == this.AttackerRegime)
                {
                  skills: Vec<i32> = this.game.Data.RegimeObj[index29].SKills;
                  numArray: Vec<i32> = skills;
                  let mut index48: i32 =  isfType;
                  let mut index49: i32 =  index48;
                  let mut index50: i32 =  0;
                  let mut index51: i32 =  index50;
                  let mut num: i32 =  skills[index48, index50] + 1;
                  numArray[index49, index51] = num;
                }
              }
            }
          }
          this.game.Data.SFObj[sf].Qty = 0;
        }
      }
      if (this.game.Data.UnitObj[unr].SFCount > -1)
      {
        let mut num: i32 =  0;
        let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount2; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index];
          num += this.game.Data.SFObj[sf].Qty;
        }
        if (num != 0)
          return;
        this.AddDetail("We are going to remove unit because no fighters left in it. (2)");
        this.UList[tunr].UDead = 1;
      }
      else
      {
        this.AddDetail("We are going to remove unit because no fighters left in it. (2)");
        this.UList[tunr].UDead = 1;
      }
    }

    pub fn KillUnit(unr: i32)
    {
      this.AddBiggy("Unit has been destroyed " + this.game.Data.UnitObj[unr].Name);
      if (this.customCombatObj.HasCustumCalls())
      {
        CustomCombatCalls customCombatObj = this.customCombatObj;
        combatClass: CombatClass = this;
        ref local: CombatClass = ref combatClass;
        let mut game: GameClass = this.game;
        let mut unr1: i32 =  unr;
        customCombatObj.UnitLost(ref local, game, unr1);
      }
      this.game.Data.RemoveUnit(unr, ref this.game);
    }

    pub fn DoRetreat(tunr: i32)
    {
      if (this.game.EditObj.CombatSim || this.UList[tunr].UDead == 1)
        return;
      let mut unr: i32 =  this.UList[tunr].UNr;
      if (this.UList[tunr].UCanRetreat.onmap & this.UList[tunr].Uattacker == 0 && !this.game.HandyFunctionsObj.HasUnitAirSF(unr) && this.game.Data.MapObj[this.UList[tunr].UCanRetreat.map].HexObj[this.UList[tunr].UCanRetreat.x, this.UList[tunr].UCanRetreat.y].UnitCounter > 14 + this.UList[tunr].Uattacker)
      {
        this.UList[tunr].UCanRetreat.onmap = false;
        this.UList[tunr].UCanRetreat.x = -1;
        this.UList[tunr].UCanRetreat.y = -1;
        if (this.UList[tunr].Uattacker == 0)
        {
          Coordinate coord = Coordinate::new();
          coord.x = this.TargetX;
          coord.y = this.TargetY;
          coord.map = this.TargetMap;
          coord.onmap = true;
          if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea)
            this.UList[tunr].UCanRetreat = this.game.HandyFunctionsObj.FindLandRetreat(unr, coord, (Neighbours) null);
          else if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.TargetMap].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea)
            this.UList[tunr].UCanRetreat = this.game.HandyFunctionsObj.FindSeaRetreat(unr, coord, (Neighbours) null);
        }
      }
      if ( this.game.Data.RuleVar[894] > 0.0 && this.game.Data.UnitObj[unr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].Type == 8 && this.UList[tunr].Uattacker == 0 && this.CombatType == 1 | this.CombatType == 12)
        this.UList[tunr].UCanRetreat.onmap = false;
      num1: i32;
      if (this.UList[tunr].UCanRetreat.onmap)
      {
        try
        {
          this.game.Data.MapObj[this.game.Data.UnitObj[unr].Map].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].RemoveUnitFromList(unr);
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          num1 = 1;
          ProjectData.ClearProjectError();
        }
        this.game.Data.UnitObj[unr].apReserve = 0;
        this.game.Data.UnitObj[unr].X = this.UList[tunr].UCanRetreat.x;
        this.game.Data.UnitObj[unr].Y = this.UList[tunr].UCanRetreat.y;
        this.game.Data.UnitObj[unr].Map = this.UList[tunr].UCanRetreat.map;
        this.game.Data.MapObj[this.UList[tunr].UCanRetreat.map].HexObj[this.UList[tunr].UCanRetreat.x, this.UList[tunr].UCanRetreat.y].AddUnitToList(unr);
        if (this.game.Data.UnitObj[unr].SFCount > -1)
        {
          let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
          for (let mut index: i32 =  0; index <= sfCount; index += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index];
            if (this.UList[tunr].Uattacker == 1)
            {
              if (!(this.CombatType == 3 | this.CombatType == 4) & (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ArtRange > 0 | this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater > 0))
              {
                this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index]].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(this.UList[tunr].UCanRetreat.x, this.UList[tunr].UCanRetreat.y, this.UList[tunr].UCanRetreat.map, this.game.Data.SFTypeObj[this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index]].Type].UnitGroup);
                this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index]].initialEntrench = this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index]].CurrentEntrench;
              }
            }
            else
              this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index]].Ap = 0;
          }
        }
        this.AddBiggy("Retreat or BreakofCombat is succesfull for " + this.game.Data.UnitObj[unr].Name);
        if (this.HexList.Exists(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map))
          return;
        this.HexList.AddCoord(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map);
      }
      else
      {
        bool flag1 = false;
        if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[455] > 0.0)
        {
          SimpleList simpleList1 = SimpleList::new();
          SimpleList simpleList2 = SimpleList::new();
          Coordinate coord1;
          coord1.x = this.TargetX;
          coord1.y = this.TargetY;
          coord1.map = 0;
          coord1.onmap = true;
          Neighbours neighbour = Neighbours::new();
          if (this.CombatType == 1 | this.CombatType == 2 | this.CombatType == 11)
          {
            let mut num2: i32 =  this.game.HandyFunctionsObj.HexFacing(coord1.x, coord1.y, coord1.map, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map);
            if (num2 > 0)
              neighbour.data[num2 - 1] = 1;
          }
          let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
          sf1: i32;
          for (let mut index1: i32 =  0; index1 <= sfCount1; index1 += 1)
          {
            sf1 = this.game.Data.UnitObj[unr].SFList[index1];
            let mut people: i32 =  this.game.Data.SFObj[sf1].People;
            let mut tid: i32 =  sf1;
            let mut tweight: i32 =  0;
            let mut type: i32 =  this.game.Data.SFObj[sf1].Type;
            let mut icounter: i32 =  this.ICounter;
            for (let mut index2: i32 =  1; index2 <= icounter; index2 += 1)
            {
              if (this.IList[index2].ISFNr == sf1 && !(this.IList[index2].IKilled > 0 | -(this.IList[index2].ICapitulate ? 1 : 0) > 0))
                tweight += 1;
            }
            simpleList2.Add(sf1, tweight, CheckExistence: false);
            if (tweight >= 1)
            {
              let mut forceDirection: i32 =  1;
              do
              {
                float num3 = this.game.Data.RuleVar[455];
                this.game.Data.RuleVar[455] = 0.0f;
                Coordinate landRetreat1 = this.game.HandyFunctionsObj.FindLandRetreat(unr, coord1, neighbour, forceDirection: forceDirection);
                this.game.Data.RuleVar[455] = num3;
                Coordinate landRetreat2 = this.game.HandyFunctionsObj.FindLandRetreat(unr, coord1, neighbour, this.game.Data.SFTypeObj[type].MoveType, forceDirection);
                if (landRetreat2.onmap & landRetreat1.onmap)
                {
                  let mut tdata1: i32 =  1;
                  let mut x: i32 =  landRetreat2.x;
                  let mut y: i32 =  landRetreat2.y;
                  simpleList1.Add(tid, 1, tdata1, x, y, CheckExistence: false);
                }
                else if (this.game.Data.SFTypeObj[type].scrapable > 0 &  this.game.Data.RuleVar[486] > 0.0)
                {
                  let mut index3: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[486]))].GetData(0, people, 1)));
                  if (index3 > -1)
                  {
                    landRetreat2 = this.game.HandyFunctionsObj.FindLandRetreat(unr, coord1, neighbour, this.game.Data.SFTypeObj[index3].MoveType, forceDirection);
                    if (landRetreat2.onmap & landRetreat1.onmap)
                    {
                      let mut tdata1: i32 =  2;
                      let mut x: i32 =  landRetreat2.x;
                      let mut y: i32 =  landRetreat2.y;
                      simpleList1.Add(tid, 2, tdata1, x, y, CheckExistence: false);
                    }
                  }
                }
                forceDirection += 1;
              }
              while (forceDirection <= 6);
            }
          }
          CoordList coordList = CoordList::new();
          let mut counter1: i32 =  simpleList1.Counter;
          for (let mut index: i32 =  0; index <= counter1; index += 1)
          {
            if (coordList.FindSlot(simpleList1.Data2[index], simpleList1.Data3[index], 0) == -1)
              coordList.AddCoord(simpleList1.Data2[index], simpleList1.Data3[index], 0);
          }
          let mut counter2: i32 =  coordList.counter;
          for (let mut index4: i32 =  0; index4 <= counter2; index4 += 1)
          {
            let mut counter3: i32 =  simpleList2.Counter;
            for (let mut index5: i32 =  0; index5 <= counter3; index5 += 1)
            {
              let mut num4: i32 =  0;
              let mut counter4: i32 =  simpleList1.Counter;
              for (let mut index6: i32 =  0; index6 <= counter4; index6 += 1)
              {
                if (simpleList1.Id[index6] == simpleList2.Id[index5] && simpleList1.Data2[index6] == coordList.coord[index4].x & simpleList1.Data3[index6] == coordList.coord[index4].y && simpleList1.Data1[index6] == 1)
                {
                  Coordinate[] coord2 = coordList.coord;
                  Coordinate[] coordinateArray = coord2;
                  let mut index7: i32 =  index4;
                  let mut index8: i32 =  index7;
                  coordinateArray[index8].data1 = coord2[index7].data1 + simpleList2.Weight[index5] * 2;
                  num4 = 1;
                }
              }
              if (num4 == 0)
              {
                let mut counter5: i32 =  simpleList1.Counter;
                for (let mut index9: i32 =  0; index9 <= counter5; index9 += 1)
                {
                  if (simpleList1.Id[index9] == simpleList2.Id[index5] && simpleList1.Data2[index9] == coordList.coord[index4].x & simpleList1.Data3[index9] == coordList.coord[index4].y && simpleList1.Data1[index9] == 2)
                  {
                    Coordinate[] coord3 = coordList.coord;
                    Coordinate[] coordinateArray = coord3;
                    let mut index10: i32 =  index4;
                    let mut index11: i32 =  index10;
                    coordinateArray[index11].data1 = coord3[index10].data1 + simpleList2.Weight[index5] * 1;
                  }
                }
              }
            }
          }
          let mut index12: i32 =  -1;
          let mut num5: i32 =  0;
          let mut counter6: i32 =  coordList.counter;
          for (let mut index13: i32 =  0; index13 <= counter6; index13 += 1)
          {
            if (coordList.coord[index13].data1 > num5)
            {
              index12 = index13;
              num5 = coordList.coord[index13].data1;
            }
          }
          if (index12 > -1)
          {
            flag1 = true;
            this.UList[tunr].UCanRetreat.onmap = true;
            this.UList[tunr].UCanRetreat.x = coordList.coord[index12].x;
            this.UList[tunr].UCanRetreat.y = coordList.coord[index12].y;
            let mut counter7: i32 =  simpleList2.Counter;
            for (let mut index14: i32 =  0; index14 <= counter7; index14 += 1)
            {
              if (simpleList2.Weight[index14] > 0)
              {
                bool flag2 = true;
                let mut counter8: i32 =  simpleList1.Counter;
                for (let mut index15: i32 =  0; index15 <= counter8; index15 += 1)
                {
                  if (simpleList1.Id[index15] == simpleList2.Id[index14] && simpleList1.Data2[index15] == this.UList[tunr].UCanRetreat.x && simpleList1.Data3[index15] == this.UList[tunr].UCanRetreat.y)
                    flag2 = false;
                }
                if (flag2)
                {
                  sf1 = simpleList2.Id[index14];
                  let mut num6: i32 =  0;
                  let mut type1: i32 =  this.game.Data.SFObj[sf1].Type;
                  str: String = "Retreat forced surrender of " + this.game.Data.SFTypeObj[type1].Name + ".";
                  this.AddBiggy(str);
                  this.AddReport(1, "Partial Retreat", str, tunr, this.CombatRound);
                  bool flag3 = false;
                  if (this.CombatType == 2 & this.game.Data.SFTypeObj[type1].Theater != 1)
                    flag3 = true;
                  if (this.CombatType == 1 & this.game.Data.SFTypeObj[type1].Theater != 0)
                    flag3 = true;
                  if (flag3)
                  {
                    let mut qty: i32 =  this.game.Data.SFObj[sf1].Qty;
                    for (let mut index16: i32 =  1; index16 <= qty; index16 += 1)
                    {
                      this += 1.ICounter;
                      this.IList = (Individual[]) Utils.CopyArray((Array) this.IList, (Array) new Individual[this.ICounter + 1]);
                      this.IList[this.ICounter].IAttacker = 0;
                      this.IList[this.ICounter].ISFType = this.game.Data.SFObj[sf1].Type;
                      this.IList[this.ICounter].ISFNr = sf1;
                      this.IList[this.ICounter].IKilled = 1;
                      num6 += 1;
                      this.IList[this.ICounter].ICapitulate = true;
                    }
                  }
                  else
                  {
                    let mut icounter: i32 =  this.ICounter;
                    for (let mut index17: i32 =  0; index17 <= icounter; index17 += 1)
                    {
                      if (this.IList[index17].ISFNr == sf1 & this.IList[index17].IKilled == 0)
                      {
                        this.IList[index17].IKilled = this.CombatRound;
                        num6 += 1;
                        this.IList[index17].ICapitulate = true;
                      }
                    }
                  }
                  let mut type2: i32 =  this.game.Data.SFObj[sf1].Type;
                  index18: i32;
                  index19: i32;
                  if (this.UList[tunr].Uattacker == 1)
                  {
                    index18 = this.AttackerRegime;
                    index19 = this.DefenderRegime;
                  }
                  else
                  {
                    index18 = this.DefenderRegime;
                    index19 = this.AttackerRegime;
                  }
                  if (index19 < this.AttackerRegime &  this.game.Data.RuleVar[841] == 0.0)
                  {
                    skills: Vec<i32> = this.game.Data.RegimeObj[index19].SKills;
                    numArray: Vec<i32> = skills;
                    let mut index20: i32 =  type2;
                    let mut index21: i32 =  index20;
                    let mut index22: i32 =  this.game.Data.Round + 1;
                    let mut index23: i32 =  index22;
                    let mut num7: i32 =  skills[index20, index22] + num6;
                    numArray[index21, index23] = num7;
                  }
                  else
                  {
                    skills: Vec<i32> = this.game.Data.RegimeObj[index19].SKills;
                    numArray: Vec<i32> = skills;
                    let mut index24: i32 =  type2;
                    let mut index25: i32 =  index24;
                    let mut round: i32 =  this.game.Data.Round;
                    let mut index26: i32 =  round;
                    let mut num8: i32 =  skills[index24, round] + num6;
                    numArray[index25, index26] = num8;
                  }
                  if (index18 < this.AttackerRegime &  this.game.Data.RuleVar[841] == 0.0)
                  {
                    sloss: Vec<i32> = this.game.Data.RegimeObj[index18].SLoss;
                    numArray: Vec<i32> = sloss;
                    let mut index27: i32 =  type2;
                    let mut index28: i32 =  index27;
                    let mut index29: i32 =  this.game.Data.Round + 1;
                    let mut index30: i32 =  index29;
                    let mut num9: i32 =  sloss[index27, index29] + num6;
                    numArray[index28, index30] = num9;
                  }
                  else
                  {
                    sloss: Vec<i32> = this.game.Data.RegimeObj[index18].SLoss;
                    numArray: Vec<i32> = sloss;
                    let mut index31: i32 =  type2;
                    let mut index32: i32 =  index31;
                    let mut round: i32 =  this.game.Data.Round;
                    let mut index33: i32 =  round;
                    let mut num10: i32 =  sloss[index31, round] + num6;
                    numArray[index32, index33] = num10;
                  }
                  if (index18 == this.AttackerRegime)
                  {
                    sloss: Vec<i32> = this.game.Data.RegimeObj[index18].SLoss;
                    numArray: Vec<i32> = sloss;
                    let mut index34: i32 =  type2;
                    let mut index35: i32 =  index34;
                    let mut index36: i32 =  0;
                    let mut index37: i32 =  index36;
                    let mut num11: i32 =  sloss[index34, index36] + num6;
                    numArray[index35, index37] = num11;
                  }
                  if (index19 == this.AttackerRegime)
                  {
                    skills: Vec<i32> = this.game.Data.RegimeObj[index19].SKills;
                    numArray: Vec<i32> = skills;
                    let mut index38: i32 =  type2;
                    let mut index39: i32 =  index38;
                    let mut index40: i32 =  0;
                    let mut index41: i32 =  index40;
                    let mut num12: i32 =  skills[index38, index40] + num6;
                    numArray[index39, index41] = num12;
                  }
                  this.game.Data.SFObj[sf1].Qty = 0;
                }
              }
            }
            let mut counter9: i32 =  simpleList2.Counter;
            for (let mut index42: i32 =  0; index42 <= counter9; index42 += 1)
            {
              if (simpleList2.Weight[index42] > 0)
              {
                bool flag4 = false;
                let mut people1: i32 =  this.game.Data.SFObj[sf1].People;
                sf1 = simpleList2.Id[index42];
                let mut counter10: i32 =  simpleList1.Counter;
                for (let mut index43: i32 =  0; index43 <= counter10; index43 += 1)
                {
                  if (simpleList1.Id[index43] == simpleList2.Id[index42] && simpleList1.Data2[index43] == this.UList[tunr].UCanRetreat.x && simpleList1.Data3[index43] == this.UList[tunr].UCanRetreat.y && simpleList1.Data1[index43] == 2)
                    flag4 = true;
                }
                if (flag4)
                {
                  let mut qty: i32 =  this.game.Data.SFObj[sf1].Qty;
                  let mut people2: i32 =  this.game.Data.SFObj[sf1].People;
                  let mut type: i32 =  this.game.Data.SFObj[sf1].Type;
                  str: String = "Retreat forced scrapping of " + this.game.Data.SFTypeObj[type].Name + ".";
                  this.AddBiggy(str);
                  this.AddReport(1, "Partial Retreat", str, tunr, this.CombatRound);
                  let mut icounter: i32 =  this.ICounter;
                  for (let mut index44: i32 =  0; index44 <= icounter; index44 += 1)
                  {
                    if (this.IList[index44].ISFNr == sf1 & this.IList[index44].IKilled < 1)
                    {
                      let mut SfType: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[486]))].GetData(0, people2, 1)));
                      let mut xp: i32 =  this.game.Data.SFObj[sf1].Xp;
                      let mut ap: i32 =  this.game.Data.SFObj[sf1].Ap;
                      let mut rdn: i32 =  this.game.Data.SFObj[sf1].Rdn;
                      let mut mor: i32 =  this.game.Data.SFObj[sf1].Mor;
                      let mut supplyConsume: i32 =  this.game.Data.UnitObj[unr].SupplyConsume;
                      let mut offMod: i32 =  this.game.Data.SFObj[sf1].OffMod;
                      let mut defMod: i32 =  this.game.Data.SFObj[sf1].DefMod;
                      let mut Qty: i32 =  0;
                      if (this.game.Data.SFTypeObj[type].scrapable >= 100)
                        Qty = 1 * (int) Math.Round(Math.Floor( this.game.Data.SFTypeObj[type].scrapable / 100.0));
                      else if (this.game.Data.SFTypeObj[type].scrapable > 1)
                      {
                        let mut num13: i32 =  1;
                        do
                        {
                          if (DrawMod.RandyNumber.Next(1, 100) <= this.game.Data.SFTypeObj[type].scrapable)
                            Qty += 1;
                          num13 += 1;
                        }
                        while (num13 <= 1);
                      }
                      else
                        Qty = 0;
                      let mut Ap: i32 =  ap - 25;
                      if (Ap < 0)
                        Ap = 0;
                      if (Qty > 0)
                      {
                        this.game.HandyFunctionsObj.AddTroops3(unr, SfType, people2, Qty, xp, rdn, Ap, mor, supplyConsume, offmod: offMod, defmod: defMod);
                        let mut num14: i32 =  -1;
                        let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
                        for (let mut index45: i32 =  0; index45 <= sfCount2; index45 += 1)
                        {
                          let mut sf2: i32 =  this.game.Data.UnitObj[unr].SFList[index45];
                          if (this.game.Data.SFObj[sf2].Type == SfType && this.game.Data.SFObj[sf2].People == people2)
                            num14 = sf2;
                        }
                        this.IList[index44].ISFNr = num14;
                        this.IList[index44].ISFType = SfType;
                      }
                      else
                      {
                        this.IList[index44].ICapitulate = true;
                        this.IList[index44].IKilled = this.CombatRound;
                      }
                    }
                  }
                  this.game.Data.SFObj[sf1].Qty = 0;
                }
              }
            }
            try
            {
              this.game.Data.MapObj[this.game.Data.UnitObj[unr].Map].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].RemoveUnitFromList(unr);
            }
            catch (Exception ex)
            {
              ProjectData.SetProjectError(ex);
              num1 = 1;
              ProjectData.ClearProjectError();
            }
            this.game.Data.UnitObj[unr].X = this.UList[tunr].UCanRetreat.x;
            this.game.Data.UnitObj[unr].Y = this.UList[tunr].UCanRetreat.y;
            this.game.Data.UnitObj[unr].Map = this.UList[tunr].UCanRetreat.map;
            this.game.Data.UnitObj[unr].apReserve = 0;
            this.game.Data.MapObj[this.UList[tunr].UCanRetreat.map].HexObj[this.UList[tunr].UCanRetreat.x, this.UList[tunr].UCanRetreat.y].AddUnitToList(unr);
            if (this.game.Data.UnitObj[unr].SFCount > -1)
            {
              let mut sfCount3: i32 =  this.game.Data.UnitObj[unr].SFCount;
              for (let mut index46: i32 =  0; index46 <= sfCount3; index46 += 1)
              {
                let mut sf3: i32 =  this.game.Data.UnitObj[unr].SFList[index46];
                if (this.game.Data.SFObj[sf3].Qty > 0)
                {
                  if (this.UList[tunr].Uattacker == 1)
                  {
                    if (!(this.CombatType == 3 | this.CombatType == 4) & (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf3].Type].ArtRange > 0 | this.game.Data.SFTypeObj[this.game.Data.SFObj[sf3].Type].Theater > 0))
                    {
                      this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index46]].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(this.UList[tunr].UCanRetreat.x, this.UList[tunr].UCanRetreat.y, this.UList[tunr].UCanRetreat.map, this.game.Data.SFTypeObj[this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index46]].Type].UnitGroup);
                      this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index46]].initialEntrench = this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index46]].CurrentEntrench;
                    }
                  }
                  else
                    this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index46]].Ap = 0;
                }
              }
            }
            str1: String = "Retreat or BreakofCombat is partially succesfull for " + this.game.Data.UnitObj[unr].Name;
            this.AddBiggy(str1);
            this.AddReport(1, "Partial Retreat", str1, tunr, this.CombatRound);
            if (!this.HexList.Exists(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map))
              this.HexList.AddCoord(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map);
          }
        }
        if (flag1)
          return;
        let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index47: i32 =  0; index47 <= sfCount; index47 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index47];
          let mut num15: i32 =  0;
          let mut type3: i32 =  this.game.Data.SFObj[sf].Type;
          bool flag5 = false;
          if (this.CombatType == 2 & this.game.Data.SFTypeObj[type3].Theater != 1)
            flag5 = true;
          if (this.CombatType == 1 & this.game.Data.SFTypeObj[type3].Theater != 0)
            flag5 = true;
          if (flag5)
          {
            let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
            for (let mut index48: i32 =  1; index48 <= qty; index48 += 1)
            {
              this += 1.ICounter;
              this.IList = (Individual[]) Utils.CopyArray((Array) this.IList, (Array) new Individual[this.ICounter + 1]);
              this.IList[this.ICounter].IAttacker = 0;
              this.IList[this.ICounter].ISFType = this.game.Data.SFObj[sf].Type;
              this.IList[this.ICounter].IKilled = 1;
              this.IList[this.ICounter].ISFNr = sf;
              num15 += 1;
              this.IList[this.ICounter].ICapitulate = true;
            }
          }
          else
          {
            let mut icounter: i32 =  this.ICounter;
            for (let mut index49: i32 =  0; index49 <= icounter; index49 += 1)
            {
              if (this.IList[index49].ISFNr == sf & this.IList[index49].IKilled == 0)
              {
                this.IList[index49].IKilled = 1;
                num15 += 1;
                this.IList[index49].ICapitulate = true;
              }
            }
          }
          let mut type4: i32 =  this.game.Data.SFObj[sf].Type;
          index50: i32;
          index51: i32;
          if (this.UList[tunr].Uattacker == 1)
          {
            index50 = this.AttackerRegime;
            index51 = this.DefenderRegime;
          }
          else
          {
            index50 = this.DefenderRegime;
            index51 = this.AttackerRegime;
          }
          if (index51 < this.AttackerRegime &  this.game.Data.RuleVar[841] == 0.0)
          {
            skills: Vec<i32> = this.game.Data.RegimeObj[index51].SKills;
            numArray: Vec<i32> = skills;
            let mut index52: i32 =  type4;
            let mut index53: i32 =  index52;
            let mut index54: i32 =  this.game.Data.Round + 1;
            let mut index55: i32 =  index54;
            let mut num16: i32 =  skills[index52, index54] + num15;
            numArray[index53, index55] = num16;
          }
          else
          {
            skills: Vec<i32> = this.game.Data.RegimeObj[index51].SKills;
            numArray: Vec<i32> = skills;
            let mut index56: i32 =  type4;
            let mut index57: i32 =  index56;
            let mut round: i32 =  this.game.Data.Round;
            let mut index58: i32 =  round;
            let mut num17: i32 =  skills[index56, round] + num15;
            numArray[index57, index58] = num17;
          }
          if (index50 < this.AttackerRegime &  this.game.Data.RuleVar[841] == 0.0)
          {
            sloss: Vec<i32> = this.game.Data.RegimeObj[index50].SLoss;
            numArray: Vec<i32> = sloss;
            let mut index59: i32 =  type4;
            let mut index60: i32 =  index59;
            let mut index61: i32 =  this.game.Data.Round + 1;
            let mut index62: i32 =  index61;
            let mut num18: i32 =  sloss[index59, index61] + num15;
            numArray[index60, index62] = num18;
          }
          else
          {
            sloss: Vec<i32> = this.game.Data.RegimeObj[index50].SLoss;
            numArray: Vec<i32> = sloss;
            let mut index63: i32 =  type4;
            let mut index64: i32 =  index63;
            let mut round: i32 =  this.game.Data.Round;
            let mut index65: i32 =  round;
            let mut num19: i32 =  sloss[index63, round] + num15;
            numArray[index64, index65] = num19;
          }
          if (index50 == this.AttackerRegime)
          {
            sloss: Vec<i32> = this.game.Data.RegimeObj[index50].SLoss;
            numArray: Vec<i32> = sloss;
            let mut index66: i32 =  type4;
            let mut index67: i32 =  index66;
            let mut index68: i32 =  0;
            let mut index69: i32 =  index68;
            let mut num20: i32 =  sloss[index66, index68] + num15;
            numArray[index67, index69] = num20;
          }
          if (index51 == this.AttackerRegime)
          {
            skills: Vec<i32> = this.game.Data.RegimeObj[index51].SKills;
            numArray: Vec<i32> = skills;
            let mut index70: i32 =  type4;
            let mut index71: i32 =  index70;
            let mut index72: i32 =  0;
            let mut index73: i32 =  index72;
            let mut num21: i32 =  skills[index70, index72] + num15;
            numArray[index71, index73] = num21;
          }
          this.game.Data.SFObj[sf].Qty = 0;
        }
        str2: String = "No retreat option for " + this.game.Data.UnitObj[unr].Name;
        this.AddReport(1, "Retreat Failed", str2, tunr, this.CombatRound);
        this.AddBiggy(str2);
        this.AddDetail("We are going to remove unit because no retreat option.");
        this.UList[tunr].UDead = 1;
      }
    }

    pub fn CheckBreakthrough()
    {
      let mut icounter1: i32 =  this.ICounter;
      num1: i32;
      num2: i32;
      for (let mut index: i32 =  0; index <= icounter1; index += 1)
      {
        if (this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0)
        {
          if (this.IList[index].IAttacker == 0 && !this.game.Data.SFTypeObj[this.IList[index].ISFType].BackBench)
            num1 += 1;
          if (this.IList[index].IAttacker == 1 && !this.game.Data.SFTypeObj[this.IList[index].ISFType].BackBench)
            num2 += 1;
        }
      }
      let mut icounter2: i32 =  this.ICounter;
      for (let mut index: i32 =  0; index <= icounter2; index += 1)
      {
        if (this.IList[index].IAttacker == 1)
        {
          if (!(this.CombatType == 4 | this.CombatType == 3) | this.game.Data.Product < 5 && this.IList[index].ISuccesfullAttack == this.CombatRound | this.IList[index].ILastTargeted + 2 <= this.CombatRound | num1 == 0 && !this.game.Data.SFTypeObj[this.IList[index].ISFType].BackBench && this.IList[index].IBattleRounds >= 2 | num1 == 0 && this.IList[index].ILastHit + 2 <= this.CombatRound | num1 == 0 && this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0 & this.IList[index].IRetreated == 0 && this.IList[index].IBreakTrough == 0 &&  VBMath.Rnd() < 1.0 /  this.CrowdingAttMod | num1 == 0)
          {
            this.IList[index].IBreakTrough = 1;
            s: String = "ATTACKER: Has broken through.";
            this.AddReport(0, "Broken through", "This individual has broken through enemy lines.", index + 10000, this.CombatRound);
            this.AddDetail(s);
          }
        }
        else if ((this.IList[index].ISuccesfullAttack == this.CombatRound | this.IList[index].ILastTargeted + 2 <= this.CombatRound) & num2 == 0 && !this.game.Data.SFTypeObj[this.IList[index].ISFType].BackBench && this.IList[index].IBattleRounds >= 2 | num2 == 0 && this.IList[index].ILastHit + 2 <= this.CombatRound | num2 == 0 && !(this.CombatType == 3 | this.CombatType == 4 | this.CombatType == 5) && this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0 & this.IList[index].IRetreated == 0 && this.IList[index].IBreakTrough == 0 &&  VBMath.Rnd() < 1.0 /  this.CrowdingDefMod | num2 == 0)
        {
          this.IList[index].IBreakTrough = 1;
          s: String = "DEFENDER: Has broken through.";
          this.AddReport(0, "Broken through", "This individual has broken through enemy lines.", index + 10000, this.CombatRound);
          this.AddDetail(s);
        }
        if (this.IList[index].IAttacker == 1)
        {
          if (this.CombatRound > 1 | num1 == 0 && this.IList[index].IBreakTrough == 0 && !this.game.Data.SFTypeObj[this.IList[index].ISFType].BackBench &&  this.IList[index].AttackedCount <= Conversion.Int( this.game.Data.SFTypeObj[this.IList[index].ISFType].MaxAttacked / 2.0) && this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 2 && this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0 & this.IList[index].IRetreated == 0 && this.IList[index].IBreakTrough == 0)
          {
            this.IList[index].IBreakTrough = 1;
            s: String = "AIR: Has broken through.";
            this.AddReport(0, "Broken through", "This individual has broken through enemy lines.", index + 10000, this.CombatRound);
            this.AddDetail(s);
          }
        }
        else if (this.CombatRound > 1 | num2 == 0 && this.IList[index].IBreakTrough == 0 && !this.game.Data.SFTypeObj[this.IList[index].ISFType].BackBench &&  this.IList[index].AttackedCount <= Conversion.Int( this.game.Data.SFTypeObj[this.IList[index].ISFType].MaxAttacked / 2.0) && this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == 2 && this.IList[index].IKilled == 0 & this.IList[index].IRetreat == 0 & this.IList[index].IRetreated == 0 && this.IList[index].IBreakTrough == 0)
        {
          this.IList[index].IBreakTrough = 1;
          s: String = "AIR: Has broken through.";
          this.AddReport(0, "Broken through", "This individual has broken through enemy lines.", index + 10000, this.CombatRound);
          this.AddDetail(s);
        }
      }
    }

    pub fn ReduceRdn(inr: i32, percent: i32)
    {
      let mut num: i32 =  (int) Math.Round(Conversion.Int( this.IList[inr].IRdn * ( percent / 100.0)));
      if (num == 0)
        num = 1;
      Individual[] ilist = this.IList;
      Individual[] individualArray = ilist;
      let mut index1: i32 =  inr;
      let mut index2: i32 =  index1;
      individualArray[index2].IRdn = ilist[index1].IRdn - num;
      if (this.IList[inr].IRdn >= 0)
        return;
      this.IList[inr].IRdn = 0;
    }

    pub fn ReduceAbsRdn(inr: i32, absnumber: i32)
    {
      Individual[] ilist = this.IList;
      Individual[] individualArray = ilist;
      let mut index1: i32 =  inr;
      let mut index2: i32 =  index1;
      individualArray[index2].IRdn = ilist[index1].IRdn - absnumber;
      if (this.IList[inr].IRdn >= 0)
        return;
      this.IList[inr].IRdn = 0;
    }

    pub fn ReduceEntr(inr: i32, percent: i32)
    {
      let mut num: i32 =  (int) Math.Round(Conversion.Int( this.IList[inr].IEntrench * ( percent / 100.0)));
      if (num == 0)
        num = 1;
      Individual[] ilist = this.IList;
      Individual[] individualArray = ilist;
      let mut index1: i32 =  inr;
      let mut index2: i32 =  index1;
      individualArray[index2].IEntrench = ilist[index1].IEntrench - num;
      if (this.IList[inr].IEntrench >= 0)
        return;
      this.IList[inr].IEntrench = 0;
    }

    pub fn ReduceEntr_AdvancedCombatRecon(attNr: i32, inr: i32, percent: i32)
    {
      bool flag = false;
      if (!(this.CombatType == 3 | this.CombatType == 4 | this.InterceptFire))
      {
        if (this.game.Data.SFTypeObj[this.IList[inr].ISFType].BackBench)
        {
          if (!this.game.Data.SFTypeObj[this.IList[attNr].ISFType].BackBench && this.IList[attNr].IBreakTrough > 0)
            flag = true;
        }
        else if (!this.game.Data.SFTypeObj[this.IList[attNr].ISFType].BackBench)
          flag = true;
      }
      let mut num: i32 =  (int) Math.Round(Conversion.Int( this.IList[inr].IEntrench * ( percent / 100.0)));
      if (num == 0)
        num = 1;
      if (flag)
      {
        Individual[] ilist = this.IList;
        Individual[] individualArray = ilist;
        let mut index1: i32 =  inr;
        let mut index2: i32 =  index1;
        individualArray[index2].IEntrench = ilist[index1].IEntrench - num;
        if (this.IList[inr].IEntrench >= 0)
          return;
        this.IList[inr].IEntrench = 0;
      }
      else if (this.game.Data.SFObj[this.IList[inr].ISFNr].initialEntrench > 0)
      {
        Individual[] ilist = this.IList;
        Individual[] individualArray = ilist;
        let mut index3: i32 =  inr;
        let mut index4: i32 =  index3;
        individualArray[index4].IEntrench = ilist[index3].IEntrench - num;
        if (this.IList[inr].IEntrench < 0)
          this.IList[inr].IEntrench = 0;
        if ((int) Math.Round(Math.Floor( this.game.Data.SFObj[this.IList[inr].ISFNr].initialEntrench * 0.66)) <= this.IList[inr].IEntrench)
          return;
        this.IList[inr].IEntrench = (int) Math.Round(Math.Floor( this.game.Data.SFObj[this.IList[inr].ISFNr].initialEntrench * 0.66));
      }
      else
      {
        Individual[] ilist = this.IList;
        Individual[] individualArray = ilist;
        let mut index5: i32 =  inr;
        let mut index6: i32 =  index5;
        individualArray[index6].IEntrench = ilist[index5].IEntrench - num;
        if (this.IList[inr].IEntrench >= 0)
          return;
        this.IList[inr].IEntrench = 0;
      }
    }

    pub fn ReduceMor(inr: i32, percent: i32)
    {
      let mut num: i32 =  (int) Math.Round(Conversion.Int( this.IList[inr].IMor * ( percent / 100.0)));
      if (num == 0)
        num = 1;
      if (num > 0 & this.customCombatObj.HasCustumCalls())
      {
        CustomCombatCalls customCombatObj = this.customCombatObj;
        combatClass: CombatClass = this;
        ref local: CombatClass = ref combatClass;
        let mut inr1: i32 =  inr;
        let mut morMod: i32 =  num;
        num = customCombatObj.IndividualMORMod(ref local, inr1, morMod);
      }
      Individual[] ilist = this.IList;
      Individual[] individualArray = ilist;
      let mut index1: i32 =  inr;
      let mut index2: i32 =  index1;
      individualArray[index2].IMor = ilist[index1].IMor - num;
      if (this.IList[inr].IMor >= 0)
        return;
      this.IList[inr].IMor = 0;
    }

    pub fn AddMor(inr: i32, percent: i32)
    {
      let mut num: i32 =  (int) Math.Round(Conversion.Int( this.IList[inr].IMor * ( percent / 100.0)));
      Individual[] ilist = this.IList;
      Individual[] individualArray = ilist;
      let mut index1: i32 =  inr;
      let mut index2: i32 =  index1;
      individualArray[index2].IMor = ilist[index1].IMor + num;
      if (this.IList[inr].IMor <= 100)
        return;
      this.IList[inr].IMor = 100;
    }

    pub fn IsUnitFighting(nr: i32) -> i32
    {
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index: i32 =  0; index <= ucounter; index += 1)
      {
        if (this.UList[index].UNr == nr)
          return index;
      }
      return -1;
    }

    pub fn AddXp(inr: i32, pointstot: i32)
    {
      if (this.game.EditObj.CombatSim)
        return;
      num1: i32;
      if (pointstot > 0)
      {
        let mut num2: i32 =  pointstot;
        for (let mut index1: i32 =  1; index1 <= num2; index1 += 1)
        {
          float a = 0.0f;
          if ( a == 0.0)
          {
            if ( this.game.Data.RuleVar[80] <= 1.0)
            {
              if ( VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
                a = 1f;
            }
            else if ( this.game.Data.RuleVar[80] <= 2.0)
            {
              if ( VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
                a = 1f;
            }
            else if ( this.game.Data.RuleVar[80] <= 3.0)
            {
              if ( VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
                a = 1f;
            }
            else if ( this.game.Data.RuleVar[80] <= 4.0 &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          if ( a > 0.0 & this.customCombatObj.HasCustumCalls())
          {
            CustomCombatCalls customCombatObj = this.customCombatObj;
            combatClass: CombatClass = this;
            ref local: CombatClass = ref combatClass;
            let mut inr1: i32 =  inr;
            let mut xpToBeGiven: i32 =  (int) Math.Round( a);
            a =  customCombatObj.IndividualXPMod(ref local, inr1, xpToBeGiven);
          }
          Individual[] ilist = this.IList;
          Individual[] individualArray = ilist;
          let mut index2: i32 =  inr;
          let mut index3: i32 =  index2;
          individualArray[index3].IXp = (int) Math.Round( ( ilist[index2].IXp + a));
          num1 = (int) Math.Round( ( num1 + a));
          if ( this.IList[inr].IXp >  this.game.Data.RuleVar[81])
            this.IList[inr].IXp = (int) Math.Round( this.game.Data.RuleVar[81]);
        }
      }
      if (this.game.Data.SFTypeObj[this.IList[inr].ISFType].StaffPts >= 1)
        return;
      let mut num3: i32 =  !this.game.Data.UnitObj[this.IList[inr].IUnr].IsHQ ? this.game.Data.UnitObj[this.IList[inr].IUnr].HQ : this.IList[inr].IUnr;
      if (!(num3 > -1 & num1 > 0))
        return;
      let mut num4: i32 =  this.IsUnitFighting(num3);
      if (num4 == -1)
      {
        let mut pointstot1: i32 =  (int) Math.Round( Conversion.Int( num1 * this.UList[this.IList[inr].IUlistNr].UStaffXpMod));
        if (pointstot1 < 1 &  VBMath.Rnd() <  this.UList[this.IList[inr].IUlistNr].UStaffXpMod)
          pointstot1 = 1;
        if (pointstot1 < 1)
          return;
        this.AddXpUnit(num3, pointstot1);
      }
      else
      {
        let mut inr2: i32 =  (int) Math.Round( (VBMath.Rnd() *  this.ICounter));
        let mut num5: i32 =  0;
        while (num5 < 2)
        {
          inr2 += 1;
          if (inr2 > this.ICounter)
          {
            inr2 = 0;
            num5 += 1;
          }
          if (this.IList[inr2].IUlistNr == num4 && this.game.Data.SFTypeObj[this.IList[inr2].ISFType].StaffPts > 0)
          {
            let mut pointstot2: i32 =  (int) Math.Round( Conversion.Int( num1 * this.UList[this.IList[inr].IUlistNr].UStaffXpMod));
            if (pointstot2 < 1 &  VBMath.Rnd() <  this.UList[this.IList[inr].IUlistNr].UStaffXpMod)
              pointstot2 = 1;
            if (pointstot2 < 1)
              break;
            this.AddXp2(inr2, pointstot2);
            break;
          }
        }
      }
    }

    pub fn AddXp2(inr: i32, pointstot: i32)
    {
      if (pointstot <= 0)
        return;
      let mut num1: i32 =  pointstot;
      for (let mut index1: i32 =  1; index1 <= num1; index1 += 1)
      {
        float a = 0.0f;
        if ( a == 0.0)
        {
          if ( this.game.Data.RuleVar[80] <= 1.0)
          {
            if (Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          else if ( this.game.Data.RuleVar[80] <= 2.0)
          {
            if (Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          else if ( this.game.Data.RuleVar[80] <= 3.0)
          {
            if (Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          else if ( this.game.Data.RuleVar[80] <= 4.0 && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81] && Math.Pow( VBMath.Rnd(),  this.game.Data.RuleVar[80]) >  this.IList[inr].IXp /  this.game.Data.RuleVar[81])
            a = 1f;
        }
        if ( a > 0.0 & this.customCombatObj.HasCustumCalls())
        {
          CustomCombatCalls customCombatObj = this.customCombatObj;
          combatClass: CombatClass = this;
          ref local: CombatClass = ref combatClass;
          let mut inr1: i32 =  inr;
          let mut xpToBeGiven: i32 =  (int) Math.Round( a);
          a =  customCombatObj.IndividualXPMod(ref local, inr1, xpToBeGiven);
        }
        Individual[] ilist = this.IList;
        Individual[] individualArray = ilist;
        let mut index2: i32 =  inr;
        let mut index3: i32 =  index2;
        individualArray[index3].IXp = (int) Math.Round( ( ilist[index2].IXp + a));
        let mut num2: i32 =  (int) Math.Round( ( num2 + a));
        if ( this.IList[inr].IXp >  this.game.Data.RuleVar[81])
          this.IList[inr].IXp = (int) Math.Round( this.game.Data.RuleVar[81]);
      }
    }

    pub fn AddXp3(sfnr: i32, pointstot: i32)
    {
      if (pointstot <= 0)
        return;
      let mut num1: i32 =  pointstot;
      for (let mut index1: i32 =  1; index1 <= num1; index1 += 1)
      {
        float a = 0.0f;
        if ( a == 0.0)
        {
          if ( this.game.Data.RuleVar[80] <= 1.0)
          {
            if ( VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          else if ( this.game.Data.RuleVar[80] <= 2.0)
          {
            if ( VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          else if ( this.game.Data.RuleVar[80] <= 3.0)
          {
            if ( VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81])
              a = 1f;
          }
          else if ( this.game.Data.RuleVar[80] <= 4.0 &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81] &&  VBMath.Rnd() >  this.game.Data.SFObj[sfnr].Xp /  this.game.Data.RuleVar[81])
            a = 1f;
        }
        if ( a > 0.0 & this.customCombatObj.HasCustumCalls())
        {
          CustomCombatCalls customCombatObj = this.customCombatObj;
          combatClass: CombatClass = this;
          ref local: CombatClass = ref combatClass;
          let mut xpToBeGiven: i32 =  (int) Math.Round( a);
          let mut sfnr1: i32 =  sfnr;
          a =  customCombatObj.IndividualXPMod(ref local, -1, xpToBeGiven, sfnr1);
        }
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index2: i32 =  sfnr;
        let mut index3: i32 =  index2;
        sfClassArray[index3].Xp = (int) Math.Round( ( sfObj[index2].Xp + a));
        let mut num2: i32 =  (int) Math.Round( ( num2 + a));
        if ( this.game.Data.SFObj[sfnr].Xp >  this.game.Data.RuleVar[81])
          this.game.Data.SFObj[sfnr].Xp = (int) Math.Round( this.game.Data.RuleVar[81]);
      }
    }

    pub fn AddXpUnit(unr: i32, pointstot: i32)
    {
      if (this.game.Data.UnitObj[unr].SFCount == -1)
        return;
      let mut historical: i32 =  this.game.Data.UnitObj[unr].Historical;
      object[] objArray1 = new object[this.game.Data.UnitObj[unr].SFCount + 1];
      let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
      Right: i32;
      for (let mut index1: i32 =  0; index1 <= sfCount1; index1 += 1)
      {
        let mut type: i32 =  this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index1]].Type;
        object[] objArray2 = objArray1;
        object[] objArray3 = objArray2;
        let mut index2: i32 =  index1;
        let mut index3: i32 =  index2;
        object obj = Operators.AddObject(objArray2[index2],  this.game.Data.SFTypeObj[type].StaffPts);
        objArray3[index3] = obj;
        Right += this.game.Data.SFTypeObj[type].StaffPts;
      }
      if (Right < 1)
        return;
      VBMath.Rnd();
      float Left = 0.0f;
      let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index4: i32 =  0; index4 <= sfCount2; index4 += 1)
      {
        Left = Conversions.ToSingle(Operators.AddObject( Left, Operators.DivideObject(objArray1[index4],  Right)));
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index4];
        if ( VBMath.Rnd() <=  Left)
        {
          if ( VBMath.Rnd() >=  pointstot /  this.game.Data.SFObj[sf].Qty)
            break;
          this.AddXp3(sf, 1);
          if (historical <= -1 || Strings.Len(this.game.Data.HistoricalUnitObj[historical].CommanderName) <= 0 ||  VBMath.Rnd() >=  this.game.Data.HistoricalUnitObj[historical].StaffSize /  Right)
            break;
          HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
          HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
          let mut index5: i32 =  historical;
          let mut index6: i32 =  index5;
          historicalUnitClassArray[index6].Xp = historicalUnitObj[index5].Xp + 1;
          break;
        }
      }
    }

    pub fn ParticipateUnitNr(unr: i32)
    {
      let mut ucounter: i32 =  this.UCounter;
      for (let mut index: i32 =  0; index <= ucounter; index += 1)
      {
        if (this.UList[index].UNr == unr)
          this.UList[index].UParticipated = 1;
      }
    }

    pub fn FindOpponent(inr: i32) -> i32
    {
      let mut favTargetTries: i32 =  this.game.Data.SFTypeObj[this.IList[inr].ISFType].FavTargetTries;
      if (this.CombatType == 3 | this.CombatType == 4 && this.IList[inr].IAttacker == 0)
      {
        if ( this.game.Data.RuleVar[419] > 0.0)
        {
          if (!this.InterceptFire && this.CombatRound < 2 & !this.previewMode)
            return -1;
        }
        else if ( this.game.Data.RuleVar[142] < 1.0)
          return -1;
      }
      let mut combatType1: i32 =  this.CombatType;
      if (this.CombatType == 9 & this.IList[inr].IParadropper & this.CombatRound < 6)
        return -1;
      bool flag1 = false;
      if (this.game.Data.Product >= 6 &&  this.game.Data.RuleVar[480] > 0.0 && this.CombatType == 3 | this.CombatType == 4)
        flag1 = true;
      if (this.IList[inr].IAttacker == 1)
      {
        let mut num1: i32 =  num1;
      }
      let mut num2: i32 =  1;
      num3: i32;
      opponent: i32;
      num4: i32;
      num5: i32;
      do
      {
        num3 = 0;
        opponent = -1;
        num4 = 0;
        if (num2 == 1 & flag1 | num2 == 2)
        {
          let mut num6: i32 =  0;
          do
          {
            let mut index: i32 =  (int) Math.Round( Conversion.Int(VBMath.Rnd() *  (this.ICounter + 1)));
            let mut combatType2: i32 =  this.CombatType;
            bool flag2 = false;
            if (this.IList[inr].IAttacker == 0 &  this.game.Data.RuleVar[493] > 0.0 && this.IList[index].IAttacker == 1 & this.IList[index].IleftOutOfPartialAttack)
              flag2 = true;
            if (this.TestTarget(inr, index) & !flag2)
            {
              let mut unitGroup: i32 =  this.game.Data.SFTypeObj[this.IList[index].ISFType].UnitGroup;
              if (this.CombatType == 3 | this.CombatType == 4)
              {
                num5 = this.game.Data.SFTypeObj[this.IList[inr].ISFType].FavArtTarget[unitGroup] * 10000;
                if (num5 == 0)
                  num5 = this.game.Data.SFTypeObj[this.IList[inr].ISFType].FavTarget[unitGroup] * 10000;
              }
              else
                num5 = this.game.Data.SFTypeObj[this.IList[inr].ISFType].FavTarget[unitGroup] * 10000;
              if (this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater == this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater)
              {
                if (this.game.Data.SFTypeObj[this.IList[inr].ISFType].Theater == 2)
                {
                  if (this.IList[inr].IAttacker == 1 && this.IList[index].IBreakTrough > 0)
                    num5 *= 2;
                }
                else if (this.IList[index].IBreakTrough > 0)
                  num5 *= 2;
                if (this.IList[inr].AttackedCount > this.game.Data.SFTypeObj[this.IList[inr].ISFType].MaxAttacked)
                  num5 = (int) Math.Round( (int) Math.Round( num5 / 50.0) * ( this.game.Data.SFTypeObj[this.IList[inr].ISFType].MaxAttacked /  this.IList[inr].AttackedCount));
              }
              if ( this.game.Data.RuleVar[431] > 0.0)
              {
                if ( this.GetEffectiveReconOnHexOfTargettedIndividual(index) <  this.IList[index].IcoverPoints)
                  num5 = (int) Math.Round( num5 / 3.0);
                if (num5 > 1)
                  num5 = DrawMod.RandyNumber.Next((int) Math.Round(Math.Ceiling( num5 / 10.0)), num5 + 1);
              }
              if (num5 > 0 & this.game.Data.Product >= 6 && this.game.Data.SFTypeObj[this.IList[index].ISFType].targettedByRangedChance > 0)
                num5 = (int) Math.Round( (num5 * this.game.Data.SFTypeObj[this.IList[index].ISFType].targettedByRangedChance) / 100.0);
              num3 += 1;
              if (this.game.Data.SFTypeObj[this.IList[index].ISFType].Theater != 2 & (this.CombatType == 6 | this.CombatType == 13 | this.CombatType == 14 | this.CombatType == 15))
                num5 = -1;
              if (num2 == 1 & flag1)
              {
                if (this.IList[index].ILastAttackDone > 0)
                {
                  if (this.IList[index].AttackedCount > this.GetMaxAttacked(index, inr))
                    num5 = 0;
                  else
                    num5 += 1;
                }
                else
                  num5 = 0;
              }
              if (num5 > num4)
              {
                num4 = num5;
                opponent = index;
              }
            }
            if (num3 < favTargetTries)
              num6 += 1;
            else
              break;
          }
          while (num6 <= 100);
          if (num2 == 1 & num3 > 0 & num4 > 0 & opponent > -1)
            break;
        }
        num2 += 1;
      }
      while (num2 <= 2);
      if (num3 < favTargetTries)
      {
        let mut icounter: i32 =  this.ICounter;
        for (let mut index: i32 =  0; index <= icounter; index += 1)
        {
          let mut defnr: i32 =  index;
          bool flag3 = false;
          if (this.IList[inr].IAttacker == 0 &  this.game.Data.RuleVar[493] > 0.0 && this.IList[defnr].IAttacker == 1 & this.IList[defnr].IleftOutOfPartialAttack)
            flag3 = true;
          if (this.TestTarget(inr, defnr) & !flag3)
          {
            let mut unitGroup: i32 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].UnitGroup;
            num5 = !(this.CombatType == 3 | this.CombatType == 4) ? this.game.Data.SFTypeObj[this.IList[inr].ISFType].FavArtTarget[unitGroup] : this.game.Data.SFTypeObj[this.IList[inr].ISFType].FavTarget[unitGroup];
            num3 += 1;
            if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater != 2 & (this.CombatType == 6 | this.CombatType == 13 & this.CombatType == 14 | this.CombatType == 15))
              num5 = -1;
            if (num5 > num4)
            {
              num4 = num5;
              opponent = defnr;
            }
          }
          if (num3 >= favTargetTries)
            break;
        }
      }
      if ( this.game.Data.RuleVar[431] > 0.0 & opponent > -1 && this.game.Data.Product != 7)
      {
        let mut inr1: i32 =  opponent;
        if ( this.GetEffectiveReconOnHexOfTargettedIndividual(inr1) <  this.IList[inr1].IcoverPoints)
        {
          let mut num7: i32 =  (int) Math.Round( num5 / 3.0);
          float num8 = this.AttackCrowding;
          let mut num9: i32 =  this.NewBattleStack;
          if (this.NewBattleStackArt > this.NewBattleStack)
          {
            num9 = this.NewBattleStackArt;
            num8 = this.AttackCrowdingArt;
          }
          if (this.IList[inr1].IAttacker == 0 & this.CrowdingDefCount > 0)
          {
            if (this.CrowdingDefCount < DrawMod.RandyNumber.Next(0, (int) Math.Round( (this.game.Data.RuleVar[30] * 2f))))
            {
              opponent = -1;
              if (!this.game.Data.FOWOn)
                this.AddReport(0, "No lucky hit", "Individual " + this.game.Data.SFTypeObj[this.IList[inr].ISFType].Name + " did not manage to score a lucky hit [Hidden]", inr + 10000, this.CombatRound);
            }
          }
          else if (this.IList[inr1].IAttacker == 1 & num9 > 0)
          {
            if (num9 >= DrawMod.RandyNumber.Next(0, (int) Math.Round( (num8 * 2f))))
            {
              opponent = opponent;
            }
            else
            {
              opponent = -1;
              if (!this.game.Data.FOWOn)
                this.AddReport(0, "No lucky hit", "Individual " + this.game.Data.SFTypeObj[this.IList[inr].ISFType].Name + " did not manage to score a lucky hit [Hidden]", inr + 10000, this.CombatRound);
            }
          }
          else
            opponent = -1;
        }
      }
      return opponent;
    }

    pub fn ThrowInitiative()
    {
      let mut icounter: i32 =  this.ICounter;
      for (let mut inr: i32 =  0; inr <= icounter; inr += 1)
      {
        if (this.TestAttack(inr))
        {
          let mut num1: i32 =  this.IList[inr].IAttacker != 1 ? this.game.Data.SFTypeObj[this.IList[inr].ISFType].InitiativeDef : this.game.Data.SFTypeObj[this.IList[inr].ISFType].Initiative;
          if (this.IList[inr].ILastAttackDone < this.CombatRound - 1 & this.CombatRound > 1)
            num1 += num1 * (this.CombatRound - 1 - this.IList[inr].ILastAttackDone);
          let mut num2: i32 =  (int) Math.Round( Conversion.Int(VBMath.Rnd() *  num1));
          this.IList[inr].IInitThrow = num2;
          this.IList[inr].IHistoricInit[this.CombatRound] = num2;
        }
        else
        {
          this.IList[inr].IInitThrow = 0;
          this.IList[inr].IHistoricInit[this.CombatRound] = 0;
        }
      }
    }

    pub TestAttack: bool(inr: i32) => this.IList[inr].IAttacker == 1 & this.CombatType == 13 ? this.game.Data.SFTypeObj[this.IList[inr].ISFType].AutoDestroy : !(this.IList[inr].IAttacker == 0 & this.InterceptFire) && this.IList[inr].IKilled == 0 && this.IList[inr].IRetreat == 0 && this.IList[inr].IRetreated == 0 && this.IList[inr].AttackCount < this.game.Data.SFTypeObj[this.IList[inr].ISFType].Attacks;

    pub TestTarget: bool(attnr: i32, defnr: i32)
    {
      if (this.IList[attnr].IAttacker == this.IList[defnr].IAttacker || this.InterceptFire && this.IList[attnr].IAttacker == 0)
        return false;
      if (this.CombatType == 3 | this.CombatType == 4 && this.IList[attnr].IAttacker == 0)
      {
        let mut x1: i32 =  this.game.Data.UnitObj[this.IList[attnr].IUnr].X;
        let mut y1: i32 =  this.game.Data.UnitObj[this.IList[attnr].IUnr].Y;
        let mut map1: i32 =  this.game.Data.UnitObj[this.IList[attnr].IUnr].Map;
        let mut x2: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].X;
        let mut y2: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].Y;
        let mut map2: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].Map;
        if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange < this.game.HandyFunctionsObj.Distance(x1, y1, map1, x2, y2, map2) && (!( this.game.Data.RuleVar[419] > 0.0 &  this.game.Data.RuleVar[429] > 0.0) || this.game.Data.SFTypeObj[this.IList[attnr].ISFType].directRange < this.game.HandyFunctionsObj.Distance(x1, y1, map1, x2, y2, map2) || Information.IsNothing( this.IList[attnr].IdirectFireDef) || !this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr]))
          return false;
      }
      if (this.IList[defnr].IAA == 1 || this.IList[defnr].IParadropper & this.CombatRound < 6 || this.IList[defnr].IRetreatMode == 1 && this.game.Data.Product != 6 && this.IList[defnr].IRetreat < this.CombatRound)
        return false;
      if (this.CombatType == 3 | this.CombatType == 4)
      {
        let mut unitGroup: i32 =  this.game.Data.SFTypeObj[this.IList[defnr].ISFType].UnitGroup;
        if (Conversions.ToInteger(this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackArt[unitGroup]) == 0 && ( this.game.Data.RuleVar[419] <= 0.0 || this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPower[unitGroup] == 0))
          return false;
      }
      else if (this.IList[attnr].IAttacker == 1)
      {
        if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPower[this.game.Data.SFTypeObj[this.IList[defnr].ISFType].UnitGroup] <= 0)
          return false;
      }
      else if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].AttackPowerDef[this.game.Data.SFTypeObj[this.IList[defnr].ISFType].UnitGroup] <= 0)
        return false;
      if (this.IList[defnr].IKilled != 0 || this.IList[defnr].IRetreated != 0 || this.IList[attnr].IAttacker == this.IList[defnr].IAttacker)
        return false;
      let mut num1: i32 =  1;
      if (this.UList[this.IList[defnr].IUlistNr].USupportInterceptFire)
        ;
      if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench && this.IList[defnr].IRetreat == 0)
      {
        if (this.IList[attnr].IBreakTrough == 0 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange < 1)
        {
          if ( this.game.Data.RuleVar[419] > 0.0)
          {
            let mut x3: i32 =  this.game.Data.UnitObj[this.IList[attnr].IUnr].X;
            let mut y3: i32 =  this.game.Data.UnitObj[this.IList[attnr].IUnr].Y;
            let mut map3: i32 =  this.game.Data.UnitObj[this.IList[attnr].IUnr].Map;
            let mut x4: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].X;
            let mut y4: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].Y;
            let mut map4: i32 =  this.game.Data.UnitObj[this.IList[defnr].IUnr].Map;
            if (this.CombatType == 3)
            {
              if (!(this.IList[attnr].IAttacker == 1 & this.InterceptFire))
              {
                if (this.IList[attnr].IAttacker == 1 &  this.IList[attnr].IdirectMod > 0.0)
                {
                  let mut num2: i32 =  this.game.Data.MapObj[0].HexObj[x3, y3].HeightLevel - this.game.Data.MapObj[0].HexObj[x4, y4].HeightLevel;
                  let mut num3: i32 =  70 - (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[x4, y4].LandscapeType].HidePts + 10) * (3 - num2);
                  num4: i32;
                  if (!Information.IsNothing( this.UList[this.IList[attnr].IUlistNr].ULos))
                    num4 = (int) Math.Round( (num3 * this.UList[this.IList[attnr].IUlistNr].ULos[this.IList[defnr].IUlistNr]) / 100.0);
                  if (new Random(attnr * this.CombatRound).Next(1, 100) >= num4)
                    return false;
                }
                else if (this.IList[attnr].IAttacker == 0)
                {
                  if (this.IList[attnr].IdirectFireDef[this.IList[defnr].IUlistNr])
                  {
                    let mut num5: i32 =  this.game.Data.MapObj[0].HexObj[x3, y3].HeightLevel - this.game.Data.MapObj[0].HexObj[x4, y4].HeightLevel;
                    let mut num6: i32 =  70 - (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[x4, y4].LandscapeType].HidePts + 10) * (3 - num5);
                    if (num6 < 0)
                      num6 = 0;
                    num7: i32;
                    if (!Information.IsNothing( this.UList[this.IList[attnr].IUlistNr].ULos))
                      num7 = (int) Math.Round( (num6 * this.UList[this.IList[attnr].IUlistNr].ULos[this.IList[defnr].IUlistNr]) / 100.0);
                    if (new Random(attnr * this.CombatRound).Next(1, 100) >= num7)
                      return false;
                  }
                  else
                    num1 = 0;
                }
                else
                  num1 = 0;
              }
            }
            else if (this.CombatType == 1)
            {
              if (this.IList[attnr].IAttacker == 1 & this.UList[this.IList[defnr].IUlistNr].USupportInterceptFire)
                return false;
              let mut num8: i32 =  this.game.Data.MapObj[0].HexObj[x3, y3].HeightLevel - this.game.Data.MapObj[0].HexObj[x4, y4].HeightLevel;
              let mut num9: i32 =  100 - (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[x4, y4].LandscapeType].HidePts + 10) * (3 - num8);
              if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].ArtRange > 0)
                num9 = (int) Math.Round( num9 / 4.0);
              else if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].directRange > 0)
                num9 = (int) Math.Round( num9 / 2.0);
              if (new Random(attnr * this.CombatRound).Next(1, 100) >= num9)
                return false;
            }
            else
              num1 = 0;
          }
          else
            num1 = 0;
        }
        if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater != 2)
        {
          if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 2)
            num1 = 1;
        }
        else if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater != 2)
        {
          if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2)
            num1 = 1;
        }
        else if (this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2)
        {
          if (this.IList[defnr].IAttacker == 0 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 2)
            num1 = 1;
        }
        else if (!(this.CombatType == 3 | this.CombatType == 4) && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater != 2)
          num1 = 1;
      }
      if (this.IList[attnr].IAttacker == 1 & this.UList[this.IList[defnr].IUlistNr].USupportInterceptFire)
        return false;
      if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 0 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].ArtRange > 0 && !this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 0 && this.IList[defnr].IBreakTrough > 0)
        num1 = 0;
      if (this.CombatType == 2 && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2 && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 1 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.TargetX, this.TargetY].LandscapeType].IsSea)
        return false;
      if (this.game.Data.SFTypeObj[this.IList[attnr].ISFType].BackBench && this.game.Data.SFTypeObj[this.IList[attnr].ISFType].Theater == 2 && !this.game.Data.SFTypeObj[this.IList[defnr].ISFType].BackBench && this.game.Data.SFTypeObj[this.IList[defnr].ISFType].Theater == 2)
        num1 = 0;
      return num1 == 1;
    }

    pub fn SortOnInitiative()
    {
      let mut icounter: i32 =  this.ICounter;
      for (let mut index1: i32 =  0; index1 <= icounter; index1 += 1)
      {
        let mut num: i32 =  this.ICounter - 1;
        for (let mut index2: i32 =  0; index2 <= num; index2 += 1)
        {
          if (this.IList[index2].IInitThrow < this.IList[index2 + 1].IInitThrow)
          {
            Individual individual = this.IList[index2];
            this.IList[index2] = this.IList[index2 + 1];
            this.IList[index2 + 1] = individual;
          }
        }
      }
    }

    pub fn AddDetail(s: String)
    {
      if (!this.game.EditObj.PrefCombatLog)
        return;
      this += 1.DetailCounter;
      this.DetailString = (string[]) Utils.CopyArray((Array) this.DetailString, (Array) new string[this.DetailCounter + 1]);
      this.DetailString[this.DetailCounter] = s;
      this.AddAllDetail(s);
    }

    pub fn AddAllDetail(s: String)
    {
      if (!this.game.EditObj.PrefCombatLog)
        return;
      this += 1.AllDetailCounter;
      this.AllDetailString = (string[]) Utils.CopyArray((Array) this.AllDetailString, (Array) new string[this.AllDetailCounter + 1]);
      this.AllDetailString[this.AllDetailCounter] = s;
    }

    pub fn AddBiggy(s: String)
    {
      if (!this.game.EditObj.PrefCombatLog)
        return;
      this.AddDetail(s);
      this += 1.BattleCounter;
      this.BattleString = (object[]) Utils.CopyArray((Array) this.BattleString, (Array) new object[this.BattleCounter + 1]);
      this.BattleString[this.BattleCounter] =  s;
    }

    pub fn ClearDetail()
    {
      this.DetailCounter = -1;
      this.DetailString = new string[1];
    }

    pub fn WriteToFileLog()
    {
      if (!this.game.EditObj.PrefCombatLog)
        return;
      StreamWriter streamWriter = this.CombatType != 12 ? File.CreateText(this.game.AppPath + "logs/lastbattle_Log.txt") : File.CreateText(this.game.AppPath + "logs/lastRebel_Log.txt");
      let mut allDetailCounter: i32 =  this.AllDetailCounter;
      for (let mut index: i32 =  0; index <= allDetailCounter; index += 1)
        streamWriter.WriteLine(this.AllDetailString[index]);
      streamWriter.Close();
    }
  }
}
