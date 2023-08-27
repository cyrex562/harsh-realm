// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.StatsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class StatsWindowClass : WindowClass
  {
     TempText1: i32;
     temptext2: i32;
     temptext3: i32;
     temptext4: i32;
     temptext5: i32;
     temptext6: i32;
     temptext7: i32;
     temptext8: i32;
     temptext9: i32;
     temptext10: i32;
     TempText11: i32;
     temptext12: i32;
     temptext13: i32;
     temptext14: i32;
     temptext15: i32;
     temptext16: i32;
     temptext17: i32;
     temptext18: i32;
     temptext19: i32;
     int[] TabId;
     but1id: i32;
     but1textid: i32;
     TabCount: i32;
     but2id: i32;
     but2textid: i32;
     but3id: i32;
     but3textid: i32;
     but4id: i32;
     but4textid: i32;
     but5id: i32;
     but5textid: i32;
     GOid: i32;
     UpgradeId: i32;
     headytxt: i32;
     sliderid: i32;
     float tempBlink;
     hq: i32;
     sfnr: i32;
     detailnr: i32;
     detailnr2: i32;
     detailnr3: i32;
     detailnr4: i32;
     detailnr5: i32;
     detailtype: i32;
     ammount: i32;
     bool hqreach;
     passenger: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     SortStart: i32;
     sortcount: i32;
     ModeTextId: i32;
     ModeButton0Id: i32;
     ModeButton1Id: i32;
     ModeButton2Id: i32;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ATListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ATListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ATListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ATListClass OptionsList6Obj;
     OptionsList7Id: i32;
     ATListClass OptionsList7Obj;
     OptionsList8Id: i32;
     ATListClass OptionsList8Obj;
     OptionsList9Id: i32;
     ATListClass OptionsList9Obj;
     StatMode: i32;
     StatAggr: i32;
     Abstr: i32;
     int[] mzx;
     int[] mzy;
     int[] mzx2;
     int[] mzy2;
     int[] mznr;
     int[] mzdetnr;
     mzcount: i32;
     bool supplycalcdone;
     int[] supplyneed1;
    pub supplyneed2: Vec<i32>;
     int[] supplyneed3;
     int[] supplyneed4;
    pub supplyout1: Vec<i32>;
     int[] supplyout2;
     int[] supplyout3;
     int[] supplyin1;
     int[] supplyin2;
     int[] supplyin3;
     int[] supplyin4;
     truextra: i32;

    pub StatsWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      self.TabId = new int[100];
      self.mzx = new int[2001];
      self.mzy = new int[2001];
      self.mzx2 = new int[2001];
      self.mzy2 = new int[2001];
      self.mznr = new int[2001];
      self.mzdetnr = new int[2001];
      self.supplyneed1 = new int[1];
      self.supplyneed2 = new int[1];
      self.supplyneed3 = new int[1];
      self.supplyneed4 = new int[1];
      self.supplyout1 = new int[1];
      self.supplyout2 = new int[1];
      self.supplyout3 = new int[1];
      self.supplyin1 = new int[1];
      self.supplyin2 = new int[1];
      self.supplyin3 = new int[1];
      self.supplyin4 = new int[1];
      self.SortStart = 0;
      self.StatMode = 0;
      if (self.game.EditObj.LastStatWindow > 0)
        self.StatMode = self.game.EditObj.LastStatWindow;
      if (self.StatMode == 9 && !self.supplycalcdone)
      {
        self.game.FormRef.Cursor = Cursors.WaitCursor;
        self.CalculateSupply();
        self.game.FormRef.Cursor = Cursors.Default;
      }
      self.StatAggr = 0;
      self.detailnr = -1;
      self.detailnr2 = -1;
      self.detailnr3 = -1;
      self.detailnr4 = -1;
      self.detailnr5 = -1;
      self.supplycalcdone = false;
      self.DoStuff();
    }

    pub fn DoStuff()
    {
      int[] numArray1 = new int[self.game.Data.UnitCounter + 1];
      int[] numArray2 = new int[self.game.Data.UnitCounter + 1];
      int[] numArray3 = new int[self.game.Data.UnitCounter + 1];
      int[] numArray4 = new int[self.game.Data.UnitCounter + 1];
      if (self.but1id > 0)
        self.RemoveSubPart(self.but1id);
      if (self.but1textid > 0)
        self.RemoveSubPart(self.but1textid);
      if (self.but2id > 0)
        self.RemoveSubPart(self.but2id);
      if (self.but2textid > 0)
        self.RemoveSubPart(self.but2textid);
      if (self.but3id > 0)
        self.RemoveSubPart(self.but3id);
      if (self.but3textid > 0)
        self.RemoveSubPart(self.but3textid);
      if (self.but4id > 0)
        self.RemoveSubPart(self.but4id);
      if (self.but4textid > 0)
        self.RemoveSubPart(self.but4textid);
      if (self.but5id > 0)
        self.RemoveSubPart(self.but5id);
      if (self.but5textid > 0)
        self.RemoveSubPart(self.but5textid);
      if (self.ModeTextId > 0)
        self.RemoveSubPart(self.ModeTextId);
      if (self.ModeButton0Id > 0)
        self.RemoveSubPart(self.ModeButton0Id);
      if (self.ModeButton1Id > 0)
        self.RemoveSubPart(self.ModeButton1Id);
      if (self.ModeButton2Id > 0)
        self.RemoveSubPart(self.ModeButton2Id);
      if (self.GOid > 0)
        self.RemoveSubPart(self.GOid);
      if (self.UpgradeId > 0)
        self.RemoveSubPart(self.UpgradeId);
      if (self.headytxt > 0)
        self.RemoveSubPart(self.headytxt);
      if (self.TempText1 > 0)
        self.RemoveSubPart(self.TempText1);
      if (self.temptext2 > 0)
        self.RemoveSubPart(self.temptext2);
      if (self.temptext3 > 0)
        self.RemoveSubPart(self.temptext3);
      if (self.temptext4 > 0)
        self.RemoveSubPart(self.temptext4);
      let mut index1: i32 = 0;
      do
      {
        if (self.TabId[index1] > 0)
        {
          self.RemoveSubPart(self.TabId[index1]);
          self.TabId[index1] = 0;
        }
        index1 += 1;
      }
      while (index1 <= 19);
      let mut mzcount1: i32 = self.mzcount;
      for (let mut index2: i32 = 0; index2 <= mzcount1; index2 += 1)
      {
        self.mzx[index2] = -1;
        self.mzy[index2] = -1;
        self.mzx2[index2] = -1;
        self.mzy2[index2] = -1;
      }
      self.mzcount = -1;
      self.NewBackGroundAndClearAll(1024, 768, self.game.BACKGROUND2MARC);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      vicFont2: Font = self.game.VicFont2;
      vicFont4: Font = self.game.VicFont4;
      vicFont3: Font = self.game.VicFont3;
      bool flag;
      if (!self.game.Data.FOWOn)
        flag = true;
      if (self.game.Data.Winner > -1)
        flag = true;
      self.mzcount = -1;
      let mut num1: i32 = 0;
      if ( self.game.Data.RuleVar[650] > 0.0)
        num1 += 1;
      if ( self.game.Data.RuleVar[651] > 0.0)
        num1 += 1;
      if ( self.game.Data.RuleVar[652] > 0.0)
        num1 += 1;
      self.truextra = num1;
      let mut num2: i32 = 20;
      let mut num3: i32 = 0;
      do
      {
        buttontext: String;
        if (num3 == 0)
        {
          buttontext = "Produce";
          num2 = 510;
        }
        if (num3 == 1)
        {
          buttontext = "Losses";
          num2 = 580;
        }
        if (num3 == 2)
        {
          buttontext = "Kills";
          num2 = 650;
        }
        if (num3 == 3)
        {
          buttontext = "Totals";
          num2 = 90;
        }
        if (num3 == 4)
        {
          buttontext = "OOB";
          num2 = 20;
        }
        if (num3 == 5)
        {
          buttontext = "Power";
          num2 = 160;
        }
        if (num3 == 6)
        {
          buttontext = "Rules";
          num2 = 230;
        }
        if (num3 == 7)
        {
          buttontext = "Brief";
          num2 = 300;
        }
        if (num3 == 8)
        {
          buttontext = "Force";
          num2 = 440;
        }
        if (num3 == 9)
        {
          buttontext = "Supply";
          num2 = 370;
        }
        int[] tabId = self.TabId;
        let mut index3: i32 = num3;
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext, 68, tBackbitmap: ( self.OwnBitmap), bbx: num2, bby: 20, tred: (self.StatMode == num3));
        let mut num4: i32 = self.AddSubPart( tsubpart, num2, 20, 68, 35, 1);
        tabId[index3] = num4;
        num3 += 1;
      }
      while (num3 <= 9);
      let mut num5: i32 = 720;
      SubPartClass tsubpart1;
      if (num1 > 0)
      {
        let mut num6: i32 = 9 + num1;
        for (let mut index4: i32 = 10; index4 <= num6; index4 += 1)
        {
          buttontext: String = Strings.Left(self.game.Data.TempString[700 + (index4 - 10)], 3);
          int[] tabId = self.TabId;
          let mut index5: i32 = index4;
          tsubpart1 =  new TextButtonPartClass(buttontext, 68, tBackbitmap: ( self.OwnBitmap), bbx: num5, bby: 20, tred: (self.StatMode == index4));
          let mut num7: i32 = self.AddSubPart( tsubpart1, num5, 20, 68, 35, 1);
          tabId[index5] = num7;
          num5 += 70;
        }
      }
      tsubpart1 =  ButtonPartClass::new(self.game.BUTTONQUIT);
      self.but1id = self.AddSubPart( tsubpart1, 952, 22, 35, 35, 1);
      num8: i32;
      if (self.StatMode == 4 | self.StatMode == 9)
      {
        DrawMod.DrawBlock( graphics, 30, 94, 900, 48, 0, 0, 0, 166);
        DrawMod.DrawBlock( graphics, 30, 144, 900, 48, 0, 0, 0, 166);
        DrawMod.DrawBlock( graphics, 30, 194, 900, 48, 0, 0, 0, 166);
        DrawMod.DrawBlock( graphics, 30, 244, 900, 248, 0, 0, 0, 166);
        DrawMod.DrawRectangle( graphics, 30, 94, 900, 48,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.DrawRectangle( graphics, 30, 144, 900, 48,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.DrawRectangle( graphics, 30, 194, 900, 48,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.DrawRectangle( graphics, 30, 244, 900, 248,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        self.sortcount = -1;
        let mut num9: i32 = -1;
        let mut unitCounter1: i32 = self.game.Data.UnitCounter;
        for (let mut index6: i32 = 0; index6 <= unitCounter1; index6 += 1)
        {
          numArray1[index6] = -1;
          numArray3[index6] = 0;
        }
        num10: i32;
        do
        {
          num10 = 0;
          num9 += 1;
          let mut unitCounter2: i32 = self.game.Data.UnitCounter;
          for (let mut index7: i32 = 0; index7 <= unitCounter2; index7 += 1)
          {
            if (self.game.Data.UnitObj[index7].PreDef == -1 && self.game.Data.UnitObj[index7].IsHQ | !self.game.Data.UnitObj[index7].IsHQ & self.game.Data.UnitObj[index7].HQ == -1 && (self.game.Data.RegimeObj[self.game.Data.UnitObj[index7].Regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[index7].Regime].UberRegime > -1 & self.game.Data.RegimeObj[self.game.Data.UnitObj[index7].Regime].UberRegime == self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == self.game.Data.UnitObj[index7].Regime | self.game.Data.Turn == self.game.Data.UnitObj[index7].Regime) & self.game.Data.UnitObj[index7].PreDef <= -1)
            {
              if (num9 == 0)
              {
                if (self.game.Data.UnitObj[index7].HQ == -1)
                {
                  numArray1[index7] = 0;
                  num10 = 1;
                }
              }
              else if (self.game.Data.UnitObj[index7].HQ > -1 & numArray1[index7] == -1 & self.game.Data.UnitObj[index7].PreDef <= -1)
              {
                let mut hq: i32 = self.game.Data.UnitObj[index7].HQ;
                if (numArray1[hq] == num9 - 1)
                {
                  num10 = 1;
                  numArray1[index7] = num9;
                }
              }
            }
          }
        }
        while (num10 > 0);
        let mut num11: i32 = num9 - 1;
        if (num11 == -1)
          return;
        if (num11 >= 2)
        {
          let mut num12: i32 = 0;
          let mut unitCounter3: i32 = self.game.Data.UnitCounter;
          for (let mut index8: i32 = 0; index8 <= unitCounter3; index8 += 1)
          {
            if (numArray1[index8] <= num11 - 2 & numArray1[index8] > -1)
              num12 += 1;
          }
          if (num12 > 0)
          {
            let mut num13: i32 =  Math.Round(850.0 /  num12);
            if (num13 > 40)
              num13 = 40;
            let mut num14: i32 = 0;
            num8 = 0;
            let mut unitCounter4: i32 = self.game.Data.UnitCounter;
            for (let mut nr: i32 = 0; nr <= unitCounter4; nr += 1)
            {
              if (numArray1[nr] <= num11 - 2 & numArray1[nr] > -1)
              {
                num8 += 1;
                if (self.detailnr == -1)
                {
                  self.detailnr = nr;
                  self.detailnr5 = nr;
                }
                self.game.CustomBitmapObj.DrawUnit(nr, toG: graphics, tx: (num14 + 50), ty: 100);
                if (nr == self.detailnr)
                  DrawMod.DrawRectangle( graphics, num14 + 50 - 1, 99, 37, 37,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                this += 1.mzcount;
                self.mzx[self.mzcount] = num14 + 50;
                self.mzy[self.mzcount] = 100;
                self.mzx2[self.mzcount] = num14 + num13 + 50;
                self.mzy2[self.mzcount] = 138;
                self.mznr[self.mzcount] = nr;
                self.mzdetnr[self.mzcount] = 1;
                num14 += num13;
                if (num8 == num12 & 38 > num13)
                {
                  int[] mzx2 = self.mzx2;
                  int[] numArray5 = mzx2;
                  let mut mzcount2: i32 = self.mzcount;
                  let mut index9: i32 = mzcount2;
                  let mut num15: i32 = mzx2[mzcount2] + (38 - num13);
                  numArray5[index9] = num15;
                }
              }
            }
          }
        }
        if (num11 >= 1 & (self.detailnr > -1 | num11 < 2))
        {
          let mut num16: i32 = 0;
          let mut unitCounter5: i32 = self.game.Data.UnitCounter;
          for (let mut index10: i32 = 0; index10 <= unitCounter5; index10 += 1)
          {
            if (numArray1[index10] <= num11 - 1 & numArray1[index10] > -1 & self.game.Data.UnitObj[index10].HQ == self.detailnr)
              num16 += 1;
          }
          if (num16 > 0)
          {
            let mut num17: i32 =  Math.Round(850.0 /  num16);
            if (num17 > 40)
              num17 = 40;
            let mut num18: i32 = 0;
            num8 = 0;
            let mut unitCounter6: i32 = self.game.Data.UnitCounter;
            for (let mut nr: i32 = 0; nr <= unitCounter6; nr += 1)
            {
              if (numArray1[nr] <= num11 - 1 & numArray1[nr] > -1 && self.game.Data.UnitObj[nr].HQ == self.detailnr)
              {
                num8 += 1;
                if (self.detailnr2 == -1 & num11 == 1)
                {
                  self.detailnr2 = nr;
                  self.detailnr5 = nr;
                }
                self.game.CustomBitmapObj.DrawUnit(nr, toG: graphics, tx: (num18 + 50), ty: 150);
                if (nr == self.detailnr2)
                  DrawMod.DrawRectangle( graphics, num18 + 50 - 1, 149, 37, 37,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                this += 1.mzcount;
                self.mzx[self.mzcount] = num18 + 50;
                self.mzy[self.mzcount] = 150;
                self.mzx2[self.mzcount] = num18 + num17 + 50;
                self.mzy2[self.mzcount] = 188;
                self.mznr[self.mzcount] = nr;
                self.mzdetnr[self.mzcount] = 2;
                num18 += num17;
                if (num8 == num16 & 38 > num17)
                {
                  int[] mzx2 = self.mzx2;
                  int[] numArray6 = mzx2;
                  let mut mzcount3: i32 = self.mzcount;
                  let mut index11: i32 = mzcount3;
                  let mut num19: i32 = mzx2[mzcount3] + (38 - num17);
                  numArray6[index11] = num19;
                }
              }
            }
          }
        }
        if (num11 >= 0 & (self.detailnr2 > -1 | num11 < 1))
        {
          let mut num20: i32 = 0;
          let mut unitCounter7: i32 = self.game.Data.UnitCounter;
          for (let mut index12: i32 = 0; index12 <= unitCounter7; index12 += 1)
          {
            if (numArray1[index12] <= num11 & numArray1[index12] > -1 & self.game.Data.UnitObj[index12].HQ == self.detailnr2)
              num20 += 1;
          }
          if (num20 > 0)
          {
            let mut num21: i32 =  Math.Round(850.0 /  num20);
            if (num21 > 40)
              num21 = 40;
            let mut num22: i32 = 0;
            num8 = 0;
            let mut unitCounter8: i32 = self.game.Data.UnitCounter;
            for (let mut nr: i32 = 0; nr <= unitCounter8; nr += 1)
            {
              if (numArray1[nr] <= num11 & numArray1[nr] > -1 && self.game.Data.UnitObj[nr].HQ == self.detailnr2)
              {
                num8 += 1;
                if (self.detailnr3 == -1 & num11 == 0)
                {
                  self.detailnr3 = nr;
                  self.detailnr5 = nr;
                }
                self.game.CustomBitmapObj.DrawUnit(nr, toG: graphics, tx: (num22 + 50), ty: 200);
                if (nr == self.detailnr3)
                  DrawMod.DrawRectangle( graphics, num22 + 50 - 1, 199, 37, 37,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                this += 1.mzcount;
                self.mzx[self.mzcount] = num22 + 50;
                self.mzy[self.mzcount] = 200;
                self.mzx2[self.mzcount] = num22 + num21 + 50;
                self.mzy2[self.mzcount] = 238;
                self.mznr[self.mzcount] = nr;
                self.mzdetnr[self.mzcount] = 3;
                num22 += num21;
                if (num8 == num20 & 38 > num21)
                {
                  int[] mzx2 = self.mzx2;
                  int[] numArray7 = mzx2;
                  let mut mzcount4: i32 = self.mzcount;
                  let mut index13: i32 = mzcount4;
                  let mut num23: i32 = mzx2[mzcount4] + (38 - num21);
                  numArray7[index13] = num23;
                }
              }
            }
          }
        }
        num24: i32;
        if (self.detailnr > -1)
          num24 = self.detailnr;
        if (self.detailnr2 > -1)
          num24 = self.detailnr2;
        if (self.detailnr3 > -1)
          num24 = self.detailnr3;
        if (self.detailnr5 > -1 && self.game.Data.UnitObj[self.detailnr5].IsHQ)
          num24 = self.detailnr5;
        if (num24 > -1)
        {
          let mut num25: i32 = 0;
          let mut unitCounter9: i32 = self.game.Data.UnitCounter;
          for (let mut index14: i32 = 0; index14 <= unitCounter9; index14 += 1)
          {
            if (numArray1[index14] == -1 & self.game.Data.UnitObj[index14].HQ == num24 && self.game.Data.UnitObj[index14].PreDef == -1 && self.game.Data.RegimeObj[self.game.Data.UnitObj[index14].Regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[index14].Regime].UberRegime > -1 & self.game.Data.RegimeObj[self.game.Data.UnitObj[index14].Regime].UberRegime == self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == self.game.Data.UnitObj[index14].Regime | self.game.Data.Turn == self.game.Data.UnitObj[index14].Regime)
              num25 += 1;
          }
          if (num25 > 0)
          {
            let mut num26: i32 =  Math.Round(5100.0 /  num25);
            if (num26 > 40)
              num26 = 40;
            let mut num27: i32 = 0;
            let mut ty: i32 = 250;
            num8 = 0;
            let mut unitCounter10: i32 = self.game.Data.UnitCounter;
            for (let mut nr: i32 = 0; nr <= unitCounter10; nr += 1)
            {
              if (numArray1[nr] == -1 && self.game.Data.UnitObj[nr].HQ == num24 && self.game.Data.UnitObj[nr].PreDef == -1 && self.game.Data.RegimeObj[self.game.Data.UnitObj[nr].Regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[nr].Regime].UberRegime > -1 & self.game.Data.RegimeObj[self.game.Data.UnitObj[nr].Regime].UberRegime == self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == self.game.Data.UnitObj[nr].Regime | self.game.Data.Turn == self.game.Data.UnitObj[nr].Regime)
              {
                num8 += 1;
                bool forcehighlight = self.detailnr4 == nr;
                self.game.CustomBitmapObj.DrawUnit(nr, forcehighlight, graphics, num27 + 50, ty);
                if (nr == self.detailnr4)
                  DrawMod.DrawRectangle( graphics, num27 + 50 - 1, ty - 1, 37, 37,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                this += 1.mzcount;
                self.mzx[self.mzcount] = num27 + 50;
                self.mzy[self.mzcount] = ty;
                self.mzx2[self.mzcount] = num27 + num26 + 50;
                self.mzy2[self.mzcount] = ty + 38;
                self.mznr[self.mzcount] = nr;
                self.mzdetnr[self.mzcount] = 4;
                num27 += num26;
                if (num27 + num26 + 38 > 850)
                {
                  num27 = 0;
                  ty += 40;
                  int[] mzx2 = self.mzx2;
                  int[] numArray8 = mzx2;
                  let mut mzcount5: i32 = self.mzcount;
                  let mut index15: i32 = mzcount5;
                  let mut num28: i32 = mzx2[mzcount5] + (38 - num26);
                  numArray8[index15] = num28;
                }
                if (num8 == num25 & 38 > num26)
                {
                  int[] mzx2 = self.mzx2;
                  int[] numArray9 = mzx2;
                  let mut mzcount6: i32 = self.mzcount;
                  let mut index16: i32 = mzcount6;
                  let mut num29: i32 = mzx2[mzcount6] + (38 - num26);
                  numArray9[index16] = num29;
                }
              }
            }
          }
        }
        if (self.detailnr5 > -1)
        {
          if (self.StatMode == 4)
          {
            tsubpart1 =  new UnitHeaderPartClass(self.detailnr5, self.game);
            self.TempText1 = self.AddSubPart( tsubpart1, 40, 500, 280, 200, 0);
          }
          if (self.StatMode == 4)
          {
            if (self.StatMode == 4)
            {
              if (self.game.Data.UnitObj[self.detailnr5].Historical == -1)
              {
                tsubpart1 =  new UnitSFPartClass(self.detailnr5, self.game);
                self.temptext2 = self.AddSubPart( tsubpart1, 340, 500, 620, 200, 0);
              }
              else if (self.game.Data.UnitObj[self.detailnr5].Historical > -1)
              {
                tsubpart1 =  new OfficerPartClass(self.detailnr5, self.game);
                self.temptext2 = self.AddSubPart( tsubpart1, 340, 500, 300, 200, 0);
              }
            }
          }
          else if (self.StatMode == 9)
          {
            let mut num30: i32 = -130;
            let mut num31: i32 = 30;
            DrawMod.DrawTextVic2( graphics, "Supply need prognosis", self.game.VicFont2, num30 + 290, num31 + 500, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Clean Supply Request", self.game.VicFont3, num30 + 290, num31 + 530, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyneed1[self.detailnr5]), self.game.VicFont3, num30 + 430, num31 + 530, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "    after range penalty", self.game.VicFont3, num30 + 290, num31 + 550, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyneed4[self.detailnr5]), self.game.VicFont3, num30 + 430, num31 + 550, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Supply Consumption", self.game.VicFont3, num30 + 290, num31 + 570, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyneed2[self.detailnr5]), self.game.VicFont3, num30 + 430, num31 + 570, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Current Supply Pts", self.game.VicFont3, num30 + 290, num31 + 590, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.game.Data.UnitObj[self.detailnr5].Supply), self.game.VicFont3, num30 + 430, num31 + 590, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Maximum Supply Pts", self.game.VicFont3, num30 + 290, num31 + 610, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyneed3[self.detailnr5]), self.game.VicFont3, num30 + 430, num31 + 610, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Supply IN prognosis", self.game.VicFont2, num30 + 520, num31 + 500, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Production", self.game.VicFont3, num30 + 520, num31 + 530, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyin1[self.detailnr5]), self.game.VicFont3, num30 + 660, num31 + 530, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Free Reserves", self.game.VicFont3, num30 + 520, num31 + 550, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyin2[self.detailnr5]), self.game.VicFont3, num30 + 660, num31 + 550, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Request at high HQ", self.game.VicFont3, num30 + 520, num31 + 570, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyin3[self.detailnr5]), self.game.VicFont3, num30 + 660, num31 + 570, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, "Get from high HQ", self.game.VicFont3, num30 + 520, num31 + 590, self.game.VicColor2, self.game.VicColor2Shade);
            DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyin4[self.detailnr5]), self.game.VicFont3, num30 + 660, num31 + 590, self.game.VicColor2, self.game.VicColor2Shade);
            if (self.game.Data.UnitObj[self.detailnr5].IsHQ)
            {
              DrawMod.DrawTextVic2( graphics, "Supply OUT prognosis", self.game.VicFont2, num30 + 750, num31 + 500, self.game.VicColor2, self.game.VicColor2Shade);
              DrawMod.DrawTextVic2( graphics, "Subordinate Requests", self.game.VicFont3, num30 + 750, num31 + 530, self.game.VicColor2, self.game.VicColor2Shade);
              DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyout1[self.detailnr5]), self.game.VicFont3, num30 + 910, num31 + 530, self.game.VicColor2, self.game.VicColor2Shade);
              DrawMod.DrawTextVic2( graphics, "   after range penalty", self.game.VicFont3, num30 + 750, num31 + 550, self.game.VicColor2, self.game.VicColor2Shade);
              DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyout2[self.detailnr5]), self.game.VicFont3, num30 + 910, num31 + 550, self.game.VicColor2, self.game.VicColor2Shade);
              DrawMod.DrawTextVic2( graphics, "Supply to subordinates", self.game.VicFont3, num30 + 750, num31 + 570, self.game.VicColor2, self.game.VicColor2Shade);
              DrawMod.DrawTextVic2( graphics, Conversion.Str( self.supplyout3[self.detailnr5]), self.game.VicFont3, num30 + 910, num31 + 570, self.game.VicColor2, self.game.VicColor2Shade);
            }
          }
          if (self.game.Data.UnitObj[self.detailnr5].X > -1)
          {
            tsubpart1 =  new TextButtonPartClass("Go to this unit", 150, tBackbitmap: ( self.OwnBitmap), bbx: 260, bby: 710);
            self.GOid = self.AddSubPart( tsubpart1, 260, 710, 150, 35, 1);
            if (self.game.Data.UnitObj[self.detailnr5].IsHQ & self.game.Data.UnitObj[self.detailnr5].X > -1 && self.game.Data.ResearchCounter > -1)
            {
              tsubpart1 =  new TextButtonPartClass("Auto Upgrade", 150, tBackbitmap: ( self.OwnBitmap), bbx: 430, bby: 710);
              self.UpgradeId = self.AddSubPart( tsubpart1, 430, 710, 150, 35, 1);
            }
          }
        }
      }
      if (self.StatMode == 0 | self.StatMode == 1 | self.StatMode == 2 | self.StatMode == 8 | self.StatMode > 9)
      {
        DrawMod.DrawBlock( graphics, 860, 120, 140, 112,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
        if (self.StatAggr == 0)
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 860, bby: 120);
          self.ModeButton0Id = self.AddSubPart( tsubpart1, 860, 120, 32, 32, 1);
        }
        else
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 860, bby: 120);
          self.ModeButton0Id = self.AddSubPart( tsubpart1, 860, 120, 32, 32, 1);
        }
        if (self.StatAggr == 1)
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 860, bby: 160);
          self.ModeButton1Id = self.AddSubPart( tsubpart1, 860, 160, 32, 32, 1);
        }
        else
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 860, bby: 160);
          self.ModeButton1Id = self.AddSubPart( tsubpart1, 860, 160, 32, 32, 1);
        }
        if (self.StatAggr == 2)
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 860, bby: 200);
          self.ModeButton2Id = self.AddSubPart( tsubpart1, 860, 200, 32, 32, 1);
        }
        else
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 860, bby: 200);
          self.ModeButton2Id = self.AddSubPart( tsubpart1, 860, 200, 32, 32, 1);
        }
        DrawMod.DrawTextVic2( graphics, "Default", self.game.VicFont3, 900, 132, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Normalized", self.game.VicFont3, 900, 172, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Very Normalized", self.game.VicFont3, 900, 212, self.game.VicColor2, self.game.VicColor2Shade);
      }
      TimeSpan timeSpan;
      if (self.StatMode > 9)
      {
        let mut num32: i32 = 10;
        let mut num33: i32 = 20;
        let mut index17: i32 = self.StatMode - 10;
        if (self.game.Data.Round > num33)
          num33 = self.game.Data.Round;
        let mut num34: i32 =  Math.Round( num33 + Conversion.Int( num33 * 0.2));
        float[,] numArray10 = new float[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        let mut regimeCounter1: i32 = self.game.Data.RegimeCounter;
        for (let mut index18: i32 = 0; index18 <= regimeCounter1; index18 += 1)
        {
          let mut round: i32 = self.game.Data.Round;
          for (let mut index19: i32 = 1; index19 <= round; index19 += 1)
          {
            float[,] numArray11 = numArray10;
            float[,] numArray12 = numArray11;
            let mut index20: i32 = index18;
            let mut index21: i32 = index20;
            let mut index22: i32 = index19;
            let mut index23: i32 = index22;
            double num35 =  numArray11[index20, index22] +  self.game.Data.RegimeObj[index18].ExtraStat[index17, index19];
            numArray12[index21, index23] =  num35;
            if ( numArray10[index18, index19] >  num32)
              num32 =  Math.Round( numArray10[index18, index19]);
          }
        }
        float[,] numArray13 = new float[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        if (self.StatAggr > 0)
        {
          if (self.StatAggr == 1)
            num8 = 3;
          if (self.StatAggr == 2)
            num8 = 10;
          let mut regimeCounter2: i32 = self.game.Data.RegimeCounter;
          for (let mut index24: i32 = 0; index24 <= regimeCounter2; index24 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index25: i32 = 1; index25 <= round; index25 += 1)
            {
              let mut num36: i32 = 0;
              let mut num37: i32 = 0;
              let mut num38: i32 = index25 - num8;
              let mut num39: i32 = index25 + num8;
              for (let mut index26: i32 = num38; index26 <= num39; index26 += 1)
              {
                if (index26 > 0 & index26 <= self.game.Data.Round)
                {
                  num36 += 1;
                  num37 =  Math.Round( ( num37 + numArray10[index24, index26]));
                }
              }
              numArray13[index24, index25] =  num37 /  num36;
            }
          }
          let mut regimeCounter3: i32 = self.game.Data.RegimeCounter;
          for (let mut index27: i32 = 0; index27 <= regimeCounter3; index27 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index28: i32 = 1; index28 <= round; index28 += 1)
              numArray10[index27, index28] = numArray13[index27, index28];
          }
        }
        DrawMod.drawLine( graphics, 100, 100, 100, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.drawLine( graphics, 100, 700, 850, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
         let mut local1: &Graphics = &graphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.STATSGRADIENT);
         let mut local2: &Bitmap = &bitmap;
        let mut y: i32 = 699 - BitmapStore.Getheight(self.game.STATSGRADIENT);
        double r =  self.game.Data.RegimeObj[self.game.Data.Turn].Red /  byte.MaxValue - 1.0;
        double g =  self.game.Data.RegimeObj[self.game.Data.Turn].Green /  byte.MaxValue - 1.0;
        double b =  self.game.Data.RegimeObj[self.game.Data.Turn].Blue /  byte.MaxValue - 1.0;
        DrawMod.Draw( local1,  local2, 101, y,  r,  g,  b, 1f);
        DrawMod.DrawTextVic2( graphics, self.game.Data.TempString[700 + index17], self.game.VicFont2, 15, 85, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Round", self.game.VicFont2, 880, 720, self.game.VicColor2, self.game.VicColor2Shade);
        let mut regimeCounter4: i32 = self.game.Data.RegimeCounter;
        for (let mut index29: i32 = 0; index29 <= regimeCounter4; index29 += 1)
        {
          let mut x2: i32 = 0;
          let mut y2: i32 = 0;
          if ( self.game.Data.RuleVar[313] == 1.0 | index29 == self.game.Data.Turn | !self.game.Data.RegimeObj[index29].DipBlock & !(self.game.Data.RegimeObj[index29].AI & self.game.Data.RegimeObj[index29].Sleep) &&  self.game.Data.RuleVar[313] == 1.0 | flag | index29 == self.game.Data.Turn)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index30: i32 = 1; index30 <= round; index30 += 1)
            {
              if (!(index30 == self.game.Data.Round & index29 > self.game.Data.Turn))
              {
                let mut red: i32 = self.game.Data.RegimeObj[index29].Red;
                let mut green: i32 = self.game.Data.RegimeObj[index29].Green;
                let mut blue: i32 = self.game.Data.RegimeObj[index29].Blue;
                let mut x1: i32 =  Math.Round( (index30 - 1) /  num34 * 750.0 + 100.0);
                let mut y1: i32 =  Math.Round(700.0 -  numArray10[index29, index30] /  num32 * 600.0);
                if (x2 > 0)
                {
                  DrawMod.drawLine( graphics, x1, y1, x2, y2, red, green, blue,  byte.MaxValue, 2);
                  DrawMod.drawLine( graphics, x1, y1 + 1, x2, y2 + 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 2);
                }
                DrawMod.DrawBlock( graphics, x1 - 3, y1 - 3, 6, 6, red, green, blue,  byte.MaxValue);
                DrawMod.DrawRectangle( graphics, x1 - 3, y1 - 3, 6, 6,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                x2 = x1;
                y2 = y1;
                if (index30 == self.game.Data.Round)
                  DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index29].Name, vicFont4, x1 + 5, y1 - 15, self.game.VicColor2, self.game.VicColor2Shade);
              }
              else if (x2 != 0)
                DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index29].Name, vicFont4, x2 + 5, y2 - 15, self.game.VicColor2, self.game.VicColor2Shade);
            }
          }
        }
        let mut num40: i32 = 1;
        do
        {
          DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num32 * ( num40 / 10.0))), vicFont4, 20, 700 - 60 * num40, self.game.VicColor2, self.game.VicColor2Shade);
          if (self.game.Data.AlternateRound > -1)
          {
            DateTime dateTime = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1));
            let mut num41: i32 =  Math.Round(Conversion.Int( num34 * ( num40 / 10.0)));
            if (self.game.Data.AlternateRound == 31)
            {
              dateTime = dateTime.AddMonths((num41 - 1) * 1);
            }
            else
            {
              timeSpan = new TimeSpan((num41 - 1) * self.game.Data.AlternateRound, 0, 0, 0);
              dateTime = dateTime.Add(timeSpan);
            }
            tstring1: String = Strings.Left(self.game.HandyFunctionsObj.GetMonth(dateTime.Month), 3) + " " + Strings.Trim(Conversion.Str( dateTime.Day));
            DrawMod.DrawTextVic2( graphics, tstring1, vicFont4,  Math.Round(( num41 - 1.5) /  num34 * 750.0 + 100.0), 710, self.game.VicColor2, self.game.VicColor2Shade);
            tstring2: String = Strings.Trim(Conversion.Str( dateTime.Year));
            DrawMod.DrawTextVic2( graphics, tstring2, vicFont4,  Math.Round(( num41 - 1.5) /  num34 * 750.0 + 100.0), 725, self.game.VicColor2, self.game.VicColor2Shade);
          }
          else
          {
            let mut num42: i32 =  Math.Round(Conversion.Int( num34 * ( num40 / 10.0)));
            DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num34 * ( num40 / 10.0))), vicFont4,  Math.Round(( num42 - 1.5) /  num34 * 750.0 + 115.0), 720, self.game.VicColor2, self.game.VicColor2Shade);
          }
          num40 += 1;
        }
        while (num40 <= 10);
      }
      if (self.StatMode == 8)
      {
        let mut num43: i32 = 1000;
        let mut num44: i32 = 20;
        if (self.game.Data.Round > num44)
          num44 = self.game.Data.Round;
        let mut num45: i32 =  Math.Round( num44 + Conversion.Int( num44 * 0.2));
        numArray14: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        let mut regimeCounter5: i32 = self.game.Data.RegimeCounter;
        for (let mut index31: i32 = 0; index31 <= regimeCounter5; index31 += 1)
        {
          let mut round: i32 = self.game.Data.Round;
          for (let mut index32: i32 = 1; index32 <= round; index32 += 1)
          {
            let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
            for (let mut index33: i32 = 0; index33 <= sfTypeCounter; index33 += 1)
            {
              numArray15: Vec<i32> = numArray14;
              numArray16: Vec<i32> = numArray15;
              let mut index34: i32 = index31;
              let mut index35: i32 = index34;
              let mut index36: i32 = index32;
              let mut index37: i32 = index36;
              let mut num46: i32 = numArray15[index34, index36] + self.game.Data.RegimeObj[index31].SPresent[index33, index32] * self.game.Data.SFTypeObj[index33].PowerPts;
              numArray16[index35, index37] = num46;
              if (numArray14[index31, index32] > num43)
                num43 = numArray14[index31, index32];
            }
          }
        }
        numArray17: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        if (self.StatAggr > 0)
        {
          if (self.StatAggr == 1)
            num8 = 3;
          if (self.StatAggr == 2)
            num8 = 10;
          let mut regimeCounter6: i32 = self.game.Data.RegimeCounter;
          for (let mut index38: i32 = 0; index38 <= regimeCounter6; index38 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index39: i32 = 1; index39 <= round; index39 += 1)
            {
              let mut num47: i32 = 0;
              let mut num48: i32 = 0;
              let mut num49: i32 = index39 - num8;
              let mut num50: i32 = index39 + num8;
              for (let mut index40: i32 = num49; index40 <= num50; index40 += 1)
              {
                if (index40 > 0 & index40 <= self.game.Data.Round)
                {
                  num47 += 1;
                  num48 += numArray14[index38, index40];
                }
              }
              numArray17[index38, index39] =  Math.Round(Conversion.Int( num48 /  num47));
            }
          }
          let mut regimeCounter7: i32 = self.game.Data.RegimeCounter;
          for (let mut index41: i32 = 0; index41 <= regimeCounter7; index41 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index42: i32 = 1; index42 <= round; index42 += 1)
              numArray14[index41, index42] = numArray17[index41, index42];
          }
        }
        DrawMod.drawLine( graphics, 100, 100, 100, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.drawLine( graphics, 100, 700, 850, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
         let mut local3: &Graphics = &graphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.STATSGRADIENT);
         let mut local4: &Bitmap = &bitmap;
        let mut y: i32 = 699 - BitmapStore.Getheight(self.game.STATSGRADIENT);
        double r =  self.game.Data.RegimeObj[self.game.Data.Turn].Red /  byte.MaxValue - 1.0;
        double g =  self.game.Data.RegimeObj[self.game.Data.Turn].Green /  byte.MaxValue - 1.0;
        double b =  self.game.Data.RegimeObj[self.game.Data.Turn].Blue /  byte.MaxValue - 1.0;
        DrawMod.Draw( local3,  local4, 101, y,  r,  g,  b, 1f);
        DrawMod.DrawTextVic2( graphics, "Power Pts", self.game.VicFont2, 15, 85, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Round", self.game.VicFont2, 880, 720, self.game.VicColor2, self.game.VicColor2Shade);
        let mut regimeCounter8: i32 = self.game.Data.RegimeCounter;
        for (let mut index43: i32 = 0; index43 <= regimeCounter8; index43 += 1)
        {
          let mut x2: i32 = 0;
          let mut y2: i32 = 0;
          if ( self.game.Data.RuleVar[313] == 1.0 | index43 == self.game.Data.Turn | !self.game.Data.RegimeObj[index43].DipBlock & !(self.game.Data.RegimeObj[index43].AI & self.game.Data.RegimeObj[index43].Sleep) &&  self.game.Data.RuleVar[313] == 1.0 | flag | index43 == self.game.Data.Turn)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index44: i32 = 1; index44 <= round; index44 += 1)
            {
              if (!(index44 == self.game.Data.Round & index43 > self.game.Data.Turn))
              {
                let mut red: i32 = self.game.Data.RegimeObj[index43].Red;
                let mut green: i32 = self.game.Data.RegimeObj[index43].Green;
                let mut blue: i32 = self.game.Data.RegimeObj[index43].Blue;
                let mut x1: i32 =  Math.Round( (index44 - 1) /  num45 * 750.0 + 100.0);
                let mut y1: i32 =  Math.Round(700.0 -  numArray14[index43, index44] /  num43 * 600.0);
                if (x2 > 0)
                {
                  DrawMod.drawLine( graphics, x1, y1, x2, y2, red, green, blue,  byte.MaxValue, 2);
                  DrawMod.drawLine( graphics, x1, y1 + 1, x2, y2 + 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 2);
                }
                DrawMod.DrawBlock( graphics, x1 - 3, y1 - 3, 6, 6, red, green, blue,  byte.MaxValue);
                DrawMod.DrawRectangle( graphics, x1 - 3, y1 - 3, 6, 6,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                x2 = x1;
                y2 = y1;
                if (index44 == self.game.Data.Round)
                  DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index43].Name, vicFont4, x1 + 5, y1 - 15, self.game.VicColor2, self.game.VicColor2Shade);
              }
              else if (x2 != 0)
                DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index43].Name, vicFont4, x2 + 5, y2 - 15, self.game.VicColor2, self.game.VicColor2Shade);
            }
          }
        }
        let mut num51: i32 = 1;
        do
        {
          DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num43 * ( num51 / 10.0))), vicFont4, 20, 700 - 60 * num51, self.game.VicColor2, self.game.VicColor2Shade);
          if (self.game.Data.AlternateRound > -1)
          {
            DateTime dateTime = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1));
            let mut num52: i32 =  Math.Round(Conversion.Int( num45 * ( num51 / 10.0)));
            if (self.game.Data.AlternateRound == 31)
            {
              dateTime = dateTime.AddMonths((num52 - 1) * 1);
            }
            else
            {
              timeSpan = new TimeSpan((num52 - 1) * self.game.Data.AlternateRound, 0, 0, 0);
              dateTime = dateTime.Add(timeSpan);
            }
            tstring3: String = Strings.Left(self.game.HandyFunctionsObj.GetMonth(dateTime.Month), 3) + " " + Strings.Trim(Conversion.Str( dateTime.Day));
            DrawMod.DrawTextVic2( graphics, tstring3, vicFont4,  Math.Round(( num52 - 1.5) /  num45 * 750.0 + 100.0), 710, self.game.VicColor2, self.game.VicColor2Shade);
            tstring4: String = Strings.Trim(Conversion.Str( dateTime.Year));
            DrawMod.DrawTextVic2( graphics, tstring4, vicFont4,  Math.Round(( num52 - 1.5) /  num45 * 750.0 + 100.0), 725, self.game.VicColor2, self.game.VicColor2Shade);
          }
          else
          {
            let mut num53: i32 =  Math.Round(Conversion.Int( num45 * ( num51 / 10.0)));
            DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num45 * ( num51 / 10.0))), vicFont4,  Math.Round(( num53 - 1.5) /  num45 * 750.0 + 115.0), 720, self.game.VicColor2, self.game.VicColor2Shade);
          }
          num51 += 1;
        }
        while (num51 <= 10);
      }
      if (self.StatMode == 0)
      {
        let mut num54: i32 = 1000;
        let mut num55: i32 = 20;
        if (self.game.Data.Round > num55)
          num55 = self.game.Data.Round;
        let mut num56: i32 =  Math.Round( num55 + Conversion.Int( num55 * 0.2));
        numArray18: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        let mut regimeCounter9: i32 = self.game.Data.RegimeCounter;
        for (let mut index45: i32 = 0; index45 <= regimeCounter9; index45 += 1)
        {
          let mut round: i32 = self.game.Data.Round;
          for (let mut index46: i32 = 1; index46 <= round; index46 += 1)
          {
            let mut itemTypeCounter: i32 = self.game.Data.ItemTypeCounter;
            for (let mut index47: i32 = 0; index47 <= itemTypeCounter; index47 += 1)
            {
              numArray19: Vec<i32> = numArray18;
              numArray20: Vec<i32> = numArray19;
              let mut index48: i32 = index45;
              let mut index49: i32 = index48;
              let mut index50: i32 = index46;
              let mut index51: i32 = index50;
              let mut num57: i32 = numArray19[index48, index50] + self.game.Data.RegimeObj[index45].SProd[index47, index46] * self.game.Data.ItemTypeObj[index47].ProdWeight;
              numArray20[index49, index51] = num57;
              if (numArray18[index45, index46] > num54)
                num54 = numArray18[index45, index46];
            }
          }
        }
        numArray21: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        if (self.StatAggr > 0)
        {
          if (self.StatAggr == 1)
            num8 = 3;
          if (self.StatAggr == 2)
            num8 = 10;
          let mut regimeCounter10: i32 = self.game.Data.RegimeCounter;
          for (let mut index52: i32 = 0; index52 <= regimeCounter10; index52 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index53: i32 = 1; index53 <= round; index53 += 1)
            {
              let mut num58: i32 = 0;
              let mut num59: i32 = 0;
              let mut num60: i32 = index53 - num8;
              let mut num61: i32 = index53 + num8;
              for (let mut index54: i32 = num60; index54 <= num61; index54 += 1)
              {
                if (index54 > 0 & index54 <= self.game.Data.Round)
                {
                  num58 += 1;
                  num59 += numArray18[index52, index54];
                }
              }
              numArray21[index52, index53] =  Math.Round(Conversion.Int( num59 /  num58));
            }
          }
          let mut regimeCounter11: i32 = self.game.Data.RegimeCounter;
          for (let mut index55: i32 = 0; index55 <= regimeCounter11; index55 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index56: i32 = 1; index56 <= round; index56 += 1)
              numArray18[index55, index56] = numArray21[index55, index56];
          }
        }
        DrawMod.drawLine( graphics, 100, 100, 100, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.drawLine( graphics, 100, 700, 850, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
         let mut local5: &Graphics = &graphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.STATSGRADIENT);
         let mut local6: &Bitmap = &bitmap;
        let mut y: i32 = 699 - BitmapStore.Getheight(self.game.STATSGRADIENT);
        double r =  self.game.Data.RegimeObj[self.game.Data.Turn].Red /  byte.MaxValue - 1.0;
        double g =  self.game.Data.RegimeObj[self.game.Data.Turn].Green /  byte.MaxValue - 1.0;
        double b =  self.game.Data.RegimeObj[self.game.Data.Turn].Blue /  byte.MaxValue - 1.0;
        DrawMod.Draw( local5,  local6, 101, y,  r,  g,  b, 1f);
        DrawMod.DrawTextVic2( graphics, "Prod Pts", self.game.VicFont2, 15, 85, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Round", self.game.VicFont2, 880, 720, self.game.VicColor2, self.game.VicColor2Shade);
        let mut regimeCounter12: i32 = self.game.Data.RegimeCounter;
        for (let mut index57: i32 = 0; index57 <= regimeCounter12; index57 += 1)
        {
          let mut x2: i32 = 0;
          let mut y2: i32 = 0;
          if ( self.game.Data.RuleVar[313] == 1.0 | index57 == self.game.Data.Turn | !self.game.Data.RegimeObj[index57].DipBlock & !(self.game.Data.RegimeObj[index57].AI & self.game.Data.RegimeObj[index57].Sleep) &&  self.game.Data.RuleVar[313] == 1.0 | flag | index57 == self.game.Data.Turn)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index58: i32 = 1; index58 <= round; index58 += 1)
            {
              if (!(index58 == self.game.Data.Round & index57 > self.game.Data.Turn))
              {
                let mut red: i32 = self.game.Data.RegimeObj[index57].Red;
                let mut green: i32 = self.game.Data.RegimeObj[index57].Green;
                let mut blue: i32 = self.game.Data.RegimeObj[index57].Blue;
                let mut x1: i32 =  Math.Round( (index58 - 1) /  num56 * 750.0 + 100.0);
                let mut y1: i32 =  Math.Round(700.0 -  numArray18[index57, index58] /  num54 * 600.0);
                if (x2 > 0)
                {
                  DrawMod.drawLine( graphics, x1, y1, x2, y2, red, green, blue,  byte.MaxValue, 2);
                  DrawMod.drawLine( graphics, x1, y1 + 1, x2, y2 + 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 2);
                }
                DrawMod.DrawBlock( graphics, x1 - 3, y1 - 3, 6, 6, red, green, blue,  byte.MaxValue);
                DrawMod.DrawRectangle( graphics, x1 - 3, y1 - 3, 6, 6,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                x2 = x1;
                y2 = y1;
                if (index58 == self.game.Data.Round)
                  DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index57].Name, vicFont4, x1 + 5, y1 - 15, self.game.VicColor2, self.game.VicColor2Shade);
              }
              else if (x2 != 0)
                DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index57].Name, vicFont4, x2 + 5, y2 - 15, self.game.VicColor2, self.game.VicColor2Shade);
            }
          }
        }
        let mut num62: i32 = 1;
        do
        {
          DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num54 * ( num62 / 10.0))), vicFont4, 20, 700 - 60 * num62, self.game.VicColor2, self.game.VicColor2Shade);
          if (self.game.Data.AlternateRound > -1)
          {
            DateTime dateTime = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1));
            let mut num63: i32 =  Math.Round(Conversion.Int( num56 * ( num62 / 10.0)));
            if (self.game.Data.AlternateRound == 31)
            {
              dateTime = dateTime.AddMonths((num63 - 1) * 1);
            }
            else
            {
              timeSpan = new TimeSpan((num63 - 1) * self.game.Data.AlternateRound, 0, 0, 0);
              dateTime = dateTime.Add(timeSpan);
            }
            tstring5: String = Strings.Left(self.game.HandyFunctionsObj.GetMonth(dateTime.Month), 3) + " " + Strings.Trim(Conversion.Str( dateTime.Day));
            DrawMod.DrawTextVic2( graphics, tstring5, vicFont4,  Math.Round(( num63 - 1.5) /  num56 * 750.0 + 100.0), 710, self.game.VicColor2, self.game.VicColor2Shade);
            tstring6: String = Strings.Trim(Conversion.Str( dateTime.Year));
            DrawMod.DrawTextVic2( graphics, tstring6, vicFont4,  Math.Round(( num63 - 1.5) /  num56 * 750.0 + 100.0), 725, self.game.VicColor2, self.game.VicColor2Shade);
          }
          else
          {
            let mut num64: i32 =  Math.Round(Conversion.Int( num56 * ( num62 / 10.0)));
            DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num56 * ( num62 / 10.0))), vicFont4,  Math.Round(( num64 - 1.5) /  num56 * 750.0 + 115.0), 720, self.game.VicColor2, self.game.VicColor2Shade);
          }
          num62 += 1;
        }
        while (num62 <= 10);
      }
      if (self.StatMode == 1)
      {
        numArray22: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1 + 1];
        let mut num65: i32 = 100;
        let mut num66: i32 = 20;
        if (self.game.Data.Round > num66)
          num66 = self.game.Data.Round;
        let mut num67: i32 =  Math.Round( num66 + Conversion.Int( num66 * 0.2));
        let mut regimeCounter13: i32 = self.game.Data.RegimeCounter;
        for (let mut index59: i32 = 0; index59 <= regimeCounter13; index59 += 1)
        {
          let mut regimeCounter14: i32 = self.game.Data.RegimeCounter;
          for (let mut index60: i32 = 0; index60 <= regimeCounter14; index60 += 1)
          {
            if (index59 == index60 | self.game.Data.RegimeObj[index60].UberRegime == index59)
            {
              let mut num68: i32 = self.game.Data.Round + 1;
              for (let mut index61: i32 = 1; index61 <= num68; index61 += 1)
              {
                let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                for (let mut index62: i32 = 0; index62 <= sfTypeCounter; index62 += 1)
                {
                  numArray23: Vec<i32> = numArray22;
                  numArray24: Vec<i32> = numArray23;
                  let mut index63: i32 = index59;
                  let mut index64: i32 = index63;
                  let mut index65: i32 = index61;
                  let mut index66: i32 = index65;
                  let mut num69: i32 = numArray23[index63, index65] + self.game.Data.RegimeObj[index60].SLoss[index62, index61] * self.game.Data.SFTypeObj[index62].PowerPts;
                  numArray24[index64, index66] = num69;
                  if (numArray22[index59, index61] > num65)
                    num65 = numArray22[index59, index61];
                }
              }
            }
          }
        }
        numArray25: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        if (self.StatAggr > 0)
        {
          if (self.StatAggr == 1)
            num8 = 3;
          if (self.StatAggr == 2)
            num8 = 10;
          let mut regimeCounter15: i32 = self.game.Data.RegimeCounter;
          for (let mut index67: i32 = 0; index67 <= regimeCounter15; index67 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index68: i32 = 1; index68 <= round; index68 += 1)
            {
              let mut num70: i32 = 0;
              let mut num71: i32 = 0;
              let mut num72: i32 = index68 - num8;
              let mut num73: i32 = index68 + num8;
              for (let mut index69: i32 = num72; index69 <= num73; index69 += 1)
              {
                if (index69 > 0 & index69 <= self.game.Data.Round)
                {
                  num70 += 1;
                  num71 += numArray22[index67, index69];
                }
              }
              numArray25[index67, index68] =  Math.Round(Conversion.Int( num71 /  num70));
            }
          }
          let mut regimeCounter16: i32 = self.game.Data.RegimeCounter;
          for (let mut index70: i32 = 0; index70 <= regimeCounter16; index70 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index71: i32 = 1; index71 <= round; index71 += 1)
              numArray22[index70, index71] = numArray25[index70, index71];
          }
        }
        if (self.StatMode == 1)
        {
          DrawMod.drawLine( graphics, 100, 100, 100, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
          DrawMod.drawLine( graphics, 100, 700, 850, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
           let mut local7: &Graphics = &graphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(self.game.STATSGRADIENT);
           let mut local8: &Bitmap = &bitmap;
          let mut y: i32 = 699 - BitmapStore.Getheight(self.game.STATSGRADIENT);
          double r =  self.game.Data.RegimeObj[self.game.Data.Turn].Red /  byte.MaxValue - 1.0;
          double g =  self.game.Data.RegimeObj[self.game.Data.Turn].Green /  byte.MaxValue - 1.0;
          double b =  self.game.Data.RegimeObj[self.game.Data.Turn].Blue /  byte.MaxValue - 1.0;
          DrawMod.Draw( local7,  local8, 101, y,  r,  g,  b, 1f);
          DrawMod.DrawTextVic2( graphics, "Power Pts", self.game.VicFont2, 10, 85, self.game.VicColor2, self.game.VicColor2Shade);
          DrawMod.DrawTextVic2( graphics, "Round", self.game.VicFont2, 880, 720, self.game.VicColor2, self.game.VicColor2Shade);
          let mut regimeCounter17: i32 = self.game.Data.RegimeCounter;
          for (let mut index72: i32 = 0; index72 <= regimeCounter17; index72 += 1)
          {
            let mut x2: i32 = 0;
            let mut y2: i32 = 0;
            if ( self.game.Data.RuleVar[313] == 1.0 | index72 == self.game.Data.Turn | !self.game.Data.RegimeObj[index72].DipBlock & !(self.game.Data.RegimeObj[index72].AI & self.game.Data.RegimeObj[index72].Sleep) &&  self.game.Data.RuleVar[313] == 1.0 | flag | index72 == self.game.Data.Turn)
            {
              let mut round: i32 = self.game.Data.Round;
              for (let mut index73: i32 = 1; index73 <= round; index73 += 1)
              {
                if (!(index73 == self.game.Data.Round & index72 > self.game.Data.Turn))
                {
                  let mut red: i32 = self.game.Data.RegimeObj[index72].Red;
                  let mut green: i32 = self.game.Data.RegimeObj[index72].Green;
                  let mut blue: i32 = self.game.Data.RegimeObj[index72].Blue;
                  let mut x1: i32 =  Math.Round( (index73 - 1) /  num67 * 750.0 + 100.0);
                  let mut y1: i32 =  Math.Round(700.0 -  numArray22[index72, index73] /  num65 * 600.0);
                  if (x2 > 0)
                  {
                    DrawMod.drawLine( graphics, x1, y1, x2, y2, red, green, blue,  byte.MaxValue, 2);
                    DrawMod.drawLine( graphics, x1, y1 + 1, x2, y2 + 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 2);
                  }
                  DrawMod.DrawBlock( graphics, x1 - 3, y1 - 3, 6, 6, red, green, blue,  byte.MaxValue);
                  DrawMod.DrawRectangle( graphics, x1 - 3, y1 - 3, 6, 6,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                  x2 = x1;
                  y2 = y1;
                  if (index73 == self.game.Data.Round)
                    DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index72].Name, vicFont4, x1 + 5, y1 - 15, self.game.VicColor2, self.game.VicColor2Shade);
                }
                else if (x2 != 0)
                  DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index72].Name, vicFont4, x2 + 5, y2 - 15, self.game.VicColor2, self.game.VicColor2Shade);
              }
            }
          }
          let mut num74: i32 = 1;
          do
          {
            DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num65 * ( num74 / 10.0))), vicFont4, 20, 700 - 60 * num74, self.game.VicColor2, self.game.VicColor2Shade);
            if (self.game.Data.AlternateRound > -1)
            {
              DateTime dateTime = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1));
              let mut num75: i32 =  Math.Round(Conversion.Int( num67 * ( num74 / 10.0)));
              if (self.game.Data.AlternateRound == 31)
              {
                dateTime = dateTime.AddMonths((num75 - 1) * 1);
              }
              else
              {
                timeSpan = new TimeSpan((num75 - 1) * self.game.Data.AlternateRound, 0, 0, 0);
                dateTime = dateTime.Add(timeSpan);
              }
              tstring7: String = Strings.Left(self.game.HandyFunctionsObj.GetMonth(dateTime.Month), 3) + " " + Strings.Trim(Conversion.Str( dateTime.Day));
              DrawMod.DrawTextVic2( graphics, tstring7, vicFont4,  Math.Round(( num75 - 1.5) /  num67 * 750.0 + 100.0), 710, self.game.VicColor2, self.game.VicColor2Shade);
              tstring8: String = Strings.Trim(Conversion.Str( dateTime.Year));
              DrawMod.DrawTextVic2( graphics, tstring8, vicFont4,  Math.Round(( num75 - 1.5) /  num67 * 750.0 + 100.0), 725, self.game.VicColor2, self.game.VicColor2Shade);
            }
            else
            {
              let mut num76: i32 =  Math.Round(Conversion.Int( num67 * ( num74 / 10.0)));
              DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num67 * ( num74 / 10.0))), vicFont4,  Math.Round(( num76 - 1.5) /  num67 * 750.0 + 115.0), 720, self.game.VicColor2, self.game.VicColor2Shade);
            }
            num74 += 1;
          }
          while (num74 <= 10);
        }
      }
      if (self.StatMode == 2)
      {
        let mut num77: i32 = 100;
        let mut num78: i32 = 20;
        if (self.game.Data.Round > num78)
          num78 = self.game.Data.Round;
        let mut num79: i32 =  Math.Round( num78 + Conversion.Int( num78 * 0.2));
        numArray26: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1 + 1];
        let mut regimeCounter18: i32 = self.game.Data.RegimeCounter;
        for (let mut index74: i32 = 0; index74 <= regimeCounter18; index74 += 1)
        {
          let mut regimeCounter19: i32 = self.game.Data.RegimeCounter;
          for (let mut index75: i32 = 0; index75 <= regimeCounter19; index75 += 1)
          {
            if (index74 == index75 | self.game.Data.RegimeObj[index75].UberRegime == index74)
            {
              let mut num80: i32 = self.game.Data.Round + 1;
              for (let mut index76: i32 = 1; index76 <= num80; index76 += 1)
              {
                let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                for (let mut index77: i32 = 0; index77 <= sfTypeCounter; index77 += 1)
                {
                  numArray27: Vec<i32> = numArray26;
                  numArray28: Vec<i32> = numArray27;
                  let mut index78: i32 = index74;
                  let mut index79: i32 = index78;
                  let mut index80: i32 = index76;
                  let mut index81: i32 = index80;
                  let mut num81: i32 = numArray27[index78, index80] + self.game.Data.RegimeObj[index75].SKills[index77, index76] * self.game.Data.SFTypeObj[index77].PowerPts;
                  numArray28[index79, index81] = num81;
                  if (numArray26[index74, index76] > num77)
                    num77 = numArray26[index74, index76];
                }
              }
            }
          }
        }
        numArray29: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.Round + 1];
        if (self.StatAggr > 0)
        {
          if (self.StatAggr == 1)
            num8 = 3;
          if (self.StatAggr == 2)
            num8 = 10;
          let mut regimeCounter20: i32 = self.game.Data.RegimeCounter;
          for (let mut index82: i32 = 0; index82 <= regimeCounter20; index82 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index83: i32 = 1; index83 <= round; index83 += 1)
            {
              let mut num82: i32 = 0;
              let mut num83: i32 = 0;
              let mut num84: i32 = index83 - num8;
              let mut num85: i32 = index83 + num8;
              for (let mut index84: i32 = num84; index84 <= num85; index84 += 1)
              {
                if (index84 > 0 & index84 <= self.game.Data.Round)
                {
                  num82 += 1;
                  num83 += numArray26[index82, index84];
                }
              }
              numArray29[index82, index83] =  Math.Round(Conversion.Int( num83 /  num82));
            }
          }
          let mut regimeCounter21: i32 = self.game.Data.RegimeCounter;
          for (let mut index85: i32 = 0; index85 <= regimeCounter21; index85 += 1)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index86: i32 = 1; index86 <= round; index86 += 1)
              numArray26[index85, index86] = numArray29[index85, index86];
          }
        }
        DrawMod.drawLine( graphics, 100, 100, 100, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        DrawMod.drawLine( graphics, 100, 700, 850, 700,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
         let mut local9: &Graphics = &graphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.STATSGRADIENT);
         let mut local10: &Bitmap = &bitmap;
        let mut y: i32 = 699 - BitmapStore.Getheight(self.game.STATSGRADIENT);
        double r =  self.game.Data.RegimeObj[self.game.Data.Turn].Red /  byte.MaxValue - 1.0;
        double g =  self.game.Data.RegimeObj[self.game.Data.Turn].Green /  byte.MaxValue - 1.0;
        double b =  self.game.Data.RegimeObj[self.game.Data.Turn].Blue /  byte.MaxValue - 1.0;
        DrawMod.Draw( local9,  local10, 101, y,  r,  g,  b, 1f);
        DrawMod.DrawTextVic2( graphics, "Power Pts", self.game.VicFont2, 10, 85, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Round", self.game.VicFont2, 880, 720, self.game.VicColor2, self.game.VicColor2Shade);
        let mut regimeCounter22: i32 = self.game.Data.RegimeCounter;
        for (let mut index87: i32 = 0; index87 <= regimeCounter22; index87 += 1)
        {
          let mut x2: i32 = 0;
          let mut y2: i32 = 0;
          if ( self.game.Data.RuleVar[313] == 1.0 | index87 == self.game.Data.Turn | !self.game.Data.RegimeObj[index87].DipBlock & !(self.game.Data.RegimeObj[index87].AI & self.game.Data.RegimeObj[index87].Sleep) &&  self.game.Data.RuleVar[313] == 1.0 | flag | index87 == self.game.Data.Turn)
          {
            let mut round: i32 = self.game.Data.Round;
            for (let mut index88: i32 = 1; index88 <= round; index88 += 1)
            {
              if (!(index88 == self.game.Data.Round & index87 > self.game.Data.Turn))
              {
                let mut red: i32 = self.game.Data.RegimeObj[index87].Red;
                let mut green: i32 = self.game.Data.RegimeObj[index87].Green;
                let mut blue: i32 = self.game.Data.RegimeObj[index87].Blue;
                let mut x1: i32 =  Math.Round( (index88 - 1) /  num79 * 750.0 + 100.0);
                let mut y1: i32 =  Math.Round(700.0 -  numArray26[index87, index88] /  num77 * 600.0);
                if (x2 > 0)
                {
                  DrawMod.drawLine( graphics, x1, y1, x2, y2, red, green, blue,  byte.MaxValue, 2);
                  DrawMod.drawLine( graphics, x1, y1 + 1, x2, y2 + 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 2);
                }
                DrawMod.DrawBlock( graphics, x1 - 3, y1 - 3, 6, 6, red, green, blue,  byte.MaxValue);
                DrawMod.DrawRectangle( graphics, x1 - 3, y1 - 3, 6, 6,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                x2 = x1;
                y2 = y1;
                if (index88 == self.game.Data.Round)
                  DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index87].Name, vicFont4, x1 + 5, y1 - 15, self.game.VicColor2, self.game.VicColor2Shade);
              }
              else if (x2 != 0)
                DrawMod.DrawTextVic2( graphics, self.game.Data.RegimeObj[index87].Name, vicFont4, x2 + 5, y2 - 15, self.game.VicColor2, self.game.VicColor2Shade);
            }
          }
        }
        let mut num86: i32 = 1;
        do
        {
          DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num77 * ( num86 / 10.0))), vicFont4, 20, 700 - 60 * num86, self.game.VicColor2, self.game.VicColor2Shade);
          if (self.game.Data.AlternateRound > -1)
          {
            DateTime dateTime = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1));
            let mut num87: i32 =  Math.Round(Conversion.Int( num79 * ( num86 / 10.0)));
            if (self.game.Data.AlternateRound == 31)
            {
              dateTime = dateTime.AddMonths((num87 - 1) * 1);
            }
            else
            {
              timeSpan = new TimeSpan((num87 - 1) * self.game.Data.AlternateRound, 0, 0, 0);
              dateTime = dateTime.Add(timeSpan);
            }
            tstring9: String = Strings.Left(self.game.HandyFunctionsObj.GetMonth(dateTime.Month), 3) + " " + Strings.Trim(Conversion.Str( dateTime.Day));
            DrawMod.DrawTextVic2( graphics, tstring9, vicFont4,  Math.Round(( num87 - 1.5) /  num79 * 750.0 + 100.0), 710, self.game.VicColor2, self.game.VicColor2Shade);
            tstring10: String = Strings.Trim(Conversion.Str( dateTime.Year));
            DrawMod.DrawTextVic2( graphics, tstring10, vicFont4,  Math.Round(( num87 - 1.5) /  num79 * 750.0 + 100.0), 725, self.game.VicColor2, self.game.VicColor2Shade);
          }
          else
          {
            let mut num88: i32 =  Math.Round(Conversion.Int( num79 * ( num86 / 10.0)));
            DrawMod.DrawTextVic2( graphics, Conversion.Str( Conversion.Int( num79 * ( num86 / 10.0))), vicFont4,  Math.Round(( num88 - 1.5) /  num79 * 750.0 + 115.0), 720, self.game.VicColor2, self.game.VicColor2Shade);
          }
          num86 += 1;
        }
        while (num86 <= 10);
      }
      if (self.StatMode == 3)
      {
        numArray30: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.SFTypeCounter + 1];
        numArray31: Vec<i32> = new int[self.game.Data.RegimeCounter + 1, self.game.Data.SFTypeCounter + 1];
        let mut regimeCounter23: i32 = self.game.Data.RegimeCounter;
        for (let mut index89: i32 = 0; index89 <= regimeCounter23; index89 += 1)
        {
          let mut regimeCounter24: i32 = self.game.Data.RegimeCounter;
          for (let mut index90: i32 = 0; index90 <= regimeCounter24; index90 += 1)
          {
            if (index89 == index90 | self.game.Data.RegimeObj[index90].UberRegime == index89)
            {
              let mut num89: i32 = self.game.Data.Round + 1;
              for (let mut index91: i32 = 1; index91 <= num89; index91 += 1)
              {
                let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                for (let mut index92: i32 = 0; index92 <= sfTypeCounter; index92 += 1)
                {
                  numArray32: Vec<i32> = numArray30;
                  numArray33: Vec<i32> = numArray32;
                  let mut index93: i32 = index89;
                  let mut index94: i32 = index93;
                  let mut index95: i32 = index92;
                  let mut index96: i32 = index95;
                  let mut num90: i32 = numArray32[index93, index95] + self.game.Data.RegimeObj[index90].SKills[index92, index91] * Math.Max(1, self.game.Data.SFTypeObj[index92].Ratio);
                  numArray33[index94, index96] = num90;
                  numArray34: Vec<i32> = numArray31;
                  numArray35: Vec<i32> = numArray34;
                  let mut index97: i32 = index89;
                  let mut index98: i32 = index97;
                  let mut index99: i32 = index92;
                  let mut index100: i32 = index99;
                  let mut num91: i32 = numArray34[index97, index99] + self.game.Data.RegimeObj[index90].SLoss[index92, index91] * Math.Max(1, self.game.Data.SFTypeObj[index92].Ratio);
                  numArray35[index98, index100] = num91;
                }
              }
            }
          }
        }
        self.OptionsListObj = ATListClass::new();
        if (self.detailnr > self.game.Data.RegimeCounter)
          self.detailnr = -1;
        let mut tlistselect: i32 = -1;
        let mut num92: i32 = -1;
        if (self.game.Data.RegimeCounter > -1)
        {
          let mut regimeCounter25: i32 = self.game.Data.RegimeCounter;
          for (let mut tdata: i32 = 0; tdata <= regimeCounter25; tdata += 1)
          {
            if ( self.game.Data.RuleVar[313] == 1.0)
            {
              num92 += 1;
              if (self.detailnr == -1)
                self.detailnr = tdata;
              if (self.detailnr == tdata)
                tlistselect = num92;
              self.OptionsListObj.add(self.game.Data.RegimeObj[tdata].Name, tdata);
            }
            else if (self.game.Data.Winner > -1 | tdata == self.game.Data.Turn | !self.game.Data.RegimeObj[tdata].DipBlock & !(self.game.Data.RegimeObj[tdata].AI & self.game.Data.RegimeObj[tdata].Sleep) && flag | tdata == self.game.Data.Turn | self.game.Data.RegimeObj[tdata].UberRegime == self.game.Data.Turn)
            {
              num92 += 1;
              if (self.detailnr == -1)
                self.detailnr = tdata;
              if (self.detailnr == tdata)
                tlistselect = num92;
              self.OptionsListObj.add(self.game.Data.RegimeObj[tdata].Name, tdata);
            }
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            tsubpart1 =  new ATListSubPartClass(self.OptionsListObj, 16, 250, tlistselect, self.game, tHeader: "Regimes", tbackbitmap: ( self.OwnBitmap), bbx: 40, bby: 100);
            self.OptionsListId = self.AddSubPart( tsubpart1, 40, 100, 250, 304, 0);
          }
        }
        self.OptionsList2Obj = ATListClass::new();
        if (self.game.Data.RegimeCounter > -1)
        {
          if (self.detailnr > -1)
          {
            let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
            for (let mut tdata: i32 = 0; tdata <= sfTypeCounter; tdata += 1)
            {
              if (numArray30[self.detailnr, tdata] > 0)
                self.OptionsList2Obj.add(self.game.Data.SFTypeObj[tdata].Name, tdata, Conversion.Str( numArray30[self.detailnr, tdata]));
            }
          }
          if (self.OptionsList2Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, -1);
            self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
          }
          else
          {
            tsubpart1 =  new ATListSubPartClass(self.OptionsList2Obj, 16, 250, -1, self.game, tHeader: "Kills", tShowPair: true, tValueWidth: 75, tbackbitmap: ( self.OwnBitmap), bbx: 330, bby: 100);
            self.OptionsList2Id = self.AddSubPart( tsubpart1, 330, 100, 250, 304, 0);
          }
        }
        self.OptionsList3Obj = ATListClass::new();
        if (self.game.Data.RegimeCounter > -1)
        {
          if (self.detailnr > -1)
          {
            let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
            for (let mut tdata: i32 = 0; tdata <= sfTypeCounter; tdata += 1)
            {
              if (numArray31[self.detailnr, tdata] > 0)
                self.OptionsList3Obj.add(self.game.Data.SFTypeObj[tdata].Name, tdata, Conversion.Str( numArray31[self.detailnr, tdata]));
            }
          }
          if (self.OptionsList3Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList3Id)].Refresh(self.OptionsList3Obj, -1);
            self.SubPartFlag[self.SubpartNr(self.OptionsList3Id)] = true;
          }
          else
          {
            tsubpart1 =  new ATListSubPartClass(self.OptionsList3Obj, 16, 250, -1, self.game, tHeader: "Losses", tShowPair: true, tValueWidth: 75, tbackbitmap: ( self.OwnBitmap), bbx: 620, bby: 100);
            self.OptionsList3Id = self.AddSubPart( tsubpart1, 620, 100, 250, 304, 0);
          }
        }
        if (self.detailnr == self.game.Data.Turn)
        {
          self.OptionsList9Obj = ATListClass::new();
          if (self.game.Data.RegimeCounter > -1)
          {
            let mut itemTypeCounter: i32 = self.game.Data.ItemTypeCounter;
            for (let mut tdata: i32 = 0; tdata <= itemTypeCounter; tdata += 1)
            {
              if (self.game.Data.RegimeObj[self.game.Data.Turn].SProd[tdata, self.game.Data.Round] > 0 | self.game.Data.RegimeObj[self.game.Data.Turn].SASProdLost[tdata] > 0)
              {
                str: String = self.game.Data.ItemTypeObj[tdata].Name;
                tvalue2: String = "-";
                if (Strings.Len(str) > 15)
                  str = Strings.Left(str, 15);
                tvalue: String = Strings.Trim(Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].SProd[tdata, self.game.Data.Round]));
                if (self.game.Data.ASOn && self.game.Data.RegimeObj[self.game.Data.Turn].SASProdLost[tdata] > 0)
                  tvalue2 = Strings.Trim(Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].SASProdLost[tdata]));
                self.OptionsList9Obj.add(str, tdata, tvalue, tvalue2);
              }
            }
            if (self.OptionsList9Id > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsList9Id)].Refresh(self.OptionsList9Obj, -1);
              tsubpart1 =  new ATListSubPartClass(self.OptionsList9Obj, 11, 250, -1, self.game, tHeader: "Produced Type                          Prod", tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tbackbitmap: ( self.OwnBitmap), bbx: 40, bby: 450);
              self.OptionsList9Id = self.AddSubPart( tsubpart1, 40, 450, 250, 224, 0);
            }
            else
            {
              tsubpart1 =  new ATListSubPartClass(self.OptionsList9Obj, 11, 250, -1, self.game, tHeader: "Produced Type                          Prod        AS Loss", tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tbackbitmap: ( self.OwnBitmap), bbx: 40, bby: 450);
              self.OptionsList9Id = self.AddSubPart( tsubpart1, 40, 450, 250, 224, 0);
            }
          }
          self.OptionsList4Obj = ATListClass::new();
          if (self.game.Data.RegimeCounter > -1)
          {
            if (self.detailnr > -1)
            {
              let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
              for (let mut tdata: i32 = 0; tdata <= sfTypeCounter; tdata += 1)
              {
                if (self.game.Data.RegimeObj[self.detailnr].SKills[tdata, 0] > 0)
                  self.OptionsList4Obj.add(self.game.Data.SFTypeObj[tdata].Name, tdata, Conversion.Str( (self.game.Data.SFTypeObj[tdata].Ratio * self.game.Data.RegimeObj[self.detailnr].SKills[tdata, 0])));
              }
            }
            if (self.OptionsList4Id > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsList4Id)].Refresh(self.OptionsList4Obj, -1);
              self.SubPartFlag[self.SubpartNr(self.OptionsList4Id)] = true;
            }
            else
            {
              tsubpart1 =  new ATListSubPartClass(self.OptionsList4Obj, 11, 250, -1, self.game, tHeader: "Kills in this turn", tShowPair: true, tValueWidth: 75, tbackbitmap: ( self.OwnBitmap), bbx: 330, bby: 450);
              self.OptionsList4Id = self.AddSubPart( tsubpart1, 330, 450, 250, 224, 0);
            }
          }
          self.OptionsList5Obj = ATListClass::new();
          if (self.game.Data.RegimeCounter > -1)
          {
            if (self.detailnr > -1)
            {
              let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
              for (let mut tdata: i32 = 0; tdata <= sfTypeCounter; tdata += 1)
              {
                if (self.game.Data.RegimeObj[self.detailnr].SLoss[tdata, 0] > 0)
                  self.OptionsList5Obj.add(self.game.Data.SFTypeObj[tdata].Name, tdata, Conversion.Str( (self.game.Data.SFTypeObj[tdata].Ratio * self.game.Data.RegimeObj[self.detailnr].SLoss[tdata, 0])));
              }
            }
            if (self.OptionsList5Id > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsList5Id)].Refresh(self.OptionsList5Obj, -1);
              self.SubPartFlag[self.SubpartNr(self.OptionsList5Id)] = true;
            }
            else
            {
              tsubpart1 =  new ATListSubPartClass(self.OptionsList5Obj, 11, 250, -1, self.game, tHeader: "Losses in this turn", tShowPair: true, tValueWidth: 75, tbackbitmap: ( self.OwnBitmap), bbx: 620, bby: 450);
              self.OptionsList5Id = self.AddSubPart( tsubpart1, 620, 450, 250, 224, 0);
            }
          }
        }
      }
      if (self.StatMode == 5)
      {
        self.OptionsList7Obj = ATListClass::new();
        self.OptionsList8Obj = ATListClass::new();
        let mut tlistselect: i32 = 0;
        self.OptionsList7Obj.add("Whole army", -1);
        if (self.detailnr < 0)
          tlistselect = 0;
        let mut num93: i32 = 0;
        let mut unitCounter11: i32 = self.game.Data.UnitCounter;
        for (let mut tdata: i32 = 0; tdata <= unitCounter11; tdata += 1)
        {
          if (self.game.Data.UnitObj[tdata].Regime > -1 && self.game.Data.UnitObj[tdata].Regime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[tdata].Regime].UberRegime == self.game.Data.Turn && self.game.Data.UnitObj[tdata].IsHQ & self.game.Data.UnitObj[tdata].PreDef == -1)
          {
            num93 += 1;
            self.OptionsList7Obj.add(self.game.Data.UnitObj[tdata].Name, tdata);
            if (self.detailnr == tdata)
              tlistselect = num93;
          }
        }
        if (self.OptionsList7Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList7Id)].Refresh(self.OptionsList7Obj, tlistselect);
          self.SubPartFlag[self.SubpartNr(self.OptionsList7Id)] = true;
        }
        else
        {
          tsubpart1 =  new ATListSubPartClass(self.OptionsList7Obj, 26, 350, tlistselect, self.game, tHeader: "Headquarters", tbackbitmap: ( self.OwnBitmap), bbx: 50, bby: 100);
          self.OptionsList7Id = self.AddSubPart( tsubpart1, 50, 100, 350, 464, 0);
        }
        object[] objArray1 = new object[self.game.Data.SFTypeCounter + 1];
        if (self.Abstr == 0)
        {
          let mut unitCounter12: i32 = self.game.Data.UnitCounter;
          for (let mut unr: i32 = 0; unr <= unitCounter12; unr += 1)
          {
            if (self.game.Data.UnitObj[unr].Regime > -1 && self.game.Data.UnitObj[unr].PreDef == -1 & (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[unr].Regime].UberRegime == self.game.Data.Turn))
            {
              if (self.detailnr == -1)
              {
                let mut sfCount: i32 = self.game.Data.UnitObj[unr].SFCount;
                for (let mut index101: i32 = 0; index101 <= sfCount; index101 += 1)
                {
                  let mut sf: i32 = self.game.Data.UnitObj[unr].SFList[index101];
                  object[] objArray2 = objArray1;
                  object[] objArray3 = objArray2;
                  let mut type: i32 = self.game.Data.SFObj[sf].Type;
                  let mut index102: i32 = type;
                  object obj = Operators.AddObject(objArray2[type],  self.game.Data.SFObj[sf].Qty);
                  objArray3[index102] = obj;
                }
              }
              else if (unr == self.detailnr | self.detailnr == -1 | !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].HQ == self.detailnr | self.game.HandyFunctionsObj.IsUnitInHQChain(unr, self.detailnr))
              {
                let mut sfCount: i32 = self.game.Data.UnitObj[unr].SFCount;
                for (let mut index103: i32 = 0; index103 <= sfCount; index103 += 1)
                {
                  let mut sf: i32 = self.game.Data.UnitObj[unr].SFList[index103];
                  object[] objArray4 = objArray1;
                  object[] objArray5 = objArray4;
                  let mut type: i32 = self.game.Data.SFObj[sf].Type;
                  let mut index104: i32 = type;
                  object obj = Operators.AddObject(objArray4[type],  self.game.Data.SFObj[sf].Qty);
                  objArray5[index104] = obj;
                }
              }
            }
          }
          let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= sfTypeCounter; tdata += 1)
          {
            if (Operators.ConditionalCompareObjectGreater(objArray1[tdata],  0, false))
              self.OptionsList8Obj.add(self.game.Data.SFTypeObj[tdata].Name, tdata, Strings.Trim(Conversion.Str(Operators.MultiplyObject(objArray1[tdata],  Math.Max(1, self.game.Data.SFTypeObj[tdata].Ratio)))));
          }
        }
        else if (self.Abstr == 1)
        {
          object[] objArray6 = new object[101];
          let mut unitCounter13: i32 = self.game.Data.UnitCounter;
          for (let mut unr: i32 = 0; unr <= unitCounter13; unr += 1)
          {
            if (self.game.Data.UnitObj[unr].Regime > -1 && self.game.Data.UnitObj[unr].PreDef == -1 & (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[unr].Regime].UberRegime == self.game.Data.Turn))
            {
              if (self.detailnr == -1)
              {
                let mut sfCount: i32 = self.game.Data.UnitObj[unr].SFCount;
                for (let mut index105: i32 = 0; index105 <= sfCount; index105 += 1)
                {
                  let mut sf: i32 = self.game.Data.UnitObj[unr].SFList[index105];
                  let mut unitGroup: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].UnitGroup;
                  if (unitGroup > -1 & unitGroup < 100)
                  {
                    object[] objArray7 = objArray6;
                    object[] objArray8 = objArray7;
                    let mut index106: i32 = unitGroup;
                    let mut index107: i32 = index106;
                    object obj = Operators.AddObject(objArray7[index106],  (self.game.Data.SFObj[sf].Qty * Math.Max(1, self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Ratio)));
                    objArray8[index107] = obj;
                  }
                }
              }
              else if (unr == self.detailnr | self.detailnr == -1 | !self.game.Data.UnitObj[unr].IsHQ & self.game.Data.UnitObj[unr].HQ == self.detailnr | self.game.HandyFunctionsObj.IsUnitInHQChain(unr, self.detailnr))
              {
                let mut sfCount: i32 = self.game.Data.UnitObj[unr].SFCount;
                for (let mut index108: i32 = 0; index108 <= sfCount; index108 += 1)
                {
                  let mut sf: i32 = self.game.Data.UnitObj[unr].SFList[index108];
                  let mut unitGroup: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].UnitGroup;
                  if (unitGroup > -1 & unitGroup < 100)
                  {
                    object[] objArray9 = objArray6;
                    object[] objArray10 = objArray9;
                    let mut index109: i32 = unitGroup;
                    let mut index110: i32 = index109;
                    object obj = Operators.AddObject(objArray9[index109],  (self.game.Data.SFObj[sf].Qty * Math.Max(1, self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Ratio)));
                    objArray10[index110] = obj;
                  }
                }
              }
            }
          }
          let mut tdata: i32 = 0;
          do
          {
            if (Operators.ConditionalCompareObjectGreater(objArray6[tdata],  0, false))
              self.OptionsList8Obj.add(self.game.Data.TempString[400 + tdata], tdata, Strings.Trim(Conversion.Str(RuntimeHelpers.GetObjectValue(objArray6[tdata]))));
            tdata += 1;
          }
          while (tdata <= 99);
        }
        if (self.OptionsList8Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList8Id)].Refresh(self.OptionsList8Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList8Id)] = true;
        }
        else
        {
          tsubpart1 =  new ATListSubPartClass(self.OptionsList8Obj, 26, 440, -1, self.game, tHeader: "Troops", tShowPair: true, tValueWidth: 100, tbackbitmap: ( self.OwnBitmap), bbx: 480, bby: 160);
          self.OptionsList8Id = self.AddSubPart( tsubpart1, 480, 160, 440, 464, 0);
        }
        DrawMod.DrawBlock( graphics, 480, 95, 440, 45,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
        if (self.Abstr == 0)
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 100);
          self.but4id = self.AddSubPart( tsubpart1, 500, 100, 32, 16, 1);
        }
        else
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 500, bby: 100);
          self.but4id = self.AddSubPart( tsubpart1, 500, 100, 32, 16, 1);
        }
        if (self.Abstr == 1)
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.OKBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 100);
          self.but5id = self.AddSubPart( tsubpart1, 700, 100, 32, 16, 1);
        }
        else
        {
          tsubpart1 =  new SteveButtonPartClass(self.game.CANCELBALL, tBackbitmap: ( self.OwnBitmap), bbx: 700, bby: 100);
          self.but5id = self.AddSubPart( tsubpart1, 700, 100, 32, 16, 1);
        }
        DrawMod.DrawTextVic2( graphics, "Specific", self.game.VicFont2, 550, 113, self.game.VicColor2, self.game.VicColor2Shade);
        DrawMod.DrawTextVic2( graphics, "Class", self.game.VicFont2, 750, 113, self.game.VicColor2, self.game.VicColor2Shade);
      }
      if (self.StatMode == 6)
      {
        self.OptionsList6Obj = ATListClass::new();
        let mut num94: i32 = 0;
        let mut Number1: i32 = 1;
        do
        {
          let mut num95: i32 = 0;
          let mut ruleCounter: i32 = self.game.Data.RuleCounter;
          for (let mut Number2: i32 = 0; Number2 <= ruleCounter; Number2 += 1)
          {
            if (self.game.Data.RuleGroup[Number2] == Number1 & self.game.Data.RuleString[Number2].Length > 2 & Operators.CompareString(self.game.Data.RuleString[Number2], "OBSOLETE", false) != 0)
            {
              if (num95 == 0 & num94 == 1)
                self.OptionsList6Obj.add("   ", -1, " ");
              if (num95 == 0)
              {
                num95 = 1;
                num94 = 1;
                self.OptionsList6Obj.add("RULEGROUP " + Strings.Trim(Conversion.Str( Number1)), -1);
              }
              str: String = Strings.Trim(Conversion.Str( Number2)) + ") " + self.game.Data.RuleString[Number2];
              if (str.Length > 75)
                str = Strings.Left(str, 73) + "...";
              self.OptionsList6Obj.add(str, -1, Strings.Trim(Conversion.Str( self.game.Data.RuleVar[Number2])));
            }
          }
          Number1 += 1;
        }
        while (Number1 <= 19);
        if (self.OptionsList6Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList6Id)].Refresh(self.OptionsList6Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList6Id)] = true;
        }
        else
        {
          tsubpart1 =  new ATListSubPartClass(self.OptionsList6Obj, 37, 830, -1, self.game, tHeader: "Rule Variables", tShowPair: true, tValueWidth: 250, tbackbitmap: ( self.OwnBitmap), bbx: 90, bby: 90);
          self.OptionsList6Id = self.AddSubPart( tsubpart1, 90, 90, 830, 640, 0);
        }
      }
      if (self.StatMode == 7)
      {
        if (self.OptionsList2Id > 0)
        {
          self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
        }
        else
        {
          description: String = self.game.Data.Description;
          str1: String = !self.game.Data.ASOn ? description + "\r\n\r\nAnti Supply is turned OFF" : description + "\r\n\r\nAnti Supply is turned ON";
          str2: String = !self.game.Data.PBEM ? str1 + "\r\n\r\nAnti PBEM Cheat is turned OFF" : str1 + "\r\n\r\nAnti PBEM Cheat is turned ON";
          tText: String = !self.game.Data.TerrorMode ? str2 + "\r\n\r\nTerror Mode is turned OFF" : str2 + "\r\n\r\nTerror Mode is turned ON";
          if (self.game.Data.VPWin > 0 | self.game.EventRelatedObj.CheckVP(self.game.Data.Turn, 0, 0, 0) > 0)
            tText = tText + "\r\n\r\nYou currently have " + Conversion.Str( self.game.EventRelatedObj.CheckVP(self.game.Data.Turn, 0, 0, 0)) + " Victory Points (VP) in possesion.";
          let mut index111: i32 = 0;
          do
          {
            if (self.game.Data.Variants[index111] > -1)
              tText = self.game.Data.GameSlot[self.game.Data.Variants[index111]] > 0 ? tText + "\r\n\r\n" + self.game.Data.GameSlotName[self.game.Data.Variants[index111]] + " is turned ON" : tText + "\r\n\r\n" + self.game.Data.GameSlotName[self.game.Data.Variants[index111]] + " is turned OFF";
            index111 += 1;
          }
          while (index111 <= 9);
          tsubpart1 =  new TextAreaClass(self.game, 840, 37, self.game.VicFont3, "Briefing + Start settings", true, tText, Color.White, tbackbitmap: ( self.OwnBitmap), bbx: 90, bby: 90);
          self.OptionsList2Id = self.AddSubPart( tsubpart1, 90, 90, 840, 640, 0);
        }
      }
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
      graphics = (Graphics) null;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
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
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut index2: i32 = 0;
            while (self.TabId[index2] <= 0 || self.SubPartID[index1] != self.TabId[index2])
            {
              index2 += 1;
              if (index2 > 12)
              {
                let mut num1: i32 = self.SubPartID[index1];
                if (num1 == self.but1id)
                {
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.but2id)
                {
                  self.SortStart -= 10;
                  if (0 > self.SortStart)
                    self.SortStart = 0;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.but3id)
                {
                  self.SortStart += 10;
                  if (self.SortStart > self.sortcount)
                    self.SortStart = self.sortcount;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.but4id)
                {
                  self.Abstr = 0;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.but5id)
                {
                  self.Abstr = 1;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.ModeButton0Id)
                {
                  self.StatAggr = 0;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.ModeButton1Id)
                {
                  self.StatAggr = 1;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.ModeButton2Id)
                {
                  self.StatAggr = 2;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsListId)
                {
                  let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  if (num2 > -1)
                    self.detailnr = num2;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList7Id)
                {
                  self.detailnr = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                num3: i32;
                if (num1 == self.OptionsList2Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList3Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList4Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList5Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList6Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList8Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsList9Id)
                {
                  num3 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.UpgradeId)
                {
                  self.AutoUpgrade(self.detailnr5);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 != self.GOid)
                  return windowReturnClass;
                self.game.EditObj.UnitSelected = self.detailnr5;
                let mut num4: i32 = 265;
                x1: i32;
                y1: i32;
                if (self.game.Data.UnitObj[self.detailnr5].X > -1)
                {
                  x1 = self.game.Data.UnitObj[self.detailnr5].X;
                  y1 = self.game.Data.UnitObj[self.detailnr5].Y;
                }
                else
                {
                  x1 = self.game.Data.UnitObj[self.game.Data.UnitObj[self.detailnr5].OnBoard].X;
                  y1 = self.game.Data.UnitObj[self.game.Data.UnitObj[self.detailnr5].OnBoard].Y;
                }
                self.game.CornerX =  Math.Round( x1 - ( self.game.ScreenWidth / 2.0 - 0.0) / 53.0);
                self.game.CornerY =  Math.Round( y1 - ( self.game.ScreenHeight / 2.0 -  num4) / 48.0);
                if (self.game.CornerX < 0)
                  self.game.CornerX = 0;
                if (self.game.CornerY < 0)
                  self.game.CornerY = 0;
                if (self.game.Data.Round == 0)
                  num4 += 100;
                let mut num5: i32 =  Math.Round( (self.game.ScreenWidth - 220 - 0) / 53.0);
                let mut num6: i32 =  Math.Round( (self.game.ScreenHeight - num4) / 48.0);
                let mut num7: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth - self.game.CornerX;
                let mut num8: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight - self.game.CornerY;
                if (num5 > num7)
                  self.game.CornerX = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth - num5 + 2;
                if (num6 > num8)
                  self.game.CornerY = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight - num6 + 2;
                if ((self.game.CornerX + 10) % 2 > 0)
                  this += 1.game.CornerX;
                if ((self.game.CornerY + 10) % 2 > 0)
                  this += 1.game.CornerY;
                self.game.SelectX = x1;
                self.game.SelectY = y1;
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            self.StatMode = index2;
            self.game.EditObj.LastStatWindow = self.StatMode;
            self.detailnr = -1;
            if (self.OptionsListId > 0)
            {
              self.RemoveSubPart(self.OptionsListId);
              self.OptionsListId = 0;
            }
            if (self.OptionsList2Id > 0)
            {
              self.RemoveSubPart(self.OptionsList2Id);
              self.OptionsList2Id = 0;
            }
            if (self.OptionsList3Id > 0)
            {
              self.RemoveSubPart(self.OptionsList3Id);
              self.OptionsList3Id = 0;
            }
            if (self.OptionsList4Id > 0)
            {
              self.RemoveSubPart(self.OptionsList4Id);
              self.OptionsList4Id = 0;
            }
            if (self.OptionsList5Id > 0)
            {
              self.RemoveSubPart(self.OptionsList5Id);
              self.OptionsList5Id = 0;
            }
            if (self.OptionsList6Id > 0)
            {
              self.RemoveSubPart(self.OptionsList6Id);
              self.OptionsList6Id = 0;
            }
            if (self.OptionsList7Id > 0)
            {
              self.RemoveSubPart(self.OptionsList7Id);
              self.OptionsList7Id = 0;
            }
            if (self.OptionsList8Id > 0)
            {
              self.RemoveSubPart(self.OptionsList8Id);
              self.OptionsList8Id = 0;
            }
            if (self.OptionsList9Id > 0)
            {
              self.RemoveSubPart(self.OptionsList9Id);
              self.OptionsList9Id = 0;
            }
            if (self.StatMode == 9 && !self.supplycalcdone)
            {
              self.game.FormRef.Cursor = Cursors.WaitCursor;
              self.CalculateSupply();
              self.game.FormRef.Cursor = Cursors.Default;
            }
            self.detailnr = -1;
            self.DoStuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        let mut mzcount: i32 = self.mzcount;
        for (let mut index: i32 = 0; index <= mzcount; index += 1)
        {
          if (x > self.mzx[index] & y > self.mzy[index] && x < self.mzx2[index] & y < self.mzy2[index])
          {
            self.detailnr5 = self.mznr[index];
            if (self.mzdetnr[index] == 1)
            {
              self.detailnr = self.mznr[index];
              self.detailnr2 = -1;
              self.detailnr3 = -1;
              self.detailnr3 = -1;
              self.detailnr4 = -1;
            }
            if (self.mzdetnr[index] == 2)
            {
              self.detailnr2 = self.mznr[index];
              self.detailnr3 = -1;
              self.detailnr4 = -1;
            }
            if (self.mzdetnr[index] == 3)
            {
              self.detailnr3 = self.mznr[index];
              self.detailnr4 = -1;
            }
            if (self.mzdetnr[index] == 4)
              self.detailnr4 = self.mznr[index];
            self.DoStuff();
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

    pub fn AutoUpgrade(hqused: i32)
    {
      object[] objArray = new object[3];
      num1: i32;
      if (self.game.HandyFunctionsObj.HowmanyHQsBelow(hqused) > 0)
        num1 =  Interaction.MsgBox( "Also auto upgrade units of subordinate HQs?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest");
      let mut unitCounter: i32 = self.game.Data.UnitCounter;
      Number1: i32;
      Number2: i32;
      Number3: i32;
      for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1 && self.game.Data.UnitObj[unr].HQ == hqused | unr == hqused | num1 == 6 & self.game.HandyFunctionsObj.HasUnitHQSomewhereUp(unr, hqused) && self.game.Data.UnitObj[unr].X > -1 & self.game.Data.UnitObj[hqused].X > -1 && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y].LandscapeType].IsSea)
        {
          for (let mut sfCount: i32 = self.game.Data.UnitObj[unr].SFCount; sfCount >= 0; sfCount += -1)
          {
            self.sfnr = self.game.Data.UnitObj[unr].SFList[sfCount];
            objArray[0] =  -1;
            objArray[1] =  -1;
            objArray[2] =  -1;
            if (self.sfnr > -1 && self.game.HandyFunctionsObj.CanUpgrade(self.sfnr, unr))
            {
              if (self.game.Data.UnitObj[unr].IsHQ)
              {
                objArray[0] =  unr;
                let mut hq1: i32 = self.game.Data.UnitObj[Conversions.ToInteger(objArray[0])].HQ;
                if (hq1 > -1 && self.game.Data.UnitObj[hq1].X > -1)
                {
                  self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[unr].Regime,  Math.Round( self.game.Data.RuleVar[99]), 99,  Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, self.game.Data.UnitObj[unr].Map, allowshoredrop: true);
                  if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq1].Map].Value[self.game.Data.UnitObj[hq1].X, self.game.Data.UnitObj[hq1].Y] <=  self.game.Data.RuleVar[53])
                  {
                    objArray[1] =  hq1;
                    let mut hq2: i32 = self.game.Data.UnitObj[Conversions.ToInteger(objArray[1])].HQ;
                    if (hq2 > -1 && self.game.Data.UnitObj[hq2].X > -1 &&  self.game.EditObj.TempValue[self.game.Data.UnitObj[hq2].Map].Value[self.game.Data.UnitObj[hq2].X, self.game.Data.UnitObj[hq2].Y] <=  self.game.Data.RuleVar[53])
                      objArray[2] =  hq2;
                  }
                }
              }
              else if (self.game.Data.UnitObj[unr].HQ > -1 && self.game.Data.UnitObj[self.game.Data.UnitObj[unr].HQ].X > -1)
              {
                self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[unr].Regime,  Math.Round( self.game.Data.RuleVar[99]), 99,  Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y, self.game.Data.UnitObj[unr].Map, allowshoredrop: true);
                let mut hq3: i32 = self.game.Data.UnitObj[unr].HQ;
                if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq3].Map].Value[self.game.Data.UnitObj[hq3].X, self.game.Data.UnitObj[hq3].Y] <=  self.game.Data.RuleVar[53])
                {
                  objArray[0] =  hq3;
                  let mut hq4: i32 = self.game.Data.UnitObj[Conversions.ToInteger(objArray[0])].HQ;
                  if (hq4 > -1 && self.game.Data.UnitObj[hq4].X > -1 &&  self.game.EditObj.TempValue[self.game.Data.UnitObj[hq4].Map].Value[self.game.Data.UnitObj[hq4].X, self.game.Data.UnitObj[hq4].Y] <=  self.game.Data.RuleVar[53])
                  {
                    objArray[1] =  hq4;
                    let mut hq5: i32 = self.game.Data.UnitObj[Conversions.ToInteger(objArray[1])].HQ;
                    if (hq5 > -1 && self.game.Data.UnitObj[hq5].X > -1 &&  self.game.EditObj.TempValue[self.game.Data.UnitObj[hq5].Map].Value[self.game.Data.UnitObj[hq5].X, self.game.Data.UnitObj[hq5].Y] <=  self.game.Data.RuleVar[53])
                      objArray[2] =  hq5;
                  }
                }
              }
              let mut qty1: i32 = self.game.Data.SFObj[self.sfnr].Qty;
              let mut num2: i32 = 0;
              if (Conversions.ToBoolean(Operators.AndObject( (qty1 > 0), Operators.CompareObjectGreater(objArray[0],  -1, false))))
              {
                let mut qty2: i32 = self.game.HandyFunctionsObj.CanUpgradeMax(self.sfnr, unr, Conversions.ToInteger(objArray[0]));
                if (qty2 > qty1)
                  qty2 = qty1;
                if (qty2 > 0)
                {
                  self.game.ProcessingObj.DoUpgrade(unr, self.sfnr, qty2, Conversions.ToInteger(objArray[0]));
                  qty1 -= qty2;
                  Number1 += qty2;
                  num2 = 1;
                }
              }
              if (Conversions.ToBoolean(Operators.AndObject( (qty1 > 0), Operators.CompareObjectGreater(objArray[1],  -1, false))))
              {
                let mut qty3: i32 = self.game.HandyFunctionsObj.CanUpgradeMax(self.sfnr, unr, Conversions.ToInteger(objArray[1]));
                if (qty3 > qty1)
                  qty3 = qty1;
                if (qty3 > 0)
                {
                  self.game.ProcessingObj.DoUpgrade(unr, self.sfnr, qty3, Conversions.ToInteger(objArray[1]));
                  qty1 -= qty3;
                  Number1 += qty3;
                  num2 = 1;
                }
              }
              if (Conversions.ToBoolean(Operators.AndObject( (qty1 > 0), Operators.CompareObjectGreater(objArray[2],  -1, false))))
              {
                let mut qty4: i32 = self.game.HandyFunctionsObj.CanUpgradeMax(self.sfnr, unr, Conversions.ToInteger(objArray[2]));
                if (qty4 > qty1)
                  qty4 = qty1;
                if (qty4 > 0)
                {
                  self.game.ProcessingObj.DoUpgrade(unr, self.sfnr, qty4, Conversions.ToInteger(objArray[2]));
                  qty1 -= qty4;
                  Number1 += qty4;
                  num2 = 1;
                }
              }
              if ((uint) num2 > 0U)
                Number2 += 1;
              if (qty1 > 0)
                Number3 += 1;
            }
          }
        }
      }
      if (Number2 > 0)
      {
        Prompt: String;
        if (num1 == 6)
          Prompt = "Auto upgraded " + Conversion.Str( Number1) + " troops in " + Conversion.Str( Number2) + " units under indirect and direct command of " + self.game.Data.UnitObj[hqused].Name + ".";
        else
          Prompt = "Auto upgraded " + Conversion.Str( Number1) + " troops in " + Conversion.Str( Number2) + " units under direct command of " + self.game.Data.UnitObj[hqused].Name + ".";
        if (Number3 > 0)
          Prompt = Prompt + "We did not have enough supplies in chain of command to upgrade troops in " + Conversion.Str( Number3) + " units.";
        let mut num3: i32 =  Interaction.MsgBox( Prompt, Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        Prompt: String = "No units have been upgraded. ";
        if (Number3 > 0)
          Prompt = Prompt + "We did not have enough supplies in chain of command to upgrade troops in " + Conversion.Str( Number3) + " units.";
        let mut num4: i32 =  Interaction.MsgBox( Prompt, Title: ( "Shadow Empire : Planetary Conquest"));
      }
    }

    pub fn CalculateSupply()
    {
      int[] numArray1 = new int[self.game.Data.UnitCounter + 1];
      int[] numArray2 = new int[self.game.Data.UnitCounter + 1];
      self.sortcount = -1;
      let mut num1: i32 = -1;
      let mut unitCounter1: i32 = self.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
      {
        numArray1[index] = -1;
        numArray2[index] = 0;
      }
      num2: i32;
      do
      {
        num2 = 0;
        num1 += 1;
        let mut unitCounter2: i32 = self.game.Data.UnitCounter;
        for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
        {
          if (self.game.Data.UnitObj[index].PreDef == -1 && (self.game.Data.RegimeObj[self.game.Data.UnitObj[index].Regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[index].Regime].UberRegime > -1 & self.game.Data.RegimeObj[self.game.Data.UnitObj[index].Regime].UberRegime == self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == self.game.Data.UnitObj[index].Regime | self.game.Data.Turn == self.game.Data.UnitObj[index].Regime) & self.game.Data.UnitObj[index].PreDef <= -1)
          {
            if (num1 == 0)
            {
              if (self.game.Data.UnitObj[index].HQ == -1)
              {
                numArray1[index] = 0;
                num2 = 1;
              }
            }
            else if (self.game.Data.UnitObj[index].HQ > -1 & numArray1[index] == -1 & self.game.Data.UnitObj[index].PreDef <= -1)
            {
              let mut hq: i32 = self.game.Data.UnitObj[index].HQ;
              if (numArray1[hq] == num1 - 1)
              {
                num2 = 1;
                numArray1[index] = num1;
              }
            }
          }
        }
      }
      while (num2 > 0);
      let mut num3: i32 = num1 - 1;
      if (num3 == -1)
        return;
      self.supplyneed1 = new int[self.game.Data.UnitCounter + 1];
      self.supplyneed2 = new int[self.game.Data.UnitCounter + 1];
      self.supplyneed3 = new int[self.game.Data.UnitCounter + 1];
      self.supplyneed4 = new int[self.game.Data.UnitCounter + 1];
      self.supplyout1 = new int[self.game.Data.UnitCounter + 1];
      self.supplyout2 = new int[self.game.Data.UnitCounter + 1];
      self.supplyout3 = new int[self.game.Data.UnitCounter + 1];
      self.supplyin1 = new int[self.game.Data.UnitCounter + 1];
      self.supplyin2 = new int[self.game.Data.UnitCounter + 1];
      self.supplyin3 = new int[self.game.Data.UnitCounter + 1];
      self.supplyin4 = new int[self.game.Data.UnitCounter + 1];
      for (let mut index1: i32 = num3; index1 >= 0; index1 += -1)
      {
        if (index1 == num3 &  self.game.Data.RuleVar[335] > 0.0)
        {
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut regnr: i32 = 0; regnr <= regimeCounter; regnr += 1)
          {
            if (regnr == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == regnr)
            {
              let mut num4: i32 = 0;
              do
              {
                if (self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 0.0f +  (num4 * 4)))] > 0 && self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 1f +  (num4 * 4)))] > 0 && self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 2f +  (num4 * 4)))] > -1 && self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 3f +  (num4 * 4)))] > 0)
                {
                  SimpleList supplyReceivingHq = self.game.HandyFunctionsObj.GetSupplyReceivingHQ(self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 0.0f +  (num4 * 4)))], self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 1f +  (num4 * 4)))], self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 2f +  (num4 * 4)))], regnr, self.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( (self.game.Data.RuleVar[335] + 3f +  (num4 * 4)))]);
                  if (supplyReceivingHq.Counter > -1)
                  {
                    let mut counter: i32 = supplyReceivingHq.Counter;
                    for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
                    {
                      int[] supplyin1 = self.supplyin1;
                      int[] numArray3 = supplyin1;
                      int[] id = supplyReceivingHq.Id;
                      int[] numArray4 = id;
                      let mut index3: i32 = index2;
                      let mut index4: i32 = index3;
                      let mut index5: i32 = numArray4[index4];
                      let mut num5: i32 = supplyin1[id[index3]] + supplyReceivingHq.Data1[index2];
                      numArray3[index5] = num5;
                    }
                  }
                }
                num4 += 1;
              }
              while (num4 <= 3);
            }
          }
        }
        let mut unitCounter3: i32 = self.game.Data.UnitCounter;
        for (let mut index6: i32 = 0; index6 <= unitCounter3; index6 += 1)
        {
          if (numArray1[index6] == index1 && self.game.Data.UnitObj[index6].X > -1 & self.game.Data.UnitObj[index6].PreDef == -1 && self.game.Data.RegimeObj[self.game.Data.UnitObj[index6].Regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[index6].Regime].UberRegime > -1 & self.game.Data.RegimeObj[self.game.Data.UnitObj[index6].Regime].UberRegime == self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == self.game.Data.UnitObj[index6].Regime | self.game.Data.Turn == self.game.Data.UnitObj[index6].Regime && self.game.Data.UnitObj[index6].IsHQ)
          {
            self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn,  Math.Round( self.game.Data.RuleVar[99]), 99,  Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[index6].X, self.game.Data.UnitObj[index6].Y, self.game.Data.UnitObj[index6].Map, allowshoredrop: true);
            let mut unitCounter4: i32 = self.game.Data.UnitCounter;
            num6: i32;
            float num7;
            for (let mut unr: i32 = 0; unr <= unitCounter4; unr += 1)
            {
              if ((unr == index6 | self.game.Data.UnitObj[unr].HQ == index6) & self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].X > -1)
              {
                num6 = 0;
                let mut num8: i32 = 0;
                let mut num9: i32 = 0;
                num7 = 1f;
                let mut sfCount: i32 = self.game.Data.UnitObj[unr].SFCount;
                for (let mut index7: i32 = 0; index7 <= sfCount; index7 += 1)
                {
                  let mut sf: i32 = self.game.Data.UnitObj[unr].SFList[index7];
                  num8 =  Math.Round( num8 +  self.game.Data.SFObj[sf].Qty * ( self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].BasicSupplyNeed * 1.5));
                  num9 += self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].SupplyCarry;
                }
                let mut num10: i32 = num9 + self.game.HandyFunctionsObj.UnitSupplyUse(unr, true);
                if (num8 + self.game.Data.UnitObj[unr].Supply > num10)
                  num8 = num10 - self.game.Data.UnitObj[unr].Supply;
                if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y].LandscapeType].IsSea)
                  num8 = 0;
                if (0 > num8)
                  num8 = 0;
                self.supplyneed1[unr] =  Math.Round( num8 * ( self.game.Data.UnitObj[unr].SOSupReqPercent / 100.0));
                let mut hq1: i32 = self.game.Data.UnitObj[unr].HQ;
                if (hq1 == index6 & !self.game.Data.UnitObj[unr].IsHQ)
                {
                  int[] supplyout1 = self.supplyout1;
                  int[] numArray5 = supplyout1;
                  let mut index8: i32 = hq1;
                  let mut index9: i32 = index8;
                  let mut num11: i32 = supplyout1[index8] + num8;
                  numArray5[index9] = num11;
                }
                if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[unr].Map].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] >  self.game.Data.RuleVar[3])
                  num8 = 0;
                else if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[unr].Map].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] >  self.game.Data.RuleVar[53])
                  num8 =  Math.Round( num8 * 0.25);
                else if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[unr].Map].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] >  self.game.Data.RuleVar[52])
                  num8 =  Math.Round( num8 * 0.5);
                else if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[unr].Map].Value[self.game.Data.UnitObj[unr].X, self.game.Data.UnitObj[unr].Y] >  self.game.Data.RuleVar[51])
                  num8 =  Math.Round( num8 * 0.75);
                self.supplyneed4[unr] = num8;
                let mut hq2: i32 = self.game.Data.UnitObj[unr].HQ;
                if (hq2 == index6 & !self.game.Data.UnitObj[unr].IsHQ)
                {
                  int[] supplyout2 = self.supplyout2;
                  int[] numArray6 = supplyout2;
                  let mut index10: i32 = hq2;
                  let mut index11: i32 = index10;
                  let mut num12: i32 = supplyout2[index10] + num8;
                  numArray6[index11] = num12;
                  self.supplyin3[unr] = num8;
                }
                self.supplyneed2[unr] = self.game.HandyFunctionsObj.UnitSupplyUse(unr, true);
                self.supplyneed3[unr] = self.game.HandyFunctionsObj.UnitSupplyStore(unr);
              }
            }
            num6 = 0;
            let mut locCounter: i32 = self.game.Data.LocCounter;
            for (let mut locnr: i32 = 0; locnr <= locCounter; locnr += 1)
            {
              let mut regime: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime;
              if (regime > -1 && regime == self.game.Data.Turn | self.game.Data.RegimeObj[regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == regime && self.game.Data.LocObj[locnr].HQ == index6)
              {
                let mut prodslot: i32 = 0;
                do
                {
                  if (self.game.Data.LocObj[locnr].Production[prodslot] > -1 && self.game.Data.ItemTypeObj[self.game.Data.LocObj[locnr].Production[prodslot]].IsSupply)
                  {
                    num6 = 0;
                    let mut num13: i32 =  self.game.EditObj.TempValue[self.game.Data.LocObj[locnr].Map].Value[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y] <=  self.game.Data.RuleVar[51] ?  Math.Round(Conversion.Int(self.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, locnr, true, false))) :  Math.Round(Conversion.Int(self.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, locnr, false, false)));
                    num7 = 1f;
                    if ( self.game.EditObj.TempValue[self.game.Data.LocObj[locnr].Map].Value[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y] >  self.game.Data.RuleVar[3])
                      num13 = 0;
                    else if ( self.game.EditObj.TempValue[self.game.Data.LocObj[locnr].Map].Value[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y] >  self.game.Data.RuleVar[53])
                      num13 =  Math.Round( num13 * 0.25);
                    else if ( self.game.EditObj.TempValue[self.game.Data.LocObj[locnr].Map].Value[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y] >  self.game.Data.RuleVar[52])
                      num13 =  Math.Round( num13 * 0.5);
                    else if ( self.game.EditObj.TempValue[self.game.Data.LocObj[locnr].Map].Value[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y] >  self.game.Data.RuleVar[51])
                      num13 =  Math.Round( num13 * 0.75);
                    if (num13 > 0)
                      num13 *= self.game.Data.ItemTypeObj[self.game.Data.LocObj[locnr].Production[prodslot]].Multiplier;
                    int[] supplyin1 = self.supplyin1;
                    int[] numArray7 = supplyin1;
                    let mut index12: i32 = index6;
                    let mut index13: i32 = index12;
                    let mut num14: i32 = supplyin1[index12] + num13;
                    numArray7[index13] = num14;
                  }
                  prodslot += 1;
                }
                while (prodslot <= 3);
              }
            }
            let mut hq: i32 = self.game.Data.UnitObj[index6].HQ;
            if (hq > -1)
            {
              if (self.game.Data.UnitObj[hq].X > -1)
              {
                let mut num15: i32 = self.supplyout1[index6] + self.supplyneed4[index6] - self.supplyin1[index6];
                let mut num16: i32 = self.game.Data.UnitObj[index6].Supply - self.game.Data.UnitObj[index6].Reserve;
                if (0 > num16)
                  num16 = 0;
                let mut num17: i32 = num15 - num16;
                self.supplyin2[index6] = num16;
                if (0 > num17)
                  num17 = 0;
                int[] supplyout1 = self.supplyout1;
                int[] numArray8 = supplyout1;
                let mut index14: i32 = hq;
                let mut index15: i32 = index14;
                let mut num18: i32 = supplyout1[index14] + num17;
                numArray8[index15] = num18;
                if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq].Map].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] >  self.game.Data.RuleVar[3])
                  num17 = 0;
                else if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq].Map].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] >  self.game.Data.RuleVar[53])
                  num17 =  Math.Round( num17 * 0.25);
                else if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq].Map].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] >  self.game.Data.RuleVar[52])
                  num17 =  Math.Round( num17 * 0.5);
                else if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq].Map].Value[self.game.Data.UnitObj[hq].X, self.game.Data.UnitObj[hq].Y] >  self.game.Data.RuleVar[51])
                  num17 =  Math.Round( num17 * 0.75);
                int[] supplyout2 = self.supplyout2;
                int[] numArray9 = supplyout2;
                let mut index16: i32 = hq;
                let mut index17: i32 = index16;
                let mut num19: i32 = supplyout2[index16] + num17;
                numArray9[index17] = num19;
                self.supplyin3[index6] = num17;
              }
              else
              {
                let mut num20: i32 = self.game.Data.UnitObj[index6].Supply - self.game.Data.UnitObj[index6].Reserve;
                if (0 > num20)
                  num20 = 0;
                self.supplyin2[index6] = num20;
              }
            }
            else
            {
              let mut num21: i32 = self.game.Data.UnitObj[index6].Supply - self.game.Data.UnitObj[index6].Reserve;
              if (0 > num21)
                num21 = 0;
              self.supplyin2[index6] = num21;
            }
          }
        }
      }
      let mut num22: i32 = num3;
      for (let mut index18: i32 = 0; index18 <= num22; index18 += 1)
      {
        let mut unitCounter5: i32 = self.game.Data.UnitCounter;
        for (let mut index19: i32 = 0; index19 <= unitCounter5; index19 += 1)
        {
          if (numArray1[index19] == index18 && self.game.Data.UnitObj[index19].X > -1 & self.game.Data.UnitObj[index19].PreDef == -1 & self.game.Data.UnitObj[index19].X > -1 && self.game.Data.RegimeObj[self.game.Data.UnitObj[index19].Regime].UberRegime == self.game.Data.Turn | self.game.Data.RegimeObj[self.game.Data.UnitObj[index19].Regime].UberRegime > -1 & self.game.Data.RegimeObj[self.game.Data.UnitObj[index19].Regime].UberRegime == self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime | self.game.Data.RegimeObj[self.game.Data.Turn].UberRegime == self.game.Data.UnitObj[index19].Regime | self.game.Data.Turn == self.game.Data.UnitObj[index19].Regime && self.game.Data.UnitObj[index19].IsHQ)
          {
            let mut num23: i32 = self.supplyneed4[index19];
            let mut unitCounter6: i32 = self.game.Data.UnitCounter;
            for (let mut index20: i32 = 0; index20 <= unitCounter6; index20 += 1)
            {
              if (self.game.Data.UnitObj[index20].HQ == index19 & self.game.Data.UnitObj[index20].PreDef == -1 & self.game.Data.UnitObj[index20].X > -1)
              {
                if (self.game.Data.UnitObj[index20].IsHQ)
                  num23 += self.supplyin3[index20];
                else
                  num23 += self.supplyneed4[index20];
              }
            }
            float num24 = num23 > 0 ?  (self.supplyin1[index19] + self.supplyin2[index19] + self.supplyin4[index19]) /  num23 : 0.0f;
            self.supplyout3[index19] = num23 <= self.supplyin1[index19] + self.supplyin2[index19] + self.supplyin4[index19] ? num23 - self.supplyneed4[index19] :  Math.Round( ( (self.supplyin1[index19] + self.supplyin2[index19] + self.supplyin4[index19]) -  self.supplyneed4[index19] * num24));
            if ( num24 > 1.0)
              num24 = 1f;
            let mut unitCounter7: i32 = self.game.Data.UnitCounter;
            for (let mut index21: i32 = 0; index21 <= unitCounter7; index21 += 1)
            {
              if (self.game.Data.UnitObj[index21].HQ == index19 & self.game.Data.UnitObj[index21].PreDef == -1 & self.game.Data.UnitObj[index21].X > -1)
                self.supplyin4[index21] = !self.game.Data.UnitObj[index21].IsHQ ?  Math.Round( (num24 *  self.supplyneed4[index21])) :  Math.Round( (num24 *  self.supplyin3[index21]));
            }
          }
        }
      }
      self.supplycalcdone = true;
    }
  }
}
