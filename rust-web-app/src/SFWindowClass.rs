// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SFWindowClass : WindowClass
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
     temptext20: i32;
     TempText21: i32;
     temptext22: i32;
     temptext23: i32;
     temptext24: i32;
     temptext25: i32;
     temptext26: i32;
     temptext27: i32;
     temptext28: i32;
     temptext29: i32;
     temptext30: i32;
     TempText31: i32;
     temptext32: i32;
     temptext33: i32;
     temptext34: i32;
     temptext35: i32;
     temptext36: i32;
     temptext37: i32;
     temptext38: i32;
     temptext39: i32;
     temptext40: i32;
     temptext41: i32;
     temptext42: i32;
     temptext43: i32;
     temptext44: i32;
     temptext45: i32;
     temptext46: i32;
     LogoListId: i32;
     but1id: i32;
     tab1id: i32;
     tab2id: i32;
     tab3id: i32;
     tab4id: i32;
     tab5id: i32;
     but1textid: i32;
     but1bid: i32;
     hqbut0: i32;
     hqbut1: i32;
     hqbut2: i32;
     but2id: i32;
     but2textid: i32;
     but3id: i32;
     but3textid: i32;
     but4id: i32;
     but4textid: i32;
     but5id: i32;
     but5textid: i32;
     but6id: i32;
     but6textid: i32;
     but7id: i32;
     quitid: i32;
     but7textid: i32;
     descid: i32;
     comparenr: i32;
     sliderid: i32;
     logolist2id: i32;
     logolist3id: i32;
     float tempBlink;
     unr: i32;
     sfnr: i32;
     sftyp: i32;
     detailnr: i32;
     detailnr2: i32;
     detailtype: i32;
     ammount: i32;
     bool hqreach;
     passenger: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
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
     combatListId: i32;
     ATListClass combatListObj;
     combatList2Id: i32;
     ATListClass combatList2Obj;
     StatTyp: i32;
     StatMode: i32;
     int[] ChainHq;
     HQselect: i32;
     infoid: i32;

    pub handleTimer: WindowReturnClass() => WindowReturnClass::new();

    pub fn DoRefresh()
    {
      self.comparenr = self.game.EditObj.SFCompare;
      if (self.descid > 0)
      {
        self.RemoveSubPart(self.descid);
        self.descid = 0;
      }
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
      if (self.combatListId > 0)
      {
        self.RemoveSubPart(self.combatListId);
        self.combatListId = 0;
      }
      if (self.combatList2Id > 0)
      {
        self.RemoveSubPart(self.combatList2Id);
        self.combatList2Id = 0;
      }
      self.DoStuff();
    }

    pub SFWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      self.ChainHq = new int[3];
      self.tempBlink = 0.0f;
      self.sfnr = -1;
      self.comparenr = -1;
      self.game.EditObj.SFCompare = -1;
      self.game.EditObj.CurrentDescript = "";
      self.unr = -1;
      if (self.game.EditObj.SFSelected > -1)
      {
        if (self.game.EditObj.SFSelected > self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFCount)
        {
          self.passenger = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].PassengerList[self.game.EditObj.SFSelected - (1 + self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFCount)];
          self.sfnr = -1;
          self.sftyp = -1;
        }
        else
        {
          self.sfnr = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFList[self.game.EditObj.SFSelected];
          self.sftyp = self.game.Data.SFObj[self.sfnr].Type;
          self.unr = self.game.EditObj.UnitSelected;
          self.passenger = -1;
        }
      }
      else
      {
        self.sftyp = self.game.EditObj.SFTypeSelected;
        self.sfnr = -1;
      }
      self.ChainHq[0] = -1;
      self.HQselect = -1;
      self.ChainHq[1] = -1;
      self.ChainHq[2] = -1;
      if (self.sfnr > -1 && self.game.HandyFunctionsObj.CanUpgrade(self.sfnr, self.unr))
      {
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
        {
          self.ChainHq[0] = self.game.EditObj.UnitSelected;
          self.HQselect = self.game.EditObj.UnitSelected;
          let mut hq1: i32 = self.game.Data.UnitObj[self.ChainHq[0]].HQ;
          if (hq1 > -1 && self.game.Data.UnitObj[hq1].X > -1)
          {
            self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime,  Math.Round( self.game.Data.RuleVar[99]), 99,  Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y, self.game.Data.UnitObj[self.unr].Map, allowshoredrop: true);
            if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq1].Map].Value[self.game.Data.UnitObj[hq1].X, self.game.Data.UnitObj[hq1].Y] <=  self.game.Data.RuleVar[53])
            {
              self.ChainHq[1] = hq1;
              let mut hq2: i32 = self.game.Data.UnitObj[self.ChainHq[1]].HQ;
              if (hq2 > -1 && self.game.Data.UnitObj[hq2].X > -1 &&  self.game.EditObj.TempValue[self.game.Data.UnitObj[hq2].Map].Value[self.game.Data.UnitObj[hq2].X, self.game.Data.UnitObj[hq2].Y] <=  self.game.Data.RuleVar[53])
                self.ChainHq[2] = hq2;
            }
          }
        }
        else if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ > -1 && self.game.Data.UnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ].X > -1)
        {
          self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime,  Math.Round( self.game.Data.RuleVar[99]), 99,  Math.Round( self.game.Data.RuleVar[3]), self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y, self.game.Data.UnitObj[self.unr].Map, allowshoredrop: true);
          let mut hq3: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ;
          if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[hq3].Map].Value[self.game.Data.UnitObj[hq3].X, self.game.Data.UnitObj[hq3].Y] <=  self.game.Data.RuleVar[53])
          {
            self.ChainHq[0] = hq3;
            self.HQselect = hq3;
            let mut hq4: i32 = self.game.Data.UnitObj[self.ChainHq[0]].HQ;
            if (hq4 > -1 && self.game.Data.UnitObj[hq4].X > -1 &&  self.game.EditObj.TempValue[self.game.Data.UnitObj[hq4].Map].Value[self.game.Data.UnitObj[hq4].X, self.game.Data.UnitObj[hq4].Y] <=  self.game.Data.RuleVar[53])
            {
              self.ChainHq[1] = hq4;
              let mut hq5: i32 = self.game.Data.UnitObj[self.ChainHq[1]].HQ;
              if (hq5 > -1 && self.game.Data.UnitObj[hq5].X > -1 &&  self.game.EditObj.TempValue[self.game.Data.UnitObj[hq5].Map].Value[self.game.Data.UnitObj[hq5].X, self.game.Data.UnitObj[hq5].Y] <=  self.game.Data.RuleVar[53])
                self.ChainHq[2] = hq5;
            }
          }
        }
      }
      self.StatTyp = 1;
      self.StatMode = 0;
      self.detailnr = -1;
      self.detailnr2 = 0;
      self.DoStuff();
    }

    pub fn DoStuff()
    {
      self.NewBackGroundAndClearAll(1024, 768, self.game.BACKGROUND2MARC);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      if (self.TempText1 > 0)
        self.RemoveSubPart(self.TempText1);
      if (self.temptext2 > 0)
        self.RemoveSubPart(self.temptext2);
      if (self.temptext3 > 0)
        self.RemoveSubPart(self.temptext3);
      if (self.temptext4 > 0)
        self.RemoveSubPart(self.temptext4);
      if (self.temptext5 > 0)
        self.RemoveSubPart(self.temptext5);
      if (self.temptext6 > 0)
        self.RemoveSubPart(self.temptext6);
      if (self.temptext7 > 0)
        self.RemoveSubPart(self.temptext7);
      if (self.temptext8 > 0)
        self.RemoveSubPart(self.temptext8);
      if (self.temptext9 > 0)
        self.RemoveSubPart(self.temptext9);
      if (self.temptext10 > 0)
        self.RemoveSubPart(self.temptext10);
      if (self.TempText11 > 0)
        self.RemoveSubPart(self.TempText11);
      if (self.temptext12 > 0)
        self.RemoveSubPart(self.temptext12);
      if (self.temptext13 > 0)
        self.RemoveSubPart(self.temptext13);
      if (self.temptext14 > 0)
        self.RemoveSubPart(self.temptext14);
      if (self.temptext15 > 0)
        self.RemoveSubPart(self.temptext15);
      if (self.temptext16 > 0)
        self.RemoveSubPart(self.temptext16);
      if (self.temptext17 > 0)
        self.RemoveSubPart(self.temptext17);
      if (self.temptext18 > 0)
        self.RemoveSubPart(self.temptext18);
      if (self.temptext19 > 0)
        self.RemoveSubPart(self.temptext19);
      if (self.temptext20 > 0)
        self.RemoveSubPart(self.temptext20);
      if (self.TempText21 > 0)
        self.RemoveSubPart(self.TempText21);
      if (self.temptext22 > 0)
        self.RemoveSubPart(self.temptext22);
      if (self.temptext23 > 0)
        self.RemoveSubPart(self.temptext23);
      if (self.temptext24 > 0)
        self.RemoveSubPart(self.temptext24);
      if (self.temptext25 > 0)
        self.RemoveSubPart(self.temptext25);
      if (self.temptext26 > 0)
        self.RemoveSubPart(self.temptext26);
      if (self.temptext27 > 0)
        self.RemoveSubPart(self.temptext27);
      if (self.temptext28 > 0)
        self.RemoveSubPart(self.temptext28);
      if (self.temptext29 > 0)
        self.RemoveSubPart(self.temptext29);
      if (self.temptext30 > 0)
        self.RemoveSubPart(self.temptext30);
      if (self.TempText31 > 0)
        self.RemoveSubPart(self.TempText31);
      if (self.temptext32 > 0)
        self.RemoveSubPart(self.temptext32);
      if (self.temptext33 > 0)
        self.RemoveSubPart(self.temptext33);
      if (self.temptext34 > 0)
        self.RemoveSubPart(self.temptext34);
      if (self.temptext35 > 0)
        self.RemoveSubPart(self.temptext35);
      if (self.temptext36 > 0)
        self.RemoveSubPart(self.temptext36);
      if (self.temptext37 > 0)
        self.RemoveSubPart(self.temptext37);
      if (self.temptext38 > 0)
        self.RemoveSubPart(self.temptext38);
      if (self.temptext39 > 0)
        self.RemoveSubPart(self.temptext39);
      if (self.temptext40 > 0)
        self.RemoveSubPart(self.temptext40);
      if (self.temptext41 > 0)
        self.RemoveSubPart(self.temptext41);
      if (self.temptext42 > 0)
        self.RemoveSubPart(self.temptext42);
      if (self.temptext43 > 0)
        self.RemoveSubPart(self.temptext43);
      if (self.temptext44 > 0)
        self.RemoveSubPart(self.temptext44);
      if (self.temptext45 > 0)
        self.RemoveSubPart(self.temptext45);
      if (self.temptext46 > 0)
        self.RemoveSubPart(self.temptext46);
      if (self.hqbut0 > 0)
        self.RemoveSubPart(self.hqbut0);
      if (self.hqbut1 > 0)
        self.RemoveSubPart(self.hqbut1);
      if (self.hqbut2 > 0)
        self.RemoveSubPart(self.hqbut2);
      if (self.LogoListId > 0)
        self.RemoveSubPart(self.LogoListId);
      if (self.logolist2id > 0)
        self.RemoveSubPart(self.logolist2id);
      if (self.logolist3id > 0)
        self.RemoveSubPart(self.logolist3id);
      if (self.but1id > 0)
        self.RemoveSubPart(self.but1id);
      if (self.but1bid > 0)
        self.RemoveSubPart(self.but1bid);
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
      if (self.but6id > 0)
        self.RemoveSubPart(self.but6id);
      if (self.but6textid > 0)
        self.RemoveSubPart(self.but6textid);
      if (self.but7id > 0)
        self.RemoveSubPart(self.but7id);
      if (self.but7textid > 0)
        self.RemoveSubPart(self.but7textid);
      if (self.sliderid > 0)
        self.RemoveSubPart(self.sliderid);
      if (self.descid > 0)
        self.RemoveSubPart(self.descid);
      if (self.tab1id > 0)
        self.RemoveSubPart(self.tab1id);
      if (self.tab2id > 0)
        self.RemoveSubPart(self.tab2id);
      if (self.tab3id > 0)
        self.RemoveSubPart(self.tab3id);
      if (self.tab4id > 0)
        self.RemoveSubPart(self.tab4id);
      if (self.tab5id > 0)
        self.RemoveSubPart(self.tab5id);
      if (self.quitid > 0)
        self.RemoveSubPart(self.quitid);
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONQUIT);
      self.but1id = self.AddSubPart( tsubpart1, 952, 22, 32, 32, 1);
      self.comparenr = self.game.EditObj.SFCompare;
      num1: i32;
      y1: i32;
      if (self.comparenr > -1)
      {
        num1 = 180;
        y1 = 50;
      }
      else
      {
        num1 = 349;
        y1 = 50;
      }
      index1: i32;
      index2: i32;
      if (self.sftyp > -1)
      {
        index1 = self.sftyp;
        index2 = self.game.Data.Turn <= -1 ? 0 : self.game.Data.RegimeObj[self.game.Data.Turn].People;
      }
      else
        index1 = -1;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (index1 > -1)
      {
        str1: String = "";
        if (self.sfnr > -1)
        {
          index1 = self.game.Data.SFObj[self.sfnr].Type;
          index2 = self.game.Data.SFObj[self.sfnr].People;
          if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime == self.game.Data.Turn)
          {
            str1 = Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Qty)) + "x ";
            if (self.game.Data.SFTypeObj[index1].Ratio > 1)
              str1 = str1 + " " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[index1].Ratio)) + "x ";
          }
        }
        str2: String;
        index3: i32;
        index4: i32;
        index5: i32;
        if (index1 > -1)
        {
          name: String = self.game.Data.SFTypeObj[index1].Name;
          index6: i32;
          if (self.game.Data.RegimeObj[index6].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[index1].ExtraCounter;
            for (let mut index7: i32 = 0; index7 <= extraCounter; index7 += 1)
            {
              if (self.game.Data.SFTypeObj[index1].ExtraCode[index7] == self.game.Data.RegimeObj[index6].ExtraGraphicUse)
                name = self.game.Data.SFTypeObj[index1].ExtraName[index7];
            }
          }
          else if (index2 > -1 & self.sfnr > -1 && self.game.Data.PeopleObj[self.game.Data.SFObj[self.sfnr].People].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[index1].ExtraCounter;
            for (let mut index8: i32 = 0; index8 <= extraCounter; index8 += 1)
            {
              if (self.game.Data.SFTypeObj[index1].ExtraCode[index8] == self.game.Data.PeopleObj[index2].ExtraGraphicUse)
                name = self.game.Data.SFTypeObj[index1].ExtraName[index8];
            }
          }
          str2 = name;
          str3: String = str1 + name;
          index6 = 0;
          index3 = self.sfnr <= -1 ? self.game.Data.Turn : self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime;
          let mut picSpriteId: i32 = self.game.Data.SFTypeObj[index1].PicSpriteID;
          let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[index1].SidewaysSpriteID;
          if (self.game.Data.RegimeObj[index3].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[index1].ExtraCounter;
            for (let mut index9: i32 = 0; index9 <= extraCounter; index9 += 1)
            {
              if (self.game.Data.SFTypeObj[index1].ExtraCode[index9] == self.game.Data.RegimeObj[index3].ExtraGraphicUse)
              {
                picSpriteId = self.game.Data.SFTypeObj[index1].ExtraPicSpriteID[index9];
                sidewaysSpriteId = self.game.Data.SFTypeObj[index1].ExtraSidewaysSpriteID[index9];
              }
            }
          }
          else if (index2 > -1 && self.game.Data.PeopleObj[index2].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[index1].ExtraCounter;
            for (let mut index10: i32 = 0; index10 <= extraCounter; index10 += 1)
            {
              if (self.game.Data.SFTypeObj[index1].ExtraCode[index10] == self.game.Data.PeopleObj[index2].ExtraGraphicUse)
              {
                picSpriteId = self.game.Data.SFTypeObj[index1].ExtraPicSpriteID[index10];
                sidewaysSpriteId = self.game.Data.SFTypeObj[index1].ExtraSidewaysSpriteID[index10];
              }
            }
          }
           let mut local1: &Graphics = &Expression;
          rectangle1 = Rectangle::new(num1, y1, 322, 14);
          let mut rect1: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(num1, y1 + 14, 322, 23);
          let mut rect2: &Rectangle = &rectangle2
          txt2: String = str3;
          DrawMod.MakeFullBoxVic2( local1, rect1, "SELECTED SUBFORMATION(TYPE)", rect2, txt2);
          let mut x1: i32 = num1;
          let mut y2: i32 = y1 + 47;
          if ( self.game.Data.RuleVar[869] >= 1.0)
          {
            index4 =  Math.Round( self.game.Data.RuleVar[873]);
            index5 = 0;
            if ( self.game.Data.RuleVar[848] > 0.0 & self.game.Data.SFTypeObj[index1].Theater == 2)
            {
              index4 =  Math.Round( self.game.Data.RuleVar[848]);
              index5 = 0;
            }
            if ( self.game.Data.RuleVar[872] > 0.0 & self.game.Data.SFTypeObj[index1].Theater == 1)
            {
              index4 =  Math.Round( self.game.Data.RuleVar[872]);
              index5 = 0;
            }
            if ( self.game.Data.RuleVar[869] == 3.0)
            {
              let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].BasicPicID[index5];
               let mut local2: &Graphics = &Expression;
              bitmap: Bitmap = BitmapStore.GetBitmap(nr);
               let mut local3: &Bitmap = &bitmap;
              rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(x1, y2, 192, 144);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local2,  local3, srcrect, destrect);
            }
            else
            {
              if ( self.game.Data.RuleVar[869] == 1.0)
              {
                let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID1[index5];
                 let mut local4: &Graphics = &Expression;
                bitmap: Bitmap = BitmapStore.GetBitmap(nr);
                 let mut local5: &Bitmap = &bitmap;
                rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(x1, y2, 192, 144);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local4,  local5, srcrect, destrect);
              }
              let mut nr1: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID2[index5];
               let mut local6: &Graphics = &Expression;
              bitmap1: Bitmap = BitmapStore.GetBitmap(nr1);
               let mut local7: &Bitmap = &bitmap1;
              rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr1));
              let mut srcrect1: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(x1, y2, 192, 144);
              let mut destrect1: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local6,  local7, srcrect1, destrect1);
            }
          }
          let mut index11: i32 = index3;
          let mut red: i32 = self.game.Data.RegimeObj[index11].Red;
          let mut green: i32 = self.game.Data.RegimeObj[index11].Green;
          let mut blue: i32 = self.game.Data.RegimeObj[index11].Blue;
          switch (self.game.Data.SFTypeObj[index1].BaseColor)
          {
            case 0:
               let mut local8: &Graphics = &Expression;
              bitmap2: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local9: &Bitmap = &bitmap2;
              let mut x2: i32 = x1;
              let mut y3: i32 = y2;
              DrawMod.DrawScaled( local8,  local9, x2, y3, 192, 144);
              break;
            case 1:
               let mut local10: &Graphics = &Expression;
              bitmap3: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local11: &Bitmap = &bitmap3;
              let mut x3: i32 = x1;
              let mut y4: i32 = y2;
              let mut width1: i32 = BitmapStore.GetWidth(picSpriteId);
              let mut origh1: i32 = BitmapStore.Getheight(picSpriteId);
              double r1 =  ( red / 256f);
              double g1 =  ( green / 256f);
              double b1 =  ( blue / 256f);
              DrawMod.DrawScaledColorized2( local10,  local11, x3, y4, 192, 144, width1, origh1,  r1,  g1,  b1, 1f);
              break;
            case 2:
              let mut red2: i32 = self.game.Data.RegimeObj[index11].Red2;
              let mut green2: i32 = self.game.Data.RegimeObj[index11].Green2;
              let mut blue2: i32 = self.game.Data.RegimeObj[index11].Blue2;
               let mut local12: &Graphics = &Expression;
              bitmap4: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local13: &Bitmap = &bitmap4;
              let mut x4: i32 = x1;
              let mut y5: i32 = y2;
              let mut width2: i32 = BitmapStore.GetWidth(picSpriteId);
              let mut origh2: i32 = BitmapStore.Getheight(picSpriteId);
              double r2 =  ( red2 / 256f);
              double g2 =  ( green2 / 256f);
              double b2 =  ( blue2 / 256f);
              DrawMod.DrawScaledColorized2( local12,  local13, x4, y5, 192, 144, width2, origh2,  r2,  g2,  b2, 1f);
              break;
            case 3:
              let mut red3: i32 = self.game.Data.RegimeObj[index11].Red3;
              let mut green3: i32 = self.game.Data.RegimeObj[index11].Green3;
              let mut blue3: i32 = self.game.Data.RegimeObj[index11].Blue3;
               let mut local14: &Graphics = &Expression;
              bitmap5: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local15: &Bitmap = &bitmap5;
              let mut x5: i32 = x1;
              let mut y6: i32 = y2;
              let mut width3: i32 = BitmapStore.GetWidth(picSpriteId);
              let mut origh3: i32 = BitmapStore.Getheight(picSpriteId);
              double r3 =  ( red3 / 256f);
              double g3 =  ( green3 / 256f);
              double b3 =  ( blue3 / 256f);
              DrawMod.DrawScaledColorized2( local14,  local15, x5, y6, 192, 144, width3, origh3,  r3,  g3,  b3, 1f);
              break;
            case 4:
              let mut red4: i32 = self.game.Data.RegimeObj[index11].Red4;
              let mut green4: i32 = self.game.Data.RegimeObj[index11].Green4;
              let mut blue4: i32 = self.game.Data.RegimeObj[index11].Blue4;
               let mut local16: &Graphics = &Expression;
              bitmap6: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local17: &Bitmap = &bitmap6;
              let mut x6: i32 = x1;
              let mut y7: i32 = y2;
              let mut width4: i32 = BitmapStore.GetWidth(picSpriteId);
              let mut origh4: i32 = BitmapStore.Getheight(picSpriteId);
              double r4 =  ( red4 / 256f);
              double g4 =  ( green4 / 256f);
              double b4 =  ( blue4 / 256f);
              DrawMod.DrawScaledColorized2( local16,  local17, x6, y7, 192, 144, width4, origh4,  r4,  g4,  b4, 1f);
              break;
            case 5:
               let mut local18: &Graphics = &Expression;
              bitmap7: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local19: &Bitmap = &bitmap7;
              let mut x7: i32 = x1;
              let mut y8: i32 = y2;
              let mut width5: i32 = BitmapStore.GetWidth(picSpriteId);
              let mut origh5: i32 = BitmapStore.Getheight(picSpriteId);
              double r5 =  ( (red + 392) / 1024f);
              double g5 =  ( (green + 392) / 1024f);
              double b5 =  ( (blue + 392) / 1024f);
              DrawMod.DrawScaledColorized2( local18,  local19, x7, y8, 192, 144, width5, origh5,  r5,  g5,  b5, 1f);
              break;
            case 6:
               let mut local20: &Graphics = &Expression;
              bitmap8: Bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local21: &Bitmap = &bitmap8;
              let mut x8: i32 = x1;
              let mut y9: i32 = y2;
              let mut width6: i32 = BitmapStore.GetWidth(picSpriteId);
              let mut origh6: i32 = BitmapStore.Getheight(picSpriteId);
              double r6 =  ( (red + 80) / 512f);
              double g6 =  ( (green + 200) / 512f);
              double b6 =  ( (blue + 80) / 512f);
              DrawMod.DrawScaledColorized2( local20,  local21, x8, y9, 192, 144, width6, origh6,  r6,  g6,  b6, 1f);
              break;
          }
          if ( self.game.Data.RuleVar[870] > 0.0 & !Information.IsNothing( BitmapStore.GetBitmap(sidewaysSpriteId)))
          {
             let mut local22: &Graphics = &Expression;
            bitmap9: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
             let mut local23: &Bitmap = &bitmap9;
            let mut x9: i32 = x1;
            let mut y10: i32 = y2;
            DrawMod.DrawScaled( local22,  local23, x9, y10, 192, 144);
          }
          if ( self.game.Data.RuleVar[869] >= 1.0 &  self.game.Data.RuleVar[869] < 3.0)
          {
            let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID3[index5];
             let mut local24: &Graphics = &Expression;
            bitmap10: Bitmap = BitmapStore.GetBitmap(nr);
             let mut local25: &Bitmap = &bitmap10;
            rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1, y2, 192, 144);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local24,  local25, srcrect, destrect);
          }
          DrawMod.DrawRectangle( Expression, num1, y1 + 47, 192, 144,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
        }
        if (self.sfnr > -1)
        {
          self.OptionsList2Obj = ATListClass::new();
          tdata: i32;
          self.OptionsList2Obj.add("AP", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Ap)));
          self.OptionsList2Obj.add("RDN", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Rdn)));
          self.OptionsList2Obj.add("EXP", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Xp)));
          self.OptionsList2Obj.add("MOR", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Mor)));
          self.OptionsList2Obj.add("ENT", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].CurrentEntrench)));
          self.OptionsList2Obj.add("OFF", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].OffMod)) + "%");
          self.OptionsList2Obj.add("DEF", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].DefMod)) + "%");
          float num2 = -1f;
          if (self.game.EditObj.UnitSelected > -1 & self.sfnr > -1 && self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Theater == 2)
          {
            num2 =  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y) >= 1.0 ? 1f :  (0.33 + 0.66 *  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y));
            if ( num2 < 1.0)
            {
              tvalue: String = Strings.Trim(Conversion.Str( - Math.Round(100.0 - 100.0 *  num2))) + "%";
              self.OptionsList2Obj.add("AIRSTK", tdata, tvalue);
            }
            else
              self.OptionsList2Obj.add("AIRSTK", tdata, "0%");
          }
          if ( num2 == -1.0)
            self.OptionsList2Obj.add("AIRSTK", tdata, "0%");
          self.OptionsList2Obj.add("EP", tdata, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].EP)));
          self.OptionsList2Obj.add("PPL", tdata, Strings.Left(self.game.Data.PeopleObj[self.game.Data.SFObj[self.sfnr].People].Name, 7));
          if (self.game.Data.SFObj[self.sfnr].MoveType > -1)
            self.OptionsList2Obj.add("MOVE", tdata, self.game.Data.TempString[self.game.Data.SFObj[self.sfnr].MoveType]);
          else
            self.OptionsList2Obj.add("MOVE", tdata, "normal");
          if (self.OptionsList2Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, -1);
            self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
          }
          else
          {
            let mut tsubpart2: SubPartClass =  new ATListSubPartClass(self.OptionsList2Obj, 10, 125, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 75, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 200), bby: (y1 + 47));
            self.OptionsList2Id = self.AddSubPart( tsubpart2, num1 + 200, y1 + 47, 125, 176, 0);
          }
        }
        if (self.comparenr == -1)
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Compare", 125, "Allows you to compare with a different subformation type",  self.OwnBitmap, num1 + 403, y1 + 115);
          self.but7id = self.AddSubPart( tsubpart3, num1 + 403, y1 + 115, 125, 35, 1);
        }
        else
        {
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change Compare", 125, "Allows you to compare with a different subformation type",  self.OwnBitmap, num1 + 623, y1 + 115);
          self.but7id = self.AddSubPart( tsubpart4, num1 + 623, y1 + 115, 125, 35, 1);
        }
        index12: i32;
        if (self.comparenr > -1)
        {
          num1 = 570;
          y1 = 50;
          index12 = self.comparenr;
          index2 = self.game.Data.Turn <= -1 ? 0 : self.game.Data.RegimeObj[self.game.Data.Turn].People;
        }
        else
          index12 = -1;
        if (self.comparenr > -1)
        {
          str4: String = "";
          if (index12 > -1)
          {
            name: String = self.game.Data.SFTypeObj[index12].Name;
            if (self.game.Data.RegimeObj[index3].ExtraGraphicUse > -1)
            {
              let mut extraCounter: i32 = self.game.Data.SFTypeObj[index12].ExtraCounter;
              for (let mut index13: i32 = 0; index13 <= extraCounter; index13 += 1)
              {
                if (self.game.Data.SFTypeObj[index12].ExtraCode[index13] == self.game.Data.RegimeObj[index3].ExtraGraphicUse)
                  name = self.game.Data.SFTypeObj[index12].ExtraName[index13];
              }
            }
            else if (index2 > -1 && self.game.Data.PeopleObj[index2].ExtraGraphicUse > -1)
            {
              let mut extraCounter: i32 = self.game.Data.SFTypeObj[index12].ExtraCounter;
              for (let mut index14: i32 = 0; index14 <= extraCounter; index14 += 1)
              {
                if (self.game.Data.SFTypeObj[index12].ExtraCode[index14] == self.game.Data.PeopleObj[index2].ExtraGraphicUse)
                  name = self.game.Data.SFTypeObj[index12].ExtraName[index14];
              }
            }
            str5: String = str4 + name;
            let mut turn: i32 = self.game.Data.Turn;
            let mut picSpriteId: i32 = self.game.Data.SFTypeObj[index12].PicSpriteID;
            let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[index12].SidewaysSpriteID;
            if (self.game.Data.RegimeObj[turn].ExtraGraphicUse > -1)
            {
              let mut extraCounter: i32 = self.game.Data.SFTypeObj[index12].ExtraCounter;
              for (let mut index15: i32 = 0; index15 <= extraCounter; index15 += 1)
              {
                if (self.game.Data.SFTypeObj[index12].ExtraCode[index15] == self.game.Data.RegimeObj[turn].ExtraGraphicUse)
                {
                  picSpriteId = self.game.Data.SFTypeObj[index12].ExtraPicSpriteID[index15];
                  sidewaysSpriteId = self.game.Data.SFTypeObj[index12].ExtraSidewaysSpriteID[index15];
                }
              }
            }
            else if (index2 > -1 && self.game.Data.PeopleObj[index2].ExtraGraphicUse > -1)
            {
              let mut extraCounter: i32 = self.game.Data.SFTypeObj[index12].ExtraCounter;
              for (let mut index16: i32 = 0; index16 <= extraCounter; index16 += 1)
              {
                if (self.game.Data.SFTypeObj[index12].ExtraCode[index16] == self.game.Data.PeopleObj[index2].ExtraGraphicUse)
                {
                  picSpriteId = self.game.Data.SFTypeObj[index12].ExtraPicSpriteID[index16];
                  sidewaysSpriteId = self.game.Data.SFTypeObj[index12].ExtraSidewaysSpriteID[index16];
                }
              }
            }
             let mut local26: &Graphics = &Expression;
            rectangle2 = Rectangle::new(num1, y1, 192, 14);
            let mut rect1: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(num1, y1 + 14, 192, 23);
            let mut rect2: &Rectangle = &rectangle1
            txt2: String = str5;
            DrawMod.MakeFullBoxVic2( local26, rect1, "COMPARISON SUBFORMATIONTYPE", rect2, txt2);
            let mut index17: i32 = turn;
            let mut red: i32 = self.game.Data.RegimeObj[index17].Red;
            let mut green: i32 = self.game.Data.RegimeObj[index17].Green;
            let mut blue: i32 = self.game.Data.RegimeObj[index17].Blue;
            let mut baseColor: i32 = self.game.Data.SFTypeObj[index12].BaseColor;
            let mut x10: i32 = num1;
            let mut y11: i32 = y1 + 47;
            if ( self.game.Data.RuleVar[869] >= 1.0)
            {
              index4 =  Math.Round( self.game.Data.RuleVar[873]);
              index5 = 0;
              if ( self.game.Data.RuleVar[848] > 0.0 & self.game.Data.SFTypeObj[index12].Theater == 2)
              {
                index4 =  Math.Round( self.game.Data.RuleVar[848]);
                index5 = 0;
              }
              if ( self.game.Data.RuleVar[872] > 0.0 & self.game.Data.SFTypeObj[index12].Theater == 1)
              {
                index4 =  Math.Round( self.game.Data.RuleVar[872]);
                index5 = 0;
              }
              if ( self.game.Data.RuleVar[869] == 3.0)
              {
                let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].BasicPicID[index5];
                 let mut local27: &Graphics = &Expression;
                bitmap: Bitmap = BitmapStore.GetBitmap(nr);
                 let mut local28: &Bitmap = &bitmap;
                rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(x10, y11, 192, 144);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local27,  local28, srcrect, destrect);
              }
              else
              {
                if ( self.game.Data.RuleVar[869] == 1.0)
                {
                  let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID1[index5];
                   let mut local29: &Graphics = &Expression;
                  bitmap: Bitmap = BitmapStore.GetBitmap(nr);
                   let mut local30: &Bitmap = &bitmap;
                  rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(x10, y11, 192, 144);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2( local29,  local30, srcrect, destrect);
                }
                let mut nr2: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID2[index5];
                 let mut local31: &Graphics = &Expression;
                bitmap11: Bitmap = BitmapStore.GetBitmap(nr2);
                 let mut local32: &Bitmap = &bitmap11;
                rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr2));
                let mut srcrect2: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(x10, y11, 192, 144);
                let mut destrect2: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local31,  local32, srcrect2, destrect2);
              }
            }
            switch (baseColor)
            {
              case 0:
                 let mut local33: &Graphics = &Expression;
                bitmap12: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local34: &Bitmap = &bitmap12;
                let mut x11: i32 = x10;
                let mut y12: i32 = y11;
                DrawMod.DrawScaled( local33,  local34, x11, y12, 192, 144);
                break;
              case 1:
                 let mut local35: &Graphics = &Expression;
                bitmap13: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local36: &Bitmap = &bitmap13;
                let mut x12: i32 = x10;
                let mut y13: i32 = y11;
                let mut width7: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh7: i32 = BitmapStore.Getheight(picSpriteId);
                double r7 =  ( red / 256f);
                double g7 =  ( green / 256f);
                double b7 =  ( blue / 256f);
                DrawMod.DrawScaledColorized2( local35,  local36, x12, y13, 192, 144, width7, origh7,  r7,  g7,  b7, 1f);
                break;
              case 2:
                let mut red2: i32 = self.game.Data.RegimeObj[index17].Red2;
                let mut green2: i32 = self.game.Data.RegimeObj[index17].Green2;
                let mut blue2: i32 = self.game.Data.RegimeObj[index17].Blue2;
                 let mut local37: &Graphics = &Expression;
                bitmap14: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local38: &Bitmap = &bitmap14;
                let mut x13: i32 = x10;
                let mut y14: i32 = y11;
                let mut width8: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh8: i32 = BitmapStore.Getheight(picSpriteId);
                double r8 =  ( red2 / 256f);
                double g8 =  ( green2 / 256f);
                double b8 =  ( blue2 / 256f);
                DrawMod.DrawScaledColorized2( local37,  local38, x13, y14, 192, 144, width8, origh8,  r8,  g8,  b8, 1f);
                break;
              case 3:
                let mut red3: i32 = self.game.Data.RegimeObj[index17].Red3;
                let mut green3: i32 = self.game.Data.RegimeObj[index17].Green3;
                let mut blue3: i32 = self.game.Data.RegimeObj[index17].Blue3;
                 let mut local39: &Graphics = &Expression;
                bitmap15: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local40: &Bitmap = &bitmap15;
                let mut x14: i32 = x10;
                let mut y15: i32 = y11;
                let mut width9: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh9: i32 = BitmapStore.Getheight(picSpriteId);
                double r9 =  ( red3 / 256f);
                double g9 =  ( green3 / 256f);
                double b9 =  ( blue3 / 256f);
                DrawMod.DrawScaledColorized2( local39,  local40, x14, y15, 192, 144, width9, origh9,  r9,  g9,  b9, 1f);
                break;
              case 4:
                let mut red4: i32 = self.game.Data.RegimeObj[index17].Red4;
                let mut green4: i32 = self.game.Data.RegimeObj[index17].Green4;
                let mut blue4: i32 = self.game.Data.RegimeObj[index17].Blue4;
                 let mut local41: &Graphics = &Expression;
                bitmap16: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local42: &Bitmap = &bitmap16;
                let mut x15: i32 = x10;
                let mut y16: i32 = y11;
                let mut width10: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh10: i32 = BitmapStore.Getheight(picSpriteId);
                double r10 =  ( red4 / 256f);
                double g10 =  ( green4 / 256f);
                double b10 =  ( blue4 / 256f);
                DrawMod.DrawScaledColorized2( local41,  local42, x15, y16, 192, 144, width10, origh10,  r10,  g10,  b10, 1f);
                break;
              case 5:
                 let mut local43: &Graphics = &Expression;
                bitmap17: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local44: &Bitmap = &bitmap17;
                let mut x16: i32 = x10;
                let mut y17: i32 = y11;
                let mut width11: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh11: i32 = BitmapStore.Getheight(picSpriteId);
                double r11 =  ( (red + 392) / 1024f);
                double g11 =  ( (green + 392) / 1024f);
                double b11 =  ( (blue + 392) / 1024f);
                DrawMod.DrawScaledColorized2( local43,  local44, x16, y17, 192, 144, width11, origh11,  r11,  g11,  b11, 1f);
                break;
              case 6:
                 let mut local45: &Graphics = &Expression;
                bitmap18: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local46: &Bitmap = &bitmap18;
                let mut x17: i32 = x10;
                let mut y18: i32 = y11;
                let mut width12: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh12: i32 = BitmapStore.Getheight(picSpriteId);
                double r12 =  ( (red + 80) / 512f);
                double g12 =  ( (green + 200) / 512f);
                double b12 =  ( (blue + 80) / 512f);
                DrawMod.DrawScaledColorized2( local45,  local46, x17, y18, 192, 144, width12, origh12,  r12,  g12,  b12, 1f);
                break;
            }
            if ( self.game.Data.RuleVar[870] > 0.0 & !Information.IsNothing( BitmapStore.GetBitmap(sidewaysSpriteId)))
            {
               let mut local47: &Graphics = &Expression;
              bitmap19: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
               let mut local48: &Bitmap = &bitmap19;
              let mut x18: i32 = x10;
              let mut y19: i32 = y11;
              DrawMod.DrawScaled( local47,  local48, x18, y19, 192, 144);
            }
            if ( self.game.Data.RuleVar[869] >= 1.0 &  self.game.Data.RuleVar[869] < 3.0)
            {
              let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID3[index5];
               let mut local49: &Graphics = &Expression;
              bitmap20: Bitmap = BitmapStore.GetBitmap(nr);
               let mut local50: &Bitmap = &bitmap20;
              rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(x10, y11, 192, 144);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local49,  local50, srcrect, destrect);
            }
            DrawMod.DrawRectangle( Expression, num1, y1 + 47, 192, 144,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
          }
        }
        let mut sftyp1: i32 = self.sftyp;
        let mut num3: i32 = 300;
        let mut num4: i32 = 15;
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Text & Upgrade", 150, tBackbitmap: ( self.OwnBitmap), bbx: (num4 + 170 - 85), bby: num3, tred: (self.StatMode == 0));
        self.tab1id = self.AddSubPart( tsubpart5, num4 + 170 - 85, num3, 150, 35, 1);
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("General Stats", 150, tBackbitmap: ( self.OwnBitmap), bbx: (num4 + 340 - 85), bby: num3, tred: (self.StatMode == 1));
        self.tab2id = self.AddSubPart( tsubpart6, num4 + 340 - 85, num3, 150, 35, 1);
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Combat Stats", 150, tBackbitmap: ( self.OwnBitmap), bbx: (num4 + 510 - 85), bby: num3, tred: (self.StatMode == 2));
        self.tab3id = self.AddSubPart( tsubpart7, num4 + 510 - 85, num3, 150, 35, 1);
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("Prevent Rules", 150, tBackbitmap: ( self.OwnBitmap), bbx: (num4 + 680 - 85), bby: num3, tred: (self.StatMode == 4));
        self.tab5id = self.AddSubPart( tsubpart8, num4 + 680 - 85, num3, 150, 35, 1);
        let mut tsubpart9: SubPartClass =  new TextButtonPartClass("Landscape Stats", 150, tBackbitmap: ( self.OwnBitmap), bbx: (num4 + 850 - 85), bby: num3, tred: (self.StatMode == 3));
        self.tab4id = self.AddSubPart( tsubpart9, num4 + 850 - 85, num3, 150, 35, 1);
        DrawMod.DrawBlock( Expression, num4 + 50, num3 + 55, 890, 355,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
        DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  Expression, num4 + 50, num3 + 55, 890, 355, -1, -1);
        if (self.StatMode == 0)
        {
          let mut num5: i32 = 0;
          let mut num6: i32 = 50;
          let mut num7: i32 = 130;
          let mut sftyp2: i32 = self.sftyp;
          bool flag;
          if (self.sfnr > -1)
          {
            if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X > -1 && self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime == self.game.Data.Turn && !self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType].IsSea)
            {
              self.detailnr = -1;
              let mut num8: i32 = 0;
              if (self.game.Data.SFTypeObj[sftyp2].UpgradeToo > -1)
              {
                let mut upgradeToo: i32 = self.game.Data.SFTypeObj[sftyp2].UpgradeToo;
                if (self.game.HandyFunctionsObj.CanUpgrade(self.sfnr, self.unr))
                {
                  if (self.HQselect > -1)
                  {
                    name: String = self.game.Data.SFTypeObj[upgradeToo].Name;
                     let mut local51: &Graphics = &Expression;
                    rectangle2 = Rectangle::new(num6 + 30, num7 +  byte.MaxValue, 192, 14);
                    let mut rect1_1: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(num6 + 30, num7 + 268, 192, 23);
                    let mut rect2_1: &Rectangle = &rectangle1
                    txt2_1: String = name;
                    DrawMod.MakeFullBoxVic2( local51, rect1_1, "UPGRADEABLE TOO", rect2_1, txt2_1);
                    flag = true;
                    num7 -= 30;
                    str6: String = self.game.Data.UnitObj[self.HQselect].Name;
                    if (Strings.Len(str6) > 18)
                      str6 = Strings.Left(str6, 18);
                     let mut local52: &Graphics = &Expression;
                    rectangle2 = Rectangle::new(num6 + 250, num7 + 360, 192, 14);
                    let mut rect1_2: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(num6 + 250, num7 + 374, 192, 23);
                    let mut rect2_2: &Rectangle = &rectangle1
                    txt2_2: String = str6;
                    DrawMod.MakeFullBoxVic2( local52, rect1_2, "SELECTED HQ", rect2_2, txt2_2);
                    str7: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.HQSupply(self.HQselect)));
                     let mut local53: &Graphics = &Expression;
                    rectangle2 = Rectangle::new(num6 + 250, num7 + 400, 192, 14);
                    let mut rect1_3: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(num6 + 250, num7 + 414, 192, 23);
                    let mut rect2_3: &Rectangle = &rectangle1
                    txt2_3: String = str7;
                    DrawMod.MakeFullBoxVic2( local53, rect1_3, "SUPPLY AVAILABLE", rect2_3, txt2_3);
                    let mut picSpriteId: i32 = self.game.Data.SFTypeObj[upgradeToo].PicSpriteID;
                    let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[upgradeToo].SidewaysSpriteID;
                    let mut index18: i32 = !(self.sfnr > -1 & self.game.EditObj.UnitSelected > -1) ? self.game.Data.Turn : self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime;
                    if (self.game.Data.RegimeObj[index18].ExtraGraphicUse > -1)
                    {
                      let mut extraCounter: i32 = self.game.Data.SFTypeObj[upgradeToo].ExtraCounter;
                      for (let mut index19: i32 = 0; index19 <= extraCounter; index19 += 1)
                      {
                        if (self.game.Data.SFTypeObj[upgradeToo].ExtraCode[index19] == self.game.Data.RegimeObj[index18].ExtraGraphicUse)
                        {
                          picSpriteId = self.game.Data.SFTypeObj[upgradeToo].ExtraPicSpriteID[index19];
                          sidewaysSpriteId = self.game.Data.SFTypeObj[upgradeToo].ExtraSidewaysSpriteID[index19];
                        }
                      }
                    }
                    else if (index2 > -1 && self.game.Data.PeopleObj[index2].ExtraGraphicUse > -1)
                    {
                      let mut extraCounter: i32 = self.game.Data.SFTypeObj[upgradeToo].ExtraCounter;
                      for (let mut index20: i32 = 0; index20 <= extraCounter; index20 += 1)
                      {
                        if (self.game.Data.SFTypeObj[upgradeToo].ExtraCode[index20] == self.game.Data.PeopleObj[index2].ExtraGraphicUse)
                        {
                          picSpriteId = self.game.Data.SFTypeObj[upgradeToo].ExtraPicSpriteID[index20];
                          sidewaysSpriteId = self.game.Data.SFTypeObj[upgradeToo].ExtraSidewaysSpriteID[index20];
                        }
                      }
                    }
                     let mut local54: &Graphics = &Expression;
                    bitmap21: Bitmap = BitmapStore.GetBitmap(picSpriteId);
                     let mut local55: &Bitmap = &bitmap21;
                    let mut x19: i32 = num6 + 30;
                    let mut y20: i32 = num7 + 330;
                    DrawMod.DrawScaled( local54,  local55, x19, y20, 192, 144);
                     let mut local56: &Graphics = &Expression;
                    bitmap22: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
                     let mut local57: &Bitmap = &bitmap22;
                    let mut x20: i32 = num6 + 30;
                    let mut y21: i32 = num7 + 330;
                    DrawMod.DrawScaled( local56,  local57, x20, y21, 192, 144);
                     let mut local58: &Graphics = &Expression;
                    rectangle2 = Rectangle::new(num6 + 248, num7 + 287, 192, 14);
                    let mut rect1_4: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(num6 + 248, num7 + 301, 192, 50);
                    let mut rect2_4: &Rectangle = &rectangle1
                    DrawMod.MakeFullBoxVic2( local58, rect1_4, "AVAILABLE HQS", rect2_4, "");
                    if (self.ChainHq[0] > -1)
                    {
                      bool forcehighlight = false;
                      if (self.HQselect == self.ChainHq[0] & self.HQselect != -1)
                        forcehighlight = true;
                      let mut tsubpart10: SubPartClass =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.ChainHq[0], forcehighlight), self.game.Data.UnitObj[self.ChainHq[0]].Name);
                      self.hqbut0 = self.AddSubPart( tsubpart10, num6 + 260, num7 + 308, 36, 36, 0);
                      if (forcehighlight)
                        DrawMod.DrawRectangle( Expression, num6 + 258, num7 + 306, 40, 40,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                    }
                    if (self.ChainHq[1] > -1)
                    {
                      bool forcehighlight = false;
                      if (self.HQselect == self.ChainHq[1] & self.HQselect != -1)
                        forcehighlight = true;
                      let mut tsubpart11: SubPartClass =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.ChainHq[1], forcehighlight), self.game.Data.UnitObj[self.ChainHq[1]].Name);
                      self.hqbut1 = self.AddSubPart( tsubpart11, num6 + 300, num7 + 308, 36, 36, 0);
                      if (forcehighlight)
                        DrawMod.DrawRectangle( Expression, num6 + 298, num7 + 306, 40, 40,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                    }
                    if (self.ChainHq[2] > -1)
                    {
                      bool forcehighlight = false;
                      if (self.HQselect == self.ChainHq[2] & self.HQselect != -1)
                        forcehighlight = true;
                      let mut tsubpart12: SubPartClass =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.ChainHq[2], forcehighlight), self.game.Data.UnitObj[self.ChainHq[2]].Name);
                      self.hqbut2 = self.AddSubPart( tsubpart12, num6 + 340, num7 + 308, 36, 36, 0);
                      if (forcehighlight)
                        DrawMod.DrawRectangle( Expression, num6 + 338, num7 + 306, 40, 40,  byte.MaxValue, 0, 0,  byte.MaxValue, 2);
                    }
                    self.detailnr = upgradeToo;
                    num8 = self.game.HandyFunctionsObj.CanUpgradeMax(self.sfnr, self.unr, self.HQselect);
                    if (num8 == -1)
                      num8 = 0;
                    if (num8 > self.game.Data.SFObj[self.sfnr].Qty)
                      num8 = self.game.Data.SFObj[self.sfnr].Qty;
                    num5 = 1;
                  }
                  else
                  {
                    str8: String = "No HQ in supply range.";
                     let mut local: &Graphics = &Expression;
                    rectangle2 = Rectangle::new(num6 + 30, num7 +  byte.MaxValue, 192, 14);
                    let mut rect1: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(num6 + 30, num7 + 268, 192, 23);
                    let mut rect2: &Rectangle = &rectangle1
                    txt2: String = str8;
                    DrawMod.MakeFullBoxVic2( local, rect1, "UPGRADEABLE TOO", rect2, txt2);
                    num5 = 1;
                  }
                }
              }
              if (self.detailnr > -1 & num8 > 0 & self.HQselect > -1)
              {
                if (self.game.HandyFunctionsObj.CanUpgradeCost(self.sfnr, self.unr, self.detailnr2) <= self.game.HandyFunctionsObj.HQSupply(self.HQselect))
                {
                  let mut tsubpart13: SubPartClass =  new TextButtonPartClass("Do Upgrade", 120, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 340), bby: (num7 + 504));
                  self.but3id = self.AddSubPart( tsubpart13, num6 + 340, num7 + 504, 120, 35, 1);
                }
                str9: String = Conversion.Str( Conversion.Int( self.game.Data.SFTypeObj[sftyp2].UpgradeCost / self.game.Data.RuleVar[77])) + " X " + Conversion.Str( self.detailnr2) + " = " + Conversion.Str( self.game.HandyFunctionsObj.CanUpgradeCost(self.sfnr, self.unr, self.detailnr2));
                 let mut local59: &Graphics = &Expression;
                rectangle2 = Rectangle::new(num6 + 250, num7 + 440, 192, 14);
                let mut rect1: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(num6 + 250, num7 + 454, 192, 23);
                let mut rect2: &Rectangle = &rectangle1
                txt2: String = str9;
                DrawMod.MakeFullBoxVic2( local59, rect1, "SUPPLY COST", rect2, txt2);
                flag = true;
                if (self.detailnr2 > num8)
                  self.detailnr2 = num8;
                let mut game: GameClass = self.game;
                tsuffix: String = " of " + Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Qty));
                let mut tmaxval: i32 = num8;
                let mut detailnr2: i32 = self.detailnr2;
                bitmap: Bitmap = (Bitmap) null;
                 let mut local60: &Bitmap = &bitmap;
                let mut tsubpart14: SubPartClass =  new NumberSliderSubPartClass2(game, "Upgrade ", tsuffix, 300, 0, tmaxval, detailnr2, tbackbitmap: ( local60));
                self.sliderid = self.AddSubPart( tsubpart14, num6 + 30, num7 + 500, 300, 40, 0);
              }
              let mut tsubpart15: SubPartClass =  new TextButtonPartClass("Disband", 100, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 30), bby: 660);
              self.but2id = self.AddSubPart( tsubpart15, num6 + 30, 660, 100, 35, 1);
            }
            if (num5 == 0 && self.game.Data.SFTypeObj[sftyp2].UpgradeToo > -1)
            {
              let mut upgradeToo: i32 = self.game.Data.SFTypeObj[sftyp2].UpgradeToo;
              let mut picSpriteId: i32 = self.game.Data.SFTypeObj[self.game.Data.SFTypeObj[sftyp2].UpgradeToo].PicSpriteID;
              let mut index21: i32 = !(self.sfnr > -1 & self.game.EditObj.UnitSelected > -1) ? self.game.Data.Turn : self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime;
              if (self.game.Data.Turn == index21)
              {
                if (self.game.Data.RegimeObj[index21].ExtraGraphicUse > -1)
                {
                  let mut extraCounter: i32 = self.game.Data.SFTypeObj[upgradeToo].ExtraCounter;
                  for (let mut index22: i32 = 0; index22 <= extraCounter; index22 += 1)
                  {
                    if (self.game.Data.SFTypeObj[upgradeToo].ExtraCode[index22] == self.game.Data.RegimeObj[index21].ExtraGraphicUse)
                      picSpriteId = self.game.Data.SFTypeObj[upgradeToo].ExtraPicSpriteID[index22];
                  }
                }
                else if (index2 > -1 && self.game.Data.PeopleObj[index2].ExtraGraphicUse > -1)
                {
                  let mut extraCounter: i32 = self.game.Data.SFTypeObj[upgradeToo].ExtraCounter;
                  for (let mut index23: i32 = 0; index23 <= extraCounter; index23 += 1)
                  {
                    if (self.game.Data.SFTypeObj[upgradeToo].ExtraCode[index23] == self.game.Data.PeopleObj[index2].ExtraGraphicUse)
                      picSpriteId = self.game.Data.SFTypeObj[upgradeToo].ExtraPicSpriteID[index23];
                  }
                }
                flag = true;
                name1: String = self.game.Data.SFTypeObj[self.game.Data.SFTypeObj[sftyp2].UpgradeToo].Name;
                 let mut local61: &Graphics = &Expression;
                rectangle2 = Rectangle::new(num6 + 30, num7 +  byte.MaxValue, 192, 14);
                let mut rect1_5: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(num6 + 30, num7 + 268, 192, 23);
                let mut rect2_5: &Rectangle = &rectangle1
                txt2_4: String = name1;
                DrawMod.MakeFullBoxVic2( local61, rect1_5, "COULD BE UPGRADED TOO", rect2_5, txt2_4);
                let mut y22: i32 = num7 - 30 +  byte.MaxValue + 30;
                if (self.game.Data.SFTypeObj[sftyp2].UpgradeXP > 0)
                {
                  str10: String = Conversion.Str( self.game.Data.SFTypeObj[sftyp2].UpgradeXP);
                   let mut local62: &Graphics = &Expression;
                  rectangle2 = Rectangle::new(num6 + 260, y22, 192, 14);
                  let mut rect1_6: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(num6 + 260, y22 + 14, 192, 23);
                  let mut rect2_6: &Rectangle = &rectangle1
                  txt2_5: String = str10;
                  DrawMod.MakeFullBoxVic2( local62, rect1_6, "Minimum XP needed", rect2_6, txt2_5);
                  y22 += 40;
                }
                let mut index24: i32 = -1;
                let mut itemTypeCounter: i32 = self.game.Data.ItemTypeCounter;
                for (let mut index25: i32 = 0; index25 <= itemTypeCounter; index25 += 1)
                {
                  if (self.game.Data.ItemTypeObj[index25].IsSFType == self.game.Data.SFTypeObj[sftyp2].UpgradeToo)
                  {
                    index24 = index25;
                    break;
                  }
                }
                if (index24 > -1)
                {
                  let mut index26: i32 = 0;
                  do
                  {
                    if (self.game.Data.ItemTypeObj[index24].ResFieldNeeded[index26] > -1)
                    {
                      name2: String = self.game.Data.ResearchObj[self.game.Data.ItemTypeObj[index24].ResFieldNeeded[index26]].Name;
                       let mut local63: &Graphics = &Expression;
                      rectangle2 = Rectangle::new(num6 + 260, y22, 192, 14);
                      let mut rect1_7: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(num6 + 260, y22 + 14, 192, 23);
                      let mut rect2_7: &Rectangle = &rectangle1
                      txt2_6: String = name2;
                      DrawMod.MakeFullBoxVic2( local63, rect1_7, "RESEARCH NEED FOR UPGRADE", rect2_7, txt2_6);
                      y22 += 40;
                    }
                    index26 += 1;
                  }
                  while (index26 <= 4);
                }
                str11: String = Conversion.Str( Conversion.Int( self.game.Data.SFTypeObj[sftyp2].UpgradeCost / self.game.Data.RuleVar[77]));
                 let mut local64: &Graphics = &Expression;
                rectangle2 = Rectangle::new(num6 + 260, y22, 192, 14);
                let mut rect1_8: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(num6 + 260, y22 + 14, 192, 23);
                let mut rect2_8: &Rectangle = &rectangle1
                txt2_7: String = str11;
                DrawMod.MakeFullBoxVic2( local64, rect1_8, "UPGRADE SUPPLY COST", rect2_8, txt2_7);
              }
            }
          }
          if (sftyp2 > -1)
          {
            if (flag)
            {
              let mut num9: i32 = 550;
              let mut num10: i32 = 385;
              DrawMod.DrawPaperSheet( Expression, num9 - 20, num10 - 10, 415, 320);
              let mut tsubpart16: SubPartClass =  new PaperAreaClass(self.game, 365, 14,  null, "Description", false, str2 + "\r\n\r\n" + self.game.Data.SFTypeObj[sftyp2].Description, self.game.VicColor8, tItemSize: 20, tbackbitmap: ( self.OwnBitmap), bbx: num9, bby: num10);
              self.descid = self.AddSubPart( tsubpart16, num9, num10, 365, 300, 0);
            }
            else
            {
              let mut num11: i32 = 220;
              let mut num12: i32 = 385;
              DrawMod.DrawPaperSheet( Expression, num11 - 20, num12 - 10, 630, 320);
              let mut tsubpart17: SubPartClass =  new PaperAreaClass(self.game, 580, 14,  null, "Description", false, str2 + "\r\n\r\n" + self.game.Data.SFTypeObj[sftyp2].Description, self.game.VicColor8, tItemSize: 20, tbackbitmap: ( self.OwnBitmap), bbx: num11, bby: num12);
              self.descid = self.AddSubPart( tsubpart17, num11, num12, 580, 300, 0);
            }
          }
        }
      }
      let mut sftyp3: i32 = self.sftyp;
      str12: String;
      if (self.StatMode == 1 & sftyp3 > -1)
      {
        let mut num13: i32 = 150;
        let mut num14: i32 = 385;
        self.OptionsList3Obj = ATListClass::new();
        str12 = "";
        tvalue2_1: String = "";
        tvalue1: String = self.game.Data.TempString[self.game.Data.SFTypeObj[sftyp3].UnitGroup + 400];
        if (self.comparenr > -1)
          tvalue2_1 = self.game.Data.TempString[self.game.Data.SFTypeObj[self.comparenr].UnitGroup + 400];
        self.OptionsList3Obj.add("SFTypeGroup", -1, tvalue1, tvalue2_1);
        str12 = "";
        tvalue2_2: String = "";
        tvalue2: String = self.game.Data.TempString[self.game.Data.SFTypeObj[sftyp3].MoveType + 0];
        if (self.comparenr > -1)
          tvalue2_2 = self.game.Data.TempString[self.game.Data.SFTypeObj[self.comparenr].MoveType + 0];
        self.OptionsList3Obj.add("MoveType", -1, tvalue2, tvalue2_2);
        str12 = "";
        tvalue2_3: String = "";
        tvalue3: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].SupplyCarry));
        if (self.comparenr > -1)
          tvalue2_3 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].SupplyCarry));
        self.OptionsList3Obj.add("Supply Carry", -1, tvalue3, tvalue2_3);
        str12 = "";
        tvalue2_4: String = "";
        tvalue4: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].BasicSupplyNeed));
        if (self.comparenr > -1)
          tvalue2_4 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].BasicSupplyNeed));
        self.OptionsList3Obj.add("Supply Consum", -1, tvalue4, tvalue2_4);
        str12 = "";
        tvalue2_5: String = "";
        tvalue5: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].Cap));
        if (self.game.Data.SFTypeObj[sftyp3].Theater > 0)
          tvalue5 = "0";
        if (self.comparenr > -1)
          tvalue2_5 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Cap));
        if (self.comparenr > -1 && self.game.Data.SFTypeObj[self.comparenr].Theater > 0)
          tvalue2_5 = "0";
        self.OptionsList3Obj.add("Land.Tr.Cap", -1, tvalue5, tvalue2_5);
        str12 = "";
        tvalue2_6: String = "";
        tvalue6: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].Cap));
        if (self.game.Data.SFTypeObj[sftyp3].Theater != 1)
          tvalue6 = "0";
        if (self.comparenr > -1)
          tvalue2_6 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Cap));
        if (self.comparenr > -1 && self.game.Data.SFTypeObj[self.comparenr].Theater != 1)
          tvalue2_6 = "0";
        self.OptionsList3Obj.add("Sea.Tr.Cap", -1, tvalue6, tvalue2_6);
        str12 = "";
        tvalue2_7: String = "";
        tvalue7: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].RailCap));
        if (self.comparenr > -1)
          tvalue2_7 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].RailCap));
        self.OptionsList3Obj.add("Rail.Tr.Cap", -1, tvalue7, tvalue2_7);
        str12 = "";
        tvalue2_8: String = "";
        tvalue8: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].Theater));
        if (self.comparenr > -1)
          tvalue2_8 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Theater));
        self.OptionsList3Obj.add("Theater", -1, tvalue8, tvalue2_8);
        str12 = "";
        tvalue2_9: String = "";
        tvalue9: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].Weight));
        if (self.comparenr > -1)
          tvalue2_9 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Weight));
        self.OptionsList3Obj.add("Weight", -1, tvalue9, tvalue2_9);
        str12 = "";
        tvalue2_10: String = "";
        tvalue10: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].CarryCap));
        if (self.comparenr > -1)
          tvalue2_10 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].CarryCap));
        self.OptionsList3Obj.add("Carry Cap.", -1, tvalue10, tvalue2_10);
        str12 = "";
        tvalue2_11: String = "";
        tvalue11: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].MoveRedux));
        if (self.comparenr > -1)
          tvalue2_11 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].MoveRedux));
        self.OptionsList3Obj.add("Move Redux", -1, tvalue11, tvalue2_11);
        str12 = "";
        tvalue2_12: String = "";
        tvalue12: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].ReconPts));
        if (self.comparenr > -1)
          tvalue2_12 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].ReconPts));
        self.OptionsList3Obj.add("Recon Points", -1, tvalue12, tvalue2_12);
        str12 = "";
        tvalue2_13: String = "";
        tvalue13: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].ZOCPts));
        if (self.comparenr > -1)
          tvalue2_13 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].ZOCPts));
        self.OptionsList3Obj.add("ZOC Points", -1, tvalue13, tvalue2_13);
        str12 = "";
        tvalue2_14: String = "";
        tvalue14: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].HidePts));
        if (self.comparenr > -1)
          tvalue2_14 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].HidePts));
        self.OptionsList3Obj.add("Hide Points", -1, tvalue14, tvalue2_14);
        str12 = "";
        tvalue2_15: String = "";
        tvalue15: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].CanDoParadrop));
        if (self.comparenr > -1)
          tvalue2_15 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].CanDoParadrop));
        self.OptionsList3Obj.add("Paradrop", -1, tvalue15, tvalue2_15);
        str12 = "";
        tvalue2_16: String = "";
        tvalue16: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].EP));
        if (self.comparenr > -1)
          tvalue2_16 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].EP));
        self.OptionsList3Obj.add("Engineer Pts", -1, tvalue16, tvalue2_16);
        str12 = "";
        tvalue2_17: String = "";
        tvalue17: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].StaffPts));
        if (self.comparenr > -1)
          tvalue2_17 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].StaffPts));
        self.OptionsList3Obj.add("Staff Pts", -1, tvalue17, tvalue2_17);
        if (self.OptionsList3Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList3Id)].Refresh(self.OptionsList3Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList3Id)] = true;
        }
        else
        {
          let mut tsubpart18: SubPartClass =  new ATListSubPartClass(self.OptionsList3Obj, 16, 330, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 230, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num13, bby: num14);
          self.OptionsList3Id = self.AddSubPart( tsubpart18, num13, num14, 330, 272, 0);
        }
        let mut num15: i32 = 540;
        let mut num16: i32 = 385;
        self.OptionsList4Obj = ATListClass::new();
        str12 = "";
        tvalue2_18: String = "";
        tvalue18: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].StaffCombatMod));
        if (self.comparenr > -1)
          tvalue2_18 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].StaffCombatMod));
        self.OptionsList4Obj.add("Staff Com Mod", -1, tvalue18, tvalue2_18);
        str12 = "";
        tvalue2_19: String = "";
        tvalue19: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].StaffMoraleMod));
        if (self.comparenr > -1)
          tvalue2_19 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].StaffMoraleMod));
        self.OptionsList4Obj.add("Staff Mor Mod", -1, tvalue19, tvalue2_19);
        str12 = "";
        tvalue2_20: String = "";
        tvalue20: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].BlowBridgePts));
        if (self.comparenr > -1)
          tvalue2_20 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].BlowBridgePts));
        self.OptionsList4Obj.add("Blow Pts", -1, tvalue20, tvalue2_20);
        str12 = "";
        tvalue2_21: String = "";
        tvalue21: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].AntiSupply));
        if (self.comparenr > -1)
          tvalue2_21 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AntiSupply));
        self.OptionsList4Obj.add("AntiSup Pts Land", -1, tvalue21, tvalue2_21);
        str12 = "";
        tvalue2_22: String = "";
        tvalue22: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].AntiSupplySea));
        if (self.comparenr > -1)
          tvalue2_22 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AntiSupplySea));
        self.OptionsList4Obj.add("AntiSup Pts Sea", -1, tvalue22, tvalue2_22);
        str12 = "";
        tvalue2_23: String = "";
        tvalue23: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].AntiSupplyRange));
        if (self.comparenr > -1)
          tvalue2_23 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AntiSupplyRange));
        self.OptionsList4Obj.add("AntiSup Range", -1, tvalue23, tvalue2_23);
        str12 = "";
        tvalue2_24: String = "";
        tvalue24: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].ConsiderCarry));
        if (self.comparenr > -1)
          tvalue2_24 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].ConsiderCarry));
        self.OptionsList4Obj.add("Consider Carry", -1, tvalue24, tvalue2_24);
        str12 = "";
        tvalue2_25: String = "";
        tvalue25: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].AirCarrierCap));
        if (self.comparenr > -1)
          tvalue2_25 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AirCarrierCap));
        self.OptionsList4Obj.add("Aircar.CarryCap", -1, tvalue25, tvalue2_25);
        str12 = "";
        tvalue2_26: String = "";
        tvalue26: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].ApMod));
        if (self.comparenr > -1)
          tvalue2_26 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].ApMod));
        self.OptionsList4Obj.add("AP Mod", -1, tvalue26, tvalue2_26);
        str12 = "";
        tvalue2_27: String = "";
        tvalue27: String = !(self.game.Data.SFTypeObj[sftyp3].FuelRegimeVar > 0 & self.game.Data.SFTypeObj[sftyp3].FuelForMove + self.game.Data.SFTypeObj[sftyp3].FuelForAttack + self.game.Data.SFTypeObj[sftyp3].FuelForAttackDef > 0) ? "None" : Strings.Trim(self.game.Data.RegimeSlotName[self.game.Data.SFTypeObj[sftyp3].FuelRegimeVar]);
        if (self.comparenr > -1)
          tvalue2_27 = !(self.game.Data.SFTypeObj[self.comparenr].FuelRegimeVar >= 0 & self.game.Data.SFTypeObj[self.comparenr].FuelForMove + self.game.Data.SFTypeObj[self.comparenr].FuelForAttack + self.game.Data.SFTypeObj[self.comparenr].FuelForAttackDef > 0) ? "None" : Strings.Trim(self.game.Data.RegimeSlotName[self.game.Data.SFTypeObj[self.comparenr].FuelRegimeVar]);
        self.OptionsList4Obj.add("Fuel Type", -1, tvalue27, tvalue2_27);
        str12 = "";
        tvalue2_28: String = "";
        tvalue28: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].FuelForMove));
        if (self.comparenr > -1)
          tvalue2_28 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].FuelForMove));
        self.OptionsList4Obj.add("Fuel for Move", -1, tvalue28, tvalue2_28);
        str12 = "";
        tvalue2_29: String = "";
        tvalue29: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].FuelForAttack));
        if (self.comparenr > -1)
          tvalue2_29 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].FuelForAttack));
        self.OptionsList4Obj.add("Fuel for Attack", -1, tvalue29, tvalue2_29);
        str12 = "";
        tvalue2_30: String = "";
        tvalue30: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].FuelForAttackDef));
        if (self.comparenr > -1)
          tvalue2_30 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].FuelForAttackDef));
        self.OptionsList4Obj.add("Fuel for Defense", -1, tvalue30, tvalue2_30);
        str12 = "";
        tvalue2_31: String = "";
        tvalue31: String = "* " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].OutOfFuelAttack));
        if (self.comparenr > -1)
          tvalue2_31 = "* " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].OutOfFuelAttack));
        self.OptionsList4Obj.add("Out of fuel Att", -1, tvalue31, tvalue2_31);
        str12 = "";
        tvalue2_32: String = "";
        tvalue32: String = "* " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].OutOfFuelDefense));
        if (self.comparenr > -1)
          tvalue2_32 = "* " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].OutOfFuelDefense));
        self.OptionsList4Obj.add("Out of fuel Def", -1, tvalue32, tvalue2_32);
        str12 = "";
        tvalue2_33: String = "";
        tvalue33: String = "/ " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].OutOfFuelMove));
        if (self.comparenr > -1)
          tvalue2_33 = "/ " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].OutOfFuelMove));
        self.OptionsList4Obj.add("Out of fuel Mov", -1, tvalue33, tvalue2_33);
        str12 = "";
        tvalue2_34: String = "";
        tvalue34: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp3].AntiStrucPts));
        if (self.comparenr > -1)
          tvalue2_34 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AntiStrucPts));
        self.OptionsList4Obj.add("Anti-Struc Pts", -1, tvalue34, tvalue2_34);
        if (self.OptionsList4Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList4Id)].Refresh(self.OptionsList4Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList4Id)] = true;
        }
        else
        {
          let mut tsubpart19: SubPartClass =  new ATListSubPartClass(self.OptionsList4Obj, 16, 330, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 230, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num15, bby: num16);
          self.OptionsList4Id = self.AddSubPart( tsubpart19, num15, num16, 330, 272, 0);
        }
      }
      let mut sftyp4: i32 = self.sftyp;
      if (self.StatMode == 2 & sftyp4 > -1)
      {
        let mut num17: i32 = 150;
        let mut num18: i32 = 385;
        self.OptionsList3Obj = ATListClass::new();
        str12 = "";
        tvalue2_35: String = "";
        tvalue35: String = Conversions.ToString(self.game.Data.SFTypeObj[sftyp4].Initiative);
        if (self.comparenr > -1)
          tvalue2_35 = Conversions.ToString(self.game.Data.SFTypeObj[self.comparenr].Initiative);
        self.OptionsList3Obj.add("Initiative Att", -1, tvalue35, tvalue2_35);
        str12 = "";
        tvalue2_36: String = "";
        tvalue36: String = Conversions.ToString(self.game.Data.SFTypeObj[sftyp4].InitiativeDef);
        if (self.comparenr > -1)
          tvalue2_36 = Conversions.ToString(self.game.Data.SFTypeObj[self.comparenr].InitiativeDef);
        self.OptionsList3Obj.add("Initiative Def", -1, tvalue36, tvalue2_36);
        str12 = "";
        tvalue2_37: String = "";
        tvalue37: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].Attacks));
        if (self.comparenr > -1)
          tvalue2_37 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Attacks));
        self.OptionsList3Obj.add("Attacks", -1, tvalue37, tvalue2_37);
        str12 = "";
        tvalue2_38: String = "";
        tvalue38: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].MaxAttacked));
        if (self.comparenr > -1)
          tvalue2_38 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].MaxAttacked));
        self.OptionsList3Obj.add("Max Attacked", -1, tvalue38, tvalue2_38);
        str12 = "";
        tvalue2_39: String = "";
        tvalue39: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].Frontage));
        if (self.comparenr > -1)
          tvalue2_39 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Frontage));
        self.OptionsList3Obj.add("Stack Pts", -1, tvalue39, tvalue2_39);
        str12 = "";
        tvalue2_40: String = "";
        tvalue40: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].BackBench));
        if (self.comparenr > -1)
          tvalue2_40 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].BackBench));
        self.OptionsList3Obj.add("Rear Area", -1, tvalue40, tvalue2_40);
        str12 = "";
        tvalue2_41: String = "";
        tvalue41: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].ArtRange));
        if (self.comparenr > -1)
          tvalue2_41 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].ArtRange));
        self.OptionsList3Obj.add("Art Range", -1, tvalue41, tvalue2_41);
        str12 = "";
        tvalue2_42: String = "";
        tvalue42: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].FavTargetTries));
        if (self.comparenr > -1)
          tvalue2_42 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].FavTargetTries));
        self.OptionsList3Obj.add("Fav.Target.Tries", -1, tvalue42, tvalue2_42);
        str12 = "";
        tvalue2_43: String = "";
        tvalue43: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].AARange));
        if (self.comparenr > -1)
          tvalue2_43 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AARange));
        self.OptionsList3Obj.add("AA Range", -1, tvalue43, tvalue2_43);
        str12 = "";
        tvalue2_44: String = "";
        tvalue44: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].KillPercent));
        let mut index27: i32 = sftyp4;
        if (self.game.Data.SFTypeObj[index27].ArtSFType > -1)
        {
          let mut artSfType: i32 = self.game.Data.SFTypeObj[index27].ArtSFType;
          tvalue44 = tvalue44 + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[artSfType].KillPercent)) + ")";
        }
        else if (self.game.Data.SFTypeObj[index27].ArtRange > 0)
          tvalue44 = tvalue44 + " (" + tvalue44 + ")";
        if (self.comparenr > -1)
        {
          tvalue2_44 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].KillPercent));
          let mut comparenr: i32 = self.comparenr;
          if (self.game.Data.SFTypeObj[comparenr].ArtSFType > -1)
          {
            let mut artSfType: i32 = self.game.Data.SFTypeObj[comparenr].ArtSFType;
            tvalue2_44 = tvalue2_44 + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[artSfType].KillPercent)) + ")";
          }
          else if (self.game.Data.SFTypeObj[comparenr].ArtRange > 0)
            tvalue2_44 = tvalue2_44 + " (" + tvalue2_44 + ")";
        }
        self.OptionsList3Obj.add("Hit->Kill%", -1, tvalue44, tvalue2_44);
        str12 = "";
        tvalue2_45: String = "";
        tvalue45: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].RetreatPercent));
        let mut index28: i32 = sftyp4;
        if (self.game.Data.SFTypeObj[index28].ArtSFType > -1)
        {
          let mut artSfType: i32 = self.game.Data.SFTypeObj[index28].ArtSFType;
          tvalue45 = tvalue45 + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[artSfType].RetreatPercent)) + ")";
        }
        else if (self.game.Data.SFTypeObj[index28].ArtRange > 0)
          tvalue45 = tvalue45 + " (" + tvalue45 + ")";
        if (self.comparenr > -1)
        {
          tvalue2_45 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].RetreatPercent));
          let mut comparenr: i32 = self.comparenr;
          if (self.game.Data.SFTypeObj[comparenr].ArtSFType > -1)
          {
            let mut artSfType: i32 = self.game.Data.SFTypeObj[comparenr].ArtSFType;
            tvalue2_45 = tvalue2_45 + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[artSfType].RetreatPercent)) + ")";
          }
          else if (self.game.Data.SFTypeObj[comparenr].ArtRange > 0)
            tvalue2_45 = tvalue2_45 + " (" + tvalue2_45 + ")";
        }
        self.OptionsList3Obj.add("Hit->Retreat%", -1, tvalue45, tvalue2_45);
        str12 = "";
        tvalue2_46: String = "";
        tvalue46: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].FirstRoundPenaltyMod));
        if (self.comparenr > -1)
          tvalue2_46 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].FirstRoundPenaltyMod));
        self.OptionsList3Obj.add("1stRnd pen.mod", -1, tvalue46, tvalue2_46);
        str12 = "";
        tvalue2_47: String = "";
        tvalue47: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].EntrenchPower));
        if (self.comparenr > -1)
          tvalue2_47 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].EntrenchPower));
        self.OptionsList3Obj.add("Entrench Pow", -1, tvalue47, tvalue2_47);
        str12 = "";
        tvalue2_48: String = "";
        tvalue48: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].PowerPts));
        if (self.comparenr > -1)
          tvalue2_48 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].PowerPts));
        self.OptionsList3Obj.add("Power Points", -1, tvalue48, tvalue2_48);
        str12 = "";
        tvalue2_49: String = "";
        tvalue49: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].RdnLossPerAttack));
        if (self.comparenr > -1)
          tvalue2_49 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].RdnLossPerAttack));
        self.OptionsList3Obj.add("RdnLos.p.attack", -1, tvalue49, tvalue2_49);
        str12 = "";
        tvalue2_50: String = "";
        tvalue50: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].AutoDestroy)) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].AutoDestroy2));
        if (self.comparenr > -1)
          tvalue2_50 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AutoDestroy)) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].AutoDestroy));
        self.OptionsList3Obj.add("AutoDestroy", -1, tvalue50, tvalue2_50);
        str12 = "";
        tvalue2_51: String = "";
        tvalue51: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp4].KilltoRetreatChance));
        if (self.comparenr > -1)
          tvalue2_51 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].KilltoRetreatChance));
        self.OptionsList3Obj.add("Kill becomes rtr", -1, tvalue51, tvalue2_51);
        if (self.OptionsList3Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList3Id)].Refresh(self.OptionsList3Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList3Id)] = true;
        }
        else
        {
          let mut tsubpart20: SubPartClass =  new ATListSubPartClass(self.OptionsList3Obj, 17, 250, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 150, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num17, bby: num18);
          self.OptionsList3Id = self.AddSubPart( tsubpart20, num17, num18, 300, 288, 0);
        }
        self.combatListObj = ATListClass::new();
        let mut index29: i32 = sftyp4;
        SimpleList simpleList1 = SimpleList::new();
        let mut tid1: i32 = 0;
        do
        {
          if (self.game.Data.SFTypeObj[index29].FavTarget[tid1] > 0)
            simpleList1.Add(tid1, 1000 * self.game.Data.SFTypeObj[index29].FavTarget[tid1] - tid1);
          else if (self.comparenr > -1)
          {
            if (self.game.Data.SFTypeObj[self.comparenr].FavTarget[tid1] > 0)
              simpleList1.Add(tid1, 100 * self.game.Data.SFTypeObj[self.comparenr].FavTarget[tid1] - 9999 - tid1);
            else
              simpleList1.Add(tid1, -9999 - tid1);
          }
          else
            simpleList1.Add(tid1, -9999 - tid1);
          tid1 += 1;
        }
        while (tid1 <= 99);
        simpleList1.Sort();
        let mut Number1: i32 = 0;
        let mut num19: i32 = 9999999;
        if (self.comparenr > -1)
          self.combatListObj.add("TARGET GROUP", -1, "ATT", "HP", "ATT", "HP");
        else
          self.combatListObj.add("TARGET GROUP", -1, "ATT", "HP");
        for (let mut counter: i32 = simpleList1.Counter; counter >= 0; counter += -1)
        {
          str12 = "";
          if (simpleList1.Weight[counter] < num19 - 99)
          {
            Number1 += 1;
            num19 = simpleList1.Weight[counter];
          }
          tname: String = Strings.Trim(Conversion.Str( Number1)) + ". " + self.game.Data.TempString[400 + simpleList1.Id[counter]];
          tvalue52: String = Strings.Trim(Conversion.Str( (self.game.Data.SFTypeObj[index29].AttackPower[simpleList1.Id[counter]] * self.game.Data.SFTypeObj[index29].Attacks))) + "/" + Strings.Trim(Conversion.Str( (self.game.Data.SFTypeObj[index29].AttackPowerDef[simpleList1.Id[counter]] * self.game.Data.SFTypeObj[index29].Attacks)));
          tvalue2_52: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[index29].HitPoints[simpleList1.Id[counter]])) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[index29].HitPointsDef[simpleList1.Id[counter]]));
          tvalue3: String;
          tvalue4: String;
          if (self.comparenr > -1)
          {
            tvalue3 = Strings.Trim(Conversion.Str( (self.game.Data.SFTypeObj[self.comparenr].AttackPower[simpleList1.Id[counter]] * self.game.Data.SFTypeObj[self.comparenr].Attacks))) + "/" + Strings.Trim(Conversion.Str( (self.game.Data.SFTypeObj[self.comparenr].AttackPowerDef[simpleList1.Id[counter]] * self.game.Data.SFTypeObj[self.comparenr].Attacks)));
            tvalue4 = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].HitPoints[simpleList1.Id[counter]])) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].HitPointsDef[simpleList1.Id[counter]]));
          }
          if (self.comparenr > -1)
            self.combatListObj.add(tname, counter, tvalue52, tvalue2_52, tvalue3, tvalue4);
          else
            self.combatListObj.add(tname, counter, tvalue52, tvalue2_52);
        }
        let mut num20: i32 = 450;
        let mut num21: i32 = 385;
        if (self.combatListId > 0)
        {
          self.SubPartList[self.SubpartNr(self.combatListId)].Refresh(self.combatListObj, -1);
          self.SubPartFlag[self.SubpartNr(self.combatListId)] = true;
        }
        else if (self.game.Data.SFTypeObj[index29].ArtRange > 0)
        {
          let mut tsubpart21: SubPartClass =  new ATListSubPartClass(self.combatListObj, 8, 430, -1, self.game, true, "AttackPoints / Hitpoints", tShowPair: true, tValueWidth: 300, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num20, bby: num21);
          self.combatListId = self.AddSubPart( tsubpart21, num20, num21, 380, 144, 0);
        }
        else
        {
          let mut tsubpart22: SubPartClass =  new ATListSubPartClass(self.combatListObj, 8, 430, -1, self.game, true, "AttackPoints / Hitpoints", tShowPair: true, tValueWidth: 300, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num20, bby: num21);
          self.combatListId = self.AddSubPart( tsubpart22, num20, num21, 380, 144, 0);
        }
        self.combatList2Obj = ATListClass::new();
        let mut index30: i32 = sftyp4;
        if (self.game.Data.SFTypeObj[sftyp4].ArtRange > 0)
        {
          SimpleList simpleList2 = SimpleList::new();
          let mut tid2: i32 = 0;
          do
          {
            if (self.game.Data.SFTypeObj[index30].FavArtTarget[tid2] > 0)
              simpleList2.Add(tid2, 1000 * self.game.Data.SFTypeObj[index30].FavArtTarget[tid2] - tid2);
            else if (self.comparenr > -1 && self.game.Data.SFTypeObj[self.comparenr].FavArtTarget[tid2] > 0)
              simpleList2.Add(tid2, 100 * self.game.Data.SFTypeObj[self.comparenr].FavArtTarget[tid2] - 9999 - tid2);
            tid2 += 1;
          }
          while (tid2 <= 99);
          simpleList2.Sort();
          if (self.comparenr > -1)
            self.combatList2Obj.add("ARTILLERY TARGET GROUP", -1, "ART ATT", "ART ATT");
          else
            self.combatList2Obj.add("ARTILLERY TARGET GROUP", -1, "ART ATT");
          let mut Number2: i32 = 0;
          let mut num22: i32 = -1;
          for (let mut counter: i32 = simpleList2.Counter; counter >= 0; counter += -1)
          {
            str12 = "";
            if (simpleList2.Weight[counter] > num22 - 99)
            {
              Number2 += 1;
              num22 = simpleList2.Weight[counter];
            }
            tname: String = Strings.Trim(Conversion.Str( Number2)) + ". " + self.game.Data.TempString[400 + simpleList2.Id[counter]];
            tvalue53: String = Strings.Trim(Conversion.Str( (Conversions.ToInteger(self.game.Data.SFTypeObj[index30].AttackArt[simpleList2.Id[counter]]) * self.game.Data.SFTypeObj[index30].Attacks)));
            if (self.comparenr > -1)
            {
              tvalue2_53: String = Strings.Trim(Conversion.Str( (Conversions.ToInteger(self.game.Data.SFTypeObj[self.comparenr].AttackArt[simpleList2.Id[counter]]) * self.game.Data.SFTypeObj[self.comparenr].Attacks)));
              self.combatList2Obj.add(tname, counter, tvalue53, tvalue2_53);
            }
            else
              self.combatList2Obj.add(tname, counter, tvalue53);
          }
          let mut num23: i32 = 450;
          let mut num24: i32 = 540;
          if (self.combatList2Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.combatList2Id)].Refresh(self.combatList2Obj, -1);
            self.SubPartFlag[self.SubpartNr(self.combatList2Id)] = true;
          }
          else
          {
            let mut tsubpart23: SubPartClass =  new ATListSubPartClass(self.combatList2Obj, 8, 430, -1, self.game, true, "AttackPoints / Hitpoints", tShowPair: true, tValueWidth: 220, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num23, bby: num24);
            self.combatList2Id = self.AddSubPart( tsubpart23, num23, num24, 380, 144, 0);
          }
        }
      }
      if (self.StatMode == 3)
      {
        self.OptionsListObj = ATListClass::new();
        self.OptionsList4Obj = ATListClass::new();
        self.OptionsList3Obj = ATListClass::new();
        let mut index31: i32 = sftyp4;
        if (self.comparenr > -1)
        {
          self.OptionsListObj.add("LANDSCAPE", -1, "AP", "ENTR", "AP", "ENTR");
          self.OptionsList4Obj.add("ROAD TYPE", -1, "AP", "AP");
          self.OptionsList3Obj.add("RIVER TYPE", -1, "AP", "AP");
        }
        else
        {
          self.OptionsListObj.add("LANDSCAPE", -1, "AP", "ENTR");
          self.OptionsList4Obj.add("ROAD TYPE", -1, "AP");
          self.OptionsList3Obj.add("RIVER TYPE", -1, "AP");
        }
        let mut landscapeTypeCounter1: i32 = self.game.Data.LandscapeTypeCounter;
        for (let mut tdata: i32 = 0; tdata <= landscapeTypeCounter1; tdata += 1)
        {
          str12 = "";
          name: String = self.game.Data.LandscapeTypeObj[tdata].Name;
          let mut num25: i32 = self.game.Data.LandscapeTypeObj[tdata].MoveCost[self.game.Data.SFTypeObj[index31].MoveType];
          tvalue: String = Strings.Trim(Conversion.Str(  Math.Round(  Math.Round( num25 - Conversion.Int( num25 * ( self.game.Data.SFTypeObj[index31].MoveRedux / 100.0))) * ( self.game.Data.MoveTypePenalty[self.game.Data.SFTypeObj[index31].MoveType] / 100.0)))) + "ap";
          tvalue2: String = Strings.Trim(Conversion.Str( self.game.Data.LandscapeTypeObj[tdata].DefBonus[self.game.Data.SFTypeObj[index31].UnitGroup])) + "-" + Strings.Trim(Conversion.Str( self.game.Data.LandscapeTypeObj[tdata].DefBonusMax[self.game.Data.SFTypeObj[index31].UnitGroup]));
          if (self.comparenr > -1)
          {
            let mut num26: i32 = self.game.Data.LandscapeTypeObj[tdata].MoveCost[self.game.Data.SFTypeObj[self.comparenr].MoveType];
            tvalue3: String = Strings.Trim(Conversion.Str(  Math.Round(  Math.Round( num26 - Conversion.Int( num26 * ( self.game.Data.SFTypeObj[self.comparenr].MoveRedux / 100.0))) * ( self.game.Data.MoveTypePenalty[self.game.Data.SFTypeObj[self.comparenr].MoveType] / 100.0)))) + "ap";
            tvalue4: String = Strings.Trim(Conversion.Str( self.game.Data.LandscapeTypeObj[tdata].DefBonus[self.game.Data.SFTypeObj[self.comparenr].UnitGroup])) + "-" + Strings.Trim(Conversion.Str( self.game.Data.LandscapeTypeObj[tdata].DefBonusMax[self.game.Data.SFTypeObj[self.comparenr].UnitGroup]));
            self.OptionsListObj.add(name, tdata, tvalue, tvalue2, tvalue3, tvalue4);
          }
          else
            self.OptionsListObj.add(name, tdata, tvalue, tvalue2);
        }
        let mut roadTypeCounter: i32 = self.game.Data.RoadTypeCounter;
        str13: String;
        str14: String;
        for (let mut index32: i32 = 0; index32 <= roadTypeCounter; index32 += 1)
        {
          name: String = self.game.Data.RoadTypeObj[index32].Name;
          let mut num27: i32 = self.game.Data.RoadTypeObj[index32].MoveCostOverrule[self.game.Data.SFTypeObj[index31].MoveType];
          tvalue: String = Strings.Trim(Conversion.Str(  Math.Round(  Math.Round( num27 - Conversion.Int( num27 * ( self.game.Data.SFTypeObj[index31].MoveRedux / 100.0))) * ( self.game.Data.MoveTypePenalty[self.game.Data.SFTypeObj[index31].MoveType] / 100.0)))) + "ap";
          str13 = "-";
          if (self.comparenr > -1)
          {
            let mut num28: i32 = self.game.Data.RoadTypeObj[index32].MoveCostOverrule[self.game.Data.SFTypeObj[self.comparenr].MoveType];
            tvalue2: String = Strings.Trim(Conversion.Str(  Math.Round(  Math.Round( num28 - Conversion.Int( num28 * ( self.game.Data.SFTypeObj[self.comparenr].MoveRedux / 100.0))) * ( self.game.Data.MoveTypePenalty[self.game.Data.SFTypeObj[self.comparenr].MoveType] / 100.0)))) + "ap";
            str14 = "-";
            self.OptionsList4Obj.add(name, 1000 + index32, tvalue, tvalue2);
          }
          else
            self.OptionsList4Obj.add(name, 1000 + index32, tvalue);
        }
        let mut riverTypeCounter1: i32 = self.game.Data.RiverTypeCounter;
        for (let mut index33: i32 = 0; index33 <= riverTypeCounter1; index33 += 1)
        {
          name: String = self.game.Data.RiverTypeObj[index33].Name;
          let mut Number3: i32 = self.game.Data.RiverTypeObj[index33].MovePenalty[self.game.Data.SFTypeObj[index31].MoveType];
          str15: String = "";
          if (Number3 > 0)
            str15 = "+";
          tvalue: String = str15 + Strings.Trim(Conversion.Str( Number3)) + "ap";
          str13 = "-";
          if (self.comparenr > -1)
          {
            let mut Number4: i32 = self.game.Data.RiverTypeObj[index33].MovePenalty[self.game.Data.SFTypeObj[self.comparenr].MoveType];
            str16: String = "";
            if (Number4 > 0)
              str16 = "+";
            tvalue2: String = str16 + Strings.Trim(Conversion.Str( Number4)) + "ap";
            str14 = "-";
            self.OptionsList3Obj.add(name, 2000 + index33, tvalue, tvalue2);
          }
          else
            self.OptionsList3Obj.add(name, 2000 + index33, tvalue);
        }
        let mut num29: i32 = 110;
        let mut num30: i32 = 385;
        if (self.OptionsListId > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        }
        else
        {
          let mut tsubpart24: SubPartClass =  new ATListSubPartClass(self.OptionsListObj, 9, 400, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 260, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num29, bby: num30);
          self.OptionsListId = self.AddSubPart( tsubpart24, num29, num30, 400, 160, 0);
        }
        let mut num31: i32 = num30 + 170;
        if (self.OptionsList4Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList4Id)].Refresh(self.OptionsList4Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList4Id)] = true;
        }
        else
        {
          let mut tsubpart25: SubPartClass =  new ATListSubPartClass(self.OptionsList4Obj, 3, 400, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 260, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num29, bby: num31);
          self.OptionsList4Id = self.AddSubPart( tsubpart25, num29, num31, 400, 64, 0);
        }
        let mut num32: i32 = num31 + 74;
        if (self.OptionsList3Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList3Id)].Refresh(self.OptionsList3Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList3Id)] = true;
        }
        else
        {
          let mut tsubpart26: SubPartClass =  new ATListSubPartClass(self.OptionsList3Obj, 3, 400, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 260, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num29, bby: num32);
          self.OptionsList3Id = self.AddSubPart( tsubpart26, num29, num32, 400, 64, 0);
        }
        self.OptionsList6Obj = ATListClass::new();
        self.combatList2Obj = ATListClass::new();
        if (self.comparenr > -1)
        {
          self.OptionsList6Obj.add("LANDSCAPE", -1, "ATT", "DEF", "ATT", "DEF");
          self.combatList2Obj.add("RIVER TYPE", -1, "ATT", "DEF", "ATT", "DEF");
        }
        else
        {
          self.OptionsList6Obj.add("LANDSCAPE", -1, "ATT", "DEF");
          self.combatList2Obj.add("RIVER TYPE", -1, "ATT", "DEF");
        }
        let mut landscapeTypeCounter2: i32 = self.game.Data.LandscapeTypeCounter;
        for (let mut tdata: i32 = 0; tdata <= landscapeTypeCounter2; tdata += 1)
        {
          str12 = "";
          name: String = self.game.Data.LandscapeTypeObj[tdata].Name;
          let mut Number5: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[sftyp4].CombatModAtt[tdata] * 100f) - 100f));
          tvalue: String = Number5 != 0 ? Strings.Trim(Conversion.Str( Number5)) + "%" : "-";
          let mut artSfType1: i32 = self.game.Data.SFTypeObj[sftyp4].ArtSFType;
          if (artSfType1 > -1)
          {
            let mut Number6: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[artSfType1].CombatModAtt[tdata] * 100f) - 100f));
            tvalue = Number6 != 0 ? tvalue + " (" + Strings.Trim(Conversion.Str( Number6)) + "%)" : tvalue + " (-)";
          }
          let mut Number7: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[sftyp4].CombatModDef[tdata] * 100f) - 100f));
          tvalue2: String = Number7 != 0 ? Strings.Trim(Conversion.Str( Number7)) + "%" : "-";
          let mut artSfType2: i32 = self.game.Data.SFTypeObj[sftyp4].ArtSFType;
          if (artSfType2 > -1)
          {
            let mut Number8: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[artSfType2].CombatModDef[tdata] * 100f) - 100f));
            tvalue2 = Number8 != 0 ? tvalue2 + " (" + Strings.Trim(Conversion.Str( Number8)) + "%)" : tvalue2 + " (-)";
          }
          if (self.comparenr > -1)
          {
            let mut Number9: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[self.comparenr].CombatModAtt[tdata] * 100f) - 100f));
            tvalue3: String = Number9 != 0 ? Strings.Trim(Conversion.Str( Number9)) + "%" : "-";
            let mut artSfType3: i32 = self.game.Data.SFTypeObj[self.comparenr].ArtSFType;
            if (artSfType3 > -1)
            {
              let mut Number10: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[artSfType3].CombatModAtt[tdata] * 100f) - 100f));
              tvalue3 = Number10 != 0 ? tvalue3 + " (" + Strings.Trim(Conversion.Str( Number10)) + "%)" : tvalue3 + " (-)";
            }
            let mut Number11: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[self.comparenr].CombatModDef[tdata] * 100f) - 100f));
            tvalue4: String = Number11 != 0 ? Strings.Trim(Conversion.Str( Number11)) + "%" : "-";
            let mut artSfType4: i32 = self.game.Data.SFTypeObj[self.comparenr].ArtSFType;
            if (artSfType4 > -1)
            {
              let mut Number12: i32 =  Math.Round( (Conversion.Int(self.game.Data.SFTypeObj[artSfType4].CombatModDef[tdata] * 100f) - 100f));
              tvalue4 = Number12 != 0 ? tvalue4 + " (" + Strings.Trim(Conversion.Str( Number12)) + "%)" : tvalue4 + " (-)";
            }
            self.OptionsList6Obj.add(name, tdata, tvalue, tvalue2, tvalue3, tvalue4);
          }
          else
            self.OptionsList6Obj.add(name, tdata, tvalue, tvalue2);
        }
        let mut riverTypeCounter2: i32 = self.game.Data.RiverTypeCounter;
        for (let mut index34: i32 = 0; index34 <= riverTypeCounter2; index34 += 1)
        {
          name: String = self.game.Data.RiverTypeObj[index34].Name;
          let mut Number13: i32 =  Math.Round( Conversion.Int(self.game.Data.RiverTypeObj[index34].AttackPenalty[self.game.Data.SFTypeObj[sftyp4].UnitGroup] * 100f));
          tvalue: String = Number13 <= 0 ? "-" : "-" + Strings.Trim(Conversion.Str( Number13)) + "%";
          tvalue2: String = "-";
          if (self.comparenr > -1)
          {
            let mut Number14: i32 =  Math.Round( Conversion.Int(self.game.Data.RiverTypeObj[index34].AttackPenalty[self.game.Data.SFTypeObj[self.comparenr].UnitGroup] * 100f));
            tvalue3: String = Number14 <= 0 ? "-" : "-" + Strings.Trim(Conversion.Str( Number14)) + "%";
            tvalue4: String = "-";
            self.combatList2Obj.add(name, 2000 + index34, tvalue, tvalue2, tvalue3, tvalue4);
          }
          else
            self.combatList2Obj.add(name, 2000 + index34, tvalue, tvalue2);
        }
        let mut num33: i32 = 520;
        let mut num34: i32 = 385;
        if (self.OptionsList6Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList6Id)].Refresh(self.OptionsList6Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList6Id)] = true;
        }
        else
        {
          let mut tsubpart27: SubPartClass =  new ATListSubPartClass(self.OptionsList6Obj, 13, 400, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 260, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num33, bby: num34);
          self.OptionsList6Id = self.AddSubPart( tsubpart27, num33, num34, 400, 224, 0);
        }
        let mut num35: i32 = 520;
        let mut num36: i32 = num34 + 244;
        if (self.combatList2Id > 0)
        {
          self.SubPartList[self.SubpartNr(self.combatList2Id)].Refresh(self.combatList2Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.combatList2Id)] = true;
        }
        else
        {
          let mut tsubpart28: SubPartClass =  new ATListSubPartClass(self.combatList2Obj, 3, 400, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 260, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num35, bby: num36);
          self.combatList2Id = self.AddSubPart( tsubpart28, num35, num36, 400, 64, 0);
        }
      }
      if (self.StatMode == 4)
      {
        let mut num37: i32 = 0;
        let mut sftyp5: i32 = self.sftyp;
        let mut num38: i32 = 190;
        let mut num39: i32 = 385;
        self.OptionsListObj = ATListClass::new();
        let mut preventCounter: i32 = self.game.Data.SFTypeObj[sftyp5].PreventCounter;
        for (let mut tdata: i32 = 0; tdata <= preventCounter; tdata += 1)
        {
          if (self.game.Data.SFTypeObj[sftyp5].PreventChance[tdata] > 0)
          {
            num37 += 1;
            if (num37 == 1)
              self.OptionsListObj.add("PREVENT HIT ON", -1, "WHEN ATT BY", "PRIORITY", "CHANCE", "PTS COST");
            str12 = "";
            tname: String = self.game.Data.SFTypeObj[sftyp5].PreventHitOn[tdata] <= -1 ? "ALL" : self.game.Data.TempString[400 + self.game.Data.SFTypeObj[sftyp5].PreventHitOn[tdata]];
            tvalue: String = self.game.Data.SFTypeObj[sftyp5].PreventHitFrom[tdata] <= -1 ? "ALL" : self.game.Data.TempString[400 + self.game.Data.SFTypeObj[sftyp5].PreventHitFrom[tdata]];
            tvalue2: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp5].PreventPriority[tdata]));
            tvalue3: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp5].PreventChance[tdata])) + "%";
            tvalue4: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[sftyp5].PreventPoints[tdata]));
            self.OptionsListObj.add(tname, tdata, tvalue, tvalue2, tvalue3, tvalue4);
          }
        }
        if (num37 > 0)
        {
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, -1);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            let mut tsubpart29: SubPartClass =  new ATListSubPartClass(self.OptionsListObj, 9, 600, -1, self.game, true, tHighlight: false, tShowPair: true, tValueWidth: 460, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num38, bby: num39);
            self.OptionsListId = self.AddSubPart( tsubpart29, num38, num39, 600, 160, 0);
          }
          num39 += 175;
        }
        let mut x21: i32 = 340;
         let mut local65: &Graphics = &Expression;
        rectangle2 = Rectangle::new(x21, num39, 300, 14);
        let mut rect1_9: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x21, num39 + 14, 300, 23);
        let mut rect2_9: &Rectangle = &rectangle1
        txt2_8: String = Conversions.ToString(self.game.Data.SFTypeObj[sftyp5].MaxPreventPointsUsed);
        DrawMod.MakeFullBoxVic2( local65, rect1_9, "MAX PREVENT PTS GIVEN TO OTHERS", rect2_9, txt2_8);
        let mut x22: i32 = 340;
        let mut y23: i32 = num39 + 55;
         let mut local66: &Graphics = &Expression;
        rectangle2 = Rectangle::new(x22, y23, 300, 14);
        let mut rect1_10: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x22, y23 + 14, 300, 23);
        let mut rect2_10: &Rectangle = &rectangle1
        txt2_9: String = Conversions.ToString(self.game.Data.SFTypeObj[sftyp5].MaxPreventPointsGiven);
        DrawMod.MakeFullBoxVic2( local66, rect1_10, "MAX PREVENT PTS RECEIVED BY PREVENTERS", rect2_10, txt2_9);
      }
      if (self.sfnr == -1 & self.sftyp == -1 && self.passenger > -1)
      {
        let mut tsubpart30: SubPartClass =  new UnitHeaderPartClass(self.passenger, self.game);
        self.TempText1 = self.AddSubPart( tsubpart30, 360, 150, 225, 200, 0);
        if (self.game.Data.UnitObj[self.passenger].Historical > -1 & self.game.Data.UnitObj[self.passenger].IsHQ)
        {
          if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.passenger].Historical].CommanderSpriteID > -1)
            ;
        }
        else
        {
          DrawMod.DrawBlock( Expression, 160, 380, 680, 240,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
          DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  Expression, 160, 380, 680, 240, -1, -1);
          let mut tsubpart31: SubPartClass =  new UnitSFPartClass(self.passenger, self.game);
          self.temptext4 = self.AddSubPart( tsubpart31, 190, 400, 620, 200, 0);
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub object ReturnSFSpriteNr(typ: i32, regnr: i32, pplnr: i32)
    {
      let mut symbolSpriteId: i32 = self.game.Data.SFTypeObj[typ].SymbolSpriteID;
      if (regnr > -1)
      {
        if (self.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index] == self.game.Data.RegimeObj[regnr].ExtraGraphicUse)
              symbolSpriteId = self.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
        else if (pplnr > -1 && self.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              symbolSpriteId = self.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
      }
      else if (self.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
      {
        let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
        for (let mut index: i32 = 0; index <= extraCounter; index += 1)
        {
          if (self.game.Data.SFTypeObj[typ].ExtraCode[index] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
            symbolSpriteId = self.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
        }
      }
      return  symbolSpriteId;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          self.game.EditObj.TempCoordList = CoordList::new();
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
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.tab1id)
            {
              self.StatMode = 0;
              self.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.tab2id)
            {
              self.StatMode = 1;
              self.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.tab3id)
            {
              self.StatMode = 2;
              self.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.tab4id)
            {
              self.StatMode = 3;
              self.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.tab5id)
            {
              self.StatMode = 4;
              self.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.quitid)
            {
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            num2: i32;
            if (num1 == self.OptionsListId)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.LogoListId)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.logolist2id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.logolist3id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList2Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList3Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.descid)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList4Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList5Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList6Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.combatListId)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.combatList2Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.but1id || num1 == self.but1bid)
            {
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.hqbut0)
            {
              self.HQselect = self.ChainHq[0];
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.hqbut1)
            {
              self.HQselect = self.ChainHq[1];
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.but7id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 74, -1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.hqbut2)
            {
              self.HQselect = self.ChainHq[2];
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.but2id)
            {
              if (Interaction.MsgBox( "Are you sure you want to disband this subformation?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                if (self.game.EditObj.SoundOn)
                  SoundMod.PlayAWave(self.game.AppPath + "sound/disband.wav",  self.game.EditObj);
                OrderResult orderResult = self.game.ProcessingObj.DoDisband(self.game.EditObj.UnitSelected, self.sfnr);
                if (orderResult.OK)
                {
                  if (orderResult.ErrorString.Length > 1)
                  {
                    let mut num3: i32 =  Interaction.MsgBox( orderResult.ErrorString, Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            else
            {
              if (num1 == self.sliderid)
              {
                self.detailnr2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but3id)
              {
                if (!self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ & self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFCount > 6 && self.game.Data.SFObj[self.sfnr].Qty != self.detailnr2)
                {
                  let mut num4: i32 =  Interaction.MsgBox( "You can only upgrade ALL because there is already 8 subformations in unit.");
                  return windowReturnClass;
                }
                OrderResult orderResult = self.game.ProcessingObj.DoUpgrade(self.game.EditObj.UnitSelected, self.sfnr, self.detailnr2, self.HQselect);
                if (self.game.EditObj.SoundOn)
                  SoundMod.PlayAWave(self.game.AppPath + "sound/building.wav",  self.game.EditObj);
                if (orderResult.OK)
                {
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
