// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoryScreenClass2
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
  pub class HistoryScreenClass2 : ScreenClass
  {
     WMap: i32;
     WPlay: i32;
     WOrder: i32;
     WRes: i32;
     WTabs: i32;
     OffSetX: i32;

    pub HistoryScreenClass2( tgame: GameClass, tformref: Form1)
      : base( tgame, -3, tformref)
    {
      this.OffSetX =  Math.Round( (this.Game.ScreenWidth - 1024) / 2.0);
      this.Game.HandyFunctionsObj.SetGameColors();
      this.WPlay = this.AddWindow((WindowClass) new HistoryWindowClass2( tgame), 0, this.Game.ScreenHeight - 222, this.Game.ScreenWidth, 222);
      Rectangle rectangle1;
      if (this.Game.Data.Product == 6)
      {
        MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, 222, 0, this.Game.EditObj.Zoom);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut h: i32 =  this.Game.ScreenHeight - 222;
        rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 387);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
      }
      else
      {
        MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, 222, 0, 0);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut h: i32 =  this.Game.ScreenHeight - 222;
        rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 387);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
      }
       let mut local1: GameClass =  tgame;
       WindowClass local2 =  this.WindowList[this.GetNr(this.WMap)];
      rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
      let mut rectangle2: &Rectangle = &rectangle1
       Rectangle local3 =  rectangle2;
      this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local1,  local2,  local3), 0, 0, this.Game.ScreenWidth, 75);
       let mut local4: GameClass =  tgame;
       WindowClass local5 =  this.WindowList[this.GetNr(this.WMap)];
      rectangle2 = Rectangle::new(0, this.Game.ScreenHeight - 312, this.Game.ScreenWidth, 90);
      let mut rectangle3: &Rectangle = &rectangle2
       Rectangle local6 =  rectangle3;
      WindowClass[] windowList = this.WindowList;
      WindowClass[] windowClassArray = windowList;
      let mut nr: i32 =  this.GetNr(this.WPlay);
      let mut index: i32 =  nr;
      HistoryWindowClass2 historyWindowClass2 = (HistoryWindowClass2) windowClassArray[index];
       HistoryWindowClass2 local7 =  historyWindowClass2;
      HistoryOrderWindowClass tmpWindow1 = new HistoryOrderWindowClass( local4,  local5,  local6,  local7);
      windowList[nr] = (WindowClass) historyWindowClass2;
      let mut y: i32 =  this.Game.ScreenHeight - 312;
      let mut screenWidth1: i32 =  this.Game.ScreenWidth;
      this.WOrder = this.AddWindow((WindowClass) tmpWindow1, 0, y, screenWidth1, 90);
      Rectangle rectForTab;
      if (this.Game.EditObj.SetViewMode2 == 1)
      {
        rectForTab = DrawMod.GetRectForTab(1);
         let mut local8: GameClass =  tgame;
         WindowClass local9 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle4: &Rectangle = &rectangle2
         Rectangle local10 =  rectangle4;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabBriefingWindowClass2( local8,  local9,  local10, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 2)
      {
        rectForTab = DrawMod.GetRectForTab(2);
         let mut local11: GameClass =  tgame;
         WindowClass local12 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle5: &Rectangle = &rectangle2
         Rectangle local13 =  rectangle5;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabStatsWindowClass2( local11,  local12,  local13, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 3)
      {
        rectForTab = DrawMod.GetRectForTab(3);
         let mut local14: GameClass =  tgame;
         WindowClass local15 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle6: &Rectangle = &rectangle2
         Rectangle local16 =  rectangle6;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabOOBWindowClass2( local14,  local15,  local16, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 4)
      {
        rectForTab = DrawMod.GetRectForTab(4);
         let mut local17: GameClass =  tgame;
         WindowClass local18 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle7: &Rectangle = &rectangle2
         Rectangle local19 =  rectangle7;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabReportsWindowClass2( local17,  local18,  local19, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 5)
      {
        rectForTab = DrawMod.GetRectForTab(5);
         let mut local20: GameClass =  tgame;
         WindowClass local21 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle8: &Rectangle = &rectangle2
         Rectangle local22 =  rectangle8;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabCardsWindowClass2( local20,  local21,  local22, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 6)
      {
        rectForTab = DrawMod.GetRectForTab(6);
         let mut local23: GameClass =  tgame;
         WindowClass local24 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle9: &Rectangle = &rectangle2
         Rectangle local25 =  rectangle9;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabSMapWindowClass2( local23,  local24,  local25, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 7)
      {
        rectForTab = DrawMod.GetRectForTab(7);
         let mut local26: GameClass =  tgame;
         WindowClass local27 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        let mut rectangle10: &Rectangle = &rectangle2
         Rectangle local28 =  rectangle10;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabMiniMapWindowClass2( local26,  local27,  local28, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 != 8)
        return;
      rectForTab = DrawMod.GetRectForTab(8);
       let mut local29: GameClass =  tgame;
       WindowClass local30 =  this.WindowList[this.GetNr(this.WMap)];
      rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      let mut rectangle11: &Rectangle = &rectangle2
       Rectangle local31 =  rectangle11;
      let mut trect1: &Rectangle = &rectForTab
      this.WTabs = this.AddWindow((WindowClass) new TabPrefsWindowClass2( local29,  local30,  local31, trect1), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
    }

    pub ScreenReturnClass HandleWR(
      wr: WindowReturnClass,
      bool setflag,
       ScreenReturnClass obj)
    {
      if (wr.Counter > -1)
      {
        let mut counter: i32 =  wr.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
        {
          if (wr.CommandType[index] == 3)
          {
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
            if (wr.CommandData[index] == 80)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
            if (wr.CommandData[index] == 12)
            {
              this.WindowList[this.GetNr(this.WMap)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WMap)] = true;
            }
            if (wr.CommandData[index] == 81 | wr.CommandData[index] == 68 | wr.CommandData[index] == 44)
            {
              this.WindowList[this.GetNr(this.WOrder)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WOrder)] = true;
            }
            if (wr.CommandData[index] == 67)
            {
              this.WindowList[this.GetNr(this.WRes)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WRes)] = true;
            }
            if (wr.CommandData[index] == 9 & this.WTabs > 0)
            {
              this.WindowList[this.GetNr(this.WTabs)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WTabs)] = true;
            }
            if (wr.CommandData[index] == 35)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
          }
          if (wr.CommandType[index] == 7)
            this.WindowFlag[this.GetNr(this.WMap)] = true;
          if (wr.CommandType[index] == 1)
          {
            if (wr.CommandData[index] == 12)
            {
              this.RemoveWindow(this.WMap);
              this.WMap = 0;
            }
            if (wr.CommandData[index] == 67)
            {
              this.RemoveWindow(this.WRes);
              this.WRes = 0;
            }
            if (wr.CommandData[index] == 69 | wr.CommandData[index] == 5)
            {
              this.RemoveWindow(this.WPlay);
              this.WPlay = 0;
            }
            if (wr.CommandData[index] == 68)
            {
              this.RemoveWindow(this.WOrder);
              this.WOrder = 0;
            }
            if (wr.CommandData[index] == 9)
            {
              this.RemoveWindow(this.WTabs);
              this.WTabs = 0;
              this.WTabs = 0;
            }
          }
          if (wr.CommandType[index] == 2)
          {
            Rectangle rectangle1;
            if (wr.CommandData[index] == 12)
            {
              MapWindowClass2 tmpWindow = new MapWindowClass2( DrawMod.TGame, 222, 0);
              let mut screenWidth: i32 =  this.Game.ScreenWidth;
              let mut h: i32 =  this.Game.ScreenHeight - 222;
              rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 387);
              let mut tShowRectangle: &Rectangle = &rectangle1
              this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
              if (this.WRes > 0)
                this.WindowList[this.GetNr(this.WRes)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.WOrder > 0)
                this.WindowList[this.GetNr(this.WOrder)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.WTabs > 0)
                this.WindowList[this.GetNr(this.WTabs)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
            }
            Rectangle rectangle2;
            if (wr.CommandData[index] == 67)
            {
               let mut local1: GameClass =  DrawMod.TGame;
               WindowClass local2 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
              rectangle2 = rectangle1;
               Rectangle local3 =  rectangle2;
              this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local1,  local2,  local3), 0, 0, this.Game.ScreenWidth, 75);
            }
            Rectangle rectForTab;
            if (wr.CommandData[index] == 70)
            {
              rectForTab = DrawMod.GetRectForTab(1);
               let mut local4: GameClass =  DrawMod.TGame;
               WindowClass local5 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local6 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabBriefingWindowClass2( local4,  local5,  local6, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 71)
            {
              rectForTab = DrawMod.GetRectForTab(2);
               let mut local7: GameClass =  DrawMod.TGame;
               WindowClass local8 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local9 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabStatsWindowClass2( local7,  local8,  local9, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 72)
            {
              rectForTab = DrawMod.GetRectForTab(3);
               let mut local10: GameClass =  DrawMod.TGame;
               WindowClass local11 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local12 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabOOBWindowClass2( local10,  local11,  local12, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 73)
            {
              rectForTab = DrawMod.GetRectForTab(4);
               let mut local13: GameClass =  DrawMod.TGame;
               WindowClass local14 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local15 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabReportsWindowClass2( local13,  local14,  local15, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 74)
            {
              rectForTab = DrawMod.GetRectForTab(5);
               let mut local16: GameClass =  DrawMod.TGame;
               WindowClass local17 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local18 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabCardsWindowClass2( local16,  local17,  local18, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 75)
            {
              rectForTab = DrawMod.GetRectForTab(6);
               let mut local19: GameClass =  DrawMod.TGame;
               WindowClass local20 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local21 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabSMapWindowClass2( local19,  local20,  local21, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 76)
            {
              rectForTab = DrawMod.GetRectForTab(7);
               let mut local22: GameClass =  DrawMod.TGame;
               WindowClass local23 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local24 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabMiniMapWindowClass2( local22,  local23,  local24, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index] == 77)
            {
              rectForTab = DrawMod.GetRectForTab(8);
               let mut local25: GameClass =  DrawMod.TGame;
               WindowClass local26 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local27 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabPrefsWindowClass2( local25,  local26,  local27, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
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
      if (this.Game.Data.Product != 7)
      {
        if (x > this.Game.ScreenWidth - 52 & x < this.Game.ScreenWidth - 28 & y < 25)
          this.Game.FormRef.WindowState = FormWindowState.Minimized;
        if (x > this.Game.ScreenWidth - 28 & x < this.Game.ScreenWidth - 4 & y < 25 && Interaction.MsgBox( "Are you sure you want to exit your current game?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
        {
          if (this.Game.AIRunning | this.Game.AIThreadRunning)
            this.Game.AIThread.Abort();
          this.Game.Data = DataClass::new();
          this.Game.EditObj = new EditClass(this.Game.AppPath + "editobj.txt");
          if (this.Game.Data.UseAI == 1 & Information.IsNothing( this.Game.NewAIObj))
            this.Game.NewAIObj = new NewAIClass(this.Game);
          if (this.Game.Data.UseAI == 1)
            this.Game.NewAIObj.LastRegime = -1;
          this.Game.EditObj.ShowInitialMenu = true;
          screenReturnClass.NewScreen = 12;
          this.Game.AIThreadRunning = false;
          this.Game.AIRunning = false;
          return screenReturnClass;
        }
      }
      if (this.WindowCounter > -1)
      {
        for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Operators.CompareString(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass", false) != 0 && x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
          {
            wr: WindowReturnClass = this.WindowList[windowCounter].HandleMouseClick(x - this.WindowX[windowCounter], y - this.WindowY[windowCounter], b);
            this.WindowFlag[windowCounter] = wr.Flag;
            if (wr.Flag)
            {
              this.HandleWR(wr, true,  screenReturnClass);
              return screenReturnClass;
            }
            if (wr.NoMouseClickBelow)
            {
              screenReturnClass.flag = false;
              return screenReturnClass;
            }
          }
        }
        for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Operators.CompareString(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass", false) == 0 && x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
          {
            wr: WindowReturnClass = this.WindowList[windowCounter].HandleMouseClick(x - this.WindowX[windowCounter], y - this.WindowY[windowCounter], b);
            this.WindowFlag[windowCounter] = wr.Flag;
            if (wr.Flag)
            {
              this.HandleWR(wr, true,  screenReturnClass);
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
      if (x > this.Game.ScreenWidth - 52 & x < this.Game.ScreenWidth - 28 & y < 25)
      {
        if (!this.doMinimize)
        {
          this.doMinimize = true;
          screenReturnClass.flag = true;
        }
      }
      else if (this.doMinimize)
      {
        this.doMinimize = false;
        screenReturnClass.flag = true;
      }
      if (x > this.Game.ScreenWidth - 28 & x < this.Game.ScreenWidth - 4 & y < 25)
      {
        if (!this.doQuit)
        {
          this.doQuit = true;
          screenReturnClass.flag = true;
        }
      }
      else if (this.doQuit)
      {
        this.doQuit = false;
        screenReturnClass.flag = true;
      }
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter1: i32 =  this.WindowCounter;
      windowReturnClass: WindowReturnClass;
      for (let mut index: i32 =  0; index <= windowCounter1; index += 1)
      {
        if (Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass", false) != 0 & Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 && x >= this.WindowX[index] & x <= this.WindowX[index] + this.WindowW[index] & y >= this.WindowY[index] & y <= this.WindowY[index] + this.WindowH[index])
        {
          windowReturnClass = this.WindowList[index].HandleMouseMove(x - this.WindowX[index], y - this.WindowY[index]);
          this.WindowFlag[index] = windowReturnClass.Flag;
          if (windowReturnClass.Counter > -1)
          {
            this.HandleWR(windowReturnClass, true,  screenReturnClass);
            windowReturnClass.Flag = true;
          }
          if (windowReturnClass.Overlay | this.LastOverlayWindow > 0)
          {
            if (this.LastOverlayWindow > 0 & this.LastOverlayWindow != this.WindowID[index])
              this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
            if (windowReturnClass.Overlay)
              this.LastOverlayWindow = this.WindowID[index];
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
        let mut windowCounter2: i32 =  this.WindowCounter;
        for (let mut index: i32 =  0; index <= windowCounter2; index += 1)
        {
          if (Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass", false) == 0 | Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 && x >= this.WindowX[index] & x <= this.WindowX[index] + this.WindowW[index] & y >= this.WindowY[index] & y <= this.WindowY[index] + this.WindowH[index])
          {
            wr: WindowReturnClass = this.WindowList[index].HandleMouseMove(x - this.WindowX[index], y - this.WindowY[index]);
            this.WindowFlag[index] = wr.Flag;
            if (wr.Counter > -1)
            {
              this.HandleWR(wr, true,  screenReturnClass);
              wr.Flag = true;
            }
            if (wr.Overlay | this.LastOverlayWindow > 0)
            {
              if (this.LastOverlayWindow > 0 & this.LastOverlayWindow != this.WindowID[index])
                this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
              if (wr.Overlay)
                this.LastOverlayWindow = this.WindowID[index];
            }
            if (!screenReturnClass.flag)
              screenReturnClass.flag = wr.Flag;
            if (screenReturnClass.flag)
              return screenReturnClass;
          }
        }
      }
      if (this.LastOverlayWindow > 0)
      {
        this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
        screenReturnClass.flag = true;
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      this.Game.FormRef.debugPoint4 = 1;
      if (this.Game.Data.Product == 6 && this.Game.EditObj.OrderType > 0 & this.Game.EditObj.OrderType != 26 && !this.Game.AIThreadRunning & !this.Game.EditObj.AIMoving)
        return screenReturnClass;
      this.Game.FormRef.debugPoint4 = 2;
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        wr: WindowReturnClass = this.WindowList[windowCounter].handleTimer();
        if (!this.WindowFlag[windowCounter] & wr.Flag)
        {
          this.WindowFlag[windowCounter] = wr.Flag;
          screenReturnClass.flag = true;
        }
        if (wr.Flag && wr.Counter > -1)
        {
          this.HandleWR(wr, false,  screenReturnClass);
          this.Game.FormRef.debugPoint4 = 20 + windowCounter;
        }
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleKeyPress(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        wr: WindowReturnClass = this.WindowList[windowCounter].HandleKeyPress(nr);
        if (!this.WindowFlag[windowCounter] & wr.Flag)
          this.WindowFlag[windowCounter] = wr.Flag;
        if (wr.Counter > -1)
          this.HandleWR(wr, false,  screenReturnClass);
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
