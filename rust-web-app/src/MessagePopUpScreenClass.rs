// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MessagePopUpScreenClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class MessagePopUpScreenClass : ScreenClass
  {
     wtop: i32;
     wup: i32;

    pub MessagePopUpScreenClass( tGame: GameClass, tformref: Form1)
      : base( tGame, -4, tformref)
    {
      if ( this.Game.Data.RuleVar[839] == 1.0 | Operators.CompareString(tformref.Screeny.GetType().ToString(), "WindowsApplication1.FirstScreenClass", false) == 0 & this.Game.ModIntroType == 1)
      {
        if (tGame.EditObj.PopupValue == 1)
          this.wup = this.AddWindow((WindowClass) new MapSelectWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 435.0),  Math.Round( tGame.ScreenHeight / 2.0 - 350.0), 860, 700);
        else if (tGame.EditObj.PopupValue == 2)
          this.wup = this.AddWindow((WindowClass) new HandCardWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 190.0),  Math.Round( tGame.ScreenHeight / 2.0 - 190.0), 380, 380);
        else if (tGame.EditObj.PopupValue == 3)
          this.wup = this.AddWindow((WindowClass) new UnitSelectWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 435.0),  Math.Round( tGame.ScreenHeight / 2.0 - 350.0), 860, 700);
        else if (tGame.EditObj.PopupValue == 4)
          this.wup = this.AddWindow((WindowClass) new OfficerInfoWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 190.0), 680, 380);
        else if (tGame.EditObj.PopupValue == 5)
          this.wup = this.AddWindow((WindowClass) new SystemMessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 100.0), 600, 200);
        else if (tGame.EditObj.PopupValue == 6)
          this.wup = this.AddWindow((WindowClass) new SFWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 340.0), 880, 580);
        else if (tGame.EditObj.PopupValue == 7)
        {
          let mut ucounter: i32 = tGame.TempCombat.UCounter;
          val1: i32;
          val2: i32;
          for (let mut index: i32 = 0; index <= ucounter; index += 1)
          {
            if (this.Game.TempCombat.UList[index].Uattacker == 1)
              val1 += 1;
            if (this.Game.TempCombat.UList[index].Uattacker == 0)
              val2 += 1;
          }
          let mut num: i32 = 180 + Math.Max(val1, val2) * 37 + 10;
          this.wup = this.AddWindow((WindowClass) new CombatResultWindowClass2( tGame, num),  Math.Round( tGame.ScreenWidth / 2.0 - 505.0),  Math.Round( tGame.ScreenHeight / 2.0 -  num / 2.0), 1010, num);
        }
        else if (tGame.EditObj.PopupValue == 8)
        {
          let mut num1: i32 =  Math.Round(Conversion.Int( this.Game.ScreenWidth / 2.0));
          let mut num2: i32 =  Math.Round(Conversion.Int( this.Game.ScreenHeight / 2.0));
          this.wup = this.AddWindow((WindowClass) new CombatDetailWindowClass( tGame, 0, this.Game.ScreenWidth, this.Game.ScreenHeight), 0, 0, this.Game.ScreenWidth, this.Game.ScreenHeight);
        }
        else if (tGame.EditObj.PopupValue == 9)
          this.wup = this.AddWindow((WindowClass) new CreditsInfoWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 340.0), 880, 680);
        else if (tGame.EditObj.PopupValue == 10)
          this.wup = this.AddWindow((WindowClass) new FlexMessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 100.0), 600, 200);
        else if (tGame.EditObj.PopupValue == 11)
          this.wup = this.AddWindow((WindowClass) new AirSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 190.0), 680, 380);
        else if (tGame.EditObj.PopupValue == 16)
          this.wup = this.AddWindow((WindowClass) new EditorPaintWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
        else if (tGame.EditObj.PopupValue == 17)
          this.wup = this.AddWindow((WindowClass) new SimpleLibImportWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
        else if (tGame.EditObj.PopupValue == 18)
          this.wup = this.AddWindow((WindowClass) new SimpleMapImportWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
        else if (tGame.EditObj.PopupValue == 19)
        {
          this.wup = this.AddWindow((WindowClass) new LoadWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 200.0),  Math.Round( tGame.ScreenHeight / 2.0 - 120.0), 400, 240);
        }
        else
        {
          if (tGame.EditObj.PopupValue != 0)
            return;
          this.wup = this.AddWindow((WindowClass) new MessageWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 290.0), 680, 480);
        }
      }
      else if (tGame.EditObj.PopupValue == 1)
        this.wup = this.AddWindow((WindowClass) new MapSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 2)
        this.wup = this.AddWindow((WindowClass) new HandCardWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 405.0),  Math.Round( tGame.ScreenHeight / 2.0 - 305.0), 810, 610);
      else if (tGame.EditObj.PopupValue == 3)
        this.wup = this.AddWindow((WindowClass) new UnitSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 4)
        this.wup = this.AddWindow((WindowClass) new OfficerInfoWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 405.0),  Math.Round( tGame.ScreenHeight / 2.0 - 305.0), 810, 610);
      else if (tGame.EditObj.PopupValue == 6)
        this.wup = this.AddWindow((WindowClass) new SFWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 9)
        this.wup = this.AddWindow((WindowClass) new ATCreditsInfoWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 340.0), 880, 680);
      else if (tGame.EditObj.PopupValue == 10)
        this.wup = this.AddWindow((WindowClass) new ATFlexMessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 400.0),  Math.Round( tGame.ScreenHeight / 2.0 - 100.0), 800, 200);
      else if (tGame.EditObj.PopupValue == 12)
        this.wup = this.AddWindow((WindowClass) new LTWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 13)
        this.wup = this.AddWindow((WindowClass) new PPLWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 14)
        this.wup = this.AddWindow((WindowClass) new OldResearchWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 15)
        this.wup = this.AddWindow((WindowClass) new StatsWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 16)
        this.wup = this.AddWindow((WindowClass) new EditorPaintWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 17)
        this.wup = this.AddWindow((WindowClass) new SimpleLibImportWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 18)
        this.wup = this.AddWindow((WindowClass) new SimpleMapImportWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 512.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1024, 768);
      else if (tGame.EditObj.PopupValue == 19)
      {
        this.wup = this.AddWindow((WindowClass) new LoadWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 200.0),  Math.Round( tGame.ScreenHeight / 2.0 - 120.0), 400, 240);
      }
      else
      {
        if (this.Game.Data.Turn <= -1)
          return;
        if (tGame.Data.RegimeObj[tGame.Data.Turn].MesStyle[tGame.EditObj.FromMessage] == 0)
        {
          if (tGame.EditObj.PopupValue != 0)
            return;
          this.wup = this.AddWindow((WindowClass) new MessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 405.0),  Math.Round( tGame.ScreenHeight / 2.0 - 305.0), 810, 610);
        }
        else
        {
          if (tGame.EditObj.PopupValue != 0)
            return;
          this.wup = this.AddWindow((WindowClass) new MessageWindowClass( tGame, 1),  Math.Round( tGame.ScreenWidth / 2.0 - 405.0),  Math.Round( tGame.ScreenHeight / 2.0 - 305.0), 810, 610);
        }
      }
    }

    pub ScreenReturnClass HandleTimer()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      bool flag = false;
      let mut windowCounter: i32 = this.WindowCounter;
      for (let mut index1: i32 = 0; index1 <= windowCounter; index1 += 1)
      {
        windowReturnClass2: WindowReturnClass = this.WindowList[index1].handleTimer();
        this.WindowFlag[index1] = windowReturnClass2.Flag;
        if (this.WindowFlag[index1])
          flag = true;
        if (windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 = windowReturnClass2.Counter;
          for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
          {
            if (windowReturnClass2.CommandType[index2] == 3)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index2] == 6)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
              screenReturnClass.ClosePopUp = true;
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index2] == 5)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
              screenReturnClass.OpenPopUp = true;
              return screenReturnClass;
            }
          }
        }
      }
      screenReturnClass.flag = flag;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseClick(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (x > this.Game.ScreenWidth - 52 & x < this.Game.ScreenWidth - 28 & y < 25)
        this.Game.FormRef.WindowState = FormWindowState.Minimized;
      if (x > this.Game.ScreenWidth - 28 & x < this.Game.ScreenWidth - 4 & y < 25 && Interaction.MsgBox( "Are you sure you want to exit your current game?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
      {
        this.Game.Data = DataClass::new();
        this.Game.EditObj = new EditClass(this.Game.AppPath + "editobj.txt");
        this.Game.EditObj.ShowInitialMenu = true;
        screenReturnClass.NewScreen = 1;
        screenReturnClass.ClosePopUp = true;
        return screenReturnClass;
      }
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 = this.WindowCounter;
      for (let mut index1: i32 = 0; index1 <= windowCounter; index1 += 1)
      {
        if (x > this.WindowX[index1] & x < this.WindowX[index1] + this.WindowW[index1] && y > this.WindowY[index1] & y < this.WindowY[index1] + this.WindowH[index1])
        {
          windowReturnClass: WindowReturnClass = this.WindowList[index1].HandleMouseClick(x - this.WindowX[index1], y - this.WindowY[index1], b);
          this.WindowFlag[index1] = windowReturnClass.Flag;
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 = windowReturnClass.Counter;
            for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
            {
              if (windowReturnClass.CommandType[index2] == 3)
              {
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index2];
                return screenReturnClass;
              }
              if (windowReturnClass.CommandType[index2] == 6)
              {
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index2];
                screenReturnClass.ClosePopUp = true;
                return screenReturnClass;
              }
              if (windowReturnClass.CommandType[index2] == 5)
              {
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index2];
                screenReturnClass.OpenPopUp = true;
                return screenReturnClass;
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
      for (let mut windowCounter: i32 = this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        windowReturnClass2: WindowReturnClass = this.WindowList[windowCounter].HandleKeyPress(nr);
        if (!this.WindowFlag[windowCounter])
          this.WindowFlag[windowCounter] = windowReturnClass2.Flag;
        if (windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 = windowReturnClass2.Counter;
          for (let mut index: i32 = 0; index <= counter; index += 1)
          {
            if (windowReturnClass2.CommandType[index] == 3)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index] == 6)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              screenReturnClass.ClosePopUp = true;
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index] == 5)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              screenReturnClass.OpenPopUp = true;
              return screenReturnClass;
            }
          }
        }
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleKeyup(nr: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 = this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        windowReturnClass2: WindowReturnClass = this.WindowList[windowCounter].HandleKeyup(nr);
        if (!this.WindowFlag[windowCounter])
          this.WindowFlag[windowCounter] = windowReturnClass2.Flag;
        if (windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 = windowReturnClass2.Counter;
          for (let mut index: i32 = 0; index <= counter; index += 1)
          {
            if (windowReturnClass2.CommandType[index] == 3)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index] == 6)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              screenReturnClass.ClosePopUp = true;
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index] == 5)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              screenReturnClass.OpenPopUp = true;
              return screenReturnClass;
            }
          }
        }
      }
      screenReturnClass.flag = true;
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
      let mut windowCounter: i32 = this.WindowCounter;
      for (let mut index1: i32 = 0; index1 <= windowCounter; index1 += 1)
      {
        if (x > this.WindowX[index1] & x < this.WindowX[index1] + this.WindowW[index1] && y > this.WindowY[index1] & y < this.WindowY[index1] + this.WindowH[index1])
        {
          windowReturnClass: WindowReturnClass = this.WindowList[index1].HandleMouseMove(x - this.WindowX[index1], y - this.WindowY[index1]);
          this.WindowFlag[index1] = windowReturnClass.Flag;
          if (windowReturnClass.Overlay)
          {
            if (this.LastOverlayWindow > 0 & this.LastOverlayWindow != this.WindowID[index1])
              this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
            this.LastOverlayWindow = this.WindowID[index1];
          }
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 = windowReturnClass.Counter;
            for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
            {
              if (windowReturnClass.CommandType[index2] != 4 || windowReturnClass.CommandData[index2] != 29)
                ;
            }
          }
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
  }
}
