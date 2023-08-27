// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabStatsWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TabStatsWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     subtabnr: i32;
     w: i32;
     h: i32;
     detailnr: i32;
     detailnr2: i32;
     detailnr3: i32;
     detailnr4: i32;
     detailhisnr: i32;
     CurrentView: i32;
     nextdetailnrhis: i32;
     prevdetailnrhis: i32;
     OptionsList5id: i32;
     optionslist6id: i32;
     optionslist7id: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Text4id: i32;
     Text5id: i32;
     text6id: i32;
     ListClass OptionsList5Obj;
     ListClass OptionsList6Obj;
     ListClass OptionsList7Obj;

    pub TabStatsWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.detailnr = -1;
      self.detailnr3 = 1;
      self.detailnr4 = 0;
      self.subtabnr = 0;
      self.detailnr2 = 0;
      self.detailhisnr = -1;
      if (self.game.EditObj.statsTab_tab > -1)
        self.subtabnr = self.game.EditObj.statsTab_tab;
      if (self.game.EditObj.statsTab_item > -1)
        self.detailnr = self.game.EditObj.statsTab_item;
      self.dostuff();
    }

    pub fn DoRefresh() => self.dostuff(true);

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (nr == 40)
      {
        if (self.nextdetailnrhis > -1)
        {
          self.detailhisnr = self.nextdetailnrhis;
          self.dostuff();
          windowReturnClass.SetFlag(true);
        }
        else
        {
          self.SubPartList[self.SubpartNr(self.OptionsList5id)].ShiftDown();
          self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
          self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsList5id)].GetSelect();
          self.dostuff();
          windowReturnClass.SetFlag(true);
        }
      }
      if (nr == 38)
      {
        if (self.prevdetailnrhis > -1)
        {
          self.detailhisnr = self.prevdetailnrhis;
          self.dostuff();
          windowReturnClass.SetFlag(true);
        }
        else
        {
          self.SubPartList[self.SubpartNr(self.OptionsList5id)].ShiftUp();
          self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
          self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsList5id)].GetSelect();
          self.dostuff();
          windowReturnClass.SetFlag(true);
        }
      }
      return windowReturnClass;
    }

    pub fn dostuff(bool IsRefresh = false)
    {
      numArray1: Vec<i32> = new int[self.game.Data.Round + 1, self.game.Data.RegimeCounter + 1 + 1];
      Color[] colorArray = new Color[self.game.Data.RegimeCounter + 1 + 1];
      strArray1: Vec<String> = new string[self.game.Data.RegimeCounter + 1 + 1];
      let mut num1: i32 = 150;
      self.nextdetailnrhis = -1;
      self.prevdetailnrhis = -1;
      self.RemoveSubPart(self.Info1Id);
      if (!IsRefresh)
      {
        self.RemoveSubPart(self.optionslist6id);
        self.optionslist6id = 0;
      }
      if (!IsRefresh)
      {
        self.RemoveSubPart(self.optionslist7id);
        self.optionslist7id = 0;
      }
      self.RemoveSubPart(self.Text1Id);
      self.RemoveSubPart(self.Text2Id);
      self.RemoveSubPart(self.Text3Id);
      self.RemoveSubPart(self.Text4id);
      self.RemoveSubPart(self.Text5id);
      self.RemoveSubPart(self.text6id);
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      self.ClearMouse();
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, "STATS", 2);
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F3]", 999);
      tregcount: i32;
      str1: String;
      SubPartClass tsubpart1;
      if (self.subtabnr == 0)
      {
        self.OptionsList5Obj = ListClass::new();
        let mut TempInt: i32 = -1;
        let mut num2: i32 = 0;
        self.OptionsList5Obj.add(" All", -2);
        if (self.detailnr == -1)
          TempInt = 0;
        let mut num3: i32 = 0;
        do
        {
          if (Information.IsNothing( self.game.Data.TempString[num3 + 400]))
            self.game.Data.TempString[num3 + 400] = "";
          if (self.game.Data.TempString[400 + num3].Length > 1 && self.game.HandyFunctionsObj.ShowStatsOfSFTypeGroup(num3))
          {
            num2 += 1;
            if (self.detailnr == num3)
              TempInt = num2;
            self.OptionsList5Obj.add("* " + self.game.Data.TempString[400 + num3] + " class", num3);
          }
          num3 += 1;
        }
        while (num3 <= 99);
        let mut sfTypeCounter1: i32 = self.game.Data.SFTypeCounter;
        for (let mut sfTypeNr: i32 = 0; sfTypeNr <= sfTypeCounter1; sfTypeNr += 1)
        {
          if (!self.game.Data.SFTypeObj[sfTypeNr].DontShowInList & self.game.Data.SFTypeObj[sfTypeNr].Name.Length > 1 & Strings.InStr(Strings.LCase(self.game.Data.SFTypeObj[sfTypeNr].Name), Strings.LCase("N/A")) <= 0 && self.game.HandyFunctionsObj.ShowStatsOfSFType(sfTypeNr))
          {
            num2 += 1;
            if (self.detailnr == sfTypeNr + 1000)
              TempInt = num2;
            self.OptionsList5Obj.add(self.game.Data.SFTypeObj[sfTypeNr].Name, sfTypeNr + 1000);
          }
        }
        let mut tlistselect1: i32 = self.OptionsList5Obj.SortWithRef(TempInt);
        let mut num4: i32 =  Math.Round( (self.h - 90) / 16.0) - 8;
        if (self.OptionsList5id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList5id)].Refresh(self.OptionsList5Obj, tlistselect1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
        }
        else if (self.game.Data.Product >= 7)
        {
          let mut tlistsize: i32 =  Math.Round( (self.h - 90) / 24.0) - 6;
          let mut tsubpart2: SubPartClass =  new ListSubPartClass(self.OptionsList5Obj, tlistsize, 170, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 30), bby: 10, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
          self.OptionsList5id = self.AddSubPart( tsubpart2, num1 + 30, 10, 170, (tlistsize + 1) * 24, 0);
        }
        else
        {
          ListClass optionsList5Obj = self.OptionsList5Obj;
          let mut tlistsize: i32 = num4;
          let mut tlistselect2: i32 = tlistselect1;
          let mut game: GameClass = self.game;
           local1: Bitmap =  self.OwnBitmap;
          let mut bbx: i32 = num1 + 30;
          font: Font =  null;
           local2: Font =  font;
          let mut tsubpart3: SubPartClass =  new ListSubPartClass(optionsList5Obj, tlistsize, 170, tlistselect2, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: 10, tMarcStyle: true, overruleFont: ( local2));
          self.OptionsList5id = self.AddSubPart( tsubpart3, num1 + 30, 10, 170, (num4 + 1) * 16, 0);
        }
        let mut tMultiplier: i32 = 100;
        bool[] flagArray1 = new bool[self.game.Data.RegimeCounter + 1];
        let mut regimeCounter1: i32 = self.game.Data.RegimeCounter;
        for (let mut regSlot: i32 = 0; regSlot <= regimeCounter1; regSlot += 1)
        {
          if (!self.game.Data.RegimeObj[regSlot].hideFromList && self.game.HandyFunctionsObj.ShowStatsOfRegime(regSlot))
          {
            if (self.game.Data.Product < 7 && !self.game.Data.RegimeObj[regSlot].Sleep)
              flagArray1[regSlot] = true;
            let mut round: i32 = self.game.Data.Round;
            for (let mut index1: i32 = 1; index1 <= round; index1 += 1)
            {
              let mut sfTypeCounter2: i32 = self.game.Data.SFTypeCounter;
              for (let mut index2: i32 = 0; index2 <= sfTypeCounter2; index2 += 1)
              {
                if (self.game.Data.RegimeObj[regSlot].SLoss[index2, index1] > 0)
                  flagArray1[regSlot] = true;
                if (self.game.Data.RegimeObj[regSlot].SKills[index2, index1] > 0)
                  flagArray1[regSlot] = true;
                if (self.game.Data.RegimeObj[regSlot].SPresent[index2, index1] > 0)
                  flagArray1[regSlot] = true;
              }
            }
          }
          if (self.game.Data.Winner > -1 & self.game.Data.Product >= 7 && !self.game.Data.RegimeObj[regSlot].hideFromList)
            flagArray1[regSlot] = true;
        }
        bool[] flagArray2 = new bool[self.game.Data.SFTypeCounter + 1];
        let mut sfTypeCounter3: i32 = self.game.Data.SFTypeCounter;
        for (let mut sfTypeNr: i32 = 0; sfTypeNr <= sfTypeCounter3; sfTypeNr += 1)
        {
          if (self.game.HandyFunctionsObj.ShowStatsOfSFType(sfTypeNr))
            flagArray2[sfTypeNr] = true;
        }
        let mut round1: i32 = self.game.Data.Round;
        num5: i32;
        num6: i32;
        for (let mut index3: i32 = 0; index3 <= round1; index3 += 1)
        {
          tregcount = 0;
          let mut regimeCounter2: i32 = self.game.Data.RegimeCounter;
          for (let mut reg2: i32 = 0; reg2 <= regimeCounter2; reg2 += 1)
          {
            if (!self.game.Data.RegimeObj[reg2].hideFromList && flagArray1[reg2] && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, reg2) | !self.game.Data.FOWOn | self.game.Data.Winner > -1 |  self.game.Data.RuleVar[313] > 0.0)
            {
              tregcount += 1;
              numArray1[index3, tregcount] = 0;
              colorArray[tregcount] = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[reg2].Red, self.game.Data.RegimeObj[reg2].Green, self.game.Data.RegimeObj[reg2].Blue);
              strArray1[tregcount] = self.game.Data.RegimeObj[reg2].Name;
              let mut sfTypeCounter4: i32 = self.game.Data.SFTypeCounter;
              for (let mut index4: i32 = 0; index4 <= sfTypeCounter4; index4 += 1)
              {
                if (flagArray2[index4])
                {
                  if (self.detailnr == -1)
                  {
                    str1 = "Total troops";
                    if (self.detailnr2 == 0)
                    {
                      numArray2: Vec<i32> = numArray1;
                      numArray3: Vec<i32> = numArray2;
                      let mut index5: i32 = index3;
                      let mut index6: i32 = index5;
                      let mut index7: i32 = tregcount;
                      let mut index8: i32 = index7;
                      let mut num7: i32 = numArray2[index5, index7] + self.game.Data.RegimeObj[reg2].SPresent[index4, index3] * self.game.Data.SFTypeObj[index4].Ratio;
                      numArray3[index6, index8] = num7;
                    }
                    if (self.detailnr2 > 0 & index3 > 0)
                    {
                      let mut num8: i32 = index3;
                      for (let mut index9: i32 = 1; index9 <= num8; index9 += 1)
                      {
                        if (self.detailnr2 == 1)
                        {
                          numArray4: Vec<i32> = numArray1;
                          numArray5: Vec<i32> = numArray4;
                          let mut index10: i32 = index3;
                          let mut index11: i32 = index10;
                          let mut index12: i32 = tregcount;
                          let mut index13: i32 = index12;
                          let mut num9: i32 = numArray4[index10, index12] + self.game.Data.RegimeObj[reg2].SKills[index4, index9] * self.game.Data.SFTypeObj[index4].Ratio;
                          numArray5[index11, index13] = num9;
                        }
                        if (self.detailnr2 == 2)
                        {
                          numArray6: Vec<i32> = numArray1;
                          numArray7: Vec<i32> = numArray6;
                          let mut index14: i32 = index3;
                          let mut index15: i32 = index14;
                          let mut index16: i32 = tregcount;
                          let mut index17: i32 = index16;
                          let mut num10: i32 = numArray6[index14, index16] + self.game.Data.RegimeObj[reg2].SLoss[index4, index9] * self.game.Data.SFTypeObj[index4].Ratio;
                          numArray7[index15, index17] = num10;
                        }
                      }
                    }
                    tMultiplier = 1;
                  }
                  else if (self.detailnr < 1000)
                  {
                    str1 = "Total " + Strings.LCase(self.game.Data.TempString[400 + self.detailnr]) + " class";
                    if (self.game.Data.SFTypeObj[index4].UnitGroup == self.detailnr)
                    {
                      if (self.detailnr2 == 0)
                      {
                        numArray8: Vec<i32> = numArray1;
                        numArray9: Vec<i32> = numArray8;
                        let mut index18: i32 = index3;
                        let mut index19: i32 = index18;
                        let mut index20: i32 = tregcount;
                        let mut index21: i32 = index20;
                        let mut num11: i32 = numArray8[index18, index20] + self.game.Data.RegimeObj[reg2].SPresent[index4, index3];
                        numArray9[index19, index21] = num11;
                      }
                      if (self.detailnr2 > 0 & index3 > 0)
                      {
                        let mut num12: i32 = index3;
                        for (let mut index22: i32 = 1; index22 <= num12; index22 += 1)
                        {
                          if (self.detailnr2 == 1)
                          {
                            numArray10: Vec<i32> = numArray1;
                            numArray11: Vec<i32> = numArray10;
                            let mut index23: i32 = index3;
                            let mut index24: i32 = index23;
                            let mut index25: i32 = tregcount;
                            let mut index26: i32 = index25;
                            let mut num13: i32 = numArray10[index23, index25] + self.game.Data.RegimeObj[reg2].SKills[index4, index22];
                            numArray11[index24, index26] = num13;
                          }
                          if (self.detailnr2 == 2)
                          {
                            numArray12: Vec<i32> = numArray1;
                            numArray13: Vec<i32> = numArray12;
                            let mut index27: i32 = index3;
                            let mut index28: i32 = index27;
                            let mut index29: i32 = tregcount;
                            let mut index30: i32 = index29;
                            let mut num14: i32 = numArray12[index27, index29] + self.game.Data.RegimeObj[reg2].SLoss[index4, index22];
                            numArray13[index28, index30] = num14;
                          }
                          if (self.detailnr2 == 1)
                          {
                            num5 += self.game.Data.RegimeObj[reg2].SKills[index4, index3] * self.game.Data.SFTypeObj[index4].Ratio;
                            num6 += self.game.Data.RegimeObj[reg2].SKills[index4, index3];
                          }
                          if (self.detailnr2 == 2)
                          {
                            num5 += self.game.Data.RegimeObj[reg2].SLoss[index4, index3] * self.game.Data.SFTypeObj[index4].Ratio;
                            num6 += self.game.Data.RegimeObj[reg2].SLoss[index4, index3];
                          }
                        }
                      }
                      if (self.detailnr2 == 0)
                      {
                        num5 += self.game.Data.RegimeObj[reg2].SPresent[index4, index3] * self.game.Data.SFTypeObj[index4].Ratio;
                        num6 += self.game.Data.RegimeObj[reg2].SPresent[index4, index3];
                      }
                    }
                  }
                  else if (self.detailnr >= 1000 && index4 == self.detailnr - 1000)
                  {
                    str1 = "Total " + Strings.LCase(self.game.Data.SFTypeObj[index4].Name);
                    if (self.detailnr2 == 0)
                    {
                      numArray14: Vec<i32> = numArray1;
                      numArray15: Vec<i32> = numArray14;
                      let mut index31: i32 = index3;
                      let mut index32: i32 = index31;
                      let mut index33: i32 = tregcount;
                      let mut index34: i32 = index33;
                      let mut num15: i32 = numArray14[index31, index33] + self.game.Data.RegimeObj[reg2].SPresent[index4, index3];
                      numArray15[index32, index34] = num15;
                    }
                    if (self.detailnr2 > 0 & index3 > 0)
                    {
                      let mut num16: i32 = index3;
                      for (let mut index35: i32 = 1; index35 <= num16; index35 += 1)
                      {
                        if (self.detailnr2 == 1)
                        {
                          numArray16: Vec<i32> = numArray1;
                          numArray17: Vec<i32> = numArray16;
                          let mut index36: i32 = index3;
                          let mut index37: i32 = index36;
                          let mut index38: i32 = tregcount;
                          let mut index39: i32 = index38;
                          let mut num17: i32 = numArray16[index36, index38] + self.game.Data.RegimeObj[reg2].SKills[index4, index35];
                          numArray17[index37, index39] = num17;
                        }
                        if (self.detailnr2 == 2)
                        {
                          numArray18: Vec<i32> = numArray1;
                          numArray19: Vec<i32> = numArray18;
                          let mut index40: i32 = index3;
                          let mut index41: i32 = index40;
                          let mut index42: i32 = tregcount;
                          let mut index43: i32 = index42;
                          let mut num18: i32 = numArray18[index40, index42] + self.game.Data.RegimeObj[reg2].SLoss[index4, index35];
                          numArray19[index41, index43] = num18;
                        }
                      }
                    }
                    tMultiplier = self.game.Data.SFTypeObj[index4].Ratio;
                  }
                }
              }
            }
          }
        }
        if (num5 > 0)
          tMultiplier =  Math.Round(Conversion.Int( num5 /  num6));
        tsubpart1 =  new GraphClass(self.game, numArray1, self.detailnr3 == 1, str1, tMultiplier, colorArray, strArray1, tregcount, self.w - 880 + 620 - num1, self.h - 80,  self.OwnBitmap, num1 + 220, 0);
        self.Info1Id = self.AddSubPart( tsubpart1, num1 + 220, 0, self.w - 880 + 620 - num1, 330, 0);
        tsubpart1 =  new MarcRadioPartClass(0, self.detailnr2 == 0, "Troops at start of round",  self.OwnBitmap, 30 + num1, self.h - 160);
        self.Text1Id = self.AddSubPart( tsubpart1, 30 + num1, self.h - 160, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( g, "Total troops", self.game.MarcFont4, 75 + num1, self.h - 150, Color.White);
        if ( self.game.Data.RuleVar[976] == 0.0 | self.game.Data.RegimeCounter > 1)
        {
          tsubpart1 =  new MarcRadioPartClass(0, self.detailnr2 == 1, "How much of selected type of troops did each regime manage to take out.",  self.OwnBitmap, 30 + num1, 300);
          self.Text2Id = self.AddSubPart( tsubpart1, 30 + num1, self.h - 80, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "Casualties inflicted", self.game.MarcFont4, 75 + num1, self.h - 70, Color.White);
        }
        tsubpart1 =  new MarcRadioPartClass(0, self.detailnr2 == 2, "How much of selected type of troops did each regime lose.",  self.OwnBitmap, 30 + num1, 260);
        self.Text3Id = self.AddSubPart( tsubpart1, 30 + num1, self.h - 120, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( g, "Casualties sustained", self.game.MarcFont4, 75 + num1, self.h - 110, Color.White);
      }
      str2: String;
      num19: i32;
      if (self.subtabnr == 2)
      {
        tsubpart1 =  new MarcRadioPartClass(0, self.detailnr4 == 1, "Only Show HQs",  self.OwnBitmap, 30 + num1, self.h - 160);
        self.Text5id = self.AddSubPart( tsubpart1, 30 + num1, self.h - 160, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( g, "Only Show HQs", self.game.MarcFont4, 75 + num1, self.h - 150, Color.White);
        self.OptionsList5Obj = ListClass::new();
        let mut tlistselect3: i32 = -1;
        let mut num20: i32 = -1;
        let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[404]));
        let mut num21: i32 =  Math.Round(Conversion.Val( self.game.Data.StringListObj[stringListById].GetHighestValue(0)));
        object[] objArray1 = new object[num21 + 1];
        object[] objArray2 = new object[num21 + 1];
        object[] objArray3 = new object[num21 + 1];
        object[,] objArray4 = new object[self.game.Data.LocCounter + 1, num21 + 1];
        object[,] objArray5 = new object[self.game.Data.LocCounter + 1, num21 + 1];
        object[,] objArray6 = new object[self.game.Data.UnitCounter + 1, num21 + 1];
        object[,] objArray7 = new object[self.game.Data.UnitCounter + 1, num21 + 1];
        object[,] objArray8 = new object[self.game.Data.LocCounter + 1, num21 + 1];
        object[,] objArray9 = new object[self.game.Data.LocCounter + 1, num21 + 1];
        object[] objArray10 = new object[self.game.Data.LocCounter + 1];
        object[] objArray11 = new object[self.game.Data.UnitCounter + 1];
        object[] objArray12 = new object[self.game.Data.LocCounter + 1];
        object[] objArray13 = new object[self.game.Data.UnitCounter + 1];
        let mut num22: i32 = num21;
        for (let mut index: i32 = 0; index <= num22; index += 1)
        {
          objArray1[index] =  0;
          objArray2[index] =  0;
          objArray3[index] =  0;
        }
        let mut unitCounter1: i32 = self.game.Data.UnitCounter;
        for (let mut index44: i32 = 0; index44 <= unitCounter1; index44 += 1)
        {
          let mut num23: i32 = num21;
          for (let mut index45: i32 = 0; index45 <= num23; index45 += 1)
          {
            objArray6[index44, index45] =  0;
            objArray7[index44, index45] =  0;
          }
        }
        let mut locCounter1: i32 = self.game.Data.LocCounter;
        for (let mut index46: i32 = 0; index46 <= locCounter1; index46 += 1)
        {
          objArray10[index46] =  0;
          objArray11[index46] =  0;
          objArray12[index46] =  0;
          objArray13[index46] =  0;
          let mut num24: i32 = num21;
          for (let mut index47: i32 = 0; index47 <= num24; index47 += 1)
          {
            objArray4[index46, index47] =  0;
            objArray5[index46, index47] =  0;
            objArray8[index46, index47] =  0;
            objArray9[index46, index47] =  0;
          }
        }
        let mut num25: i32 = 0;
        let mut num26: i32 = 0;
        let mut num27: i32 = 0;
        let mut unitCounter2: i32 = self.game.Data.UnitCounter;
        for (let mut index48: i32 = 0; index48 <= unitCounter2; index48 += 1)
        {
          if (self.game.Data.UnitObj[index48].Regime == self.game.Data.Turn & self.game.Data.UnitObj[index48].PreDef == -1 & self.game.Data.UnitObj[index48].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index48].Historical].Type < 8)
          {
            let mut logCounter: i32 = self.game.Data.UnitObj[index48].LogCounter;
            for (let mut index49: i32 = 0; index49 <= logCounter; index49 += 1)
            {
              if (self.game.Data.UnitObj[index48].LogData1[index49] > 0)
              {
                if (self.game.Data.UnitObj[index48].LogType[index49] == 202)
                {
                  num26 += 1;
                  object[] objArray14 = objArray2;
                  object[] objArray15 = objArray14;
                  int[] logData1_1 = self.game.Data.UnitObj[index48].LogData1;
                  int[] numArray20 = logData1_1;
                  let mut index50: i32 = index49;
                  let mut index51: i32 = index50;
                  let mut index52: i32 = numArray20[index51];
                  object obj1 = Operators.AddObject(objArray14[logData1_1[index50]],  self.game.Data.UnitObj[index48].LogData3[index49]);
                  objArray15[index52] = obj1;
                  object[,] objArray16 = objArray6;
                  object[,] objArray17 = objArray16;
                  let mut index53: i32 = index48;
                  let mut index54: i32 = index53;
                  int[] logData1_2 = self.game.Data.UnitObj[index48].LogData1;
                  int[] numArray21 = logData1_2;
                  let mut index55: i32 = index49;
                  let mut index56: i32 = index55;
                  let mut index57: i32 = numArray21[index56];
                  object obj2 = Operators.AddObject(objArray16[index53, logData1_2[index55]],  self.game.Data.UnitObj[index48].LogData3[index49]);
                  objArray17[index54, index57] = obj2;
                }
                if (self.game.Data.UnitObj[index48].LogType[index49] == 105)
                {
                  num26 += 1;
                  object[] objArray18 = objArray2;
                  object[] objArray19 = objArray18;
                  int[] logData1_3 = self.game.Data.UnitObj[index48].LogData1;
                  int[] numArray22 = logData1_3;
                  let mut index58: i32 = index49;
                  let mut index59: i32 = index58;
                  let mut index60: i32 = numArray22[index59];
                  object obj3 = Operators.AddObject(objArray18[logData1_3[index58]],  self.game.Data.UnitObj[index48].LogData3[index49]);
                  objArray19[index60] = obj3;
                  object[,] objArray20 = objArray7;
                  object[,] objArray21 = objArray20;
                  let mut index61: i32 = index48;
                  let mut index62: i32 = index61;
                  int[] logData1_4 = self.game.Data.UnitObj[index48].LogData1;
                  int[] numArray23 = logData1_4;
                  let mut index63: i32 = index49;
                  let mut index64: i32 = index63;
                  let mut index65: i32 = numArray23[index64];
                  object obj4 = Operators.AddObject(objArray20[index61, logData1_4[index63]],  self.game.Data.UnitObj[index48].LogData3[index49]);
                  objArray21[index62, index65] = obj4;
                }
                if (self.game.Data.UnitObj[index48].LogType[index49] == 503)
                {
                  object[] objArray22 = objArray11;
                  object[] objArray23 = objArray22;
                  let mut index66: i32 = index48;
                  let mut index67: i32 = index66;
                  object obj = Operators.AddObject(objArray22[index66],  self.game.Data.UnitObj[index48].LogData3[index49]);
                  objArray23[index67] = obj;
                }
              }
              if (self.game.Data.UnitObj[index48].LogType[index49] == 506)
              {
                object[] objArray24 = objArray13;
                object[] objArray25 = objArray24;
                let mut index68: i32 = index48;
                let mut index69: i32 = index68;
                object obj = Operators.AddObject(objArray24[index68],  self.game.Data.UnitObj[index48].LogData3[index49]);
                objArray25[index69] = obj;
              }
            }
          }
        }
        let mut locCounter2: i32 = self.game.Data.LocCounter;
        for (let mut index70: i32 = 0; index70 <= locCounter2; index70 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index70].X, self.game.Data.LocObj[index70].Y].Regime == self.game.Data.Turn)
          {
            let mut logCounter: i32 = self.game.Data.LocObj[index70].LogCounter;
            for (let mut index71: i32 = 0; index71 <= logCounter; index71 += 1)
            {
              if (self.game.Data.LocObj[index70].LogData1[index71] > 0)
              {
                if (self.game.Data.LocObj[index70].LogType[index71] == 202)
                {
                  num27 += 1;
                  object[] objArray26 = objArray3;
                  object[] objArray27 = objArray26;
                  int[] logData1_5 = self.game.Data.LocObj[index70].LogData1;
                  int[] numArray24 = logData1_5;
                  let mut index72: i32 = index71;
                  let mut index73: i32 = index72;
                  let mut index74: i32 = numArray24[index73];
                  object obj5 = Operators.AddObject(objArray26[logData1_5[index72]],  self.game.Data.LocObj[index70].LogData3[index71]);
                  objArray27[index74] = obj5;
                  object[,] objArray28 = objArray8;
                  object[,] objArray29 = objArray28;
                  let mut index75: i32 = index70;
                  let mut index76: i32 = index75;
                  int[] logData1_6 = self.game.Data.LocObj[index70].LogData1;
                  int[] numArray25 = logData1_6;
                  let mut index77: i32 = index71;
                  let mut index78: i32 = index77;
                  let mut index79: i32 = numArray25[index78];
                  object obj6 = Operators.AddObject(objArray28[index75, logData1_6[index77]],  self.game.Data.LocObj[index70].LogData3[index71]);
                  objArray29[index76, index79] = obj6;
                }
                if (self.game.Data.LocObj[index70].LogType[index71] == 102)
                {
                  num27 += 1;
                  object[] objArray30 = objArray3;
                  object[] objArray31 = objArray30;
                  int[] logData1_7 = self.game.Data.LocObj[index70].LogData1;
                  int[] numArray26 = logData1_7;
                  let mut index80: i32 = index71;
                  let mut index81: i32 = index80;
                  let mut index82: i32 = numArray26[index81];
                  object obj7 = Operators.AddObject(objArray30[logData1_7[index80]],  self.game.Data.LocObj[index70].LogData3[index71]);
                  objArray31[index82] = obj7;
                  object[,] objArray32 = objArray9;
                  object[,] objArray33 = objArray32;
                  let mut index83: i32 = index70;
                  let mut index84: i32 = index83;
                  int[] logData1_8 = self.game.Data.LocObj[index70].LogData1;
                  int[] numArray27 = logData1_8;
                  let mut index85: i32 = index71;
                  let mut index86: i32 = index85;
                  let mut index87: i32 = numArray27[index86];
                  object obj8 = Operators.AddObject(objArray32[index83, logData1_8[index85]],  self.game.Data.LocObj[index70].LogData3[index71]);
                  objArray33[index84, index87] = obj8;
                }
                if (self.game.Data.LocObj[index70].LogType[index71] == 505)
                {
                  object[] objArray34 = objArray12;
                  object[] objArray35 = objArray34;
                  let mut index88: i32 = index70;
                  let mut index89: i32 = index88;
                  object obj = Operators.AddObject(objArray34[index88],  self.game.Data.LocObj[index70].LogData3[index71]);
                  objArray35[index89] = obj;
                }
              }
            }
          }
        }
        let mut locCounter3: i32 = self.game.Data.LocCounter;
        for (let mut index90: i32 = 0; index90 <= locCounter3; index90 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index90].X, self.game.Data.LocObj[index90].Y].Regime == self.game.Data.Turn)
          {
            let mut logCounter: i32 = self.game.Data.LocObj[index90].LogCounter;
            for (let mut index91: i32 = 0; index91 <= logCounter; index91 += 1)
            {
              if (self.game.Data.LocObj[index90].LogData1[index91] > 0)
              {
                if (self.game.Data.LocObj[index90].LogType[index91] == 201)
                {
                  num25 += 1;
                  object[] objArray36 = objArray1;
                  object[] objArray37 = objArray36;
                  int[] logData1_9 = self.game.Data.LocObj[index90].LogData1;
                  int[] numArray28 = logData1_9;
                  let mut index92: i32 = index91;
                  let mut index93: i32 = index92;
                  let mut index94: i32 = numArray28[index93];
                  object obj9 = Operators.AddObject(objArray36[logData1_9[index92]],  self.game.Data.LocObj[index90].LogData3[index91]);
                  objArray37[index94] = obj9;
                  object[,] objArray38 = objArray4;
                  object[,] objArray39 = objArray38;
                  let mut index95: i32 = index90;
                  let mut index96: i32 = index95;
                  int[] logData1_10 = self.game.Data.LocObj[index90].LogData1;
                  int[] numArray29 = logData1_10;
                  let mut index97: i32 = index91;
                  let mut index98: i32 = index97;
                  let mut index99: i32 = numArray29[index98];
                  object obj10 = Operators.AddObject(objArray38[index95, logData1_10[index97]],  self.game.Data.LocObj[index90].LogData3[index91]);
                  objArray39[index96, index99] = obj10;
                }
                if (self.game.Data.LocObj[index90].LogType[index91] == 101)
                {
                  num25 += 1;
                  object[] objArray40 = objArray1;
                  object[] objArray41 = objArray40;
                  int[] logData1_11 = self.game.Data.LocObj[index90].LogData1;
                  int[] numArray30 = logData1_11;
                  let mut index100: i32 = index91;
                  let mut index101: i32 = index100;
                  let mut index102: i32 = numArray30[index101];
                  object obj11 = Operators.AddObject(objArray40[logData1_11[index100]],  self.game.Data.LocObj[index90].LogData3[index91]);
                  objArray41[index102] = obj11;
                  object[,] objArray42 = objArray5;
                  object[,] objArray43 = objArray42;
                  let mut index103: i32 = index90;
                  let mut index104: i32 = index103;
                  int[] logData1_12 = self.game.Data.LocObj[index90].LogData1;
                  int[] numArray31 = logData1_12;
                  let mut index105: i32 = index91;
                  let mut index106: i32 = index105;
                  let mut index107: i32 = numArray31[index106];
                  object obj12 = Operators.AddObject(objArray42[index103, logData1_12[index105]],  self.game.Data.LocObj[index90].LogData3[index91]);
                  objArray43[index104, index107] = obj12;
                }
                if (self.game.Data.LocObj[index90].LogType[index91] == 502)
                {
                  object[] objArray44 = objArray10;
                  object[] objArray45 = objArray44;
                  let mut index108: i32 = index90;
                  let mut index109: i32 = index108;
                  object obj = Operators.AddObject(objArray44[index108],  self.game.Data.LocObj[index90].LogData3[index91]);
                  objArray45[index109] = obj;
                }
              }
            }
          }
        }
        if (num25 > 0)
        {
          num20 += 1;
          if (self.detailnr == 200)
            tlistselect3 = num20;
          self.OptionsList5Obj.add("=== Loc Supply Logistics ===", 200);
          let mut num28: i32 = num21;
          for (let mut idValue: i32 = 0; idValue <= num28; idValue += 1)
          {
            if (Operators.ConditionalCompareObjectGreater(objArray1[idValue],  0, false))
            {
              str2 = self.game.Data.StringListObj[stringListById].GetData(0, idValue, 2);
              num20 += 1;
              if (self.detailnr == 200 + idValue)
                tlistselect3 = num20;
              self.OptionsList5Obj.add("Loc " + str2 + " Supply", 200 + idValue);
            }
          }
        }
        if (num26 > 0)
        {
          num20 += 1;
          if (self.detailnr == 300)
            tlistselect3 = num20;
          self.OptionsList5Obj.add("=== Unit Supply Logistics ===", 300);
          let mut num29: i32 = num21;
          for (let mut idValue: i32 = 0; idValue <= num29; idValue += 1)
          {
            if (Operators.ConditionalCompareObjectGreater(objArray2[idValue],  0, false))
            {
              str2 = self.game.Data.StringListObj[stringListById].GetData(0, idValue, 2);
              num20 += 1;
              if (self.detailnr == 300 + idValue)
                tlistselect3 = num20;
              self.OptionsList5Obj.add("Unit " + str2 + " Supply", 300 + idValue);
            }
          }
        }
        if (num27 > 0)
        {
          num20 += 1;
          if (self.detailnr == 500)
            tlistselect3 = num20;
          self.OptionsList5Obj.add("=== Lock pick-up Logistics ===", 500);
          let mut num30: i32 = num21;
          for (let mut idValue: i32 = 0; idValue <= num30; idValue += 1)
          {
            if (Operators.ConditionalCompareObjectGreater(objArray3[idValue],  0, false))
            {
              str2 = self.game.Data.StringListObj[stringListById].GetData(0, idValue, 2);
              num20 += 1;
              if (self.detailnr == 500 + idValue)
                tlistselect3 = num20;
              self.OptionsList5Obj.add("Loc " + str2 + " Pickup", 500 + idValue);
            }
          }
        }
        if ( self.game.Data.RuleVar[337] > 0.0)
        {
          let mut num31: i32 = num20 + 1;
          if (self.detailnr == 600)
            tlistselect3 = num31;
          self.OptionsList5Obj.add("=== Replacements Logistics ===", 600);
          let mut num32: i32 = num31 + 1;
          if (self.detailnr == -1)
            self.detailnr = 11;
          if (self.detailnr == 11)
            tlistselect3 = num32;
          self.OptionsList5Obj.add("Replacements received", 11);
          num20 = num32 + 1;
          if (self.detailnr == 12)
            tlistselect3 = num20;
          self.OptionsList5Obj.add("Replacements requested", 12);
          if ( self.game.Data.RuleVar[977] < 1.0)
          {
            num20 += 1;
            if (self.detailnr == 13)
              tlistselect3 = num20;
            self.OptionsList5Obj.add("Replacements returned", 13);
          }
        }
        if (num20 > -1)
        {
          let mut num33: i32 =  Math.Round( (self.h - 90) / 16.0) - 8;
          if (self.OptionsList5id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList5id)].Refresh(self.OptionsList5Obj, tlistselect3);
            self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
          }
          else if (self.game.Data.Product >= 7)
          {
            let mut tlistsize: i32 =  Math.Round( (self.h - 90) / 24.0) - 6;
            tsubpart1 =  new ListSubPartClass(self.OptionsList5Obj, tlistsize, 170, tlistselect3, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 30), bby: 10, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
            self.OptionsList5id = self.AddSubPart( tsubpart1, num1 + 30, 10, 170, (tlistsize + 1) * 24, 0);
          }
          else
          {
            ListClass optionsList5Obj = self.OptionsList5Obj;
            let mut tlistsize: i32 = num33;
            let mut tlistselect4: i32 = tlistselect3;
            let mut game: GameClass = self.game;
             local3: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 30;
            font: Font =  null;
             local4: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList5Obj, tlistsize, 170, tlistselect4, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( local3), bbx: bbx, bby: 10, tMarcStyle: true, overruleFont: ( local4));
            self.OptionsList5id = self.AddSubPart( tsubpart1, num1 + 30, 10, 170, (num33 + 1) * 16, 0);
          }
        }
        if (self.detailnr == 200 | self.detailnr == 500)
        {
          self.OptionsList6Obj = ListClass::new();
          let mut tlistselect5: i32 = -1;
          str3: String = "---";
          let mut num34: i32 = -1;
          let mut unitCounter3: i32 = self.game.Data.UnitCounter;
          for (let mut tdata: i32 = 0; tdata <= unitCounter3; tdata += 1)
          {
            if (self.detailnr4 < 1 | self.game.Data.UnitObj[tdata].IsHQ && self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[tdata].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[tdata].PreDef == -1 & self.game.Data.UnitObj[tdata].Historical > -1 && self.game.Data.UnitObj[tdata].HQ == -1 & self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 8)
            {
              num34 += 1;
              if (self.detailhisnr == -1)
                self.detailhisnr = tdata;
              if (self.detailhisnr == tdata)
                tlistselect5 = num34;
              self.OptionsList6Obj.add(self.game.Data.UnitObj[tdata].Name, tdata, tWeight: 0);
              let mut locCounter4: i32 = self.game.Data.LocCounter;
              for (let mut index: i32 = 0; index <= locCounter4; index += 1)
              {
                if (self.game.Data.LocObj[index].HQ == tdata)
                {
                  if (self.detailnr == 200)
                    str2 = objArray10[index].ToString();
                  else if (self.detailnr == 500)
                    str2 = objArray12[index].ToString();
                  num34 += 1;
                  if (self.detailhisnr == -1)
                    self.detailhisnr = index + 1000000;
                  if (self.detailhisnr == index + 1000000)
                    tlistselect5 = num34;
                  self.OptionsList6Obj.add(str3 + self.game.Data.LocObj[index].Name, index + 1000000, str2, tWeight: 0);
                }
              }
            }
          }
          if (num34 == -1)
          {
            self.RemoveSubPart(self.optionslist6id);
            self.optionslist6id = 0;
          }
          let mut num35: i32 =  Math.Round( (self.h - 80) / 16.0) - 1;
          if (self.optionslist6id > 0)
          {
            self.SubPartList[self.SubpartNr(self.optionslist6id)].Refresh(self.OptionsList6Obj, tlistselect5);
            self.SubPartFlag[self.SubpartNr(self.optionslist6id)] = true;
          }
          else if (num34 > -1)
          {
            ListClass optionsList6Obj = self.OptionsList6Obj;
            let mut tlistsize: i32 = num35;
            let mut twidth: i32 = 320 +  Math.Round( (self.w - 880) / 2.0);
            let mut tlistselect6: i32 = tlistselect5;
            let mut game: GameClass = self.game;
            let mut tValueWidth: i32 = 100 +  Math.Round( (self.w - 880) / 5.0);
             local5: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 220;
            font: Font =  null;
             local6: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList6Obj, tlistsize, twidth, tlistselect6, game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local5), bbx: bbx, bby: 16, tMarcStyle: true, overruleFont: ( local6));
            self.optionslist6id = self.AddSubPart( tsubpart1, num1 + 220, 16, 320 +  Math.Round( (self.w - 880) / 2.0), (num35 + 1) * 16, 0);
          }
          else
          {
            str2 = "No locations or SHQ";
            DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont2, num1 + 260, 90, Color.FromArgb(177,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
          }
          if (num34 > -1)
          {
            let mut x1: i32 = num1 + 220 + 10;
            DrawMod.DrawTextColouredMarc( g, "SHQ / LOCATION", self.game.MarcFont5, x1, 3, Color.White);
            let mut x2: i32 = num1 + 220 + 320 +  Math.Round( (self.w - 880) / 2.0) -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 1.0);
            upper: String = "Logistical Points used".ToUpper();
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont5, x2, 3, Color.White);
          }
        }
        if (self.detailnr >= 201 & self.detailnr <= 299)
        {
          self.OptionsList6Obj = ListClass::new();
          let mut tlistselect7: i32 = -1;
          str4: String = "---";
          let mut idValue: i32 = self.detailnr - 200;
          let mut num36: i32 = -1;
          let mut unitCounter4: i32 = self.game.Data.UnitCounter;
          for (let mut tdata: i32 = 0; tdata <= unitCounter4; tdata += 1)
          {
            if (self.detailnr4 < 1 | self.game.Data.UnitObj[tdata].IsHQ && self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[tdata].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[tdata].PreDef == -1 & self.game.Data.UnitObj[tdata].Historical > -1 && self.game.Data.UnitObj[tdata].HQ == -1 & self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 8)
            {
              num36 += 1;
              if (self.detailhisnr == -1)
                self.detailhisnr = tdata;
              if (self.detailhisnr == tdata)
                tlistselect7 = num36;
              self.OptionsList6Obj.add(self.game.Data.UnitObj[tdata].Name, tdata, tWeight: 0);
              let mut locCounter5: i32 = self.game.Data.LocCounter;
              for (let mut index: i32 = 0; index <= locCounter5; index += 1)
              {
                if (self.game.Data.LocObj[index].HQ == tdata && Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectGreater(objArray4[index, idValue],  0, false), Operators.CompareObjectGreater(objArray4[index, idValue],  0, false))))
                {
                  tvalue2: String = objArray4[index, idValue].ToString();
                  tvalue: String = objArray5[index, idValue].ToString();
                  num36 += 1;
                  if (self.detailhisnr == -1)
                    self.detailhisnr = index + 1000000;
                  if (self.detailhisnr == index + 1000000)
                    tlistselect7 = num36;
                  self.OptionsList6Obj.add(str4 + self.game.Data.LocObj[index].Name, index + 1000000, tvalue, tvalue2, tWeight: 0);
                }
              }
            }
          }
          if (num36 == -1)
          {
            self.RemoveSubPart(self.optionslist6id);
            self.optionslist6id = 0;
          }
          let mut num37: i32 =  Math.Round( (self.h - 80) / 16.0) - 1;
          if (self.optionslist6id > 0)
          {
            self.SubPartList[self.SubpartNr(self.optionslist6id)].Refresh(self.OptionsList6Obj, tlistselect7);
            self.SubPartFlag[self.SubpartNr(self.optionslist6id)] = true;
          }
          else if (num36 > -1)
          {
            ListClass optionsList6Obj = self.OptionsList6Obj;
            let mut tlistsize: i32 = num37;
            let mut twidth: i32 = 320 +  Math.Round( (self.w - 880) / 2.0);
            let mut tlistselect8: i32 = tlistselect7;
            let mut game: GameClass = self.game;
            let mut tValueWidth: i32 = 100 +  Math.Round( (self.w - 880) / 5.0);
             local7: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 220;
            font: Font =  null;
             local8: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList6Obj, tlistsize, twidth, tlistselect8, game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local7), bbx: bbx, bby: 16, tMarcStyle: true, overruleFont: ( local8));
            self.optionslist6id = self.AddSubPart( tsubpart1, num1 + 220, 16, 320 +  Math.Round( (self.w - 880) / 2.0), (num37 + 1) * 16, 0);
          }
          let mut x3: i32 = num1 + 220 + 10;
          str2 = self.game.Data.StringListObj[stringListById].GetData(0, idValue, 2);
          DrawMod.DrawTextColouredMarc( g, "SHQ / LOCATION", self.game.MarcFont5, x3, 3, Color.White);
          let mut x4: i32 = num1 + 220 + 320 +  Math.Round( (self.w - 880) / 2.0) -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          upper1: String = "requested".ToUpper();
          DrawMod.DrawTextColouredMarc( g, upper1, self.game.MarcFont5, x4, 3, Color.White);
          let mut x5: i32 = x4 -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          upper2: String = "received".ToUpper();
          DrawMod.DrawTextColouredMarc( g, upper2, self.game.MarcFont5, x5, 3, Color.White);
        }
        if (self.detailnr >= 501 & self.detailnr <= 599)
        {
          self.OptionsList6Obj = ListClass::new();
          let mut tlistselect9: i32 = -1;
          str5: String = "---";
          let mut idValue: i32 = self.detailnr - 500;
          let mut num38: i32 = -1;
          let mut unitCounter5: i32 = self.game.Data.UnitCounter;
          for (let mut tdata: i32 = 0; tdata <= unitCounter5; tdata += 1)
          {
            if (self.detailnr4 < 1 | self.game.Data.UnitObj[tdata].IsHQ && self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[tdata].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[tdata].PreDef == -1 & self.game.Data.UnitObj[tdata].Historical > -1 && self.game.Data.UnitObj[tdata].HQ == -1 & self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 8)
            {
              num38 += 1;
              if (self.detailhisnr == -1)
                self.detailhisnr = tdata;
              if (self.detailhisnr == tdata)
                tlistselect9 = num38;
              self.OptionsList6Obj.add(self.game.Data.UnitObj[tdata].Name, tdata, tWeight: 0);
              let mut locCounter6: i32 = self.game.Data.LocCounter;
              for (let mut index: i32 = 0; index <= locCounter6; index += 1)
              {
                if (self.game.Data.LocObj[index].HQ == tdata && Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectGreater(objArray8[index, idValue],  0, false), Operators.CompareObjectGreater(objArray8[index, idValue],  0, false))))
                {
                  tvalue2: String = objArray8[index, idValue].ToString();
                  tvalue: String = objArray9[index, idValue].ToString();
                  num38 += 1;
                  if (self.detailhisnr == -1)
                    self.detailhisnr = index + 1000000;
                  if (self.detailhisnr == index + 1000000)
                    tlistselect9 = num38;
                  self.OptionsList6Obj.add(str5 + self.game.Data.LocObj[index].Name, index + 1000000, tvalue, tvalue2, tWeight: 0);
                }
              }
            }
          }
          if (num38 == -1)
          {
            self.RemoveSubPart(self.optionslist6id);
            self.optionslist6id = 0;
          }
          let mut num39: i32 =  Math.Round( (self.h - 80) / 16.0) - 1;
          if (self.optionslist6id > 0)
          {
            self.SubPartList[self.SubpartNr(self.optionslist6id)].Refresh(self.OptionsList6Obj, tlistselect9);
            self.SubPartFlag[self.SubpartNr(self.optionslist6id)] = true;
          }
          else if (num38 > -1)
          {
            ListClass optionsList6Obj = self.OptionsList6Obj;
            let mut tlistsize: i32 = num39;
            let mut twidth: i32 = 320 +  Math.Round( (self.w - 880) / 2.0);
            let mut tlistselect10: i32 = tlistselect9;
            let mut game: GameClass = self.game;
            let mut tValueWidth: i32 = 100 +  Math.Round( (self.w - 880) / 5.0);
             local9: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 220;
            font: Font =  null;
             local10: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList6Obj, tlistsize, twidth, tlistselect10, game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local9), bbx: bbx, bby: 16, tMarcStyle: true, overruleFont: ( local10));
            self.optionslist6id = self.AddSubPart( tsubpart1, num1 + 220, 16, 320 +  Math.Round( (self.w - 880) / 2.0), (num39 + 1) * 16, 0);
          }
          let mut x6: i32 = num1 + 220 + 10;
          str2 = self.game.Data.StringListObj[stringListById].GetData(0, idValue, 2);
          DrawMod.DrawTextColouredMarc( g, "SHQ / LOCATION", self.game.MarcFont5, x6, 3, Color.White);
          let mut x7: i32 = num1 + 220 + 320 +  Math.Round( (self.w - 880) / 2.0) -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          upper3: String = "offered".ToUpper();
          DrawMod.DrawTextColouredMarc( g, upper3, self.game.MarcFont5, x7, 3, Color.White);
          let mut x8: i32 = x7 -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          upper4: String = "picked-up".ToUpper();
          DrawMod.DrawTextColouredMarc( g, upper4, self.game.MarcFont5, x8, 3, Color.White);
        }
        if (self.detailnr == 300 | self.detailnr == 600)
        {
          self.OptionsList6Obj = ListClass::new();
          bool[] flagArray3 = new bool[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray32 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray33 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray34 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray35 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray36 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray37 = new int[self.game.Data.HistoricalUnitCounter + 1];
          let mut TempInt: i32 = -1;
          let mut num40: i32 = -1;
          let mut unitCounter6: i32 = self.game.Data.UnitCounter;
          for (let mut unr: i32 = 0; unr <= unitCounter6; unr += 1)
          {
            if (self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[unr].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Historical > -1 && !flagArray3[self.game.Data.UnitObj[unr].Historical])
            {
              if (self.game.Data.UnitObj[unr].IsHQ)
              {
                num41: i32;
                num41 += 1;
                numArray37[self.game.Data.UnitObj[unr].Historical] = num41;
              }
              let mut Left: i32 = 0;
              if (self.detailnr == 300)
              {
                if (Operators.ConditionalCompareObjectGreater(objArray11[unr],  0, false))
                  Left = Conversions.ToInteger(Operators.AddObject( Left, objArray11[unr]));
              }
              else if (self.detailnr == 600 && Operators.ConditionalCompareObjectGreater(objArray13[unr],  0, false))
                Left = Conversions.ToInteger(Operators.AddObject( Left, objArray13[unr]));
              num42: i32;
              if (Left > 0 | num42 > 0 | self.game.Data.UnitObj[unr].IsHQ)
              {
                let mut num43: i32 = self.game.HandyFunctionsObj.HowmanyHQsAbove(unr);
                numArray35[self.game.Data.UnitObj[unr].Historical] = num43;
                numArray36[self.game.Data.UnitObj[unr].Historical] = self.game.HandyFunctionsObj.HowmanyHQsBelow(unr);
                flagArray3[self.game.Data.UnitObj[unr].Historical] = true;
                int[] numArray38 = numArray32;
                int[] numArray39 = numArray38;
                let mut historical1: i32 = self.game.Data.UnitObj[unr].Historical;
                let mut index110: i32 = historical1;
                let mut num44: i32 = numArray38[historical1] + Left;
                numArray39[index110] = num44;
                let mut hq: i32 = self.game.Data.UnitObj[unr].HQ;
                if (hq > -1)
                {
                  int[] numArray40 = numArray33;
                  int[] numArray41 = numArray40;
                  let mut historical2: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index111: i32 = historical2;
                  let mut num45: i32 = numArray40[historical2] + Left;
                  numArray41[index111] = num45;
                  int[] numArray42 = numArray34;
                  int[] numArray43 = numArray42;
                  let mut historical3: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index112: i32 = historical3;
                  let mut num46: i32 = numArray42[historical3] + Left;
                  numArray43[index112] = num46;
                }
                if (hq > -1)
                  hq = self.game.Data.UnitObj[hq].HQ;
                for (; hq > -1; hq = self.game.Data.UnitObj[hq].HQ)
                {
                  int[] numArray44 = numArray34;
                  int[] numArray45 = numArray44;
                  let mut historical4: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index113: i32 = historical4;
                  let mut num47: i32 = numArray44[historical4] + Left;
                  numArray45[index113] = num47;
                }
              }
            }
          }
          bool[] flagArray4 = new bool[self.game.Data.HistoricalUnitCounter + 1];
          let mut unitCounter7: i32 = self.game.Data.UnitCounter;
          for (let mut tdata: i32 = 0; tdata <= unitCounter7; tdata += 1)
          {
            if (self.detailnr4 < 1 | self.game.Data.UnitObj[tdata].IsHQ && self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[tdata].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[tdata].PreDef == -1 & self.game.Data.UnitObj[tdata].Historical > -1 && !flagArray4[self.game.Data.UnitObj[tdata].Historical])
            {
              let mut num48: i32 = numArray34[self.game.Data.UnitObj[tdata].Historical];
              let mut num49: i32 = numArray33[self.game.Data.UnitObj[tdata].Historical];
              let mut num50: i32 = numArray32[self.game.Data.UnitObj[tdata].Historical];
              flagArray4[self.game.Data.UnitObj[tdata].Historical] = true;
              let mut num51: i32 = numArray35[self.game.Data.UnitObj[tdata].Historical];
              let mut num52: i32 = num51;
              str6: String;
              for (let mut index: i32 = 1; index <= num52; index += 1)
                str6 += "---";
              num19 = 0;
              let mut index114: i32 = !self.game.Data.UnitObj[tdata].IsHQ ? self.game.Data.UnitObj[tdata].HQ : tdata;
              let mut num53: i32 = 1;
              if (!self.game.Data.UnitObj[tdata].IsHQ)
                num53 += 5;
              for (; index114 > -1; index114 = self.game.Data.UnitObj[index114].HQ)
              {
                let mut num54: i32 = numArray36[self.game.Data.UnitObj[index114].Historical];
                let mut num55: i32 = 1;
                let mut num56: i32 = num54;
                for (let mut index115: i32 = 0; index115 <= num56; index115 += 1)
                  num55 *= 1000;
                let mut num57: i32 = num55 * numArray37[self.game.Data.UnitObj[index114].Historical];
                num53 += num57;
              }
              let mut tWeight: i32 = 0;
              str6 = "";
              let mut num58: i32 = num51;
              for (let mut index116: i32 = 1; index116 <= num58; index116 += 1)
              {
                str6 += Strings.Space(6);
                index114 += 1;
              }
              if (self.game.Data.UnitObj[tdata].IsHQ)
                tWeight = tWeight + 0 + num53;
              else if (self.game.Data.UnitObj[tdata].HQ > -1)
                tWeight = tWeight + 1 + num53;
              if (num48 + num49 + num50 > 0)
              {
                num40 += 1;
                if (self.detailhisnr == -1)
                  self.detailhisnr = tdata;
                if (self.detailhisnr == tdata)
                  TempInt = num40;
                str2 = num48.ToString();
                str7: String = num50.ToString();
                if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 8)
                  str7 = "n/a";
                if (Operators.CompareString(str2, "0", false) == 0)
                  str2 = "...";
                if (Operators.CompareString(str7, "0", false) == 0)
                  str7 = "...";
                if (!self.game.Data.UnitObj[tdata].IsHQ)
                  str2 = "...";
                self.OptionsList6Obj.add(str6 + self.game.Data.UnitObj[tdata].Name, tdata, str2, str7, tWeight: tWeight);
              }
            }
          }
          let mut tlistselect11: i32 = self.OptionsList6Obj.SortWithRefOnWeightAndName(TempInt);
          if (num40 == -1)
          {
            self.RemoveSubPart(self.optionslist6id);
            self.optionslist6id = 0;
          }
          let mut num59: i32 =  Math.Round( (self.h - 80) / 16.0) - 1;
          if (self.optionslist6id > 0)
          {
            self.SubPartList[self.SubpartNr(self.optionslist6id)].Refresh(self.OptionsList6Obj, tlistselect11);
            self.SubPartFlag[self.SubpartNr(self.optionslist6id)] = true;
          }
          else if (num40 > -1)
          {
            ListClass optionsList6Obj = self.OptionsList6Obj;
            let mut tlistsize: i32 = num59;
            let mut twidth: i32 = 320 +  Math.Round( (self.w - 880) / 2.0);
            let mut tlistselect12: i32 = tlistselect11;
            let mut game: GameClass = self.game;
            let mut tValueWidth: i32 = 100 +  Math.Round( (self.w - 880) / 5.0);
             local11: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 220;
            font: Font =  null;
             local12: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList6Obj, tlistsize, twidth, tlistselect12, game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local11), bbx: bbx, bby: 16, tMarcStyle: true, overruleFont: ( local12));
            self.optionslist6id = self.AddSubPart( tsubpart1, num1 + 220, 16, 320 +  Math.Round( (self.w - 880) / 2.0), (num59 + 1) * 16, 0);
          }
          else
          {
            str2 = "No unit that received or returned replacements";
            DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont2, num1 + 260, 90, Color.FromArgb(177,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
          }
          if (num40 > -1)
          {
            let mut x9: i32 = num1 + 220 + 10;
            DrawMod.DrawTextColouredMarc( g, "UNIT", self.game.MarcFont5, x9, 3, Color.White);
            let mut x10: i32 = num1 + 220 + 320 +  Math.Round( (self.w - 880) / 2.0) -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
            upper5: String = "Log. Pts".ToUpper();
            DrawMod.DrawTextColouredMarc( g, upper5, self.game.MarcFont5, x10, 3, Color.White);
            let mut x11: i32 = x10 -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
            upper6: String = "Cumulative".ToUpper();
            DrawMod.DrawTextColouredMarc( g, upper6, self.game.MarcFont5, x11, 3, Color.White);
            if (self.detailhisnr > -1)
            {
              let mut num60: i32 =  Math.Round( (self.h - 80) / 16.0) - 3;
              tsubpart1 =  new TextButtonPartClass("GO TO UNIT", 140, "Click to center map on this unit.",  self.OwnBitmap, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), (num60 + 2) * 16 - 7, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
              self.text6id = self.AddSubPart( tsubpart1, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), (num60 + 2) * 16 - 7, 140, 40, 1);
            }
          }
        }
        if (self.detailnr >= 301 & self.detailnr <= 399)
        {
          let mut idValue: i32 = self.detailnr - 300;
          self.OptionsList6Obj = ListClass::new();
          bool[] flagArray5 = new bool[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray46 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray47 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray48 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray49 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray50 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray51 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray52 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray53 = new int[self.game.Data.HistoricalUnitCounter + 1];
          int[] numArray54 = new int[self.game.Data.HistoricalUnitCounter + 1];
          let mut TempInt: i32 = -1;
          let mut num61: i32 = -1;
          let mut unitCounter8: i32 = self.game.Data.UnitCounter;
          for (let mut unr: i32 = 0; unr <= unitCounter8; unr += 1)
          {
            if (self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[unr].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Historical > -1 && !flagArray5[self.game.Data.UnitObj[unr].Historical])
            {
              if (self.game.Data.UnitObj[unr].IsHQ)
              {
                num62: i32;
                num62 += 1;
                numArray54[self.game.Data.UnitObj[unr].Historical] = num62;
              }
              let mut Left1: i32 = 0;
              let mut Left2: i32 = 0;
              if (Operators.ConditionalCompareObjectGreater(objArray6[unr, idValue],  0, false))
                Left1 = Conversions.ToInteger(Operators.AddObject( Left1, objArray6[unr, idValue]));
              if (Operators.ConditionalCompareObjectGreater(objArray6[unr, idValue],  0, false))
                Left2 = Conversions.ToInteger(Operators.AddObject( Left2, objArray7[unr, idValue]));
              if (Left1 > 0 | Left2 > 0 | self.game.Data.UnitObj[unr].IsHQ)
              {
                let mut num63: i32 = self.game.HandyFunctionsObj.HowmanyHQsAbove(unr);
                numArray52[self.game.Data.UnitObj[unr].Historical] = num63;
                numArray53[self.game.Data.UnitObj[unr].Historical] = self.game.HandyFunctionsObj.HowmanyHQsBelow(unr);
                flagArray5[self.game.Data.UnitObj[unr].Historical] = true;
                int[] numArray55 = numArray46;
                int[] numArray56 = numArray55;
                let mut historical5: i32 = self.game.Data.UnitObj[unr].Historical;
                let mut index117: i32 = historical5;
                let mut num64: i32 = numArray55[historical5] + Left1;
                numArray56[index117] = num64;
                int[] numArray57 = numArray47;
                int[] numArray58 = numArray57;
                let mut historical6: i32 = self.game.Data.UnitObj[unr].Historical;
                let mut index118: i32 = historical6;
                let mut num65: i32 = numArray57[historical6] + Left2;
                numArray58[index118] = num65;
                let mut hq: i32 = self.game.Data.UnitObj[unr].HQ;
                if (hq > -1)
                {
                  int[] numArray59 = numArray48;
                  int[] numArray60 = numArray59;
                  let mut historical7: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index119: i32 = historical7;
                  let mut num66: i32 = numArray59[historical7] + Left1;
                  numArray60[index119] = num66;
                  int[] numArray61 = numArray49;
                  int[] numArray62 = numArray61;
                  let mut historical8: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index120: i32 = historical8;
                  let mut num67: i32 = numArray61[historical8] + Left1;
                  numArray62[index120] = num67;
                  int[] numArray63 = numArray50;
                  int[] numArray64 = numArray63;
                  let mut historical9: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index121: i32 = historical9;
                  let mut num68: i32 = numArray63[historical9] + Left2;
                  numArray64[index121] = num68;
                  int[] numArray65 = numArray51;
                  int[] numArray66 = numArray65;
                  let mut historical10: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index122: i32 = historical10;
                  let mut num69: i32 = numArray65[historical10] + Left2;
                  numArray66[index122] = num69;
                }
                if (hq > -1)
                  hq = self.game.Data.UnitObj[hq].HQ;
                for (; hq > -1; hq = self.game.Data.UnitObj[hq].HQ)
                {
                  int[] numArray67 = numArray49;
                  int[] numArray68 = numArray67;
                  let mut historical11: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index123: i32 = historical11;
                  let mut num70: i32 = numArray67[historical11] + Left1;
                  numArray68[index123] = num70;
                  int[] numArray69 = numArray51;
                  int[] numArray70 = numArray69;
                  let mut historical12: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index124: i32 = historical12;
                  let mut num71: i32 = numArray69[historical12] + Left2;
                  numArray70[index124] = num71;
                }
              }
            }
          }
          bool[] flagArray6 = new bool[self.game.Data.HistoricalUnitCounter + 1];
          let mut unitCounter9: i32 = self.game.Data.UnitCounter;
          for (let mut tdata: i32 = 0; tdata <= unitCounter9; tdata += 1)
          {
            if (self.detailnr4 < 1 | self.game.Data.UnitObj[tdata].IsHQ && self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[tdata].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[tdata].PreDef == -1 & self.game.Data.UnitObj[tdata].Historical > -1 && !flagArray6[self.game.Data.UnitObj[tdata].Historical])
            {
              let mut num72: i32 = numArray49[self.game.Data.UnitObj[tdata].Historical];
              let mut num73: i32 = numArray48[self.game.Data.UnitObj[tdata].Historical];
              let mut num74: i32 = numArray46[self.game.Data.UnitObj[tdata].Historical];
              let mut num75: i32 = numArray51[self.game.Data.UnitObj[tdata].Historical];
              let mut num76: i32 = numArray50[self.game.Data.UnitObj[tdata].Historical];
              let mut num77: i32 = numArray47[self.game.Data.UnitObj[tdata].Historical];
              flagArray6[self.game.Data.UnitObj[tdata].Historical] = true;
              let mut num78: i32 = numArray52[self.game.Data.UnitObj[tdata].Historical];
              let mut num79: i32 = num78;
              str8: String;
              for (let mut index: i32 = 1; index <= num79; index += 1)
                str8 += "---";
              num19 = 0;
              let mut index125: i32 = !self.game.Data.UnitObj[tdata].IsHQ ? self.game.Data.UnitObj[tdata].HQ : tdata;
              let mut num80: i32 = 1;
              if (!self.game.Data.UnitObj[tdata].IsHQ)
                num80 += 5;
              for (; index125 > -1; index125 = self.game.Data.UnitObj[index125].HQ)
              {
                let mut num81: i32 = numArray53[self.game.Data.UnitObj[index125].Historical];
                let mut num82: i32 = 1;
                let mut num83: i32 = num81;
                for (let mut index126: i32 = 0; index126 <= num83; index126 += 1)
                  num82 *= 1000;
                let mut num84: i32 = num82 * numArray54[self.game.Data.UnitObj[index125].Historical];
                num80 += num84;
              }
              let mut tWeight: i32 = 0;
              str8 = "";
              let mut num85: i32 = num78;
              for (let mut index127: i32 = 1; index127 <= num85; index127 += 1)
              {
                str8 += Strings.Space(6);
                index125 += 1;
              }
              if (self.game.Data.UnitObj[tdata].IsHQ)
                tWeight = tWeight + 0 + num80;
              else if (self.game.Data.UnitObj[tdata].HQ > -1)
                tWeight = tWeight + 1 + num80;
              if (num72 + num73 + num74 + num75 + num76 + num77 > 0)
              {
                num61 += 1;
                if (self.detailhisnr == -1)
                  self.detailhisnr = tdata;
                if (self.detailhisnr == tdata)
                  TempInt = num61;
                str9: String = num75.ToString() + "/" + num72.ToString();
                str10: String = num77.ToString() + "/" + num74.ToString();
                if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 8)
                  str10 = "n/a";
                if (Operators.CompareString(str9, "0/0", false) == 0)
                  str9 = "...";
                if (Operators.CompareString(str10, "0/0", false) == 0)
                  str10 = "...";
                if (!self.game.Data.UnitObj[tdata].IsHQ)
                  str9 = "...";
                self.OptionsList6Obj.add(str8 + self.game.Data.UnitObj[tdata].Name, tdata, str9, str10, tWeight: tWeight);
              }
            }
          }
          let mut tlistselect13: i32 = self.OptionsList6Obj.SortWithRefOnWeightAndName(TempInt);
          if (num61 == -1)
          {
            self.RemoveSubPart(self.optionslist6id);
            self.optionslist6id = 0;
          }
          let mut num86: i32 =  Math.Round( (self.h - 80) / 16.0) - 1;
          if (self.optionslist6id > 0)
          {
            self.SubPartList[self.SubpartNr(self.optionslist6id)].Refresh(self.OptionsList6Obj, tlistselect13);
            self.SubPartFlag[self.SubpartNr(self.optionslist6id)] = true;
          }
          else if (num61 > -1)
          {
            ListClass optionsList6Obj = self.OptionsList6Obj;
            let mut tlistsize: i32 = num86;
            let mut twidth: i32 = 320 +  Math.Round( (self.w - 880) / 2.0);
            let mut tlistselect14: i32 = tlistselect13;
            let mut game: GameClass = self.game;
            let mut tValueWidth: i32 = 100 +  Math.Round( (self.w - 880) / 5.0);
             local13: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 220;
            font: Font =  null;
             local14: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList6Obj, tlistsize, twidth, tlistselect14, game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local13), bbx: bbx, bby: 16, tMarcStyle: true, overruleFont: ( local14));
            self.optionslist6id = self.AddSubPart( tsubpart1, num1 + 220, 16, 320 +  Math.Round( (self.w - 880) / 2.0), (num86 + 1) * 16, 0);
          }
          let mut x12: i32 = num1 + 220 + 10;
          str2 = self.game.Data.StringListObj[stringListById].GetData(0, idValue, 2);
          DrawMod.DrawTextColouredMarc( g, "UNIT", self.game.MarcFont5, x12, 3, Color.White);
          let mut x13: i32 = num1 + 220 + 320 +  Math.Round( (self.w - 880) / 2.0) -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          upper7: String = "rec/req".ToUpper();
          DrawMod.DrawTextColouredMarc( g, upper7, self.game.MarcFont5, x13, 3, Color.White);
          let mut x14: i32 = x13 -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          upper8: String = "cumulative".ToUpper();
          DrawMod.DrawTextColouredMarc( g, upper8, self.game.MarcFont5, x14, 3, Color.White);
          if (self.detailhisnr > -1)
          {
            let mut num87: i32 =  Math.Round( (self.h - 80) / 16.0) - 3;
            tsubpart1 =  new TextButtonPartClass("GO TO UNIT", 140, "Click to center map on this unit.",  self.OwnBitmap, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), (num87 + 2) * 16 - 7, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.text6id = self.AddSubPart( tsubpart1, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), (num87 + 2) * 16 - 7, 140, 40, 1);
          }
        }
      }
      if (self.subtabnr == 1)
      {
        tsubpart1 =  new MarcRadioPartClass(0, self.detailnr4 == 1, "Only Show HQs",  self.OwnBitmap, 30 + num1, self.h - 160);
        self.Text5id = self.AddSubPart( tsubpart1, 30 + num1, self.h - 160, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( g, "Only Show HQs", self.game.MarcFont4, 75 + num1, self.h - 150, Color.White);
        self.OptionsList5Obj = ListClass::new();
        let mut tlistselect15: i32 = -1;
        let mut index128: i32 = -1;
        let mut tdata: i32 = 0;
        do
        {
          if ( self.game.Data.RuleVar[650 + tdata] > 0.0)
          {
            index128 += 1;
            if (self.detailnr == -1)
              self.detailnr = tdata;
            if (self.detailnr == tdata)
              tlistselect15 = index128;
            self.OptionsList5Obj.add(self.game.Data.TempString[700 + tdata], tdata);
          }
          tdata += 1;
        }
        while (tdata <= 2);
        if ( self.game.Data.RuleVar[403] < 1.0)
        {
          if ( self.game.Data.RuleVar[337] > 0.0)
          {
            let mut num88: i32 = index128 + 1;
            if (self.detailnr == -1)
              self.detailnr = 11;
            if (self.detailnr == 11)
              tlistselect15 = num88;
            self.OptionsList5Obj.add("Replacements received", 11);
            index128 = num88 + 1;
            if (self.detailnr == 12)
              tlistselect15 = index128;
            self.OptionsList5Obj.add("Replacements requested", 12);
            if ( self.game.Data.RuleVar[977] < 1.0)
            {
              index128 += 1;
              if (self.detailnr == 13)
                tlistselect15 = index128;
              self.OptionsList5Obj.add("Replacements returned", 13);
            }
          }
          if ( self.game.Data.RuleVar[499] > 0.0 &  self.game.Data.RuleVar[957] > 0.0)
          {
            let mut num89: i32 = index128 + 1;
            if (self.detailnr == 14)
              tlistselect15 = num89;
            self.OptionsList5Obj.add("Unit Supply Logbook", 14);
            index128 = num89 + 1;
            if (self.detailnr == 15)
              tlistselect15 = index128;
            self.OptionsList5Obj.add("Unit Fuel Logbook", 15);
          }
        }
        if ( self.game.Data.RuleVar[957] > 0.0)
        {
          let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[957]));
          if (stringListById > -1)
          {
            let mut length: i32 = self.game.Data.StringListObj[stringListById].Length;
            for (let mut index129: i32 = 0; index129 <= length; index129 += 1)
            {
              str11: String = self.game.Data.StringListObj[stringListById].Data[index129, 0];
              let mut num90: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index129, 1]));
              bool flag = true;
              if (num90 == self.game.Data.Turn && !Information.IsNothing( str11) && str11.Length > 0)
              {
                let mut num91: i32 = index129 - 1;
                for (let mut index130: i32 = 0; index130 <= num91; index130 += 1)
                {
                  Left: String = self.game.Data.StringListObj[stringListById].Data[index130, 0];
                  if (self.game.Data.Product < 6)
                  {
                    if (Operators.CompareString(Left, str11, false) == 0)
                    {
                      flag = false;
                      break;
                    }
                  }
                  else
                  {
                    let mut integer: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index130, 1]);
                    if (Operators.CompareString(Left, str11, false) == 0 & integer == num90)
                    {
                      flag = false;
                      break;
                    }
                  }
                }
                if (flag)
                {
                  index128 += 1;
                  if (self.game.Data.Product == 7 && Strings.InStr(str11, "<REGIMES>") > 0)
                    str11 = str11.Replace("<REGIMES>", "");
                  if (self.detailnr == -1)
                    self.detailnr = 100 + index129;
                  if (self.detailnr == 100 + index129)
                    tlistselect15 = index128;
                  self.OptionsList5Obj.add(str11, 100 + index129);
                }
              }
            }
          }
        }
        if (index128 > -1)
        {
          let mut num92: i32 =  Math.Round( (self.h - 90) / 16.0) - 8;
          if (self.OptionsList5id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList5id)].Refresh(self.OptionsList5Obj, tlistselect15);
            self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
          }
          else if (self.game.Data.Product >= 7)
          {
            let mut tlistsize: i32 =  Math.Round( (self.h - 90) / 24.0) - 6;
            tsubpart1 =  new ListSubPartClass(self.OptionsList5Obj, tlistsize, 170, tlistselect15, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 30), bby: 10, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
            self.OptionsList5id = self.AddSubPart( tsubpart1, num1 + 30, 10, 170, (tlistsize + 1) * 24, 0);
          }
          else
          {
            ListClass optionsList5Obj = self.OptionsList5Obj;
            let mut tlistsize: i32 = num92;
            let mut tlistselect16: i32 = tlistselect15;
            let mut game: GameClass = self.game;
             local15: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 30;
            font: Font =  null;
             local16: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList5Obj, tlistsize, 170, tlistselect16, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( local15), bbx: bbx, bby: 10, tMarcStyle: true, overruleFont: ( local16));
            self.OptionsList5id = self.AddSubPart( tsubpart1, num1 + 30, 10, 170, (num92 + 1) * 16, 0);
          }
        }
        if (self.detailnr >= 100 &  self.game.Data.RuleVar[957] > 0.0)
        {
          let mut index131: i32 = self.detailnr - 100;
          let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[957]));
          Left3: String = self.game.Data.StringListObj[stringListById].Data[index131, 0];
          tregcount = 0;
          str1 = Left3;
          let mut tMultiplier: i32 = 1;
          bool flag1 = false;
          if (self.game.Data.Product == 7 && Strings.InStr(str1, "<REGIMES>") > 0)
          {
            str1 = str1.Replace("<REGIMES>", "");
            flag1 = true;
          }
          let mut length1: i32 = self.game.Data.StringListObj[stringListById].Length;
          for (let mut index132: i32 = 0; index132 <= length1; index132 += 1)
          {
            Right: String = self.game.Data.StringListObj[stringListById].Data[index132, 0];
            let mut integer1: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index132, 1]);
            if (Operators.CompareString(Left3, Right, false) == 0 & integer1 == self.game.Data.Turn)
            {
              str12: String = self.game.Data.StringListObj[stringListById].Data[index132, 2];
              bool flag2 = true;
              if (!Information.IsNothing( str12) & str12.Length > 0)
              {
                let mut num93: i32 = index132 - 1;
                for (index128 = 0; index128 <= num93; index128 += 1)
                {
                  Left4: String = self.game.Data.StringListObj[stringListById].Data[index128, 2];
                  Left5: String = self.game.Data.StringListObj[stringListById].Data[index128, 0];
                  if (Operators.CompareString(Left4, str12, false) == 0 && Operators.CompareString(Left5, Right, false) == 0 & integer1 == self.game.Data.Turn)
                  {
                    if (self.game.Data.Product < 6)
                    {
                      flag2 = false;
                      break;
                    }
                    let mut integer2: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index128, 1]);
                    if (Operators.CompareString(Left5, Right, false) == 0 & integer2 == integer1 & integer1 == self.game.Data.Turn)
                    {
                      flag2 = false;
                      break;
                    }
                  }
                }
                if (flag2 & flag1)
                {
                  let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
                  for (let mut regSlot: i32 = 0; regSlot <= regimeCounter; regSlot += 1)
                  {
                    if (Operators.CompareString(self.game.Data.RegimeObj[regSlot].Name, str12, false) == 0 && !self.game.HandyFunctionsObj.ShowStatsOfRegime(regSlot))
                      flag2 = false;
                  }
                }
                if (flag2)
                {
                  tregcount += 1;
                  colorArray = (Color[]) Utils.CopyArray((Array) colorArray, (Array) new Color[tregcount + 1]);
                  strArray1 = (string[]) Utils.CopyArray((Array) strArray1, (Array) new string[tregcount + 1]);
                  if (tregcount == 1)
                    colorArray[tregcount] = Color.White;
                  if (tregcount == 2)
                    colorArray[tregcount] = Color.Salmon;
                  if (tregcount == 3)
                    colorArray[tregcount] = Color.FromArgb( byte.MaxValue, 50, 180, 180);
                  if (tregcount == 4)
                    colorArray[tregcount] = Color.LightGreen;
                  if (tregcount == 5)
                    colorArray[tregcount] = Color.Orange;
                  if (tregcount == 6)
                    colorArray[tregcount] = Color.Yellow;
                  if (tregcount == 7)
                    colorArray[tregcount] = Color.Pink;
                  if (tregcount == 8)
                    colorArray[tregcount] = Color.Brown;
                  if (tregcount == 9)
                    colorArray[tregcount] = Color.Blue;
                  if (tregcount == 10)
                    colorArray[tregcount] = Color.Indigo;
                  if (tregcount > 10)
                    colorArray[tregcount] = Color.LightGray;
                  strArray1[tregcount] = str12;
                }
              }
            }
          }
          if (tregcount > 0)
          {
            numArray1 = new int[self.game.Data.Round + 1, tregcount + 1];
            bool flag3 = false;
            if (self.game.Data.Product == 7 && Operators.CompareString(str1, "Comparative Victory Score", false) == 0)
              flag3 = true;
            if (self.game.Data.Product == 7 & (flag1 | flag3))
            {
              let mut round: i32 = self.game.Data.Round;
              for (let mut index133: i32 = 0; index133 <= round; index133 += 1)
              {
                let mut num94: i32 = tregcount;
                for (index128 = 0; index128 <= num94; index128 += 1)
                  numArray1[index133, index128] = -1;
              }
            }
            let mut length2: i32 = self.game.Data.StringListObj[stringListById].Length;
            for (let mut index134: i32 = 0; index134 <= length2; index134 += 1)
            {
              Right1: String = self.game.Data.StringListObj[stringListById].Data[index134, 0];
              let mut integer: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById].Data[index134, 1]);
              let mut num95: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index134, 3]));
              if (Operators.CompareString(Left3, Right1, false) == 0 & integer == self.game.Data.Turn & num95 <= self.game.Data.Round)
              {
                Right2: String = self.game.Data.StringListObj[stringListById].Data[index134, 2];
                let mut num96: i32 = tregcount;
                for (index128 = 1; index128 <= num96; index128 += 1)
                {
                  if (Operators.CompareString(strArray1[index128], Right2, false) == 0)
                  {
                    let mut num97: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index134, 4]));
                    if (num97 > -1000000)
                    {
                      numArray71: Vec<i32> = numArray1;
                      numArray72: Vec<i32> = numArray71;
                      let mut index135: i32 = num95;
                      let mut index136: i32 = index135;
                      let mut index137: i32 = index128;
                      let mut index138: i32 = index137;
                      let mut num98: i32 = numArray71[index135, index137] + num97;
                      numArray72[index136, index138] = num98;
                    }
                  }
                }
              }
            }
            if (self.game.Data.Product == 7 & (flag1 | flag3))
            {
              if (self.game.Data.Round == 1)
              {
                let mut num99: i32 = 0;
                let mut num100: i32 = tregcount;
                for (index128 = 0; index128 <= num100; index128 += 1)
                {
                  if (numArray1[self.game.Data.Round, index128] == -1)
                  {
                    num99 = index128 - 1;
                    break;
                  }
                }
                if (num99 >= 0)
                  numArray1 = (int[,]) Utils.CopyArray((Array) numArray1, (Array) new int[self.game.Data.Round + 1, num99 + 1]);
              }
              else
              {
                for (let mut round: i32 = self.game.Data.Round; round >= 0; round += -1)
                {
                  let mut num101: i32 = tregcount;
                  for (index128 = 0; index128 <= num101; index128 += 1)
                  {
                    if (numArray1[round, index128] == -1 & round > 0)
                      numArray1[round, index128] = numArray1[round - 1, index128];
                  }
                }
              }
            }
            tsubpart1 =  new GraphClass(self.game, numArray1, self.detailnr3 == 1, str1, tMultiplier, colorArray, strArray1, tregcount, self.w - 880 + 620 - num1, self.h - 80,  self.OwnBitmap, num1 + 220, 0, false);
            self.Info1Id = self.AddSubPart( tsubpart1, num1 + 220, 0, self.w - 880 + 620 - num1, 330, 0);
          }
        }
        if (self.detailnr <= 9 & index128 > -1)
        {
          bool[] flagArray = new bool[self.game.Data.RegimeCounter + 1];
          let mut regimeCounter3: i32 = self.game.Data.RegimeCounter;
          for (let mut index139: i32 = 0; index139 <= regimeCounter3; index139 += 1)
          {
            if (!self.game.Data.RegimeObj[index139].hideFromList)
            {
              let mut round: i32 = self.game.Data.Round;
              for (let mut index140: i32 = 1; index140 <= round; index140 += 1)
              {
                try
                {
                  if (self.game.Data.RegimeObj[index139].ExtraStat[index139, index140] > 0)
                    flagArray[index139] = true;
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  flagArray[index139] = false;
                  ProjectData.ClearProjectError();
                }
              }
            }
          }
          let mut round2: i32 = self.game.Data.Round;
          for (let mut index141: i32 = 0; index141 <= round2; index141 += 1)
          {
            tregcount = 0;
            let mut regimeCounter4: i32 = self.game.Data.RegimeCounter;
            for (let mut reg2: i32 = 0; reg2 <= regimeCounter4; reg2 += 1)
            {
              if (!self.game.Data.RegimeObj[reg2].hideFromList && !self.game.Data.RegimeObj[reg2].Sleep | flagArray[reg2] && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, reg2) | !self.game.Data.FOWOn | self.game.Data.Winner > -1 |  self.game.Data.RuleVar[313] > 0.0)
              {
                tregcount += 1;
                numArray1[index141, tregcount] = 0;
                colorArray[tregcount] = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[reg2].Red, self.game.Data.RegimeObj[reg2].Green, self.game.Data.RegimeObj[reg2].Blue);
                strArray1[tregcount] = self.game.Data.RegimeObj[reg2].Name;
                str1 = self.game.Data.TempString[700 + self.detailnr];
                numArray73: Vec<i32> = numArray1;
                numArray74: Vec<i32> = numArray73;
                let mut index142: i32 = index141;
                let mut index143: i32 = index142;
                let mut index144: i32 = tregcount;
                let mut index145: i32 = index144;
                let mut num102: i32 = numArray73[index142, index144] + self.game.Data.RegimeObj[reg2].ExtraStat[self.detailnr, index141];
                numArray74[index143, index145] = num102;
              }
            }
          }
          let mut tMultiplier: i32 = 1;
          tsubpart1 =  new GraphClass(self.game, numArray1, self.detailnr3 == 1, str1, tMultiplier, colorArray, strArray1, tregcount, self.w - 880 + 620 - num1, self.h - 80,  self.OwnBitmap, num1 + 220, 0);
          self.Info1Id = self.AddSubPart( tsubpart1, num1 + 220, 0, self.w - 880 + 620 - num1, 330, 0);
        }
      }
      if (self.subtabnr == 1 &  self.game.Data.RuleVar[403] < 1.0 | self.subtabnr == 2 &  self.game.Data.RuleVar[403] > 0.0 && self.detailnr >= 11 & self.detailnr <= 13 | self.detailnr >= 14 & self.detailnr <= 15 &  self.game.Data.RuleVar[499] > 0.0)
      {
        self.OptionsList6Obj = ListClass::new();
        bool[] flagArray7 = new bool[self.game.Data.HistoricalUnitCounter + 1];
        int[] numArray75 = new int[self.game.Data.HistoricalUnitCounter + 1];
        int[] numArray76 = new int[self.game.Data.HistoricalUnitCounter + 1];
        int[] numArray77 = new int[self.game.Data.HistoricalUnitCounter + 1];
        int[] numArray78 = new int[self.game.Data.HistoricalUnitCounter + 1];
        int[] numArray79 = new int[self.game.Data.HistoricalUnitCounter + 1];
        int[] numArray80 = new int[self.game.Data.HistoricalUnitCounter + 1];
        numArray81: Vec<i32> = new int[self.game.Data.HistoricalUnitCounter + 1, self.game.Data.ReinfCounter + 1 + 1];
        numArray82: Vec<i32> = new int[self.game.Data.HistoricalUnitCounter + 1, self.game.Data.ReinfCounter + 1 + 1];
        let mut TempInt: i32 = -1;
        let mut num103: i32 = -1;
        if (self.detailnr >= 11 & self.detailnr <= 15)
        {
          let mut unitCounter10: i32 = self.game.Data.UnitCounter;
          for (let mut unr: i32 = 0; unr <= unitCounter10; unr += 1)
          {
            if (self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[unr].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].Historical > -1 && !flagArray7[self.game.Data.UnitObj[unr].Historical])
            {
              if (self.game.Data.UnitObj[unr].IsHQ)
              {
                num104: i32;
                num104 += 1;
                numArray80[self.game.Data.UnitObj[unr].Historical] = num104;
              }
              let mut num105: i32 = 0;
              let mut unitCounter11: i32 = self.game.Data.UnitCounter;
              for (let mut index146: i32 = 0; index146 <= unitCounter11; index146 += 1)
              {
                if (self.game.Data.UnitObj[index146].PreDef == -1 & self.game.Data.UnitObj[index146].Historical == self.game.Data.UnitObj[unr].Historical)
                {
                  let mut logCounter: i32 = self.game.Data.UnitObj[index146].LogCounter;
                  for (let mut index147: i32 = 0; index147 <= logCounter; index147 += 1)
                  {
                    if (self.detailnr == 11)
                    {
                      if (self.game.Data.UnitObj[index146].LogType[index147] == 2 & self.game.Data.UnitObj[index146].LogData1[index147] > -1)
                      {
                        num105 += self.game.Data.UnitObj[index146].LogData3[index147];
                        numArray83: Vec<i32> = numArray82;
                        numArray84: Vec<i32> = numArray83;
                        let mut historical: i32 = self.game.Data.UnitObj[index146].Historical;
                        let mut index148: i32 = historical;
                        int[] logData1 = self.game.Data.UnitObj[index146].LogData1;
                        int[] numArray85 = logData1;
                        let mut index149: i32 = index147;
                        let mut index150: i32 = index149;
                        let mut index151: i32 = numArray85[index150];
                        let mut num106: i32 = numArray83[historical, logData1[index149]] + self.game.Data.UnitObj[index146].LogData3[index147];
                        numArray84[index148, index151] = num106;
                      }
                    }
                    else if (self.detailnr == 12)
                    {
                      if (self.game.Data.UnitObj[index146].LogType[index147] == 1 & self.game.Data.UnitObj[index146].LogData1[index147] > -1)
                      {
                        num105 += self.game.Data.UnitObj[index146].LogData3[index147];
                        numArray86: Vec<i32> = numArray82;
                        numArray87: Vec<i32> = numArray86;
                        let mut historical: i32 = self.game.Data.UnitObj[index146].Historical;
                        let mut index152: i32 = historical;
                        int[] logData1 = self.game.Data.UnitObj[index146].LogData1;
                        int[] numArray88 = logData1;
                        let mut index153: i32 = index147;
                        let mut index154: i32 = index153;
                        let mut index155: i32 = numArray88[index154];
                        let mut num107: i32 = numArray86[historical, logData1[index153]] + self.game.Data.UnitObj[index146].LogData3[index147];
                        numArray87[index152, index155] = num107;
                      }
                    }
                    else if (self.detailnr == 13 && self.game.Data.UnitObj[index146].LogType[index147] == 3 & self.game.Data.UnitObj[index146].LogData1[index147] > -1)
                    {
                      num105 += self.game.Data.UnitObj[index146].LogData3[index147];
                      numArray89: Vec<i32> = numArray82;
                      numArray90: Vec<i32> = numArray89;
                      let mut historical: i32 = self.game.Data.UnitObj[index146].Historical;
                      let mut index156: i32 = historical;
                      int[] logData1 = self.game.Data.UnitObj[index146].LogData1;
                      int[] numArray91 = logData1;
                      let mut index157: i32 = index147;
                      let mut index158: i32 = index157;
                      let mut index159: i32 = numArray91[index158];
                      let mut num108: i32 = numArray89[historical, logData1[index157]] + self.game.Data.UnitObj[index146].LogData3[index147];
                      numArray90[index156, index159] = num108;
                    }
                  }
                }
              }
              if (num105 > 0 | self.game.Data.UnitObj[unr].IsHQ | self.detailnr >= 14 & self.detailnr <= 15)
              {
                let mut num109: i32 = self.game.HandyFunctionsObj.HowmanyHQsAbove(unr);
                numArray78[self.game.Data.UnitObj[unr].Historical] = num109;
                numArray79[self.game.Data.UnitObj[unr].Historical] = self.game.HandyFunctionsObj.HowmanyHQsBelow(unr);
                flagArray7[self.game.Data.UnitObj[unr].Historical] = true;
                int[] numArray92 = numArray75;
                int[] numArray93 = numArray92;
                let mut historical13: i32 = self.game.Data.UnitObj[unr].Historical;
                let mut index160: i32 = historical13;
                let mut num110: i32 = numArray92[historical13] + num105;
                numArray93[index160] = num110;
                let mut hq: i32 = self.game.Data.UnitObj[unr].HQ;
                if (hq > -1)
                {
                  int[] numArray94 = numArray76;
                  int[] numArray95 = numArray94;
                  let mut historical14: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index161: i32 = historical14;
                  let mut num111: i32 = numArray94[historical14] + num105;
                  numArray95[index161] = num111;
                  let mut num112: i32 = self.game.Data.ReinfCounter + 1;
                  for (let mut index162: i32 = 0; index162 <= num112; index162 += 1)
                  {
                    numArray96: Vec<i32> = numArray81;
                    numArray97: Vec<i32> = numArray96;
                    let mut historical15: i32 = self.game.Data.UnitObj[hq].Historical;
                    let mut index163: i32 = historical15;
                    let mut index164: i32 = index162;
                    let mut index165: i32 = index164;
                    let mut num113: i32 = numArray96[historical15, index164] + numArray82[self.game.Data.UnitObj[unr].Historical, index162];
                    numArray97[index163, index165] = num113;
                  }
                  int[] numArray98 = numArray77;
                  int[] numArray99 = numArray98;
                  let mut historical16: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index166: i32 = historical16;
                  let mut num114: i32 = numArray98[historical16] + num105;
                  numArray99[index166] = num114;
                }
                if (hq > -1)
                  hq = self.game.Data.UnitObj[hq].HQ;
                for (; hq > -1; hq = self.game.Data.UnitObj[hq].HQ)
                {
                  let mut num115: i32 = self.game.Data.ReinfCounter + 1;
                  for (let mut index167: i32 = 0; index167 <= num115; index167 += 1)
                  {
                    numArray100: Vec<i32> = numArray81;
                    numArray101: Vec<i32> = numArray100;
                    let mut historical17: i32 = self.game.Data.UnitObj[hq].Historical;
                    let mut index168: i32 = historical17;
                    let mut index169: i32 = index167;
                    let mut index170: i32 = index169;
                    let mut num116: i32 = numArray100[historical17, index169] + numArray82[self.game.Data.UnitObj[unr].Historical, index167];
                    numArray101[index168, index170] = num116;
                  }
                  int[] numArray102 = numArray77;
                  int[] numArray103 = numArray102;
                  let mut historical18: i32 = self.game.Data.UnitObj[hq].Historical;
                  let mut index171: i32 = historical18;
                  let mut num117: i32 = numArray102[historical18] + num105;
                  numArray103[index171] = num117;
                }
              }
            }
          }
          bool[] flagArray8 = new bool[self.game.Data.HistoricalUnitCounter + 1];
          let mut unitCounter12: i32 = self.game.Data.UnitCounter;
          for (let mut tdata: i32 = 0; tdata <= unitCounter12; tdata += 1)
          {
            if (self.detailnr4 < 1 | self.game.Data.UnitObj[tdata].IsHQ && self.game.HandyFunctionsObj.GetRegime(self.game.Data.UnitObj[tdata].Regime) == self.game.HandyFunctionsObj.GetRegime(self.game.Data.Turn) && self.game.Data.UnitObj[tdata].PreDef == -1 & self.game.Data.UnitObj[tdata].Historical > -1 && !flagArray8[self.game.Data.UnitObj[tdata].Historical])
            {
              let mut num118: i32 = numArray77[self.game.Data.UnitObj[tdata].Historical];
              let mut num119: i32 = numArray76[self.game.Data.UnitObj[tdata].Historical];
              let mut num120: i32 = numArray75[self.game.Data.UnitObj[tdata].Historical];
              flagArray8[self.game.Data.UnitObj[tdata].Historical] = true;
              let mut num121: i32 = numArray78[self.game.Data.UnitObj[tdata].Historical];
              let mut num122: i32 = num121;
              str13: String;
              for (let mut index: i32 = 1; index <= num122; index += 1)
                str13 += "---";
              num19 = 0;
              let mut index172: i32 = !self.game.Data.UnitObj[tdata].IsHQ ? self.game.Data.UnitObj[tdata].HQ : tdata;
              let mut num123: i32 = 1;
              if (!self.game.Data.UnitObj[tdata].IsHQ)
                num123 += 5;
              for (; index172 > -1; index172 = self.game.Data.UnitObj[index172].HQ)
              {
                let mut num124: i32 = numArray79[self.game.Data.UnitObj[index172].Historical];
                let mut num125: i32 = 1;
                let mut num126: i32 = num124;
                for (let mut index173: i32 = 0; index173 <= num126; index173 += 1)
                  num125 *= 1000;
                let mut num127: i32 = num125 * numArray80[self.game.Data.UnitObj[index172].Historical];
                num123 += num127;
              }
              let mut tWeight: i32 = 0;
              str13 = "";
              let mut num128: i32 = num121;
              for (let mut index174: i32 = 1; index174 <= num128; index174 += 1)
              {
                str13 += Strings.Space(6);
                index172 += 1;
              }
              if (self.game.Data.UnitObj[tdata].IsHQ)
                tWeight = tWeight + 0 + num123;
              else if (self.game.Data.UnitObj[tdata].HQ > -1)
                tWeight = tWeight + 1 + num123;
              if (num118 + num119 + num120 > 0 & self.detailnr <= 13)
              {
                num103 += 1;
                if (self.detailhisnr == -1)
                  self.detailhisnr = tdata;
                if (self.detailhisnr == tdata)
                  TempInt = num103;
                if (self.detailnr <= 13)
                  self.OptionsList6Obj.add(str13 + self.game.Data.UnitObj[tdata].Name, tdata, num118.ToString(), num120.ToString(), tWeight: tWeight);
              }
              else if (self.detailnr == 14)
              {
                num103 += 1;
                if (self.detailhisnr == -1)
                  self.detailhisnr = tdata;
                if (self.detailhisnr == tdata)
                  TempInt = num103;
                self.OptionsList6Obj.add(str13 + self.game.Data.UnitObj[tdata].Name, tdata, Conversions.ToString(self.game.Data.UnitObj[tdata].SupplyInReq + self.game.Data.UnitObj[tdata].supplyBaseSupplyIn), Conversions.ToString(self.game.Data.UnitObj[tdata].SupplyIn + self.game.Data.UnitObj[tdata].supplyBaseSupplyIn), tWeight: tWeight);
              }
              else if (self.detailnr == 15)
              {
                num103 += 1;
                if (self.detailhisnr == -1)
                  self.detailhisnr = tdata;
                if (self.detailhisnr == tdata)
                  TempInt = num103;
                self.OptionsList6Obj.add(str13 + self.game.Data.UnitObj[tdata].Name, tdata, Conversions.ToString(self.game.Data.UnitObj[tdata].FuelRequested + self.game.Data.UnitObj[tdata].supplyBaseFuelIn), Conversions.ToString(self.game.Data.UnitObj[tdata].FuelReceived + self.game.Data.UnitObj[tdata].supplyBaseFuelIn), tWeight: tWeight);
              }
            }
          }
        }
        let mut tlistselect17: i32 = self.OptionsList6Obj.SortWithRefOnWeightAndName(TempInt);
        let mut upperBound1: i32 = self.OptionsList6Obj.ListData.GetUpperBound(0);
        for (let mut index: i32 = 0; index <= upperBound1; index += 1)
        {
          if (self.OptionsList6Obj.ListData[index] == self.detailhisnr)
          {
            self.nextdetailnrhis = index >= self.OptionsList6Obj.ListData.GetUpperBound(0) ? self.OptionsList6Obj.ListData[index] : self.OptionsList6Obj.ListData[index + 1];
            self.prevdetailnrhis = index <= 0 ? self.detailhisnr : self.OptionsList6Obj.ListData[index - 1];
          }
        }
        if (num103 == -1)
        {
          self.RemoveSubPart(self.optionslist6id);
          self.optionslist6id = 0;
        }
        let mut num129: i32 =  Math.Round( (self.h - 80) / 16.0) - 1;
        if (self.optionslist6id > 0)
        {
          self.SubPartList[self.SubpartNr(self.optionslist6id)].Refresh(self.OptionsList6Obj, tlistselect17);
          self.SubPartFlag[self.SubpartNr(self.optionslist6id)] = true;
        }
        else if (num103 > -1)
        {
          ListClass optionsList6Obj = self.OptionsList6Obj;
          let mut tlistsize: i32 = num129;
          let mut twidth: i32 = 320 +  Math.Round( (self.w - 880) / 2.0);
          let mut tlistselect18: i32 = tlistselect17;
          let mut game: GameClass = self.game;
          let mut tValueWidth: i32 = 100 +  Math.Round( (self.w - 880) / 5.0);
           local17: Bitmap =  self.OwnBitmap;
          let mut bbx: i32 = num1 + 220;
          font: Font =  null;
           local18: Font =  font;
          tsubpart1 =  new ListSubPartClass(optionsList6Obj, tlistsize, twidth, tlistselect18, game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local17), bbx: bbx, bby: 0, tMarcStyle: true, overruleFont: ( local18));
          self.optionslist6id = self.AddSubPart( tsubpart1, num1 + 220, 16, 320 +  Math.Round( (self.w - 880) / 2.0), (num129 + 1) * 16, 0);
        }
        else
        {
          if (self.detailnr == 11)
            str2 = "No unit that received replacements";
          if (self.detailnr == 12)
            str2 = "No unit that requested replacements";
          if (self.detailnr == 13)
            str2 = "No unit that returned troops";
          if (self.detailnr == 14)
            str2 = "No unit that has a supply logbook";
          if (self.detailnr == 15)
            str2 = "No unit that has a fuel logbook";
          DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont2, num1 + 260, 90, Color.FromArgb(177,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        }
        if (self.optionslist6id > 0)
        {
          let mut x15: i32 = num1 + 220 + 10;
          DrawMod.DrawTextColouredMarc( g, "UNIT / HQ", self.game.MarcFont5, x15, 3, Color.White);
          let mut x16: i32 = num1 + 220 + 320 +  Math.Round( (self.w - 880) / 2.0) -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          if (self.detailnr == 11)
            DrawMod.DrawTextColouredMarc( g, "RECEIVED", self.game.MarcFont5, x16, 3, Color.White);
          if (self.detailnr == 12)
            DrawMod.DrawTextColouredMarc( g, "REQUESTED", self.game.MarcFont5, x16, 3, Color.White);
          if (self.detailnr == 13)
            DrawMod.DrawTextColouredMarc( g, "RETURNED", self.game.MarcFont5, x16, 3, Color.White);
          if (self.detailnr == 14)
            DrawMod.DrawTextColouredMarc( g, "RECEIVED", self.game.MarcFont5, x16, 3, Color.White);
          if (self.detailnr == 15)
            DrawMod.DrawTextColouredMarc( g, "RECEIVED", self.game.MarcFont5, x16, 3, Color.White);
          let mut x17: i32 = x16 -  Math.Round( (100 +  Math.Round( (self.w - 880) / 5.0)) / 2.0);
          if (self.detailnr == 11)
            DrawMod.DrawTextColouredMarc( g, "REC CUM.", self.game.MarcFont5, x17, 3, Color.White);
          if (self.detailnr == 12)
            DrawMod.DrawTextColouredMarc( g, "REQ CUM.", self.game.MarcFont5, x17, 3, Color.White);
          if (self.detailnr == 13)
            DrawMod.DrawTextColouredMarc( g, "RET CUM.", self.game.MarcFont5, x17, 3, Color.White);
          if (self.detailnr == 14)
            DrawMod.DrawTextColouredMarc( g, "REQUEST", self.game.MarcFont5, x17, 3, Color.White);
          if (self.detailnr == 15)
            DrawMod.DrawTextColouredMarc( g, "REQUEST", self.game.MarcFont5, x17, 3, Color.White);
        }
        if (self.detailhisnr > -1 & self.detailhisnr <= self.game.Data.UnitCounter)
        {
          if (self.detailnr >= 11 & self.detailnr <= 13)
          {
            self.OptionsList7Obj = ListClass::new();
            let mut tdata1: i32 = -1;
            SimpleList simpleList = SimpleList::new();
            if (self.game.Data.UnitObj[self.detailhisnr].IsHQ)
            {
              let mut tdata2: i32 = tdata1 + 1;
              let mut num130: i32 = 0;
              self.OptionsList7Obj.add("For subordinates:", tdata2);
              let mut num131: i32 = self.game.Data.ReinfCounter + 1;
              for (let mut index: i32 = 0; index <= num131; index += 1)
              {
                if (numArray81[self.game.Data.UnitObj[self.detailhisnr].Historical, index] > 0)
                {
                  tname: String = (numArray81[self.game.Data.UnitObj[self.detailhisnr].Historical, index] * self.game.Data.ReinfRatio[index]).ToString() + "x " + self.game.Data.ReinfName[index];
                  tdata2 += 1;
                  num130 += 1;
                  self.OptionsList7Obj.add(tname, tdata2);
                }
              }
              if (num130 == 0)
              {
                tdata2 += 1;
                self.OptionsList7Obj.add("-none-", tdata2);
              }
              let mut tdata3: i32 = tdata2 + 1;
              self.OptionsList7Obj.add("", tdata3);
              tdata1 = tdata3 + 1;
              self.OptionsList7Obj.add("Unit itself:", tdata1);
            }
            tlistselect17 = -1;
            let mut unitCounter: i32 = self.game.Data.UnitCounter;
            for (let mut tid: i32 = 0; tid <= unitCounter; tid += 1)
            {
              if (self.game.Data.UnitObj[tid].PreDef == -1 & self.game.Data.UnitObj[tid].Historical == self.game.Data.UnitObj[self.detailhisnr].Historical)
                simpleList.Add(tid, 1);
            }
            let mut num132: i32 = 0;
            let mut peopleCounter: i32 = self.game.Data.PeopleCounter;
            for (let mut index175: i32 = 0; index175 <= peopleCounter; index175 += 1)
            {
              let mut num133: i32 = self.game.Data.ReinfCounter + 1;
              for (let mut index176: i32 = 0; index176 <= num133; index176 += 1)
              {
                let mut num134: i32 = 0;
                let mut counter: i32 = simpleList.Counter;
                for (let mut index177: i32 = 0; index177 <= counter; index177 += 1)
                {
                  let mut index178: i32 = simpleList.Id[index177];
                  if (self.game.Data.UnitObj[index178].PreDef == -1 & self.game.Data.UnitObj[index178].Historical == self.game.Data.UnitObj[self.detailhisnr].Historical)
                  {
                    let mut logCounter: i32 = self.game.Data.UnitObj[index178].LogCounter;
                    for (let mut index179: i32 = 0; index179 <= logCounter; index179 += 1)
                    {
                      if (self.game.Data.UnitObj[index178].LogData1[index179] == index176 & self.game.Data.UnitObj[index178].LogData2[index179] == index175 && self.game.Data.UnitObj[index178].LogType[index179] >= 1 & self.game.Data.UnitObj[index178].LogType[index179] <= 3)
                      {
                        if (self.detailnr == 11)
                        {
                          if (self.game.Data.UnitObj[index178].LogType[index179] == 2)
                            num134 += self.game.Data.UnitObj[index178].LogData3[index179] * self.game.Data.ReinfRatio[index176];
                        }
                        else if (self.detailnr == 12)
                        {
                          if (self.game.Data.UnitObj[index178].LogType[index179] == 1)
                            num134 += self.game.Data.UnitObj[index178].LogData3[index179] * self.game.Data.ReinfRatio[index176];
                        }
                        else if (self.detailnr == 13 && self.game.Data.UnitObj[index178].LogType[index179] == 3)
                          num134 += self.game.Data.UnitObj[index178].LogData3[index179] * self.game.Data.ReinfRatio[index176];
                      }
                    }
                  }
                }
                if (num134 > 0)
                {
                  tname: String = num134.ToString() + "x " + self.game.Data.ReinfName[index176] + " (" + Strings.Left(self.game.Data.PeopleObj[index175].Name, 3) + ")";
                  tdata1 += 1;
                  num132 += 1;
                  self.OptionsList7Obj.add(tname, tdata1, tcol: 1);
                }
              }
            }
            if (num132 == 0)
              self.OptionsList7Obj.add("-none-", tdata1 + 1);
          }
          else if (self.detailnr >= 14 & self.detailnr <= 15)
          {
            self.OptionsList7Obj = ListClass::new();
            let mut num135: i32 = -1;
            SimpleList simpleList = SimpleList::new();
            let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[499]));
            let mut tdata4: i32 = num135 + 1;
            let mut num136: i32 = 0;
            if (self.detailnr == 14)
              self.OptionsList7Obj.add("UNIT SUPPLY LOGBOOK:", tdata4);
            if (self.detailnr == 15)
              self.OptionsList7Obj.add("UNIT FUEL LOGBOOK:", tdata4);
            let mut tdata5: i32 = tdata4 + 1;
            self.OptionsList7Obj.add(" ", tdata5);
            let mut length: i32 = self.game.Data.StringListObj[stringListById].Length;
            for (let mut index180: i32 = 0; index180 <= length; index180 += 1)
            {
              if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index180, 3])) == 10 * self.game.Data.UnitObj[self.detailhisnr].Historical + self.game.Data.UnitObj[self.detailhisnr].HistoricalSubPart)
              {
                let mut num137: i32 = 0;
                if (self.detailnr == 14 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index180, 5])) == 1)
                  num137 = 1;
                if (self.detailnr == 15 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index180, 5])) == 2)
                  num137 = 1;
                if (self.detailnr == 14 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index180, 5])) == 7)
                  num137 = 1;
                if (self.detailnr == 15 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[index180, 5])) == 8)
                  num137 = 1;
                if (num137 == 1)
                {
                  String1: String = self.game.Data.StringListObj[stringListById].Data[index180, 11];
                  if (String1.Length > 0 & Strings.InStr(String1, ". ") > 0)
                  {
                    separator: Vec<String> = new string[1]
                    {
                      ". "
                    };
                    strArray2: Vec<String> = String1.Split(separator, StringSplitOptions.None);
                    let mut upperBound2: i32 = strArray2.GetUpperBound(0);
                    for (let mut index181: i32 = 0; index181 <= upperBound2; index181 += 1)
                    {
                      tdata5 += 1;
                      num136 += 1;
                      self.OptionsList7Obj.add(strArray2[index181], tdata5);
                    }
                  }
                }
              }
            }
            let mut num138: i32 = self.game.Data.ReinfCounter + 1;
            for (let mut index: i32 = 0; index <= num138; index += 1)
            {
              if (numArray81[self.game.Data.UnitObj[self.detailhisnr].Historical, index] > 0)
              {
                tname: String = (numArray81[self.game.Data.UnitObj[self.detailhisnr].Historical, index] * self.game.Data.ReinfRatio[index]).ToString() + "x " + self.game.Data.ReinfName[index];
                tdata5 += 1;
                num136 += 1;
                self.OptionsList7Obj.add(tname, tdata5);
              }
            }
            if (num136 == 0)
              self.OptionsList7Obj.add("-none-", tdata5 + 1);
          }
          let mut num139: i32 =  Math.Round( (self.h - 80) / 16.0) - 3;
          if (self.optionslist7id > 0)
          {
            self.SubPartList[self.SubpartNr(self.optionslist7id)].Refresh(self.OptionsList7Obj, tlistselect17);
            self.SubPartFlag[self.SubpartNr(self.optionslist7id)] = true;
          }
          else
          {
            ListClass optionsList7Obj = self.OptionsList7Obj;
            let mut tlistsize: i32 = num139;
            let mut twidth: i32 = 140 +  Math.Round( (self.w - 880) / 2.0);
            let mut game: GameClass = self.game;
             local19: Bitmap =  self.OwnBitmap;
            let mut bbx: i32 = num1 + 560 +  Math.Round( (self.w - 880) / 2.0);
            font: Font =  null;
             local20: Font =  font;
            tsubpart1 =  new ListSubPartClass(optionsList7Obj, tlistsize, twidth, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local19), bbx: bbx, bby: 0, tMarcStyle: true, overruleFont: ( local20));
            self.optionslist7id = self.AddSubPart( tsubpart1, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), 0, 140 +  Math.Round( (self.w - 880) / 2.0), (num139 + 1) * 16, 0);
          }
          tsubpart1 =  new TextButtonPartClass("GO TO UNIT", 140, "Click to center map on this unit.",  self.OwnBitmap, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), (num139 + 2) * 16 - 7, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.text6id = self.AddSubPart( tsubpart1, num1 + 560 +  Math.Round( (self.w - 880) / 2.0), (num139 + 2) * 16 - 7, 140, 40, 1);
        }
      }
      let mut num140: i32 = -1;
      let mut num141: i32 = 0;
      do
      {
        if ( self.game.Data.RuleVar[650 + num141] > 0.0)
          num140 += 1;
        num141 += 1;
      }
      while (num141 <= 2);
      if ( self.game.Data.RuleVar[337] > 0.0)
        num140 += 2;
      let mut num142: i32 = -1;
      if ( self.game.Data.RuleVar[403] > 0.0)
        num142 += 1;
      Rectangle trect2 = Rectangle::new(15, 20, 135, 70);
      self.AddMouse( trect2, "Click to see statistics on troops", "", 1);
      if (num140 > -1)
      {
        trect2 = Rectangle::new(15, 90, 135, 70);
        let mut trect3: &Rectangle = &trect2
        self.AddMouse( trect3, "Click to see statistics on regime variables", "", 2);
      }
      if (num142 > -1)
      {
        trect2 = Rectangle::new(15, 160, 135, 70);
        let mut trect4: &Rectangle = &trect2
        self.AddMouse( trect4, "Click to see statistics on logistics", "", 3);
      }
      if (self.subtabnr == 0)
        DrawMod.DrawBlockGradient( g, 25, 20, 125, 70, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
      else if (self.subtabnr == 1)
        DrawMod.DrawBlockGradient( g, 25, 90, 125, 70, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
      else if (self.subtabnr == 2)
        DrawMod.DrawBlockGradient( g, 25, 160, 125, 70, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
      DrawMod.drawLineDot( g, 150, 0, 150, self.h - 80, Color.White);
      DrawMod.drawLineDot( g, 15, 20, 150, 20, Color.White);
      DrawMod.drawLineDot( g, 15, 90, 150, 90, Color.White);
      if (num140 > -1)
        DrawMod.drawLineDot( g, 15, 160, 150, 160, Color.White);
      if (num142 > -1)
        DrawMod.drawLineDot( g, 15, 230, 150, 230, Color.White);
      DrawMod.DrawTextColouredMarcCenter( g, "Troop Statistics", self.game.MarcFont5, 90, 50, Color.White);
      if (num140 > -1)
        DrawMod.DrawTextColouredMarcCenter( g, "Regime Statistics", self.game.MarcFont5, 90, 120, Color.White);
      if (num142 > -1)
        DrawMod.DrawTextColouredMarcCenter( g, "Logistical Statistics", self.game.MarcFont5, 90, 190, Color.White);
      tsubpart1 =  new MarcRadioPartClass(0, self.detailnr3 == 1, "Switch on/off to show/hide numbers in the statistic grahps.",  self.OwnBitmap, 30, self.h - 160);
      self.Text4id = self.AddSubPart( tsubpart1, 30, self.h - 160, 35, 35, 1);
      DrawMod.DrawTextColouredMarc( g, "Numbers", self.game.MarcFont4, 75, self.h - 150, Color.White);
      self.game.EditObj.statsTab_tab = self.subtabnr;
      self.game.EditObj.statsTab_item = self.detailnr;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            return;
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          if (self.MouseData[mouseCounter] == 1)
          {
            self.detailnr = -1;
            self.subtabnr = 0;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[mouseCounter] == 2)
          {
            self.detailnr = -1;
            self.subtabnr = 1;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[mouseCounter] == 3)
          {
            self.detailnr = -1;
            self.subtabnr = 2;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.OptionsList5id)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num2 != -1 & self.detailnr != num2)
              {
                if (num2 == -2)
                  num2 = -1;
                self.detailnr = num2;
                self.detailhisnr = -1;
                self.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.optionslist6id)
            {
              let mut num3: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num3 > -1 & self.detailhisnr != num3)
              {
                self.detailhisnr = num3;
                self.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.optionslist7id)
            {
              self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text1Id)
            {
              self.detailnr2 = 0;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text2Id)
            {
              self.detailnr2 = 1;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text3Id)
            {
              self.detailnr2 = 2;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.text6id)
            {
              let mut detailhisnr: i32 = self.detailhisnr;
              if (detailhisnr > -1)
              {
                self.game.EditObj.UnitSelected = detailhisnr;
                self.game.SelectX = self.game.Data.UnitObj[detailhisnr].X;
                self.game.SelectY = self.game.Data.UnitObj[detailhisnr].Y;
                self.game.HandyFunctionsObj.SetcornerXY2();
                self.game.EditObj.TempCoordList = CoordList::new();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 69);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                windowReturnClass.SetFlag(true);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.Text5id)
              {
                self += 1.detailnr4;
                if (self.detailnr4 > 1)
                  self.detailnr4 = 0;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.Text4id)
              {
                self += 1.detailnr3;
                if (self.detailnr3 > 1)
                  self.detailnr3 = 0;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
          }
        }
      }
      let mut mouseCounter1: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter1; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] == 999)
        {
          self.game.EditObj.SetViewMode2 = 0;
          windowReturnClass.AddCommand(1, 9);
          windowReturnClass.AddCommand(7, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
