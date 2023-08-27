// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UdsUnitOpsWindowClass
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
  pub class UdsUnitOpsWindowClass : WindowClass
  {
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
     cancelid: i32;
     int[] udsBut;
     udsButCounter: i32;
     zoneId: i32;
     SimpleStringList SL;

    pub UdsUnitOpsWindowClass(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
       useHeight: i32)
      : base( tGame, 200, useHeight, 8)
    {
      self.udsBut = new int[100];
      self.udsButCounter = -1;
      self.zoneId = -1;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.SL = self.game.HandyFunctionsObj.UnitPopupUdsButtons();
      self.udsButCounter = -1;
      self.View();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn View()
    {
      if (self.cancelid > 0)
      {
        self.RemoveSubPart(self.cancelid);
        self.cancelid = 0;
      }
      self.ClearMouse();
      self.NewBackGroundAndClearAll(200, self.LowerRect.Height, -1);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 200, self.LowerRect.Height);
      SizeF sizeF1 = SizeF::new();
      str1: String = "Select Order Mode";
      SizeF sizeF2 = graphics.MeasureString(str1, self.game.MarcFont4);
      DrawMod.DrawTextColouredMarc( graphics, str1, self.game.MarcFont4,  Math.Round(100.0 -  sizeF2.Width / 2.0), 10, Color.White);
      let mut num1: i32 = 28;
      let mut num2: i32 = 30;
      bool flag1 = false;
      if (self.game.EditObj.udsUnitOrderMode == 0)
        flag1 = true;
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Inspect", 160, "Allows you to just look at your units [Shortkey I]",  self.OwnBitmap, 20, num2, flag1, flag1, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but1 = self.AddSubPart( tsubpart1, 20, num2, 160, num1, 1);
      let mut num3: i32 = num2 + (num1 + 2);
      bool flag2 = false;
      if (self.game.EditObj.udsUnitOrderMode == 1)
        flag2 = true;
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Move", 160, "Allows you to move units and initiate regular attacks [Shortkey M]",  self.OwnBitmap, 20, num3, flag2, flag2, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but2 = self.AddSubPart( tsubpart2, 20, num3, 160, num1, 1);
      let mut num4: i32 = num3 + (num1 + 2);
      bool flag3 = false;
      if (self.game.EditObj.udsUnitOrderMode == 48)
        flag3 = true;
      let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Group Move", 160, "Allows you to move all units in the same hex at the same time [Shortkey G]",  self.OwnBitmap, 20, num4, flag3, flag3, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but7 = self.AddSubPart( tsubpart3, 20, num4, 160, num1, 1);
      let mut num5: i32 = num4 + (num1 + 2);
      bool flag4 = false;
      if (self.game.EditObj.udsUnitOrderMode == 18)
        flag4 = true;
      let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Strategic Move", 160, "Allows your units to be transfered by your logistical network [Shortkey S]",  self.OwnBitmap, 20, num5, flag4, flag4, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but3 = self.AddSubPart( tsubpart4, 20, num5, 160, num1, 1);
      let mut num6: i32 = num5 + (num1 + 2);
      bool flag5 = false;
      if (self.game.EditObj.udsUnitOrderMode == 11)
        flag5 = true;
      let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Ranged Attack", 160, "Allows your to target units with artillery or missile fire [Shortkey A]",  self.OwnBitmap, 20, num6, flag5, flag5, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but4 = self.AddSubPart( tsubpart5, 20, num6, 160, num1, 1);
      if (self.game.EventRelatedObj.Helper_AirEnabled())
      {
        let mut num7: i32 = num6 + (num1 + 2);
        bool flag6 = false;
        if (self.game.EditObj.udsUnitOrderMode == 14)
          flag6 = true;
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Air Attack", 160, "Allows your to target units with your airforce [Shortkey X]",  self.OwnBitmap, 20, num7, flag6, flag6, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.but9 = self.AddSubPart( tsubpart6, 20, num7, 160, num1, 1);
        let mut num8: i32 = num7 + (num1 + 2);
        bool flag7 = false;
        if (self.game.EditObj.udsUnitOrderMode == 33)
          flag7 = true;
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Air Recon", 160, "Allows your to do a recon mission with your airforce [Shortkey Y]",  self.OwnBitmap, 20, num8, flag7, flag7, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.but10 = self.AddSubPart( tsubpart7, 20, num8, 160, num1, 1);
        num6 = num8 + (num1 + 2);
        bool flag8 = false;
        if (self.game.EditObj.udsUnitOrderMode == 55)
          flag8 = true;
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("Air Bridge", 160, "Allows your to order Air Bridges",  self.OwnBitmap, 20, num6, flag8, flag8, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.but11 = self.AddSubPart( tsubpart8, 20, num6, 160, num1, 1);
      }
      if ( self.game.Data.RuleVar[702] > 0.0)
      {
        num6 += num1 + 2;
        bool flag9 = false;
        if (self.game.EditObj.udsUnitOrderMode == 36)
          flag9 = true;
        let mut tsubpart9: SubPartClass =  new TextButtonPartClass("Construct Road", 160, "Allows your to construct roads [Shortkey R]",  self.OwnBitmap, 20, num6, flag9, flag9, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.but5 = self.AddSubPart( tsubpart9, 20, num6, 160, num1, 1);
      }
      let mut num9: i32 = num6 + (num1 + 2);
      bool flag10 = false;
      if (self.game.EditObj.udsUnitOrderMode == 53)
        flag10 = true;
      let mut tsubpart10: SubPartClass =  new TextButtonPartClass("Traffic Signs", 160, "Allows you to place and remove Traffic Signs [Shortkey T]",  self.OwnBitmap, 20, num9, flag10, flag10, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but6 = self.AddSubPart( tsubpart10, 20, num9, 160, num1, 1);
      data: DataClass = DrawMod.TGame.Data;
      str2: String = "Zones";
       local: String =  str2;
      self.zoneId = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].GetHexLibVarValue(data.FindLibVar( local, "SE_Data"));
      let mut num10: i32 = -1;
      if (self.zoneId > 0)
        num10 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0))].GetData(0, self.zoneId, 8)));
      let mut num11: i32 = num9 + (num1 + 2);
      bool flag11 = false;
      if (self.game.EditObj.udsUnitOrderMode == 54)
        flag11 = true;
      if (self.game.Data.RegimeObj[self.game.Data.Turn].id == num10 & self.zoneId > 0)
      {
        let mut tsubpart11: SubPartClass =  new TextButtonPartClass("Zone Borders", 160, "Allows you to re-draw the Zone Borders [Shortkey Z]",  self.OwnBitmap, 20, num11, flag11, flag11, num1, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.but8 = self.AddSubPart( tsubpart11, 20, num11, 160, num1, 1);
      }
      let mut num12: i32 = num11 + (num1 + 2);
      let mut tsubpart12: SubPartClass =  new TextButtonPartClass("Exit", 100, "Click to return to main screen.",  self.OwnBitmap, 50, self.LowerRect.Height - 55, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.cancelid = self.AddSubPart( tsubpart12, 50, self.LowerRect.Height - 55, 100, 40, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(1, 118);
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

    pub HandleMouseClickOutsideWindow: WindowReturnClass(
      x: i32,
      y: i32,
      b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
      ScreenClass screeny = self.formref.Screeny;
      System.Type type = typeof (MapWindowClass2);
       System.Type local =  type;
      MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
      if (!Information.IsNothing( window) & self.game.EditObj.UnitSelected > -1)
        window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
      windowReturnClass.AddCommand(1, 118);
      windowReturnClass.SetFlag(true);
      windowReturnClass.alwaysExecuteWR = true;
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
            let mut num: i32 = self.SubPartID[index1];
            if (num == self.cancelid)
            {
              self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
              ScreenClass screeny = self.formref.Screeny;
              System.Type type = typeof (MapWindowClass2);
               System.Type local =  type;
              MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
              if (!Information.IsNothing( window))
              {
                if (self.game.EditObj.UnitSelected > -1)
                  window.UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                else
                  window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
              }
              windowReturnClass.AddCommand(1, 118);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.but1)
            {
              self.game.EditObj.udsUnitOrderMode = 0;
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
            if (num == self.but6)
            {
              self.game.EditObj.udsUnitOrderMode = 53;
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
            if (num == self.but8)
            {
              self.game.EditObj.udsUnitOrderMode = 54;
              self.game.EditObj.OrderSubType = self.zoneId;
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
            if (num == self.but2)
            {
              self.game.EditObj.udsUnitOrderMode = 1;
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
            if (num == self.but7)
            {
              self.game.EditObj.udsUnitOrderMode = 48;
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
            if (num == self.but3)
            {
              self.game.EditObj.udsUnitOrderMode = 18;
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
            if (num == self.but4)
            {
              self.game.EditObj.udsUnitOrderMode = 11;
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
            if (num == self.but9)
            {
              self.game.EditObj.udsUnitOrderMode = 14;
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
            if (num == self.but10)
            {
              self.game.EditObj.udsUnitOrderMode = 33;
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
            if (num == self.but11)
            {
              self.game.EditObj.udsUnitOrderMode = 55;
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
            if (num == self.but5)
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
                windowReturnClass.AddCommand(1, 118);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.EditObj.udsUnitOrderMode = 36;
              ScreenClass screeny = self.formref.Screeny;
              System.Type type = typeof (MapWindowClass2);
               System.Type local =  type;
              MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
              windowReturnClass.AddCommand(1, 118);
              if (!Information.IsNothing( window))
              {
                self.game.EditObj.OldUnit = self.game.EditObj.UnitSelected;
                window.UdsClickUnit(self.game.SelectX, self.game.SelectY, 0, true);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 68);
                windowReturnClass.AddCommand(4, 9);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            let mut udsButCounter: i32 = self.udsButCounter;
            for (let mut index2: i32 = 0; index2 <= udsButCounter; index2 += 1)
            {
              if (self.udsBut[index2] == self.SubPartID[index1])
              {
                self.game.HandyFunctionsObj.UnitPopupUdsButtons_SetIO(self.SL.Data2[index2]);
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                let mut areaX: i32 = self.game.EditObj.AreaX;
                let mut areaY: i32 = self.game.EditObj.AreaY;
                self.game.EditObj.AreaX = self.game.SelectX;
                self.game.EditObj.AreaY = self.game.SelectY;
                self.game.EventRelatedObj.DoCheckSpecificEvent(self.SL.Data1[index2]);
                self.game.EditObj.AreaX = areaX;
                self.game.EditObj.AreaY = areaY;
                self.formref.Cursor = Cursors.Default;
                self.game.EditObj.UDSpushedPopupText = self.game.EditObj.UDSpopupText;
                self.game.EditObj.UDSpopupText = "";
                windowReturnClass.AddCommand(1, 118);
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
  }
}
