// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoryScreenClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class HistoryScreenClass : ScreenClass
  {
     wtop: i32;
     wup: i32;
     wdown: i32;
     wres: i32;
     offsetx: i32;

    pub HistoryScreenClass( tGame: GameClass)
      : base( tGame, tGame.BACKGROUND1MARC)
    {
      let mut num: i32 =   Math.Round( (this.Game.ScreenWidth - 1024) / 2.0);
      this.offsetx = 0;
      this.wtop = this.AddWindow((WindowClass) new HistoryWindowClass( tGame, this.OwnBackground, this.offsetx, this.Game.ScreenHeight - 250), this.offsetx, this.Game.ScreenHeight - 250, this.Game.ScreenWidth, 250);
      this.wup = this.AddWindow((WindowClass) new InfoWindowClass( tGame), 0, this.Game.ScreenHeight - 270, this.Game.ScreenWidth, 20);
      if (this.Game.EditObj.AIMoving)
      {
        this.wdown = this.AddWindow((WindowClass) new MapWindowClass( tGame, 270, 0, 0), 0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight - 270);
      }
      else
      {
        this.wdown = this.AddWindow((WindowClass) new MapWindowClass( tGame, 305, 0, 0), 0, 35, this.Game.ScreenWidth, this.Game.ScreenHeight - 305);
        this.wres = this.AddWindow((WindowClass) new ResourceWindowClass( tGame, 220), 0, 0, this.Game.ScreenWidth, 35);
      }
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
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        if (x >= this.WindowX[index1] & x <= this.WindowX[index1] + this.WindowW[index1] & y >= this.WindowY[index1] & y <= this.WindowY[index1] + this.WindowH[index1])
        {
          windowReturnClass: WindowReturnClass = this.WindowList[index1].HandleMouseMove(x - this.WindowX[index1], y - this.WindowY[index1]);
          this.WindowFlag[index1] = windowReturnClass.Flag;
          if (windowReturnClass.Overlay | this.LastOverlayWindow > 0)
          {
            if (this.LastOverlayWindow > 0 & this.LastOverlayWindow != this.WindowID[index1])
              this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
            if (windowReturnClass.Overlay)
              this.LastOverlayWindow = this.WindowID[index1];
          }
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 =  windowReturnClass.Counter;
            for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
            {
              if (windowReturnClass.CommandType[index2] == 4 && windowReturnClass.CommandData[index2] == 29)
              {
                this.WindowList[this.GetNr(this.wup)].DoRefresh();
                this.WindowFlag[this.GetNr(this.wup)] = true;
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
      if (this.LastOverlayWindow > 0)
      {
        this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
        screenReturnClass.flag = true;
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseClick(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (x > this.Game.ScreenWidth - 52 & x < this.Game.ScreenWidth - 28 & y < 25)
        this.Game.FormRef.WindowState = FormWindowState.Minimized;
      if (x > this.Game.ScreenWidth - 28 & x < this.Game.ScreenWidth - 4 & y < 25 && Interaction.MsgBox( "Are you sure you want to exit your current game?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
      {
        if (this.Game.AIRunning | this.Game.AIThreadRunning)
          this.Game.AIThread.Abort();
        this.Game.Data = DataClass::new();
        this.Game.EditObj = new EditClass(this.Game.AppPath + "editobj.txt");
        if (this.Game.Data.UseAI == 1)
          this.Game.NewAIObj.LastRegime = -1;
        this.Game.EditObj.ShowInitialMenu = true;
        screenReturnClass.NewScreen = 1;
        this.Game.AIThreadRunning = false;
        return screenReturnClass;
      }
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        if (x > this.WindowX[index1] & x < this.WindowX[index1] + this.WindowW[index1] && y > this.WindowY[index1] & y < this.WindowY[index1] + this.WindowH[index1])
        {
          windowReturnClass: WindowReturnClass = this.WindowList[index1].HandleMouseClick(x - this.WindowX[index1], y - this.WindowY[index1], b);
          this.WindowFlag[index1] = windowReturnClass.Flag;
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
              if (windowReturnClass.CommandType[index2] == 4 && windowReturnClass.CommandData[index2] == 12)
              {
                this.WindowList[this.GetNr(this.wdown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.wdown)] = true;
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

    pub ScreenReturnClass HandleKeyPress(nr: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        windowReturnClass2: WindowReturnClass = this.WindowList[windowCounter].HandleKeyPress(nr);
        if (!this.WindowFlag[windowCounter])
          this.WindowFlag[windowCounter] = windowReturnClass2.Flag;
        if (windowReturnClass2.Counter > -1 && windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 =  windowReturnClass2.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
          {
            if (windowReturnClass2.CommandType[index] == 3)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              return screenReturnClass;
            }
          }
        }
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        windowReturnClass: WindowReturnClass = this.WindowList[index1].handleTimer();
        this.WindowFlag[index1] = windowReturnClass.Flag;
        if (windowReturnClass.Flag)
        {
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
              if (windowReturnClass.CommandType[index2] == 4 && windowReturnClass.CommandData[index2] == 12)
              {
                this.WindowList[this.GetNr(this.wdown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.wdown)] = true;
                screenReturnClass.flag = true;
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
  }
}
