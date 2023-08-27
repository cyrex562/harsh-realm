// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MainEditScreenClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class MainEditScreenClass : ScreenClass
  {
     WEditMenu: i32;
     Wmiddle: i32;
     wdown: i32;
     wleft: i32;
     worder: i32;
     wup: i32;
     OffSetX: i32;

    pub MainEditScreenClass( tgame: GameClass, tformref: Form1)
      : base( tgame, tgame.BACKGROUND3MARC, tformref)
    {
      self.OffSetX =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
      self.AllowRightMouse = true;
      if ( tgame.Data.RuleVar[839] == 1.0)
      {
        self.wdown = self.AddWindow((WindowClass) new PlayExtraWindowClass2( tgame, self.OwnBackground, 0, self.Game.ScreenHeight - 210), 0, self.Game.ScreenHeight - 210, self.Game.ScreenWidth, 222);
        self.wleft = self.AddWindow((WindowClass) new PlayMainWindowClass( tgame, 125), self.Game.ScreenWidth - 220, 120, 220, self.Game.ScreenHeight - 370);
        self.WEditMenu = self.AddWindow((WindowClass) new EditOptionsWindowClass( tgame, self.OwnBackground, 0, 0), 0, 0, self.Game.ScreenWidth - 220, 100);
        self.Wmiddle = self.AddWindow((WindowClass) new MapWindowClass( tgame, 370, 220), 0, 120, self.Game.ScreenWidth - 220, self.Game.ScreenHeight - 370);
        self.worder = self.AddWindow((WindowClass) new OrderWindowClass( tgame), 0, self.Game.ScreenHeight - 250, self.Game.ScreenWidth, 40);
        self.wup = self.AddWindow((WindowClass) new InfoWindowClass( tgame), 0, 100, self.Game.ScreenWidth, 20);
      }
      else
      {
        self.wdown = self.AddWindow((WindowClass) new PlayExtraWindowClass( tgame, self.OwnBackground, 0, self.Game.ScreenHeight - 210), 0, self.Game.ScreenHeight - 210, self.Game.ScreenWidth, 210);
        self.wleft = self.AddWindow((WindowClass) new PlayMainWindowClass( tgame, 125), self.Game.ScreenWidth - 220, 120, 220, self.Game.ScreenHeight - 370);
        self.WEditMenu = self.AddWindow((WindowClass) new EditOptionsWindowClass( tgame, self.OwnBackground, 0, 0), 0, 0, self.Game.ScreenWidth - 220, 100);
        self.Wmiddle = self.AddWindow((WindowClass) new MapWindowClass( tgame, 370, 220), 0, 120, self.Game.ScreenWidth - 220, self.Game.ScreenHeight - 370);
        self.worder = self.AddWindow((WindowClass) new OrderWindowClass( tgame), 0, self.Game.ScreenHeight - 250, self.Game.ScreenWidth, 40);
        self.wup = self.AddWindow((WindowClass) new InfoWindowClass( tgame), 0, 100, self.Game.ScreenWidth, 20);
      }
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  self.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        windowReturnClass: WindowReturnClass = self.WindowList[windowCounter].handleTimer();
        if (!self.WindowFlag[windowCounter] & windowReturnClass.Flag)
          self.WindowFlag[windowCounter] = windowReturnClass.Flag;
        if (windowReturnClass.Flag)
        {
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 =  windowReturnClass.Counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (windowReturnClass.CommandType[index] == 5)
              {
                screenReturnClass.OpenPopUp = true;
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index];
                return screenReturnClass;
              }
              if (windowReturnClass.CommandType[index] == 6)
              {
                screenReturnClass.ClosePopUp = true;
                screenReturnClass.NewScreen = 0;
                return screenReturnClass;
              }
            }
          }
          if (windowReturnClass.Flag)
            screenReturnClass.flag = windowReturnClass.Flag;
        }
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseClick(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (x > self.Game.ScreenWidth - 52 & x < self.Game.ScreenWidth - 28 & y < 25)
        self.Game.FormRef.WindowState = FormWindowState.Minimized;
      if (x > self.Game.ScreenWidth - 28 & x < self.Game.ScreenWidth - 4 & y < 25 && Interaction.MsgBox( "Are you sure you want to exit the editor?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
      {
        self.Game.Data = DataClass::new();
        self.Game.EditObj = new EditClass(self.Game.AppPath + "editobj.txt");
        self.Game.EditObj.ShowInitialMenu = true;
        screenReturnClass.NewScreen = self.Game.ModIntroType != 0 ? 12 : 1;
        return screenReturnClass;
      }
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  self.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        if (x > self.WindowX[index1] & x < self.WindowX[index1] + self.WindowW[index1] && y > self.WindowY[index1] & y < self.WindowY[index1] + self.WindowH[index1])
        {
          windowReturnClass: WindowReturnClass = self.WindowList[index1].HandleMouseClick(x - self.WindowX[index1], y - self.WindowY[index1], b);
          self.WindowFlag[index1] = windowReturnClass.Flag;
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 =  windowReturnClass.Counter;
            for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
            {
              if (windowReturnClass.CommandType[index2] == 3)
              {
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index2];
                return screenReturnClass;
              }
              if (windowReturnClass.CommandType[index2] == 5)
              {
                screenReturnClass.OpenPopUp = true;
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index2];
                return screenReturnClass;
              }
              if (windowReturnClass.CommandType[index2] == 6)
              {
                screenReturnClass.ClosePopUp = true;
                screenReturnClass.NewScreen = 0;
                return screenReturnClass;
              }
              if (windowReturnClass.CommandType[index2] == 1)
              {
                if (windowReturnClass.CommandData[index2] == 4 | windowReturnClass.CommandData[index2] == 3 && self.Wmiddle > -1)
                {
                  self.RemoveWindow(self.Wmiddle);
                  self.Wmiddle = -1;
                }
                if (windowReturnClass.CommandData[index2] == 2 && self.wleft > -1)
                {
                  self.RemoveWindow(self.wleft);
                  self.wleft = -1;
                }
                if (windowReturnClass.CommandData[index2] == 5 && self.wdown > -1)
                {
                  self.RemoveWindow(self.wdown);
                  self.wdown = -1;
                }
                if (windowReturnClass.CommandData[index2] == 13 && self.WEditMenu > -1)
                {
                  self.RemoveWindow(self.WEditMenu);
                  self.WEditMenu = -1;
                }
                if (windowReturnClass.CommandData[index2] == 7 && self.worder > -1)
                {
                  self.RemoveWindow(self.worder);
                  self.worder = -1;
                }
              }
              if (windowReturnClass.CommandType[index2] == 4)
              {
                if (windowReturnClass.CommandData[index2] == 13)
                {
                  self.RemoveWindow(self.WEditMenu);
                  self.WEditMenu = self.AddWindow((WindowClass) new EditOptionsWindowClass( self.Game), 0, 0, self.Game.ScreenWidth, 100);
                }
                if (windowReturnClass.CommandData[index2] == 18 && self.wleft > -1)
                {
                  self.WindowList[self.GetNr(self.wleft)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.wleft)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 20 && self.wdown > -1)
                {
                  self.WindowList[self.GetNr(self.wdown)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.wdown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 12 && self.Wmiddle > -1)
                {
                  self.WindowList[self.GetNr(self.Wmiddle)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.Wmiddle)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 44 && self.worder > -1)
                {
                  self.WindowList[self.GetNr(self.worder)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.worder)] = true;
                }
              }
              if (windowReturnClass.CommandType[index2] == 2)
              {
                if (windowReturnClass.CommandData[index2] == 11)
                  self.Wmiddle = self.AddWindow((WindowClass) new LandscapeTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 12)
                  self.Wmiddle = self.AddWindow((WindowClass) new MapWindowClass( self.Game, 370, 220), 0, 120, self.Game.ScreenWidth - 220, self.Game.ScreenHeight - 370);
                if (windowReturnClass.CommandData[index2] == 13)
                  self.WEditMenu = self.AddWindow((WindowClass) new EditOptionsWindowClass( self.Game), 0, 0, self.Game.ScreenWidth - 220, 100);
                if (windowReturnClass.CommandData[index2] == 14)
                  self.Wmiddle = self.AddWindow((WindowClass) new RoadTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 93)
                  self.Wmiddle = self.AddWindow((WindowClass) new LibraryWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 60)
                  self.Wmiddle = self.AddWindow((WindowClass) new StringListWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 44)
                  self.worder = self.AddWindow((WindowClass) new OrderWindowClass( self.Game), 0, self.Game.ScreenHeight - 250, self.Game.ScreenWidth, 40);
                if (windowReturnClass.CommandData[index2] == 15)
                  self.Wmiddle = self.AddWindow((WindowClass) new RegimeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 16)
                  self.Wmiddle = self.AddWindow((WindowClass) new SFTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 17)
                  self.Wmiddle = self.AddWindow((WindowClass) new EditUnitWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 86)
                  self.Wmiddle = self.AddWindow((WindowClass) new EditHisWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 21)
                  self.Wmiddle = self.AddWindow((WindowClass) new LocTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 58)
                  self.Wmiddle = self.AddWindow((WindowClass) new ConnectionWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 33)
                  self.Wmiddle = self.AddWindow((WindowClass) new RiverTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 57)
                  self.Wmiddle = self.AddWindow((WindowClass) new ActionCardWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 34)
                  self.Wmiddle = self.AddWindow((WindowClass) new BridgeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 20)
                  self.wdown =  self.Game.Data.RuleVar[839] != 1.0 ? self.AddWindow((WindowClass) new PlayExtraWindowClass( DrawMod.TGame, self.OwnBackground, 0, self.Game.ScreenHeight - 210), 0, self.Game.ScreenHeight - 210, self.Game.ScreenWidth, 210) : self.AddWindow((WindowClass) new PlayExtraWindowClass2( DrawMod.TGame, self.OwnBackground, 0, self.Game.ScreenHeight - 210), 0, self.Game.ScreenHeight - 210, self.Game.ScreenWidth, 222);
                if (windowReturnClass.CommandData[index2] == 22)
                  self.Wmiddle = self.AddWindow((WindowClass) new PeopleWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 18)
                  self.wleft = self.AddWindow((WindowClass) new PlayMainWindowClass( self.Game, 125), self.Game.ScreenWidth - 220, 120, 220, self.Game.ScreenHeight - 370);
                if (windowReturnClass.CommandData[index2] == 23)
                  self.Wmiddle = self.AddWindow((WindowClass) new GeneralWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 24)
                  self.Wmiddle = self.AddWindow((WindowClass) new ItemTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 37)
                  self.Wmiddle = self.AddWindow((WindowClass) new ResearchWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 61)
                  self.wdown = self.AddWindow((WindowClass) new OfficerPoolWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 25)
                  self.wdown = self.AddWindow((WindowClass) new ProdWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
                if (windowReturnClass.CommandData[index2] == 45)
                  self.wdown = self.AddWindow((WindowClass) new PrefsWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
                if (windowReturnClass.CommandData[index2] == 28)
                  self.wdown = self.AddWindow((WindowClass) new NewUnitWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
                if (windowReturnClass.CommandData[index2] == 26)
                  self.Wmiddle = self.AddWindow((WindowClass) new LocWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass.CommandData[index2] == 30)
                  self.wdown = self.AddWindow((WindowClass) new TransferWindowClass( self.Game), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
              }
            }
          }
          screenReturnClass.flag = windowReturnClass.Flag;
          return screenReturnClass;
        }
      }
      screenReturnClass.flag = false;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseMove(x: i32, y: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (x > self.Game.ScreenWidth - 52 & x < self.Game.ScreenWidth - 28 & y < 25)
      {
        if (!self.doMinimize)
        {
          self.doMinimize = true;
          screenReturnClass.flag = true;
        }
      }
      else if (self.doMinimize)
      {
        self.doMinimize = false;
        screenReturnClass.flag = true;
      }
      if (x > self.Game.ScreenWidth - 28 & x < self.Game.ScreenWidth - 4 & y < 25)
      {
        if (!self.doQuit)
        {
          self.doQuit = true;
          screenReturnClass.flag = true;
        }
      }
      else if (self.doQuit)
      {
        self.doQuit = false;
        screenReturnClass.flag = true;
      }
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  self.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        if (x > self.WindowX[index1] & x < self.WindowX[index1] + self.WindowW[index1] & y > self.WindowY[index1] & y < self.WindowY[index1] + self.WindowH[index1])
        {
          windowReturnClass: WindowReturnClass = self.WindowList[index1].HandleMouseMove(x - self.WindowX[index1], y - self.WindowY[index1]);
          self.WindowFlag[index1] = windowReturnClass.Flag;
          if (windowReturnClass.Overlay | self.LastOverlayWindow > 0)
          {
            if (self.LastOverlayWindow > 0 & self.LastOverlayWindow != self.WindowID[index1])
              self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
            if (windowReturnClass.Overlay)
              self.LastOverlayWindow = self.WindowID[index1];
          }
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 =  windowReturnClass.Counter;
            for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
            {
              if (windowReturnClass.CommandType[index2] == 4 && windowReturnClass.CommandData[index2] == 29)
              {
                self.WindowList[self.GetNr(self.wup)].DoRefresh();
                self.WindowFlag[self.GetNr(self.wup)] = true;
                screenReturnClass.flag = true;
                return screenReturnClass;
              }
            }
          }
          if (!screenReturnClass.flag)
            screenReturnClass.flag = windowReturnClass.Flag;
          return screenReturnClass;
        }
      }
      if (self.LastOverlayWindow > 0)
      {
        self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
        screenReturnClass.flag = true;
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleKeyPress(nr: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter > -1)
      {
        let mut windowCounter: i32 =  self.WindowCounter;
        for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
        {
          windowReturnClass2: WindowReturnClass = self.WindowList[index1].HandleKeyPress(nr);
          if (!self.WindowFlag[index1])
            self.WindowFlag[index1] = windowReturnClass2.Flag;
          if (!screenReturnClass.flag)
            screenReturnClass.flag = windowReturnClass2.Flag;
          if (windowReturnClass2.Counter > -1)
          {
            let mut counter: i32 =  windowReturnClass2.Counter;
            for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
            {
              if (windowReturnClass2.CommandType[index2] == 3)
              {
                screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
                return screenReturnClass;
              }
              if (windowReturnClass2.CommandType[index2] == 5)
              {
                screenReturnClass.OpenPopUp = true;
                screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
                return screenReturnClass;
              }
              if (windowReturnClass2.CommandType[index2] == 6)
              {
                screenReturnClass.ClosePopUp = true;
                screenReturnClass.NewScreen = 0;
                return screenReturnClass;
              }
              if (windowReturnClass2.CommandType[index2] == 1)
              {
                if (windowReturnClass2.CommandData[index2] == 4 | windowReturnClass2.CommandData[index2] == 3 && self.Wmiddle > -1)
                {
                  self.RemoveWindow(self.Wmiddle);
                  self.Wmiddle = -1;
                }
                if (windowReturnClass2.CommandData[index2] == 2 && self.wleft > -1)
                {
                  self.RemoveWindow(self.wleft);
                  self.wleft = -1;
                }
                if (windowReturnClass2.CommandData[index2] == 5 && self.wdown > -1)
                {
                  self.RemoveWindow(self.wdown);
                  self.wdown = -1;
                }
                if (windowReturnClass2.CommandData[index2] == 13 && self.WEditMenu > -1)
                {
                  self.RemoveWindow(self.WEditMenu);
                  self.WEditMenu = -1;
                }
                if (windowReturnClass2.CommandData[index2] == 7 && self.worder > -1)
                {
                  self.RemoveWindow(self.worder);
                  self.worder = -1;
                }
              }
              if (windowReturnClass2.CommandType[index2] == 4)
              {
                if (windowReturnClass2.CommandData[index2] == 13)
                {
                  self.RemoveWindow(self.WEditMenu);
                  self.WEditMenu = self.AddWindow((WindowClass) new EditOptionsWindowClass( self.Game), 0, 0, self.Game.ScreenWidth, 100);
                }
                if (windowReturnClass2.CommandData[index2] == 18 && self.wleft > -1)
                {
                  self.WindowList[self.GetNr(self.wleft)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.wleft)] = true;
                }
                if (windowReturnClass2.CommandData[index2] == 20 && self.wdown > -1)
                {
                  self.WindowList[self.GetNr(self.wdown)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.wdown)] = true;
                }
                if (windowReturnClass2.CommandData[index2] == 12 && self.Wmiddle > -1)
                {
                  self.WindowList[self.GetNr(self.Wmiddle)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.Wmiddle)] = true;
                }
                if (windowReturnClass2.CommandData[index2] == 44 && self.worder > -1)
                {
                  self.WindowList[self.GetNr(self.worder)].DoRefresh();
                  self.WindowFlag[self.GetNr(self.worder)] = true;
                }
              }
              if (windowReturnClass2.CommandType[index2] == 2)
              {
                if (windowReturnClass2.CommandData[index2] == 11)
                  self.Wmiddle = self.AddWindow((WindowClass) new LandscapeTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 12)
                  self.Wmiddle = self.AddWindow((WindowClass) new MapWindowClass( self.Game, 370, 220), 0, 120, self.Game.ScreenWidth - 220, self.Game.ScreenHeight - 370);
                if (windowReturnClass2.CommandData[index2] == 13)
                  self.WEditMenu = self.AddWindow((WindowClass) new EditOptionsWindowClass( self.Game), 0, 0, self.Game.ScreenWidth - 220, 100);
                if (windowReturnClass2.CommandData[index2] == 14)
                  self.Wmiddle = self.AddWindow((WindowClass) new RoadTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 93)
                  self.Wmiddle = self.AddWindow((WindowClass) new LibraryWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 60)
                  self.Wmiddle = self.AddWindow((WindowClass) new StringListWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 44)
                  self.worder = self.AddWindow((WindowClass) new OrderWindowClass( self.Game), 0, self.Game.ScreenHeight - 250, self.Game.ScreenWidth, 40);
                if (windowReturnClass2.CommandData[index2] == 15)
                  self.Wmiddle = self.AddWindow((WindowClass) new RegimeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 16)
                  self.Wmiddle = self.AddWindow((WindowClass) new SFTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 17)
                  self.Wmiddle = self.AddWindow((WindowClass) new EditUnitWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 21)
                  self.Wmiddle = self.AddWindow((WindowClass) new LocTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 58)
                  self.Wmiddle = self.AddWindow((WindowClass) new ConnectionWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 33)
                  self.Wmiddle = self.AddWindow((WindowClass) new RiverTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 57)
                  self.Wmiddle = self.AddWindow((WindowClass) new ActionCardWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 34)
                  self.Wmiddle = self.AddWindow((WindowClass) new BridgeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 20)
                  self.wdown =  self.Game.Data.RuleVar[839] != 1.0 ? self.AddWindow((WindowClass) new PlayExtraWindowClass( DrawMod.TGame, self.OwnBackground, 0, self.Game.ScreenHeight - 210), 0, self.Game.ScreenHeight - 210, self.Game.ScreenWidth, 210) : self.AddWindow((WindowClass) new PlayExtraWindowClass2( DrawMod.TGame, self.OwnBackground, 0, self.Game.ScreenHeight - 210), 0, self.Game.ScreenHeight - 210, self.Game.ScreenWidth, 222);
                if (windowReturnClass2.CommandData[index2] == 22)
                  self.Wmiddle = self.AddWindow((WindowClass) new PeopleWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 18)
                  self.wleft = self.AddWindow((WindowClass) new PlayMainWindowClass( self.Game, 125), self.Game.ScreenWidth - 220, 120, 220, self.Game.ScreenHeight - 370);
                if (windowReturnClass2.CommandData[index2] == 23)
                  self.Wmiddle = self.AddWindow((WindowClass) new GeneralWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 24)
                  self.Wmiddle = self.AddWindow((WindowClass) new ItemTypeWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 37)
                  self.Wmiddle = self.AddWindow((WindowClass) new ResearchWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 61)
                  self.wdown = self.AddWindow((WindowClass) new OfficerPoolWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass2.CommandData[index2] == 25)
                  self.wdown = self.AddWindow((WindowClass) new ProdWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
                if (windowReturnClass2.CommandData[index2] == 45)
                  self.wdown = self.AddWindow((WindowClass) new PrefsWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
                if (windowReturnClass2.CommandData[index2] == 28)
                  self.wdown = self.AddWindow((WindowClass) new NewUnitWindowClass( self.Game, self.OwnBackground, self.OffSetX, self.Game.ScreenHeight - 200), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
                if (windowReturnClass2.CommandData[index2] == 26)
                  self.Wmiddle = self.AddWindow((WindowClass) new LocWindowClass( self.Game), 0, 120, self.Game.ScreenWidth, self.Game.ScreenHeight - 120);
                if (windowReturnClass2.CommandData[index2] == 30)
                  self.wdown = self.AddWindow((WindowClass) new TransferWindowClass( self.Game), self.OffSetX, self.Game.ScreenHeight - 200, 1000, 200);
              }
            }
          }
        }
        screenReturnClass.flag = true;
        return screenReturnClass;
      }
      screenReturnClass.flag = false;
      return screenReturnClass;
    }
  }
}
