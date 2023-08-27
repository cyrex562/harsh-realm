// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatResultWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class CombatResultWindowClass : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     b1textid: i32;
     b2textid: i32;
     txt1id: i32;
     txt2id: i32;
     txt3id: i32;
     txt4id: i32;
     txt5id: i32;
     txt6id: i32;
     txt7id: i32;
     txt8id: i32;
     CombatListId: i32;
     ListClass CombatListObj;
     ResolveId: i32;
     CombatListDId: i32;
     ListClass CombatListDObj;
     detailnr: i32;
     detailnr2: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     DetailId: i32;
     showdetail: i32;
     DateTime lasttime;
     int[] modid;
     LogTxt: Vec<String>;
     LogCounter: i32;
     UnitA: Vec<i32>;
     UnitL: Vec<i32>;
     Bitmap[,] UnitBitMap;
     UnitC: i32;
     int[] StartRdn;
     int[] StartEntr;
     int[] StartMor;
     int[] StartXp;
     bool ForwardKey;

    pub CombatResultWindowClass(ref tGame: GameClass)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.modid = new int[61];
      this.LogTxt = new string[1];
      this.UnitA = new int[2, 200];
      this.UnitL = new int[2, 200];
      this.UnitBitMap = new Bitmap[2, 200];
      this.StartRdn = new int[2];
      this.StartEntr = new int[2];
      this.StartMor = new int[2];
      this.StartXp = new int[2];
      this.showdetail = 0;
      this.dostuff();
      this.lasttime = DateAndTime.Now;
      if (!this.game.EditObj.CombatSim)
        return;
      this.DoCombatSim();
      this.dostuff();
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn dostuff()
    {
      new CombatAnalysis(this.game).analyse(false, true);
      if (this.CombatListId > 0)
        this.RemoveSubPart(this.CombatListId);
      if (this.showdetail == 0 & this.ResolveId > 0)
      {
        this.RemoveSubPart(this.ResolveId);
        this.ResolveId = 0;
      }
      if (this.CombatListDId > 0)
        this.RemoveSubPart(this.CombatListDId);
      if (this.b1textid > 0)
        this.RemoveSubPart(this.b1textid);
      if (this.b2textid > 0)
        this.RemoveSubPart(this.b2textid);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B4TextId > 0)
        this.RemoveSubPart(this.B4TextId);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B5TextId > 0)
        this.RemoveSubPart(this.B5TextId);
      if (this.B6Id > 0)
        this.RemoveSubPart(this.B6Id);
      if (this.B6TextId > 0)
        this.RemoveSubPart(this.B6TextId);
      if (this.DetailId > 0)
        this.RemoveSubPart(this.DetailId);
      if (this.txt1id > 0)
        this.RemoveSubPart(this.txt1id);
      if (this.txt2id > 0)
        this.RemoveSubPart(this.txt2id);
      if (this.txt3id > 0)
        this.RemoveSubPart(this.txt3id);
      if (this.txt4id > 0)
        this.RemoveSubPart(this.txt4id);
      if (this.txt5id > 0)
        this.RemoveSubPart(this.txt5id);
      if (this.txt6id > 0)
        this.RemoveSubPart(this.txt6id);
      if (this.txt7id > 0)
        this.RemoveSubPart(this.txt7id);
      if (this.txt8id > 0)
        this.RemoveSubPart(this.txt8id);
      let mut index1: i32 =  0;
      do
      {
        if (this.modid[index1] > 0)
          this.RemoveSubPart(this.modid[index1]);
        index1 += 1;
      }
      while (index1 <= 60);
      let mut index2: i32 =  0;
      do
      {
        this.UnitA[0, index2] = -1;
        this.UnitA[1, index2] = -1;
        index2 += 1;
      }
      while (index2 <= 199);
      this.UnitC = -1;
      let mut num1: i32 =  1;
      let mut index3: i32 =  -1;
      let mut index4: i32 =  -1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut index5: i32 =  0; index5 <= icounter; index5 += 1)
        {
          if (this.game.TempCombat.IList[index5].IAttacker == 1)
          {
            let mut num2: i32 =  0;
            let mut num3: i32 =  index3;
            for (let mut index6: i32 =  0; index6 <= num3; index6 += 1)
            {
              if (this.UnitA[1, index6] == this.game.TempCombat.IList[index5].IUnr)
              {
                num2 = 1;
                break;
              }
            }
            if (num2 == 0)
            {
              index3 += 1;
              this.UnitA[1, index3] = this.game.TempCombat.IList[index5].IUnr;
              this.UnitBitMap[1, index3] = (Bitmap) this.game.CustomBitmapObj.DrawUnit(this.game.TempCombat.IList[index5].IUnr).Clone();
              this.UnitL[1, index3] = this.game.TempCombat.IList[index5].IUlistNr;
              if (index3 > this.UnitC)
                this.UnitC = index3;
            }
          }
        }
      }
      let mut num4: i32 =  1;
      let mut index7: i32 =  -1;
      while (num4 == 1)
      {
        num4 = 0;
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut index8: i32 =  0; index8 <= icounter; index8 += 1)
        {
          if (this.game.TempCombat.IList[index8].IAttacker == 0)
          {
            let mut num5: i32 =  0;
            let mut num6: i32 =  index7;
            for (let mut index9: i32 =  0; index9 <= num6; index9 += 1)
            {
              if (this.UnitA[0, index9] == this.game.TempCombat.IList[index8].IUnr)
              {
                num5 = 1;
                break;
              }
            }
            if (num5 == 0)
            {
              index7 += 1;
              index4 += 1;
              this.UnitA[0, index7] = this.game.TempCombat.IList[index8].IUnr;
              this.UnitBitMap[0, index7] = (Bitmap) this.game.CustomBitmapObj.DrawUnit(this.game.TempCombat.IList[index8].IUnr).Clone();
              this.UnitL[0, index7] = this.game.TempCombat.IList[index8].IUlistNr;
              if (index7 > this.UnitC)
                this.UnitC = index7;
            }
          }
        }
      }
      let mut unitCounter: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].UnitCounter;
      for (let mut index10: i32 =  0; index10 <= unitCounter; index10 += 1)
      {
        let mut unit: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].UnitList[index10];
        let mut num7: i32 =  0;
        let mut num8: i32 =  index7;
        for (let mut index11: i32 =  0; index11 <= num8; index11 += 1)
        {
          if (this.UnitA[0, index11] == unit)
          {
            num7 = 1;
            break;
          }
        }
        if (num7 == 0 && this.game.Data.UnitObj[unit].Regime == this.game.TempCombat.DefenderRegime)
        {
          index4 += 1;
          this.UnitA[0, index4] = unit;
          this.UnitBitMap[0, index4] = (Bitmap) this.game.CustomBitmapObj.DrawUnit(unit).Clone();
          this.UnitL[0, index4] = -1;
          if (index4 > this.UnitC)
            this.UnitC = index4;
        }
      }
      this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND2MARC);
      Graphics Expression1 = Graphics.FromImage((Image) this.OwnBitmap);
      tText: String;
      SubPartClass tsubpart;
      if (!this.game.EditObj.CombatSim)
      {
        numArray1: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
        numArray2: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
        numArray3: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
        int[] numArray4 = new int[2];
        int[] numArray5 = new int[2];
        int[] numArray6 = new int[2];
        int[] numArray7 = new int[2];
        int[] numArray8 = new int[2];
        int[] numArray9 = new int[2];
        int[] numArray10 = new int[2];
        int[] numArray11 = new int[2];
        let mut num9: i32 =  0;
        let mut num10: i32 =  0;
        numArray4[0] = 0;
        numArray4[1] = 0;
        numArray5[0] = 0;
        numArray5[1] = 0;
        numArray6[0] = 0;
        numArray6[1] = 0;
        numArray7[0] = 0;
        numArray7[1] = 0;
        numArray8[0] = 0;
        numArray8[1] = 0;
        numArray9[0] = 0;
        numArray9[1] = 0;
        numArray10[0] = 0;
        numArray10[1] = 0;
        numArray11[0] = 0;
        numArray11[1] = 0;
        let mut icounter1: i32 =  this.game.TempCombat.ICounter;
        index12: i32;
        for (let mut index13: i32 =  0; index13 <= icounter1; index13 += 1)
        {
          let mut iattacker: i32 =  this.game.TempCombat.IList[index13].IAttacker;
          index12 = this.game.TempCombat.IList[index13].ISFType;
          if (this.game.TempCombat.IList[index13].IKilled > 0)
          {
            numArray12: Vec<i32> = numArray1;
            numArray13: Vec<i32> = numArray12;
            let mut index14: i32 =  index12;
            let mut index15: i32 =  index14;
            let mut index16: i32 =  iattacker;
            let mut index17: i32 =  index16;
            let mut num11: i32 =  numArray12[index14, index16] + 1;
            numArray13[index15, index17] = num11;
            int[] numArray14 = numArray8;
            int[] numArray15 = numArray14;
            let mut index18: i32 =  iattacker;
            let mut index19: i32 =  index18;
            let mut num12: i32 =  numArray14[index18] + this.game.Data.SFTypeObj[index12].PowerPts;
            numArray15[index19] = num12;
          }
          else if (this.game.TempCombat.IList[index13].IRetreat > 0)
          {
            numArray16: Vec<i32> = numArray2;
            numArray17: Vec<i32> = numArray16;
            let mut index20: i32 =  index12;
            let mut index21: i32 =  index20;
            let mut index22: i32 =  iattacker;
            let mut index23: i32 =  index22;
            let mut num13: i32 =  numArray16[index20, index22] + 1;
            numArray17[index21, index23] = num13;
            int[] numArray18 = numArray9;
            int[] numArray19 = numArray18;
            let mut index24: i32 =  iattacker;
            let mut index25: i32 =  index24;
            let mut num14: i32 =  numArray18[index24] + this.game.Data.SFTypeObj[index12].PowerPts;
            numArray19[index25] = num14;
          }
          else
          {
            numArray20: Vec<i32> = numArray3;
            numArray21: Vec<i32> = numArray20;
            let mut index26: i32 =  index12;
            let mut index27: i32 =  index26;
            let mut index28: i32 =  iattacker;
            let mut index29: i32 =  index28;
            let mut num15: i32 =  numArray20[index26, index28] + 1;
            numArray21[index27, index29] = num15;
            int[] numArray22 = numArray10;
            int[] numArray23 = numArray22;
            let mut index30: i32 =  iattacker;
            let mut index31: i32 =  index30;
            let mut num16: i32 =  numArray22[index30] + this.game.Data.SFTypeObj[index12].PowerPts;
            numArray23[index31] = num16;
          }
          if (this.game.TempCombat.IList[index13].IKilled <= 0)
          {
            if (this.game.TempCombat.CombatRound == 0)
            {
              int[] numArray24 = numArray11;
              int[] numArray25 = numArray24;
              let mut index32: i32 =  iattacker;
              let mut index33: i32 =  index32;
              let mut num17: i32 =  numArray24[index32] + this.game.Data.SFTypeObj[index12].PowerPts;
              numArray25[index33] = num17;
              int[] startRdn = this.StartRdn;
              int[] numArray26 = startRdn;
              let mut index34: i32 =  iattacker;
              let mut index35: i32 =  index34;
              let mut num18: i32 =  startRdn[index34] + this.game.TempCombat.IList[index13].IRdn;
              numArray26[index35] = num18;
              int[] startXp = this.StartXp;
              int[] numArray27 = startXp;
              let mut index36: i32 =  iattacker;
              let mut index37: i32 =  index36;
              let mut num19: i32 =  startXp[index36] + this.game.TempCombat.IList[index13].IXp;
              numArray27[index37] = num19;
              int[] startMor = this.StartMor;
              int[] numArray28 = startMor;
              let mut index38: i32 =  iattacker;
              let mut index39: i32 =  index38;
              let mut num20: i32 =  startMor[index38] + this.game.TempCombat.IList[index13].IMor;
              numArray28[index39] = num20;
              int[] startEntr = this.StartEntr;
              int[] numArray29 = startEntr;
              let mut index40: i32 =  iattacker;
              let mut index41: i32 =  index40;
              let mut num21: i32 =  startEntr[index40] + this.game.TempCombat.IList[index13].IEntrench;
              numArray29[index41] = num21;
            }
            else
            {
              int[] numArray30 = numArray4;
              int[] numArray31 = numArray30;
              let mut index42: i32 =  iattacker;
              let mut index43: i32 =  index42;
              let mut num22: i32 =  numArray30[index42] + this.game.TempCombat.IList[index13].IRdn;
              numArray31[index43] = num22;
              int[] numArray32 = numArray5;
              int[] numArray33 = numArray32;
              let mut index44: i32 =  iattacker;
              let mut index45: i32 =  index44;
              let mut num23: i32 =  numArray32[index44] + this.game.TempCombat.IList[index13].IXp;
              numArray33[index45] = num23;
              int[] numArray34 = numArray6;
              int[] numArray35 = numArray34;
              let mut index46: i32 =  iattacker;
              let mut index47: i32 =  index46;
              let mut num24: i32 =  numArray34[index46] + this.game.TempCombat.IList[index13].IMor;
              numArray35[index47] = num24;
              int[] numArray36 = numArray7;
              int[] numArray37 = numArray36;
              let mut index48: i32 =  iattacker;
              let mut index49: i32 =  index48;
              let mut num25: i32 =  numArray36[index48] + this.game.TempCombat.IList[index13].IEntrench;
              numArray37[index49] = num25;
            }
            if (iattacker == 1)
              num9 += 1;
            else
              num10 += 1;
          }
        }
        if (num9 < 1)
          num9 = 1;
        if (num10 < 1)
          num10 = 1;
        if (this.game.TempCombat.CombatRound == 0)
        {
          this.StartRdn[0] =  Math.Round(Conversion.Int( this.StartRdn[0] /  num10));
          this.StartXp[0] =  Math.Round(Conversion.Int( this.StartXp[0] /  num10));
          this.StartMor[0] =  Math.Round(Conversion.Int( this.StartMor[0] /  num10));
          this.StartEntr[0] =  Math.Round(Conversion.Int( this.StartEntr[0] /  num10));
          this.StartRdn[1] =  Math.Round(Conversion.Int( this.StartRdn[1] /  num9));
          this.StartXp[1] =  Math.Round(Conversion.Int( this.StartXp[1] /  num9));
          this.StartMor[1] =  Math.Round(Conversion.Int( this.StartMor[1] /  num9));
          this.StartEntr[1] =  Math.Round(Conversion.Int( this.StartEntr[1] /  num9));
          let mut index50: i32 =  0;
          do
          {
            this.game.EditObj.StartRdn[index50] = this.StartRdn[index50];
            this.game.EditObj.StartXp[index50] = this.StartXp[index50];
            this.game.EditObj.StartMor[index50] = this.StartMor[index50];
            this.game.EditObj.StartEntr[index50] = this.StartEntr[index50];
            index50 += 1;
          }
          while (index50 <= 1);
        }
        else
        {
          numArray4[0] =  Math.Round(Conversion.Int( numArray4[0] /  num10));
          numArray5[0] =  Math.Round(Conversion.Int( numArray5[0] /  num10));
          numArray6[0] =  Math.Round(Conversion.Int( numArray6[0] /  num10));
          numArray7[0] =  Math.Round(Conversion.Int( numArray7[0] /  num10));
          numArray4[1] =  Math.Round(Conversion.Int( numArray4[1] /  num9));
          numArray5[1] =  Math.Round(Conversion.Int( numArray5[1] /  num9));
          numArray6[1] =  Math.Round(Conversion.Int( numArray6[1] /  num9));
          numArray7[1] =  Math.Round(Conversion.Int( numArray7[1] /  num9));
          let mut index51: i32 =  0;
          do
          {
            this.StartRdn[index51] = this.game.EditObj.StartRdn[index51];
            this.StartXp[index51] = this.game.EditObj.StartXp[index51];
            this.StartMor[index51] = this.game.EditObj.StartMor[index51];
            this.StartEntr[index51] = this.game.EditObj.StartEntr[index51];
            index51 += 1;
          }
          while (index51 <= 1);
        }
        if (this.showdetail == 0)
        {
          let mut num26: i32 =  185;
          bitmap: Bitmap;
          if (this.game.TempCombat.DefenderRegime > -1)
          {
            ref Graphics local1 = ref Expression1;
            bitmap = BitmapStore.GetBitmap(this.game.COMBATGRADIENT);
            ref local2: Bitmap = ref bitmap;
            let mut y: i32 =  num26;
            double r =  this.game.Data.RegimeObj[this.game.TempCombat.DefenderRegime].Red /  byte.MaxValue - 1.0;
            double g =  this.game.Data.RegimeObj[this.game.TempCombat.DefenderRegime].Green /  byte.MaxValue - 1.0;
            double b =  this.game.Data.RegimeObj[this.game.TempCombat.DefenderRegime].Blue /  byte.MaxValue - 1.0;
            DrawMod.Draw(ref local1, ref local2, 150, y,  r,  g,  b, 1f);
          }
          else
          {
            ref Graphics local3 = ref Expression1;
            bitmap = BitmapStore.GetBitmap(this.game.COMBATGRADIENT);
            ref local4: Bitmap = ref bitmap;
            let mut y: i32 =  num26;
            DrawMod.Draw(ref local3, ref local4, 150, y, -0.4980392f, -0.4980392f, -0.4980392f, 1f);
          }
          let mut num27: i32 =  495;
          ref Graphics local5 = ref Expression1;
          bitmap = BitmapStore.GetBitmap(this.game.COMBATGRADIENT);
          ref local6: Bitmap = ref bitmap;
          let mut y1: i32 =  num27;
          double r1 =  ( this.game.Data.RegimeObj[this.game.TempCombat.AttackerRegime].Red /  byte.MaxValue);
          double g1 =  ( this.game.Data.RegimeObj[this.game.TempCombat.AttackerRegime].Green /  byte.MaxValue);
          double b1 =  ( this.game.Data.RegimeObj[this.game.TempCombat.AttackerRegime].Blue /  byte.MaxValue);
          DrawMod.Draw(ref local5, ref local6, 150, y1,  r1,  g1,  b1, 1f);
          DrawMod.DrawBlock(ref Expression1, 150, 110, 840, 310,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
          DrawMod.DrawBlock(ref Expression1, 150, 420, 840, 310,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
          let mut unitC1: i32 =  this.UnitC;
          for (let mut index52: i32 =  0; index52 <= unitC1; index52 += 1)
          {
            let mut num28: i32 =   Math.Round(640.0 /  (this.UnitC + 1));
            let mut num29: i32 =  num28 * (index52 + 1);
            let mut num30: i32 =  num28 * index52;
            let mut Length: i32 =   Math.Round(100.0 /  (this.UnitC + 1));
            if (index52 > -1)
            {
              num29 =  Math.Round( num29 + 100.0 /  (this.UnitC + 1) *  (index52 + 1));
              num30 =  Math.Round( num30 + 100.0 /  (this.UnitC + 1) *  index52);
            }
            if (index52 < this.UnitC)
            {
              DrawMod.drawLine(ref Expression1, num29 + 150, 110, num29 + 150, 420,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
              DrawMod.drawLine(ref Expression1, num29 + 150, 420, num29 + 150, 730,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
            }
            if (this.UnitA[0, index52] > -1)
            {
              DrawMod.DrawSimple(ref Expression1, ref this.UnitBitMap.Address(0, index52), 150 + num30 + 5, 120);
              DrawMod.DrawText(ref Expression1, Strings.Left(this.game.Data.UnitObj[this.UnitA[0, index52]].Name, Length), this.game.VicFont4, 190 + num30 + 5, 120);
              if (this.UnitL[0, index52] != -1)
              {
                if (this.game.TempCombat.UList[this.UnitL[0, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[0, index52]].URetreatMode == 1)
                  DrawMod.DrawText(ref Expression1, "RETREAT", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[0, index52]].URetreatMode == 2)
                  DrawMod.DrawText(ref Expression1, "END AIR", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[0, index52]].URetreatMode == 3)
                  DrawMod.DrawText(ref Expression1, "OUT OF AP", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[0, index52]].URetreatMode == 4)
                  DrawMod.DrawText(ref Expression1, "END SURPRISE", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].UShatter)
                  DrawMod.DrawText(ref Expression1, "xxxxSHATTERED", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].UBreaks & this.game.TempCombat.UList[this.UnitL[0, index52]].URetreatMode == 5)
                {
                  DrawMod.DrawText(ref Expression1, "PANICKED", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                  DrawMod.DrawText(ref Expression1, "+BROKEN", this.game.VicFont3, 150 + num30 + 5 + 40, 155);
                }
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].UBreaks)
                  DrawMod.DrawText(ref Expression1, "BROKEN", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
                else if (this.game.TempCombat.UList[this.UnitL[0, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[0, index52]].URetreatMode == 5)
                  DrawMod.DrawText(ref Expression1, "PANICKED", this.game.VicFont3, 150 + num30 + 5 + 40, 135);
              }
            }
            if (this.UnitA[1, index52] > -1)
            {
              DrawMod.DrawSimple(ref Expression1, ref this.UnitBitMap.Address(1, index52), 150 + num30 + 5, 430);
              DrawMod.DrawText(ref Expression1, Strings.Left(this.game.Data.UnitObj[this.UnitA[1, index52]].Name, Length), this.game.VicFont4, 190 + num30 + 5, 430);
              if (this.UnitL[1, index52] != -1)
              {
                if (this.game.TempCombat.UList[this.UnitL[1, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[1, index52]].URetreatMode == 1)
                  DrawMod.DrawText(ref Expression1, "RETREAT", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[1, index52]].URetreatMode == 2)
                  DrawMod.DrawText(ref Expression1, "END AIR", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[1, index52]].URetreatMode == 3)
                  DrawMod.DrawText(ref Expression1, "OUT OF AP", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[1, index52]].URetreatMode == 4)
                  DrawMod.DrawText(ref Expression1, "END SURPRISE", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].UShatter)
                  DrawMod.DrawText(ref Expression1, "xxxxxSHATTERED", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].UBreaks & this.game.TempCombat.UList[this.UnitL[1, index52]].URetreatMode == 5)
                {
                  DrawMod.DrawText(ref Expression1, "PANICKED", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                  DrawMod.DrawText(ref Expression1, "+BROKEN", this.game.VicFont3, 150 + num30 + 5 + 40, 475);
                }
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].UBreaks)
                  DrawMod.DrawText(ref Expression1, "BROKEN", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
                else if (this.game.TempCombat.UList[this.UnitL[1, index52]].URetreat > 0 & this.game.TempCombat.UList[this.UnitL[1, index52]].URetreatMode == 5)
                  DrawMod.DrawText(ref Expression1, "PANICKED", this.game.VicFont3, 150 + num30 + 5 + 40, 455);
              }
            }
          }
          DrawMod.drawLine(ref Expression1, 150, 110, 990, 110,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          DrawMod.drawLine(ref Expression1, 150, 180, 990, 180,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  Math.Round( this.game.VicColor3.A / 2.0));
          DrawMod.drawLine(ref Expression1, 150, 260, 990, 260,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  Math.Round( this.game.VicColor3.A / 2.0));
          DrawMod.drawLine(ref Expression1, 150, 340, 990, 340,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  Math.Round( this.game.VicColor3.A / 2.0));
          DrawMod.drawLine(ref Expression1, 150, 420, 990, 420,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          DrawMod.drawLine(ref Expression1, 150, 490, 990, 490,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  Math.Round( this.game.VicColor3.A / 2.0));
          DrawMod.drawLine(ref Expression1, 150, 570, 990, 570,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  Math.Round( this.game.VicColor3.A / 2.0));
          DrawMod.drawLine(ref Expression1, 150, 650, 990, 650,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  Math.Round( this.game.VicColor3.A / 2.0));
          DrawMod.drawLine(ref Expression1, 150, 730, 990, 730,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          DrawMod.drawLine(ref Expression1, 990, 110, 990, 350,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          DrawMod.drawLine(ref Expression1, 990, 350, 990, 730,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          DrawMod.drawLine(ref Expression1, 150, 110, 150, 350,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          DrawMod.drawLine(ref Expression1, 150, 350, 150, 730,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
          let mut index53: i32 =  0;
          do
          {
            let mut unitC2: i32 =  this.UnitC;
            for (let mut index54: i32 =  0; index54 <= unitC2; index54 += 1)
            {
              let mut num31: i32 =  0;
              do
              {
                let mut num32: i32 =  70 + index53 * 240 + 50 + num31 * 80;
                if (index53 == 1)
                  num32 += 70;
                let mut num33: i32 =  -1;
                let mut num34: i32 =  0;
                let mut sfTypeCounter1: i32 =  this.game.Data.SFTypeCounter;
                for (let mut index55: i32 =  0; index55 <= sfTypeCounter1; index55 += 1)
                {
                  let mut icounter2: i32 =  this.game.TempCombat.ICounter;
                  for (let mut index56: i32 =  0; index56 <= icounter2; index56 += 1)
                  {
                    if (this.game.TempCombat.IList[index56].ISFType == index55 & this.game.TempCombat.IList[index56].IUnr == this.UnitA[index53, index54] && this.game.TempCombat.IList[index56].IAttacker == index53)
                    {
                      switch (num31)
                      {
                        case 0:
                          if (this.game.TempCombat.IList[index56].IRetreated == 0 & this.game.TempCombat.IList[index56].IKilled == 0)
                          {
                            num34 += 1;
                            continue;
                          }
                          continue;
                        case 1:
                          if (this.game.TempCombat.IList[index56].IKilled > 0)
                          {
                            num34 += 1;
                            continue;
                          }
                          continue;
                        case 2:
                          if (this.game.TempCombat.IList[index56].IRetreated > 0 & this.game.TempCombat.IList[index56].IKilled == 0)
                          {
                            num34 += 1;
                            continue;
                          }
                          continue;
                        default:
                          continue;
                      }
                    }
                  }
                }
                let mut sfTypeCounter2: i32 =  this.game.Data.SFTypeCounter;
                for (let mut index57: i32 =  0; index57 <= sfTypeCounter2; index57 += 1)
                {
                  let mut icounter3: i32 =  this.game.TempCombat.ICounter;
                  for (let mut index58: i32 =  0; index58 <= icounter3; index58 += 1)
                  {
                    let mut num35: i32 =  0;
                    if (this.game.TempCombat.IList[index58].ISFType == index57 && this.game.TempCombat.IList[index58].IAttacker == index53 & this.game.TempCombat.IList[index58].IUnr == this.UnitA[index53, index54])
                    {
                      switch (num31)
                      {
                        case 0:
                          if (this.game.TempCombat.IList[index58].IRetreated == 0 & this.game.TempCombat.IList[index58].IKilled == 0)
                          {
                            num35 = 1;
                            break;
                          }
                          break;
                        case 1:
                          if (this.game.TempCombat.IList[index58].IKilled > 0)
                          {
                            num35 = 1;
                            break;
                          }
                          break;
                        case 2:
                          if (this.game.TempCombat.IList[index58].IRetreated > 0 & this.game.TempCombat.IList[index58].IKilled == 0)
                          {
                            num35 = 1;
                            break;
                          }
                          break;
                      }
                      if (num35 > 0)
                      {
                        let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[index57].SymbolSpriteID;
                        let mut index59: i32 =  -1;
                        if (index53 == 0)
                          index59 = this.game.TempCombat.DefenderRegime;
                        if (index53 == 1)
                          index59 = this.game.TempCombat.AttackerRegime;
                        if (index59 > -1)
                        {
                          if (this.game.Data.RegimeObj[index59].ExtraGraphicUse > -1)
                          {
                            let mut extraCounter: i32 =  this.game.Data.SFTypeObj[index57].ExtraCounter;
                            for (let mut index60: i32 =  0; index60 <= extraCounter; index60 += 1)
                            {
                              if (this.game.Data.SFTypeObj[index57].ExtraCode[index60] == this.game.Data.RegimeObj[index59].ExtraGraphicUse)
                                symbolSpriteId = this.game.Data.SFTypeObj[index57].ExtraSymbolSpriteID[index60];
                            }
                          }
                          else if (this.game.Data.PeopleObj[this.game.Data.SFObj[this.game.TempCombat.IList[index58].ISFNr].People].ExtraGraphicUse > -1)
                          {
                            let mut extraCounter: i32 =  this.game.Data.SFTypeObj[index57].ExtraCounter;
                            for (let mut index61: i32 =  0; index61 <= extraCounter; index61 += 1)
                            {
                              if (this.game.Data.SFTypeObj[index57].ExtraCode[index61] == this.game.Data.PeopleObj[this.game.Data.SFObj[this.game.TempCombat.IList[index58].ISFNr].People].ExtraGraphicUse)
                                symbolSpriteId = this.game.Data.SFTypeObj[index57].ExtraSymbolSpriteID[index61];
                            }
                          }
                        }
                        num33 += 1;
                        let mut num36: i32 =   Math.Round(640.0 /  (this.UnitC + 1));
                        let mut num37: i32 =   Math.Round(Conversion.Int( (num36 * 4) /  num34));
                        if (num37 > 25)
                          num37 = 25;
                        if (num37 < 1)
                          num37 = 1;
                        let mut num38: i32 =  num37 * num33;
                        let mut num39: i32 =  num36 % num37;
                        let mut num40: i32 =  num36 - num39;
                        let mut num41: i32 =  (num40 + num39) * index54;
                        if (index54 > 0)
                          num41 =  Math.Round( num41 + 100.0 /  (this.UnitC + 1) *  index54);
                        num42: i32;
                        num43: i32;
                        if (num38 <= num40)
                        {
                          num42 = 0;
                          num43 = 64;
                        }
                        else if (num38 <= 2 * num40 + num37)
                        {
                          num42 = 1 * num40 + num37;
                          num43 = 82;
                        }
                        else if (num38 <= 3 * num40 + num37)
                        {
                          num42 = 2 * num40 + 2 * num37;
                          num43 = 100;
                        }
                        else if (num38 <= 4 * num40 + num37)
                        {
                          num42 = 3 * num40 + 2 * num37;
                          num43 = 118;
                        }
                        let mut num44: i32 =  0;
                        if (this.game.TempCombat.IList[index58].ICapitulate)
                          num44 = 1;
                        if (this.game.TempCombat.IList[index58].IBreakTrough > 0 & (num31 == 0 & index53 == 0 | num31 == 0 & index53 == 1))
                          num44 = 2;
                        switch (num44)
                        {
                          case 0:
                            ref Graphics local7 = ref Expression1;
                            bitmap = BitmapStore.GetBitmap(symbolSpriteId);
                            ref local8: Bitmap = ref bitmap;
                            let mut x1: i32 =  num41 + 150 + num37 * num33 - num42;
                            let mut y2: i32 =  num32 + num43;
                            DrawMod.DrawSimple(ref local7, ref local8, x1, y2);
                            break;
                          case 1:
                            ref Graphics local9 = ref Expression1;
                            bitmap = BitmapStore.GetBitmap(symbolSpriteId);
                            ref local10: Bitmap = ref bitmap;
                            let mut x2: i32 =  num41 + 150 + num37 * num33 - num42;
                            let mut y3: i32 =  num32 + num43;
                            DrawMod.DrawSimple(ref local9, ref local10, x2, y3);
                            ref Graphics local11 = ref Expression1;
                            bitmap = BitmapStore.GetBitmap(this.game.WHITEFLAG);
                            ref local12: Bitmap = ref bitmap;
                            let mut x3: i32 =  num41 + 157 + num37 * num33 - num42;
                            let mut y4: i32 =  num32 + num43;
                            DrawMod.DrawSimple(ref local11, ref local12, x3, y4);
                            break;
                          case 2:
                            ref Graphics local13 = ref Expression1;
                            bitmap = BitmapStore.GetBitmap(symbolSpriteId);
                            ref local14: Bitmap = ref bitmap;
                            let mut x4: i32 =  num41 + 150 + num37 * num33 - num42;
                            let mut y5: i32 =  num32 + num43;
                            DrawMod.Draw(ref local13, ref local14, x4, y5, 0.0f, 0.0f, -1f, 1f);
                            break;
                        }
                        let mut num45: i32 =   Math.Round(Conversion.Int( this.game.TempCombat.IList[index58].IRdn / 10.0));
                        DrawMod.drawLine(ref Expression1, num41 + 150 + 8 + num37 * num33 - num42, num32 + num43, num41 + 150 + 8 + num37 * num33 - num42, num32 + num43 + 10, 0, 0, 0,  byte.MaxValue);
                        DrawMod.drawLine(ref Expression1, num41 + 150 + 8 + num37 * num33 - num42, num32 + num43 + 10 - num45, num41 + 150 + 8 + num37 * num33 - num42, num32 + num43 + 10, 0,  byte.MaxValue, 0,  byte.MaxValue);
                      }
                    }
                  }
                }
                num31 += 1;
              }
              while (num31 <= 2);
            }
            index53 += 1;
          }
          while (index53 <= 1);
        }
        else
        {
          str1: String = "ATTACKER TOTALS\r\n";
          Expression2: String = "SUBFORMATIONTYPE".to_owned();
          let mut Number1: i32 =  35 - Strings.Len(Expression2);
          if (0 > Number1)
            Number1 = 0;
          str2: String = str1 + Expression2 + Strings.Space(Number1);
          Expression3: String = "INITIAL".to_owned();
          let mut Number2: i32 =  10 - Strings.Len(Expression3);
          if (0 > Number2)
            Number2 = 0;
          str3: String = str2 + Expression3 + Strings.Space(Number2);
          Expression4: String = "ATTACK".to_owned();
          let mut Number3: i32 =  9 - Strings.Len(Expression4);
          if (0 > Number3)
            Number3 = 0;
          str4: String = str3 + Expression4 + Strings.Space(Number3);
          Expression5: String = "DEAD".to_owned();
          let mut Number4: i32 =  7 - Strings.Len(Expression5);
          if (0 > Number4)
            Number4 = 0;
          str5: String = str4 + Expression5 + Strings.Space(Number4);
          Expression6: String = "RETREAT".to_owned();
          let mut Number5: i32 =  9 - Strings.Len(Expression6);
          if (0 > Number5)
            Number5 = 0;
          str6: String = str5 + Expression6 + Strings.Space(Number5);
          Expression7: String = "ALIVE".to_owned();
          let mut Number6: i32 =  6 - Strings.Len(Expression7);
          if (0 > Number6)
            Number6 = 0;
          str7: String = str6 + Expression7 + Strings.Space(Number6) + "\r\n";
          let mut sfTypeCounter3: i32 =  this.game.Data.SFTypeCounter;
          for (let mut index62: i32 =  0; index62 <= sfTypeCounter3; index62 += 1)
          {
            index12 = 1;
            if (this.game.Data.SFTypeObj[index62].Ratio > 0)
              index12 = this.game.Data.SFTypeObj[index62].Ratio;
            if (numArray3[index62, 1] > 0 | numArray1[index62, 1] > 0 | numArray2[index62, 1] > 0)
            {
              Expression8: String = this.game.Data.SFTypeObj[index62].Name;
              if (Strings.Len(Expression8) > 29)
                Expression8 = Strings.Left(str7, 29);
              let mut Number7: i32 =  35 - Strings.Len(Expression8);
              if (0 > Number7)
                Number7 = 0;
              str8: String = str7 + Expression8 + Strings.Space(Number7);
              Expression9: String = Strings.Trim(Conversion.Str( (index12 * (numArray3[index62, 1] + numArray1[index62, 1] + numArray2[index62, 1]))));
              let mut Number8: i32 =  10 - Strings.Len(Expression9);
              if (0 > Number8)
                Number8 = 0;
              str9: String = str8 + Expression9 + Strings.Space(Number8);
              Expression10: String = Strings.Trim(Conversion.Str( (index12 * numArray3[index62, 1])));
              let mut Number9: i32 =  9 - Strings.Len(Expression10);
              if (0 > Number9)
                Number9 = 0;
              str10: String = str9 + Expression10 + Strings.Space(Number9);
              Expression11: String = Strings.Trim(Conversion.Str( (index12 * numArray1[index62, 1])));
              let mut Number10: i32 =  7 - Strings.Len(Expression11);
              if (0 > Number10)
                Number10 = 0;
              str11: String = str10 + Expression11 + Strings.Space(Number10);
              Expression12: String = Strings.Trim(Conversion.Str( (index12 * numArray2[index62, 1])));
              let mut Number11: i32 =  9 - Strings.Len(Expression12);
              if (0 > Number11)
                Number11 = 0;
              str12: String = str11 + Expression12 + Strings.Space(Number11);
              Expression13: String = Strings.Trim(Conversion.Str( (index12 * (numArray2[index62, 1] + numArray3[index62, 1]))));
              let mut Number12: i32 =  6 - Strings.Len(Expression13);
              if (0 > Number12)
                Number12 = 0;
              str7 = str12 + Expression13 + Strings.Space(Number12) + "\r\n";
            }
          }
          Expression14: String = "TOTAL POWERPOINTS";
          if (Strings.Len(Expression14) > 29)
            Expression14 = Strings.Left(str7, 29);
          let mut Number13: i32 =  35 - Strings.Len(Expression14);
          if (0 > Number13)
            Number13 = 0;
          str13: String = str7 + Expression14 + Strings.Space(Number13);
          Expression15: String = Strings.Trim(Conversion.Str( (numArray10[1] + numArray8[1] + numArray9[1])));
          let mut Number14: i32 =  10 - Strings.Len(Expression15);
          if (0 > Number14)
            Number14 = 0;
          str14: String = str13 + Expression15 + Strings.Space(Number14);
          Expression16: String = Strings.Trim(Conversion.Str( numArray10[1]));
          let mut Number15: i32 =  9 - Strings.Len(Expression16);
          if (0 > Number15)
            Number15 = 0;
          str15: String = str14 + Expression16 + Strings.Space(Number15);
          Expression17: String = Strings.Trim(Conversion.Str( numArray8[1]));
          let mut Number16: i32 =  7 - Strings.Len(Expression17);
          if (0 > Number16)
            Number16 = 0;
          str16: String = str15 + Expression17 + Strings.Space(Number16);
          Expression18: String = Strings.Trim(Conversion.Str( numArray9[1]));
          let mut Number17: i32 =  9 - Strings.Len(Expression18);
          if (0 > Number17)
            Number17 = 0;
          str17: String = str16 + Expression18 + Strings.Space(Number17);
          Expression19: String = Strings.Trim(Conversion.Str( (numArray9[1] + numArray10[1])));
          let mut Number18: i32 =  6 - Strings.Len(Expression19);
          if (0 > Number18)
            Number18 = 0;
          str18: String = str17 + Expression19 + Strings.Space(Number18) + "\r\n" + "\r\n" + "DEFENDERS TOTALS\r\n";
          Expression20: String = "SUBFORMATIONTYPE".to_owned();
          let mut Number19: i32 =  35 - Strings.Len(Expression20);
          if (0 > Number19)
            Number19 = 0;
          str19: String = str18 + Expression20 + Strings.Space(Number19);
          Expression21: String = "INITIAL".to_owned();
          let mut Number20: i32 =  10 - Strings.Len(Expression21);
          if (0 > Number20)
            Number20 = 0;
          str20: String = str19 + Expression21 + Strings.Space(Number20);
          Expression22: String = "ATTACK".to_owned();
          let mut Number21: i32 =  9 - Strings.Len(Expression22);
          if (0 > Number21)
            Number21 = 0;
          str21: String = str20 + Expression22 + Strings.Space(Number21);
          Expression23: String = "DEAD".to_owned();
          let mut Number22: i32 =  7 - Strings.Len(Expression23);
          if (0 > Number22)
            Number22 = 0;
          str22: String = str21 + Expression23 + Strings.Space(Number22);
          Expression24: String = "RETREAT".to_owned();
          let mut Number23: i32 =  9 - Strings.Len(Expression24);
          if (0 > Number23)
            Number23 = 0;
          str23: String = str22 + Expression24 + Strings.Space(Number23);
          Expression25: String = "ALIVE".to_owned();
          let mut Number24: i32 =  6 - Strings.Len(Expression25);
          if (0 > Number24)
            Number24 = 0;
          str24: String = str23 + Expression25 + Strings.Space(Number24) + "\r\n";
          let mut sfTypeCounter4: i32 =  this.game.Data.SFTypeCounter;
          for (let mut index63: i32 =  0; index63 <= sfTypeCounter4; index63 += 1)
          {
            index12 = 1;
            if (this.game.Data.SFTypeObj[index63].Ratio > 0)
              index12 = this.game.Data.SFTypeObj[index63].Ratio;
            if (numArray3[index63, 0] > 0 | numArray1[index63, 0] > 0 | numArray2[index63, 0] > 0)
            {
              Expression26: String = this.game.Data.SFTypeObj[index63].Name;
              if (Strings.Len(Expression26) > 29)
                Expression26 = Strings.Left(str24, 29);
              let mut Number25: i32 =  35 - Strings.Len(Expression26);
              if (0 > Number25)
                Number25 = 0;
              str25: String = str24 + Expression26 + Strings.Space(Number25);
              Expression27: String = Strings.Trim(Conversion.Str( (index12 * (numArray3[index63, 0] + numArray1[index63, 0] + numArray2[index63, 0]))));
              let mut Number26: i32 =  10 - Strings.Len(Expression27);
              if (0 > Number26)
                Number26 = 0;
              str26: String = str25 + Expression27 + Strings.Space(Number26);
              Expression28: String = Strings.Trim(Conversion.Str( (index12 * numArray3[index63, 0])));
              let mut Number27: i32 =  9 - Strings.Len(Expression28);
              if (0 > Number27)
                Number27 = 0;
              str27: String = str26 + Expression28 + Strings.Space(Number27);
              Expression29: String = Strings.Trim(Conversion.Str( (index12 * numArray1[index63, 0])));
              let mut Number28: i32 =  7 - Strings.Len(Expression29);
              if (0 > Number28)
                Number28 = 0;
              str28: String = str27 + Expression29 + Strings.Space(Number28);
              Expression30: String = Strings.Trim(Conversion.Str( (index12 * numArray2[index63, 0])));
              let mut Number29: i32 =  9 - Strings.Len(Expression30);
              if (0 > Number29)
                Number29 = 0;
              str29: String = str28 + Expression30 + Strings.Space(Number29);
              Expression31: String = Strings.Trim(Conversion.Str( (index12 * (numArray2[index63, 0] + numArray3[index63, 0]))));
              let mut Number30: i32 =  6 - Strings.Len(Expression31);
              if (0 > Number30)
                Number30 = 0;
              str24 = str29 + Expression31 + Strings.Space(Number30) + "\r\n";
            }
          }
          Expression32: String = "TOTAL POWERPOINTS";
          if (Strings.Len(Expression32) > 29)
            Expression32 = Strings.Left(str24, 29);
          let mut Number31: i32 =  35 - Strings.Len(Expression32);
          if (0 > Number31)
            Number31 = 0;
          str30: String = str24 + Expression32 + Strings.Space(Number31);
          Expression33: String = Strings.Trim(Conversion.Str( (index12 * (numArray10[0] + numArray8[0] + numArray9[0]))));
          let mut Number32: i32 =  10 - Strings.Len(Expression33);
          if (0 > Number32)
            Number32 = 0;
          str31: String = str30 + Expression33 + Strings.Space(Number32);
          Expression34: String = Strings.Trim(Conversion.Str( (index12 * numArray10[0])));
          let mut Number33: i32 =  9 - Strings.Len(Expression34);
          if (0 > Number33)
            Number33 = 0;
          str32: String = str31 + Expression34 + Strings.Space(Number33);
          Expression35: String = Strings.Trim(Conversion.Str( (index12 * numArray8[0])));
          let mut Number34: i32 =  7 - Strings.Len(Expression35);
          if (0 > Number34)
            Number34 = 0;
          str33: String = str32 + Expression35 + Strings.Space(Number34);
          Expression36: String = Strings.Trim(Conversion.Str( (index12 * numArray9[0])));
          let mut Number35: i32 =  9 - Strings.Len(Expression36);
          if (0 > Number35)
            Number35 = 0;
          str34: String = str33 + Expression36 + Strings.Space(Number35);
          Expression37: String = Strings.Trim(Conversion.Str( (index12 * (numArray9[0] + numArray10[0]))));
          let mut Number36: i32 =  6 - Strings.Len(Expression37);
          if (0 > Number36)
            Number36 = 0;
          str35: String = str34 + Expression37 + Strings.Space(Number36) + "\r\n" + "\r\n" + "\r\n";
          Expression38: String = "ATTACKER".to_owned();
          let mut Number37: i32 =  10 - Strings.Len(Expression38);
          if (0 > Number37)
            Number37 = 0;
          str36: String = str35 + Strings.Space(20) + Expression38 + Strings.Space(Number37);
          Expression39: String = "DEFENDER".to_owned();
          let mut num46: i32 =  10 - Strings.Len(Expression39);
          if (0 > num46)
            num46 = 0;
          str37: String = str36 + Strings.Space(20) + Expression39 + Strings.Space(num46 + 50) + "\r\n";
          Expression40: String = "STAT".to_owned();
          let mut Number38: i32 =  20 - Strings.Len(Expression40);
          if (0 > Number38)
            Number38 = 0;
          str38: String = str37 + Expression40 + Strings.Space(Number38);
          Expression41: String = "INITIAL".to_owned();
          let mut Number39: i32 =  10 - Strings.Len(Expression41);
          if (0 > Number39)
            Number39 = 0;
          str39: String = str38 + Expression41 + Strings.Space(Number39);
          Expression42: String = "CURRENT".to_owned();
          let mut Number40: i32 =  20 - Strings.Len(Expression42);
          if (0 > Number40)
            Number40 = 0;
          str40: String = str39 + Expression42 + Strings.Space(Number40);
          Expression43: String = "INITIAL".to_owned();
          let mut Number41: i32 =  10 - Strings.Len(Expression43);
          if (0 > Number41)
            Number41 = 0;
          str41: String = str40 + Expression43 + Strings.Space(Number41);
          Expression44: String = "CURRENT".to_owned();
          let mut Number42: i32 =  10 - Strings.Len(Expression44);
          if (0 > Number42)
            Number42 = 0;
          str42: String = str41 + Expression44 + Strings.Space(Number42) + "\r\n";
          Expression45: String = "Readiness".to_owned();
          if (Strings.Len(Expression45) > 29)
            Expression45 = Strings.Left(str42, 29);
          let mut Number43: i32 =  20 - Strings.Len(Expression45);
          if (0 > Number43)
            Number43 = 0;
          str43: String = str42 + Expression45 + Strings.Space(Number43);
          Expression46: String = Strings.Trim(Conversion.Str( this.StartRdn[1]));
          let mut Number44: i32 =  10 - Strings.Len(Expression46);
          if (0 > Number44)
            Number44 = 0;
          str44: String = str43 + Expression46 + Strings.Space(Number44);
          Expression47: String = Strings.Trim(Conversion.Str( numArray4[1]));
          let mut Number45: i32 =  20 - Strings.Len(Expression47);
          if (0 > Number45)
            Number45 = 0;
          str45: String = str44 + Expression47 + Strings.Space(Number45);
          Expression48: String = Strings.Trim(Conversion.Str( this.StartRdn[0]));
          let mut Number46: i32 =  10 - Strings.Len(Expression48);
          if (0 > Number46)
            Number46 = 0;
          str46: String = str45 + Expression48 + Strings.Space(Number46);
          Expression49: String = Strings.Trim(Conversion.Str( numArray4[0]));
          let mut Number47: i32 =  10 - Strings.Len(Expression49);
          if (0 > Number47)
            Number47 = 0;
          str47: String = str46 + Expression49 + Strings.Space(Number47) + "\r\n";
          Expression50: String = "Experience".to_owned();
          if (Strings.Len(Expression50) > 29)
            Expression50 = Strings.Left(str47, 29);
          let mut Number48: i32 =  20 - Strings.Len(Expression50);
          if (0 > Number48)
            Number48 = 0;
          str48: String = str47 + Expression50 + Strings.Space(Number48);
          Expression51: String = Strings.Trim(Conversion.Str( this.StartXp[1]));
          let mut Number49: i32 =  10 - Strings.Len(Expression51);
          if (0 > Number49)
            Number49 = 0;
          str49: String = str48 + Expression51 + Strings.Space(Number49);
          Expression52: String = Strings.Trim(Conversion.Str( numArray5[1]));
          let mut Number50: i32 =  20 - Strings.Len(Expression52);
          if (0 > Number50)
            Number50 = 0;
          str50: String = str49 + Expression52 + Strings.Space(Number50);
          Expression53: String = Strings.Trim(Conversion.Str( this.StartXp[0]));
          let mut Number51: i32 =  10 - Strings.Len(Expression53);
          if (0 > Number51)
            Number51 = 0;
          str51: String = str50 + Expression53 + Strings.Space(Number51);
          Expression54: String = Strings.Trim(Conversion.Str( numArray5[0]));
          let mut Number52: i32 =  10 - Strings.Len(Expression54);
          if (0 > Number52)
            Number52 = 0;
          str52: String = str51 + Expression54 + Strings.Space(Number52) + "\r\n";
          Expression55: String = "Morale".to_owned();
          if (Strings.Len(Expression55) > 29)
            Expression55 = Strings.Left(str52, 29);
          let mut Number53: i32 =  20 - Strings.Len(Expression55);
          if (0 > Number53)
            Number53 = 0;
          str53: String = str52 + Expression55 + Strings.Space(Number53);
          Expression56: String = Strings.Trim(Conversion.Str( this.StartMor[1]));
          let mut Number54: i32 =  10 - Strings.Len(Expression56);
          if (0 > Number54)
            Number54 = 0;
          str54: String = str53 + Expression56 + Strings.Space(Number54);
          Expression57: String = Strings.Trim(Conversion.Str( numArray6[1]));
          let mut Number55: i32 =  20 - Strings.Len(Expression57);
          if (0 > Number55)
            Number55 = 0;
          str55: String = str54 + Expression57 + Strings.Space(Number55);
          Expression58: String = Strings.Trim(Conversion.Str( this.StartMor[0]));
          let mut Number56: i32 =  10 - Strings.Len(Expression58);
          if (0 > Number56)
            Number56 = 0;
          str56: String = str55 + Expression58 + Strings.Space(Number56);
          Expression59: String = Strings.Trim(Conversion.Str( numArray6[0]));
          let mut Number57: i32 =  10 - Strings.Len(Expression59);
          if (0 > Number57)
            Number57 = 0;
          str57: String = str56 + Expression59 + Strings.Space(Number57) + "\r\n";
          Expression60: String = "Entrenchment".to_owned();
          if (Strings.Len(Expression60) > 29)
            Expression60 = Strings.Left(str57, 29);
          let mut Number58: i32 =  20 - Strings.Len(Expression60);
          if (0 > Number58)
            Number58 = 0;
          str58: String = str57 + Expression60 + Strings.Space(Number58);
          Expression61: String = Strings.Trim(Conversion.Str( this.StartEntr[1]));
          let mut Number59: i32 =  10 - Strings.Len(Expression61);
          if (0 > Number59)
            Number59 = 0;
          str59: String = str58 + Expression61 + Strings.Space(Number59);
          Expression62: String = Strings.Trim(Conversion.Str( numArray7[1]));
          let mut Number60: i32 =  20 - Strings.Len(Expression62);
          if (0 > Number60)
            Number60 = 0;
          str60: String = str59 + Expression62 + Strings.Space(Number60);
          Expression63: String = Strings.Trim(Conversion.Str( this.StartEntr[0]));
          let mut Number61: i32 =  10 - Strings.Len(Expression63);
          if (0 > Number61)
            Number61 = 0;
          str61: String = str60 + Expression63 + Strings.Space(Number61);
          Expression64: String = Strings.Trim(Conversion.Str( numArray7[0]));
          let mut Number62: i32 =  10 - Strings.Len(Expression64);
          if (0 > Number62)
            Number62 = 0;
          tText = str61 + Expression64 + Strings.Space(Number62) + "\r\n" + "\r\n";
          DrawMod.DrawPaperSheet(ref Expression1, 65, 130, 890, 592);
          if (this.ResolveId == 0)
          {
            tsubpart =  new PaperAreaClass(this.game, 850, 26, Font::new("Courier New", 15f, FontStyle.Regular, GraphicsUnit.Pixel), "Description", false, tText, this.game.VicColor8, tItemSize: 20, tbackbitmap: (ref this.OwnBitmap), bbx: 85, bby: 150);
            this.ResolveId = this.AddSubPart(ref tsubpart, 85, 150, 850, 580, 0);
          }
        }
      }
      if (this.Info1Id > 0)
        this.RemoveSubPart(this.Info1Id);
      if (this.game.TempCombat.CombatRound > 0 | this.game.TempCombat.BattleEnded > 0)
      {
        if (this.game.TempCombat.CombatType == 11)
        {
          if (this.game.TempCombat.BattleEnded == 0)
            tText = "Surprise Combat  at round " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 1)
            tText = "VICTORY: Attacker won in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 2)
            tText = "Surprise Combat ended. " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 3)
            tText = "STANDOFF: at Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
        }
        else if (this.game.TempCombat.CombatType == 1 | this.game.TempCombat.CombatType == 2 | this.game.TempCombat.CombatType == 10 | this.game.TempCombat.CombatType == 9)
        {
          if (this.game.TempCombat.BattleEnded == 0)
            tText = "Battle at Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 1)
            tText = "VICTORY: Attacker won in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 2)
            tText = "ATTACK STALLED: in CombatRound " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 3)
            tText = "STANDOFF: in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
        }
        else if (this.game.TempCombat.CombatType == 3 | this.game.TempCombat.CombatType == 4)
        {
          if (this.game.TempCombat.BattleEnded == 0)
            tText = "Artillery Bombardment at Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 1)
            tText = "Defender has retreated in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 2)
            tText = "Artillery run out of AP in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 3)
            tText = "Defender has retreated in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
        }
        else if (this.game.TempCombat.CombatType == 5 | this.game.TempCombat.CombatType == 6 | this.game.TempCombat.CombatType == 13)
        {
          if (this.game.TempCombat.BattleEnded == 0)
            tText = "Air attack at Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 1)
            tText = "Defender has retreated in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 2)
            tText = "Air attack broken off in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 3)
            tText = "Defender has retreated in Combatround  " + Conversion.Str( this.game.TempCombat.CombatRound);
        }
        else if (this.game.TempCombat.CombatType == 14)
        {
          if (this.game.TempCombat.BattleEnded == 0)
            tText = "Air Supply at Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 1)
            tText = "Air Supply retreated in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 2)
            tText = "Air Supply broken off in Combatround " + Conversion.Str( this.game.TempCombat.CombatRound);
          else if (this.game.TempCombat.BattleEnded == 3)
            tText = "Defender has retreated in Combatround  " + Conversion.Str( this.game.TempCombat.CombatRound);
        }
      }
      else
        tText = "Battle is about to commence!";
      if (this.game.EditObj.CombatSim)
        tText = "Combat sim. Check log file for result";
      ref Graphics local15 = ref Expression1;
      Rectangle rectangle1 = Rectangle::new(150, 18, 500, 14);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(150, 32, 500, 23);
      let mut rect2_1: &Rectangle = &rectangle2
      txt2_1: String = tText;
      DrawMod.MakeFullBoxVic2(ref local15, rect1_1, "COMBAT STATUS", rect2_1, txt2_1);
      ref Graphics local16 = ref Expression1;
      rectangle2 = Rectangle::new(150, 58, 700, 14);
      let mut rect1_2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(150, 72, 700, 23);
      let mut rect2_2: &Rectangle = &rectangle1
      DrawMod.MakeFullBoxVic2(ref local16, rect1_2, "COMBAT EFFECTS", rect2_2, "");
      if (this.showdetail == 0 & !this.game.EditObj.CombatSim)
      {
        SizeF sizeF1 = SizeF::new();
        vicFont1: Font = this.game.VicFont1;
        str62: String = "DEFENDERS".to_owned();
        SizeF sizeF2 = Expression1.MeasureString(str62, vicFont1);
        DrawMod.DrawTextVic2(ref Expression1, str62, vicFont1,  Math.Round( (140f - sizeF2.Width)), 180, this.game.VicColor2, this.game.VicColor1Shade);
        str63: String = "CASUALTIES".to_owned();
        SizeF sizeF3 = Expression1.MeasureString(str63, vicFont1);
        DrawMod.DrawTextVic2(ref Expression1, str63, vicFont1,  Math.Round( (140f - sizeF3.Width)), 260, this.game.VicColor2, this.game.VicColor1Shade);
        str64: String = "RETREATED".to_owned();
        SizeF sizeF4 = Expression1.MeasureString(str64, vicFont1);
        DrawMod.DrawTextVic2(ref Expression1, str64, vicFont1,  Math.Round( (140f - sizeF4.Width)), 340, this.game.VicColor2, this.game.VicColor1Shade);
        str65: String = "ATTACKERS".to_owned();
        SizeF sizeF5 = Expression1.MeasureString(str65, vicFont1);
        DrawMod.DrawTextVic2(ref Expression1, str65, vicFont1,  Math.Round( (140f - sizeF5.Width)), 490, this.game.VicColor2, this.game.VicColor1Shade);
        str66: String = "CASUALTIES".to_owned();
        SizeF sizeF6 = Expression1.MeasureString(str66, vicFont1);
        DrawMod.DrawTextVic2(ref Expression1, str66, vicFont1,  Math.Round( (140f - sizeF6.Width)), 570, this.game.VicColor2, this.game.VicColor1Shade);
        str67: String = "RETREATED".to_owned();
        SizeF sizeF7 = Expression1.MeasureString(str67, vicFont1);
        DrawMod.DrawTextVic2(ref Expression1, str67, vicFont1,  Math.Round( (140f - sizeF7.Width)), 650, this.game.VicColor2, this.game.VicColor1Shade);
      }
      if (this.game.TempCombat.BattleEnded == 0 && !this.game.EditObj.AutoCombat)
      {
        tsubpart =  new SteveButtonPartClass(this.game.BUTTONNEXT, tBackbitmap: (ref this.OwnBitmap), bbx: 965, bby: 50);
        this.B3Id = this.AddSubPart(ref tsubpart, 965, 50, 32, 32, 1);
      }
      if (this.game.TempCombat.BattleEnded > 0)
      {
        if (!this.game.EditObj.CombatSim)
        {
          if (this.showdetail == 0)
          {
            tsubpart =  new TextButtonPartClass("Switch View", 120, tBackbitmap: (ref this.OwnBitmap), bbx: 20, bby: 23);
            this.B5Id = this.AddSubPart(ref tsubpart, 20, 23, 120, 35, 1);
          }
          if (this.showdetail == 1)
          {
            tsubpart =  new TextButtonPartClass("Switch View", 120, tBackbitmap: (ref this.OwnBitmap), bbx: 20, bby: 23);
            this.B5Id = this.AddSubPart(ref tsubpart, 20, 23, 120, 35, 1);
          }
          tsubpart =  new TextButtonPartClass("Details", 120, tBackbitmap: (ref this.OwnBitmap), bbx: 20, bby: 65);
          this.DetailId = this.AddSubPart(ref tsubpart, 20, 65, 120, 35, 1);
        }
        Left: String = this.game.EditObj.CombatOneSentence;
        if (Operators.CompareString(Left, "", false) == 0)
          Left = "no effects";
        ref Graphics local17 = ref Expression1;
        rectangle2 = Rectangle::new(150, 58, 700, 14);
        let mut rect1_3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(150, 72, 700, 23);
        let mut rect2_3: &Rectangle = &rectangle1
        txt2_2: String = Left;
        DrawMod.MakeFullBoxVic2(ref local17, rect1_3, "COMBAT EFFECTS", rect2_3, txt2_2);
        tsubpart =  new SteveButtonPartClass(this.game.BUTTONQUIT, tBackbitmap: (ref this.OwnBitmap), bbx: 965, bby: 15);
        this.B6Id = this.AddSubPart(ref tsubpart, 965, 15, 32, 32, 1);
      }
      if (this.game.EditObj.CombatSim)
      {
        this.CombatListObj = ListClass::new();
        if (this.LogCounter > -1)
        {
          let mut logCounter: i32 =  this.LogCounter;
          for (let mut index64: i32 =  0; index64 <= logCounter; index64 += 1)
            this.CombatListObj.add(this.LogTxt[index64], 0);
          ListClass combatListObj = this.CombatListObj;
          let mut game: GameClass = this.game;
          ref local18: Bitmap = ref this.OwnBitmap;
          font: Font =  null;
          ref local19: Font = ref font;
          tsubpart =  new ListSubPartClass(combatListObj, 30, 800, -1, game, true, tbackbitmap: (ref local18), bbx: 10, bby: 50, overruleFont: (ref local19));
          this.CombatListId = this.AddSubPart(ref tsubpart, 10, 50, 800, 528, 0);
        }
      }
      if (Information.IsNothing( Expression1))
        return;
      Expression1.Dispose();
      Expression1 = (Graphics) null;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!this.game.EditObj.CombatSim && this.game.TempCombat.BattleEnded == 0 & this.game.EditObj.AutoCombat)
      {
        TimeSpan timeSpan = DateAndTime.Now.Subtract(this.lasttime);
        if (timeSpan.Milliseconds + timeSpan.Seconds * 1000 > 500)
        {
          this.game.TempCombat.DoRound();
          this.dostuff();
          this.lasttime = DateAndTime.Now;
          windowReturnClass.SetFlag(true);
          if (this.game.TempCombat.BattleEnded > 0 && this.game.EditObj.DoCardSlot > -1)
          {
            if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot > -1)
            {
              this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
              this.game.EditObj.AreaSlot = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot;
              this.game.EditObj.AreaValue = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaValue;
              this.game.EditObj.PopupValue = 1;
              windowReturnClass.AddCommand(5, 10);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].UnitSelect)
            {
              this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
              this.game.EditObj.PopupValue = 3;
              windowReturnClass.AddCommand(5, 10);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
            this.game.ProcessingObj.PlayCard(this.game.EditObj.DoCardSlot);
            if (Strings.Len(this.game.Data.LoadGame) > 0)
            {
              this.game.FormRef.Cursor = Cursors.WaitCursor;
              Form formRef =  this.game.FormRef;
              this.game.HandyFunctionsObj.LoadGameNow();
              this.game.FormRef = (Form1) formRef;
              this.game.FormRef.Cursor = Cursors.Default;
              windowReturnClass.AddCommand(3, 4);
              return windowReturnClass;
            }
            let mut Number: i32 =  0;
            let mut locCounter: i32 =  this.game.Data.LocCounter;
            for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
              {
                let mut index: i32 =  0;
                do
                {
                  if (this.game.Data.LocObj[locnr].Production[index] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index]).result)
                  {
                    Number += 1;
                    this.game.Data.LocObj[locnr].Production[index] = -1;
                    this.game.Data.LocObj[locnr].ProdPointRemainder[index] = 0;
                    this.game.Data.LocObj[locnr].ProdPercent[index] = 0;
                  }
                  index += 1;
                }
                while (index <= 3);
              }
            }
            if (Number > 0)
            {
              let mut num: i32 =   Interaction.MsgBox( (Conversion.Str( Number) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
            }
            if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
            {
              this.game.EditObj.PopupValue = 0;
              this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
              windowReturnClass.AddCommand(5, 10);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          return windowReturnClass;
        }
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn PopUpRefresh()
    {
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaValue = -1;
      this.game.EditObj.DoCardSlot = -1;
      this.DoRefresh();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          if (this.game.TempCombat.BattleEnded > 0)
          {
            if (!this.ForwardKey)
            {
              this.game.TempCombat.EndBattle();
              this.game.EditObj.TempCoordList.DeActivate();
              this.game.EditObj.CombatOneSentence = "";
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.TargetX = -1;
              this.game.EditObj.TargetY = -1;
              this.game.EditObj.OrderBombMode = 0;
              if ( this.game.Data.RuleVar[839] == 0.0)
                windowReturnClass.AddCommand(3, 3);
              else
                windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          else if (nr == 32 | nr == 27)
          {
            this.game.TempCombat.DoBattle();
            this.ForwardKey = true;
            this.dostuff();
            windowReturnClass.SetFlag(true);
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
          this.ForwardKey = false;
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.CombatListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (!this.game.EditObj.CombatSim & num2 > -1)
              {
                this.detailnr = num2;
                windowReturnClass.SetFlag(true);
                this.dostuff();
                return windowReturnClass;
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.ResolveId)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.CombatListDId)
            {
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (!this.game.EditObj.CombatSim & num3 > -1)
              {
                this.detailnr2 = num3;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B3Id)
            {
              this.game.TempCombat.DoRound();
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B4Id)
            {
              this.game.TempCombat.DoBattle();
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B5Id)
            {
              this.showdetail = this.showdetail != 0 ? 0 : 1;
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.DetailId)
            {
              windowReturnClass.AddCommand(1, 84);
              windowReturnClass.AddCommand(2, 82);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 != this.B6Id)
              return windowReturnClass;
            this.game.TempCombat.EndBattle();
            this.game.EditObj.TempCoordList.DeActivate();
            this.game.EditObj.CombatOneSentence = "";
            this.game.EditObj.OrderType = 0;
            this.game.EditObj.TargetX = -1;
            this.game.EditObj.TargetY = -1;
            this.game.EditObj.OrderBombMode = 0;
            if ( this.game.Data.RuleVar[839] == 0.0)
              windowReturnClass.AddCommand(3, 3);
            else
              windowReturnClass.AddCommand(3, 11);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn DoCombatSim()
    {
      let mut Number1: i32 =  200;
      numArray1: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      numArray2: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      numArray3: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      numArray4: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      numArray5: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray6: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray7: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray8: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray9: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray10: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray11: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      numArray12: Vec<f32> = new float[this.game.Data.SFTypeCounter + 1, 2];
      let mut num1: i32 =  Number1;
      Number2: i32;
      Number3: i32;
      Number4: i32;
      float num2;
      for (let mut index1: i32 =  1; index1 <= num1; index1 += 1)
      {
        Coordinate tempTarget = this.game.TempCombat.TempTarget;
        let mut tempType: i32 =  this.game.TempCombat.TempType;
        UnitList tempUnits = this.game.TempCombat.TempUnits;
        let mut tempattacktype: i32 =  this.game.TempCombat.Tempattacktype;
        this.game.TempCombat.DoBattle();
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut index2: i32 =  0; index2 <= icounter; index2 += 1)
        {
          let mut iattacker: i32 =  this.game.TempCombat.IList[index2].IAttacker;
          let mut isfType: i32 =  this.game.TempCombat.IList[index2].ISFType;
          if (this.game.TempCombat.IList[index2].IKilled > 0)
          {
            numArray13: Vec<i32> = numArray1;
            numArray14: Vec<i32> = numArray13;
            let mut index3: i32 =  isfType;
            let mut index4: i32 =  index3;
            let mut index5: i32 =  iattacker;
            let mut index6: i32 =  index5;
            let mut num3: i32 =  numArray13[index3, index5] + 1;
            numArray14[index4, index6] = num3;
          }
          else if (this.game.TempCombat.IList[index2].IRetreat > 0)
          {
            numArray15: Vec<i32> = numArray2;
            numArray16: Vec<i32> = numArray15;
            let mut index7: i32 =  isfType;
            let mut index8: i32 =  index7;
            let mut index9: i32 =  iattacker;
            let mut index10: i32 =  index9;
            let mut num4: i32 =  numArray15[index7, index9] + 1;
            numArray16[index8, index10] = num4;
          }
          else
          {
            numArray17: Vec<i32> = numArray3;
            numArray18: Vec<i32> = numArray17;
            let mut index11: i32 =  isfType;
            let mut index12: i32 =  index11;
            let mut index13: i32 =  iattacker;
            let mut index14: i32 =  index13;
            let mut num5: i32 =  numArray17[index11, index13] + 1;
            numArray18[index12, index14] = num5;
          }
          numArray19: Vec<i32> = numArray4;
          numArray20: Vec<i32> = numArray19;
          let mut index15: i32 =  isfType;
          let mut index16: i32 =  index15;
          let mut index17: i32 =  iattacker;
          let mut index18: i32 =  index17;
          let mut num6: i32 =  numArray19[index15, index17] + this.game.TempCombat.IList[index2].IRdn;
          numArray20[index16, index18] = num6;
          numArray21: Vec<f32> = numArray9;
          numArray22: Vec<f32> = numArray21;
          let mut index19: i32 =  isfType;
          let mut index20: i32 =  index19;
          let mut index21: i32 =  iattacker;
          let mut index22: i32 =  index21;
          double num7 =  numArray21[index19, index21] + 1.0;
          numArray22[index20, index22] =  num7;
          numArray23: Vec<f32> = numArray11;
          numArray24: Vec<f32> = numArray23;
          let mut index23: i32 =  isfType;
          let mut index24: i32 =  index23;
          let mut index25: i32 =  iattacker;
          let mut index26: i32 =  index25;
          double num8 =  numArray23[index23, index25] +  this.game.TempCombat.IList[index2].IMor;
          numArray24[index24, index26] =  num8;
          numArray25: Vec<f32> = numArray12;
          numArray26: Vec<f32> = numArray25;
          let mut index27: i32 =  isfType;
          let mut index28: i32 =  index27;
          let mut index29: i32 =  iattacker;
          let mut index30: i32 =  index29;
          double num9 =  numArray25[index27, index29] + 1.0;
          numArray26[index28, index30] =  num9;
        }
        if (this.game.TempCombat.BattleEnded == 1)
          Number2 += 1;
        if (this.game.TempCombat.BattleEnded == 3)
          Number3 += 1;
        if (this.game.TempCombat.BattleEnded == 2)
          Number4 += 1;
        num2 +=  this.game.TempCombat.AntiStrucDam;
        if (index1 != Number1)
        {
          this.game.TempCombat = new CombatClass(this.game);
          this.game.TempCombat.Init(tempTarget, tempType, tempUnits, tempattacktype);
        }
      }
      float Number5 = num2 /  Number1;
      let mut sfTypeCounter1: i32 =  this.game.Data.SFTypeCounter;
      for (let mut index31: i32 =  0; index31 <= sfTypeCounter1; index31 += 1)
      {
        let mut index32: i32 =  0;
        do
        {
          numArray5[index31, index32] =  numArray1[index31, index32] /  Number1;
          numArray6[index31, index32] =  numArray2[index31, index32] /  Number1;
          numArray7[index31, index32] =  numArray3[index31, index32] /  Number1;
          numArray8[index31, index32] =  ( numArray4[index31, index32] /  Number1 / ( numArray9[index31, index32] /  Number1));
          numArray10[index31, index32] =  ( numArray11[index31, index32] /  Number1 / ( numArray12[index31, index32] /  Number1));
          index32 += 1;
        }
        while (index32 <= 1);
      }
      this.AddLog("We did " + Conversion.Str( Number1) + " simulations. and these are averages:");
      this.AddLog(" ");
      this.AddLog("COMBAT OUTCOME:");
      this.AddLog(Strings.Space(3) + "Attack succeeded: " + Conversion.Str( Number2));
      this.AddLog(Strings.Space(3) + "Standoff: " + Conversion.Str( Number3));
      this.AddLog(Strings.Space(3) + "Attack failed: " + Conversion.Str( Number4));
      this.AddLog(" ");
      this.AddLog("DEFENDER AVERAGES:");
      let mut sfTypeCounter2: i32 =  this.game.Data.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter2; index += 1)
      {
        if ( numArray5[index, 0] > 0.0 |  numArray6[index, 0] > 0.0 |  numArray7[index, 0] > 0.0)
        {
          this.AddLog(Strings.Space(3) + "*" + this.game.Data.SFTypeObj[index].Name + ":");
          this.AddLog(Strings.Space(6) + "Death: " + Conversion.Str( numArray5[index, 0]));
          this.AddLog(Strings.Space(6) + "Retreat: " + Conversion.Str( numArray6[index, 0]));
          this.AddLog(Strings.Space(6) + "Live: " + Conversion.Str( numArray7[index, 0]));
          this.AddLog(Strings.Space(6) + "Rdn: " + Conversion.Str( numArray8[index, 0]));
          this.AddLog(Strings.Space(6) + "Mor: " + Conversion.Str( numArray10[index, 0]));
        }
      }
      this.AddLog(" ");
      this.AddLog("ATTACKER AVERAGES:");
      let mut sfTypeCounter3: i32 =  this.game.Data.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter3; index += 1)
      {
        if ( numArray5[index, 1] > 0.0 |  numArray6[index, 1] > 0.0 |  numArray7[index, 1] > 0.0)
        {
          this.AddLog(Strings.Space(3) + "*" + this.game.Data.SFTypeObj[index].Name + ":");
          this.AddLog(Strings.Space(6) + "Death: " + Conversion.Str( numArray5[index, 1]));
          this.AddLog(Strings.Space(6) + "Retreat: " + Conversion.Str( numArray6[index, 1]));
          this.AddLog(Strings.Space(6) + "Live: " + Conversion.Str( numArray7[index, 1]));
          this.AddLog(Strings.Space(6) + "Rdn: " + Conversion.Str( numArray8[index, 1]));
          this.AddLog(Strings.Space(6) + "Mor: " + Conversion.Str( numArray10[index, 1]));
        }
      }
      this.AddLog(" ");
      this.AddLog("Structural Damage =" + Conversion.Str( Number5));
      this.WriteLog();
    }

    pub fn AddLog(s: String)
    {
      this += 1.LogCounter;
      this.LogTxt = (string[]) Utils.CopyArray((Array) this.LogTxt, (Array) new string[this.LogCounter + 1]);
      this.LogTxt[this.LogCounter] = s;
    }

    pub fn WriteLog()
    {
      StreamWriter text = File.CreateText(this.game.AppPath + "logs\\combatsim.txt");
      let mut logCounter: i32 =  this.LogCounter;
      for (let mut index: i32 =  0; index <= logCounter; index += 1)
        text.WriteLine(this.LogTxt[index]);
      text.Close();
    }
  }
}
