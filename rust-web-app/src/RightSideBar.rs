// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RightSideBar
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class RightSideBar : WindowClass
  {
     hideId: i32;
     Info1Id: i32;
     info2id: i32;
     info3id: i32;
     but1id: i32;
     but2id: i32;
     but3id: i32;
     but1: i32;
     but2: i32;
     but3: i32;
     but4: i32;
     but5: i32;
     but6: i32;
     but7: i32;
     but8: i32;
     but9: i32;
     but10: i32;
     but11: i32;
     w: i32;
     h: i32;
     currentShqNr: i32;
     profId: i32;
     cacheList: String;
     MouseOverWhichTab: i32;
     orderReplId: i32;
     orderRepl2Id: i32;
     orderAmmoId: i32;
     orderTradeId: i32;
     orderChangeShqId: i32;
     orderProduceUnitId: i32;
     orderColonizeId: i32;
     orderNewAssetId: i32;
     orderNewZoneId: i32;
     orderMoreId: i32;
     orderChangeId: i32;
     orderTransferId: i32;
     orderBattlegroupId2: i32;
     orderBattlegroupId: i32;
     orderUnitNr: i32;
     orderShqNr: i32;
     orderZoneNr: i32;
     ListClass rlistobj;
     rlistid: i32;
     layerLog1: i32;
     layerLog2: i32;
     layerLog3: i32;
     layerLog4: i32;
     layerLog5: i32;
     layerUnit: i32;
     layerLabel: i32;
     layerDetail: i32;
     layerGrid: i32;
     layerColor: i32;
     layerLisRange: i32;
     bool initialOpeningPreviewSet;

    pub RightSideBar(
       tGame: GameClass,
      theight: i32,
       WindowClass tLowerWindow,
       Rectangle tLowerRect)
      : base( tGame, 185, theight, 8)
    {
      self.initialOpeningPreviewSet = false;
      self.NewGfx = true;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.w = 185;
      self.h = theight;
      self.BlockBlit = true;
      self.currentShqNr = -1;
      self.dostuff();
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = base.HandleMouseMove(x, y);
      let mut num: i32 = -1;
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
          num = self.MouseData[mouseCounter];
      }
      if (num > 0)
      {
        if (self.MouseOverWhichTab != num)
        {
          if (self.game.EmpireStyle)
            SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
          self.MouseOverWhichTab = num;
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      else if (self.MouseOverWhichTab > 0)
      {
        self.MouseOverWhichTab = -1;
        self.dostuff();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      return windowReturnClass;
    }

    pub fn DoRefresh() => self.dostuff();

    pub Rectangle DrawOneTab(
      Graphics g,
      ty: i32,
      bool active,
      bool openSideWindow,
      iconSlot: i32,
      bool mouseOverRightNow = false)
    {
      let mut x1: i32 = 132;
      if (openSideWindow)
        x1 = -5;
      bitmap: Bitmap;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (mouseOverRightNow)
      {
        if (!active & openSideWindow)
        {
           let mut local1: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TABRIGHT);
           let mut local2: &Bitmap = &bitmap;
          rectangle1 = Rectangle::new(0, 0, 28, 70);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(x1 + 7, ty, 28, 70);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2ColouredNew( local1,  local2, srcrect, destrect, 1.1f, 1.1f, 1.1f, 1f);
        }
        if (active | !openSideWindow)
        {
           let mut local3: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TABRIGHT);
           let mut local4: &Bitmap = &bitmap;
          let mut x2: i32 = x1;
          let mut y: i32 = ty;
          DrawMod.Draw( local3,  local4, x2, y, 0.05f, 0.05f, 0.05f, 1f);
        }
      }
      else
      {
        if (!active & openSideWindow)
        {
           let mut local5: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TABRIGHT);
           let mut local6: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 0, 28, 70);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x1 + 7, ty, 28, 70);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
        }
        if (active | !openSideWindow)
        {
           let mut local7: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TABRIGHT);
           let mut local8: &Bitmap = &bitmap;
          let mut x3: i32 = x1;
          let mut y: i32 = ty;
          DrawMod.DrawSimple( local7,  local8, x3, y);
        }
      }
      if (iconSlot > -1)
      {
        if (mouseOverRightNow)
        {
          if (!active & openSideWindow)
          {
             let mut local9: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local10: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1 + 4, ty + 17, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
          }
          if (!active & !openSideWindow)
          {
             let mut local11: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local12: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1, ty + 17, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
          }
        }
        else
        {
          if (!active & openSideWindow)
          {
             let mut local13: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local14: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 0, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1 + 4, ty + 17, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local13,  local14, srcrect, destrect);
          }
          if (!active & !openSideWindow)
          {
             let mut local15: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local16: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 0, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1, ty + 17, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
          }
        }
        if (active)
        {
           let mut local17: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
           let mut local18: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x1, ty + 17, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local17,  local18, srcrect, destrect);
        }
      }
      return Rectangle::new(x1, ty, 35, 65);
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      bool openingPreviewSet = self.initialOpeningPreviewSet;
      self.initialOpeningPreviewSet = true;
      if (self.game.EditObj.SetViewModeExtraNr == 3 | !openingPreviewSet && self.game.EditObj.layerLisPreview && self.game.EditObj.se1_SelectAssetButton > 0 && self.game.EditObj.layerLisOnlyAssetId > 0 && self.game.EditObj.se1_SelectAssetButton < 1000000 && self.game.EditObj.se1_SelectAssetButton != self.game.EditObj.layerLisOnlyAssetId | !openingPreviewSet)
      {
        let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 148, 0, 0))].GetData(9, self.game.EditObj.se1_SelectAssetButton, 1)));
        bool flag = false;
        if (num >= 2012 & num <= 2017)
          flag = true;
        if (num == 201)
          flag = true;
        if (num >= 311 & num <= 313)
          flag = true;
        if (num >= 941 & num <= 943)
          flag = true;
        if (num >= 3201 & num <= 3207)
          flag = true;
        if (num >= 3261 & num <= 3267)
          flag = true;
        if (flag && flag)
        {
          let mut layerLisOnlyAssetId: i32 = self.game.EditObj.layerLisOnlyAssetId;
          self.game.EditObj.layerLisOnlyAssetId = self.game.EditObj.se1_SelectAssetButton;
          let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
            {
              let mut index3: i32 = 0;
              do
              {
                self.game.Data.MapObj[0].HexObj[index1, index2].tempPreviewAssetLIS[index3] = 0;
                index3 += 1;
              }
              while (index3 <= 8);
            }
          }
          if (self.game.EditObj.layerLisOnlyAssetId > 0)
          {
            self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (layerLisOnlyAssetId > 0)
          {
            self.game.ProcessingObj.LIS_SetNetwork(false, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (self.MouseOverWhichTab <= 0 || self.MouseInThisWindow)
        return windowReturnClass;
      self.MouseOverWhichTab = 0;
      self.dostuff();
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn dostuff()
    {
      libName: String = "SE_Data";
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 123, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 148, 0, 0));
      let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 144, 0, 0));
      let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 149, 0, 0));
      let mut stringListById5: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 284, 0, 0));
      let mut stringListById6: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 273, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName,  byte.MaxValue, 0, 0));
      DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 291, 0, 0));
      DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 256, 0, 0));
      Conversions.ToString(DrawMod.TGame.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[951])));
      if (self.game.SelectX == -1)
        return;
      let mut integer1: i32 = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(self.game.SelectX, self.game.SelectY, libName, "Zones"));
      Conversions.ToInteger(self.game.Data.StringListObj[stringListById1].GetData(0, integer1, 1));
      Conversions.ToInteger(self.game.Data.StringListObj[stringListById1].GetData(0, integer1, 2));
      let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, integer1, 6)));
      data: String = self.game.Data.StringListObj[stringListById1].GetData(0, integer1, 7);
      let mut num1: i32 = self.game.EventRelatedObj.CheckRegimeSlot( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, integer1, 8))), 0, 0, 0);
      let mut index1: i32 = -1;
      if (id > 0)
        index1 = self.game.HandyFunctionsObj.GetLocationByID(id);
      let mut index2: i32 = -1;
      if (index1 > -1)
        index2 = self.game.Data.LocObj[index1].HQ;
      let mut num2: i32 = index2;
      let mut unitSelected: i32 = self.game.EditObj.UnitSelected;
      let mut index3: i32 = -1;
      let mut num3: i32 = -1;
      if (integer1 > 0)
        num3 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, integer1, 8)));
      if (unitSelected > -1)
        index3 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ;
      if (self.but1 > 0)
      {
        self.RemoveSubPart(self.but1);
        self.but1 = 0;
      }
      if (self.but2 > 0)
      {
        self.RemoveSubPart(self.but2);
        self.but2 = 0;
      }
      if (self.but3 > 0)
      {
        self.RemoveSubPart(self.but3);
        self.but3 = 0;
      }
      if (self.but4 > 0)
      {
        self.RemoveSubPart(self.but4);
        self.but4 = 0;
      }
      if (self.but5 > 0)
      {
        self.RemoveSubPart(self.but5);
        self.but5 = 0;
      }
      if (self.but6 > 0)
      {
        self.RemoveSubPart(self.but6);
        self.but6 = 0;
      }
      if (self.but7 > 0)
      {
        self.RemoveSubPart(self.but7);
        self.but7 = 0;
      }
      if (self.but8 > 0)
      {
        self.RemoveSubPart(self.but8);
        self.but8 = 0;
      }
      if (self.but9 > 0)
      {
        self.RemoveSubPart(self.but9);
        self.but9 = 0;
      }
      if (self.but10 > 0)
      {
        self.RemoveSubPart(self.but10);
        self.but10 = 0;
      }
      if (self.but11 > 0)
      {
        self.RemoveSubPart(self.but11);
        self.but11 = 0;
      }
      if (self.but1id > 0)
      {
        self.RemoveSubPart(self.but1id);
        self.but1id = 0;
      }
      if (self.but2id > 0)
      {
        self.RemoveSubPart(self.but2id);
        self.but2id = 0;
      }
      if (self.but3id > 0)
      {
        self.RemoveSubPart(self.but3id);
        self.but3id = 0;
      }
      if (self.game.EditObj.rightSideBarMode == 1 | self.game.ScreenHeight < 1040 && self.profId > 0)
      {
        self.RemoveSubPart(self.profId);
        self.profId = 0;
      }
      if (self.orderAmmoId > 0)
      {
        self.RemoveSubPart(self.orderAmmoId);
        self.orderAmmoId = 0;
      }
      if (self.orderTradeId > 0)
      {
        self.RemoveSubPart(self.orderTradeId);
        self.orderTradeId = 0;
      }
      if (self.orderReplId > 0)
      {
        self.RemoveSubPart(self.orderReplId);
        self.orderReplId = 0;
      }
      if (self.orderRepl2Id > 0)
      {
        self.RemoveSubPart(self.orderRepl2Id);
        self.orderRepl2Id = 0;
      }
      if (self.orderChangeShqId > 0)
      {
        self.RemoveSubPart(self.orderChangeShqId);
        self.orderChangeShqId = 0;
      }
      if (self.orderChangeId > 0)
      {
        self.RemoveSubPart(self.orderChangeId);
        self.orderChangeId = 0;
      }
      if (self.orderColonizeId > 0)
      {
        self.RemoveSubPart(self.orderColonizeId);
        self.orderColonizeId = 0;
      }
      if (self.orderMoreId > 0)
      {
        self.RemoveSubPart(self.orderMoreId);
        self.orderMoreId = 0;
      }
      if (self.orderNewAssetId > 0)
      {
        self.RemoveSubPart(self.orderNewAssetId);
        self.orderNewAssetId = 0;
      }
      if (self.orderNewZoneId > 0)
      {
        self.RemoveSubPart(self.orderNewZoneId);
        self.orderNewZoneId = 0;
      }
      if (self.orderTransferId > 0)
      {
        self.RemoveSubPart(self.orderTransferId);
        self.orderTransferId = 0;
      }
      if (self.orderBattlegroupId2 > 0)
      {
        self.RemoveSubPart(self.orderBattlegroupId2);
        self.orderBattlegroupId2 = 0;
      }
      if (self.orderBattlegroupId > 0)
      {
        self.RemoveSubPart(self.orderBattlegroupId);
        self.orderBattlegroupId = 0;
      }
      if (self.orderProduceUnitId > 0)
      {
        self.RemoveSubPart(self.orderProduceUnitId);
        self.orderProduceUnitId = 0;
      }
      if (self.layerColor > 0)
      {
        self.RemoveSubPart(self.layerColor);
        self.layerColor = 0;
      }
      if (self.layerDetail > 0)
      {
        self.RemoveSubPart(self.layerDetail);
        self.layerDetail = 0;
      }
      if (self.layerGrid > 0)
      {
        self.RemoveSubPart(self.layerGrid);
        self.layerGrid = 0;
      }
      if (self.layerUnit > 0)
      {
        self.RemoveSubPart(self.layerUnit);
        self.layerUnit = 0;
      }
      if (self.layerLabel > 0)
      {
        self.RemoveSubPart(self.layerLabel);
        self.layerLabel = 0;
      }
      if (self.layerLog1 > 0)
      {
        self.RemoveSubPart(self.layerLog1);
        self.layerLog1 = 0;
      }
      if (self.layerLog2 > 0)
      {
        self.RemoveSubPart(self.layerLog2);
        self.layerLog2 = 0;
      }
      if (self.layerLog3 > 0)
      {
        self.RemoveSubPart(self.layerLog3);
        self.layerLog3 = 0;
      }
      if (self.layerLog4 > 0)
      {
        self.RemoveSubPart(self.layerLog4);
        self.layerLog4 = 0;
      }
      if (self.layerLog5 > 0)
      {
        self.RemoveSubPart(self.layerLog5);
        self.layerLog5 = 0;
      }
      if (self.layerLisRange > 0)
      {
        self.RemoveSubPart(self.layerLisRange);
        self.layerLisRange = 0;
      }
      if (self.rlistid > 0)
      {
        self.RemoveSubPart(self.rlistid);
        self.rlistid = 0;
      }
      self.ClearMouse();
      if (index2 == -1 && unitSelected > -1)
      {
        if (self.game.Data.UnitObj[unitSelected].Regime == self.game.Data.Turn)
        {
          let mut historical1: i32 = self.game.Data.UnitObj[unitSelected].Historical;
          if (!self.game.Data.UnitObj[unitSelected].IsHQ | self.game.Data.HistoricalUnitObj[historical1].Type < 8)
          {
            if (index3 > -1)
            {
              if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index3].Historical].Type == 8)
                index2 = index3;
              else if (self.game.Data.UnitObj[index3].HQ > -1)
              {
                let mut historical2: i32 = self.game.Data.UnitObj[self.game.Data.UnitObj[index3].HQ].Historical;
                if (historical2 > -1 && self.game.Data.HistoricalUnitObj[historical2].Type == 8)
                  index2 = self.game.Data.UnitObj[index3].HQ;
              }
            }
          }
          else
            index2 = unitSelected;
        }
        num1 = self.game.Data.UnitObj[unitSelected].Regime;
      }
      if (unitSelected > -1)
      {
        let mut historical: i32 = self.game.Data.UnitObj[unitSelected].Historical;
        if (historical > -1 && self.game.Data.UnitObj[unitSelected].IsHQ & self.game.Data.HistoricalUnitObj[historical].Type == 8 && self.game.Data.UnitObj[unitSelected].Regime == self.game.Data.Turn)
          index2 = unitSelected;
      }
      self.currentShqNr = index2;
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      bool openSideWindow = false;
      if (self.game.EditObj.rightSideBarMode > 0)
        openSideWindow = true;
      let mut x1: i32 = self.w - 40;
      if (openSideWindow)
        x1 = self.w - 185;
      if (openSideWindow)
      {
        BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TEXTURE).RotateFlip(RotateFlipType.RotateNoneFlipX);
        for (let mut index4: i32 = 0; index4 < self.h; index4 += 185)
        {
           let mut local1: &Graphics = &objgraphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TEXTURE);
           let mut local2: &Bitmap = &bitmap;
          let mut x2: i32 = x1 + 40;
          let mut y: i32 = index4;
          DrawMod.DrawSimple( local1,  local2, x2, y);
        }
        BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TEXTURE).RotateFlip(RotateFlipType.RotateNoneFlipX);
      }
      if (!openSideWindow)
      {
         let mut local3: &Graphics = &objgraphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TOPSHADOWRIGHT);
         let mut local4: &Bitmap = &bitmap;
        let mut x3: i32 = self.w - 150;
        DrawMod.DrawSimple( local3,  local4, x3, 0);
      }
       let mut local5: &Graphics = &objgraphics;
      bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_RIGHT);
       let mut local6: &Bitmap = &bitmap1;
      Rectangle trect1 = Rectangle::new(0, 0, 40, 128);
      let mut srcrect1: &Rectangle = &trect1
      Rectangle rectangle = Rectangle::new(x1, 0, 40, 128);
      let mut destrect1: &Rectangle = &rectangle
      DrawMod.DrawSimplePart2( local5,  local6, srcrect1, destrect1);
      for (let mut y: i32 = 128; y < self.h - 128; y += 124)
      {
         let mut local7: &Graphics = &objgraphics;
        bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_RIGHT);
         let mut local8: &Bitmap = &bitmap2;
        rectangle = Rectangle::new(0, 128, 40, 124);
        let mut srcrect2: &Rectangle = &rectangle
        trect1 = Rectangle::new(x1, y, 40, 124);
        let mut destrect2: &Rectangle = &trect1
        DrawMod.DrawSimplePart2( local7,  local8, srcrect2, destrect2);
      }
       let mut local9: &Graphics = &objgraphics;
      bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_RIGHT);
       let mut local10: &Bitmap = &bitmap3;
      rectangle = Rectangle::new(0, 252, 40, 128);
      let mut srcrect3: &Rectangle = &rectangle
      trect1 = Rectangle::new(x1, self.h - 128, 40, 128);
      let mut destrect3: &Rectangle = &trect1
      DrawMod.DrawSimplePart2( local9,  local10, srcrect3, destrect3);
      let mut ty1: i32 = 90;
      Rectangle trect2 = self.DrawOneTab(objgraphics, ty1, self.game.EditObj.rightSideBarMode == 2, openSideWindow, 9, self.MouseOverWhichTab == 2);
      self.AddMouse( trect2, "ORDERS BAR", "All orders available for Units, Zones and SHQs.", 2);
      let mut ty2: i32 = ty1 + 68;
      Rectangle trect3 = self.DrawOneTab(objgraphics, ty2, self.game.EditObj.rightSideBarMode == 3, openSideWindow, 8, self.MouseOverWhichTab == 3);
      self.AddMouse( trect3, "MAP LAYERS BAR", "Toggles to add or switch off Map Layers.", 3);
      let mut ty3: i32 = ty2 + 68;
      Rectangle trect4 = self.DrawOneTab(objgraphics, ty3, self.game.EditObj.rightSideBarMode == 4, openSideWindow, 11, self.MouseOverWhichTab == 4);
      self.AddMouse( trect4, "MAP ORDER MODES", "Same as right clicking on the map, gives you access to change the 'Order Mode' for map/unit clicking you are currently using.", 4);
      let mut num4: i32 = ty3 + 68;
      if (openSideWindow)
      {
        let mut num5: i32 = self.h - 67;
         let mut local11: &Graphics = &objgraphics;
        bitmap4: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_EXITRIGHT);
         let mut local12: &Bitmap = &bitmap4;
        let mut y: i32 = num5;
        DrawMod.DrawSimple( local11,  local12, 8, y);
        if (self.but1id < 1)
        {
          let mut tsubpart: SubPartClass =  new SEButtonPartClass(self.game.SE1_ARROW1, "Hide the right side bar.", 23);
          self.but1id = self.AddSubPart( tsubpart, 15, num5 + 18, 23, 35, 1);
        }
      }
      if (openSideWindow)
      {
        self.orderZoneNr = integer1;
        self.orderShqNr = index2;
        self.orderUnitNr = unitSelected;
        let mut num6: i32 = 45;
        let mut num7: i32 = 10;
        let mut num8: i32 = 0;
        let mut num9: i32 = 0;
        bool flag1 = false;
        if (self.game.ScreenHeight >= 1080)
          flag1 = true;
        if (self.game.EditObj.rightSideBarMode == 4)
        {
          tstring: String = "Order Mode";
           let mut local13: &Graphics = &objgraphics;
          bitmap5: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
           let mut local14: &Bitmap = &bitmap5;
          let mut x4: i32 = num8 + num6;
          let mut y: i32 = num9 + num7;
          DrawMod.DrawSimple( local13,  local14, x4, y);
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring, DrawMod.TGame.MarcFont7, num8 + num6 + 65, num9 + num7 + 12, DrawMod.TGame.seColGray);
          let mut num10: i32 = 34;
          let mut num11: i32 = 32;
          if (self.game.ScreenHeight < 820)
          {
            num10 = 30;
            num11 = 28;
          }
          num8 = 1;
          let mut num12: i32 = num9 + 42;
          bool tPushed1 = false;
          let mut overlay1: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 0)
          {
            tPushed1 = true;
            overlay1 = 0;
          }
          let mut tsubpart1: SubPartClass =  new SEButtonPushPartClassWithText("Inspect", 38, tPushed1, "Allows you to just look at your units [Shortkey I]", 132, num11);
          self.but1 = self.AddSubPart( tsubpart1, num8 + num6, num12 + num7, 132, num11, overlay1);
          let mut num13: i32 = num12 + num10;
          bool tPushed2 = false;
          let mut overlay2: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 1)
          {
            tPushed2 = true;
            overlay2 = 0;
          }
          let mut tsubpart2: SubPartClass =  new SEButtonPushPartClassWithText("Move", 31, tPushed2, "Allows you to move and attack with your units [Shortkey M]", 132, num11);
          self.but2 = self.AddSubPart( tsubpart2, num8 + num6, num13 + num7, 132, num11, overlay2);
          let mut num14: i32 = num13 + num10;
          bool tPushed3 = false;
          let mut overlay3: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 48)
          {
            tPushed3 = true;
            overlay3 = 0;
          }
          let mut tsubpart3: SubPartClass =  new SEButtonPushPartClassWithText("Group Move", 31, tPushed3, "Allows you to move and attack with all units in the Hex [Shortkey G]", 132, num11);
          self.but7 = self.AddSubPart( tsubpart3, num8 + num6, num14 + num7, 132, num11, overlay3);
          let mut num15: i32 = num14 + num10;
          bool tPushed4 = false;
          let mut overlay4: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 18)
          {
            tPushed4 = true;
            overlay4 = 0;
          }
          let mut tsubpart4: SubPartClass =  new SEButtonPushPartClassWithText("Strat. Move", 25, tPushed4, "Allows your units to be transfered by your logistical network [Shortkey S]", 132, num11);
          self.but3 = self.AddSubPart( tsubpart4, num8 + num6, num15 + num7, 132, num11, overlay4);
          let mut num16: i32 = num15 + num10;
          bool tPushed5 = false;
          let mut overlay5: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 11)
          {
            tPushed5 = true;
            overlay5 = 0;
          }
          let mut tsubpart5: SubPartClass =  new SEButtonPushPartClassWithText("Ranged Att.", 39, tPushed5, "Allows your to target units with artillery or missile fire [Shortkey A]", 132, num11);
          self.but4 = self.AddSubPart( tsubpart5, num8 + num6, num16 + num7, 132, num11, overlay5);
          if (self.game.EventRelatedObj.Helper_AirEnabled())
          {
            let mut num17: i32 = num16 + num10;
            bool tPushed6 = false;
            let mut overlay6: i32 = 1;
            if (self.game.EditObj.udsUnitOrderMode == 14)
            {
              tPushed6 = true;
              overlay6 = 0;
            }
            let mut tsubpart6: SubPartClass =  new SEButtonPushPartClassWithText("Air Attack", 61, tPushed6, "Allows your to target units with airstrikes [Shortkey X]", 132, num11);
            self.but9 = self.AddSubPart( tsubpart6, num8 + num6, num17 + num7, 132, num11, overlay6);
            let mut num18: i32 = num17 + num10;
            bool tPushed7 = false;
            let mut overlay7: i32 = 1;
            if (self.game.EditObj.udsUnitOrderMode == 33)
            {
              tPushed7 = true;
              overlay7 = 0;
            }
            let mut tsubpart7: SubPartClass =  new SEButtonPushPartClassWithText("Air Recon", 63, tPushed7, "Allows your to do recon missions on Hexes [Shortkey Y]", 132, num11);
            self.but10 = self.AddSubPart( tsubpart7, num8 + num6, num18 + num7, 132, num11, overlay7);
            num16 = num18 + num10;
            bool tPushed8 = false;
            let mut overlay8: i32 = 1;
            if (self.game.EditObj.udsUnitOrderMode == 55)
            {
              tPushed8 = true;
              overlay8 = 0;
            }
            let mut tsubpart8: SubPartClass =  new SEButtonPushPartClassWithText("Air Bridge", 62, tPushed8, "Allows your to do order Air Bridges", 132, num11);
            self.but11 = self.AddSubPart( tsubpart8, num8 + num6, num16 + num7, 132, num11, overlay8);
          }
          if ( self.game.Data.RuleVar[702] > 0.0)
          {
            num16 += num10;
            bool tPushed9 = false;
            let mut overlay9: i32 = 1;
            if (self.game.EditObj.udsUnitOrderMode == 36)
            {
              tPushed9 = true;
              overlay9 = 0;
            }
            let mut tsubpart9: SubPartClass =  new SEButtonPushPartClassWithText("Constr. Road", 16, tPushed9, "Allows your to construct roads [Shortkey R]", 132, num11);
            self.but5 = self.AddSubPart( tsubpart9, num8 + num6, num16 + num7, 132, num11, overlay9);
          }
          let mut num19: i32 = num16 + num10;
          bool tPushed10 = false;
          let mut overlay10: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 53)
          {
            tPushed10 = true;
            overlay10 = 0;
          }
          let mut tsubpart10: SubPartClass =  new SEButtonPushPartClassWithText("Traffic Signs", 53, tPushed10, "Allows you to place and remove Traffic Signs [Shortkey T]", 132, num11);
          self.but6 = self.AddSubPart( tsubpart10, num8 + num6, num19 + num7, 132, num11, overlay10);
          let mut num20: i32 = num19 + num10;
          bool tPushed11 = false;
          let mut overlay11: i32 = 1;
          if (self.game.EditObj.udsUnitOrderMode == 54)
          {
            tPushed11 = true;
            overlay11 = 0;
          }
          let mut tsubpart11: SubPartClass =  new SEButtonPushPartClassWithText("Zone Borders", 38, tPushed11, "Allows you to redraw the borders of your Zones [Shortkey Z]", 132, num11);
          self.but8 = self.AddSubPart( tsubpart11, num8 + num6, num20 + num7, 132, num11, overlay11);
          num9 = num20 + 47;
        }
        SubPartClass tsubpart12;
        if (self.game.EditObj.rightSideBarMode == 3)
        {
          tstring1: String = "Logistics";
           let mut local15: &Graphics = &objgraphics;
          bitmap6: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
           let mut local16: &Bitmap = &bitmap6;
          let mut x5: i32 = num8 + num6;
          let mut y1: i32 = num9 + num7;
          DrawMod.DrawSimple( local15,  local16, x5, y1);
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring1, DrawMod.TGame.MarcFont7, num8 + num6 + 65, num9 + num7 + 12, DrawMod.TGame.seColGray);
          if (!flag1)
          {
            let mut num21: i32 = 1;
            let mut num22: i32 = num9 + 42;
            let mut tsubpart13: SubPartClass =  new SEButtonPushPartClass(50, self.game.EditObj.layerLisAvailable, "Show Current Logistical Points that you have left", 42, 40);
            self.layerLog1 = self.AddSubPart( tsubpart13, num21 + num6, num22 + num7, 42, 42, 1);
            let mut num23: i32 = num21 + 45;
            let mut tsubpart14: SubPartClass =  new SEButtonPushPartClass(51, self.game.EditObj.layerLisUsed, "Show Used Logistical Points during your turn so far", 42, 40);
            self.layerLog2 = self.AddSubPart( tsubpart14, num23 + num6, num22 + num7, 42, 42, 1);
            let mut num24: i32 = num23 + 45;
            let mut tsubpart15: SubPartClass =  new SEButtonPushPartClass(52, self.game.EditObj.layerLisBottlenecks, "Show Bottlenecks in Logistical Pousage: i32 during your turn so far", 42, 40);
            self.layerLog3 = self.AddSubPart( tsubpart15, num24 + num6, num22 + num7, 42, 42, 1);
            let mut num25: i32 = 1;
            let mut num26: i32 = num22 + 45;
            let mut tsubpart16: SubPartClass =  new SEButtonPushPartClass(49, self.game.EditObj.layerLisTotal, "Show your initial start-of-turn Logistical Points before any usage", 42, 40);
            self.layerLog4 = self.AddSubPart( tsubpart16, num25 + num6, num26 + num7, 42, 42, 1);
            let mut num27: i32 = num25 + 45;
            let mut tsubpart17: SubPartClass =  new SEButtonPushPartClass(31, self.game.EditObj.layerLisPreview, "Show a preview of upcomming turn Logistical Points", 42, 40);
            self.layerLog5 = self.AddSubPart( tsubpart17, num27 + num6, num26 + num7, 42, 42, 1);
            num8 = 0;
            num9 = num26 + 55;
          }
          else
          {
            num8 = 1;
            let mut num28: i32 = num9 + 42;
            let mut tsubpart18: SubPartClass =  new SEButtonPushPartClassWithText("Current Pts", 50, self.game.EditObj.layerLisAvailable, "Show Current Logistical Points that you have left [Shortkey L]", 132, 32);
            self.layerLog1 = self.AddSubPart( tsubpart18, num8 + num6, num28 + num7, 132, 32, 1);
            let mut num29: i32 = num28 + 34;
            let mut tsubpart19: SubPartClass =  new SEButtonPushPartClassWithText("Used Pts", 51, self.game.EditObj.layerLisUsed, "Show Used Logistical Points during your turn so far", 132, 32);
            self.layerLog2 = self.AddSubPart( tsubpart19, num8 + num6, num29 + num7, 132, 32, 1);
            let mut num30: i32 = num29 + 34;
            let mut tsubpart20: SubPartClass =  new SEButtonPushPartClassWithText("Initial Pts", 49, self.game.EditObj.layerLisTotal, "Show your initial start-of-turn Logistical Points before any usage", 132, 32);
            self.layerLog4 = self.AddSubPart( tsubpart20, num8 + num6, num30 + num7, 132, 32, 1);
            let mut num31: i32 = num30 + 34;
            let mut tsubpart21: SubPartClass =  new SEButtonPushPartClassWithText("Bottlenecks", 52, self.game.EditObj.layerLisBottlenecks, "Show Bottlenecks in Logistical Pousage: i32 during your turn so far", 132, 32);
            self.layerLog3 = self.AddSubPart( tsubpart21, num8 + num6, num31 + num7, 132, 32, 1);
            let mut num32: i32 = num31 + 34;
            let mut tsubpart22: SubPartClass =  new SEButtonPushPartClassWithText("Preview Pts", 31, self.game.EditObj.layerLisPreview, "Show a preview of upcomming turn Logistical Points [Shortkey P]", 132, 32);
            self.layerLog5 = self.AddSubPart( tsubpart22, num8 + num6, num32 + num7, 132, 32, 1);
            num9 = num32 + 47;
          }
          if (self.game.EditObj.layerLisPreview)
          {
            tstring2: String = "Zone Log. Assets";
             let mut local17: &Graphics = &objgraphics;
            bitmap7: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local18: &Bitmap = &bitmap7;
            let mut x6: i32 = num8 + num6;
            let mut y2: i32 = num9 + num7;
            DrawMod.DrawSimple( local17,  local18, x6, y2);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring2, DrawMod.TGame.MarcFont7, num8 + num6 + 65, num9 + num7 + 12, DrawMod.TGame.seColGray);
            let mut num33: i32 = num9 + 52;
            let mut num34: i32 = self.h - num33 - 20;
            if (num34 >= 200)
              num34 = 200;
            let mut tlistsize: i32 =  Math.Round(Math.Floor( num34 / 20.0)) - 1;
            let mut tlistselect: i32 = -1;
            let mut num35: i32 = 0;
            self.rlistobj = ListClass::new();
            self.rlistobj.add("None", 0);
            if (1 > self.game.EditObj.layerLisOnlyAssetId)
              tlistselect = num35;
            let mut length: i32 = self.game.Data.StringListObj[stringListById2].Length;
            for (let mut index5: i32 = 0; index5 <= length; index5 += 1)
            {
              if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index5, 0])) == integer1)
              {
                let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index5, 1]));
                bool flag2 = false;
                if (idValue >= 2012 & idValue <= 2017)
                  flag2 = true;
                if (idValue == 201)
                  flag2 = true;
                if (idValue >= 311 & idValue <= 313)
                  flag2 = true;
                if (idValue >= 941 & idValue <= 943)
                  flag2 = true;
                if (idValue >= 3201 & idValue <= 3207)
                  flag2 = true;
                if (idValue >= 3261 & idValue <= 3267)
                  flag2 = true;
                if (flag2)
                {
                  let mut tdata: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index5, 9]));
                  str: String = self.game.Data.StringListObj[stringListById3].GetData(0, idValue, 12);
                  let mut integer2: i32 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById3].GetData(0, idValue, 2));
                  let mut index6: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index5, 3]));
                  let mut index7: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index5, 4]));
                  if (index6 > -1 & index7 > -1 && self.game.Data.MapObj[0].HexObj[index6, index7].Regime == self.game.Data.Turn)
                  {
                    if (str.Length > 13)
                      str = Strings.Left(str, 13) + ".";
                    if (integer2 > 0)
                      str = str + " " + self.game.HandyFunctionsObj.GetRomanNumerical(integer2);
                    num35 += 1;
                    tname: String = str + " (" + index6.ToString() + "," + index7.ToString() + ")";
                    if (self.game.EditObj.layerLisOnlyAssetId == tdata)
                      tlistselect = num35;
                    self.rlistobj.add(tname, tdata);
                  }
                }
              }
            }
            if (!(self.game.EditObj.layerLisOnlyAssetId > 0 & tlistselect == -1) && tlistselect <= 0)
              tlistselect = 0;
            let mut tsubpart23: SubPartClass =  new ListSubPartClass(self.rlistobj, tlistsize, 136, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num8 + num6), bby: num33, tMarcStyle: true, overruleFont: ( self.game.MarcFont5), overruleItemSize: 20);
            self.rlistid = self.AddSubPart( tsubpart23, num8 + num6, num33, 136, tlistsize * 20, 1);
            num9 = num33 + (tlistsize * 20 + 20);
          }
          if (!self.game.EditObj.layerLisPreview | flag1 & num9 + 315 < self.h | !flag1 & num9 + 135 < self.h)
          {
            tstring3: String = "Map Layers";
             let mut local19: &Graphics = &objgraphics;
            bitmap8: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local20: &Bitmap = &bitmap8;
            let mut x7: i32 = num8 + num6;
            let mut y3: i32 = num9 + num7;
            DrawMod.DrawSimple( local19,  local20, x7, y3);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring3, DrawMod.TGame.MarcFont7, num8 + num6 + 65, num9 + num7 + 12, DrawMod.TGame.seColGray);
            if (!flag1)
            {
              let mut num36: i32 = 1;
              let mut num37: i32 = num9 + 42;
              let mut tsubpart24: SubPartClass =  new SEButtonPushPartClass(53, !self.game.EditObj.layerUnits, "Hide Units", 42, 40);
              self.layerUnit = self.AddSubPart( tsubpart24, num36 + num6, num37 + num7, 42, 42, 1);
              let mut num38: i32 = num36 + 45;
              let mut tsubpart25: SubPartClass =  new SEButtonPushPartClass(54, !self.game.EditObj.ShowLabel, "Hide Map Labels", 42, 40);
              self.layerLabel = self.AddSubPart( tsubpart25, num38 + num6, num37 + num7, 42, 42, 1);
              let mut num39: i32 = num38 + 45;
              let mut tsubpart26: SubPartClass =  new SEButtonPushPartClass(55, self.game.EditObj.HideAS, "Hide AP and Stack penalties", 42, 40);
              self.layerDetail = self.AddSubPart( tsubpart26, num39 + num6, num37 + num7, 42, 42, 1);
              num9 = num37 + 45;
              let mut num40: i32 = 1;
              let mut tsubpart27: SubPartClass =  new SEButtonPushPartClass(56, self.game.EditObj.HexRasterOn, "Show Hex Grid", 42, 40);
              self.layerGrid = self.AddSubPart( tsubpart27, num40 + num6, num9 + num7, 42, 42, 1);
              let mut num41: i32 = num40 + 45;
              let mut tsubpart28: SubPartClass =  new SEButtonPushPartClass(57, self.game.EditObj.RegimeColoring, "Show Regime Coloring", 42, 40);
              self.layerColor = self.AddSubPart( tsubpart28, num41 + num6, num9 + num7, 42, 42, 1);
              num8 = num41 + 45;
              tsubpart12 =  new SEButtonPushPartClass(50, self.game.EditObj.ShowLISRange, "Show the range in which Units Operational Logistics will be able to pick up Supplies from Roads.", 42, 40);
              self.layerLisRange = self.AddSubPart( tsubpart12, num8 + num6, num9 + num7, 42, 42, 1);
            }
            else
            {
              num8 = 1;
              let mut num42: i32 = num9 + 42;
              let mut tsubpart29: SubPartClass =  new SEButtonPushPartClassWithText("Hide Units", 53, !self.game.EditObj.layerUnits, "Hide Units [Shortkey 1]", 132, 32);
              self.layerUnit = self.AddSubPart( tsubpart29, num8 + num6, num42 + num7, 132, 32, 1);
              let mut num43: i32 = num42 + 34;
              let mut tsubpart30: SubPartClass =  new SEButtonPushPartClassWithText("Hide Labels", 54, !self.game.EditObj.ShowLabel, "Hide Map Labels [Shortkey 2]", 132, 32);
              self.layerLabel = self.AddSubPart( tsubpart30, num8 + num6, num43 + num7, 132, 32, 1);
              let mut num44: i32 = num43 + 34;
              let mut tsubpart31: SubPartClass =  new SEButtonPushPartClassWithText("Hide Penalty", 55, self.game.EditObj.HideAS, "Hide AP and Stack penalties [Shortkey 3]", 132, 32);
              self.layerDetail = self.AddSubPart( tsubpart31, num8 + num6, num44 + num7, 132, 32, 1);
              let mut num45: i32 = num44 + 34;
              let mut tsubpart32: SubPartClass =  new SEButtonPushPartClassWithText("Show Grid", 56, self.game.EditObj.HexRasterOn, "Show Hex Grid [Shortkey 4]", 132, 32);
              self.layerGrid = self.AddSubPart( tsubpart32, num8 + num6, num45 + num7, 132, 32, 1);
              let mut num46: i32 = num45 + 34;
              let mut tsubpart33: SubPartClass =  new SEButtonPushPartClassWithText("Show Color", 57, self.game.EditObj.RegimeColoring, "Show Regime Coloring [Shortkey 5]", 132, 32);
              self.layerColor = self.AddSubPart( tsubpart33, num8 + num6, num46 + num7, 132, 32, 1);
              num9 = num46 + 34;
              let mut tsubpart34: SubPartClass =  new SEButtonPushPartClassWithText("Show Op. Log", 50, self.game.EditObj.ShowLISRange, "Show the range in which Units Operational Logistics will be able to pick up Supplies from Roads. [Shortkey 6]", 132, 32);
              self.layerLisRange = self.AddSubPart( tsubpart34, num8 + num6, num9 + num7, 132, 32, 1);
            }
          }
        }
        if (self.game.EditObj.rightSideBarMode == 2)
        {
          num47: i32;
          num48: i32;
          if (index2 > -1 & self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime == self.game.Data.Turn)
          {
            str: String = self.game.Data.UnitObj[index2].Name;
            if (str.Length > 15)
              str = Strings.Left(str, 15) + ".";
             let mut local21: &Graphics = &objgraphics;
            bitmap9: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local22: &Bitmap = &bitmap9;
            let mut x8: i32 = num8 + num6;
            let mut y: i32 = num9 + num7;
            DrawMod.DrawSimple( local21,  local22, x8, y);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, str, DrawMod.TGame.MarcFont7, num8 + num6 + 65, num9 + num7 + 12, DrawMod.TGame.seColGray);
            let mut num49: i32 = 1;
            let mut num50: i32 = num9 + 42;
            if (!flag1)
            {
              tsubpart12 =  new SEButtonPartClass(-1, "Workshop", 42, 42, 40);
              self.orderAmmoId = self.AddSubPart( tsubpart12, num49 + num6, num50 + num7, 42, 42, 1);
              let mut num51: i32 = num49 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Replacement Troops", 42, 42, 41);
              self.orderReplId = self.AddSubPart( tsubpart12, num51 + num6, num50 + num7, 42, 42, 1);
              let mut num52: i32 = num51 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Trade", 42, 42, 42);
              self.orderTradeId = self.AddSubPart( tsubpart12, num52 + num6, num50 + num7, 42, 42, 1);
              let mut num53: i32 = num50 + 45;
              let mut num54: i32 = 1;
              tsubpart12 =  new SEButtonPartClass(-1, "Transfer", 42, 42, 39);
              self.orderTransferId = self.AddSubPart( tsubpart12, num54 + num6, num53 + num7, 42, 42, 1);
              num47 = 0;
              num48 = num53 + 55;
            }
            else
            {
              let mut num55: i32 = 1;
              tsubpart12 =  new SEButtonPartClassWithText("Workshop", -1, "Workshop (produce Ammo and Machines)", 132, 32, 40);
              self.orderAmmoId = self.AddSubPart( tsubpart12, num55 + num6, num50 + num7, 132, 32, 1);
              let mut num56: i32 = num50 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Repl.Troops", -1, "Replacement Troops", 132, 32, 41);
              self.orderReplId = self.AddSubPart( tsubpart12, num55 + num6, num56 + num7, 132, 32, 1);
              let mut num57: i32 = num56 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Trade", -1, "Trade", 132, 32, 42);
              self.orderTradeId = self.AddSubPart( tsubpart12, num55 + num6, num57 + num7, 132, 32, 1);
              let mut num58: i32 = num57 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Transfer", -1, "Transfer", 132, 32, 39);
              self.orderTransferId = self.AddSubPart( tsubpart12, num55 + num6, num58 + num7, 132, 32, 1);
              num47 = 0;
              num48 = num58 + 47;
            }
          }
          else
          {
            str: String = "No SHQ";
            if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime != self.game.Data.Turn)
              str = "Unfriendly";
            if (str.Length > 15)
              str = Strings.Left(str, 15) + ".";
             let mut local23: &Graphics = &objgraphics;
            bitmap10: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local24: &Bitmap = &bitmap10;
            let mut x9: i32 = num8 + num6;
            let mut y: i32 = num9 + num7;
            DrawMod.DrawSimple( local23,  local24, x9, y);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, str, DrawMod.TGame.MarcFont7, num8 + num6 + 65, num9 + num7 + 12, DrawMod.TGame.seColGray);
            let mut num59: i32 = 1;
            let mut num60: i32 = num9 + 42;
            if (!flag1)
            {
              tsubpart12 =  new SEButtonPartClass(-1, "Workshop: Produce Ammunitions & Machines", 42, 42, 40, true);
              self.orderAmmoId = self.AddSubPart( tsubpart12, num59 + num6, num60 + num7, 42, 42, 0);
              let mut num61: i32 = num59 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Produce Replacement Troops", 42, 42, 41, true);
              self.orderReplId = self.AddSubPart( tsubpart12, num61 + num6, num60 + num7, 42, 42, 0);
              let mut num62: i32 = num61 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Trade with the Traders in SHQ its Zone", 42, 42, 42, true);
              self.orderTradeId = self.AddSubPart( tsubpart12, num62 + num6, num60 + num7, 42, 42, 0);
              let mut num63: i32 = num60 + 45;
              let mut num64: i32 = 1;
              tsubpart12 =  new SEButtonPartClass(-1, "Use Logistical Network to send Items / Troops", 42, 42, 39, true);
              self.orderTransferId = self.AddSubPart( tsubpart12, num64 + num6, num63 + num7, 42, 42, 0);
              num47 = 0;
              num48 = num63 + 55;
            }
            else
            {
              let mut num65: i32 = 1;
              tsubpart12 =  new SEButtonPartClassWithText("Workshop", -1, "Workshop (produce Ammo and Machines)", 132, 32, 40, true);
              self.orderAmmoId = self.AddSubPart( tsubpart12, num65 + num6, num60 + num7, 132, 32, 0);
              let mut num66: i32 = num60 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Repl.Troops", -1, "Replacement Troops", 132, 32, 41, true);
              self.orderReplId = self.AddSubPart( tsubpart12, num65 + num6, num66 + num7, 132, 32, 0);
              let mut num67: i32 = num66 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Trade", -1, "Trade", 132, 32, 42, true);
              self.orderTradeId = self.AddSubPart( tsubpart12, num65 + num6, num67 + num7, 132, 32, 0);
              let mut num68: i32 = num67 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Transfer", -1, "Transfer", 132, 32, 39, true);
              self.orderTransferId = self.AddSubPart( tsubpart12, num65 + num6, num68 + num7, 132, 32, 0);
              num47 = 0;
              num48 = num68 + 47;
            }
          }
          num69: i32;
          num70: i32;
          if (integer1 > -1 & num3 == self.game.Data.RegimeObj[self.game.Data.Turn].id & self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime == self.game.Data.Turn)
          {
            str1: String = data;
            if (str1.Length > 15)
              str1 = Strings.Left(str1, 15) + ".";
             let mut local25: &Graphics = &objgraphics;
            bitmap11: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local26: &Bitmap = &bitmap11;
            let mut x10: i32 = num47 + num6;
            let mut y: i32 = num48 + num7;
            DrawMod.DrawSimple( local25,  local26, x10, y);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, str1, DrawMod.TGame.MarcFont7, num47 + num6 + 65, num48 + num7 + 12, DrawMod.TGame.seColGray);
            num71: i32;
            num72: i32;
            if (!flag1)
            {
              let mut num73: i32 = 1;
              let mut num74: i32 = num48 + 42;
              tsubpart12 =  new SEButtonPartClass(-1, "Change SHQ", 42, 42, 37);
              self.orderChangeShqId = self.AddSubPart( tsubpart12, num73 + num6, num74 + num7, 42, 42, 1);
              let mut num75: i32 = num73 + 45;
              if (num2 > -1)
              {
                tsubpart12 =  new SEButtonPartClass(-1, "New Formation", 42, 42, 43);
                self.orderProduceUnitId = self.AddSubPart( tsubpart12, num75 + num6, num74 + num7, 42, 42, 1);
                let mut num76: i32 = num75 + 45;
                tsubpart12 =  new SEButtonPartClass(-1, "Colonize", 42, 42, 18);
                self.orderColonizeId = self.AddSubPart( tsubpart12, num76 + num6, num74 + num7, 42, 42, 1);
                num71 = num74 + 45;
                let mut num77: i32 = 1;
                tsubpart12 =  new SEButtonPartClass(-1, "Construction", 42, 42, 46);
                self.orderNewAssetId = self.AddSubPart( tsubpart12, num77 + num6, num71 + num7, 42, 42, 1);
                num72 = num77 + 45;
              }
              else
              {
                tsubpart12 =  new SEButtonPartClass(-1, "Form a new Formation on the selected Hex", 42, 42, 43, true);
                self.orderProduceUnitId = self.AddSubPart( tsubpart12, num75 + num6, num74 + num7, 42, 42, 0);
                let mut num78: i32 = num75 + 45;
                tsubpart12 =  new SEButtonPartClass(-1, "Send Colonists to this Zone", 42, 42, 18, true);
                self.orderColonizeId = self.AddSubPart( tsubpart12, num78 + num6, num74 + num7, 42, 42, 0);
                num71 = num74 + 45;
                let mut num79: i32 = 1;
                tsubpart12 =  new SEButtonPartClass(-1, "Start Construction of a new Asset on selected Hex", 42, 42, 46, true);
                self.orderNewAssetId = self.AddSubPart( tsubpart12, num79 + num6, num71 + num7, 42, 42, 0);
                num72 = num79 + 45;
              }
            }
            else
            {
              num72 = 1;
              let mut num80: i32 = num48 + 42;
              tsubpart12 =  new SEButtonPartClassWithText("Zone SHQ", -1, "Change SHQ", 132, 32, 37);
              self.orderChangeShqId = self.AddSubPart( tsubpart12, num72 + num6, num80 + num7, 132, 32, 1);
              let mut num81: i32 = num80 + 34;
              if (num2 > -1)
              {
                tsubpart12 =  new SEButtonPartClassWithText("Raise Form.", -1, "New Formation", 132, 32, 43);
                self.orderProduceUnitId = self.AddSubPart( tsubpart12, num72 + num6, num81 + num7, 132, 32, 1);
                let mut num82: i32 = num81 + 34;
                tsubpart12 =  new SEButtonPartClassWithText("Colonize", -1, "Colonize", 132, 32, 18);
                self.orderColonizeId = self.AddSubPart( tsubpart12, num72 + num6, num82 + num7, 132, 32, 1);
                let mut num83: i32 = num82 + 34;
                tsubpart12 =  new SEButtonPartClassWithText("Construct", -1, "Construction", 132, 32, 46);
                self.orderNewAssetId = self.AddSubPart( tsubpart12, num72 + num6, num83 + num7, 132, 32, 1);
                num71 = num83 + 34;
              }
              else
              {
                tsubpart12 =  new SEButtonPartClassWithText("Raise Form.", -1, "New Formation", 132, 32, 43, true);
                self.orderProduceUnitId = self.AddSubPart( tsubpart12, num72 + num6, num81 + num7, 132, 32, 0);
                let mut num84: i32 = num81 + 34;
                tsubpart12 =  new SEButtonPartClassWithText("Colonize", -1, "Colonize", 132, 32, 18, true);
                self.orderColonizeId = self.AddSubPart( tsubpart12, num72 + num6, num84 + num7, 132, 32, 0);
                let mut num85: i32 = num84 + 34;
                tsubpart12 =  new SEButtonPartClassWithText("Construct", -1, "Construction", 132, 32, 46, true);
                self.orderNewAssetId = self.AddSubPart( tsubpart12, num72 + num6, num85 + num7, 132, 32, 0);
                num71 = num85 + 34;
              }
            }
            let mut idValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData3(3, self.game.SelectX, 4, self.game.SelectY, 8, 0, 9)));
            let mut idValue2: i32 = self.orderZoneNr;
            if (idValue1 > 0)
              idValue2 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(9, idValue1, 0)));
            let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, idValue2, 6))));
            let mut num86: i32 = 0;
            str2: String = "";
            if (self.game.Data.RegimeObj[self.game.Data.Turn].ResPts < 5)
            {
              str2 = "You need at least 5 PP to make a new Zone";
              num86 = 1;
            }
            if (5 > self.game.EventRelatedObj.Helper_GetClosestOtherCity(self.game.SelectX, self.game.SelectY))
            {
              str2 = "You can only create a new Zone at distance 5 or more from an existing City";
              num86 = 1;
            }
            if (idValue1 < 1)
            {
              str2 = "No asset present in selected hex";
              num86 = 1;
            }
            if (locationById > -1 && self.game.Data.LocObj[locationById].X == self.game.SelectX & self.game.Data.LocObj[locationById].Y == self.game.SelectY)
            {
              str2 = "You need to select an out-of-city Asset to make a new Zone";
              num86 = 1;
            }
            if (num2 == -1)
            {
              str2 = "Hex needs to be in a Zone with an SHQ assigned to it";
              num86 = 1;
            }
            if (Operators.CompareString(str2, "", false) == 0)
              str2 = "Found new Zone [5PP]";
            if (!flag1)
            {
              if (num86 == 0)
              {
                tsubpart12 =  new SEButtonPartClass(-1, str2, 42, 42, 44, num86 == 1);
                self.orderNewZoneId = self.AddSubPart( tsubpart12, num72 + num6, num71 + num7, 42, 42, 1);
              }
              else
              {
                tsubpart12 =  new SEButtonPartClass(-1, str2, 42, 42, 44, num86 == 1);
                self.orderNewZoneId = self.AddSubPart( tsubpart12, num72 + num6, num71 + num7, 42, 42, 0);
              }
              num69 = 0;
              num70 = num71 + 55;
            }
            else
            {
              if (num86 == 0)
              {
                tsubpart12 =  new SEButtonPartClassWithText("New Zone", -1, str2, 132, 32, 44, num86 == 1);
                self.orderNewZoneId = self.AddSubPart( tsubpart12, num72 + num6, num71 + num7, 132, 32, 1);
              }
              else
              {
                tsubpart12 =  new SEButtonPartClassWithText("New Zone", -1, str2, 132, 32, 44, num86 == 1);
                self.orderNewZoneId = self.AddSubPart( tsubpart12, num72 + num6, num71 + num7, 132, 32, 0);
              }
              num69 = 0;
              num70 = num71 + 47;
            }
          }
          else
          {
            str: String = "No Zone";
            if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime == self.game.Data.Turn)
              str = "Unfriendly";
            if (num3 != self.game.Data.RegimeObj[self.game.Data.Turn].id & self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime == self.game.Data.Turn)
              str = "Zone in Conquest";
            if (str.Length > 16)
              str = Strings.Left(str, 16) + ".";
             let mut local27: &Graphics = &objgraphics;
            bitmap12: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local28: &Bitmap = &bitmap12;
            let mut x11: i32 = num47 + num6;
            let mut y: i32 = num48 + num7;
            DrawMod.DrawSimple( local27,  local28, x11, y);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, str, DrawMod.TGame.MarcFont7, num47 + num6 + 65, num48 + num7 + 12, DrawMod.TGame.seColGray);
            if (!flag1)
            {
              let mut num87: i32 = 1;
              let mut num88: i32 = num48 + 42;
              tsubpart12 =  new SEButtonPartClass(-1, "Change the SHQ that this Zone is assigned to", 42, 42, 37, true);
              self.orderChangeShqId = self.AddSubPart( tsubpart12, num87 + num6, num88 + num7, 42, 42, 0);
              let mut num89: i32 = num87 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Form a new Formation on the selected Hex", 42, 42, 43, true);
              self.orderProduceUnitId = self.AddSubPart( tsubpart12, num89 + num6, num88 + num7, 42, 42, 0);
              let mut num90: i32 = num89 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Send Colonists to this Zone", 42, 42, 18, true);
              self.orderColonizeId = self.AddSubPart( tsubpart12, num90 + num6, num88 + num7, 42, 42, 0);
              let mut num91: i32 = num88 + 45;
              let mut num92: i32 = 1;
              tsubpart12 =  new SEButtonPartClass(-1, "Start Construction of a new Asset on selected Hex", 42, 42, 46, true);
              self.orderNewAssetId = self.AddSubPart( tsubpart12, num92 + num6, num91 + num7, 42, 42, 0);
              let mut num93: i32 = num92 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, str, 42, 42, 44, true);
              self.orderNewZoneId = self.AddSubPart( tsubpart12, num93 + num6, num91 + num7, 42, 42, 0);
              num69 = 0;
              num70 = num91 + 55;
            }
            else
            {
              num69 = 1;
              let mut num94: i32 = num48 + 42;
              tsubpart12 =  new SEButtonPartClassWithText("Zone SHQ", -1, "Change SHQ", 132, 32, 37, true);
              self.orderChangeShqId = self.AddSubPart( tsubpart12, num69 + num6, num94 + num7, 132, 32, 0);
              let mut num95: i32 = num94 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Raise Form.", -1, "New Formation", 132, 32, 43, true);
              self.orderProduceUnitId = self.AddSubPart( tsubpart12, num69 + num6, num95 + num7, 132, 32, 0);
              let mut num96: i32 = num95 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Colonize", -1, "Colonize", 132, 32, 18, true);
              self.orderColonizeId = self.AddSubPart( tsubpart12, num69 + num6, num96 + num7, 132, 32, 0);
              let mut num97: i32 = num96 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Construct", -1, "Construction", 132, 32, 46, true);
              self.orderNewAssetId = self.AddSubPart( tsubpart12, num69 + num6, num97 + num7, 132, 32, 0);
              let mut num98: i32 = num97 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("New Zone", -1, str, 132, 32, 44, true);
              self.orderNewZoneId = self.AddSubPart( tsubpart12, num69 + num6, num98 + num7, 132, 32, 0);
              num70 = num98 + 47;
            }
          }
          bool flag3 = true;
          if (unitSelected < 0)
            flag3 = false;
          if (unitSelected > -1 && self.game.Data.UnitObj[unitSelected].Regime != self.game.Data.Turn)
            flag3 = false;
          if (flag3)
          {
            str: String = self.game.Data.UnitObj[unitSelected].Name;
            if (str.Length > 15)
              str = Strings.Left(str, 15) + ".";
             let mut local29: &Graphics = &objgraphics;
            bitmap13: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local30: &Bitmap = &bitmap13;
            let mut x12: i32 = num69 + num6;
            let mut y: i32 = num70 + num7;
            DrawMod.DrawSimple( local29,  local30, x12, y);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, str, DrawMod.TGame.MarcFont7, num69 + num6 + 65, num70 + num7 + 12, DrawMod.TGame.seColGray);
            let mut num99: i32 = 1;
            let mut num100: i32 = num70 + 42;
            if (!flag1)
            {
              tsubpart12 =  new SEButtonPartClass(-1, "Order of Battle", 42, 42, 1);
              self.orderChangeId = self.AddSubPart( tsubpart12, num99 + num6, num100 + num7, 42, 42, 1);
              if (self.orderShqNr != self.orderUnitNr)
              {
                if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.orderUnitNr].Historical].GiveHisVarValue(11) < 1)
                {
                  num99 += 45;
                  tsubpart12 =  new SEButtonPartClass(-1, "Replacement Troops", 42, 42, 33);
                  self.orderRepl2Id = self.AddSubPart( tsubpart12, num99 + num6, num100 + num7, 42, 42, 1);
                }
                else
                {
                  num99 += 45;
                  tsubpart12 =  new SEButtonPartClass(-1, "Militia cannot use your regular Replacement Troops. They have their own autonomous replacement system.", 42, 42, 33, true);
                  self.orderRepl2Id = self.AddSubPart( tsubpart12, num99 + num6, num100 + num7, 42, 42, 0);
                }
              }
              let mut num101: i32 = num99 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Transfer between Units in same Hex and Battlegroup formation", 42, 42, 12);
              self.orderBattlegroupId = self.AddSubPart( tsubpart12, num101 + num6, num100 + num7, 42, 42, 1);
            }
            else
            {
              tsubpart12 =  new SEButtonPartClassWithText("Unit Admin", -1, "Order of Battle", 132, 32, 1);
              self.orderChangeId = self.AddSubPart( tsubpart12, num99 + num6, num100 + num7, 132, 32, 1);
              if (self.orderShqNr != self.orderUnitNr)
              {
                if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.orderUnitNr].Historical].GiveHisVarValue(11) < 1)
                {
                  num100 += 34;
                  tsubpart12 =  new SEButtonPartClassWithText("Repl.Troops", -1, "Replacement Troops", 132, 32, 33);
                  self.orderRepl2Id = self.AddSubPart( tsubpart12, num99 + num6, num100 + num7, 132, 32, 1);
                }
                else
                {
                  num100 += 34;
                  tsubpart12 =  new SEButtonPartClassWithText("Repl.Troops", -1, "Militia cannot use your regular Replacement Troops. They have their own autonomous replacement system.", 132, 32, 33, true);
                  self.orderRepl2Id = self.AddSubPart( tsubpart12, num99 + num6, num100 + num7, 132, 32, 0);
                }
              }
              let mut num102: i32 = num100 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Transfer/BG", -1, "Transfer between Units in same Hex and Battlegroup formation", 132, 32, 12);
              self.orderBattlegroupId = self.AddSubPart( tsubpart12, num99 + num6, num102 + num7, 132, 32, 1);
            }
          }
          else
          {
            str: String = "No friendy Unit";
            if (str.Length > 15)
              str = Strings.Left(str, 15) + ".";
             let mut local31: &Graphics = &objgraphics;
            bitmap14: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
             let mut local32: &Bitmap = &bitmap14;
            let mut x13: i32 = num69 + num6;
            let mut y: i32 = num70 + num7;
            DrawMod.DrawSimple( local31,  local32, x13, y);
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, str, DrawMod.TGame.MarcFont7, num69 + num6 + 65, num70 + num7 + 12, DrawMod.TGame.seColGray);
            let mut num103: i32 = 1;
            let mut num104: i32 = num70 + 42;
            if (!flag1)
            {
              tsubpart12 =  new SEButtonPartClass(-1, "Change the selected Formation", 42, 42, 1, true);
              self.orderChangeId = self.AddSubPart( tsubpart12, num103 + num6, num104 + num7, 42, 42, 0);
              let mut num105: i32 = num103 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Produce Replacement Troops", 42, 42, 33, true);
              self.orderRepl2Id = self.AddSubPart( tsubpart12, num105 + num6, num104 + num7, 42, 42, 0);
              let mut num106: i32 = num105 + 45;
              tsubpart12 =  new SEButtonPartClass(-1, "Transfer between Units in same Hex and Battlegroup formation", 42, 42, 12, true);
              self.orderBattlegroupId2 = self.AddSubPart( tsubpart12, num106 + num6, num104 + num7, 42, 42, 0);
            }
            else
            {
              tsubpart12 =  new SEButtonPartClassWithText("Unit Admin", -1, "Change the selected Formation", 132, 32, 1, true);
              self.orderChangeId = self.AddSubPart( tsubpart12, num103 + num6, num104 + num7, 132, 32, 0);
              let mut num107: i32 = num104 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Repl.Troops", -1, "Produce Replacement Troops", 132, 32, 33, true);
              self.orderRepl2Id = self.AddSubPart( tsubpart12, num103 + num6, num107 + num7, 132, 32, 0);
              let mut num108: i32 = num107 + 34;
              tsubpart12 =  new SEButtonPartClassWithText("Transfer/BG", -1, "Transfer between Units in same Hex and Battlegroup formation", 132, 32, 12, true);
              self.orderBattlegroupId2 = self.AddSubPart( tsubpart12, num103 + num6, num108 + num7, 132, 32, 0);
            }
          }
        }
      }
      if (self.game.EditObj.rightSideBarMode == 1)
      {
        if (num1 != self.game.Data.Turn || index2 < 0)
          return;
        let mut x14: i32 = 45;
        let mut num109: i32 = 10;
        let mut width: i32 = 135;
        let mut height: i32 = 28;
        SimpleList simpleList = SimpleList::new();
        SimpleStringList simpleStringList = SimpleStringList::new();
        simpleList.Add(7, 0, 19);
        simpleStringList.Add("Food\r\nYour workers and soldiers need food in order not to starve.", 1);
        simpleList.Add(5, 0, 20);
        simpleStringList.Add("Water\r\nYour domed farms need water in order to produce food.", 1);
        simpleList.Add(1, 0, 18);
        simpleStringList.Add("Oil\r\nNeeded to keep your mechanized troops mobile.", 1);
        simpleList.Add(10, 0, 17);
        simpleStringList.Add("Ammo\r\nNeeded to keep your army fighting. Build up reserves before starting a war.", 1);
        simpleList.Add(2, 0);
        simpleStringList.Add("Metals\r\nBase resource needed to build almost anything. Includes iron, copper and nickel.", 1);
        simpleList.Add(8, 0, 22);
        simpleStringList.Add("Industrial Points\r\nKey to producing almost anything.", 1);
        simpleList.Add(15, 0, 16);
        simpleStringList.Add("Energy\r\nSome assets require energy in-order to run. Energy can be produced in power plants.", 1);
        simpleList.Add(4, 0);
        simpleStringList.Add("Radioactives\r\nSome models will require Radioactives for construction and/or ammunitions.", 1);
        simpleList.Add(9, 0);
        simpleStringList.Add("Recruits\r\nIn order to raise new troops you need recruits.", 1);
        simpleList.Add(12, 0);
        simpleStringList.Add("Colonists\r\nTo found a new city or increase populace of a zone you need colonists.", 1);
        simpleList.Add(3, 0);
        simpleStringList.Add("Rare Metals\r\nIncludes rare earth metals. For advanced production.", 1);
        simpleList.Add(13, 0);
        simpleStringList.Add("Machines\r\nFor construction of advanced equipment and assets.", 1);
        simpleList.Add(14, 0);
        simpleStringList.Add("Hi-Tech Parts\r\nFor construction of very advanced equipment and assets.", 1);
        let mut eventPic1: i32 = self.game.Data.FindEventPic("", 9, "SE_Present");
        let mut eventPic2: i32 = self.game.Data.FindEventPic("", 8, "SE_Present");
        let mut eventPic3: i32 = self.game.Data.FindEventPic("", 11, "SE_Present");
        str3: String = self.game.SelectX.ToString() + "," + self.game.SelectY.ToString();
        str4: String = self.game.Data.UnitObj[index2].Name;
        if (str4.Length > 15)
          str4 = Strings.Left(str4, 15) + ".";
         let mut local33: &Graphics = &objgraphics;
        bitmap15: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBARHEADER);
         let mut local34: &Bitmap = &bitmap15;
        let mut x15: i32 = x14;
        let mut y4: i32 = num109;
        DrawMod.DrawSimple( local33,  local34, x15, y4);
        DrawMod.DrawTextColouredConsoleCenter( objgraphics, str4, DrawMod.TGame.MarcFont7, x14 + 65, num109 + 12, DrawMod.TGame.seColGray);
        let mut y5: i32 = num109 + 42;
        let mut counter: i32 = simpleList.Counter;
        for (let mut index8: i32 = 0; index8 <= counter; index8 += 1)
        {
          let mut num110: i32 = self.game.Data.UnitObj[index2].items.list.FindWeight(simpleList.Id[index8]);
          let mut num111: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData2(0, self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index2].Historical].ID, 2, simpleList.Id[index8], 3)));
          let mut num112: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData2(0, self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index2].Historical].ID, 2, simpleList.Id[index8], 3)));
          if (num110 > 0 | num111 > 0 | num112 > 0 | self.game.ScreenHeight >= 920)
          {
            self.game.Data.StringListObj[stringListById4].GetData(0, simpleList.Id[index8], 2);
            str5: String = simpleStringList.Id[index8];
             let mut local35: &Graphics = &objgraphics;
            bitmap16: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_VARBOX);
             let mut local36: &Bitmap = &bitmap16;
            let mut x16: i32 = x14;
            let mut y6: i32 = y5;
            DrawMod.DrawSimple( local35,  local36, x16, y6);
            let mut eventPicSlotFor: i32 = self.game.EventRelatedObj.GetEventPicSlotFor(simpleList.Id[index8], "", "");
             let mut local37: &Graphics = &objgraphics;
            bitmap17: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPicSlotFor]);
             let mut local38: &Bitmap = &bitmap17;
            let mut x17: i32 = x14 + 2;
            let mut y7: i32 = y5 + 6;
            DrawMod.DrawSimple( local37,  local38, x17, y7);
            tstring4: String = num110.ToString();
            double num113;
            if (num110 > 9999)
            {
              num113 = Math.Round( num110 / 1000.0, 1);
              tstring4 = num113.ToString() + "k";
            }
            let mut num114: i32 = num112 - num111;
            ttitle: String = simpleStringList.Id[index8];
            num110 = 0;
            let mut num115: i32 = 0;
            let mut num116: i32 = 0;
            let mut num117: i32 = 0;
            let mut num118: i32 = 0;
            let mut num119: i32 = 0;
            let mut logCounter: i32 = self.game.Data.UnitObj[index2].LogCounter;
            for (let mut index9: i32 = 0; index9 <= logCounter; index9 += 1)
            {
              if (self.game.Data.UnitObj[index2].LogData1[index9] == simpleList.Id[index8])
              {
                if (self.game.Data.UnitObj[index2].LogType[index9] == 101)
                  num110 += self.game.Data.UnitObj[index2].LogData3[index9];
                if (self.game.Data.UnitObj[index2].LogType[index9] == 102)
                  num115 += self.game.Data.UnitObj[index2].LogData3[index9];
                if (self.game.Data.UnitObj[index2].LogType[index9] == 103)
                  num116 += self.game.Data.UnitObj[index2].LogData3[index9];
                if (self.game.Data.UnitObj[index2].LogType[index9] == 104)
                  num117 += self.game.Data.UnitObj[index2].LogData3[index9];
                if (self.game.Data.UnitObj[index2].LogType[index9] == 301)
                  num118 += self.game.Data.UnitObj[index2].LogData3[index9];
                if (self.game.Data.UnitObj[index2].LogType[index9] == 302)
                  num119 += self.game.Data.UnitObj[index2].LogData3[index9];
              }
            }
            if (num118 > 0)
              tstring4 += "*";
            DrawMod.DrawTextColouredConsole( objgraphics, tstring4, self.game.MarcFont16, x14 + 31, y5 + 4, self.game.seColGray);
            tstring5: String = Math.Abs(num114).ToString();
            if (num114 > 9999 | num114 < -9999)
            {
              num113 = Math.Round( Math.Abs(num114) / 1000.0, 1);
              tstring5 = num113.ToString() + "k";
            }
            if (num118 > 0)
              num114 = 0;
            if (num114 != 0)
              DrawMod.DrawTextColouredConsole( objgraphics, tstring5, self.game.MarcFont7, x14 + 91, y5 + 5, self.game.seColGray);
            if (num114 > 0)
            {
               let mut local39: &Graphics = &objgraphics;
              bitmap18: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPic2]);
               let mut local40: &Bitmap = &bitmap18;
              rectangle = Rectangle::new(0, 0, 32, 16);
              let mut srcrect4: &Rectangle = &rectangle
              trect1 = Rectangle::new(x14 + 76, y5 + 6, 32, 16);
              let mut destrect4: &Rectangle = &trect1
              DrawMod.DrawSimplePart2ColouredNew( local39,  local40, srcrect4, destrect4, 0.0f, 1.5f, 0.0f, 1f);
            }
            else if (num114 < 0)
            {
               let mut local41: &Graphics = &objgraphics;
              bitmap19: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPic1]);
               let mut local42: &Bitmap = &bitmap19;
              rectangle = Rectangle::new(0, 0, 32, 16);
              let mut srcrect5: &Rectangle = &rectangle
              trect1 = Rectangle::new(x14 + 76, y5 + 6, 32, 16);
              let mut destrect5: &Rectangle = &trect1
              DrawMod.DrawSimplePart2ColouredNew( local41,  local42, srcrect5, destrect5, 1.5f, 0.0f, 0.0f, 1f);
            }
            else
            {
               let mut local43: &Graphics = &objgraphics;
              bitmap20: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPic3]);
               let mut local44: &Bitmap = &bitmap20;
              rectangle = Rectangle::new(0, 0, 32, 16);
              let mut srcrect6: &Rectangle = &rectangle
              trect1 = Rectangle::new(x14 + 76, y5 + 6, 32, 16);
              let mut destrect6: &Rectangle = &trect1
              DrawMod.DrawSimplePart2ColouredNew( local43,  local44, srcrect6, destrect6, 0.0f, 1.8f, 1.8f, 1f);
            }
            str6: String = "Delivered from zones: " + num110.ToString() + "\r\n" + "Sent to zones: " + num115.ToString() + "\r\n" + "Sent to units: " + num116.ToString() + "\r\n" + "Consumed in SHQ: " + num117.ToString() + "\r\n" + "Consumed by colonists & recruits: " + num119.ToString() + "\r\n" + "Lost due to max storage reached: " + num118.ToString();
            ttext: String;
            if (simpleList.Data1[index8] > 0)
            {
              let mut weight: i32 = self.game.Data.UnitObj[index2].items.list.FindWeight(simpleList.Data1[index8]);
              ttext = str6 + "\r\nMaximum storage: " + weight.ToString();
            }
            else
              ttext = str6 + "\r\nMaximum storage: Unlimited";
            rectangle = Rectangle::new(x14, y5, width, height);
            trect1 = rectangle;
            self.AddMouse( trect1, ttitle, ttext);
            y5 += height;
            if (y5 + (220 + (height + 5) + 60) > self.game.ScreenHeight)
              break;
          }
        }
      }
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          if (Strings.InStr(self.game.EditObj.TipText, "MX-ENTR") <= 0)
            return;
          self.game.EditObj.TipTitle += "<FIXEDSYS>";
          return;
        }
      }
      if (self.SubPartCounter <= -1)
        return;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.game.EditObj.TipButton = false;
          self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (self.game.EditObj.TipButton)
            break;
          if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            break;
          }
        }
      }
    }

    pub fn PopUpRefresh() => self.DoRefresh();

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.TutOrder > -1)
        return windowReturnClass;
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1] & self.SubPartOverlay[index1] > 0)
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.rlistid)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 >= 0)
              {
                let mut layerLisOnlyAssetId: i32 = self.game.EditObj.layerLisOnlyAssetId;
                self.game.EditObj.layerLisOnlyAssetId = num2;
                let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
                {
                  let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                  for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                  {
                    let mut index4: i32 = 0;
                    do
                    {
                      self.game.Data.MapObj[0].HexObj[index2, index3].tempPreviewAssetLIS[index4] = 0;
                      index4 += 1;
                    }
                    while (index4 <= 8);
                  }
                }
                if (self.game.EditObj.layerLisOnlyAssetId > 0)
                {
                  self.game.ProcessingObj.LIS_SetNetwork(false, true, self.game.EditObj.layerLisOnlyAssetId);
                  let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 148, 0, 0));
                  let mut num3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(9, self.game.EditObj.layerLisOnlyAssetId, 0)));
                  let mut num4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(9, self.game.EditObj.layerLisOnlyAssetId, 3)));
                  let mut num5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(9, self.game.EditObj.layerLisOnlyAssetId, 4)));
                  self.game.SelectX = num4;
                  self.game.SelectY = num5;
                  self.game.HandyFunctionsObj.SetcornerXY2();
                  self.game.EditObj.TempCoordList = CoordList::new();
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                  self.game.EditObj.se1_SelectAssetButton = self.game.EditObj.layerLisOnlyAssetId;
                  if (self.game.EditObj.SetViewModeExtraNr == 3 && self.game.EditObj.layerLisOnlyAssetId > 0)
                    windowReturnClass.AddCommand(4, 69);
                  self.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (layerLisOnlyAssetId > 0)
                {
                  self.game.ProcessingObj.LIS_SetNetwork(false, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                  self.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            else
            {
              if (num1 == self.but1id)
              {
                self.game.EditObj.rightSideBarMode = 0;
                windowReturnClass.AddCommand(4, 68);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but1)
              {
                self.game.EditObj.udsUnitOrderMode = 0;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but6)
              {
                self.game.EditObj.udsUnitOrderMode = 53;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but2)
              {
                self.game.EditObj.udsUnitOrderMode = 1;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but7)
              {
                self.game.EditObj.udsUnitOrderMode = 48;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but8)
              {
                self.game.EditObj.udsUnitOrderMode = 54;
                self.game.EditObj.OrderSubType = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(self.game.SelectX, self.game.SelectY, "SE_Data", "Zones"));
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                windowReturnClass.AddCommand(1, 118);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but3)
              {
                self.game.EditObj.udsUnitOrderMode = 18;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but4)
              {
                self.game.EditObj.udsUnitOrderMode = 11;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but9)
              {
                self.game.EditObj.udsUnitOrderMode = 14;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but10)
              {
                self.game.EditObj.udsUnitOrderMode = 33;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but11)
              {
                self.game.EditObj.udsUnitOrderMode = 55;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  if (self.game.EditObj.UnitSelected > -1)
                    window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                  else
                    window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but5)
              {
                let mut enr: i32 =  Math.Round( self.game.Data.RuleVar[705]);
                self.game.EditObj.UDSpopupText = "";
                self.game.EditObj.UDSAddInput("ROADCHOICE", 0);
                if (enr > 0)
                  self.game.EventRelatedObj.DoCheckSpecificEvent(enr);
                if (self.game.EditObj.UDSpopupText.Length > 1)
                {
                  self.game.EditObj.UDSpushedPopupText = self.game.EditObj.UDSpopupText;
                  self.game.EditObj.UDSpopupText = "";
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.game.EditObj.udsUnitOrderMode = 36;
                ScreenClass screeny = self.formref.Screeny;
                System.Type type = typeof (MapWindowClass2);
                 System.Type local =  type;
                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                if (!Information.IsNothing( window))
                {
                  self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                  window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 67);
                  windowReturnClass.AddCommand(4, 68);
                  windowReturnClass.AddCommand(4, 9);
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerUnit)
              {
                self.game.EditObj.layerUnits = !self.game.EditObj.layerUnits;
                self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
                if (!self.game.EditObj.layerUnits)
                  self.game.EditObj.UnitSelected = -1;
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 69);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLabel)
              {
                self.game.EditObj.ShowLabel = !self.game.EditObj.ShowLabel;
                self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerDetail)
              {
                self.game.EditObj.HideAS = !self.game.EditObj.HideAS;
                self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerGrid)
              {
                self.game.EditObj.HexRasterOn = !self.game.EditObj.HexRasterOn;
                self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerColor)
              {
                self.game.EditObj.RegimeColoring = !self.game.EditObj.RegimeColoring;
                self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLisRange)
              {
                self.game.HandyFunctionsObj.RedimTempSup(9999);
                self.game.EditObj.ShowLISRange = !self.game.EditObj.ShowLISRange;
                if (self.game.EditObj.ShowLISRange)
                {
                  self.game.EditObj.ShowHQPower = false;
                  self.game.EditObj.ShowAirRange = false;
                }
                self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLog1)
              {
                self.game.EditObj.layerLisAvailable = !self.game.EditObj.layerLisAvailable;
                self.game.EditObj.layerLisUsed = false;
                self.game.EditObj.layerLisTotal = false;
                self.game.EditObj.layerLisBottlenecks = false;
                self.game.EditObj.layerLisPreview = false;
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLog2)
              {
                self.game.EditObj.layerLisAvailable = false;
                self.game.EditObj.layerLisUsed = !self.game.EditObj.layerLisUsed;
                self.game.EditObj.layerLisTotal = false;
                self.game.EditObj.layerLisBottlenecks = false;
                self.game.EditObj.layerLisPreview = false;
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLog3)
              {
                self.game.EditObj.layerLisAvailable = false;
                self.game.EditObj.layerLisUsed = false;
                self.game.EditObj.layerLisTotal = false;
                self.game.EditObj.layerLisBottlenecks = !self.game.EditObj.layerLisBottlenecks;
                self.game.EditObj.layerLisPreview = false;
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLog4)
              {
                self.game.EditObj.layerLisAvailable = false;
                self.game.EditObj.layerLisUsed = false;
                self.game.EditObj.layerLisTotal = !self.game.EditObj.layerLisTotal;
                self.game.EditObj.layerLisBottlenecks = false;
                self.game.EditObj.layerLisPreview = false;
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.layerLog5)
              {
                self.game.EditObj.layerLisAvailable = false;
                self.game.EditObj.layerLisUsed = false;
                self.game.EditObj.layerLisTotal = false;
                self.game.EditObj.layerLisBottlenecks = false;
                self.game.EditObj.layerLisPreview = !self.game.EditObj.layerLisPreview;
                self.game.EditObj.layerLisOnlyAssetId = -1;
                if (!self.game.EditObj.layerLisPreview)
                {
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
                    {
                      let mut index7: i32 = 0;
                      do
                      {
                        self.game.Data.MapObj[0].HexObj[index5, index6].tempPreviewLIS[index7] = 0;
                        self.game.Data.MapObj[0].HexObj[index5, index6].tempPreviewAssetLIS[index7] = 0;
                        index7 += 1;
                      }
                      while (index7 <= 8);
                    }
                  }
                }
                else
                  self.game.ProcessingObj.LIS_SetNetwork(false, true);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderAmmoId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("ZONE", self.orderShqNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 542, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderReplId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("SHQNR", self.orderShqNr);
                self.game.EventRelatedObj.SetUDSKey("UNR", -1);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 518, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderRepl2Id)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("UNR", self.orderUnitNr);
                self.game.EventRelatedObj.SetUDSKey("SHQNR", -1);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 518, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderBattlegroupId)
              {
                DrawMod.TGame.EditObj.PopupValue = 27;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderTransferId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 589, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderTradeId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("SHQ", self.orderShqNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 527, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderChangeShqId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("ZONE", self.orderZoneNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 571, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderProduceUnitId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("ZONE", self.orderZoneNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 521, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderColonizeId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("ZONE", self.orderZoneNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 551, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderNewAssetId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("ZONE", self.orderZoneNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 106, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderNewZoneId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("ZONE", self.orderZoneNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 508, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderMoreId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("UNIT", self.orderUnitNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 524, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.orderChangeId)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EditObj.UDSClearInput();
                self.game.EventRelatedObj.SetUDSKey("UNIT", self.orderUnitNr);
                let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 548, 0, 0);
                self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
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
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          switch (self.MouseData[index])
          {
            case 1:
            case 2:
            case 3:
            case 4:
              if (self.game.EditObj.rightSideBarMode == self.MouseData[index])
                self.game.EditObj.rightSideBarMode = 0;
              else
                self.game.EditObj.rightSideBarMode = self.MouseData[index];
              windowReturnClass.AddCommand(4, 68);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 101:
              if (self.currentShqNr > -1)
              {
                let mut x1: i32 = self.game.Data.UnitObj[self.currentShqNr].X;
                let mut y1: i32 = self.game.Data.UnitObj[self.currentShqNr].Y;
                self.game.Data.MapObj[0].HexObj[x1, y1].RemoveUnitFromList(self.currentShqNr);
                self.game.Data.MapObj[0].HexObj[x1, y1].AddUnitToList(self.currentShqNr);
                self.game.EditObj.TempCoordList = CoordList::new();
                self.game.EditObj.UnitSelected = self.currentShqNr;
                self.game.HandyFunctionsObj.CenterOnXY(self.game.Data.UnitObj[self.currentShqNr].X, self.game.Data.UnitObj[self.currentShqNr].Y);
                windowReturnClass.AddCommand(3, 11);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              continue;
            default:
              continue;
          }
        }
      }
      windowReturnClass.NoMouseClickBelow = self.game.EditObj.rightSideBarMode <= 0 ? x > self.OwnBitmap.Width - 30 : x >= 22;
      let mut mouseCounter2: i32 = self.MouseCounter;
      bool flag;
      for (let mut index: i32 = 0; index <= mouseCounter2; index += 1)
      {
        if (x > self.MouseRect[index].X - 16 & x < self.MouseRect[index].X + self.MouseRect[index].Width + 32 && y > self.MouseRect[index].Y - 16 & y < self.MouseRect[index].Y + self.MouseRect[index].Height + 32)
          flag = true;
      }
      if (flag)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false) => WindowReturnClass::new();
  }
}
