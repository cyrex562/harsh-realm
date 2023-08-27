// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UdsOrderWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class UdsOrderWindowClass : WindowClass
  {
    pub w: i32;
    pub h: i32;
    pub lastorderx: i32;
    pub lastordery: i32;
     exitId: i32;
     tab1: i32;
     tab2: i32;
     tab3: i32;
     tab4: i32;
     tab51: i32;
     tab52: i32;
     tab53: i32;
     tab54: i32;
     tab6: i32;
     currentview: i32;
     butCount: i32;
     int[] butId;
     butString: Vec<String>;
     int[] butEvent;
     butMouseOver: Vec<String>;
     int[] butSmallGfx;
     int[] butTempVar0;
     int[] butTempVar1;
     int[] butTempVarStringlistId;
     MouseOverWhichTab: i32;

    pub UdsOrderWindowClass(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect)
      : base( tGame, tGame.ScreenWidth, 90)
    {
      self.butId = new int[100];
      self.butString = new string[100];
      self.butEvent = new int[100];
      self.butMouseOver = new string[100];
      self.butSmallGfx = new int[100];
      self.butTempVar0 = new int[100];
      self.butTempVar1 = new int[100];
      self.butTempVarStringlistId = new int[100];
      self.NewGfx = true;
      self.w = tGame.ScreenWidth;
      self.h = 90;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      if (self.game.SelectX > -1 && self.game.EditObj.UnitSelected == -1 & self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1 && self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime))
        self.game.EditObj.UnitSelected = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
      self.game.EditObj.PurelyOrderRedrawRefresh = false;
      self.DoRefresh();
    }

    pub fn DoRefresh()
    {
      if (self.game.EditObj.OrderType == 0)
      {
        self.lastorderx = -1;
        self.lastordery = -1;
      }
      self.dostuff();
    }

    pub handleTimerWheel: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass.Flag = false;
      if (self.game.EditObj.MouseWheel > 0 & self.game.EditObj.Zoom < 1 & self.game.EditObj.TutOrder == -1)
      {
        if (self.game.EditObj.MouseOverX > -1 & self.game.EditObj.MouseOverY > -1)
          self.game.HandyFunctionsObj.CenterOnXY(self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY, true);
        self.game.EditObj.MouseWheel = 0;
        self.game.EditObj.MouseWheelWait = 4;
        return self.actionZoomIn();
      }
      if (self.game.EditObj.MouseWheel < 0 & self.game.EditObj.Zoom > -1 & self.game.EditObj.TutOrder == -1)
      {
        let mut num: i32 = self.game.EditObj.MouseOverX > -1 & self.game.EditObj.MouseOverY > -1 & self.game.Data.Product <= 5 ? 1 : 0;
        self.game.EditObj.MouseWheel = 0;
        self.game.EditObj.MouseWheelWait = 4;
        return self.actionZoomOut();
      }
      if (!self.game.EditObj.GuiDown || Information.IsNothing( self.game.EditObj.UDSpushedPopupText) || self.game.EditObj.UDSpushedPopupText.Length <= 1)
        return windowReturnClass;
      self.game.EditObj.UDSpopupText = self.game.EditObj.UDSpushedPopupText;
      self.game.EditObj.UDSpushedPopupText = "";
      self.game.EditObj.PopupValue = 21;
      windowReturnClass.AddCommand(5, 14);
      self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.MouseOverWhichTab <= 0 || self.MouseInThisWindow)
        return windowReturnClass;
      self.MouseOverWhichTab = 0;
      self.dostuff();
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn DoTabs( Graphics g)
    {
      SizeF sizeF1 = SizeF::new();
      self.tab1 = -1;
      self.tab2 = -1;
      self.tab3 = -1;
      self.tab4 = -1;
      self.tab51 = -1;
      self.tab52 = -1;
      self.tab53 = -1;
      self.tab54 = -1;
      self.tab6 = -1;
      if (self.game.EditObj.GuiDown)
        return;
      num1: i32;
      if (self.game.EditObj.UnitSelected == -1)
      {
        if (self.game.Data.ExtraTabName.Length <= 0)
          return;
        let mut width: i32 = 182;
        let mut num2: i32 = 52;
        let mut num3: i32 = 1;
        if (self.game.Data.ExtraTabName2.Length > 0)
        {
          width = 140;
          num2 = 132;
          num3 = 2;
        }
        if (self.game.Data.ExtraTabName3.Length > 0)
        {
          width = 112;
          num2 = 172;
          num3 = 3;
        }
        if (self.game.Data.ExtraTabName4.Length > 0)
        {
          width = 102;
          num2 = 130;
          num3 = 4;
        }
        let mut num4: i32 = num2 - width;
        if (self.game.EditObj.SetViewModeExtraNr == 0)
        {
          let mut x1: i32 =  Math.Round( num4 +  self.game.ScreenWidth / 2.0 - 370.0) + (width - 12);
          bitmap: Bitmap;
          SizeF sizeF2;
          Rectangle trect1;
          Rectangle trect2;
          if (self.game.Data.ExtraTabName4.Length > 0)
          {
            x1 -= width - 12;
             let mut local1: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local2: &Bitmap = &bitmap;
            let mut x2: i32 = x1;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local1,  local2, x2, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper: String = self.game.Data.ExtraTabName4.ToUpper();
            sizeF2 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x1 +  width / 2.0 -  sizeF2.Width / 2.0), 70, Color.White);
            trect1 = Rectangle::new(x1, 66, width, 24);
            trect2 = trect1;
            self.AddMouse( trect2, "", "Extra data sheet.", 54);
            self.tab54 = self.MouseCounter;
          }
          if (self.game.Data.ExtraTabName3.Length > 0)
          {
            x1 -= width - 12;
             let mut local3: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local4: &Bitmap = &bitmap;
            let mut x3: i32 = x1;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local3,  local4, x3, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper: String = self.game.Data.ExtraTabName3.ToUpper();
            sizeF2 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x1 +  width / 2.0 -  sizeF2.Width / 2.0), 70, Color.White);
            trect2 = Rectangle::new(x1, 66, width, 24);
            trect1 = trect2;
            self.AddMouse( trect1, "", "Extra data sheet.", 53);
            self.tab53 = self.MouseCounter;
          }
          if (self.game.Data.ExtraTabName2.Length > 0)
          {
            x1 -= width - 12;
             let mut local5: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local6: &Bitmap = &bitmap;
            let mut x4: i32 = x1;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local5,  local6, x4, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper: String = self.game.Data.ExtraTabName2.ToUpper();
            sizeF2 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x1 +  width / 2.0 -  sizeF2.Width / 2.0), 70, Color.White);
            trect2 = Rectangle::new(x1, 66, width, 24);
            trect1 = trect2;
            self.AddMouse( trect1, "", "Extra data sheet.", 52);
            self.tab52 = self.MouseCounter;
          }
          let mut x5: i32 = x1 - (width - 12);
           let mut local7: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
           let mut local8: &Bitmap = &bitmap;
          let mut x6: i32 = x5;
          let mut w1: i32 = width;
          DrawMod.DrawScaledColorized( local7,  local8, x6, 66, w1, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
          upper1: String = self.game.Data.ExtraTabName.ToUpper();
          sizeF2 = g.MeasureString(upper1, self.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, upper1, self.game.MarcFont16,  Math.Round( x5 +  width / 2.0 -  sizeF2.Width / 2.0), 70, Color.White);
          trect2 = Rectangle::new(x5, 66, width, 24);
          trect1 = trect2;
          self.AddMouse( trect1, "", "Extra data sheet.", 51);
          self.tab51 = self.MouseCounter;
        }
        else
        {
          let mut x7: i32 =  Math.Round( num4 +  self.game.ScreenWidth / 2.0 - 370.0) + (width - 12);
          bitmap: Bitmap;
          upper: String;
          SizeF sizeF3;
          Rectangle rectangle;
          if (self.game.Data.ExtraTabName4.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 4)
          {
            x7 -= width - 12;
             let mut local9: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local10: &Bitmap = &bitmap;
            let mut x8: i32 = x7;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local9,  local10, x8, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper = self.game.Data.ExtraTabName4.ToUpper();
            sizeF3 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x7 +  width / 2.0 -  sizeF3.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x7, 66, width, 24);
            let mut trect: &Rectangle = &rectangle
            self.AddMouse( trect, "", "Extra data sheet.", 54);
            self.tab54 = self.MouseCounter;
          }
          else if (self.game.Data.ExtraTabName3.Length > 0)
            x7 -= width - 12;
          if (self.game.Data.ExtraTabName3.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 3)
          {
            x7 -= width - 12;
             let mut local11: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local12: &Bitmap = &bitmap;
            let mut x9: i32 = x7;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local11,  local12, x9, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper = self.game.Data.ExtraTabName3.ToUpper();
            sizeF3 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x7 +  width / 2.0 -  sizeF3.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x7, 66, width, 24);
            let mut trect: &Rectangle = &rectangle
            self.AddMouse( trect, "", "Extra data sheet.", 53);
            self.tab53 = self.MouseCounter;
          }
          else if (self.game.Data.ExtraTabName3.Length > 0)
            x7 -= width - 12;
          if (self.game.Data.ExtraTabName2.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 2)
          {
            x7 -= width - 12;
             let mut local13: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local14: &Bitmap = &bitmap;
            let mut x10: i32 = x7;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local13,  local14, x10, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper = self.game.Data.ExtraTabName2.ToUpper();
            sizeF3 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x7 +  width / 2.0 -  sizeF3.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x7, 66, width, 24);
            let mut trect: &Rectangle = &rectangle
            self.AddMouse( trect, "", "Extra data sheet.", 52);
            self.tab52 = self.MouseCounter;
          }
          else if (self.game.Data.ExtraTabName2.Length > 0)
            x7 -= width - 12;
          if (self.game.Data.ExtraTabName.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 1)
          {
            let mut x11: i32 = x7 - (width - 12);
             let mut local15: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local16: &Bitmap = &bitmap;
            let mut x12: i32 = x11;
            let mut w: i32 = width;
            DrawMod.DrawScaledColorized( local15,  local16, x12, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper = self.game.Data.ExtraTabName.ToUpper();
            sizeF3 = g.MeasureString(upper, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x11 +  width / 2.0 -  sizeF3.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x11, 66, width, 24);
            let mut trect: &Rectangle = &rectangle
            self.AddMouse( trect, "", "Extra data sheet.", 51);
            self.tab51 = self.MouseCounter;
          }
          else if (self.game.Data.ExtraTabName.Length > 0)
            num1 = x7 - (width - 12);
          let mut x13: i32 =  Math.Round( num4 +  self.game.ScreenWidth / 2.0 - 370.0) + (width - 12);
          if (self.game.EditObj.SetViewModeExtraNr == 1)
          {
            upper = self.game.Data.ExtraTabName.ToUpper();
            x13 -= (width - 12) * (num3 - 0);
          }
          if (self.game.EditObj.SetViewModeExtraNr == 2)
          {
            upper = self.game.Data.ExtraTabName2.ToUpper();
            x13 -= (width - 12) * (num3 - 1);
          }
          if (self.game.EditObj.SetViewModeExtraNr == 3)
          {
            upper = self.game.Data.ExtraTabName3.ToUpper();
            x13 -= (width - 12) * (num3 - 2);
          }
          if (self.game.EditObj.SetViewModeExtraNr == 4)
          {
            upper = self.game.Data.ExtraTabName4.ToUpper();
            x13 -= (width - 12) * (num3 - 3);
          }
           let mut local17: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
           let mut local18: &Bitmap = &bitmap;
          let mut x14: i32 = x13;
          let mut w2: i32 = width;
          DrawMod.DrawScaledColorized( local17,  local18, x14, 66, w2, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
          sizeF3 = g.MeasureString(upper, self.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x13 +  width / 2.0 -  sizeF3.Width / 2.0), 70, Color.White);
          rectangle = Rectangle::new(x13, 66, width, 24);
          let mut trect3: &Rectangle = &rectangle
          self.AddMouse( trect3, "", "Extra data sheet.", 50 + self.game.EditObj.SetViewModeExtraNr);
          if (self.game.EditObj.SetViewModeExtraNr == 1)
            self.tab51 = self.MouseCounter;
          if (self.game.EditObj.SetViewModeExtraNr == 2)
            self.tab52 = self.MouseCounter;
          if (self.game.EditObj.SetViewModeExtraNr == 3)
            self.tab53 = self.MouseCounter;
          if (self.game.EditObj.SetViewModeExtraNr != 4)
            return;
          self.tab54 = self.MouseCounter;
        }
      }
      else
      {
        if (self.game.EditObj.UnitSelected <= -1)
          return;
        object obj =  true;
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical > -1)
        {
          if (Information.IsNothing( self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName))
            obj =  false;
          else if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName.Length < 1)
            obj =  false;
          if (!self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
            ;
        }
        else
          obj =  false;
        if (self.game.Data.ExtraTabName.Length > 0)
        {
          let mut width: i32 = 182;
          let mut num5: i32 = 52;
          let mut num6: i32 = 1;
          if (self.game.Data.ExtraTabName2.Length > 0)
          {
            width = 140;
            num5 = 132;
            num6 = 2;
          }
          if (self.game.Data.ExtraTabName3.Length > 0)
          {
            width = 112;
            num5 = 172;
            num6 = 3;
          }
          if (self.game.Data.ExtraTabName4.Length > 0)
          {
            width = 102;
            num5 = 228;
            num6 = 4;
          }
          if (self.game.EditObj.SetViewModeExtraNr == 0)
          {
            let mut x15: i32 =  Math.Round( num5 +  self.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            bitmap: Bitmap;
            SizeF sizeF4;
            Rectangle rectangle;
            if (self.game.Data.ExtraTabName4.Length > 0)
            {
              x15 -= width - 12;
               let mut local19: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local20: &Bitmap = &bitmap;
              let mut x16: i32 = x15;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local19,  local20, x16, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName4.ToUpper();
              sizeF4 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x15 +  width / 2.0 -  sizeF4.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x15, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 54);
              self.tab54 = self.MouseCounter;
            }
            if (self.game.Data.ExtraTabName3.Length > 0)
            {
              x15 -= width - 12;
               let mut local21: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local22: &Bitmap = &bitmap;
              let mut x17: i32 = x15;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local21,  local22, x17, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName3.ToUpper();
              sizeF4 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x15 +  width / 2.0 -  sizeF4.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x15, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 53);
              self.tab53 = self.MouseCounter;
            }
            if (self.game.Data.ExtraTabName2.Length > 0)
            {
              x15 -= width - 12;
               let mut local23: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local24: &Bitmap = &bitmap;
              let mut x18: i32 = x15;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local23,  local24, x18, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName2.ToUpper();
              sizeF4 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x15 +  width / 2.0 -  sizeF4.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x15, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 52);
              self.tab52 = self.MouseCounter;
            }
            let mut x19: i32 = x15 - (width - 12);
             let mut local25: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local26: &Bitmap = &bitmap;
            let mut x20: i32 = x19;
            let mut w3: i32 = width;
            DrawMod.DrawScaledColorized( local25,  local26, x20, 66, w3, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper2: String = self.game.Data.ExtraTabName.ToUpper();
            sizeF4 = g.MeasureString(upper2, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper2, self.game.MarcFont16,  Math.Round( x19 +  width / 2.0 -  sizeF4.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x19, 66, width, 24);
            let mut trect4: &Rectangle = &rectangle
            self.AddMouse( trect4, "", "Extra data sheet.", 51);
            self.tab51 = self.MouseCounter;
            let mut x21: i32 = x19 - (width - 12);
             let mut local27: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local28: &Bitmap = &bitmap;
            let mut x22: i32 = x21;
            let mut w4: i32 = width;
            DrawMod.DrawScaledColorized( local27,  local28, x22, 66, w4, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            str: String = "UNIT INFO";
            sizeF4 = g.MeasureString(str, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16,  Math.Round( x21 +  width / 2.0 -  sizeF4.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x21, 66, width, 24);
            let mut trect5: &Rectangle = &rectangle
            self.AddMouse( trect5, "", "The base info of the unit is always shown.");
          }
          else
          {
            let mut x23: i32 =  Math.Round( num5 +  self.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            bitmap: Bitmap;
            SizeF sizeF5;
            Rectangle rectangle;
            if (self.game.Data.ExtraTabName4.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 4)
            {
              x23 -= width - 12;
               let mut local29: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local30: &Bitmap = &bitmap;
              let mut x24: i32 = x23;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local29,  local30, x24, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName4.ToUpper();
              sizeF5 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x23 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x23, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 54);
              self.tab54 = self.MouseCounter;
            }
            else if (self.game.Data.ExtraTabName3.Length > 0)
              x23 -= width - 12;
            if (self.game.Data.ExtraTabName3.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 3)
            {
              x23 -= width - 12;
               let mut local31: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local32: &Bitmap = &bitmap;
              let mut x25: i32 = x23;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local31,  local32, x25, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName3.ToUpper();
              sizeF5 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x23 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x23, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 53);
              self.tab53 = self.MouseCounter;
            }
            else if (self.game.Data.ExtraTabName3.Length > 0)
              x23 -= width - 12;
            if (self.game.Data.ExtraTabName2.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 2)
            {
              x23 -= width - 12;
               let mut local33: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local34: &Bitmap = &bitmap;
              let mut x26: i32 = x23;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local33,  local34, x26, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName2.ToUpper();
              sizeF5 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x23 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x23, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 52);
              self.tab52 = self.MouseCounter;
            }
            else if (self.game.Data.ExtraTabName2.Length > 0)
              x23 -= width - 12;
            if (self.game.Data.ExtraTabName.Length > 0 & self.game.EditObj.SetViewModeExtraNr != 1)
            {
              let mut x27: i32 = x23 - (width - 12);
               let mut local35: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
               let mut local36: &Bitmap = &bitmap;
              let mut x28: i32 = x27;
              let mut w: i32 = width;
              DrawMod.DrawScaledColorized( local35,  local36, x28, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = self.game.Data.ExtraTabName.ToUpper();
              sizeF5 = g.MeasureString(upper, self.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, self.game.MarcFont16,  Math.Round( x27 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x27, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              self.AddMouse( trect, "", "Extra data sheet.", 51);
              self.tab51 = self.MouseCounter;
            }
            else if (self.game.Data.ExtraTabName.Length > 0)
              num1 = x23 - (width - 12);
            let mut num7: i32 =  Math.Round( num5 +  self.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            if (self.game.Data.ExtraTabName4.Length > 0)
              num7 -= width - 12;
            if (self.game.Data.ExtraTabName3.Length > 0)
              num7 -= width - 12;
            if (self.game.Data.ExtraTabName2.Length > 0)
              num7 -= width - 12;
            if (self.game.Data.ExtraTabName.Length > 0)
              num7 -= width - 12;
            let mut x29: i32 = num7 - (width - 12);
             let mut local37: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local38: &Bitmap = &bitmap;
            let mut x30: i32 = x29;
            let mut w5: i32 = width;
            DrawMod.DrawScaledColorized( local37,  local38, x30, 66, w5, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            str: String = "UNIT INFO";
            sizeF5 = g.MeasureString(str, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16,  Math.Round( x29 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x29, 66, width, 24);
            let mut trect6: &Rectangle = &rectangle
            self.AddMouse( trect6, "", "The base info of the unit is always shown.", 6);
            self.tab6 = self.MouseCounter;
            let mut x31: i32 =  Math.Round( num5 +  self.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            if (self.game.EditObj.SetViewModeExtraNr == 1)
            {
              str = self.game.Data.ExtraTabName.ToUpper();
              x31 -= (width - 12) * (num6 - 0);
            }
            if (self.game.EditObj.SetViewModeExtraNr == 2)
            {
              str = self.game.Data.ExtraTabName2.ToUpper();
              x31 -= (width - 12) * (num6 - 1);
            }
            if (self.game.EditObj.SetViewModeExtraNr == 3)
            {
              str = self.game.Data.ExtraTabName3.ToUpper();
              x31 -= (width - 12) * (num6 - 2);
            }
            if (self.game.EditObj.SetViewModeExtraNr == 4)
            {
              str = self.game.Data.ExtraTabName4.ToUpper();
              x31 -= (width - 12) * (num6 - 3);
            }
             let mut local39: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
             let mut local40: &Bitmap = &bitmap;
            let mut x32: i32 = x31;
            let mut w6: i32 = width;
            DrawMod.DrawScaledColorized( local39,  local40, x32, 66, w6, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            sizeF5 = g.MeasureString(str, self.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16,  Math.Round( x31 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x31, 66, width, 24);
            let mut trect7: &Rectangle = &rectangle
            self.AddMouse( trect7, "", "Extra data sheet.", 50 + self.game.EditObj.SetViewModeExtraNr);
            if (self.game.EditObj.SetViewModeExtraNr == 1)
              self.tab51 = self.MouseCounter;
            if (self.game.EditObj.SetViewModeExtraNr == 2)
              self.tab52 = self.MouseCounter;
            if (self.game.EditObj.SetViewModeExtraNr == 3)
              self.tab53 = self.MouseCounter;
            if (self.game.EditObj.SetViewModeExtraNr == 4)
              self.tab54 = self.MouseCounter;
          }
        }
        else
        {
          let mut x33: i32 =  Math.Round( self.game.ScreenWidth / 2.0 - 480.0);
           let mut local41: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
           let mut local42: &Bitmap = &bitmap;
          let mut x34: i32 = x33;
          DrawMod.DrawSimple( local41,  local42, x34, 66);
          str: String = "UNIT BASE INFO";
          SizeF sizeF6 = g.MeasureString(str, self.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16,  Math.Round( ( (x33 + 91) - sizeF6.Width / 2f)), 70, Color.White);
          Rectangle trect = Rectangle::new(x33, 66, BitmapStore.GetWidth(self.game.MARCLARGETAB), 24);
          self.AddMouse( trect, "", "The base info of the unit is always shown.");
        }
        if (self.game.EditObj.SetViewModeExtraNr != 0)
          return;
        bool flag;
        if (self.game.EditObj.OrderType == 14)
          flag = true;
        if (self.game.EditObj.OrderType == 33)
          flag = true;
        if (self.game.EditObj.OrderType == 15)
          flag = true;
        if (self.game.EditObj.OrderType == 2)
          flag = true;
        if (self.game.EditObj.OrderType == 12)
          flag = true;
        if (self.game.EditObj.OrderType == 11)
          flag = true;
        if (self.game.EditObj.OrderType == 13)
          flag = true;
        if (self.currentview == 2 & !flag)
        {
          self.currentview = 0;
          self.game.EditObj.SetViewMode = 0;
        }
        if (Conversions.ToBoolean(Operators.AndObject( (self.currentview == 3), Operators.OrObject( !self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ, Operators.CompareObjectEqual(obj,  false, false)))))
        {
          self.currentview = 0;
          self.game.EditObj.SetViewMode = 0;
          self.game.EditObj.SetViewMode3 = false;
        }
        if (Conversions.ToBoolean(Operators.AndObject( (self.currentview == 0), Operators.AndObject( self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ, obj))) && !self.game.EditObj.SetViewMode3)
        {
          self.currentview = 3;
          self.game.EditObj.SetViewMode = 3;
          self.game.EditObj.SetViewMode3 = true;
        }
        if (Conversions.ToBoolean(Operators.AndObject( self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ, Operators.CompareObjectEqual(obj,  true, false))))
        {
          if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.Turn))
          {
            if (self.currentview == 3)
            {
              self.DoTabs2B( g);
              self.DoTabs1B( g);
              self.DoTabs4( g, true);
            }
            else if (self.currentview == 0)
            {
              self.DoTabs2B( g);
              self.DoTabs4( g);
              self.DoTabs1B( g, true);
            }
            else if (self.currentview == 1)
            {
              self.DoTabs4( g);
              self.DoTabs1B( g);
              self.DoTabs2B( g, true);
            }
            else
            {
              if (self.currentview != 2)
                return;
              self.DoTabs4( g);
              if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.Turn))
                self.DoTabs1B( g);
              self.DoTabs2B( g, true);
            }
          }
          else if (self.currentview != 3)
          {
            self.DoTabs4( g);
            self.DoTabs1B( g, true);
          }
          else
          {
            self.DoTabs1B( g);
            self.DoTabs4( g, true);
          }
        }
        else if (self.currentview == 0)
        {
          if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.Turn))
            self.DoTabs2( g);
          self.DoTabs1( g, true);
        }
        else if (self.currentview == 1)
        {
          self.DoTabs1( g);
          if (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.Turn))
            return;
          self.DoTabs2( g, true);
        }
        else
        {
          if (self.currentview != 2)
            return;
          self.DoTabs1( g);
          if (!self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.Turn))
            return;
          self.DoTabs2( g);
        }
      }
    }

    pub Rectangle DrawOneTab(
      Graphics g,
      bool wideTab,
      bool active,
      tx: i32,
      sHeader: String,
      sText: String,
      spriteSlot: i32,
      iconSlot: i32,
      let mut smallNumber: i32 = -1,
      bool grayedOut = false,
      let mut textOffsetX: i32 = 0,
      let mut spriteOffsetY: i32 = 0,
      bool tMousingOverNow = false)
    {
      let mut y1: i32 = 24;
      bitmap: Bitmap;
      if (tMousingOverNow)
      {
        if (active & wideTab)
        {
           let mut local1: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1HIGH);
           let mut local2: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y2: i32 = y1;
          DrawMod.Draw( local1,  local2, x, y2, 0.05f, 0.05f, 0.05f, 1f);
        }
        if (!active & wideTab)
        {
           let mut local3: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1LOW);
           let mut local4: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y3: i32 = y1;
          DrawMod.Draw( local3,  local4, x, y3, 0.05f, 0.05f, 0.05f, 1f);
        }
        if (active & !wideTab)
        {
           let mut local5: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB2HIGH);
           let mut local6: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y4: i32 = y1;
          DrawMod.Draw( local5,  local6, x, y4, 0.05f, 0.05f, 0.05f, 1f);
        }
        if (!active & !wideTab)
        {
           let mut local7: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB2LOW);
           let mut local8: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y5: i32 = y1;
          DrawMod.Draw( local7,  local8, x, y5, 0.05f, 0.05f, 0.05f, 1f);
        }
      }
      else
      {
        if (active & wideTab)
        {
           let mut local9: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1HIGH);
           let mut local10: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y6: i32 = y1;
          DrawMod.DrawSimple( local9,  local10, x, y6);
        }
        if (!active & wideTab)
        {
           let mut local11: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1LOW);
           let mut local12: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y7: i32 = y1;
          DrawMod.DrawSimple( local11,  local12, x, y7);
        }
        if (active & !wideTab)
        {
           let mut local13: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB2HIGH);
           let mut local14: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y8: i32 = y1;
          DrawMod.DrawSimple( local13,  local14, x, y8);
        }
        if (!active & !wideTab)
        {
           let mut local15: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB2LOW);
           let mut local16: &Bitmap = &bitmap;
          let mut x: i32 = tx;
          let mut y9: i32 = y1;
          DrawMod.DrawSimple( local15,  local16, x, y9);
        }
      }
      if (wideTab)
      {
        if (spriteSlot > 0)
        {
          if (active)
          {
             let mut local17: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(spriteSlot);
             let mut local18: &Bitmap = &bitmap;
            let mut x: i32 = tx + 3;
            let mut y10: i32 = y1 + 4 + spriteOffsetY;
            DrawMod.DrawSimple( local17,  local18, x, y10);
          }
          if (!active)
          {
             let mut local19: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(spriteSlot);
             let mut local20: &Bitmap = &bitmap;
            let mut x: i32 = tx + 3;
            let mut y11: i32 = y1 + 11 + spriteOffsetY;
            DrawMod.DrawSimple( local19,  local20, x, y11);
          }
        }
        else if (iconSlot > -1 && !grayedOut)
        {
          Rectangle rectangle1;
          Rectangle rectangle2;
          if (active)
          {
             let mut local21: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local22: &Bitmap = &bitmap;
            rectangle1 = Rectangle::new(iconSlot * 42, 32, 42, 32);
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(tx + 4, y1 + 11, 42, 32);
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local21,  local22, srcrect, destrect);
          }
          if (!active)
          {
             let mut local23: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local24: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 0, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 4, y1 + 18, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local23,  local24, srcrect, destrect);
          }
        }
      }
      SizeF sizeF = g.MeasureString(sText, DrawMod.TGame.MarcFont16);
      c1: Color;
      c2: Color;
      if (active)
      {
        c1 = self.game.seColWhite;
        c2 = self.game.seColGray;
      }
      if (!active)
      {
        c1 = self.game.seColGray;
        c2 = self.game.seColGray;
      }
      if (grayedOut)
      {
        c1 = Color.FromArgb( byte.MaxValue, 128, 128, 128);
        c2 = Color.FromArgb( byte.MaxValue, 128, 128, 128);
      }
      if (active)
      {
        if (wideTab)
        {
          if ( sizeF.Width > 150.0)
          {
            strArray: Vec<String> = sText.Split(new char[1]
            {
              ' '
            }, StringSplitOptions.RemoveEmptyEntries);
            sHeader = "";
            let mut num1: i32 = -1;
            let mut upperBound1: i32 = strArray.GetUpperBound(0);
            for (let mut index: i32 = 0; index <= upperBound1 &&  g.MeasureString(sHeader + " " + strArray[index], DrawMod.TGame.MarcFont16).Width < 150.0; index += 1)
            {
              if (sHeader.Length > 0)
                sHeader += " ";
              sHeader += strArray[index];
              num1 = index;
            }
            sText = "";
            let mut num2: i32 = num1 + 1;
            let mut upperBound2: i32 = strArray.GetUpperBound(0);
            for (let mut index: i32 = num2; index <= upperBound2; index += 1)
            {
              if ( g.MeasureString(sText + " " + strArray[index], DrawMod.TGame.MarcFont16).Width < 150.0)
              {
                if (sText.Length > 0)
                  sText += " ";
                sText += strArray[index];
              }
              else
              {
                sText += "..";
                break;
              }
            }
            DrawMod.DrawTextColouredConsole( g, sHeader, self.game.MarcFont16, tx + 44 + textOffsetX, y1 + 10, c1);
            DrawMod.DrawTextColouredConsole( g, sText, self.game.MarcFont16, tx + 44 + textOffsetX, y1 + 26, c1);
          }
          else if (sHeader.Length > 0)
          {
            DrawMod.DrawTextColouredConsole( g, sHeader, self.game.MarcFont5, tx + 44 + textOffsetX, y1 + 13, c2);
            DrawMod.DrawTextColouredConsole( g, sText, self.game.MarcFont16, tx + 44 + textOffsetX, y1 + 24, c1);
          }
          else
            DrawMod.DrawTextColouredConsole( g, sText, self.game.MarcFont16, tx + 44 + textOffsetX, y1 + 18, c1);
        }
        else
          DrawMod.DrawTextColouredConsoleCenter( g, sText, self.game.MarcFont16, tx + 37, y1 + 18, c1);
      }
      else if (wideTab)
      {
        if ( sizeF.Width > 150.0)
        {
          strArray: Vec<String> = sText.Split(new char[1]{ ' ' }, StringSplitOptions.RemoveEmptyEntries);
          sText = "";
          let mut num: i32 = -1;
          let mut upperBound: i32 = strArray.GetUpperBound(0);
          for (let mut index: i32 = 0; index <= upperBound; index += 1)
          {
            if ( g.MeasureString(sText + " " + strArray[index], DrawMod.TGame.MarcFont16).Width < 141.0)
            {
              if (sText.Length > 0)
                sText += " ";
              sText += strArray[index];
              num = index;
            }
            else
            {
              sText += "..";
              break;
            }
          }
        }
        DrawMod.DrawTextColouredConsole( g, sText, self.game.MarcFont16, tx + 44 + textOffsetX, y1 + 25, c1);
      }
      else
        DrawMod.DrawTextColouredConsoleCenter( g, sText, self.game.MarcFont16, tx + 37, y1 + 25, c1);
      Rectangle rectangle = Rectangle::new(tx, y1, 200, 50);
      if (!wideTab)
        rectangle = Rectangle::new(tx, y1, 75, 50);
      return rectangle;
    }

    pub fn dostuff()
    {
      self.currentview = self.game.EditObj.SetViewMode;
      self.ClearMouse();
      if (self.exitId > 0)
      {
        self.RemoveSubPart(self.exitId);
        self.exitId = 0;
      }
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      bool flag = false;
      if (self.game.EditObj.leftSideBarMode > 0)
        flag = true;
      bitmap: Bitmap;
      Rectangle trect1;
      Rectangle rectangle;
      if (flag)
      {
        if (self.game.EditObj.leftSideBarMode == 1 & self.game.ScreenWidth >= 1435)
        {
           let mut local1: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER2);
           let mut local2: &Bitmap = &bitmap;
          trect1 = Rectangle::new(0, 0, 145, 90);
          let mut srcrect: &Rectangle = &trect1
          rectangle = Rectangle::new(0, 0, 145, 90);
          let mut destrect: &Rectangle = &rectangle
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
        }
        else
        {
           let mut local3: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_TEXTURE);
           let mut local4: &Bitmap = &bitmap;
          rectangle = Rectangle::new(35, 0, 145, 90);
          let mut srcrect: &Rectangle = &rectangle
          trect1 = Rectangle::new(0, 0, 145, 90);
          let mut destrect: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
        }
         let mut local5: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_LEFT);
         let mut local6: &Bitmap = &bitmap;
        rectangle = Rectangle::new(0, 148, 40, self.h);
        let mut srcrect1: &Rectangle = &rectangle
        trect1 = Rectangle::new(145, 0, 40, self.h);
        let mut destrect1: &Rectangle = &trect1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect1, destrect1);
      }
       let mut local7: &Graphics = &objgraphics;
      bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_MIDDLE);
       let mut local8: &Bitmap = &bitmap;
      rectangle = Rectangle::new(10, 0, 60, 32);
      let mut srcrect2: &Rectangle = &rectangle
      trect1 = Rectangle::new(120, 58, self.w - 595, 32);
      let mut destrect2: &Rectangle = &trect1
      DrawMod.DrawSimplePart2( local7,  local8, srcrect2, destrect2);
      self.dostuff2(objgraphics);
      let mut num1: i32 =  Math.Round( self.game.ScreenWidth / 2.0 - 158.0);
      if (self.game.SelectX > -1 & self.game.SelectY > -1)
      {
        let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 370, 0, 0));
        let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
        let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 125, 0, 0));
        let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 382, 0, 0));
        let mut stringListById5: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 288, 0, 0));
        let mut num2: i32 = self.game.Data.StringListObj[stringListById3].Length + 1;
        let mut idValue: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType;
        let mut num3: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].SpriteNr;
        data1: DataClass = DrawMod.TGame.Data;
        str1: String = "Zones";
         local9: String =  str1;
        let mut libVar: i32 = data1.FindLibVar( local9, "SE_Data");
        let mut num4: i32 = 0;
        let mut hexLibVarValue1: i32 = DrawMod.TGame.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].GetHexLibVarValue(libVar);
        if (self.game.Data.FOWOn & self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].MaxRecon < 1)
        {
          if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].get_LastLT(self.game.Data.Turn) == -1)
          {
            idValue = -1;
          }
          else
          {
            idValue = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].get_LastLT(self.game.Data.Turn);
            num3 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].get_LastSpr(self.game.Data.Turn);
          }
        }
        if (hexLibVarValue1 > 0)
          num4 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, hexLibVarValue1, 13)));
        let mut num5: i32 =  Math.Round( num4 /  num2);
        eventPicOrigSlot1: i32;
        eventPicOrigSlot2: i32;
        if (stringListById1 > -1)
        {
          eventPicOrigSlot1 = num5 >= 50 ? (num5 >= 500 ?  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, idValue, 3))) :  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, idValue, 2)))) :  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, idValue, 1)));
          eventPicOrigSlot2 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, idValue, 6)));
        }
        if (idValue == -1)
        {
          eventPicOrigSlot2 = 61;
          eventPicOrigSlot1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, 0, 1)));
        }
        let mut eventPic1: i32 = self.game.Data.FindEventPic(eventPicOrigSlot1, "SE_Present");
        let mut x1: i32 = self.w - 274;
        let mut y1: i32 = 6;
        num6: i32;
        num7: i32;
        if (eventPic1 > -1)
        {
          let mut nr: i32 = self.game.Data.EventPicNr[eventPic1];
          num6 = 256;
          num7 = 80;
           let mut local10: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(nr);
           let mut local11: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 256, 80);
          let mut srcrect3: &Rectangle = &rectangle
          trect1 = Rectangle::new(x1, y1, 256, 80);
          let mut destrect3: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local10,  local11, srcrect3, destrect3);
        }
        let mut eventPic2: i32 = self.game.Data.FindEventPic(eventPicOrigSlot2, "SE_Present");
        if (eventPic2 > -1)
        {
          let mut nr: i32 = self.game.Data.EventPicNr[eventPic2];
          num6 = 256;
          num7 = 80;
           let mut local12: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(nr);
           let mut local13: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 256, 80);
          let mut srcrect4: &Rectangle = &rectangle
          trect1 = Rectangle::new(x1, y1, 256, 80);
          let mut destrect4: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local12,  local13, srcrect4, destrect4);
        }
         let mut local14: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_RIGHT2);
         let mut local15: &Bitmap = &bitmap;
        let mut x2: i32 = self.w - 475;
        DrawMod.DrawSimple( local14,  local15, x2, 0);
        if (idValue > -1 & num3 > -1)
        {
          name: String = self.game.Data.LandscapeTypeObj[idValue].Name;
          data2: DataClass = self.game.Data;
          str2: String = "hexName";
           local16: String =  str2;
          let mut hexLibVarValue2: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].GetHexLibVarValue(data2.FindLibVar( local16, "SE_Data"));
          str3: String = "";
          if (hexLibVarValue2 > 0)
            str3 = self.game.Data.StringListObj[stringListById4].GetData(0, hexLibVarValue2, 1);
          if (idValue == 43)
            str3 = "";
          let mut integer: i32 = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(self.game.SelectX, self.game.SelectY, "SE_Data", "Zones"));
          str4: String = "";
          let mut num8: i32 = 0;
          if (integer > 0)
          {
            str4 = self.game.Data.StringListObj[stringListById2].GetData(0, integer, 7);
            num8 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, integer, 2, "recon", 3)));
          }
          if (num8 < 1 & self.game.Data.FOWOn)
            str4 = "Unknown";
          str5: String = "";
          if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location > -1)
            str5 = self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location].Name + " ";
          str6: String;
          if (Operators.CompareString(Strings.Trim(Strings.LCase(str4)), Strings.Trim(Strings.LCase(str5)), false) == 0 & str4.Length > 0)
            str6 = str4 + " ";
          else if (str5.Length > 0)
            str6 = str5 + " ";
          else if (str3.Length > 0)
            str6 = str3 + " ";
          tstring: String = str6 + "(" + Strings.Trim(Conversion.Str( self.game.SelectX)) + "," + Strings.Trim(Conversion.Str( self.game.SelectY)) + ")";
          let mut x3: i32 = self.w - 475 + 103;
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, name, self.game.MarcFont16, x3, 35, self.game.seColGray);
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring, self.game.MarcFont4, x3, 52, self.game.seColGray);
          ttext: String = self.game.HandyFunctionsObj.GetLandscapeMouseOverText();
          if (str3.Length > 0)
            ttext = "Area Name: " + str3 + "\r\n" + ttext;
          if (str4.Length > 0)
            ttext = "Zone Name: " + str4 + "\r\n" + ttext;
          ttitle: String = name + " (" + Strings.Trim(Conversion.Str( self.game.SelectX)) + "," + Strings.Trim(Conversion.Str( self.game.SelectY)) + ")" + "<FIXEDSYS>";
          rectangle = Rectangle::new(x3 - 103, 18, 360, 65);
          trect1 = rectangle;
          self.AddMouse( trect1, ttitle, ttext);
          str7: String = "";
          regime: i32;
          if (self.game.EditObj.OrderType == 26)
          {
            if (self.game.EditObj.HisOwner[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY] > -1)
            {
              str7 = self.game.Data.RegimeObj[self.game.EditObj.HisOwner[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY]].Name;
              regime = self.game.EditObj.HisOwner[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY];
            }
          }
          else if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime > -1)
          {
            str7 = self.game.EventRelatedObj.Helper_GetOwnerNameForGuiDisplay(self.game.SelectX, self.game.SelectY);
            regime = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime;
          }
          let mut num9: i32 = self.w - 67;
          let mut y2: i32 = 13;
          let mut num10: i32 = 54;
          let mut height: i32 = 68;
          if (Operators.CompareString(str7.ToLower(), "unknown", false) != 0)
          {
            let mut bannerSpriteNr: i32 = self.game.Data.RegimeObj[regime].BannerSpriteNr;
             let mut local17: &Graphics = &objgraphics;
            bitmap = BitmapStore.GetBitmap(bannerSpriteNr);
             let mut local18: &Bitmap = &bitmap;
            let mut x4: i32 = num9;
            let mut y3: i32 = y2;
            let mut w1: i32 = num10;
            let mut h1: i32 = height;
            double r1 =  ( self.game.Data.RegimeObj[regime].Red /  byte.MaxValue);
            double g1 =  ( self.game.Data.RegimeObj[regime].Green /  byte.MaxValue);
            double b1 =  ( self.game.Data.RegimeObj[regime].Blue /  byte.MaxValue);
            DrawMod.DrawScaledColorized2( local17,  local18, x4, y3, w1, h1, 124, 210,  r1,  g1,  b1, 1f);
            let mut bannerSpriteNr2: i32 = self.game.Data.RegimeObj[regime].BannerSpriteNr2;
            if (bannerSpriteNr2 > 0)
            {
               let mut local19: &Graphics = &objgraphics;
              bitmap = BitmapStore.GetBitmap(bannerSpriteNr2);
               let mut local20: &Bitmap = &bitmap;
              let mut x5: i32 = num9;
              let mut y4: i32 = y2;
              let mut w2: i32 = num10;
              let mut h2: i32 = height;
              double r2 =  ( self.game.Data.RegimeObj[regime].Red2 /  byte.MaxValue);
              double g2 =  ( self.game.Data.RegimeObj[regime].Green2 /  byte.MaxValue);
              double b2 =  ( self.game.Data.RegimeObj[regime].Blue2 /  byte.MaxValue);
              DrawMod.DrawScaledColorized2( local19,  local20, x5, y4, w2, h2, 124, 210,  r2,  g2,  b2, 1f);
            }
            let mut hqSpriteNr2: i32 = self.game.Data.RegimeObj[regime].HQSpriteNr2;
            if (hqSpriteNr2 > 0)
            {
               let mut local21: &Graphics = &objgraphics;
              bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
               let mut local22: &Bitmap = &bitmap;
              let mut x6: i32 = num9 + 4;
              let mut y5: i32 = y2 + 18;
              double r3 =  ( self.game.Data.RegimeObj[regime].Red3 /  byte.MaxValue) - 1.0;
              double g3 =  ( self.game.Data.RegimeObj[regime].Green3 /  byte.MaxValue) - 1.0;
              double b3 =  ( self.game.Data.RegimeObj[regime].Blue3 /  byte.MaxValue) - 1.0;
              DrawMod.Draw( local21,  local22, x6, y5,  r3,  g3,  b3, 0.95f);
            }
            rectangle = Rectangle::new(num9 - 20, y2, num10 + 60, height);
            trect1 = rectangle;
            self.AddMouse( trect1, "", "Hex is controlled by " + self.game.Data.RegimeObj[regime].Name);
          }
        }
        else
        {
          tstring1: String = "Unknown Landscape";
          str8: String;
          tstring2: String = str8 + "(" + Strings.Trim(Conversion.Str( self.game.SelectX)) + "," + Strings.Trim(Conversion.Str( self.game.SelectY)) + ")";
          let mut x7: i32 = self.w - 475 + 103;
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring2, self.game.MarcFont16, x7, 35, self.game.seColGray);
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, tstring1, self.game.MarcFont4, x7, 52, self.game.seColGray);
        }
      }
       let mut local23: &Graphics = &objgraphics;
      bitmap = BitmapStore.GetBitmap(self.game.SE1_MAINFRAME_LEFT2);
       let mut local24: &Bitmap = &bitmap;
      DrawMod.DrawSimple( local23,  local24, 0, 0);
      if (self.exitId < 1)
      {
        if (self.game.EditObj.GuiDown)
        {
          let mut tsubpart: SubPartClass =  new SEButtonPartClass(self.game.SE1_ARROW3, "Show the bottom bar.", 45, 20);
          self.exitId = self.AddSubPart( tsubpart, 8, 53, 45, 20, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new SEButtonPartClass(self.game.SE1_ARROW4, "Hide the bottom bar.", 45, 20);
          self.exitId = self.AddSubPart( tsubpart, 8, 53, 45, 20, 1);
        }
      }
      if (!self.game.EditObj.GuiDown)
      {
        let mut tx1: i32 =  Math.Round( (self.w - 1280) / 2.0);
        if (self.w <= 1280)
          tx1 += 56;
        bool active1 = false;
        if (self.game.EditObj.SetViewModeExtraNr == 1)
          active1 = true;
        nameForGuiDisplay1: String = self.game.EventRelatedObj.Helper_GetZoneNameForGuiDisplay(self.game.SelectX, self.game.SelectY);
        Rectangle trect2 = self.DrawOneTab(objgraphics, true, active1, tx1, "ZONE", nameForGuiDisplay1, -1, 16, tMousingOverNow: (self.MouseOverWhichTab == 51));
        self.AddMouse( trect2, "", "Zone bottom tab", 51);
        let mut tx2: i32 = tx1 + 200;
        bool active2 = false;
        if (self.game.EditObj.SetViewModeExtraNr == 0)
          active2 = true;
        bool grayedOut = false;
        let mut spriteSlot1: i32 = -1;
        let mut spriteOffsetY1: i32 = 0;
        sText1: String;
        if (self.game.EditObj.UnitSelected > -1)
        {
          Coordinate reconMinusHide;
          if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime == self.game.Data.Turn | self.game.Data.Round == 0 | !self.game.Data.FOWOn)
            reconMinusHide.x = 3;
          else
            reconMinusHide = self.game.HandyFunctionsObj.GetReconMinusHide(self.game.EditObj.UnitSelected, self.game.Data.Turn);
          if (reconMinusHide.x >= 2)
          {
            let mut historical: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical;
            if (self.game.Data.HistoricalUnitObj[historical].SmallGfx > -1)
              spriteSlot1 = self.game.Data.SmallPicNr[self.game.Data.HistoricalUnitObj[historical].SmallGfx];
            else if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
            {
              spriteSlot1 = self.game.Data.RegimeObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime].HQSpriteNr2;
              spriteOffsetY1 = 13;
            }
            sText1 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Name;
          }
          else
            sText1 = "Unknown Unit";
        }
        else
        {
          sText1 = "No Unit selected";
          grayedOut = true;
        }
        trect2 = self.DrawOneTab(objgraphics, true, active2, tx2, "UNIT", sText1, spriteSlot1, -1, grayedOut: grayedOut, textOffsetX: -6, spriteOffsetY: spriteOffsetY1, tMousingOverNow: (self.MouseOverWhichTab == 6));
        self.AddMouse( trect2, "", "Unit bottom tab", 6);
        let mut tx3: i32 = tx2 + 200;
        bool active3 = false;
        let mut spriteOffsetY2: i32 = 13;
        if (self.game.EditObj.SetViewModeExtraNr == 2)
          active3 = true;
        nameForGuiDisplay2: String = self.game.EventRelatedObj.Helper_GetOwnerNameForGuiDisplay(self.game.SelectX, self.game.SelectY);
        let mut spriteSlot2: i32 = !(Operators.CompareString(nameForGuiDisplay2.ToLower(), "unknown", false) == 0 | Operators.CompareString(nameForGuiDisplay2.ToLower(), "unclear", false) == 0 | Operators.CompareString(nameForGuiDisplay2.ToLower(), "none", false) == 0 | self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime == -1) ? self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime].HQSpriteNr2 : -1;
        trect2 = self.DrawOneTab(objgraphics, true, active3, tx3, "REGIME", nameForGuiDisplay2, spriteSlot2, -1, textOffsetX: -2, spriteOffsetY: spriteOffsetY2, tMousingOverNow: (self.MouseOverWhichTab == 52));
        self.AddMouse( trect2, "", "Regime bottom tab", 52);
        let mut tx4: i32 = tx3 + 200;
        if (self.game.ScreenWidth >= 1530)
        {
          bool active4 = false;
          if (self.game.EditObj.SetViewModeExtraNr == 3)
            active4 = true;
          sText2: String = "ASSETS";
          trect2 = self.DrawOneTab(objgraphics, true, active4, tx4, "", sText2, -1, 17, tMousingOverNow: (self.MouseOverWhichTab == 53));
          self.AddMouse( trect2, "", "Zone Assets bottom tab", 53);
          let mut tx5: i32 = tx4 + 200;
          bool active5 = false;
          if (self.game.EditObj.SetViewModeExtraNr == 4)
            active5 = true;
          sText3: String = "ITEMS";
          trect2 = self.DrawOneTab(objgraphics, true, active5, tx5, "", sText3, -1, 10, tMousingOverNow: (self.MouseOverWhichTab == 54));
          self.AddMouse( trect2, "", "Zone Items bottom tab", 54);
          num1 = tx5 + 200;
        }
        else
        {
          bool active6 = false;
          if (self.game.EditObj.SetViewModeExtraNr == 3)
            active6 = true;
          sText4: String = "ASSETS";
          trect2 = self.DrawOneTab(objgraphics, false, active6, tx4, "", sText4, -1, 16, tMousingOverNow: (self.MouseOverWhichTab == 53));
          self.AddMouse( trect2, "", "Zone Assets bottom tab", 53);
          let mut tx6: i32 = tx4 + 75;
          bool active7 = false;
          if (self.game.EditObj.SetViewModeExtraNr == 4)
            active7 = true;
          sText5: String = "ITEMS";
          trect2 = self.DrawOneTab(objgraphics, false, active7, tx6, "", sText5, -1, 16, tMousingOverNow: (self.MouseOverWhichTab == 54));
          self.AddMouse( trect2, "", "Zone Items bottom tab", 54);
          num1 = tx6 + 75;
        }
      }
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
      objgraphics = (Graphics) null;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.BattleTimerActive)
        return windowReturnClass;
      if ((nr == 187 | nr == 191 | nr == 107) & self.game.EditObj.Zoom < 1)
        windowReturnClass = self.actionZoomIn();
      if ((nr == 189 | nr == 219 | nr == 109) & self.game.EditObj.Zoom > -1)
        windowReturnClass = self.actionZoomOut();
      bool flag = false;
      if (nr == 27 & self.tab1 > -1 & self.game.EditObj.SetViewMode2 < 1)
        flag = true;
      if (nr == 73 | flag)
      {
        self.game.EditObj.udsUnitOrderMode = 0;
        ScreenClass screeny = self.formref.Screeny;
        Type type = typeof (MapWindowClass2);
         Type local =  type;
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
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      switch (nr)
      {
        case 49:
          self.game.EditObj.layerUnits = !self.game.EditObj.layerUnits;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          if (!self.game.EditObj.layerUnits)
            self.game.EditObj.UnitSelected = -1;
          windowReturnClass.AddCommand(4, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
          windowReturnClass.AddCommand(4, 69);
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 50:
          self.game.EditObj.ShowLabel = !self.game.EditObj.ShowLabel;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          windowReturnClass.AddCommand(4, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 51:
          self.game.EditObj.HideAS = !self.game.EditObj.HideAS;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          windowReturnClass.AddCommand(4, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 52:
          self.game.EditObj.HexRasterOn = !self.game.EditObj.HexRasterOn;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          windowReturnClass.AddCommand(4, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 53:
          self.game.EditObj.RegimeColoring = !self.game.EditObj.RegimeColoring;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          windowReturnClass.AddCommand(4, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 54:
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
        case 65:
          self.game.EditObj.udsUnitOrderMode = 11;
          ScreenClass screeny1 = self.formref.Screeny;
          Type type1 = typeof (MapWindowClass2);
           Type local1 =  type1;
          MapWindowClass2 window1 = (MapWindowClass2) screeny1.GetWindow( local1);
          if (!Information.IsNothing( window1))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window1.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window1.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 71:
          self.game.EditObj.udsUnitOrderMode = 48;
          ScreenClass screeny2 = self.formref.Screeny;
          Type type2 = typeof (MapWindowClass2);
           Type local2 =  type2;
          MapWindowClass2 window2 = (MapWindowClass2) screeny2.GetWindow( local2);
          if (!Information.IsNothing( window2))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window2.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window2.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 76:
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
        case 77:
          self.game.EditObj.udsUnitOrderMode = 1;
          ScreenClass screeny3 = self.formref.Screeny;
          Type type3 = typeof (MapWindowClass2);
           Type local3 =  type3;
          MapWindowClass2 window3 = (MapWindowClass2) screeny3.GetWindow( local3);
          if (!Information.IsNothing( window3))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window3.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window3.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 80:
          self.game.EditObj.layerLisAvailable = false;
          self.game.EditObj.layerLisUsed = false;
          self.game.EditObj.layerLisTotal = false;
          self.game.EditObj.layerLisBottlenecks = false;
          self.game.EditObj.layerLisPreview = !self.game.EditObj.layerLisPreview;
          self.game.EditObj.layerLisOnlyAssetId = -1;
          if (!self.game.EditObj.layerLisPreview)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
              {
                let mut index3: i32 = 0;
                do
                {
                  self.game.Data.MapObj[0].HexObj[index1, index2].tempPreviewLIS[index3] = 0;
                  self.game.Data.MapObj[0].HexObj[index1, index2].tempPreviewAssetLIS[index3] = 0;
                  index3 += 1;
                }
                while (index3 <= 8);
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
        case 82:
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
          ScreenClass screeny4 = self.formref.Screeny;
          Type type4 = typeof (MapWindowClass2);
           Type local4 =  type4;
          MapWindowClass2 window4 = (MapWindowClass2) screeny4.GetWindow( local4);
          if (!Information.IsNothing( window4))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            window4.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 83:
          self.game.EditObj.udsUnitOrderMode = 18;
          ScreenClass screeny5 = self.formref.Screeny;
          Type type5 = typeof (MapWindowClass2);
           Type local5 =  type5;
          MapWindowClass2 window5 = (MapWindowClass2) screeny5.GetWindow( local5);
          if (!Information.IsNothing( window5))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window5.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window5.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 84:
          self.game.EditObj.udsUnitOrderMode = 53;
          ScreenClass screeny6 = self.formref.Screeny;
          Type type6 = typeof (MapWindowClass2);
           Type local6 =  type6;
          MapWindowClass2 window6 = (MapWindowClass2) screeny6.GetWindow( local6);
          if (!Information.IsNothing( window6))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window6.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window6.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 88:
          self.game.EditObj.udsUnitOrderMode = 14;
          ScreenClass screeny7 = self.formref.Screeny;
          Type type7 = typeof (MapWindowClass2);
           Type local7 =  type7;
          MapWindowClass2 window7 = (MapWindowClass2) screeny7.GetWindow( local7);
          if (!Information.IsNothing( window7))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window7.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window7.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 89:
          self.game.EditObj.udsUnitOrderMode = 33;
          ScreenClass screeny8 = self.formref.Screeny;
          Type type8 = typeof (MapWindowClass2);
           Type local8 =  type8;
          MapWindowClass2 window8 = (MapWindowClass2) screeny8.GetWindow( local8);
          if (!Information.IsNothing( window8))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window8.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window8.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 90:
          self.game.EditObj.udsUnitOrderMode = 54;
          self.game.EditObj.OrderSubType = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(self.game.SelectX, self.game.SelectY, "SE_Data", "Zones"));
          ScreenClass screeny9 = self.formref.Screeny;
          Type type9 = typeof (MapWindowClass2);
           Type local9 =  type9;
          MapWindowClass2 window9 = (MapWindowClass2) screeny9.GetWindow( local9);
          windowReturnClass.AddCommand(1, 118);
          if (!Information.IsNothing( window9))
          {
            self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
            if (self.game.EditObj.UnitSelected > -1)
              window9.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
            else
              window9.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.AddCommand(4, 68);
            windowReturnClass.AddCommand(4, 9);
          }
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        default:
          return windowReturnClass;
      }
    }

    pub fn dostuff2(Graphics g)
    {
      SizeF sizeF1 = SizeF::new();
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      let mut index1: i32 = self.game.EditObj.UnitSelected;
      if (self.game.EditObj.OrderUnit > -1 & self.game.EditObj.OrderType > 0)
        index1 = self.game.EditObj.OrderUnit;
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0 + 830.0 + 64.0 - 128.0);
      str1: String = "";
      str2: String = "";
      let mut num2: i32 = -1;
      let mut num3: i32 = -1;
      let mut regime: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime;
      if (index1 > -1)
      {
        if (self.game.Data.UnitObj[index1].IsHQ)
        {
          if (self.game.Data.UnitObj[index1].Historical > -1)
          {
            if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index1].Historical].Type == 5)
              num3 = index1;
            else if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index1].Historical].Type == 8)
            {
              num3 = index1;
              num2 = index1;
            }
          }
        }
        else if (self.game.Data.UnitObj[index1].HQ > -1 && self.game.Data.UnitObj[self.game.Data.UnitObj[index1].HQ].Historical > -1)
        {
          if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.Data.UnitObj[index1].HQ].Historical].Type == 5)
            num3 = self.game.Data.UnitObj[index1].HQ;
          else if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.Data.UnitObj[index1].HQ].Historical].Type == 8)
          {
            num2 = self.game.Data.UnitObj[index1].HQ;
            num3 = self.game.Data.UnitObj[index1].HQ;
          }
        }
      }
      bool flag1 = false;
      bool flag2 = false;
      if (self.game.EditObj.OrderType == 36)
      {
        let mut enr: i32 =  Math.Round( self.game.Data.RuleVar[703]);
        if (enr > 0)
        {
          self.game.EventRelatedObj.DoCheckSpecificEvent(enr);
          if (self.game.EditObj.OrderSubType <= 2)
            str1 = self.game.Data.RoadTypeObj[self.game.EditObj.OrderSubType].Name + " Construction Mode";
          else if (self.game.EditObj.OrderSubType == 8)
            str1 = " Road Demolition Mode";
          str2 = self.game.EditObj.udsOrderBarFeedbackString;
          color: Color;
          if (self.game.EditObj.udsOrderBarFeedbackColor <= 1)
          {
            color = Color.FromArgb( byte.MaxValue, 0,  byte.MaxValue, 0);
            flag1 = true;
          }
          else if (self.game.EditObj.udsOrderBarFeedbackColor == 2)
          {
            color = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 0, 0);
            flag2 = true;
          }
          else if (self.game.EditObj.udsOrderBarFeedbackColor == 3)
            color = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 0);
        }
      }
      else if (self.game.EditObj.udsUnitOrderMode == 0)
        str1 = "Inspection Mode";
      else if (self.game.EditObj.udsUnitOrderMode == 1 | self.game.EditObj.udsUnitOrderMode == 48)
      {
        str1 = "Move Mode";
        if (self.game.EditObj.udsUnitOrderMode == 48)
          str1 = "Group Move Mode";
        if (self.game.EditObj.OrderUnit > -1 & self.game.EditObj.MouseOverX > -1 & self.game.EditObj.MouseOverY > -1 && self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Regime == self.game.Data.Turn)
        {
          if (self.game.EditObj.TempValue[0].Value[self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY] < 999)
            str2 = self.game.EditObj.TempValue[0].Value[self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY].ToString() + " AP";
          else if (self.game.EditObj.TempValue2[0].Value[self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY] < 999 & self.game.EditObj.TempValue2[0].Value[self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY] > 0)
          {
            str2 = "Out of range - " + self.game.EditObj.TempValue2[0].Value[self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY].ToString() + " AP";
            flag2 = true;
          }
          else
          {
            str2 = "Out of range / Not enough AP";
            flag1 = true;
          }
          bool flag3 = false;
          let mut mouseOverX: i32 = self.game.EditObj.MouseOverX;
          let mut mouseOverY: i32 = self.game.EditObj.MouseOverY;
          if (mouseOverX > -1 && self.game.HandyFunctionsObj.Distance(mouseOverX, mouseOverY, 0, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].X, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Y, 0, 1) == 1)
          {
            let mut index2: i32 = 0;
            do
            {
              if (self.game.EditObj.TempAttack[0].Value[mouseOverX, mouseOverY, index2])
                flag3 = true;
              index2 += 1;
            }
            while (index2 <= 5);
            if (flag3)
            {
              flag1 = false;
              flag2 = false;
              str2 = "Right click to Attack!";
            }
          }
        }
      }
      else if (self.game.EditObj.udsUnitOrderMode == 18)
      {
        str1 = "Strategic Move Mode";
        if (self.game.EditObj.UnitSelected > -1 && self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime == self.game.Data.Turn)
        {
          let mut sizeForAirBridge: i32 = self.game.HandyFunctionsObj.GetHighestSizeForAirBridge(self.game.EditObj.UnitSelected);
          let mut mouseOverX: i32 = self.game.EditObj.MouseOverX;
          let mut mouseOverY: i32 = self.game.EditObj.MouseOverY;
          str1 = str1 + " - Weight: " + self.game.HandyFunctionsObj.GetUnitWeight(self.game.EditObj.UnitSelected, includeLisWeight: true).ToString();
          if (mouseOverX > -1 & mouseOverY > -1)
            str1 = str1 + " / " + self.game.EditObj.TempValue[0].Value[mouseOverX, mouseOverY].ToString();
          bool flag4 = false;
          if (mouseOverX > -1 & mouseOverY > -1 & self.game.EventRelatedObj.Helper_AirEnabled())
          {
            Coordinate coordinate = self.game.EditObj.TempCameFrom[0].Value[mouseOverX, mouseOverY];
            if (coordinate.onmap && self.game.HandyFunctionsObj.HexFacing(mouseOverX, mouseOverY, 0, coordinate.x, coordinate.y, 0) == -1 & coordinate.data1 > 0 | coordinate.data2 > 0)
            {
              let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 534, 0, 0));
              let mut length: i32 = self.game.Data.StringListObj[stringListById2].Length;
              for (let mut index3: i32 = 0; index3 <= length; index3 += 1)
              {
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 0])) == self.game.Data.RegimeObj[self.game.Data.Turn].id)
                {
                  if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 1])) == coordinate.x &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 2])) == coordinate.y &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 3])) == mouseOverX &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 4])) == mouseOverY)
                  {
                    flag4 = true;
                    letter: String = self.game.HandyFunctionsObj.CovertNumberToLetter( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 8])));
                    str1 = str1 + ", Air Bridge Points used: " + self.game.HandyFunctionsObj.GetUnitWeight(self.game.EditObj.UnitSelected, includeLisWeight: true).ToString() + "/" + coordinate.data1.ToString() + " (" + letter + "), " + "Dam: " + ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 7]))).ToString() + ", " + "Max Size: " + sizeForAirBridge.ToString() + " / " + ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 10]))).ToString();
                  }
                  if (!flag4 &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 3])) == coordinate.x &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 4])) == coordinate.y &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 1])) == mouseOverX &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 2])) == mouseOverY)
                  {
                    flag4 = true;
                    letter: String = self.game.HandyFunctionsObj.CovertNumberToLetter( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 8])));
                    str1 = str1 + ", Air Bridge Points used: " + self.game.HandyFunctionsObj.GetUnitWeight(self.game.EditObj.UnitSelected, includeLisWeight: true).ToString() + "/" + coordinate.data1.ToString() + " (" + letter + "), " + "Dam: " + ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 7]))).ToString() + ", " + "Max Size: " + sizeForAirBridge.ToString() + " / " + ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index3, 10]))).ToString();
                  }
                }
              }
            }
          }
          if (!flag4)
          {
            str3: String = "";
            str4: String = "";
            let mut num4: i32 = 0;
            let mut num5: i32 = 0;
            let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 534, 0, 0));
            if (stringListById3 > 0)
            {
              let mut length: i32 = self.game.Data.StringListObj[stringListById3].Length;
              for (let mut index4: i32 = 0; index4 <= length; index4 += 1)
              {
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 0])) == self.game.Data.RegimeObj[self.game.Data.Turn].id)
                {
                  if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 1])) == mouseOverX &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 2])) == mouseOverY)
                  {
                    letter: String = self.game.HandyFunctionsObj.CovertNumberToLetter( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 8])));
                    if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 5])) > num4)
                    {
                      num4 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 5]));
                      str3 = letter;
                    }
                    if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 10])) > num5)
                    {
                      num5 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 10]));
                      str4 = letter;
                    }
                  }
                  if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 3])) == mouseOverX &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 4])) == mouseOverY)
                  {
                    letter: String = self.game.HandyFunctionsObj.CovertNumberToLetter( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 8])));
                    if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 5])) > num4)
                    {
                      num4 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 5]));
                      str3 = letter;
                    }
                    if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 10])) > num5)
                    {
                      num5 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[index4, 10]));
                      str4 = letter;
                    }
                  }
                }
              }
            }
            if (num5 > 0)
              str1 = str1 + ", Best Airbr. Pts in Hex: " + num4.ToString() + " (" + str3 + "), " + "Best Airbr. Max Size in Hex: " + sizeForAirBridge.ToString() + " / " + num5.ToString() + " (" + str4 + ") ";
          }
        }
      }
      else if (self.game.EditObj.udsUnitOrderMode == 11)
        str1 = "Ranged Attack Mode";
      else if (self.game.EditObj.udsUnitOrderMode == 14)
        str1 = "Air Attack Mode";
      else if (self.game.EditObj.udsUnitOrderMode == 55)
        str1 = "Air Bridge Mode";
      else if (self.game.EditObj.udsUnitOrderMode == 33)
        str1 = "Air Recon Mode";
      else if (self.game.EditObj.udsUnitOrderMode == 53)
        str1 = "Traffic Signs Mode";
      else if (self.game.EditObj.udsUnitOrderMode == 54)
        str1 = "Zone Border Mode - Drawing Hexes for ZONE '" + self.game.Data.StringListObj[stringListById1].GetData(0, self.game.EditObj.OrderSubType, 7) + "'";
      str5: String = str1.ToUpper();
      if (str2.Length > 0)
        str5 = str5 + ": " + str2;
      SizeF sizeF2 = g.MeasureString(str5, DrawMod.TGame.MarcFont16);
       let mut local1: &Graphics = &g;
      bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_BLACKGRADIENT);
       let mut local2: &Bitmap = &bitmap;
      let mut x: i32 =  Math.Round( ( self.w -  (270.0 +  sizeF2.Width + 100.0)));
      let mut w: i32 =  Math.Round( (sizeF2.Width + 100f));
      DrawMod.DrawScaled( local1,  local2, x, 3, w, 28, true);
      c: Color = !flag1 ? (!flag2 ? self.game.seColWhite : Color.FromArgb( byte.MaxValue,  byte.MaxValue, 175, 100)) : Color.FromArgb( byte.MaxValue,  byte.MaxValue, 100, 100);
      DrawMod.DrawTextColouredConsole( g, str5, self.game.MarcFont16,  Math.Round( ( self.w -  (270.0 +  sizeF2.Width + 20.0))), 7, c);
    }

    pub fn DoTabs1( Graphics g, bool Active = false)
    {
      SizeF sizeF = SizeF::new();
    }

    pub fn DoTabs1B( Graphics g, bool Active = false)
    {
      SizeF sizeF = SizeF::new();
    }

    pub fn DoTabs2( Graphics g, bool Active = false)
    {
      SizeF sizeF = SizeF::new();
    }

    pub fn DoTabs2B( Graphics g, bool Active = false)
    {
      SizeF sizeF = SizeF::new();
    }

    pub fn DoTabs3( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      if (self.game.Data.Round == 0)
        return;
      let mut num: i32 =  Math.Round( self.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num + 340 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 = num + 340 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "COMBAT SETUP";
      SizeF sizeF2 = g.MeasureString(str, self.game.MarcFont16);
      let mut x1: i32 =  Math.Round( ( (num + 420 + 340 + 91) - sizeF2.Width / 2f));
      let mut y: i32 = 66;
      DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 340 + 420, y, 182, 24);
      self.AddMouse( trect, "", "Click to see the combat setup. [F12]", 3);
      self.tab3 = self.MouseCounter;
    }

    pub fn DoTabs4( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =  Math.Round( self.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num + 0 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 = num + 0 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "OFFICER INFO";
      SizeF sizeF2 = g.MeasureString(str, self.game.MarcFont16);
      let mut x1: i32 =  Math.Round( ( (num + 420 + 0 + 91) - sizeF2.Width / 2f));
      let mut y: i32 = 66;
      DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 0 + 420, y, 182, 24);
      self.AddMouse( trect, "", "Click to see the info on the officer with this unit. [F9]", 4);
      self.tab4 = self.MouseCounter;
    }

    pub fn DoTabs4B( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =  Math.Round( self.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num + 170 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 = num + 170 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "OFFICER INFO";
      SizeF sizeF2 = g.MeasureString(str, self.game.MarcFont16);
      let mut x1: i32 =  Math.Round( ( (num + 420 + 170 + 91) - sizeF2.Width / 2f));
      let mut y: i32 = 66;
      DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 170 + 420, y, 182, 24);
      self.AddMouse( trect, "", "Click to see the info on the officer with this unit. [F9]", 4);
      self.tab4 = self.MouseCounter;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
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

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = base.HandleMouseMove(x, y);
      if (y > 18 &&  self.w / 2.0 - 500.0 <  x &  x <  self.w / 2.0 + 500.0)
        windowReturnClass.NoMouseClickBelow = true;
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

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      numArray: Vec<i32> = new int[self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1, self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight + 1];
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.BattleTimerActive)
        return windowReturnClass;
      if (y > 18 &&  self.w / 2.0 - 500.0 <  x &  x <  self.w / 2.0 + 500.0)
        windowReturnClass.NoMouseClickBelow = true;
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          switch (self.MouseData[mouseCounter])
          {
            case 1:
              if (self.game.EditObj.SetViewMode == 0)
                self.game.EditObj.SubformationListMode = !self.game.EditObj.SubformationListMode;
              self.game.EditObj.SetViewMode = 0;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 2:
              self.game.EditObj.SetViewMode = 1;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 3:
              self.game.EditObj.SetViewMode = 2;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 4:
              self.game.EditObj.SetViewMode = 3;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 6:
              self.game.EditObj.SetViewModeExtraNr = 0;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 51:
              self.game.EditObj.SetViewModeExtraNr = 1;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 52:
              self.game.EditObj.SetViewModeExtraNr = 2;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 53:
              self.game.EditObj.SetViewModeExtraNr = 3;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 54:
              self.game.EditObj.SetViewModeExtraNr = 4;
              self.dostuff();
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            default:
              continue;
          }
        }
      }
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            if (self.SubPartID[index1] == self.exitId)
            {
              if (!self.game.EditObj.GuiDown)
                self.game.EditObj.GuiDown = true;
              else
                self.game.EditObj.GuiDown = false;
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            let mut butCount: i32 = self.butCount;
            for (let mut index2: i32 = 0; index2 <= butCount; index2 += 1)
            {
              if (self.butId[index2] == self.SubPartID[index1])
              {
                let mut areaX: i32 = self.game.EditObj.AreaX;
                let mut areaY: i32 = self.game.EditObj.AreaY;
                self.game.EditObj.AreaX = self.game.SelectX;
                self.game.EditObj.AreaY = self.game.SelectY;
                self.game.EditObj.UDSinputCounter = -1;
                self.game.EventRelatedObj.DoCheckSpecificEvent(self.butEvent[index2], self.butTempVar0[index2], self.butTempVar1[index2]);
                let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[409]));
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.butTempVarStringlistId[index2], 9))) < 1)
                {
                  self.game.EditObj.AreaX = areaX;
                  self.game.EditObj.AreaY = areaY;
                  windowReturnClass.SetFlag(true);
                  self.game.EditObj.PopupValue = 21;
                  windowReturnClass.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.SetFlag(true);
                self.game.EditObj.QuestionText = self.game.Data.StringListObj[stringListById].GetData(0, self.butTempVarStringlistId[index2], 10);
                self.game.EditObj.DoCardSlot = -1;
                self.game.EditObj.HandCard = -1;
                self.game.EditObj.PopupValue = 1;
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn PopUpRefresh() => self.DoRefresh();

    pub actionZoomOut: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = 0;
      if (self.game.EditObj.GuiDown)
        num1 = 222;
      let mut num2: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 53.0));
      let mut num3: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 106.0));
      let mut num4: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 53.0));
      let mut num5: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 106.0));
      num6: i32;
      num7: i32;
      if (self.game.EditObj.Zoom == 0)
      {
        self.game.EditObj.Zoom = -1;
        self.game.CornerX -=  Math.Round(Conversion.Int( num2 / 2.0));
        self.game.CornerY -=  Math.Round(Conversion.Int( num4 / 2.0));
        num6 = 27;
        num7 = 24;
      }
      else
      {
        self.game.EditObj.Zoom = 0;
        self.game.CornerX -=  Math.Round(Conversion.Int( num3 / 2.0));
        self.game.CornerY -=  Math.Round(Conversion.Int( num5 / 2.0));
        num6 = 53;
        num7 = 48;
      }
      if (!self.game.Data.MapObj[0].MapLoop)
      {
        if ( self.game.CornerX +  self.game.ScreenWidth /  num6 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
          self.game.CornerX =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth) -  (self.game.ScreenWidth - 200) /  num6);
        if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
          self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
        if (self.game.CornerX < 0)
          self.game.CornerX = 0;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
      }
      else
      {
        if (self.game.CornerX > self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
        {
          self.game.CornerX =  Math.Round( self.game.CornerX +  self.game.ScreenWidth /  num6);
          self.game.CornerX -= self.game.Data.MapObj[0].MapWidth;
        }
        if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
          self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
        if (self.game.CornerX < 0)
          self.game.CornerX = self.game.Data.MapObj[0].MapWidth + self.game.CornerX;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
      }
      self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
      self.game.EditObj.TempCoordList = CoordList::new();
      windowReturnClass.AddCommand(1, 12);
      windowReturnClass.AddCommand(2, 12);
      windowReturnClass.AddCommand(4, 9);
      windowReturnClass.AddCommand(4, 67);
      windowReturnClass.AddCommand(4, 68);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub actionZoomIn: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = 0;
      if (self.game.EditObj.GuiDown)
        num1 = 222;
      let mut num2: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 53.0));
      let mut num3: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 106.0));
      let mut num4: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 53.0));
      let mut num5: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 106.0));
      num6: i32;
      num7: i32;
      if (self.game.EditObj.Zoom == 0)
      {
        self.game.EditObj.Zoom = 1;
        self.game.CornerX +=  Math.Round(Conversion.Int( num3 / 2.0));
        self.game.CornerY +=  Math.Round(Conversion.Int( num5 / 2.0));
        num6 = 106;
        num7 = 96;
      }
      else
      {
        self.game.EditObj.Zoom = 0;
        self.game.CornerX +=  Math.Round(Conversion.Int( num2 / 2.0));
        self.game.CornerY +=  Math.Round(Conversion.Int( num4 / 2.0));
        num6 = 53;
        num7 = 48;
      }
      if (!self.game.Data.MapObj[0].MapLoop)
      {
        if ( self.game.CornerX +  self.game.ScreenWidth /  num6 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
          self.game.CornerX =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth) -  (self.game.ScreenWidth - 200) /  num6);
        if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
          self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
        if (self.game.CornerX < 0)
          self.game.CornerX = 0;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
      }
      else
      {
        if (self.game.CornerX > self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
          self.game.CornerX -= self.game.Data.MapObj[0].MapWidth;
        if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
          self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
        if (self.game.CornerX < 0)
          self.game.CornerX = self.game.Data.MapObj[0].MapWidth + self.game.CornerX;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
      }
      self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
      self.game.EditObj.TempCoordList = CoordList::new();
      windowReturnClass.AddCommand(1, 12);
      windowReturnClass.AddCommand(2, 12);
      windowReturnClass.AddCommand(4, 9);
      windowReturnClass.AddCommand(4, 67);
      windowReturnClass.AddCommand(4, 68);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }
  }
}
