// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ManagementScreenClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ManagementScreenClass : ScreenClass
  {
     WMap: i32;
     WRes: i32;
     OffSetX: i32;

    pub ManagementScreenClass( tgame: GameClass, tformref: Form1)
      : base( tgame, -3, tformref)
    {
      this.OffSetX =  Math.Round( (this.Game.ScreenWidth - 1280) / 2.0);
      this.Game.HandyFunctionsObj.SetGameColors();
      Rectangle rectangle1;
      if (this.Game.EditObj.se1_ManagementTab <= 51)
      {
        SpecialWindowClass1 tmpWindow = new SpecialWindowClass1( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
      if (this.Game.EditObj.se1_ManagementTab == 52)
      {
        SpecialWindowClass2 tmpWindow = new SpecialWindowClass2( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
      if (this.Game.EditObj.se1_ManagementTab == 53)
      {
        SpecialWindowClass3 tmpWindow = new SpecialWindowClass3( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
      if (this.Game.EditObj.se1_ManagementTab == 54)
      {
        SpecialWindowClass4 tmpWindow = new SpecialWindowClass4( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
      if (this.Game.EditObj.se1_ManagementTab == 55)
      {
        SpecialWindowClass5 tmpWindow = new SpecialWindowClass5( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
      if (this.Game.EditObj.se1_ManagementTab == 56)
      {
        SpecialWindowClass6 tmpWindow = new SpecialWindowClass6( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
      if (this.Game.EditObj.se1_ManagementTab == 57)
      {
        SpecialWindowClass7 tmpWindow = new SpecialWindowClass7( tgame, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut screenHeight: i32 =  this.Game.ScreenHeight;
        rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        let mut tShowRectangle: &Rectangle = &rectangle1
        this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
      }
       let mut local1: GameClass =  tgame;
       WindowClass local2 =  this.WindowList[this.GetNr(this.WMap)];
      rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
      let mut rectangle2: &Rectangle = &rectangle1
       Rectangle local3 =  rectangle2;
      this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local1,  local2,  local3), 0, 0, this.Game.ScreenWidth, 75);
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
            if (wr.CommandData[index] == 12)
            {
              this.WindowList[this.GetNr(this.WMap)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WMap)] = true;
            }
            if (wr.CommandData[index] == 67)
            {
              this.WindowList[this.GetNr(this.WRes)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WRes)] = true;
            }
          }
          if (wr.CommandType[index] == 7)
          {
            if (wr.CommandData[index] == 12)
              this.WindowFlag[this.GetNr(this.WMap)] = true;
            if (wr.CommandData[index] == 67)
              this.WindowFlag[this.GetNr(this.WRes)] = true;
          }
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
          }
          if (wr.CommandType[index] == 2)
          {
            Rectangle rectangle1;
            if (wr.CommandData[index] == 12)
            {
              if (this.Game.EditObj.se1_ManagementTab <= 51)
              {
                SpecialWindowClass1 tmpWindow = new SpecialWindowClass1( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
              if (this.Game.EditObj.se1_ManagementTab == 52)
              {
                SpecialWindowClass2 tmpWindow = new SpecialWindowClass2( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
              if (this.Game.EditObj.se1_ManagementTab == 53)
              {
                SpecialWindowClass3 tmpWindow = new SpecialWindowClass3( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
              if (this.Game.EditObj.se1_ManagementTab == 54)
              {
                SpecialWindowClass4 tmpWindow = new SpecialWindowClass4( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
              if (this.Game.EditObj.se1_ManagementTab == 55)
              {
                SpecialWindowClass5 tmpWindow = new SpecialWindowClass5( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
              if (this.Game.EditObj.se1_ManagementTab == 56)
              {
                SpecialWindowClass6 tmpWindow = new SpecialWindowClass6( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
              if (this.Game.EditObj.se1_ManagementTab == 57)
              {
                SpecialWindowClass7 tmpWindow = new SpecialWindowClass7( DrawMod.TGame, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut screenHeight: i32 =  this.Game.ScreenHeight;
                rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
                let mut tShowRectangle: &Rectangle = &rectangle1
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, screenHeight, tShowRectangle);
              }
            }
            if (wr.CommandData[index] == 67)
            {
               let mut local1: GameClass =  DrawMod.TGame;
               WindowClass local2 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
              let mut rectangle2: &Rectangle = &rectangle1
               Rectangle local3 =  rectangle2;
              this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local1,  local2,  local3), 0, 0, this.Game.ScreenWidth, 75);
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
      if (this.WindowCounter > -1)
      {
        for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Strings.InStr(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.Special") > 0 && x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
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
          if (Strings.InStr(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.Special") <= 0 && x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
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
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index: i32 =  0; index <= windowCounter; index += 1)
      {
        this.WindowList[index].MouseInThisWindow = false;
        if (x >= this.WindowX[index] & x < this.WindowX[index] + this.WindowW[index] && y >= this.WindowY[index] & y < this.WindowY[index] + this.WindowH[index])
          this.WindowList[index].MouseInThisWindow = true;
        windowReturnClass: WindowReturnClass = this.WindowList[index].HandleMouseMove(x - this.WindowX[index], y - this.WindowY[index]);
        this.WindowFlag[index] = windowReturnClass.Flag;
        if (windowReturnClass.Overlay | this.LastOverlayWindow > 0)
        {
          if (this.LastOverlayWindow > 0 & this.LastOverlayWindow != this.WindowID[index])
            this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
          if (this.LastOverlayWindow > 0 & !windowReturnClass.Overlay)
            this.LastOverlayWindow = 0;
          if (windowReturnClass.Overlay)
            this.LastOverlayWindow = this.WindowID[index];
          screenReturnClass.flag = true;
          return screenReturnClass;
        }
        if (windowReturnClass.Flag)
        {
          screenReturnClass.flag = windowReturnClass.Flag;
          return screenReturnClass;
        }
      }
      if (this.LastOverlayWindow > 0)
      {
        this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
        screenReturnClass.flag = true;
      }
      else
        screenReturnClass.flag = false;
      if (this.Game.Data.Product < 7)
      {
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
      }
      else if (Operators.CompareString(this.GetType().FullName, "WindowsApplication1.RandomScreenClass2", false) == 0 | Operators.CompareString(this.GetType().FullName, "WindowsApplication1.GameLoopMainWindowClass2", false) == 0)
      {
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
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
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
          this.HandleWR(wr, false,  screenReturnClass);
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
