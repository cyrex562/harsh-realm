// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RandomScreenClass2
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
  pub class RandomScreenClass2 : ScreenClass
  {
     WMap: i32;
     WTop: i32;
     WTabs: i32;
     WBottom: i32;
     OffSetX: i32;

    pub RandomScreenClass2( tgame: GameClass, tformref: Form1)
      : base( tgame, -3, tformref)
    {
      self.Game.HandyFunctionsObj.RedimTempValue3(9999);
      if ( self.Game.Data.RuleVar[701] > 0.0)
        self.Game.HandyFunctionsObj.RedimTempValue4(9999);
      self.OffSetX =  Math.Round( (self.Game.ScreenWidth - 1280) / 2.0);
      self.Game.SelectX = -1;
      self.Game.SelectY = -1;
      self.Game.EditObj.interfaceCue = 0;
      if (self.Game.Data.UseAI == 1)
      {
        if (Information.IsNothing( self.Game.NewAIObj))
          self.Game.NewAIObj = new NewAIClass(self.Game);
        self.Game.DC2AIObj = (DC2AIClass) null;
        self.Game.AIObj = (AIClass) null;
      }
      else if (self.Game.Data.UseAI == 2)
      {
        if (Information.IsNothing( self.Game.DC2AIObj))
          self.Game.DC2AIObj = new DC2AIClass(self.Game);
        self.Game.NewAIObj = (NewAIClass) null;
        self.Game.AIObj = (AIClass) null;
      }
      if ( self.Game.Data.RuleVar[442] > 0.0)
        self.Game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( self.Game.Data.RuleVar[442]));
      self.Game.EditObj.inRandomScreen = true;
      MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, 32, 0);
      let mut screenWidth: i32 = self.Game.ScreenWidth;
      let mut h: i32 = self.Game.ScreenHeight - 32;
      Rectangle rectangle1 = Rectangle::new(0, 75, self.Game.ScreenWidth, self.Game.ScreenHeight - 107);
      let mut tShowRectangle: &Rectangle = &rectangle1
      self.WMap = self.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
      self.WBottom = self.AddWindow((WindowClass) new RandomBottomClass( tgame), 0, self.Game.ScreenHeight - 32, self.Game.ScreenWidth, 32);
      self.Game.EditObj.dssLastDominant = 1002;
      self.Game.EditObj.SetViewMode2 = 101;
      Rectangle rectForTab = DrawMod.GetRectForTab(self.Game.EditObj.SetViewMode2);
       let mut local1: GameClass =  tgame;
       WindowClass local2 =  self.WindowList[self.GetNr(self.WMap)];
      rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      let mut rectangle2: &Rectangle = &rectangle1
       Rectangle local3 =  rectangle2;
      let mut trect: &Rectangle = &rectForTab
      self.WTabs = self.AddWindow((WindowClass) new TabManagementWindowClass2( local1,  local2,  local3, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
       let mut local4: GameClass =  tgame;
       WindowClass local5 =  self.WindowList[self.GetNr(self.WMap)];
      rectangle2 = Rectangle::new(0, 0, self.Game.ScreenWidth, 75);
      rectangle1 = rectangle2;
       Rectangle local6 =  rectangle1;
      self.WTop = self.AddWindow((WindowClass) new RandomTopClass( local4,  local5,  local6), 0, 0, self.Game.ScreenWidth, 75);
    }

    pub ScreenReturnClass HandleWR(
      wr: WindowReturnClass,
      bool setflag,
       ScreenReturnClass obj)
    {
      if (wr.Counter > -1)
      {
        let mut counter: i32 = wr.Counter;
        for (let mut index: i32 = 0; index <= counter; index += 1)
        {
          if (wr.CommandType[index] == 3)
          {
            self.Game.EditObj.dssLastDominant = -1;
            obj.NewScreen = wr.CommandData[index];
            return obj;
          }
          if (wr.CommandType[index] == 5)
          {
            obj.OpenPopUp = true;
            obj.NewScreen = wr.CommandData[index];
            return obj;
          }
          if (wr.CommandType[index] == 6)
          {
            obj.ClosePopUp = true;
            obj.NewScreen = 0;
            return obj;
          }
          if (wr.CommandType[index] == 4)
          {
            if ((wr.CommandData[index] == 114 | wr.CommandData[index] == 67) & self.WTop > 0 & !self.WindowFlag[self.GetNr(self.WTop)])
            {
              self.WindowList[self.GetNr(self.WTop)].DoRefresh();
              self.WindowFlag[self.GetNr(self.WTop)] = true;
            }
            if (wr.CommandData[index] == 115 & self.WMap > 0)
            {
              self.WindowList[self.GetNr(self.WMap)].DoRefresh();
              self.WindowFlag[self.GetNr(self.WMap)] = true;
            }
            if (wr.CommandData[index] == 12 & self.WMap > 0)
            {
              self.WindowList[self.GetNr(self.WMap)].DoRefresh();
              self.WindowFlag[self.GetNr(self.WMap)] = true;
            }
            if (wr.CommandData[index] == 9 & self.WTabs > 0)
            {
              self.WindowList[self.GetNr(self.WTabs)].DoRefresh();
              self.WindowFlag[self.GetNr(self.WTabs)] = true;
            }
            if (wr.CommandData[index] == 116 & self.WBottom > 0)
            {
              self.WindowList[self.GetNr(self.WBottom)].DoRefresh();
              self.WindowFlag[self.GetNr(self.WBottom)] = true;
            }
          }
          if (wr.CommandType[index] == 7)
            self.WindowFlag[self.GetNr(self.WMap)] = true;
          if (wr.CommandType[index] == 1)
          {
            if (wr.CommandData[index] == 12)
            {
              self.RemoveWindow(self.WMap);
              self.WMap = 0;
            }
            if (wr.CommandData[index] == 114)
            {
              self.RemoveWindow(self.WTop);
              self.WTop = 0;
            }
            if (wr.CommandData[index] == 116)
            {
              self.RemoveWindow(self.WBottom);
              self.WBottom = 0;
            }
            if (wr.CommandData[index] == 115)
            {
              self.RemoveWindow(self.WMap);
              self.WMap = 0;
            }
            if (wr.CommandData[index] == 9)
            {
              self.RemoveWindow(self.WTabs);
              self.WTabs = 0;
            }
          }
          if (wr.CommandType[index] == 2)
          {
            Rectangle rectangle1;
            if (wr.CommandData[index] == 12)
            {
              MapWindowClass2 tmpWindow = new MapWindowClass2( DrawMod.TGame, 32, 0);
              let mut screenWidth: i32 = self.Game.ScreenWidth;
              let mut h: i32 = self.Game.ScreenHeight - 32;
              rectangle1 = Rectangle::new(0, 75, self.Game.ScreenWidth, self.Game.ScreenHeight - 107);
              let mut tShowRectangle: &Rectangle = &rectangle1
              self.WMap = self.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
              if (self.WTop > 0)
                self.WindowList[self.GetNr(self.WTop)].LowerWindow = self.WindowList[self.GetNr(self.WMap)];
              if (self.WBottom > 0)
                self.WindowList[self.GetNr(self.WBottom)].LowerWindow = self.WindowList[self.GetNr(self.WMap)];
              if (self.WTabs > 0)
                self.WindowList[self.GetNr(self.WTabs)].LowerWindow = self.WindowList[self.GetNr(self.WMap)];
            }
            Rectangle rectangle2;
            if (wr.CommandData[index] == 114)
            {
               let mut local1: GameClass =  DrawMod.TGame;
               WindowClass local2 =  self.WindowList[self.GetNr(self.WMap)];
              rectangle1 = Rectangle::new(0, 0, self.Game.ScreenWidth, 75);
              rectangle2 = rectangle1;
               Rectangle local3 =  rectangle2;
              self.WTop = self.AddWindow((WindowClass) new RandomTopClass( local1,  local2,  local3), 0, 0, self.Game.ScreenWidth, 75);
            }
            if (wr.CommandData[index] == 116)
              self.WBottom = self.AddWindow((WindowClass) new RandomBottomClass( DrawMod.TGame), 0, self.Game.ScreenHeight - 32, self.Game.ScreenWidth, 32);
            Rectangle rectForTab;
            if (wr.CommandData[index] == 113)
            {
              rectForTab = DrawMod.GetRectForTab(self.Game.EditObj.SetViewMode2);
               let mut local4: GameClass =  DrawMod.TGame;
               WindowClass local5 =  self.WindowList[self.GetNr(self.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local6 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              self.WTabs = self.AddWindow((WindowClass) new TabManagementWindowClass2( local4,  local5,  local6, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 76)
            {
              rectForTab = DrawMod.GetRectForTab(7);
               let mut local7: GameClass =  DrawMod.TGame;
               WindowClass local8 =  self.WindowList[self.GetNr(self.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local9 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              self.WTabs = self.AddWindow((WindowClass) new TabMiniMapWindowClass2( local7,  local8,  local9, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
          }
        }
      }
      if (setflag)
        obj.flag = wr.Flag;
      else if (wr.Flag)
        obj.flag = wr.Flag;
      ScreenReturnClass screenReturnClass;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseClick(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (x > self.Game.ScreenWidth - 52 & x < self.Game.ScreenWidth - 28 & y < 25)
        self.Game.FormRef.WindowState = FormWindowState.Minimized;
      if (x > self.Game.ScreenWidth - 28 & x < self.Game.ScreenWidth - 4 & y < 25 && Interaction.MsgBox( "Are you sure you want to exit your current game?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
      {
        self.Game.Data = DataClass::new();
        self.Game.EditObj = new EditClass(self.Game.AppPath + "editobj.txt");
        if (self.Game.Data.UseAI == 1)
        {
          if (Information.IsNothing( self.Game.NewAIObj))
            self.Game.NewAIObj = new NewAIClass(self.Game);
          self.Game.NewAIObj.LastRegime = -1;
        }
        self.Game.EditObj.ShowInitialMenu = true;
        screenReturnClass.NewScreen = 12;
        return screenReturnClass;
      }
      if (self.WindowCounter > -1)
      {
        for (let mut windowCounter: i32 = self.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Operators.CompareString(self.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 && x > self.WindowX[windowCounter] & x < self.WindowX[windowCounter] + self.WindowW[windowCounter] && y > self.WindowY[windowCounter] & y < self.WindowY[windowCounter] + self.WindowH[windowCounter])
          {
            wr: WindowReturnClass = self.WindowList[windowCounter].HandleMouseClick(x - self.WindowX[windowCounter], y - self.WindowY[windowCounter], b);
            self.WindowFlag[windowCounter] = wr.Flag;
            if (wr.Flag)
            {
              self.HandleWR(wr, true,  screenReturnClass);
              return screenReturnClass;
            }
            if (wr.NoMouseClickBelow)
            {
              screenReturnClass.flag = false;
              return screenReturnClass;
            }
          }
        }
        for (let mut windowCounter: i32 = self.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Operators.CompareString(self.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 && x > self.WindowX[windowCounter] & x < self.WindowX[windowCounter] + self.WindowW[windowCounter] && y > self.WindowY[windowCounter] & y < self.WindowY[windowCounter] + self.WindowH[windowCounter])
          {
            wr: WindowReturnClass = self.WindowList[windowCounter].HandleMouseClick(x - self.WindowX[windowCounter], y - self.WindowY[windowCounter], b);
            self.WindowFlag[windowCounter] = wr.Flag;
            if (wr.Flag)
            {
              self.HandleWR(wr, true,  screenReturnClass);
              return screenReturnClass;
            }
            if (wr.NoMouseClickBelow)
            {
              screenReturnClass.flag = false;
              return screenReturnClass;
            }
          }
        }
        screenReturnClass.flag = false;
        return screenReturnClass;
      }
      screenReturnClass.flag = false;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseUp(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter > -1)
      {
        for (let mut windowCounter: i32 = self.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (x > self.WindowX[windowCounter] & x < self.WindowX[windowCounter] + self.WindowW[windowCounter] && y > self.WindowY[windowCounter] & y < self.WindowY[windowCounter] + self.WindowH[windowCounter])
          {
            wr: WindowReturnClass = self.WindowList[windowCounter].HandleMouseUp(x - self.WindowX[windowCounter], y - self.WindowY[windowCounter], b);
            self.WindowFlag[windowCounter] = wr.Flag;
            if (wr.Flag)
            {
              self.HandleWR(wr, true,  screenReturnClass);
              return screenReturnClass;
            }
            if (wr.NoMouseClickBelow)
            {
              screenReturnClass.flag = false;
              return screenReturnClass;
            }
          }
        }
        screenReturnClass.flag = false;
        return screenReturnClass;
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
      let mut windowCounter1: i32 = self.WindowCounter;
      windowReturnClass: WindowReturnClass;
      for (let mut index: i32 = 0; index <= windowCounter1; index += 1)
      {
        if (Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 && x >= self.WindowX[index] & x <= self.WindowX[index] + self.WindowW[index] & y >= self.WindowY[index] & y <= self.WindowY[index] + self.WindowH[index])
        {
          windowReturnClass = self.WindowList[index].HandleMouseMove(x - self.WindowX[index], y - self.WindowY[index]);
          self.WindowFlag[index] = windowReturnClass.Flag;
          if (windowReturnClass.Counter > -1)
          {
            self.HandleWR(windowReturnClass, true,  screenReturnClass);
            windowReturnClass.Flag = true;
          }
          if (windowReturnClass.Overlay | self.LastOverlayWindow > 0)
          {
            if (self.LastOverlayWindow > 0 & self.LastOverlayWindow != self.WindowID[index])
              self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
            if (windowReturnClass.Overlay)
              self.LastOverlayWindow = self.WindowID[index];
          }
          if (!screenReturnClass.flag)
            screenReturnClass.flag = windowReturnClass.Flag;
          if (screenReturnClass.flag | windowReturnClass.Overlay || windowReturnClass.NoMouseClickBelow)
            return screenReturnClass;
        }
      }
      num: i32;
      if (Information.IsNothing( windowReturnClass))
        num = 1;
      else if (!windowReturnClass.NoMouseClickBelow)
        num = 1;
      if (num == 1)
      {
        let mut windowCounter2: i32 = self.WindowCounter;
        for (let mut index: i32 = 0; index <= windowCounter2; index += 1)
        {
          if (Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 && x >= self.WindowX[index] & x <= self.WindowX[index] + self.WindowW[index] & y >= self.WindowY[index] & y <= self.WindowY[index] + self.WindowH[index])
          {
            wr: WindowReturnClass = self.WindowList[index].HandleMouseMove(x - self.WindowX[index], y - self.WindowY[index]);
            self.WindowFlag[index] = wr.Flag;
            if (wr.Counter > -1)
            {
              self.HandleWR(wr, true,  screenReturnClass);
              wr.Flag = true;
            }
            if (wr.Overlay | self.LastOverlayWindow > 0)
            {
              if (self.LastOverlayWindow > 0 & self.LastOverlayWindow != self.WindowID[index])
                self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
              if (wr.Overlay)
                self.LastOverlayWindow = self.WindowID[index];
            }
            if (!screenReturnClass.flag)
              screenReturnClass.flag = wr.Flag;
            if (screenReturnClass.flag)
              return screenReturnClass;
          }
        }
      }
      if (self.LastOverlayWindow > 0)
      {
        self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
        screenReturnClass.flag = true;
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleTimerWheel(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass1 = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
      {
        ScreenReturnClass screenReturnClass2;
        return screenReturnClass2;
      }
      let mut num: i32 = 1;
      do
      {
        let mut windowCounter1: i32 = self.WindowCounter;
        for (let mut index1: i32 = 0; index1 <= windowCounter1; index1 += 1)
        {
          bool flag;
          if (x > self.WindowX[index1] & y > self.WindowY[index1] & x < self.WindowX[index1] + self.WindowW[index1] & y < self.WindowY[index1] + self.WindowH[index1])
            flag = true;
          if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.RandomBottomClass", false) == 0)
          {
            if (num == 2)
            {
              let mut windowCounter2: i32 = self.WindowCounter;
              for (let mut index2: i32 = 0; index2 <= windowCounter2; index2 += 1)
              {
                if (Operators.CompareString(self.WindowList[index2].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 && x > self.WindowX[index2] & y > self.WindowY[index2] & x < self.WindowX[index2] + self.WindowW[index2] & y < self.WindowY[index2] + self.WindowH[index2])
                  flag = true;
              }
            }
            else
              flag = false;
          }
          else if (num == 2)
            flag = false;
          if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0)
            flag = false;
          if (flag)
          {
            wr: WindowReturnClass = self.WindowList[index1].handleTimerWheel(x - self.WindowX[index1], y - self.WindowY[index1]);
            if (wr.Flag)
            {
              screenReturnClass1.flag = true;
              self.WindowFlag[index1] = true;
              if (wr.Counter > -1)
                self.HandleWR(wr, false,  screenReturnClass1);
              return screenReturnClass1;
            }
            self.Game.EditObj.MouseWheel = 0;
            screenReturnClass1.flag = false;
            return screenReturnClass1;
          }
        }
        num += 1;
      }
      while (num <= 2);
      self.Game.EditObj.MouseWheel = 0;
      screenReturnClass1.flag = false;
      return screenReturnClass1;
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      bool flag = false;
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 = self.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        if (Operators.CompareString(self.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0)
        {
          wr: WindowReturnClass = self.WindowList[windowCounter].handleTimer();
          if (!self.WindowFlag[windowCounter] & wr.Flag)
            self.WindowFlag[windowCounter] = wr.Flag;
          if (wr.Flag)
          {
            if (wr.Counter > -1)
              self.HandleWR(wr, false,  screenReturnClass);
            if (wr.Flag)
              flag = true;
          }
        }
      }
      for (let mut windowCounter: i32 = self.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        if (Operators.CompareString(self.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0)
        {
          wr: WindowReturnClass = self.WindowList[windowCounter].handleTimer();
          if (!self.WindowFlag[windowCounter] & wr.Flag)
            self.WindowFlag[windowCounter] = wr.Flag;
          if (wr.Flag)
          {
            if (wr.Counter > -1)
              self.HandleWR(wr, false,  screenReturnClass);
            if (wr.Flag)
              flag = true;
          }
        }
      }
      if (flag)
        screenReturnClass.flag = true;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleKeyPress(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 = self.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        wr: WindowReturnClass = self.WindowList[windowCounter].HandleKeyPress(nr);
        if (!self.WindowFlag[windowCounter] & wr.Flag)
          self.WindowFlag[windowCounter] = wr.Flag;
        if (wr.Counter > -1)
          self.HandleWR(wr, false,  screenReturnClass);
        if (wr.Flag)
        {
          screenReturnClass.flag = true;
          return screenReturnClass;
        }
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }
  }
}
