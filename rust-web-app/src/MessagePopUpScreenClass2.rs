// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MessagePopUpScreenClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class MessagePopUpScreenClass2 : ScreenClass
  {
     wtop: i32;
     wup: i32;

    pub MessagePopUpScreenClass2( tGame: GameClass, tformref: Form1)
      : base( tGame, -4, tformref)
    {
      if (tGame.EditObj.PopupValue == 1)
        this.wup = this.AddWindow((WindowClass) new MapSelectWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 435.0),  Math.Round( tGame.ScreenHeight / 2.0 - 350.0), 860, 700);
      else if (tGame.EditObj.PopupValue == 2)
        this.wup = this.AddWindow((WindowClass) new HandCardWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 190.0),  Math.Round( tGame.ScreenHeight / 2.0 - 190.0), 380, 380);
      else if (tGame.EditObj.PopupValue == 3)
        this.wup = this.AddWindow((WindowClass) new UnitSelectWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 505.0),  Math.Round( tGame.ScreenHeight / 2.0 - 350.0), 1010, 700);
      else if (tGame.EditObj.PopupValue == 4)
        this.wup = this.AddWindow((WindowClass) new OfficerInfoWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 340.0), 680, 680);
      else if (tGame.EditObj.PopupValue == 5)
        this.wup = this.AddWindow((WindowClass) new SystemMessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 100.0), 600, 200);
      else if (tGame.EditObj.PopupValue == 6)
      {
        if ( this.Game.Data.RuleVar[460] > 0.0)
          this.wup = this.AddWindow((WindowClass) new SFWindowClass3( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 880, 768);
        else
          this.wup = this.AddWindow((WindowClass) new SFWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 340.0), 880, 580);
      }
      else if (tGame.EditObj.PopupValue == 7)
      {
        let mut num1: i32 =  Math.Max(1280, this.Game.ScreenWidth - 400);
        let mut num2: i32 =  Math.Max(768, this.Game.ScreenHeight - 200);
        this.wup = this.AddWindow((WindowClass) new CombatResultWindowClass2( tGame, num1, num2),  Math.Round( (this.Game.ScreenWidth - num1) / 2.0),  Math.Round( (this.Game.ScreenHeight - num2) / 2.0), num1, num2);
      }
      else if (tGame.EditObj.PopupValue == 8)
      {
        let mut num3: i32 =  Math.Min(1324, this.Game.ScreenWidth);
        let mut num4: i32 =  Math.Min(968, this.Game.ScreenHeight);
        let mut x: i32 =   Math.Round(Conversion.Int( (this.Game.ScreenWidth - num3) / 2.0));
        let mut y: i32 =   Math.Round(Conversion.Int( (this.Game.ScreenHeight - num4) / 2.0));
        this.wup = this.AddWindow((WindowClass) new CombatDetailWindowClass( tGame, 0, num3, num4), x, y, num3, num4);
      }
      else if (tGame.EditObj.PopupValue == 9)
        this.wup = this.AddWindow((WindowClass) new CreditsInfoWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 340.0), 880, 680);
      else if (tGame.EditObj.PopupValue == 10)
        this.wup = this.AddWindow((WindowClass) new FlexMessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 100.0), 600, 200);
      else if (tGame.EditObj.PopupValue == 11)
        this.wup = this.AddWindow((WindowClass) new AirSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 440.0),  Math.Round( tGame.ScreenHeight / 2.0 - 290.0), 880, 580);
      else if (tGame.EditObj.PopupValue == 12)
        this.wup = this.AddWindow((WindowClass) new ArtSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 190.0), 680, 380);
      else if (tGame.EditObj.PopupValue == 14)
        this.wup = this.AddWindow((WindowClass) new RegisterPopup( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 240.0), 600, 480);
      else if (tGame.EditObj.PopupValue == 15)
        this.wup = this.AddWindow((WindowClass) new ServerPopup( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 200.0), 600, 400);
      else if (tGame.EditObj.PopupValue == 16)
        this.wup = this.AddWindow((WindowClass) new LoginPopup( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 300.0),  Math.Round( tGame.ScreenHeight / 2.0 - 240.0), 600, 480);
      else if (tGame.EditObj.PopupValue == 17)
        this.wup = this.AddWindow((WindowClass) new LoadWindow( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 200.0),  Math.Round( tGame.ScreenHeight / 2.0 - 120.0), 400, 240);
      else if (tGame.EditObj.PopupValue == 18)
        this.wup = this.AddWindow((WindowClass) new LandSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 190.0), 680, 380);
      else if (tGame.EditObj.PopupValue == 19)
        this.wup = this.AddWindow((WindowClass) new BigMessageWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 480.0),  Math.Round( tGame.ScreenHeight / 2.0 - 375.0), 960, 750);
      else if (tGame.EditObj.PopupValue == 20)
        this.wup = this.AddWindow((WindowClass) new SimplePrefsWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 240.0),  Math.Round( tGame.ScreenHeight / 2.0 - 125.0), 480, 250);
      else if (tGame.EditObj.PopupValue == 21)
      {
        let mut extraHeight: i32 =   Math.Round( (this.Game.ScreenHeight - 750) / 2.0);
        if (extraHeight > 120)
          extraHeight = 120;
        if (Strings.InStr(Strings.LCase(this.Game.EditObj.UDSpopupText), "[key]nobackground[/key]") < 1)
          this.wup = this.AddWindow((WindowClass) new UDSMessageWindowClass( tGame, extraHeight),  Math.Round( tGame.ScreenWidth / 2.0 - 630.0),  Math.Round( tGame.ScreenHeight / 2.0 -  (375 +  Math.Round( extraHeight / 2.0))), 1260, 750 + extraHeight);
        else
          this.wup = this.AddWindow((WindowClass) new UDSMessageWindowClass( tGame, true, extraHeight),  Math.Round( tGame.ScreenWidth / 2.0 - 630.0),  Math.Round( tGame.ScreenHeight / 2.0 -  (375 +  Math.Round( extraHeight / 2.0))), 1260, 750 + extraHeight);
      }
      else if (tGame.EditObj.PopupValue == 22)
        this.wup = this.AddWindow((WindowClass) new CombatSelectWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1280, 768);
      else if (tGame.EditObj.PopupValue == 23)
      {
        let mut num: i32 =  Math.Max( Math.Round( this.Game.ScreenHeight * 0.9), 768);
        if (num > 1280)
          num = 1280;
        this.wup = this.AddWindow((WindowClass) new SpecialWindowClass1( tGame, 1280, num),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 -   Math.Round( num / 2.0)), 1280, num);
      }
      else if (tGame.EditObj.PopupValue == 24)
      {
        let mut num5: i32 =  Math.Max( Math.Round( this.Game.ScreenHeight * 0.9), 768);
        if (num5 > 1280)
          num5 = 1280;
        let mut num6: i32 =  Math.Max( Math.Round( this.Game.ScreenWidth * 0.9), 1280);
        if (num6 > 1920)
          num6 = 1920;
        this.wup = this.AddWindow((WindowClass) new SpecialWindowClass2( tGame, num6, num5),  Math.Round( tGame.ScreenWidth / 2.0 -   Math.Round( num6 / 2.0)),  Math.Round( tGame.ScreenHeight / 2.0 -   Math.Round( num5 / 2.0)), num6, num5);
      }
      else if (tGame.EditObj.PopupValue == 25)
      {
        let mut num7: i32 =  Math.Max( Math.Round( this.Game.ScreenHeight * 0.9), 768);
        if (num7 > 1280)
          num7 = 1280;
        let mut num8: i32 =  Math.Max( Math.Round( this.Game.ScreenWidth * 0.9), 1280);
        if (num8 > 1920)
          num8 = 1920;
        this.wup = this.AddWindow((WindowClass) new SpecialWindowClass3( tGame, num8, num7),  Math.Round( tGame.ScreenWidth / 2.0 -   Math.Round( num8 / 2.0)),  Math.Round( tGame.ScreenHeight / 2.0 -   Math.Round( num7 / 2.0)), num8, num7);
      }
      else if (tGame.EditObj.PopupValue == 26)
        this.wup = this.AddWindow((WindowClass) new TransportWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 500.0),  Math.Round( tGame.ScreenHeight / 2.0 - 380.0), 1000, 760);
      else if (tGame.EditObj.PopupValue == 27)
        this.wup = this.AddWindow((WindowClass) new BattlegroupWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 500.0),  Math.Round( tGame.ScreenHeight / 2.0 - 380.0), 1000, 760);
      else if (tGame.EditObj.PopupValue == 28)
        this.wup = this.AddWindow((WindowClass) new LISTrafficWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 600.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1200, 768);
      else if (tGame.EditObj.PopupValue == 29)
        this.wup = this.AddWindow((WindowClass) new FlexCardWindowClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 200.0),  Math.Round( tGame.ScreenHeight / 2.0 - 200.0), 400, 400);
      else if (tGame.EditObj.PopupValue == 30)
        this.wup = this.AddWindow((WindowClass) new NonCardSelectWindowClass( tGame, SelectUsageMode.joinAttack),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1280, 768);
      else if (tGame.EditObj.PopupValue == 31)
        this.wup = this.AddWindow((WindowClass) new NonCardSelectWindowClass( tGame, SelectUsageMode.selectHQ),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1280, 768);
      else if (tGame.EditObj.PopupValue == 32)
        this.wup = this.AddWindow((WindowClass) new NonCardSelectWindowClass( tGame, SelectUsageMode.blowBridge),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1280, 768);
      else if (tGame.EditObj.PopupValue == 33)
        this.wup = this.AddWindow((WindowClass) new NonCardSelectWindowClass( tGame, SelectUsageMode.repairBridge),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1280, 768);
      else if (tGame.EditObj.PopupValue == 34)
        this.wup = this.AddWindow((WindowClass) new ModLibraryPickerClass( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 500.0),  Math.Round( tGame.ScreenHeight / 2.0 - 350.0), 1000, 700);
      else if (tGame.EditObj.PopupValue == 35)
      {
        this.wup = this.AddWindow((WindowClass) new NonCardSelectWindowClass( tGame, SelectUsageMode.autoMove),  Math.Round( tGame.ScreenWidth / 2.0 - 640.0),  Math.Round( tGame.ScreenHeight / 2.0 - 384.0), 1280, 768);
      }
      else
      {
        if (tGame.EditObj.PopupValue != 0)
          return;
        this.wup = this.AddWindow((WindowClass) new MessageWindowClass2( tGame),  Math.Round( tGame.ScreenWidth / 2.0 - 340.0),  Math.Round( tGame.ScreenHeight / 2.0 - 290.0), 680, 480);
      }
    }

    pub ScreenReturnClass HandleMouseClick(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      bool flag = false;
      if (this.Game.Data.Product == 6)
      {
        if (Operators.CompareString(this.GetType().FullName, "WindowsApplication1.PlayScreenClass2", false) == 0)
          flag = true;
        if (Operators.CompareString(this.GetType().FullName, "WindowsApplication1.MessagePopUpScreenClass2", false) == 0)
          flag = true;
      }
      if (!flag && x > this.Game.ScreenWidth - 52 & x < this.Game.ScreenWidth - 28 & y < 25)
        this.Game.FormRef.WindowState = FormWindowState.Minimized;
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        if (Conversions.ToBoolean(Operators.OrObject( (x > this.WindowX[index1] & x < this.WindowX[index1] + this.WindowW[index1]), Operators.CompareObjectEqual(this.WindowList[index1].allowClickOutsideWindow(),  true, false))) && Conversions.ToBoolean(Operators.OrObject( (y > this.WindowY[index1] & y < this.WindowY[index1] + this.WindowH[index1]), this.WindowList[index1].allowClickOutsideWindow())))
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

    pub ScreenReturnClass HandleTimer()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      bool flag = false;
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
      {
        windowReturnClass2: WindowReturnClass = this.WindowList[index1].handleTimer();
        this.WindowFlag[index1] = windowReturnClass2.Flag;
        if (this.WindowFlag[index1])
          flag = true;
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
        if (windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 =  windowReturnClass2.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
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
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        windowReturnClass2: WindowReturnClass = this.WindowList[windowCounter].HandleKeyup(nr);
        if (!this.WindowFlag[windowCounter])
          this.WindowFlag[windowCounter] = windowReturnClass2.Flag;
        if (windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 =  windowReturnClass2.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
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
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 =  this.WindowCounter;
      for (let mut index1: i32 =  0; index1 <= windowCounter; index1 += 1)
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
            let mut counter: i32 =  windowReturnClass.Counter;
            for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
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
