// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleEditMapOptionsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SimpleEditMapOptionsWindowClass : WindowClass
  {
    pub BBackId: i32;
    pub BBackTextId: i32;
    pub LibId: i32;
    pub LibIdb: i32;
    pub DashId: i32;
    pub DashIdb: i32;
    pub BackId: i32;
    pub BackIdb: i32;
    pub MapId: i32;
    pub MapIdb: i32;
    pub UnitId: i32;
    pub UnitIdb: i32;
    pub ReplId: i32;
    pub ReplIdb: i32;
    pub RegId: i32;
    pub RegIdb: i32;
    pub StringId: i32;
    pub StringIdb: i32;

    pub fn PopUpRefresh()
    {
    }

    pub SimpleEditMapOptionsWindowClass(
       tGame: GameClass,
      screenbitmap: Bitmap = null,
      let mut sx: i32 = -1,
      let mut sy: i32 = -1)
      : base( tGame, tGame.ScreenWidth, 50, 9, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      self.game.EditObj.inSimpleEditor = true;
      self.domenu();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = -50;
      if (self.game.EditObj.SimpleEditWindow == 98 | self.game.EditObj.SimpleEditWindow == 99)
      {
        if ((nr == 187 | nr == 191 | nr == 107) & self.game.EditObj.Zoom < 1)
        {
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
          if ( self.game.CornerX +  self.game.ScreenWidth /  num6 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
            self.game.CornerX =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth) -  (self.game.ScreenWidth - 200) /  num6);
          if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
            self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
          if (self.game.CornerX < 0)
            self.game.CornerX = 0;
          if (self.game.CornerY < 0)
            self.game.CornerY = 0;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          self.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass.AddCommand(1, 5);
          windowReturnClass.AddCommand(2, 12);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if ((nr == 189 | nr == 219 | nr == 109) & self.game.EditObj.Zoom > -1)
        {
          let mut num8: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 53.0));
          let mut num9: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 106.0));
          let mut num10: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 53.0));
          let mut num11: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 106.0));
          num12: i32;
          num13: i32;
          if (self.game.EditObj.Zoom == 0)
          {
            self.game.EditObj.Zoom = -1;
            self.game.CornerX -=  Math.Round(Conversion.Int( num8 / 2.0));
            self.game.CornerY -=  Math.Round(Conversion.Int( num10 / 2.0));
            num12 = 27;
            num13 = 24;
          }
          else
          {
            self.game.EditObj.Zoom = 0;
            self.game.CornerX -=  Math.Round(Conversion.Int( num9 / 2.0));
            self.game.CornerY -=  Math.Round(Conversion.Int( num11 / 2.0));
            num12 = 53;
            num13 = 48;
          }
          if ( self.game.CornerX +  self.game.ScreenWidth /  num12 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
            self.game.CornerX =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth) -  (self.game.ScreenWidth - 200) /  num12);
          if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num13 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
            self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num13);
          if (self.game.CornerX < 0)
            self.game.CornerX = 0;
          if (self.game.CornerY < 0)
            self.game.CornerY = 0;
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          self.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass.AddCommand(1, 5);
          windowReturnClass.AddCommand(2, 12);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      return windowReturnClass;
    }

    pub fn domenu()
    {
      if (self.BackId > 0)
        self.RemoveSubPart(self.BackId);
      if (self.LibId > 0)
        self.RemoveSubPart(self.LibId);
      if (self.BackIdb > 0)
        self.RemoveSubPart(self.BackIdb);
      if (self.LibIdb > 0)
        self.RemoveSubPart(self.LibIdb);
      if (self.DashId > 0)
        self.RemoveSubPart(self.DashId);
      if (self.DashIdb > 0)
        self.RemoveSubPart(self.DashIdb);
      if (self.MapId > 0)
        self.RemoveSubPart(self.MapId);
      if (self.MapIdb > 0)
        self.RemoveSubPart(self.MapIdb);
      if (self.UnitId > 0)
        self.RemoveSubPart(self.UnitId);
      if (self.UnitIdb > 0)
        self.RemoveSubPart(self.UnitIdb);
      if (self.RegId > 0)
        self.RemoveSubPart(self.RegId);
      if (self.RegIdb > 0)
        self.RemoveSubPart(self.RegIdb);
      if (self.ReplId > 0)
        self.RemoveSubPart(self.ReplId);
      if (self.ReplIdb > 0)
        self.RemoveSubPart(self.ReplIdb);
      if (self.StringId > 0)
        self.RemoveSubPart(self.StringId);
      if (self.StringIdb > 0)
        self.RemoveSubPart(self.StringIdb);
      let mut num1: i32 = 10 +  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Exit", 100, tBackbitmap: ( self.OwnBitmap), bbx: num1, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.BackId = self.AddSubPart( tsubpart1, num1, 5, 100, 40, 1);
      let mut num2: i32 = num1 + 110;
      SubPartClass tsubpart2;
      if (self.game.EditObj.SimpleEditWindow != 95)
      {
        tsubpart2 =  new TextButtonPartClass("Options", 100, tBackbitmap: ( self.OwnBitmap), bbx: num2, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.DashId = self.AddSubPart( tsubpart2, num2, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Options", 100, tBackbitmap: ( self.OwnBitmap), bbx: num2, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.DashIdb = self.AddSubPart( tsubpart2, num2, 5, 100, 40, 1);
      }
      let mut num3: i32 = num2 + 110;
      if (self.game.EditObj.SimpleEditWindow != 98)
      {
        tsubpart2 =  new TextButtonPartClass("Map", 100, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.MapId = self.AddSubPart( tsubpart2, num3, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Map", 100, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.MapIdb = self.AddSubPart( tsubpart2, num3, 5, 100, 40, 1);
      }
      let mut num4: i32 = num3 + 110;
      if (self.game.EditObj.SimpleEditWindow != 112)
      {
        tsubpart2 =  new TextButtonPartClass("Replace", 100, tBackbitmap: ( self.OwnBitmap), bbx: num4, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.ReplId = self.AddSubPart( tsubpart2, num4, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Replace", 100, tBackbitmap: ( self.OwnBitmap), bbx: num4, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.ReplIdb = self.AddSubPart( tsubpart2, num4, 5, 100, 40, 1);
      }
    }

    pub fn DoRefresh() => self.domenu();

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
            let mut num: i32 = self.SubPartID[index];
            if (num == self.BackId)
            {
              self.game.EditObj.InEditor = false;
              self.game.EditObj.inSimpleMapEditor = false;
              if (self.game.EditorBlock)
                self.game.EditObj.ShowInitialMenu = true;
              if (self.game.ModIntroType == 0)
                windowReturnClass.AddCommand(3, 1);
              else
                windowReturnClass.AddCommand(3, 12);
            }
            else
            {
              if (num == self.DashId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 107);
                self.game.EditObj.SimpleEditWindow = 107;
                self.domenu();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num == self.MapId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 98);
                windowReturnClass.AddCommand(2, 12);
                self.game.EditObj.SimpleEditWindow = 98;
                self.domenu();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num == self.ReplId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 112);
                self.game.EditObj.SimpleEditWindow = 112;
                self.domenu();
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
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
