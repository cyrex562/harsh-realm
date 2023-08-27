// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SSEditOptionsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SSEditOptionsWindowClass : WindowClass
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
    pub RegId: i32;
    pub RegIdb: i32;
    pub StringId: i32;
    pub StringIdb: i32;
    pub DebugId: i32;
    pub DebugIdb: i32;

    pub fn PopUpRefresh()
    {
    }

    pub SSEditOptionsWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 = -1, let mut sy: i32 = -1)
      : base( tGame, tGame.ScreenWidth, 50, 9, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      self.game.EditObj.inSimpleEditor = true;
      self.domenu();
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
      if (self.StringId > 0)
        self.RemoveSubPart(self.StringId);
      if (self.StringIdb > 0)
        self.RemoveSubPart(self.StringIdb);
      if (self.DebugId > 0)
        self.RemoveSubPart(self.DebugId);
      if (self.DebugIdb > 0)
        self.RemoveSubPart(self.DebugIdb);
      let mut num1: i32 = 10 +  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Exit", 100, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.BackId = self.AddSubPart( tsubpart1, 10, 5, 100, 40, 1);
      let mut num2: i32 = num1 + 110;
      SubPartClass tsubpart2;
      if (self.game.EditObj.SimpleEditWindow != 95)
      {
        tsubpart2 =  new TextButtonPartClass("Dashboard", 100, tBackbitmap: ( self.OwnBitmap), bbx: num2, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.DashId = self.AddSubPart( tsubpart2, num2, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Dashboard", 100, tBackbitmap: ( self.OwnBitmap), bbx: num2, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.DashIdb = self.AddSubPart( tsubpart2, num2, 5, 100, 40, 1);
      }
      let mut num3: i32 = num2 + 110;
      if (self.game.EditObj.SimpleEditWindow != 101)
      {
        tsubpart2 =  new TextButtonPartClass("Tables", 100, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.StringId = self.AddSubPart( tsubpart2, num3, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Tables", 100, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.StringIdb = self.AddSubPart( tsubpart2, num3, 5, 100, 40, 1);
      }
      let mut num4: i32 = num3 + 110;
      if (self.game.EditObj.SimpleEditWindow != 122)
      {
        tsubpart2 =  new TextButtonPartClass("Event Pics", 100, tBackbitmap: ( self.OwnBitmap), bbx: num4, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.LibId = self.AddSubPart( tsubpart2, num4, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Event Pics", 100, tBackbitmap: ( self.OwnBitmap), bbx: num4, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.LibIdb = self.AddSubPart( tsubpart2, num4, 5, 100, 40, 1);
      }
      let mut num5: i32 = num4 + 110;
      if (self.game.EditObj.SimpleEditWindow != 123)
      {
        tsubpart2 =  new TextButtonPartClass("Small Gfx", 100, tBackbitmap: ( self.OwnBitmap), bbx: num5, bby: 5, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.UnitId = self.AddSubPart( tsubpart2, num5, 5, 100, 40, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Small Gfx", 100, tBackbitmap: ( self.OwnBitmap), bbx: num5, bby: 5, tinactive: true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.UnitIdb = self.AddSubPart( tsubpart2, num5, 5, 100, 40, 1);
      }
      let mut num6: i32 = num5 + 110;
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
              if (!self.game.SuperAdminRights)
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
                windowReturnClass.AddCommand(2, 95);
                self.game.EditObj.SimpleEditWindow = 95;
                self.domenu();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num == self.LibId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 122);
                self.game.EditObj.SimpleEditWindow = 122;
                self.domenu();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num == self.UnitId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 123);
                self.game.EditObj.SimpleEditWindow = 123;
                self.domenu();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num == self.DebugId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 109);
                self.game.EditObj.SimpleEditWindow = 109;
                self.domenu();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num == self.StringId)
              {
                self.game.EditObj.PencilMode = 0;
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 101);
                self.game.EditObj.SimpleEditWindow = 101;
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
