// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabSMapWindowClass2
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
  pub class TabSMapWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;
     detailnr: i32;
     regnr: i32;
     subtab: i32;
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     b3Id: i32;
     b3textid: i32;
     b4Id: i32;
     b4textid: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList3Id: i32;
     ListClass OptionsList3Obj;
     int[] regimesToBeShown;
     int[] zonesToBeShown;

    pub TabSMapWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.regimesToBeShown = new int[1];
      self.zonesToBeShown = new int[1];
      self.w = trect.Width;
      self.h = trect.Height;
      self.game.DC2AIObj.SetTempHexNeighbours();
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.game.EditObj.MiniMap = new Bitmap(10, 10);
      self.prepareTempValue4();
      self.regnr = self.game.Data.Turn;
      self.detailnr = -1;
      if (self.game.SelectX > -1)
        self.detailnr = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime;
      if (self.game.EditObj.se1_StrategyTab > -1)
      {
        self.subtab = self.game.EditObj.se1_StrategyTab;
        if (self.subtab > 0)
          self.detailnr = -1;
      }
      if (self.subtab == 2)
        self.detailnr = self.game.EditObj.se1_StrategySpecial1;
      self.dostuff();
    }

    pub fn DoRefresh() => self.dostuff();

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = -1;
      if (nr == 40)
      {
        self.SubPartList[self.SubpartNr(self.OptionsListId)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        if (self.subtab == 0)
        {
          self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsListId)].GetSelect();
          self.dostuff();
          windowReturnClass.SetFlag(true);
        }
        else
          num1 = self.SubPartList[self.SubpartNr(self.OptionsListId)].GetSelect();
      }
      if (nr == 38)
      {
        self.SubPartList[self.SubpartNr(self.OptionsListId)].ShiftUp();
        self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        if (self.subtab == 0)
        {
          self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsListId)].GetSelect();
          self.dostuff();
          windowReturnClass.SetFlag(true);
        }
        else
          num1 = self.SubPartList[self.SubpartNr(self.OptionsListId)].GetSelect();
      }
      if (num1 > -1)
      {
        bool flag1 = false;
        if (self.subtab == 0)
        {
          if (num1 >= -1 & num1 != self.detailnr)
          {
            self.detailnr = num1;
            self.prepareTempValue4();
            flag1 = true;
          }
        }
        else if (self.subtab == 1 && num1 >= -1 & num1 != self.detailnr)
        {
          self.detailnr = num1;
          self.prepareTempValue4();
          flag1 = true;
        }
        let mut num2: i32 = -1;
        let mut num3: i32 = -1;
        bool flag2 = false;
        if (flag1)
        {
          num4: i32;
          num5: i32;
          if (self.subtab == 1)
          {
            let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
            num4 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.detailnr, 10)));
            num5 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.detailnr, 11)));
            SimpleList simpleList = SimpleList::new();
            let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut tdata1: i32 = 0; tdata1 <= mapWidth; tdata1 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
              {
                if (self.game.EditObj.TempValue4[0].Value[tdata1, tdata2] == self.detailnr && self.game.Data.MapObj[0].HexObj[tdata1, tdata2].MaxRecon > 0)
                {
                  let mut tweight: i32 = !(tdata1 == num4 & tdata2 == num5) ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location <= -1 ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].UnitCounter <= -1 ? 1 : 5) : 10) : 100;
                  simpleList.Add(tdata2 * self.game.Data.MapObj[0].MapWidth + tdata1, tweight, tdata1, tdata2);
                }
              }
            }
            simpleList.ReverseSortHighSpeed();
            if (simpleList.Counter > -1)
            {
              flag2 = true;
              num2 = simpleList.Data1[0];
              num3 = simpleList.Data2[0];
              self.game.EditObj.SetViewModeExtraNr = 1;
            }
          }
          if (self.subtab == 0 & self.detailnr > -1)
          {
            let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0))].GetData(0, self.game.Data.RegimeObj[self.detailnr].id, 12)));
            if (id > 0)
            {
              let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID(id);
              if (locationById > -1)
              {
                num4 = self.game.Data.LocObj[locationById].X;
                num5 = self.game.Data.LocObj[locationById].Y;
              }
            }
            SimpleList simpleList = SimpleList::new();
            let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut tdata1: i32 = 0; tdata1 <= mapWidth; tdata1 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
              {
                if (self.game.EditObj.TempValue4[0].Value[tdata1, tdata2] == self.game.Data.RegimeObj[self.detailnr].id && self.game.Data.MapObj[0].HexObj[tdata1, tdata2].MaxRecon > 0)
                {
                  let mut tweight: i32 = !(tdata1 == num4 & tdata2 == num5) ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location <= -1 ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].UnitCounter <= -1 ? 1 : 5) : 10) : 100;
                  simpleList.Add(tdata2 * self.game.Data.MapObj[0].MapWidth + tdata1, tweight, tdata1, tdata2);
                }
              }
            }
            simpleList.ReverseSortHighSpeed();
            if (simpleList.Counter > -1)
            {
              flag2 = true;
              num2 = simpleList.Data1[0];
              num3 = simpleList.Data2[0];
              self.game.EditObj.SetViewModeExtraNr = 2;
            }
          }
        }
        if (flag2 & num2 > -1 & num3 > -1)
        {
          self.game.SelectX = num2;
          self.game.SelectY = num3;
          self.game.EditObj.UnitSelected = -1;
          if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1)
            self.game.EditObj.UnitSelected = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
          self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
          let mut num6: i32 = self.game.EditObj.ShowHQ ? 1 : 0;
          windowReturnClass.AddCommand(4, 69);
          windowReturnClass.AddCommand(4, 68);
          flag1 = true;
        }
        if (flag1)
        {
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      return windowReturnClass;
    }

    pub fn dostuff()
    {
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 288, 0, 0));
      let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 275, 0, 0));
      let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
      let mut stringListById5: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
      let mut stringListById6: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData(0, 58, 2)));
      if (self.B1Id > 0)
      {
        self.RemoveSubPart(self.B1Id);
        self.B1Id = 0;
      }
      if (self.B2Id > 0)
      {
        self.RemoveSubPart(self.B2Id);
        self.B2Id = 0;
      }
      if (self.B2TextId > 0)
      {
        self.RemoveSubPart(self.B2TextId);
        self.B2TextId = 0;
      }
      if (self.b3Id > 0)
      {
        self.RemoveSubPart(self.b3Id);
        self.b3Id = 0;
      }
      if (self.b3textid > 0)
      {
        self.RemoveSubPart(self.b3textid);
        self.b3textid = 0;
      }
      if (self.b4Id > 0)
      {
        self.RemoveSubPart(self.b4Id);
        self.b4Id = 0;
      }
      if (self.b4textid > 0)
      {
        self.RemoveSubPart(self.b4textid);
        self.b4textid = 0;
      }
      if (self.info2id > 0)
      {
        self.RemoveSubPart(self.info2id);
        self.info2id = 0;
      }
      if (self.Info1Id > 0)
      {
        self.RemoveSubPart(self.Info1Id);
        self.Info1Id = 0;
      }
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      self.ClearMouse();
      Rectangle trect1 = DrawMod.DrawBackTab(objgraphics, self.w, self.h, "S.MAP", 6);
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F7]", 999);
      let mut num2: i32 = self.w - 680;
      bool tpaintview;
      if (self.game.EditObj.Zoom == 0)
      {
        if ( (self.game.Data.MapObj[0].MapHeight + self.game.Data.MapObj[0].MapWidth) >  self.game.ScreenWidth / 52.0 +  self.game.ScreenHeight / 48.0)
          tpaintview = true;
      }
      else if (self.game.EditObj.Zoom == -1)
      {
        if ( (self.game.Data.MapObj[0].MapHeight + self.game.Data.MapObj[0].MapWidth) >  self.game.ScreenWidth / 27.0 +  self.game.ScreenHeight / 24.0)
          tpaintview = true;
      }
      else if (self.game.EditObj.Zoom == 1 &&  (self.game.Data.MapObj[0].MapHeight + self.game.Data.MapObj[0].MapWidth) >  self.game.ScreenWidth / 104.0 +  self.game.ScreenHeight / 96.0)
        tpaintview = true;
      if (self.subtab == 1 & self.detailnr < 0)
        self.detailnr = 0;
      let mut ttempValue4mustBe: i32 = self.detailnr;
      if (self.subtab == 0 & self.detailnr > -1)
        ttempValue4mustBe = self.game.Data.RegimeObj[self.detailnr].id;
      bool tTempZones = false;
      if (self.subtab == 1)
        tTempZones = true;
      if (self.subtab > 0)
        self.game.EditObj.se1_StrategySpecial2 = 0;
      if (self.subtab == 2)
      {
        let mut tsubpart: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, tpaintview, self.w - 300, self.h - 40, true, true, self.game.ScreenWidth, self.game.ScreenHeight - 340, self.game.EditObj.Zoom, alsoHQ: self.game.EditObj.ShowHQ, tTempZones: tTempZones, tspecialMode1: self.detailnr);
        self.Info1Id = self.AddSubPart( tsubpart, 10, 0, self.w - 300, self.h - 40, 0);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, tpaintview, self.w - 300, self.h - 40, true, true, self.game.ScreenWidth, self.game.ScreenHeight - 340, self.game.EditObj.Zoom, alsoHQ: self.game.EditObj.ShowHQ, ttempValue4mustBe: ttempValue4mustBe, tTempValue3usedForAlpha: true, tTempAi2use: true, tTempZones: tTempZones);
        self.Info1Id = self.AddSubPart( tsubpart, 10, 0, self.w - 300, self.h - 40, 0);
      }
      self.OptionsListObj = ListClass::new();
      let mut tlistselect: i32 = -1;
      SizeF sizeF = SizeF::new();
      let mut x1: i32 = 410 + num2;
      let mut num3: i32 = self.h - 200;
      let mut num4: i32 =  Math.Round(Math.Floor( num3 / 24.0));
      let mut num5: i32 = 24;
      if (self.subtab == 0)
      {
        let mut num6: i32 = -1;
        self.OptionsListObj.add("  --None-- ", -1);
        if (self.game.Data.RegimeCounter > -1)
        {
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut tdata: i32 = 0; tdata <= regimeCounter; tdata += 1)
          {
            if (self.regimesToBeShown[tdata] == -1)
            {
              name: String = self.game.Data.RegimeObj[tdata].Name;
              let mut num7: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, self.game.Data.RegimeObj[tdata].id, 2, "recon", 3)));
              tname: String;
              if (tdata == 1)
                tname = "  --" + name + "-- ";
              else if (tdata == self.game.Data.Turn)
                tname = "⍟ " + name;
              else if (num7 >= 2)
              {
                switch ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.RegimeObj[tdata].id, 1))))
                {
                  case 1:
                    tname = "⌾ " + name;
                    break;
                  case 2:
                    let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.RegimeObj[tdata].id, 2)));
                    tname =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData(0, idValue, 1))) != num1 ? "∘ " + name : "∞ " + name;
                    break;
                  case 3:
                    tname = "⊷ " + name;
                    break;
                  default:
                    tname = "  " + name;
                    break;
                }
              }
              else
              {
                switch ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.RegimeObj[tdata].id, 1))))
                {
                  case 1:
                    tname = "⌾ " + name;
                    break;
                  case 2:
                    tname = "∘ " + name;
                    break;
                  case 3:
                    tname = "⊷ " + name;
                    break;
                  default:
                    tname = "  " + name;
                    break;
                }
              }
              self.OptionsListObj.add(tname, tdata);
            }
          }
          self.OptionsListObj.Sort();
          let mut listCount: i32 = self.OptionsListObj.ListCount;
          for (let mut index: i32 = 0; index <= listCount; index += 1)
          {
            num6 += 1;
            if (self.OptionsListObj.ListData[index] == self.detailnr)
              tlistselect = num6;
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            let mut tsubpart: SubPartClass =  new ListSubPartClass(self.OptionsListObj, num4 - 1, 250, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (410 + num2), bby: num5, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
            self.OptionsListId = self.AddSubPart( tsubpart, 410 + num2, num5, 250, num4 * 24, 0);
          }
        }
      }
      else if (self.subtab == 1)
      {
        let mut num8: i32 = -1;
        self.OptionsListObj.add(" --None-- ", 0);
        let mut length: i32 = self.game.Data.StringListObj[stringListById1].Length;
        for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
        {
          let mut index2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index1, 0]));
          let mut num9: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index1, 8]));
          tname: String = self.game.Data.StringListObj[stringListById1].Data[index1, 7];
          if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, index2, 2, "recon", 3))) > -1 | num9 == self.game.Data.RegimeObj[self.game.Data.Turn].id | self.zonesToBeShown[index2] == -1)
          {
            if (num9 == self.game.Data.RegimeObj[self.game.Data.Turn].id)
              tname += " *";
            self.OptionsListObj.add(tname, index2);
          }
        }
        self.OptionsListObj.Sort();
        let mut listCount: i32 = self.OptionsListObj.ListCount;
        for (let mut index: i32 = 0; index <= listCount; index += 1)
        {
          num8 += 1;
          if (self.OptionsListObj.ListData[index] == self.detailnr)
            tlistselect = num8;
        }
        if (self.OptionsListId > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
          self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        }
        else
        {
          let mut tsubpart: SubPartClass =  new ListSubPartClass(self.OptionsListObj, num4 - 1, 250, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (410 + num2), bby: num5, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
          self.OptionsListId = self.AddSubPart( tsubpart, 410 + num2, num5, 250, num4 * 24, 0);
        }
      }
      else if (self.subtab == 2)
      {
        let mut num10: i32 = 0;
        if (self.detailnr < 0)
          self.detailnr = 0;
        self.OptionsListObj.add("-- Nothing --", 0);
        if (self.detailnr > 12)
          self.detailnr = 0;
        if (0 == self.detailnr)
          tlistselect = num10;
        let mut num11: i32 = num10 + 1;
        self.OptionsListObj.add("All Resources", 6);
        if (6 == self.detailnr)
          tlistselect = num11;
        let mut num12: i32 = 1;
        do
        {
          num11 += 1;
          if (num12 == 1)
            self.OptionsListObj.add("Oil", 1);
          if (num12 == 2)
            self.OptionsListObj.add("Metals", 2);
          if (num12 == 3)
            self.OptionsListObj.add("Rare Metals", 3);
          if (num12 == 4)
            self.OptionsListObj.add("Radioactives", 4);
          if (num12 == 5)
            self.OptionsListObj.add("Water", 5);
          if (num12 == 6)
            self.OptionsListObj.add("Rain", 7);
          if (num12 == 7)
            self.OptionsListObj.add("Temperature", 8);
          if (num12 == 8)
            self.OptionsListObj.add("Radiation", 9);
          if (num12 == 9)
            self.OptionsListObj.add("Height", 10);
          if (num12 == 10)
            self.OptionsListObj.add("Tectonic Plates", 11);
          if (num12 == 11)
            self.OptionsListObj.add("Scavenge Points", 12);
          if (self.OptionsListObj.ListData[self.OptionsListObj.ListCount] == self.detailnr)
            tlistselect = num11;
          num12 += 1;
        }
        while (num12 <= 11);
        if (self.OptionsListId > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
          self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        }
        else
        {
          let mut tsubpart: SubPartClass =  new ListSubPartClass(self.OptionsListObj, num4 - 1, 250, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (410 + num2), bby: num5, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
          self.OptionsListId = self.AddSubPart( tsubpart, 410 + num2, num5, 250, num4 * 24, 0);
        }
      }
      Rectangle rectangle1;
      Rectangle trect2;
      bitmap1: Bitmap;
      if (self.subtab == 2)
      {
         let mut local1: &Graphics = &objgraphics;
        bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap2;
        Rectangle rectangle2 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect1: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1 + 0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect1: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &objgraphics;
        bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap3;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect2: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(x1 + 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect2: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
        str1: String = "REGIMES";
        sizeF = objgraphics.MeasureString(str1, self.game.MarcFont16);
        let mut x2: i32 = x1 + 20;
        let mut y1: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str1, self.game.MarcFont16, x2, y1 + 4, Color.White);
        rectangle1 = Rectangle::new(x1, y1, 80, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a regime", 101);
         let mut local5: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local6: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect3: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 70, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect3: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
         let mut local7: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local8: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect4: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 122, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect4: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
        str2: String = "ZONES";
        sizeF = objgraphics.MeasureString(str2, self.game.MarcFont16);
        let mut x3: i32 = x1 + 20 + 70;
        let mut y2: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str2, self.game.MarcFont16, x3, y2 + 4, Color.White);
        rectangle1 = Rectangle::new(x1 + 70, y2, 182, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a Zone", 102);
         let mut local9: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local10: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect5: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 140, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect5: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
         let mut local11: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local12: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect6: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 122 + 70, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect6: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
        str3: String = "STATS";
        sizeF = objgraphics.MeasureString(str3, self.game.MarcFont16);
        let mut x4: i32 = x1 + 20 + 140;
        let mut y3: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str3, self.game.MarcFont16, x4, y3 + 4, Color.White);
        rectangle1 = Rectangle::new(x1 + 140, y3, 80, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a specific Stat to inspect", 103);
      }
      if (self.subtab == 1)
      {
         let mut local13: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local14: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect7: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect7: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local13,  local14, srcrect7, destrect7);
         let mut local15: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local16: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect8: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect8: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local15,  local16, srcrect8, destrect8);
        str4: String = "REGIMES";
        sizeF = objgraphics.MeasureString(str4, self.game.MarcFont16);
        let mut x5: i32 = x1 + 20;
        let mut y4: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str4, self.game.MarcFont16, x5, y4 + 4, Color.White);
        rectangle1 = Rectangle::new(x1, y4, 80, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a regime", 101);
         let mut local17: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local18: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect9: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 140, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect9: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local17,  local18, srcrect9, destrect9);
         let mut local19: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local20: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect10: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 122 + 70, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect10: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local19,  local20, srcrect10, destrect10);
        str5: String = "STATS";
        sizeF = objgraphics.MeasureString(str5, self.game.MarcFont16);
        let mut x6: i32 = x1 + 20 + 140;
        let mut y5: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str5, self.game.MarcFont16, x6, y5 + 4, Color.White);
        rectangle1 = Rectangle::new(x1 + 140, y5, 80, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a specific Stat to inspect", 103);
         let mut local21: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local22: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect11: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 70, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect11: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local21,  local22, srcrect11, destrect11);
         let mut local23: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local24: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect12: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 122, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect12: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local23,  local24, srcrect12, destrect12);
        str6: String = "ZONES";
        sizeF = objgraphics.MeasureString(str6, self.game.MarcFont16);
        let mut x7: i32 = x1 + 20 + 70;
        let mut y6: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str6, self.game.MarcFont16, x7, y6 + 4, Color.White);
        rectangle1 = Rectangle::new(x1 + 70, y6, 182, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a Zone", 102);
      }
      if (self.subtab == 0)
      {
         let mut local25: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local26: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect13: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 140, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect13: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local25,  local26, srcrect13, destrect13);
         let mut local27: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local28: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect14: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 122 + 70, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect14: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local27,  local28, srcrect14, destrect14);
        str7: String = "STATS";
        sizeF = objgraphics.MeasureString(str7, self.game.MarcFont16);
        let mut x8: i32 = x1 + 20 + 140;
        let mut y7: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str7, self.game.MarcFont16, x8, y7 + 4, Color.White);
        rectangle1 = Rectangle::new(x1 + 140, y7, 80, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a specific Stat to inspect", 103);
         let mut local29: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local30: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect15: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 70, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect15: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local29,  local30, srcrect15, destrect15);
         let mut local31: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local32: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect16: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 122, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect16: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local31,  local32, srcrect16, destrect16);
        str8: String = "ZONES";
        sizeF = objgraphics.MeasureString(str8, self.game.MarcFont16);
        let mut x9: i32 = x1 + 20 + 70;
        let mut y8: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str8, self.game.MarcFont16, x9, y8 + 4, Color.White);
        rectangle1 = Rectangle::new(x1 + 70, y8, 182, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a Zone", 102);
         let mut local33: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local34: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect17: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 0, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect17: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local33,  local34, srcrect17, destrect17);
         let mut local35: &Graphics = &objgraphics;
        bitmap1 = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local36: &Bitmap = &bitmap1;
        rectangle1 = Rectangle::new(BitmapStore.GetWidth(self.game.MARCLARGETAB) - 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut srcrect18: &Rectangle = &rectangle1
        trect2 = Rectangle::new(x1 + 52, 0, 52, BitmapStore.Getheight(self.game.MARCLARGETAB));
        let mut destrect18: &Rectangle = &trect2
        DrawMod.DrawSimplePart2( local35,  local36, srcrect18, destrect18);
        str9: String = "REGIMES";
        sizeF = objgraphics.MeasureString(str9, self.game.MarcFont16);
        let mut x10: i32 = x1 + 20;
        let mut y9: i32 = 0;
        DrawMod.DrawTextColouredMarc( objgraphics, str9, self.game.MarcFont16, x10, y9 + 4, Color.White);
        rectangle1 = Rectangle::new(x1, y9, 80, 24);
        trect2 = rectangle1;
        self.AddMouse( trect2, "", "Click to select a Regime", 101);
      }
      let mut num13: i32 = num3 + 32;
      if (self.detailnr > -1 & self.subtab == 0)
      {
        if (self.detailnr != self.regnr)
        {
          if (!self.game.Data.RegimeObj[self.detailnr].AI)
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("MESSAGE", 240, "Send this regime a text message",  self.OwnBitmap, 410 + num2, num13, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.B1Id = self.AddSubPart( tsubpart, 410 + num2, num13, 240, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("MESSAGE", 240, "Cant send message to an AI.\r\nIt will not understand.",  self.OwnBitmap, 410 + num2, num13, true, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.info2id = self.AddSubPart( tsubpart, 410 + num2, num13, 240, 35, 1);
          }
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("MESSAGE", 240, "Cant send message to self.",  self.OwnBitmap, 410 + num2, 290, true, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.info2id = self.AddSubPart( tsubpart, 410 + num2, num13, 240, 35, 1);
        }
        num13 += 48;
      }
      else if (self.subtab == 0)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("MESSAGE", 240, "Select the regime you want to send message too first.",  self.OwnBitmap, 410 + num2, num13, true, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.info2id = self.AddSubPart( tsubpart, 410 + num2, num13, 240, 35, 1);
        num13 += 48;
      }
      let mut num14: i32 = 410 + num2;
      let mut tsubpart1: SubPartClass =  new MarcRadioPartClass(0, self.game.EditObj.ShowHQ, "Show HQs on map",  self.OwnBitmap, num14, 290);
      self.B2Id = self.AddSubPart( tsubpart1, num14, num13, 35, 35, 1);
      DrawMod.DrawTextColouredMarc( objgraphics, "SHOW HQ", self.game.MarcFont4, num14 + 40, num13 + 6, Color.White);
      let mut num15: i32 = num14 + 122;
      SubPartClass tsubpart2;
      if (self.subtab < 2)
      {
        tsubpart2 =  new TextButtonPartClass("GO THERE", 110, "Focus map on selection.",  self.OwnBitmap, num15, num13, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.b3Id = self.AddSubPart( tsubpart2, num15, num13, 110, 35, 1);
      }
      if (self.subtab < 1)
      {
        let mut num16: i32 = 410 + num2;
        let mut y: i32 = num13 + 40;
        tsubpart2 =  new MarcRadioPartClass(0, self.game.EditObj.se1_StrategySpecial2 == 1, "Diplomatic Colors will be shown.\r\nGreen for peace.\r\nRed for war\r\nOrange to Yellow-Green for unclear.",  self.OwnBitmap, num16, 290);
        self.b4Id = self.AddSubPart( tsubpart2, num16, y, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( objgraphics, "DIP.COLORS", self.game.MarcFont4, num16 + 40, y + 6, Color.White);
      }
      self.game.EditObj.se1_StrategyTab = self.subtab;
    }

    pub handleTimerWheel: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass.Flag = false;
      if (self.SubPartCounter > -1)
      {
        for (let mut subPartCounter: i32 = self.SubPartCounter; subPartCounter >= 0; subPartCounter += -1)
        {
          if (x > self.SubPartX[subPartCounter] & y > self.SubPartY[subPartCounter] & x < self.SubPartX[subPartCounter] + self.SubPartW[subPartCounter] & y < self.SubPartY[subPartCounter] + self.SubPartH[subPartCounter])
          {
            let mut subPart: SubPartClass = self.SubPartList[subPartCounter];
            let mut x1: i32 = x - self.SubPartX[subPartCounter];
            let mut y1: i32 = y - self.SubPartY[subPartCounter];
            WindowClass windowClass = (WindowClass) this;
             WindowClass local =  windowClass;
            if (subPart.HandleTimerWheel(x1, y1,  local))
            {
              windowReturnClass.SetFlag(true);
              self.SubPartFlag[subPartCounter] = true;
              if (self.Info1Id == self.SubPartID[subPartCounter])
              {
                self.game.EditObj.MiniMap = new Bitmap(10, 10);
                self.dostuff();
              }
              return windowReturnClass;
            }
          }
        }
      }
      windowReturnClass.SetFlag(false);
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
      if (self.SubPartCounter <= -1)
        return;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = "";
          self.game.EditObj.TipText = self.SubPartList[index].Descript;
          break;
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          switch (self.MouseData[index])
          {
            case 101:
              self.detailnr = -1;
              self.subtab = 0;
              self.RemoveSubPart(self.OptionsListId);
              self.OptionsListId = 0;
              self.prepareTempValue4();
              self.game.EditObj.MiniMap = new Bitmap(10, 10);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 102:
              self.subtab = 1;
              self.detailnr = 0;
              self.RemoveSubPart(self.OptionsListId);
              self.OptionsListId = 0;
              self.prepareTempValue4();
              self.game.EditObj.MiniMap = new Bitmap(10, 10);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 103:
              self.subtab = 2;
              self.detailnr = self.game.EditObj.se1_StrategySpecial1;
              self.RemoveSubPart(self.OptionsListId);
              self.OptionsListId = 0;
              self.game.HandyFunctionsObj.RedimTempValue4(-1);
              self.game.HandyFunctionsObj.RedimTempValue3(-1);
              self.game.EditObj.MiniMap = new Bitmap(10, 10);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 999:
              self.game.EditObj.SetViewMode2 = 0;
              windowReturnClass.AddCommand(1, 9);
              windowReturnClass.AddCommand(7, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            default:
              continue;
          }
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
      {
        if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
        {
          let mut num1: i32 = self.SubPartID[index1];
          if (num1 == self.B1Id)
          {
            Form2::new( self.formref).Initialize(self.game.Data, 7, self.detailnr);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.B2Id)
          {
            self.game.EditObj.ShowHQ = !self.game.EditObj.ShowHQ;
            self.game.EditObj.MiniMap = new Bitmap(10, 10);
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.b4Id)
          {
            if (self.game.EditObj.se1_StrategySpecial2 == 0)
              self.game.EditObj.se1_StrategySpecial2 = 1;
            else
              self.game.EditObj.se1_StrategySpecial2 = 0;
            self.game.EditObj.MiniMap = new Bitmap(10, 10);
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.b3Id)
          {
            self.game.EditObj.UnitSelected = -1;
            if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1)
              self.game.EditObj.UnitSelected = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            self.game.HandyFunctionsObj.SetcornerXY2();
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 69);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.OptionsListId)
          {
            let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
            self.SubPartFlag[index1] = true;
            if (num2 > -1)
            {
              bool flag1 = false;
              if (self.subtab == 0)
              {
                if (num2 >= -1 & num2 != self.detailnr)
                {
                  self.detailnr = num2;
                  self.prepareTempValue4();
                  flag1 = true;
                }
              }
              else if (self.subtab == 1)
              {
                if (num2 >= -1 & num2 != self.detailnr)
                {
                  self.detailnr = num2;
                  self.prepareTempValue4();
                  flag1 = true;
                }
              }
              else if (self.subtab == 2)
              {
                self.detailnr = num2;
                self.game.EditObj.se1_StrategySpecial1 = self.detailnr;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num3: i32 = -1;
              let mut num4: i32 = -1;
              bool flag2 = false;
              if (flag1)
              {
                num5: i32;
                num6: i32;
                if (self.subtab == 1)
                {
                  let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
                  num5 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.detailnr, 10)));
                  num6 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.detailnr, 11)));
                  SimpleList simpleList = SimpleList::new();
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut tdata1: i32 = 0; tdata1 <= mapWidth; tdata1 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
                    {
                      if (self.game.EditObj.TempValue4[0].Value[tdata1, tdata2] == self.detailnr && self.game.Data.MapObj[0].HexObj[tdata1, tdata2].MaxRecon > 0 | !self.game.Data.ShrowdOn)
                      {
                        let mut tweight: i32 = !(tdata1 == num5 & tdata2 == num6) ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location <= -1 ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].UnitCounter <= -1 ? 1 : 5) : 10) : 100;
                        simpleList.Add(tdata2 * self.game.Data.MapObj[0].MapWidth + tdata1, tweight, tdata1, tdata2);
                      }
                    }
                  }
                  simpleList.ReverseSortHighSpeed();
                  if (simpleList.Counter > -1)
                  {
                    flag2 = true;
                    num3 = simpleList.Data1[0];
                    num4 = simpleList.Data2[0];
                    self.game.EditObj.SetViewModeExtraNr = 1;
                  }
                }
                if (self.subtab == 0 & self.detailnr > -1)
                {
                  let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0))].GetData(0, self.game.Data.RegimeObj[self.detailnr].id, 12)));
                  if (id > 0)
                  {
                    let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID(id);
                    if (locationById > -1)
                    {
                      num5 = self.game.Data.LocObj[locationById].X;
                      num6 = self.game.Data.LocObj[locationById].Y;
                    }
                  }
                  SimpleList simpleList = SimpleList::new();
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut tdata1: i32 = 0; tdata1 <= mapWidth; tdata1 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
                    {
                      if (self.game.EditObj.TempValue4[0].Value[tdata1, tdata2] == self.game.Data.RegimeObj[self.detailnr].id && self.game.Data.MapObj[0].HexObj[tdata1, tdata2].MaxRecon > 0 | !self.game.Data.ShrowdOn)
                      {
                        let mut tweight: i32 = !(tdata1 == num5 & tdata2 == num6) ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location <= -1 ? (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].UnitCounter <= -1 ? 1 : 5) : 10) : 100;
                        simpleList.Add(tdata2 * self.game.Data.MapObj[0].MapWidth + tdata1, tweight, tdata1, tdata2);
                      }
                    }
                  }
                  simpleList.ReverseSortHighSpeed();
                  if (simpleList.Counter > -1)
                  {
                    flag2 = true;
                    num3 = simpleList.Data1[0];
                    num4 = simpleList.Data2[0];
                    self.game.EditObj.SetViewModeExtraNr = 2;
                  }
                }
              }
              if (flag2 & num3 > -1 & num4 > -1)
              {
                self.game.SelectX = num3;
                self.game.SelectY = num4;
                self.game.EditObj.UnitSelected = -1;
                if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1)
                  self.game.EditObj.UnitSelected = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
                self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                let mut num7: i32 = self.game.EditObj.ShowHQ ? 1 : 0;
                windowReturnClass.AddCommand(4, 69);
                windowReturnClass.AddCommand(4, 68);
                flag1 = true;
              }
              if (flag1)
              {
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          else if (num1 == self.Info1Id)
          {
            self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1], 3);
            Coordinate hexCoord = ((MiniMapPartClass) self.SubPartList[index1]).GetHexCoord(x - self.SubPartX[index1], y - self.SubPartY[index1]);
            if (self.subtab == 2 && hexCoord.onmap)
            {
              let mut selectX: i32 = self.game.SelectX;
              let mut selectY: i32 = self.game.SelectY;
              self.game.SelectX = hexCoord.x;
              self.game.SelectY = hexCoord.y;
              self.game.HandyFunctionsObj.SetcornerXY2();
              self.game.SelectX = selectX;
              self.game.SelectY = selectY;
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (self.game.EditObj.CurrentMiniX > -1 & self.game.EditObj.CurrentMiniY > -1 & self.game.EditObj.MapSelected > -1 && self.game.EditObj.CurrentMiniX <= self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth & self.game.EditObj.CurrentMiniY <= self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight && self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.EditObj.CurrentMiniX, self.game.EditObj.CurrentMiniY].Regime > -1 && !self.game.Data.RegimeObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.EditObj.CurrentMiniX, self.game.EditObj.CurrentMiniY].Regime].Sleep)
            {
              self.detailnr = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.EditObj.CurrentMiniX, self.game.EditObj.CurrentMiniY].Regime;
              if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.EditObj.CurrentMiniX, self.game.EditObj.CurrentMiniY].get_SeeNow(self.game.Data.Turn) < 1)
                self.detailnr = -1;
            }
            bool flag3 = false;
            if (hexCoord.onmap)
            {
              if (self.subtab == 0 & self.game.EditObj.TempValue4[0].Value[hexCoord.x, hexCoord.y] > 0)
              {
                let mut regimeById: i32 = self.game.HandyFunctionsObj.GetRegimeByID(self.game.EditObj.TempValue4[0].Value[hexCoord.x, hexCoord.y]);
                if (regimeById > -1)
                {
                  self.detailnr = regimeById;
                  self.prepareTempValue4();
                  flag3 = true;
                }
              }
              else if (self.subtab == 0 & self.game.EditObj.TempValue4[0].Value[hexCoord.x, hexCoord.y] < 1)
              {
                self.regnr = -1;
                self.detailnr = self.regnr;
                self.prepareTempValue4();
                flag3 = true;
              }
              else if (self.subtab == 1)
              {
                self.detailnr = self.game.EditObj.TempValue4[0].Value[hexCoord.x, hexCoord.y];
                self.prepareTempValue4();
                flag3 = true;
              }
            }
            let mut num8: i32 = -1;
            let mut num9: i32 = -1;
            bool flag4 = false;
            if (flag3)
            {
              if (self.game.Data.MapObj[0].HexObj[hexCoord.x, hexCoord.y].MaxRecon > 0 | !self.game.Data.ShrowdOn)
              {
                if (self.subtab == 1)
                {
                  flag4 = true;
                  num8 = hexCoord.x;
                  num9 = hexCoord.y;
                  self.game.EditObj.SetViewModeExtraNr = 1;
                }
                else if (self.subtab == 0)
                {
                  flag4 = true;
                  num8 = hexCoord.x;
                  num9 = hexCoord.y;
                  self.game.EditObj.SetViewModeExtraNr = 2;
                }
              }
              else
              {
                num10: i32;
                num11: i32;
                if (self.subtab == 1)
                {
                  let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
                  num10 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.detailnr, 10)));
                  num11 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.detailnr, 11)));
                  SimpleList simpleList = SimpleList::new();
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                    {
                      if (self.game.EditObj.TempValue4[0].Value[index2, index3] == self.detailnr && self.game.Data.MapObj[0].HexObj[index2, index3].MaxRecon > 0 | !self.game.Data.ShrowdOn)
                      {
                        let mut tweight: i32 = self.game.HandyFunctionsObj.Distance(hexCoord.x, hexCoord.y, 0, index2, index3, 0, 19);
                        simpleList.Add(index3 * self.game.Data.MapObj[0].MapWidth + index2, tweight, index2, index3);
                      }
                    }
                  }
                  simpleList.SortHighSpeed();
                  if (simpleList.Counter > -1)
                  {
                    flag4 = true;
                    num8 = simpleList.Data1[0];
                    num9 = simpleList.Data2[0];
                    self.game.EditObj.SetViewModeExtraNr = 1;
                  }
                }
                if (self.subtab == 0 & self.detailnr > -1)
                {
                  let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0))].GetData(0, self.game.Data.RegimeObj[self.detailnr].id, 12)));
                  if (id > 0)
                  {
                    let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID(id);
                    if (locationById > -1)
                    {
                      num10 = self.game.Data.LocObj[locationById].X;
                      num11 = self.game.Data.LocObj[locationById].Y;
                    }
                  }
                  SimpleList simpleList = SimpleList::new();
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut index4: i32 = 0; index4 <= mapWidth; index4 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut index5: i32 = 0; index5 <= mapHeight; index5 += 1)
                    {
                      if (self.game.EditObj.TempValue4[0].Value[index4, index5] == self.game.Data.RegimeObj[self.detailnr].id && self.game.Data.MapObj[0].HexObj[index4, index5].MaxRecon > 0 | !self.game.Data.ShrowdOn)
                      {
                        let mut tweight: i32 = self.game.HandyFunctionsObj.Distance(hexCoord.x, hexCoord.y, 0, index4, index5, 0, 19);
                        simpleList.Add(index5 * self.game.Data.MapObj[0].MapWidth + index4, tweight, index4, index5);
                      }
                    }
                  }
                  simpleList.SortHighSpeed();
                  if (simpleList.Counter > -1)
                  {
                    flag4 = true;
                    num8 = simpleList.Data1[0];
                    num9 = simpleList.Data2[0];
                    self.game.EditObj.SetViewModeExtraNr = 2;
                  }
                }
              }
            }
            if (flag4 & num8 > -1 & num9 > -1)
            {
              self.game.SelectX = num8;
              self.game.SelectY = num9;
              self.game.EditObj.UnitSelected = -1;
              if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1)
                self.game.EditObj.UnitSelected = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
              self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
              let mut num12: i32 = self.game.EditObj.ShowHQ | flag3 ? 1 : 0;
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.AddCommand(4, 68);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 9);
            let mut num13: i32 = self.game.EditObj.ShowHQ ? 1 : 0;
            self.dostuff();
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

    pub fn prepareTempValue4()
    {
      self.regimesToBeShown = new int[self.game.Data.RegimeCounter + 1];
      self.game.HandyFunctionsObj.RedimTempValue4(-1);
      self.game.HandyFunctionsObj.RedimTempValue3(-1);
      self.game.EditObj.TempAI = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.game.EditObj.TempAI2 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      data: DataClass = DrawMod.TGame.Data;
      str: String = "Zones";
       local: String =  str;
      let mut libVar: i32 = data.FindLibVar( local, "SE_Data");
      self.zonesToBeShown = new int[self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0))].GetHighestValue(0) + 1 + 1];
      AIMatrix specialMask = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
      bool[] flagArray = new bool[self.game.Data.RegimeCounter + 1];
      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (index1 == 61 & index2 == 14)
            index1 = index1;
          if (self.game.Data.MapObj[0].HexObj[index1, index2].Regime > -1)
            flagArray[self.game.Data.MapObj[0].HexObj[index1, index2].Regime] = true;
          if (self.subtab == 0)
          {
            let mut regime: i32 = self.game.Data.MapObj[0].HexObj[index1, index2].Regime;
            if (regime > -1)
              self.game.EditObj.TempValue4[0].Value[index1, index2] = self.game.Data.RegimeObj[regime].id;
          }
          else if (self.subtab == 1)
          {
            let mut hexLibVarValue: i32 = DrawMod.TGame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar);
            if (hexLibVarValue > 0 & (self.game.Data.MapObj[0].HexObj[index1, index2].MaxRecon > 0 | !self.game.Data.ShrowdOn))
            {
              self.game.EditObj.TempValue4[0].Value[index1, index2] = hexLibVarValue;
              self.zonesToBeShown[hexLibVarValue] = -1;
            }
            specialMask.Value[index1, index2] = 0;
            if ((self.game.Data.MapObj[0].HexObj[index1, index2].MaxRecon > 0 | !self.game.Data.ShrowdOn) & self.game.Data.MapObj[0].HexObj[index1, index2].Regime > -1)
              specialMask.Value[index1, index2] = self.game.Data.MapObj[0].HexObj[index1, index2].Regime + 2;
            else if (self.game.Data.MapObj[0].HexObj[index1, index2].get_LastReg(self.game.Data.Turn) > 0)
              specialMask.Value[index1, index2] = self.game.Data.MapObj[0].HexObj[index1, index2].get_LastReg(self.game.Data.Turn) + 2;
            else if (self.game.Data.MapObj[0].HexObj[index1, index2].get_LastReg(self.game.Data.Turn) == -1 & self.game.Data.MapObj[0].HexObj[index1, index2].get_LastLT(self.game.Data.Turn) > -1)
              specialMask.Value[index1, index2] = 0;
          }
        }
      }
      AIMatrix aiMatrix1 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
      AIMatrix aiMatrix2 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
      AIMatrix aiMatrix3 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
      AIMatrix aiMatrix4 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
      let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index3: i32 = 0; index3 <= mapWidth2; index3 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
        {
          self.game.EditObj.TempAI[index3, index4] = 0;
          aiMatrix4.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].get_LastLT(self.game.Data.Turn);
          if (aiMatrix4.Value[index3, index4] < 0)
            aiMatrix4.Value[index3, index4] = 0;
          if (index3 == 38 & index4 == 21)
            index3 = index3;
          if (self.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0 | !self.game.Data.ShrowdOn)
          {
            aiMatrix1.Value[index3, index4] =  byte.MaxValue;
            aiMatrix2.Value[index3, index4] = self.game.EditObj.TempValue4[0].Value[index3, index4];
            if (aiMatrix2.Value[index3, index4] <= 0)
              aiMatrix2.Value[index3, index4] = 999999;
            aiMatrix3.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].Regime + 2;
            aiMatrix4.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].LandscapeType + 1;
            if (self.game.Data.MapObj[0].HexObj[index3, index4].Regime > -1)
              self.regimesToBeShown[self.game.Data.MapObj[0].HexObj[index3, index4].Regime] = -1;
          }
          else if (self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn) > 0)
          {
            if (flagArray[self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn)])
              aiMatrix3.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn) + 2;
            aiMatrix4.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].get_LastLT(self.game.Data.Turn) + 1;
            aiMatrix1.Value[index3, index4] =  byte.MaxValue;
            aiMatrix2.Value[index3, index4] = self.game.EditObj.TempValue4[0].Value[index3, index4];
            if (aiMatrix2.Value[index3, index4] <= 0 & self.subtab == 0)
              aiMatrix2.Value[index3, index4] = 999999;
            if (self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn) > -1 && self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn) <= self.game.Data.RegimeCounter && flagArray[self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn)])
              self.regimesToBeShown[self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn)] = -1;
          }
        }
      }
      if (self.subtab == 1)
        specialMask.ExpandAllNonZeroValuesForAnyRegime(15);
      aiMatrix1.ExpandAndRemovePercentageForAnyRegime( byte.MaxValue, 0.7f, true);
      if (self.subtab == 1)
        aiMatrix2.ExpandAllNonZeroValuesForAnyRegime(15, specialMask);
      else
        aiMatrix2.ExpandAllNonZeroValuesForAnyRegime(15);
      aiMatrix3.ExpandAllNonZeroValuesForAnyRegime(15);
      aiMatrix4.ExpandAllNonZeroValuesForAnyRegime(15);
      int[] numArray1 = new int[self.game.Data.RegimeCounter + 2 + 1];
      let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index5: i32 = 0; index5 <= mapWidth3; index5 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index5, index6].Regime > -1)
          {
            int[] numArray2 = numArray1;
            int[] numArray3 = numArray2;
            let mut regime: i32 = self.game.Data.MapObj[0].HexObj[index5, index6].Regime;
            let mut index7: i32 = regime;
            let mut num: i32 = numArray2[regime] + 1;
            numArray3[index7] = num;
          }
          if (self.game.Data.MapObj[0].HexObj[index5, index6].MaxRecon > 0 | !self.game.Data.ShrowdOn)
          {
            aiMatrix1.Value[index5, index6] =  byte.MaxValue;
            aiMatrix2.Value[index5, index6] = self.game.EditObj.TempValue4[0].Value[index5, index6];
            aiMatrix3.Value[index5, index6] = self.game.Data.MapObj[0].HexObj[index5, index6].Regime + 2;
            aiMatrix4.Value[index5, index6] = self.game.Data.MapObj[0].HexObj[index5, index6].LandscapeType + 1;
          }
          else if (self.game.Data.MapObj[0].HexObj[index5, index6].get_LastReg(self.game.Data.Turn) > 0)
          {
            aiMatrix1.Value[index5, index6] =  byte.MaxValue;
            if (self.subtab == 0)
              aiMatrix2.Value[index5, index6] = self.game.EditObj.TempValue4[0].Value[index5, index6];
            aiMatrix3.Value[index5, index6] = self.game.Data.MapObj[0].HexObj[index5, index6].get_LastReg(self.game.Data.Turn) + 2;
            aiMatrix4.Value[index5, index6] = self.game.Data.MapObj[0].HexObj[index5, index6].get_LastLT(self.game.Data.Turn) + 1;
          }
          else if (aiMatrix1.Value[index5, index6] < 20)
          {
            aiMatrix1.Value[index5, index6] = 0;
            aiMatrix2.Value[index5, index6] = 0;
            aiMatrix3.Value[index5, index6] = 0;
            aiMatrix4.Value[index5, index6] = 0;
          }
          if (index5 == 47 & index6 == 20)
            index5 = index5;
          if (aiMatrix2.Value[index5, index6] == 999999)
            aiMatrix2.Value[index5, index6] = 0;
          if (self.subtab == 0)
          {
            self.game.EditObj.TempValue4[0].Value[index5, index6] = 0;
            if (aiMatrix3.Value[index5, index6] > 1)
              self.game.EditObj.TempValue4[0].Value[index5, index6] = self.game.Data.RegimeObj[aiMatrix3.Value[index5, index6] - 2].id;
          }
          else
            self.game.EditObj.TempValue4[0].Value[index5, index6] = aiMatrix2.Value[index5, index6];
          self.game.EditObj.TempValue3[0].Value[index5, index6] = aiMatrix1.Value[index5, index6];
          self.game.EditObj.TempAI[index5, index6] = aiMatrix3.Value[index5, index6] - 2;
          self.game.EditObj.TempAI2[index5, index6] = aiMatrix4.Value[index5, index6] - 1;
        }
      }
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
      {
        if (numArray1[index] < 1)
          self.regimesToBeShown[index] = 0;
      }
    }
  }
}
