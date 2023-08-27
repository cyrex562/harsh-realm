// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabManagementWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class TabManagementWindowClass2 : WindowClass
  {
     w: i32;
     h: i32;
     tabnr: i32;
     tabname: String;
     subtabcount: i32;
     subtabname: Vec<String>;
     subtabnr: i32;
     int[] pagecount;
     string[,] pagename;
     pagerow: Vec<i32>;
     pageclickable: Vec<i32>;
     pagenr: i32;
     stringlistslot: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     pageId: i32;
     prevPageNr: i32;
     lastEventNr: String;
     prevPageId: i32;
     curPageId: i32;
     cache_udsManagementOverride: i32;

    pub TabManagementWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.subtabname = new string[11];
      self.pagecount = new int[11];
      self.pagename = new string[11, 211];
      self.pagerow = new int[11, 211];
      self.pageclickable = new int[11, 211];
      self.cache_udsManagementOverride = 0;
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.initTabs();
      self.prevPageNr = self.pagenr;
      self.dostuff();
    }

    pub fn initTabs()
    {
      bool forUDSRandomTab = false;
      self.subtabcount = 0;
      self.subtabname = new string[11];
      self.pagecount = new int[11];
      self.pagename = new string[11, 211];
      self.pagerow = new int[11, 211];
      id: i32;
      if (self.game.EditObj.SetViewMode2 < 100)
      {
        self.tabnr = self.game.EditObj.SetViewMode2 - 10;
        id =  Math.Round( self.game.Data.RuleVar[441]);
      }
      else
      {
        self.tabnr = self.game.EditObj.SetViewMode2 - 100;
        forUDSRandomTab = true;
        id =  Math.Round( self.game.Data.RuleVar[443]);
      }
      if (self.tabnr < 0)
        self.tabnr = 0;
      self.tabname = self.game.HandyFunctionsObj.GetUDSmanagementTabName(self.tabnr, forUDSRandomTab);
      self.stringlistslot = self.game.HandyFunctionsObj.GetStringListByID(id);
      self.subtabcount = 0;
      let mut subTabNr: i32 = 1;
      do
      {
        smanagementSubTabName: String = self.game.HandyFunctionsObj.GetUDSmanagementSubTabName(self.tabnr, subTabNr, forUDSRandomTab);
        if (smanagementSubTabName.Length > 1)
        {
          self += 1.subtabcount;
          self.subtabname[self.subtabcount] = smanagementSubTabName;
          let mut pageNr: i32 = 1;
          do
          {
            let mut integer: i32 = Conversions.ToInteger(self.game.HandyFunctionsObj.GetUDSmanagementPageRow(self.tabnr, self.subtabcount, pageNr, forUDSRandomTab));
            if (integer >= 0)
            {
              let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(id);
              str: String = self.game.Data.StringListObj[stringListById].Data[integer, 4];
              if (str.Length > 1)
              {
                int[] pagecount = self.pagecount;
                int[] numArray = pagecount;
                let mut subtabcount: i32 = self.subtabcount;
                let mut index: i32 = subtabcount;
                let mut num: i32 = pagecount[subtabcount] + 1;
                numArray[index] = num;
                if (self.subtabnr < 1)
                  self.subtabnr = self.subtabcount;
                self.pagename[self.subtabcount, self.pagecount[self.subtabcount]] = str;
                self.pagerow[self.subtabcount, self.pagecount[self.subtabcount]] = integer;
                self.pageclickable[self.subtabcount, self.pagecount[self.subtabcount]] =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[integer, 5])) <= 0 ? 1 : 0;
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[integer, 0])) == self.game.EditObj.udsManagementTabOverrideId)
                {
                  self.subtabnr = self.subtabcount;
                  self.pagenr = self.pagecount[self.subtabcount];
                  self.game.EditObj.uds_subtab[self.tabnr] = self.subtabnr;
                  self.game.EditObj.uds_page[self.tabnr, self.subtabnr] = self.pagenr;
                  self.game.EditObj.udsManagementTabOverrideId = -1;
                }
              }
              else
                pageNr = 999;
            }
            else
              pageNr = 999;
            pageNr += 1;
          }
          while (pageNr <= 200);
        }
        else
          subTabNr = 999;
        subTabNr += 1;
      }
      while (subTabNr <= 8);
      if (self.tabnr < 0)
        self.tabnr = 0;
      if (self.game.EditObj.uds_subtab[self.tabnr] > 0)
        self.subtabnr = self.game.EditObj.uds_subtab[self.tabnr];
      if (self.subtabnr <= 0 || self.game.EditObj.uds_page[self.tabnr, self.subtabnr] <= 0)
        return;
      self.pagenr = self.game.EditObj.uds_page[self.tabnr, self.subtabnr];
    }

    pub fn DoRefresh()
    {
      self.prevPageNr = self.pagenr;
      if (self.OptionsListId > 0)
      {
        self.RemoveSubPart(self.OptionsListId);
        self.OptionsListId = 0;
      }
      self.initTabs();
      self.dostuff();
    }

    pub fn PopUpRefresh()
    {
      let mut num: i32 = 0;
      self.prevPageNr = self.pagenr;
      bool dontDrawPage = false;
      UDSData dyn;
      StringListClass Expression;
      if (self.pageId > 0)
      {
        num = ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).curY;
        UDSPartClass subPart = (UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)];
        dyn = subPart.dyn;
        let mut elementCounter: i32 = subPart.dyn.elementCounter;
        for (let mut index1: i32 = 0; index1 <= elementCounter; index1 += 1)
        {
          if (subPart.dyn.element[index1].type == UDSType.Table)
          {
            StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(subPart.dyn.element[index1].texty))];
            Expression = stringListClass.Clone();
            let mut length: i32 = stringListClass.Length;
            for (let mut index2: i32 = 0; index2 <= length; index2 += 1)
            {
              let mut width: i32 = stringListClass.Width;
              for (let mut index3: i32 = 0; index3 <= width; index3 += 1)
              {
                Expression.TempDesc[index2, index3] = stringListClass.TempDesc[index2, index3];
                Expression.TempBmp[index2, index3] = stringListClass.TempBmp[index2, index3];
              }
            }
          }
        }
        self.RemoveSubPart(self.pageId);
        self.pageId = 0;
        dontDrawPage = true;
      }
      self.initTabs();
      self.dostuff(dontDrawPage);
      if (num > 0 & self.pageId > 0 & dontDrawPage & self.prevPageId == self.curPageId)
        ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).curY = num;
      if (!(self.pageId > 0 & dontDrawPage))
        return;
      UDSPartClass subPart1 = (UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)];
      let mut elementCounter1: i32 = subPart1.dyn.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter1; index += 1)
      {
        if (dyn.elementCounter >= index)
        {
          if (subPart1.dyn.element[index].type == dyn.element[index].type && subPart1.dyn.element[index].x == dyn.element[index].x & subPart1.dyn.element[index].y == dyn.element[index].y)
          {
            subPart1.dyn.element[index].topRow = dyn.element[index].topRow;
            if (subPart1.dyn.element[index].parentElement > -1)
            {
              subPart1.dyn.element[index].grayed = dyn.element[index].grayed;
              subPart1.dyn.element[index].texty = dyn.element[index].texty;
            }
          }
          if (subPart1.dyn.element[index].type == UDSType.Table & !Information.IsNothing( Expression))
          {
            StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(subPart1.dyn.element[index].texty))];
            self.game.Data.StringListObj[ Math.Round(Conversion.Val(subPart1.dyn.element[index].texty))] = Expression;
          }
        }
      }
      subPart1.MakeBitmap();
    }

    pub fn dostuff(bool dontDrawPage = false)
    {
      if (self.stringlistslot == -1)
        return;
      let mut num1: i32 = 0;
      let mut subtabcount1: i32 = self.subtabcount;
      for (let mut index: i32 = 1; index <= subtabcount1; index += 1)
      {
        if (self.pagecount[index] > 0)
          num1 += 1;
      }
      let mut num2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 6]));
      if (num2 > 0)
      {
        let mut num3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 10]));
        let mut num4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 11]));
        let mut num5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 12]));
        let mut num6: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 13]));
        Left: String = num2.ToString() + "," + num3.ToString() + "," + num4.ToString() + "," + num5.ToString() + "," + num6.ToString();
        if (Operators.CompareString(Left, self.lastEventNr, false) != 0 && self.pageId > 0)
        {
          self.RemoveSubPart(self.pageId);
          self.pageId = 0;
        }
        self.lastEventNr = Left;
      }
      else
      {
        if (Operators.CompareString(self.lastEventNr, self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 8], false) != 0 && self.pageId > 0)
        {
          self.RemoveSubPart(self.pageId);
          self.pageId = 0;
        }
        self.lastEventNr = self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 8];
      }
      if (self.prevPageNr != self.pagenr && self.pageId > 0)
      {
        self.RemoveSubPart(self.pageId);
        self.pageId = 0;
      }
      self.ClearMouse();
      if (!dontDrawPage)
        self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, self.tabname, self.game.EditObj.SetViewMode2);
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC]", 999);
      if (num1 == 0)
      {
        DrawMod.DrawTextColouredMarcCenter( g, "No decisions remaining.", self.game.MarcFont4, 100, 40, Color.White);
      }
      else
      {
        if (self.subtabcount < 1)
          return;
        let mut width: i32 =  Math.Round( Math.Min(self.w - 80, 1180) /  self.subtabcount);
        if (width > 250)
          width = 250;
        g.SmoothingMode = SmoothingMode.None;
        DrawMod.drawLineDot( g, 25, 8, 25 + width * self.subtabcount + 25, 8, Color.White);
        DrawMod.drawLineDot( g, 25, 52, 25 + width * self.subtabcount + 25, 52, Color.White);
        DrawMod.drawLineDot( g, 25, 10, 25, 50, Color.White);
        g.SmoothingMode = SmoothingMode.AntiAlias;
        g.PixelOffsetMode = PixelOffsetMode.Half;
        let mut num7: i32 = 0;
        if (self.subtabnr > 0 && self.pagecount[self.subtabnr] < 1)
          self.subtabnr = -1;
        let mut subtabcount2: i32 = self.subtabcount;
        for (let mut index1: i32 = 1; index1 <= subtabcount2; index1 += 1)
        {
          if (self.pagecount[index1] > 0)
          {
            num7 += 1;
            let mut num8: i32 = 25 + (num7 - 1) * width;
            let mut num9: i32 = 10;
            if (self.subtabnr == -1)
              self.subtabnr = index1;
            if (index1 == self.subtabnr)
            {
              DrawMod.DrawBlockGradient( g, num8, num9,  Math.Round( width / 2.0), 40, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
              DrawMod.DrawBlockGradient( g, num8 +  Math.Round( width / 2.0), num9,  Math.Round( width / 2.0), 40, Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue), Color.FromArgb(0, 0, 0, 0));
            }
            SizeF sizeF = SizeF::new();
            let mut num10: i32 =  Math.Round( g.MeasureString(self.subtabname[index1], self.game.MarcFont4).Width);
            Rectangle trect2 = Rectangle::new(num8, num9, width, 40);
            self.AddMouse( trect2, self.subtabname[index1], "Click to see all pages for this tab.", 1000 + index1);
            if (num10 > width - 20 & Strings.InStr(self.subtabname[index1], " ") > 0)
            {
              strArray: Vec<String> = self.subtabname[index1].Split(' ');
              tstring1: String = "";
              tstring2: String = "";
              let mut upperBound: i32 = strArray.GetUpperBound(0);
              for (let mut index2: i32 = 0; index2 <= upperBound; index2 += 1)
              {
                if ( index2 < Math.Max(1.0,  strArray.GetUpperBound(0) / 2.0))
                {
                  if (tstring1.Length > 0)
                    tstring1 += " ";
                  tstring1 += strArray[index2];
                }
                else
                {
                  if (tstring2.Length > 0)
                    tstring2 += " ";
                  tstring2 += strArray[index2];
                }
              }
              DrawMod.DrawTextColouredMarcCenter( g, tstring1, self.game.MarcFont4,  Math.Round( num8 +  width / 2.0), num9 + 3, Color.White);
              DrawMod.DrawTextColouredMarcCenter( g, tstring2, self.game.MarcFont4,  Math.Round( num8 +  width / 2.0), num9 + 20, Color.White);
            }
            else
              DrawMod.DrawTextColouredMarcCenter( g, self.subtabname[index1], self.game.MarcFont4,  Math.Round( num8 +  width / 2.0), num9 + 12, Color.White);
            g.SmoothingMode = SmoothingMode.None;
            DrawMod.drawLineDot( g, num8 + width, num9, num8 + width, num9 + 40, Color.White);
            g.SmoothingMode = SmoothingMode.AntiAlias;
          }
        }
        let mut num11: i32 = 0;
        let mut num12: i32 = 0;
        let mut num13: i32 = 0;
        if (self.w < 1240)
        {
          let mut num14: i32 = 1240 - self.w;
          num11 =  Math.Round( num14 * 0.65);
          let mut num15: i32 =  Math.Round( num14 * 0.35);
          if (num15 > 10)
          {
            num12 = 10;
            num15 -= 10;
          }
          if (num15 > 10)
          {
            num13 = 10;
            num15 -= 10;
          }
          if (num15 > 10)
          {
            num13 += 10;
            num15 -= 10;
          }
          if (num15 > 10)
          {
            num13 += 10;
            let mut num16: i32 = num15 - 10;
          }
        }
        let mut num17: i32 = 25;
        let mut num18: i32 = 74;
        let mut overruleItemSize1: i32 = 24;
        let mut tlistsize1: i32 =  Math.Round( (self.h - 134) /  overruleItemSize1) - 1;
        self.OptionsListObj = ListClass::new();
        if (self.subtabnr < 0)
          return;
        let mut num19: i32 = self.pagecount[self.subtabnr];
        for (let mut tdata: i32 = 1; tdata <= num19; tdata += 1)
        {
          if (self.pagenr < 1)
            self.pagenr = tdata;
          if (self.pageclickable[self.subtabnr, tdata] > 0)
            self.OptionsListObj.add(self.pagename[self.subtabnr, tdata], tdata);
          else
            self.OptionsListObj.add(self.pagename[self.subtabnr, tdata].ToUpper(), -1);
        }
        if (self.pagenr > self.pagecount[self.subtabnr])
          self.pagenr = self.pagecount[self.subtabnr];
        SubPartClass tsubpart;
        if (self.OptionsListId > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, self.pagenr - 1);
          self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        }
        else if (num11 > 10)
        {
          ListClass optionsListObj = self.OptionsListObj;
          let mut tlistsize2: i32 = tlistsize1;
          let mut twidth: i32 = 250 - num11;
          let mut tlistselect: i32 = self.pagenr - 1;
          let mut game: GameClass = self.game;
           local1: Bitmap =  self.OwnBitmap;
          let mut bbx: i32 = num17;
          let mut bby: i32 = num18;
          font: Font =  null;
           local2: Font =  font;
          let mut overruleItemSize2: i32 = overruleItemSize1;
          tsubpart =  new ListSubPartClass(optionsListObj, tlistsize2, twidth, tlistselect, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: bby, tMarcStyle: true, overruleFont: ( local2), overruleItemSize: overruleItemSize2);
          self.OptionsListId = self.AddSubPart( tsubpart, num17, num18, 250 - num11, (tlistsize1 + 1) * overruleItemSize1, 0);
        }
        else
        {
          tsubpart =  new ListSubPartClass(self.OptionsListObj, tlistsize1, 250 - num11, self.pagenr - 1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num17, bby: num18, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: overruleItemSize1);
          self.OptionsListId = self.AddSubPart( tsubpart, num17, num18, 250 - num11, (tlistsize1 + 1) * overruleItemSize1, 0);
        }
        let mut num20: i32 = 293 - (num12 + num11);
        let mut num21: i32 = 74;
        let mut num22: i32 = self.w - (326 - num13) + num11;
        let mut num23: i32 = self.h - 124;
        let mut enr: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 6]));
        if (self.pageId == 0)
        {
          self.game.EditObj.UdsInsideTabOpenMode = false;
          let mut num24: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 9]));
          udStabText: String;
          if (enr > 0)
          {
            let mut tv0: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 10]));
            let mut tv1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 11]));
            let mut tv2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 12]));
            let mut tv3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 13]));
            self.game.EventRelatedObj.DoCheckSpecificEvent(enr, tv0, tv1, tv2, tv3);
            udStabText = self.game.EditObj.UDStabText;
            let mut num25: i32 =  Math.Round(Conversion.Val( Conversions.ToInteger(self.game.Data.StringListObj[self.stringlistslot].GetData(4, self.subtabname[self.subtabnr], 0))));
            if (!(num25 >= 77 & num25 <= 79))
            {
              let mut id: i32 = self.game.EventRelatedObj.CheckStringlistID("SE_IO", 158, 0, 0);
              self.game.EventRelatedObj.IO_AddClear();
              self.game.EventRelatedObj.IO_AddStamp(2, -35, -35);
              str: String = self.game.EventRelatedObj.CheckKey(id, "FINALTEXT", 0, 0);
              udStabText += str;
            }
            self.game.EditObj.UDStabText = "";
          }
          else
            udStabText = self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 8];
          self.prevPageId = self.curPageId;
          self.curPageId =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.stringlistslot].Data[self.pagerow[self.subtabnr, self.pagenr], 0]));
          if (dontDrawPage)
          {
            tsubpart =  new UDSPartClass(self.game, num22, num23, udStabText,  self.OwnBitmap, num20, num21, tAllGray: (num24 == 1), noBitmapDraw: true);
            self.pageId = self.AddSubPart( tsubpart, num20, num21, num22, num23, 0);
          }
          else
          {
            tsubpart =  new UDSPartClass(self.game, num22, num23, udStabText,  self.OwnBitmap, num20, num21, tAllGray: (num24 == 1));
            self.pageId = self.AddSubPart( tsubpart, num20, num21, num22, num23, 0);
          }
        }
        else
          self.SubPartFlag[self.SubpartNr(self.pageId)] = true;
        if (self.subtabnr > -1)
          self.game.EditObj.uds_subtab[self.tabnr] = self.subtabnr;
        else
          self.game.EditObj.uds_subtab[self.tabnr] = -1;
        if (self.subtabnr <= -1)
          return;
        self.game.EditObj.uds_page[self.tabnr, self.subtabnr] = self.pagenr;
      }
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.interfaceCue == 1 & self.game.EditObj.SetViewMode2 > 100)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.game.EditObj.inRandomScreen = false;
        windowReturnClass.AddCommand(3, 12);
        self.game.EditObj.interfaceCue = 0;
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (!(self.game.EditObj.interfaceCue == 5 & self.game.EditObj.SetViewMode2 > 100))
        return windowReturnClass;
      VBMath.Randomize();
      self.game.Data.GameID =  Math.Round( (VBMath.Rnd() * 1E+08f));
      self.game.HandyFunctionsObj.RedimStats();
      self.game.HandyFunctionsObj.DoResMod();
      if (self.game.EditObj.AutoSave)
      {
        self.game.Data.specialSaveMode = 1;
        self.game.EventRelatedObj.ExecSuperImposeMessage("Saving Planet", "Making an Auto-Save of your generated Planet", 0, 0, "");
        str: String = self.game.AppPath_SAVEGAMES + "lastplanetgenerated.se1";
        self.game.Data.serialize(str);
        self.game.HandyFunctionsObj.ZipFile(str);
        GC.Collect();
        Application.DoEvents();
        self.game.Data.specialSaveMode = 0;
      }
      windowReturnClass.AddCommand(3, 13);
      self.game.EditObj.SetViewMode2 = 0;
      self.game.EditObj.inRandomScreen = false;
      self.game.EditObj.interfaceCue = 0;
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          return;
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.game.EditObj.TipButton = false;
          self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (self.game.EditObj.TipButton)
            break;
          let mut num: i32 = self.SubPartID[index];
        }
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false) => WindowReturnClass::new();

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 1000)
          {
            self.subtabnr = self.MouseData[index] - 1000;
            if (self.pageId > 0)
            {
              self.RemoveSubPart(self.pageId);
              self.pageId = 0;
            }
            if (self.OptionsListId > 0)
            {
              self.RemoveSubPart(self.OptionsListId);
              self.OptionsListId = 0;
            }
            self.prevPageNr = self.pagenr;
            if (self.subtabnr > -1 && self.game.EditObj.uds_page[self.tabnr, self.subtabnr] > -1)
              self.pagenr = self.game.EditObj.uds_page[self.tabnr, self.subtabnr];
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[index] == 999)
          {
            if (self.game.EditObj.SetViewMode2 > 100)
            {
              windowReturnClass.AddCommand(1, 9);
              windowReturnClass.AddCommand(7, 12);
              windowReturnClass.AddCommand(4, 114);
            }
            else if (self.game.EditObj.GuiDown | self.game.EditObj.RightDown)
            {
              self.game.EditObj.GuiDown = false;
              self.game.EditObj.RightDown = false;
              self.game.EditObj.SetViewMode2 = 0;
              windowReturnClass.AddCommand(3, 11);
            }
            else
            {
              windowReturnClass.AddCommand(1, 9);
              windowReturnClass.AddCommand(7, 12);
              windowReturnClass.AddCommand(4, 67);
            }
            self.game.EditObj.SetViewMode2 = 0;
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          let mut num1: i32 = self.SubPartID[index];
          if (num1 == self.OptionsListId)
          {
            let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
            self.SubPartFlag[index] = true;
            if (num2 > -1 && self.pagenr != num2)
            {
              self.pagenr = num2;
              if (self.pageId > 0)
              {
                self.RemoveSubPart(self.pageId);
                self.pageId = 0;
              }
            }
            self.prevPageNr = self.pagenr;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.pageId)
          {
            self.game.EditObj.interfaceCue = 0;
            let mut num3: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (num3 > 0)
            {
              bool flag = false;
              if (self.game.EditObj.UdsInsideTabOpenMode & !Information.IsNothing( self.game.EditObj.udsInsideTabOpenModeList) && self.game.EditObj.udsInsideTabOpenModeList.FindNr(num3) > -1)
                flag = true;
              if (self.game.EditObj.UdsInsideTabOpenMode & flag)
              {
                if (self.pageId > 0)
                {
                  self.RemoveSubPart(self.pageId);
                  self.pageId = 0;
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.EditObj.UDSpopupText = "";
              self.formref.Cursor = Cursors.WaitCursor;
              self.game.EventRelatedObj.DoCheckSpecificEvent(num3);
              self.formref.Cursor = Cursors.Default;
              if (self.game.EditObj.interfaceCue == 2)
              {
                if (self.pageId > 0)
                {
                  self.RemoveSubPart(self.pageId);
                  self.pageId = 0;
                }
                self.game.EditObj.interfaceCue = 0;
                if (self.game.EditObj.SetViewMode2 > 100)
                {
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 114);
                  windowReturnClass.AddCommand(4, 116);
                  windowReturnClass.AddCommand(4, 9);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(7, 12);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 9);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.EditObj.UDSpopupText.Length > 1)
              {
                self.game.EditObj.PopupValue = 21;
                self.game.EditObj.udsLastCalledPopupEventNr = num3;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else if (self.game.EditObj.interfaceCue == 2)
            {
              if (self.pageId > 0)
              {
                self.RemoveSubPart(self.pageId);
                self.pageId = 0;
              }
              self.game.EditObj.interfaceCue = 0;
              if (self.game.EditObj.SetViewMode2 > 100)
              {
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 114);
                windowReturnClass.AddCommand(4, 116);
                windowReturnClass.AddCommand(4, 9);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.AddCommand(7, 12);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            self.SubPartFlag[index] = true;
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
