// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatAnalysis
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;

namespace WindowsApplication1
{
  pub class CombatAnalysis
  {
    pub int[,,] stats;
    pub int[,,] ustats;
    pub game: GameClass;

    pub CombatAnalysis(let mut tgame: GameClass = null)
    {
      this.stats = new int[2, 41, 8];
      this.ustats = new int[100, 41, 8];
      this.game = tgame;
    }

    pub fn analyse(bool attlog, bool deflog)
    {
      let mut attackerRegime: i32 =  this.game.TempCombat.AttackerRegime;
      let mut defenderRegime: i32 =  this.game.TempCombat.DefenderRegime;
      let mut icounter: i32 =  this.game.TempCombat.ICounter;
      iulistNr: i32;
      for (let mut index1: i32 =  0; index1 <= icounter; index1 += 1)
      {
        let mut iattacker: i32 =  this.game.TempCombat.IList[index1].IAttacker;
        iulistNr = this.game.TempCombat.IList[index1].IUlistNr;
        let mut unitGroup: i32 =  this.game.Data.SFTypeObj[this.game.TempCombat.IList[index1].ISFType].UnitGroup;
        if (this.game.TempCombat.IList[index1].IKilled > 0)
        {
          int[,,] stats1 = this.stats;
          int[,,] numArray1 = stats1;
          let mut index2: i32 =  iattacker;
          let mut index3: i32 =  index2;
          let mut index4: i32 =  unitGroup;
          let mut index5: i32 =  index4;
          let mut index6: i32 =  4;
          let mut index7: i32 =  index6;
          let mut num1: i32 =  stats1[index2, index4, index6] + 1;
          numArray1[index3, index5, index7] = num1;
          int[,,] stats2 = this.stats;
          int[,,] numArray2 = stats2;
          let mut index8: i32 =  iattacker;
          let mut index9: i32 =  index8;
          let mut index10: i32 =  40;
          let mut index11: i32 =  index10;
          let mut index12: i32 =  4;
          let mut index13: i32 =  index12;
          let mut num2: i32 =  stats2[index8, index10, index12] + 1;
          numArray2[index9, index11, index13] = num2;
          int[,,] stats3 = this.stats;
          int[,,] numArray3 = stats3;
          let mut index14: i32 =  iattacker;
          let mut index15: i32 =  index14;
          let mut index16: i32 =  unitGroup;
          let mut index17: i32 =  index16;
          let mut index18: i32 =  7;
          let mut index19: i32 =  index18;
          let mut num3: i32 =  stats3[index14, index16, index18] + 1;
          numArray3[index15, index17, index19] = num3;
          int[,,] ustats1 = this.ustats;
          int[,,] numArray4 = ustats1;
          let mut index20: i32 =  iulistNr;
          let mut index21: i32 =  index20;
          let mut index22: i32 =  unitGroup;
          let mut index23: i32 =  index22;
          let mut index24: i32 =  4;
          let mut index25: i32 =  index24;
          let mut num4: i32 =  ustats1[index20, index22, index24] + 1;
          numArray4[index21, index23, index25] = num4;
          int[,,] ustats2 = this.ustats;
          int[,,] numArray5 = ustats2;
          let mut index26: i32 =  iulistNr;
          let mut index27: i32 =  index26;
          let mut index28: i32 =  40;
          let mut index29: i32 =  index28;
          let mut index30: i32 =  4;
          let mut index31: i32 =  index30;
          let mut num5: i32 =  ustats2[index26, index28, index30] + 1;
          numArray5[index27, index29, index31] = num5;
          int[,,] ustats3 = this.ustats;
          int[,,] numArray6 = ustats3;
          let mut index32: i32 =  iulistNr;
          let mut index33: i32 =  index32;
          let mut index34: i32 =  unitGroup;
          let mut index35: i32 =  index34;
          let mut index36: i32 =  7;
          let mut index37: i32 =  index36;
          let mut num6: i32 =  ustats3[index32, index34, index36] + 1;
          numArray6[index33, index35, index37] = num6;
        }
        else if (this.game.TempCombat.IList[index1].IRetreated == 1)
        {
          int[,,] stats4 = this.stats;
          int[,,] numArray7 = stats4;
          let mut index38: i32 =  iattacker;
          let mut index39: i32 =  index38;
          let mut index40: i32 =  unitGroup;
          let mut index41: i32 =  index40;
          let mut index42: i32 =  1;
          let mut index43: i32 =  index42;
          let mut num7: i32 =  stats4[index38, index40, index42] + 1;
          numArray7[index39, index41, index43] = num7;
          int[,,] stats5 = this.stats;
          int[,,] numArray8 = stats5;
          let mut index44: i32 =  iattacker;
          let mut index45: i32 =  index44;
          let mut index46: i32 =  40;
          let mut index47: i32 =  index46;
          let mut index48: i32 =  1;
          let mut index49: i32 =  index48;
          let mut num8: i32 =  stats5[index44, index46, index48] + 1;
          numArray8[index45, index47, index49] = num8;
          int[,,] stats6 = this.stats;
          int[,,] numArray9 = stats6;
          let mut index50: i32 =  iattacker;
          let mut index51: i32 =  index50;
          let mut index52: i32 =  unitGroup;
          let mut index53: i32 =  index52;
          let mut index54: i32 =  7;
          let mut index55: i32 =  index54;
          let mut num9: i32 =  stats6[index50, index52, index54] + 1;
          numArray9[index51, index53, index55] = num9;
          int[,,] ustats4 = this.ustats;
          int[,,] numArray10 = ustats4;
          let mut index56: i32 =  iulistNr;
          let mut index57: i32 =  index56;
          let mut index58: i32 =  unitGroup;
          let mut index59: i32 =  index58;
          let mut index60: i32 =  1;
          let mut index61: i32 =  index60;
          let mut num10: i32 =  ustats4[index56, index58, index60] + 1;
          numArray10[index57, index59, index61] = num10;
          int[,,] ustats5 = this.ustats;
          int[,,] numArray11 = ustats5;
          let mut index62: i32 =  iulistNr;
          let mut index63: i32 =  index62;
          let mut index64: i32 =  40;
          let mut index65: i32 =  index64;
          let mut index66: i32 =  1;
          let mut index67: i32 =  index66;
          let mut num11: i32 =  ustats5[index62, index64, index66] + 1;
          numArray11[index63, index65, index67] = num11;
          int[,,] ustats6 = this.ustats;
          int[,,] numArray12 = ustats6;
          let mut index68: i32 =  iulistNr;
          let mut index69: i32 =  index68;
          let mut index70: i32 =  unitGroup;
          let mut index71: i32 =  index70;
          let mut index72: i32 =  7;
          let mut index73: i32 =  index72;
          let mut num12: i32 =  ustats6[index68, index70, index72] + 1;
          numArray12[index69, index71, index73] = num12;
        }
        else if (this.game.TempCombat.IList[index1].IRetreat > 0 & this.game.TempCombat.IList[index1].IRetreated == 0)
        {
          int[,,] stats7 = this.stats;
          int[,,] numArray13 = stats7;
          let mut index74: i32 =  iattacker;
          let mut index75: i32 =  index74;
          let mut index76: i32 =  unitGroup;
          let mut index77: i32 =  index76;
          let mut index78: i32 =  5;
          let mut index79: i32 =  index78;
          let mut num13: i32 =  stats7[index74, index76, index78] + 1;
          numArray13[index75, index77, index79] = num13;
          int[,,] stats8 = this.stats;
          int[,,] numArray14 = stats8;
          let mut index80: i32 =  iattacker;
          let mut index81: i32 =  index80;
          let mut index82: i32 =  40;
          let mut index83: i32 =  index82;
          let mut index84: i32 =  5;
          let mut index85: i32 =  index84;
          let mut num14: i32 =  stats8[index80, index82, index84] + 1;
          numArray14[index81, index83, index85] = num14;
          int[,,] stats9 = this.stats;
          int[,,] numArray15 = stats9;
          let mut index86: i32 =  iattacker;
          let mut index87: i32 =  index86;
          let mut index88: i32 =  unitGroup;
          let mut index89: i32 =  index88;
          let mut index90: i32 =  7;
          let mut index91: i32 =  index90;
          let mut num15: i32 =  stats9[index86, index88, index90] + 1;
          numArray15[index87, index89, index91] = num15;
          int[,,] ustats7 = this.ustats;
          int[,,] numArray16 = ustats7;
          let mut index92: i32 =  iulistNr;
          let mut index93: i32 =  index92;
          let mut index94: i32 =  unitGroup;
          let mut index95: i32 =  index94;
          let mut index96: i32 =  5;
          let mut index97: i32 =  index96;
          let mut num16: i32 =  ustats7[index92, index94, index96] + 1;
          numArray16[index93, index95, index97] = num16;
          int[,,] ustats8 = this.ustats;
          int[,,] numArray17 = ustats8;
          let mut index98: i32 =  iulistNr;
          let mut index99: i32 =  index98;
          let mut index100: i32 =  40;
          let mut index101: i32 =  index100;
          let mut index102: i32 =  5;
          let mut index103: i32 =  index102;
          let mut num17: i32 =  ustats8[index98, index100, index102] + 1;
          numArray17[index99, index101, index103] = num17;
          int[,,] ustats9 = this.ustats;
          int[,,] numArray18 = ustats9;
          let mut index104: i32 =  iulistNr;
          let mut index105: i32 =  index104;
          let mut index106: i32 =  unitGroup;
          let mut index107: i32 =  index106;
          let mut index108: i32 =  7;
          let mut index109: i32 =  index108;
          let mut num18: i32 =  ustats9[index104, index106, index108] + 1;
          numArray18[index105, index107, index109] = num18;
        }
        else
        {
          int[,,] stats10 = this.stats;
          int[,,] numArray19 = stats10;
          let mut index110: i32 =  iattacker;
          let mut index111: i32 =  index110;
          let mut index112: i32 =  unitGroup;
          let mut index113: i32 =  index112;
          let mut index114: i32 =  6;
          let mut index115: i32 =  index114;
          let mut num19: i32 =  stats10[index110, index112, index114] + 1;
          numArray19[index111, index113, index115] = num19;
          int[,,] stats11 = this.stats;
          int[,,] numArray20 = stats11;
          let mut index116: i32 =  iattacker;
          let mut index117: i32 =  index116;
          let mut index118: i32 =  40;
          let mut index119: i32 =  index118;
          let mut index120: i32 =  6;
          let mut index121: i32 =  index120;
          let mut num20: i32 =  stats11[index116, index118, index120] + 1;
          numArray20[index117, index119, index121] = num20;
          int[,,] stats12 = this.stats;
          int[,,] numArray21 = stats12;
          let mut index122: i32 =  iattacker;
          let mut index123: i32 =  index122;
          let mut index124: i32 =  unitGroup;
          let mut index125: i32 =  index124;
          let mut index126: i32 =  7;
          let mut index127: i32 =  index126;
          let mut num21: i32 =  stats12[index122, index124, index126] + 1;
          numArray21[index123, index125, index127] = num21;
          int[,,] ustats10 = this.ustats;
          int[,,] numArray22 = ustats10;
          let mut index128: i32 =  iulistNr;
          let mut index129: i32 =  index128;
          let mut index130: i32 =  unitGroup;
          let mut index131: i32 =  index130;
          let mut index132: i32 =  6;
          let mut index133: i32 =  index132;
          let mut num22: i32 =  ustats10[index128, index130, index132] + 1;
          numArray22[index129, index131, index133] = num22;
          int[,,] ustats11 = this.ustats;
          int[,,] numArray23 = ustats11;
          let mut index134: i32 =  iulistNr;
          let mut index135: i32 =  index134;
          let mut index136: i32 =  40;
          let mut index137: i32 =  index136;
          let mut index138: i32 =  6;
          let mut index139: i32 =  index138;
          let mut num23: i32 =  ustats11[index134, index136, index138] + 1;
          numArray23[index135, index137, index139] = num23;
          int[,,] ustats12 = this.ustats;
          int[,,] numArray24 = ustats12;
          let mut index140: i32 =  iulistNr;
          let mut index141: i32 =  index140;
          let mut index142: i32 =  unitGroup;
          let mut index143: i32 =  index142;
          let mut index144: i32 =  7;
          let mut index145: i32 =  index144;
          let mut num24: i32 =  ustats12[index140, index142, index144] + 1;
          numArray24[index141, index143, index145] = num24;
        }
        if (this.game.TempCombat.IList[index1].IBreakTrough > 0 & this.game.TempCombat.IList[index1].IRetreated == 0 & this.game.TempCombat.IList[index1].IKilled == 0 & this.game.TempCombat.IList[index1].IRetreat == 0)
        {
          int[,,] stats13 = this.stats;
          int[,,] numArray25 = stats13;
          let mut index146: i32 =  iattacker;
          let mut index147: i32 =  index146;
          let mut index148: i32 =  unitGroup;
          let mut index149: i32 =  index148;
          let mut index150: i32 =  0;
          let mut index151: i32 =  index150;
          let mut num25: i32 =  stats13[index146, index148, index150] + 1;
          numArray25[index147, index149, index151] = num25;
          int[,,] stats14 = this.stats;
          int[,,] numArray26 = stats14;
          let mut index152: i32 =  iattacker;
          let mut index153: i32 =  index152;
          let mut index154: i32 =  40;
          let mut index155: i32 =  index154;
          let mut index156: i32 =  0;
          let mut index157: i32 =  index156;
          let mut num26: i32 =  stats14[index152, index154, index156] + 1;
          numArray26[index153, index155, index157] = num26;
          int[,,] stats15 = this.stats;
          int[,,] numArray27 = stats15;
          let mut index158: i32 =  iattacker;
          let mut index159: i32 =  index158;
          let mut index160: i32 =  unitGroup;
          let mut index161: i32 =  index160;
          let mut index162: i32 =  7;
          let mut index163: i32 =  index162;
          let mut num27: i32 =  stats15[index158, index160, index162] + 1;
          numArray27[index159, index161, index163] = num27;
          int[,,] ustats13 = this.ustats;
          int[,,] numArray28 = ustats13;
          let mut index164: i32 =  iulistNr;
          let mut index165: i32 =  index164;
          let mut index166: i32 =  unitGroup;
          let mut index167: i32 =  index166;
          let mut index168: i32 =  0;
          let mut index169: i32 =  index168;
          let mut num28: i32 =  ustats13[index164, index166, index168] + 1;
          numArray28[index165, index167, index169] = num28;
          int[,,] ustats14 = this.ustats;
          int[,,] numArray29 = ustats14;
          let mut index170: i32 =  iulistNr;
          let mut index171: i32 =  index170;
          let mut index172: i32 =  40;
          let mut index173: i32 =  index172;
          let mut index174: i32 =  0;
          let mut index175: i32 =  index174;
          let mut num29: i32 =  ustats14[index170, index172, index174] + 1;
          numArray29[index171, index173, index175] = num29;
          int[,,] ustats15 = this.ustats;
          int[,,] numArray30 = ustats15;
          let mut index176: i32 =  iulistNr;
          let mut index177: i32 =  index176;
          let mut index178: i32 =  unitGroup;
          let mut index179: i32 =  index178;
          let mut index180: i32 =  7;
          let mut index181: i32 =  index180;
          let mut num30: i32 =  ustats15[index176, index178, index180] + 1;
          numArray30[index177, index179, index181] = num30;
        }
      }
      let mut ucounter: i32 =  this.game.TempCombat.UCounter;
      for (let mut index: i32 =  0; index <= ucounter; index += 1)
      {
        let mut num: i32 =  0;
        if (this.game.TempCombat.UList[index].Uattacker == 0 & !deflog)
          num = 1;
        if (this.game.TempCombat.UList[index].Uattacker == 1 & !attlog)
          num = 1;
        if (this.game.TempCombat.BattleEnded <= 0)
          num = 1;
        if (num == 0)
          "Lost " + Conversion.Str( (this.ustats[iulistNr, 40, 4] + this.ustats[iulistNr, 40, 2])) + " troops in combat at " + Conversion.Str( this.game.TempCombat.CombatTarget.x) + "," + Conversion.Str( this.game.TempCombat.CombatTarget.y);
      }
    }
  }
}
