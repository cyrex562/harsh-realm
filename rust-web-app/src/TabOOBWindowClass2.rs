// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabOOBWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TabOOBWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;
     int[] Ounit;
     int[] Oselect;
     int[] Ox;
     int[] Oy;
     int[] Ow;
     int[] Showcount;
     RowHeight: i32;
     MaxLayer: i32;
     RowOffset: i32;
     Olastselect: i32;
     detailnr: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Text4id: i32;
     Text5id: i32;
     text6id: i32;
     Text11Id: i32;
     Text12Id: i32;
     Text13Id: i32;
     Text14id: i32;
     Text15id: i32;
     text16id: i32;
     opt1: i32;
     opt2: i32;
     opt3: i32;
     opt4: i32;
     opt5: i32;
     opt11: i32;
     opt12: i32;
     opt13: i32;
     opt14: i32;
     opt15: i32;
     OptionsList5id: i32;
     ListClass OptionsList5Obj;
     OptionsList6id: i32;
     ListClass OptionsList6Obj;
    pub onlyLand: bool;
     lastUnitCount: i32;
    pub SimpleList UL;
    pub ULselected: i32;

    pub TabOOBWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.Ounit = new int[2];
      self.Oselect = new int[7];
      self.Ox = new int[2];
      self.Oy = new int[2];
      self.Ow = new int[2];
      self.Showcount = new int[7];
      self.ULselected = -1;
      self.detailnr = -1;
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.opt1 = 1;
      self.opt2 = 0;
      self.opt3 = 0;
      self.opt4 = 0;
      self.opt5 = 1;
      self.lastUnitCount = self.game.Data.UnitCounter;
      self.MakeUL();
      self.Ounit = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
      self.Ox = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
      self.Oy = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
      self.Ow = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
      self.Olastselect = -1;
      self.Makeoob();
      let mut unitCounter: i32 = self.game.Data.UnitCounter;
      num1: i32;
      num2: i32;
      num3: i32;
      for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
      {
        if (self.game.Data.UnitObj[unr].PreDef == -1)
        {
          if (self.game.HandyFunctionsObj.HasUnitAirSF(unr))
            num1 += 1;
          if (self.game.HandyFunctionsObj.HasUnitlandSF(unr))
            num2 += 1;
          if (self.game.HandyFunctionsObj.HasUnitNavySF(unr))
            num3 += 1;
        }
      }
      self.onlyLand = false;
      if (num2 > 0 & num3 < 1 & num1 < 1)
        self.onlyLand = true;
      self.dostuff();
    }

    pub fn MakeUL()
    {
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 236, 0, 0));
      AIMatrix aiMatrix = new AIMatrix( DrawMod.TGame.DC2AIObj);
      let mut mapWidth: i32 = DrawMod.TGame.Data.MapObj[0].MapWidth;
      let mut mapHeight: i32 = DrawMod.TGame.Data.MapObj[0].MapHeight;
      data: DataClass = DrawMod.TGame.Data;
      str: String = "Zones";
       local: String =  str;
      let mut libVar: i32 = data.FindLibVar( local, "SE_Data");
      let mut num1: i32 = mapWidth;
      for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 = mapHeight;
        for (let mut index2: i32 = 0; index2 <= num2; index2 += 1)
          aiMatrix.Value[index1, index2] = DrawMod.TGame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar);
      }
      bool flag1 = false;
      if (self.game.EventRelatedObj.Helper_AirEnabled())
        flag1 = true;
      self.UL = SimpleList::new();
      let mut num3: i32 = 1;
      do
      {
        let mut unitCounter: i32 = self.game.Data.UnitCounter;
        for (let mut tid1: i32 = 0; tid1 <= unitCounter; tid1 += 1)
        {
          if (self.game.Data.UnitObj[tid1].PreDef == -1 && self.game.Data.UnitObj[tid1].Regime == self.game.Data.Turn)
          {
            let mut historical: i32 = self.game.Data.UnitObj[tid1].Historical;
            if (historical > -1)
            {
              if (num3 == 1 && self.game.Data.HistoricalUnitObj[historical].Type >= 7)
              {
                self.UL.Add(tid1, 1, 1, -1, -1, CheckExistence: false);
                let mut counter1: i32 = self.UL.Counter;
                self.UL.Add(0, 1, 11, counter1, -1, CheckExistence: false);
                let mut counter2: i32 = self.UL.Counter;
                self.UL.Add(0, 1, 12, counter1, -1, CheckExistence: false);
                let mut counter3: i32 = self.UL.Counter;
                if (flag1)
                  self.UL.Add(0, 1, 13, counter1, -1, CheckExistence: false);
                let mut counter4: i32 = self.UL.Counter;
                self.UL.Add(0, 1, 21, counter1, -1, CheckExistence: false);
                self.UL.Add(0, 1, 22, counter1, -1, CheckExistence: false);
                self.UL.Add(0, 1, 23, counter1, -1, CheckExistence: false);
                self.UL.Add(0, 1, 24, counter1, -1, CheckExistence: false);
                if (flag1)
                  self.UL.Add(0, 1, 31, counter1, -1, CheckExistence: false);
                let mut length: i32 = self.game.Data.StringListObj[stringListById1].Length;
                for (let mut tid2: i32 = 0; tid2 <= length; tid2 += 1)
                {
                  if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[tid2, 8])) == self.game.Data.RegimeObj[self.game.Data.Turn].id)
                  {
                    let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[tid2, 6]));
                    if (id > 0)
                    {
                      let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID(id);
                      if (locationById > -1)
                      {
                        if (self.game.Data.LocObj[locationById].HQ == tid1)
                          self.UL.Add(tid2, 1, 2, counter2, -1, CheckExistence: false);
                        self.UL.Add(tid2, 1, 3, counter3, counter1, CheckExistence: false);
                        if (flag1)
                          self.UL.Add(tid2, 1, 4, counter4, counter1, CheckExistence: false);
                      }
                    }
                  }
                }
              }
              if (num3 == 2 && self.game.Data.HistoricalUnitObj[historical].Type == 5)
              {
                let mut idValue: i32 = self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(1);
                if (idValue > 0)
                {
                  let mut integer: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 26));
                  if (self.game.Data.UnitObj[tid1].HQ > -1)
                  {
                    let mut nr1: i32 = self.UL.FindNr(self.game.Data.UnitObj[tid1].HQ, 1);
                    if (nr1 > -1)
                    {
                      let mut nr2: i32 = self.UL.FindNr(0, integer, nr1);
                      if (nr2 > -1)
                        self.UL.Add(tid1, 1, 1, nr2, -1);
                    }
                  }
                }
                else if (self.game.Data.HistoricalUnitObj[historical].TempVar1 == 1 && self.game.Data.UnitObj[tid1].HQ > -1)
                {
                  let mut nr3: i32 = self.UL.FindNr(self.game.Data.UnitObj[tid1].HQ, 1);
                  let mut tdata1: i32 = 31;
                  if (nr3 > -1)
                  {
                    let mut nr4: i32 = self.UL.FindNr(0, tdata1, nr3);
                    if (nr4 > -1)
                      self.UL.Add(tid1, 1, 1, nr4, -1);
                  }
                }
              }
              if (num3 == 3 && self.game.Data.HistoricalUnitObj[historical].Type < 5)
              {
                bool flag2 = false;
                let mut num4: i32 = self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(1);
                let mut num5: i32 = self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(11);
                if (num4 < 1 | num5 > 0)
                  flag2 = true;
                if (num5 < 1 & self.game.Data.HistoricalUnitObj[historical].BattleGroup > 0)
                  flag2 = false;
                if (!flag2 && self.game.Data.UnitObj[tid1].HQ > -1)
                {
                  let mut nr5: i32 = self.UL.FindNr(self.game.Data.UnitObj[tid1].HQ, 1);
                  if (nr5 > -1)
                  {
                    if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.UL.Id[nr5]].Historical].Type == 5)
                      self.UL.Add(tid1, 1, 1, nr5, -1);
                    else if (nr5 > -1)
                    {
                      let mut tval: i32 = aiMatrix.Value[self.game.Data.UnitObj[tid1].X, self.game.Data.UnitObj[tid1].Y];
                      let mut row: i32 = self.game.Data.StringListObj[stringListById1].FindRow(0, tval);
                      if (row > -1)
                      {
                        if (flag1)
                        {
                          if (self.game.HandyFunctionsObj.HasUnitAirSF(self.game.HandyFunctionsObj.GetPreDef(self.game.Data.HistoricalUnitObj[historical].SubParts[0])))
                          {
                            let mut nr6: i32 = self.UL.FindNr(row, 4, tdata3: nr5);
                            if (nr6 > -1)
                              self.UL.Add(tid1, 1, 1, nr6, -1, CheckExistence: false);
                          }
                          else
                          {
                            let mut nr7: i32 = self.UL.FindNr(row, 3, tdata3: nr5);
                            if (nr7 > -1)
                              self.UL.Add(tid1, 1, 1, nr7, -1, CheckExistence: false);
                          }
                        }
                        else
                        {
                          let mut nr8: i32 = self.UL.FindNr(row, 3, tdata3: nr5);
                          if (nr8 > -1)
                            self.UL.Add(tid1, 1, 1, nr8, -1, CheckExistence: false);
                        }
                      }
                    }
                  }
                }
                if (flag2 && self.game.Data.UnitObj[tid1].HQ > -1)
                {
                  let mut nr: i32 = self.UL.FindNr(self.game.Data.UnitObj[tid1].HQ, 1);
                  if (nr > -1)
                  {
                    let mut num6: i32 = self.UL.Id[nr];
                    let mut tval1: i32 = aiMatrix.Value[self.game.Data.UnitObj[tid1].X, self.game.Data.UnitObj[tid1].Y];
                    let mut tval2: i32 = self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(51);
                    let mut row1: i32 = self.game.Data.StringListObj[stringListById1].FindRow(0, tval2);
                    let mut tdata2: i32 = -1;
                    if (row1 > -1)
                      tdata2 = self.UL.FindNr(row1, 2);
                    if (tdata2 < 0 & tval1 > 0)
                    {
                      let mut row2: i32 = self.game.Data.StringListObj[stringListById1].FindRow(0, tval1);
                      if (row2 > -1)
                        tdata2 = self.UL.FindNr(row2, 2);
                    }
                    if (tdata2 > -1)
                      self.UL.Add(tid1, 1, 1, tdata2, -1);
                  }
                }
              }
            }
          }
        }
        num3 += 1;
      }
      while (num3 <= 3);
      let mut num7: i32 = 1;
      do
      {
        for (let mut counter5: i32 = self.UL.Counter; counter5 >= 0; counter5 += -1)
        {
          if (self.UL.Data1[counter5] > 1)
          {
            bool flag3 = false;
            let mut counter6: i32 = self.UL.Counter;
            for (let mut index: i32 = 0; index <= counter6; index += 1)
            {
              if (counter5 != index && self.UL.Data2[index] == counter5)
                flag3 = true;
            }
            if (!flag3)
            {
              self.UL.RemoveNr(counter5);
              let mut counter7: i32 = self.UL.Counter;
              for (let mut index: i32 = 0; index <= counter7; index += 1)
              {
                if (self.UL.Data2[index] > counter5)
                  self.UL.Data2[index] = self.UL.Data2[index] - 1;
              }
            }
          }
        }
        num7 += 1;
      }
      while (num7 <= 3);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (nr == 40)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList5id)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
        self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsList5id)].GetSelect();
        self.dostuff();
        windowReturnClass.SetFlag(true);
      }
      if (nr == 38)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList5id)].ShiftUp();
        self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
        self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsList5id)].GetSelect();
        if (self.detailnr == -2)
          self.detailnr = -1;
        self.dostuff();
        windowReturnClass.SetFlag(true);
      }
      return windowReturnClass;
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn dostuff()
    {
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 237, 0, 0));
      self.RemoveSubPart(self.Text1Id);
      self.RemoveSubPart(self.Text2Id);
      self.RemoveSubPart(self.Text3Id);
      self.RemoveSubPart(self.Text4id);
      self.RemoveSubPart(self.Text5id);
      self.RemoveSubPart(self.text6id);
      self.RemoveSubPart(self.Text11Id);
      self.RemoveSubPart(self.Text12Id);
      self.RemoveSubPart(self.Text13Id);
      self.RemoveSubPart(self.Text14id);
      self.RemoveSubPart(self.Text15id);
      self.RemoveSubPart(self.text16id);
      let mut num1: i32 =  Math.Round( Math.Max(0, self.w - 1060) / 2.0);
      if (num1 > 200)
        num1 = 200;
      if (self.detailnr <= -1)
      {
        self.RemoveSubPart(self.OptionsList6id);
        self.OptionsList6id = 0;
      }
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      bool flag1 = false;
      if (self.game.EventRelatedObj.Helper_AirEnabled())
        flag1 = true;
      self.ClearMouse();
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, " OOB ", 3);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F4]", 99999);
      DrawMod.DrawBlockGradient2( g, 215 + num1, 20, 654 + Math.Max(0, self.w - num1 - 920), 289 + Math.Max(0, self.h - 380), self.game.MarcCol1, self.game.MarcCol2);
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, 215 + num1, 20, 655 + Math.Max(0, self.w - num1 - 920), 290 + Math.Max(0, self.h - 380), 15, 0);
      let mut num2: i32 = num1 + 16;
      self.OptionsList5Obj = ListClass::new();
      let mut TempInt: i32 = -1;
      let mut num3: i32 = 0;
      self.OptionsList5Obj.add(" OOB", -2);
      if (self.detailnr == -1)
        TempInt = 0;
      let mut num4: i32 = self.game.Data.StringListObj[stringListById2].GetHighestValue(0);
      let mut historicalIdCounter: i32 = self.game.Data.HistoricalIDCounter;
      let mut num5: i32 = 29000;
      bool[] flagArray1 = new bool[num4 + 1];
      bool[] flagArray2 = new bool[historicalIdCounter + 1];
      int[] numArray1 = new int[num5 + 1];
      if (self.lastUnitCount != self.game.Data.UnitCounter)
      {
        self.lastUnitCount = self.game.Data.UnitCounter;
        self.MakeUL();
        self.Ounit = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
        self.Ox = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
        self.Oy = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
        self.Ow = new int[Math.Max(self.game.Data.UnitCounter, self.UL.Counter + 1) + 1];
        self.Makeoob();
        self.dostuff();
      }
      let mut counter1: i32 = self.UL.Counter;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
        self.Ox[index] = -1;
      let mut counter2: i32 = self.UL.Counter;
      for (let mut index1: i32 = 0; index1 <= counter2; index1 += 1)
      {
        if (self.UL.Id[index1] > 0 & self.UL.Data1[index1] == 1)
        {
          let mut historical: i32 = self.game.Data.UnitObj[self.UL.Id[index1]].Historical;
          let mut index2: i32 = self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(2);
          self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(11);
          if (index2 > 0 && !flagArray2[historical])
          {
            flagArray1[index2] = true;
            int[] numArray2 = numArray1;
            int[] numArray3 = numArray2;
            let mut index3: i32 = index2;
            let mut index4: i32 = index3;
            let mut num6: i32 = numArray2[index3] + 1;
            numArray3[index4] = num6;
            flagArray2[historical] = true;
          }
        }
      }
      let mut num7: i32 = num4;
      for (let mut index: i32 = 0; index <= num7; index += 1)
      {
        if (flagArray1[index])
        {
          num3 += 1;
          if (self.detailnr == index)
            TempInt = num3;
          self.OptionsList5Obj.add(self.game.Data.StringListObj[stringListById2].GetData(0, index, 1) + "(" + numArray1[index].ToString() + ")", index);
        }
      }
      let mut tlistselect: i32 = self.OptionsList5Obj.SortWithRef(TempInt);
      if (self.OptionsList5id > 0)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList5id)].Refresh(self.OptionsList5Obj, tlistselect);
        self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
      }
      else
      {
        let mut num8: i32 =  Math.Round(Math.Floor( (self.h - 80) / 24.0));
        if (num2 < 50)
        {
          let mut tsubpart: SubPartClass =  new ListSubPartClass(self.OptionsList5Obj, num8 - 1, 170 + num2, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: 30, bby: 20, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 24);
          self.OptionsList5id = self.AddSubPart( tsubpart, 30, 20, 170 + num2, 24 * num8, 0);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new ListSubPartClass(self.OptionsList5Obj, num8 - 1, 170 + num2, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: 30, bby: 20, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
          self.OptionsList5id = self.AddSubPart( tsubpart, 30, 20, 170 + num2, 24 * num8, 0);
        }
      }
      let mut num9: i32 = 220 + num2;
      SubPartClass tsubpart1;
      if (self.detailnr <= -1)
      {
        let mut tsubpart2: SubPartClass =  new MarcRadioPartClass(0, self.opt11 == 1, "Show average Unit Experience",  self.OwnBitmap, num9, 316);
        self.Text11Id = self.AddSubPart( tsubpart2, num9, 316, 35, 35, 1);
        let mut x1: i32 = num9 + 45;
        DrawMod.DrawTextColouredMarc( g, "XP", self.game.MarcFont4, x1, 325, Color.White);
        let mut num10: i32 = x1 + 45;
        let mut tsubpart3: SubPartClass =  new MarcRadioPartClass(0, self.opt11 == 2, "Show average Unit Morale",  self.OwnBitmap, num10, 316);
        self.Text12Id = self.AddSubPart( tsubpart3, num10, 316, 35, 35, 1);
        let mut x2: i32 = num10 + 45;
        DrawMod.DrawTextColouredMarc( g, "MOR", self.game.MarcFont4, x2, 325, Color.White);
        let mut num11: i32 = x2 + 45;
        let mut tsubpart4: SubPartClass =  new MarcRadioPartClass(0, self.opt11 == 3, "Show average Unit Readiness",  self.OwnBitmap, num11, 316);
        self.Text13Id = self.AddSubPart( tsubpart4, num11, 316, 35, 35, 1);
        let mut x3: i32 = num11 + 45;
        DrawMod.DrawTextColouredMarc( g, "RDN", self.game.MarcFont4, x3, 325, Color.White);
        let mut num12: i32 = x3 + 45;
        let mut tsubpart5: SubPartClass =  new MarcRadioPartClass(0, self.opt11 == 4, "Show average Unit Supply In / Requested %",  self.OwnBitmap, num12, 316);
        self.Text14id = self.AddSubPart( tsubpart5, num12, 316, 35, 35, 1);
        let mut x4: i32 = num12 + 45;
        DrawMod.DrawTextColouredMarc( g, "SUP", self.game.MarcFont4, x4, 325, Color.White);
        let mut num13: i32 = x4 + 45;
        tsubpart1 =  new MarcRadioPartClass(0, self.opt11 == 5, "Show average Unit Hunger Score",  self.OwnBitmap, num13, 316);
        self.Text15id = self.AddSubPart( tsubpart1, num13, 316, 35, 35, 1);
        let mut x5: i32 = num13 + 45;
        DrawMod.DrawTextColouredMarc( g, "HUNGER", self.game.MarcFont4, x5, 325, Color.White);
      }
      else
      {
        let mut tsubpart6: SubPartClass =  new MarcRadioPartClass(0, self.opt5 == 0, "Specific unit data",  self.OwnBitmap, num9, 316);
        self.Text5id = self.AddSubPart( tsubpart6, num9, 316, 35, 35, 1);
        let mut x6: i32 = num9 + 45;
        DrawMod.DrawTextColouredMarc( g, "SPECIFIC", self.game.MarcFont4, x6, 325, Color.White);
        let mut num14: i32 = x6 + 145;
        let mut tsubpart7: SubPartClass =  new MarcRadioPartClass(0, self.opt5 == 1, "Average unit data for selected OOB model",  self.OwnBitmap, num14, 316);
        self.text6id = self.AddSubPart( tsubpart7, num14, 316, 35, 35, 1);
        let mut x7: i32 = num14 + 45;
        DrawMod.DrawTextColouredMarc( g, "AVERAGE", self.game.MarcFont4, x7, 325, Color.White);
      }
      let mut num15: i32 = 190 + num2;
      let mut num16: i32 = 360 +  Math.Round( Math.Max(0, self.w - num2 - 920) / 2.0) + num2;
      let mut num17: i32 = 785 + Math.Max(0, self.w - num2 - 920) + num2;
      let mut num18: i32 = 825 + Math.Max(0, self.w - num2 - 920) + num2;
      let mut num19: i32 = 865 + Math.Max(0, self.w - num2 - 920) + num2;
      let mut counter3: i32 = self.UL.Counter;
      for (let mut index: i32 = 0; index <= counter3; index += 1)
        self.Ox[index] = -1;
      if (self.ULselected == -1 & self.Olastselect > -1)
      {
        bool flag2 = false;
        if (self.UL.Data1[self.Olastselect] == 1 && self.game.Data.UnitObj[self.UL.Id[self.Olastselect]].IsHQ)
          flag2 = true;
        self.ULselected = !flag2 ? self.UL.FindNr(self.game.EditObj.UnitSelected, 1) : self.Olastselect;
      }
      else if (self.ULselected == -1 & self.game.EditObj.UnitSelected > -1)
        self.ULselected = self.UL.FindNr(self.game.EditObj.UnitSelected, 1);
      let mut index5: i32 = -1;
      if (self.ULselected > -1 && self.UL.Data1[self.ULselected] == 1)
      {
        index5 = self.UL.Id[self.ULselected];
        self.Oselect[self.Ounit[self.ULselected]] = self.ULselected;
        for (let mut index6: i32 = self.UL.Data2[self.ULselected]; index6 > -1; index6 = self.UL.Data2[index6])
          self.Oselect[self.Ounit[index6]] = index6;
      }
      SimpleList simpleList1;
      tdata1: i32;
      if (self.detailnr <= -1)
      {
        DrawMod.DrawTextColouredMarc( g, Strings.UCase("Order of Battle (OOB)"), self.game.MarcFont4, 235 + num2, 30, Color.FromArgb(200,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        let mut num20: i32 = num16 - 20 + num2;
        let mut num21: i32 = -self.RowHeight + self.RowOffset;
        let mut maxLayer: i32 = self.MaxLayer;
        for (let mut index7: i32 = 1; index7 <= maxLayer; index7 += 1)
        {
          let mut num22: i32 = 0;
          if (index7 == 1 | self.Oselect[index7 - 1] > -1)
          {
            num21 += self.RowHeight;
            simpleList1 = SimpleList::new();
            let mut counter4: i32 = self.UL.Counter;
            for (let mut tid: i32 = 0; tid <= counter4; tid += 1)
            {
              let mut tweight: i32 = 10;
              bool flag3 = false;
              if (self.UL.Data1[tid] == 1 && self.game.Data.UnitObj[self.UL.Id[tid]].IsHQ)
                flag3 = true;
              if (flag3)
                tweight = 100;
              if (self.Ounit[tid] == index7 & tweight > 0)
              {
                if (index7 == 1)
                  simpleList1.Add(tid, tweight);
                else if (self.Oselect[index7 - 1] > -1 && self.UL.Data2[tid] == self.Oselect[index7 - 1])
                  simpleList1.Add(tid, tweight);
              }
            }
            simpleList1.ReverseSort();
            if (index7 == 1)
              num20 = num16 + 180;
            if (index7 > 1)
              num20 = self.Ox[self.Oselect[index7 - 1]];
            if (simpleList1.Counter > -1)
            {
              num22 = 1;
              let mut num23: i32 =  Math.Round(Conversion.Int( (num18 - num15) /  (simpleList1.Counter + 1)));
              if (num23 > 100)
                num23 = 100;
              let mut num24: i32 =  Math.Round( num20 -  ((simpleList1.Counter + 1) * num23) / 2.0);
              if (num24 + num23 * (simpleList1.Counter + 1) > num19 - num15)
                num24 -= num24 + num23 * (simpleList1.Counter + 1) - (num19 - num15);
              if (num24 < 226)
                num24 += 226 - num24;
              if (num24 < num15)
                num24 = num15;
              let mut num25: i32 = num24 - num23;
              let mut counter5: i32 = simpleList1.Counter;
              for (let mut index8: i32 = 0; index8 <= counter5; index8 += 1)
              {
                num25 += num23;
                let mut num26: i32 =  Math.Round( num25 + ( num23 / 2.0 - 18.0));
                let mut num27: i32 =  Math.Round( num21 + ( self.RowHeight / 2.0 - 18.0));
                self.Ox[simpleList1.Id[index8]] = num26;
                self.Oy[simpleList1.Id[index8]] = num27;
                if (index7 == 1 & simpleList1.Counter == 0)
                  self.Oselect[1] = simpleList1.Id[index8];
                if (index7 == 2 & simpleList1.Counter == 0)
                  self.Oselect[2] = simpleList1.Id[index8];
                if (index7 > 1)
                {
                  let mut index9: i32 = self.UL.Data2[simpleList1.Id[index8]];
                  DrawMod.drawLine( g, num26 + 18, num27 + 18, num26 + 18, self.RowOffset + self.RowHeight * (index7 - 1),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                  DrawMod.drawLine( g, num26 + 18, self.RowOffset + self.RowHeight * (index7 - 1), self.Ox[index9] + 18, self.RowOffset + self.RowHeight * (index7 - 1),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                  DrawMod.drawLine( g, self.Ox[index9] + 18, self.RowOffset + self.RowHeight * (index7 - 1), self.Ox[index9] + 18, self.Oy[index9] + 18,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                }
              }
            }
          }
          if (num22 == 0)
            break;
        }
      }
      else
      {
        data: String = self.game.Data.StringListObj[stringListById2].GetData(0, self.detailnr, 1);
        DrawMod.DrawTextColouredMarc( g, Strings.UCase(data + " units:"), self.game.MarcFont4, 235 + num2, 30, Color.FromArgb(210,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        let mut num28: i32 = 340 + num2;
        if (self.game.Data.UnitCounter > num4)
          num4 = self.game.Data.UnitCounter;
        bool[] flagArray3 = new bool[num4 + 1];
        simpleList1 = SimpleList::new();
        let mut unitCounter1: i32 = self.game.Data.UnitCounter;
        for (let mut tid: i32 = 0; tid <= unitCounter1; tid += 1)
        {
          if (self.game.Data.UnitObj[tid].PreDef == -1 & self.game.Data.UnitObj[tid].Historical > -1 & self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tid].Historical].GiveHisVarValue(2) == self.detailnr)
          {
            let mut tweight: i32 = 0 + 1;
            simpleList1.Add(tid, tweight);
          }
        }
        simpleList1.Sort();
        if (simpleList1.Counter > -1)
        {
          let mut num29: i32 =  Math.Round(Conversion.Int(3900.0 /  (simpleList1.Counter + 7)));
          if (num29 > 50)
            num29 = 50;
          let mut num30: i32 = 60;
          if (num29 < 30)
            num30 = 42;
          let mut num31: i32 =  Math.Round(220.0 /   Math.Round(1.0 +  (num29 * (simpleList1.Counter + 1)) /  Conversion.Int(390)));
          if (num31 > 60)
            num31 = 60;
          let mut num32: i32 = -num29;
          let mut num33: i32 = 0;
          let mut counter6: i32 = simpleList1.Counter;
          for (let mut index10: i32 = 0; index10 <= counter6; index10 += 1)
          {
            num32 += num29;
            if (num32 < 10 & index10 == 0)
              num32 = 10;
            if (num32 > 390)
            {
              num32 = 10;
              num33 += num31;
            }
            self.Ox[simpleList1.Id[index10]] = 235 + num32 + num2;
            self.Oy[simpleList1.Id[index10]] = 65 + num33;
          }
        }
        SimpleList Expression = SimpleList::new();
        SimpleList simpleList2 = SimpleList::new();
        if (self.opt5 == 0)
        {
          if (index5 > -1)
          {
            let mut historical: i32 = self.game.Data.UnitObj[index5].Historical;
            if (historical > -1)
            {
              let mut ToeTypeId: i32 = self.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(2);
              if (ToeTypeId > 0)
              {
                Expression = self.game.EventRelatedObj.Helper_GetReinfListForToe(ToeTypeId);
                let mut counter7: i32 = Expression.Counter;
                for (let mut index11: i32 = 0; index11 <= counter7; index11 += 1)
                {
                  let mut reinfTypeById: i32 = self.game.HandyFunctionsObj.GetReinfTypeByID(Expression.Id[index11]);
                  Expression.Weight[index11] = Expression.Weight[index11] * self.game.Data.ReinfRatio[reinfTypeById];
                }
                let mut unitCounter2: i32 = self.game.Data.UnitCounter;
                for (let mut index12: i32 = 0; index12 <= unitCounter2; index12 += 1)
                {
                  if (self.game.Data.UnitObj[index12].Historical == historical & self.game.Data.UnitObj[index12].PreDef == -1)
                  {
                    let mut sfCount: i32 = self.game.Data.UnitObj[index12].SFCount;
                    for (let mut index13: i32 = 0; index13 <= sfCount; index13 += 1)
                    {
                      let mut sf: i32 = self.game.Data.UnitObj[index12].SFList[index13];
                      let mut type: i32 = self.game.Data.SFObj[sf].Type;
                      let mut tid: i32 = self.game.Data.ReinfId[self.game.Data.SFTypeObj[type].ReinforcementType];
                      if (Expression.FindNr(tid) == -1)
                      {
                        let mut reinforcementType2: i32 = self.game.Data.SFTypeObj[type].ReinforcementType2;
                        if (simpleList2.FindNr(reinforcementType2) > -1)
                        {
                          int[] weight = simpleList2.Weight;
                          int[] numArray4 = weight;
                          let mut nr: i32 = simpleList2.FindNr(reinforcementType2);
                          let mut index14: i32 = nr;
                          let mut num34: i32 = weight[nr] + self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio;
                          numArray4[index14] = num34;
                        }
                        else
                          simpleList2.Add(reinforcementType2, self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio);
                      }
                      else if (simpleList2.FindNr(tid) > -1)
                      {
                        int[] weight = simpleList2.Weight;
                        int[] numArray5 = weight;
                        let mut nr: i32 = simpleList2.FindNr(tid);
                        let mut index15: i32 = nr;
                        let mut num35: i32 = weight[nr] + self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio;
                        numArray5[index15] = num35;
                      }
                      else
                        simpleList2.Add(tid, self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio);
                    }
                  }
                }
              }
            }
          }
        }
        else if (self.opt5 == 1)
        {
          let mut detailnr: i32 = self.detailnr;
          bool[] flagArray4 = new bool[self.game.Data.HistoricalUnitCounter + 1];
          if (detailnr > -1)
          {
            Expression = self.game.EventRelatedObj.Helper_GetReinfListForToe(detailnr);
            let mut counter8: i32 = Expression.Counter;
            for (let mut index16: i32 = 0; index16 <= counter8; index16 += 1)
            {
              let mut reinfTypeById: i32 = self.game.HandyFunctionsObj.GetReinfTypeByID(Expression.Id[index16]);
              Expression.Weight[index16] = Expression.Weight[index16] * self.game.Data.ReinfRatio[reinfTypeById];
            }
            let mut unitCounter3: i32 = self.game.Data.UnitCounter;
            num36: i32;
            for (let mut index17: i32 = 0; index17 <= unitCounter3; index17 += 1)
            {
              if (self.game.Data.UnitObj[index17].Historical > -1 & self.game.Data.UnitObj[index17].PreDef == -1 & self.game.Data.UnitObj[index17].Regime == self.game.Data.Turn && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index17].Historical].GiveHisVarValue(2) == detailnr)
              {
                let mut index18: i32 = index17;
                if (index18 > -1)
                {
                  if (!flagArray4[self.game.Data.UnitObj[index17].Historical])
                  {
                    flagArray4[self.game.Data.UnitObj[index17].Historical] = true;
                    num36 += 1;
                  }
                  let mut sfCount: i32 = self.game.Data.UnitObj[index18].SFCount;
                  for (let mut index19: i32 = 0; index19 <= sfCount; index19 += 1)
                  {
                    let mut sf: i32 = self.game.Data.UnitObj[index18].SFList[index19];
                    let mut type: i32 = self.game.Data.SFObj[sf].Type;
                    let mut tid: i32 = self.game.Data.ReinfId[self.game.Data.SFTypeObj[type].ReinforcementType];
                    if (Expression.FindNr(tid) == -1)
                    {
                      let mut reinforcementType2: i32 = self.game.Data.SFTypeObj[type].ReinforcementType2;
                      if (reinforcementType2 > -1)
                      {
                        if (simpleList2.FindNr(reinforcementType2) > -1)
                        {
                          int[] weight = simpleList2.Weight;
                          int[] numArray6 = weight;
                          let mut nr: i32 = simpleList2.FindNr(reinforcementType2);
                          let mut index20: i32 = nr;
                          let mut num37: i32 = weight[nr] + self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio;
                          numArray6[index20] = num37;
                        }
                        else
                          simpleList2.Add(reinforcementType2, self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio);
                      }
                    }
                    else if (tid > -1)
                    {
                      if (simpleList2.FindNr(tid) > -1)
                      {
                        int[] weight = simpleList2.Weight;
                        int[] numArray7 = weight;
                        let mut nr: i32 = simpleList2.FindNr(tid);
                        let mut index21: i32 = nr;
                        let mut num38: i32 = weight[nr] + self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio;
                        numArray7[index21] = num38;
                      }
                      else
                        simpleList2.Add(tid, self.game.Data.SFObj[sf].Qty * self.game.Data.SFTypeObj[type].Ratio);
                    }
                  }
                }
              }
            }
            let mut counter9: i32 = simpleList2.Counter;
            for (let mut index22: i32 = 0; index22 <= counter9; index22 += 1)
              simpleList2.Weight[index22] =  Math.Round( simpleList2.Weight[index22] /  num36);
          }
        }
        self.OptionsList6Obj = ListClass::new();
        if (self.opt5 == 0 & index5 > -1)
        {
          self.OptionsList6Obj.add("SPECIFIC UNIT", 0);
          self.OptionsList6Obj.add(self.game.Data.UnitObj[index5].Name, 0);
        }
        else if (self.opt5 == 1)
        {
          self.OptionsList6Obj.add("AVERAGE UNIT", 0);
          self.OptionsList6Obj.add(self.game.Data.StringListObj[stringListById2].GetData(0, self.detailnr, 1), 0);
        }
        if (self.opt5 == 1 | index5 > -1 & !Information.IsNothing( Expression))
        {
          self.OptionsList6Obj.add("", 0);
          self.OptionsList6Obj.add("TYPE", 0, "TOE", "CUR", "MISS");
          let mut reinfCounter: i32 = self.game.Data.ReinfCounter;
          for (let mut index23: i32 = 0; index23 <= reinfCounter; index23 += 1)
          {
            let mut tid: i32 = self.game.Data.ReinfId[index23];
            tdata1 = Expression.FindNr(tid);
            if (tdata1 > -1)
            {
              tdata1 = Expression.Weight[tdata1];
              let mut nr: i32 = simpleList2.FindNr(tid);
              num39: i32;
              str: String;
              if (nr > -1)
              {
                num39 = simpleList2.Weight[nr];
                let mut num40: i32 = tdata1 - num39;
                str = num40 != 0 ? (num40 <= 0 ? "(+" + Math.Abs(num40).ToString() + ")" : num40.ToString()) : "-";
              }
              else
              {
                num39 = 0;
                str = tdata1.ToString();
              }
              if (Information.IsNothing( str))
                str = "0";
              self.OptionsList6Obj.add(self.game.Data.ReinfName[index23], 0, tdata1.ToString(), num39.ToString(), str);
            }
          }
        }
        if (self.OptionsList6id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList6id)].Refresh(self.OptionsList6Obj, -1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList6id)] = true;
        }
        else
        {
          ListClass optionsList6Obj = self.OptionsList6Obj;
          let mut twidth: i32 = 200 + Math.Max(0, self.w - num2 - 1060);
          let mut game: GameClass = self.game;
          let mut tValueWidth: i32 = 100 + Math.Max(0,  Math.Round( (self.w - num2 - 1060) / 3.0));
           local1: Bitmap =  self.OwnBitmap;
          let mut bbx: i32 = 660 + num2;
          font: Font =  null;
           local2: Font =  font;
          tsubpart1 =  new ListSubPartClass(optionsList6Obj, 15, twidth, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: 36, tMarcStyle: true, overruleFont: ( local2));
          self.OptionsList6id = self.AddSubPart( tsubpart1, 660 + num2, 36, 200 + Math.Max(0, self.w - 1060), 256, 0);
        }
      }
      if (self.detailnr <= -1)
      {
        let mut counter10: i32 = self.UL.Counter;
        for (let mut tdata2: i32 = 0; tdata2 <= counter10; tdata2 += 1)
        {
          if (self.Ox[tdata2] > -1)
          {
            str: String = "Auxilliary";
            tstring: String = "Aux";
            if (self.UL.Data1[tdata2] == 1)
              str = self.game.Data.UnitObj[self.UL.Id[tdata2]].Name;
            else if (self.UL.Data1[tdata2] == 2 | self.UL.Data1[tdata2] == 3 | self.UL.Data1[tdata2] == 4)
            {
              str = self.game.Data.StringListObj[stringListById1].Data[self.UL.Id[tdata2], 7];
              tstring = Strings.Left(str, 4) + ".";
            }
            else
            {
              if (self.UL.Data1[tdata2] == 11)
              {
                str = "Militia Forces";
                tstring = "Mil";
              }
              if (self.UL.Data1[tdata2] == 12)
              {
                str = "Independent Regular Units";
                tstring = "Ind";
              }
              if (self.UL.Data1[tdata2] == 13)
              {
                str = "Independent Air Units";
                tstring = "Air";
              }
              if (self.UL.Data1[tdata2] == 21)
              {
                str = "Infantry Formations";
                tstring = "Inf";
              }
              if (self.UL.Data1[tdata2] == 22)
              {
                str = "Mobile Formations";
                tstring = "Mob";
              }
              if (self.UL.Data1[tdata2] == 23)
              {
                str = "Tank Formations";
                tstring = "Tank";
              }
              if (self.UL.Data1[tdata2] == 24)
              {
                str = "Mechanized Formations";
                tstring = "Mech";
              }
              if (self.UL.Data1[tdata2] == 31)
              {
                str = "Air Commands";
                tstring = "Com";
              }
            }
            bool forcehighlight = false;
            if (self.Olastselect == tdata2)
              forcehighlight = true;
            if (self.UL.Data1[tdata2] == 1)
            {
               let mut local3: &Graphics = &g;
              bitmap: Bitmap = self.game.CustomBitmapObj.DrawUnit(self.UL.Id[tdata2], forcehighlight, ForceHideUnitMode: 2);
               let mut local4: &Bitmap = &bitmap;
              let mut x: i32 = self.Ox[tdata2];
              let mut y: i32 = self.Oy[tdata2];
              DrawMod.DrawSimple( local3,  local4, x, y);
              if (!self.game.Data.UnitObj[self.UL.Id[tdata2]].IsHQ)
              {
                if (self.opt11 == 1)
                {
                  tdata1 = self.game.HandyFunctionsObj.GetAverageXp(self.UL.Id[tdata2]);
                  DrawMod.DrawTextColouredMarcCenter( g, "xp=" + tdata1.ToString(), self.game.MarcFont5, self.Ox[tdata2] + 18, self.Oy[tdata2] + 40, Color.White);
                }
                if (self.opt11 == 2)
                {
                  tdata1 = self.game.HandyFunctionsObj.GetAverageMor(self.UL.Id[tdata2]);
                  DrawMod.DrawTextColouredMarcCenter( g, "mor=" + tdata1.ToString(), self.game.MarcFont5, self.Ox[tdata2] + 18, self.Oy[tdata2] + 40, Color.White);
                }
                if (self.opt11 == 3)
                {
                  tdata1 = self.game.HandyFunctionsObj.GetAverageRdn(self.UL.Id[tdata2]);
                  DrawMod.DrawTextColouredMarcCenter( g, "rdn=" + tdata1.ToString(), self.game.MarcFont5, self.Ox[tdata2] + 18, self.Oy[tdata2] + 40, Color.White);
                }
                if (self.opt11 == 5)
                {
                  tdata1 = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.UL.Id[tdata2]].Historical].GiveHisVarValue(81);
                  DrawMod.DrawTextColouredMarcCenter( g, "hun=" + tdata1.ToString(), self.game.MarcFont5, self.Ox[tdata2] + 18, self.Oy[tdata2] + 40, Color.White);
                }
                if (self.opt11 == 4)
                {
                  tdata1 = 0;
                  let mut num41: i32 = 0;
                  let mut logCounter: i32 = self.game.Data.UnitObj[self.UL.Id[tdata2]].LogCounter;
                  for (let mut index24: i32 = 0; index24 <= logCounter; index24 += 1)
                  {
                    if (self.game.Data.UnitObj[self.UL.Id[tdata2]].LogType[index24] == 202)
                      tdata1 += self.game.Data.UnitObj[self.UL.Id[tdata2]].LogData3[index24];
                    else if (self.game.Data.UnitObj[self.UL.Id[tdata2]].LogType[index24] == 105)
                      num41 += self.game.Data.UnitObj[self.UL.Id[tdata2]].LogData3[index24];
                  }
                  if (tdata1 > 0)
                    tdata1 =  Math.Round( (100 * num41) /  tdata1);
                  else if (tdata1 == 0 & num41 == 0)
                    tdata1 = 100;
                  DrawMod.DrawTextColouredMarcCenter( g, "sup=" + tdata1.ToString(), self.game.MarcFont5, self.Ox[tdata2] + 18, self.Oy[tdata2] + 40, Color.White);
                }
              }
            }
            else
            {
              DrawMod.DrawBlock( g, self.Ox[tdata2], self.Oy[tdata2], 38, 38, 128, 128, 128,  byte.MaxValue);
              DrawMod.DrawTextColouredConsoleCenter( g, tstring, self.game.MarcFont4, self.Ox[tdata2] + 20, self.Oy[tdata2] + 10, self.game.seColWhite);
            }
            Rectangle trect2 = Rectangle::new(self.Ox[tdata2], self.Oy[tdata2], 38, 38);
            self.AddMouse( trect2, "", str, tdata2);
            if (tdata2 == self.Oselect[self.Ounit[tdata2]])
              DrawMod.DrawRectangle( g, self.Ox[tdata2] - 2, self.Oy[tdata2] - 2, 41, 41,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 3);
          }
        }
      }
      else
      {
        let mut counter11: i32 = simpleList1.Counter;
        for (let mut index25: i32 = 0; index25 <= counter11; index25 += 1)
        {
          let mut index26: i32 = simpleList1.Id[index25];
          if (self.Ox[index26] > -1)
          {
            str: String = self.game.Data.UnitObj[index26].Name;
            bool forcehighlight = false;
            if (self.Olastselect == index26)
              forcehighlight = true;
            if (self.game.Data.UnitObj[index26].Historical > -1 && self.game.Data.UnitObj[index26].IsHQ)
            {
              tdata1 = self.game.Data.UnitObj[index26].Historical;
              if (!Information.IsNothing( self.game.Data.HistoricalUnitObj[tdata1].CommanderName) && self.game.Data.HistoricalUnitObj[tdata1].CommanderName.Length > 1)
                str = str + "\r\n" + self.game.Data.HistoricalUnitObj[tdata1].CommanderName;
            }
            ttext: String = str + "\r\n(click to select, double click for selecting HQ)";
            if (self.Olastselect == index26)
              ttext = ttext;
            if (Strings.InStr(self.game.Data.UnitObj[index26].Name, "SS") > 0)
              ttext = ttext;
             let mut local5: &Graphics = &g;
            bitmap: Bitmap = self.game.CustomBitmapObj.DrawUnit(index26, forcehighlight, ForceHideUnitMode: 2);
             let mut local6: &Bitmap = &bitmap;
            let mut x: i32 = self.Ox[index26];
            let mut y: i32 = self.Oy[index26];
            DrawMod.DrawSimple( local5,  local6, x, y);
            tdata1 = self.UL.FindNr(index26);
            if (tdata1 > -1)
            {
              let mut num42: i32 = self.UL.Id[tdata1];
              Rectangle trect3 = Rectangle::new(self.Ox[index26], self.Oy[index26], 38, 38);
              self.AddMouse( trect3, "", ttext, tdata1);
              if (index5 == index26)
                DrawMod.DrawRectangle( g, self.Ox[index26] - 1, self.Oy[index26] - 1, 39, 39,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
            }
          }
        }
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[mouseCounter];
          self.game.EditObj.TipText = self.MouseText[mouseCounter];
          break;
        }
      }
    }

    pub HandleMouseUp: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (b == 2097152)
        return windowReturnClass1;
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > -1 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height && self.MouseData[mouseCounter] > -1 & self.MouseData[mouseCounter] < 99999)
        {
          if (self.Ounit[self.MouseData[mouseCounter]] > -1)
          {
            self.Oselect[self.Ounit[self.MouseData[mouseCounter]]] = self.MouseData[mouseCounter];
            bool flag = false;
            if (self.UL.Data1[self.MouseData[mouseCounter]] == 1 && self.game.Data.UnitObj[self.UL.Id[self.MouseData[mouseCounter]]].IsHQ)
              flag = true;
            if (self.Olastselect == self.MouseData[mouseCounter] | !flag)
            {
              self.Olastselect = self.ULselected;
              if (self.ULselected != self.MouseData[mouseCounter] | flag)
              {
                self.ULselected = self.MouseData[mouseCounter];
                if (self.UL.Data1[self.MouseData[mouseCounter]] == 1)
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  self.game.EditObj.UnitSelected = self.UL.Id[self.MouseData[mouseCounter]];
                  self.game.SelectX = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X;
                  self.game.SelectY = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y;
                  if (self.game.SelectX == -1 && self.game.EditObj.UnitSelected > -1 && self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X == -1)
                  {
                    self.game.EditObj.UnitSelected = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].OnBoard;
                    self.game.SelectX = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X;
                    self.game.SelectY = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y;
                  }
                  self.game.HandyFunctionsObj.SetcornerXY2();
                  let mut num: i32 = self.game.ScreenHeight - self.h;
                  if (self.game.EditObj.GuiDown)
                    num -= 222;
                  if (self.game.EditObj.Zoom == 1)
                    num =  Math.Round( num / 128.0);
                  if (self.game.EditObj.Zoom == 0)
                    num =  Math.Round( num / 64.0);
                  if (self.game.EditObj.Zoom == -1)
                    num =  Math.Round( num / 32.0);
                  self.game.CornerY -=  Math.Round( num / 2.0) - 1;
                  if (self.game.CornerY > self.game.Data.MapObj[0].MapHeight - num)
                    self.game.CornerY = self.game.Data.MapObj[0].MapHeight - num;
                  if (self.game.CornerY < 0)
                    self.game.CornerY = 0;
                  if ( self.game.Data.RuleVar[701] > 0.0)
                  {
                    ScreenClass screeny = self.formref.Screeny;
                    Type type = typeof (MapWindowClass2);
                     Type local =  type;
                    MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                    if (!Information.IsNothing( window))
                    {
                      windowReturnClass2: WindowReturnClass = (WindowReturnClass) window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                      self.game.EditObj.TempCoordList = CoordList::new();
                      windowReturnClass2.AddCommand(4, 12);
                      windowReturnClass2.AddCommand(4, 69);
                      windowReturnClass2.AddCommand(4, 67);
                      windowReturnClass2.AddCommand(4, 68);
                      windowReturnClass2.AddCommand(4, 9);
                      windowReturnClass2.SetFlag(true);
                      return windowReturnClass2;
                    }
                  }
                  else
                  {
                    windowReturnClass1.AddCommand(4, 12);
                    windowReturnClass1.AddCommand(4, 69);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                }
              }
            }
            else
            {
              self.Olastselect = self.MouseData[mouseCounter];
              self.ULselected = self.MouseData[mouseCounter];
            }
            let mut num1: i32 = self.Ounit[self.MouseData[mouseCounter]] + 1;
            let mut maxLayer: i32 = self.MaxLayer;
            for (let mut index: i32 = num1; index <= maxLayer; index += 1)
              self.Oselect[index] = -1;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          if (self.ULselected != self.MouseData[mouseCounter])
          {
            self.Olastselect = self.MouseData[mouseCounter];
            self.ULselected = self.MouseData[mouseCounter];
            if (self.UL.Data1[self.MouseData[mouseCounter]] == 1)
            {
              self.game.EditObj.UnitSelected = self.UL.Id[self.MouseData[mouseCounter]];
              self.game.SelectX = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X;
              self.game.SelectY = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y;
              self.game.HandyFunctionsObj.SetcornerXY2();
              windowReturnClass1.AddCommand(4, 12);
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.AddCommand(4, 67);
              windowReturnClass1.AddCommand(4, 68);
              windowReturnClass1.AddCommand(4, 9);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass1.NoMouseClickBelow = true;
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > -1 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height && self.MouseData[mouseCounter] == 99999)
        {
          self.game.EditObj.SetViewMode2 = 0;
          windowReturnClass.AddCommand(1, 9);
          windowReturnClass.AddCommand(7, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.SetFlag(true);
          windowReturnClass.NoMouseClickBelow = true;
          return windowReturnClass;
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
              switch (num2)
              {
                case -2:
                  num2 = -1;
                  break;
                case -1:
                  self.SubPartFlag[index] = true;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
              }
              self.SubPartFlag[index] = true;
              if (self.detailnr == -1 & num2 > -1)
              {
                self.detailnr = num2;
                self.Makeoob();
              }
              else if (self.detailnr > -1 & num2 == -1)
              {
                self.detailnr = num2;
                self.Makeoob();
              }
              else
                self.detailnr = num2;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text11Id)
            {
              self.opt11 = self.opt11 != 1 ? 1 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text12Id)
            {
              self.opt11 = self.opt11 != 2 ? 2 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text13Id)
            {
              self.opt11 = self.opt11 != 3 ? 3 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text14id)
            {
              self.opt11 = self.opt11 != 4 ? 4 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text15id)
            {
              self.opt11 = self.opt11 != 5 ? 5 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text1Id)
            {
              self.opt1 = self.opt1 != 1 ? 1 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text2Id)
            {
              self.opt2 = self.opt2 != 1 ? 1 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text3Id)
            {
              self.opt3 = self.opt3 != 1 ? 1 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text4id)
            {
              self.opt4 = self.opt4 != 1 ? 1 : 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Text5id)
            {
              self.opt5 = 0;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.text6id)
            {
              self.opt5 = 1;
              let mut olastselect: i32 = self.Olastselect;
              self.Makeoob();
              self.Olastselect = olastselect;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn Makeoob()
    {
      let mut index1: i32 = 1;
      do
      {
        self.Oselect[index1] = -1;
        self.Showcount[index1] = 0;
        index1 += 1;
      }
      while (index1 <= 6);
      self.MaxLayer = 1;
      self.RowHeight = 50;
      let mut counter1: i32 = self.UL.Counter;
      for (let mut index2: i32 = 0; index2 <= counter1; index2 += 1)
      {
        self.Ounit[index2] = -1;
        bool flag = false;
        if (self.UL.Data1[index2] == 1)
        {
          let mut index3: i32 = self.UL.Id[index2];
          if (self.game.Data.UnitObj[index3].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index3].Historical].Type >= 7)
            flag = true;
          if (flag)
          {
            self.Ounit[index2] = 1;
            if (self.MaxLayer < 1)
              self.MaxLayer = 1;
          }
        }
        if (self.UL.Data1[index2] >= 10)
        {
          self.Ounit[index2] = 2;
          if (self.MaxLayer < 2)
            self.MaxLayer = 2;
        }
        if (self.UL.Data1[index2] == 2 | self.UL.Data1[index2] == 3 | self.UL.Data1[index2] == 4)
        {
          self.Ounit[index2] = 3;
          if (self.MaxLayer < 3)
            self.MaxLayer = 3;
        }
      }
      let mut num1: i32 = 1;
      do
      {
        let mut counter2: i32 = self.UL.Counter;
        for (let mut index4: i32 = 0; index4 <= counter2; index4 += 1)
        {
          if (self.UL.Data1[index4] == 1)
          {
            let mut num2: i32 = self.UL.Id[index4];
            if (self.UL.Data2[index4] > -1 & self.Ounit[index4] == -1)
            {
              let mut index5: i32 = self.UL.Data2[index4];
              if (index5 > -1)
              {
                self.Ounit[index4] = self.Ounit[index5] + 1;
                if (self.Ounit[index4] > self.MaxLayer)
                  self.MaxLayer = self.Ounit[index4];
              }
            }
          }
        }
        num1 += 1;
      }
      while (num1 <= 2);
      let mut num3: i32 = self.h - 60;
      self.RowOffset = 0;
      if (num3 > 400)
        self.RowOffset =  Math.Round(  Math.Round( (num3 - 400) * 0.3) / 2.0);
      self.RowHeight =  Math.Round(Conversion.Int( (self.h - (100 + self.RowOffset * 2)) /  self.MaxLayer));
      if (self.RowHeight > 150)
        self.RowHeight = 150;
      self.RowOffset += 20;
      if (self.ULselected <= -1)
        return;
      let mut index6: i32 = self.ULselected;
      self.Olastselect = index6;
      while (index6 > -1)
      {
        if (self.Ounit[index6] > -1)
        {
          self.Oselect[self.Ounit[index6]] = index6;
          index6 = self.UL.Data2[index6];
        }
        else
          index6 = -1;
      }
    }
  }
}
