// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlayScreenClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class PlayScreenClass : ScreenClass
  {
     WLeft: i32;
     Wleft2: i32;
     WRight: i32;
     WDown: i32;
     WOrder: i32;
     WUp: i32;
     WRes: i32;
     OffSetX: i32;
     OffSetY: i32;

    pub PlayScreenClass( tgame: GameClass, tformref: Form1)
      : base( tgame, tgame.BACKGROUND1MARC, tformref)
    {
      this.AllowRightMouse = true;
      this.Game.AIThreadRunning = false;
      this.OffSetX =  Math.Round( (this.Game.ScreenWidth - 1024) / 2.0);
      this.OffSetY =  Math.Round( (this.Game.ScreenHeight - 768) / 2.0);
      if (this.Game.EditObj.CameFrom == 25)
      {
        if (this.Game.Data.MapObj[this.Game.EditObj.MapSelected].HexObj[this.Game.SelectX, this.Game.SelectY].Location == -1)
        {
          this.Game.SelectX = this.Game.EditObj.OrderX;
          this.Game.SelectY = this.Game.EditObj.OrderY;
        }
        this.WDown = this.AddWindow((WindowClass) new ProdWindowClass( tgame, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200, tgame.EditObj.TempProdList1, tgame.EditObj.TempProdList2, tgame.EditObj.TempProdList3), this.OffSetX, this.Game.ScreenHeight - 200, 1000, 200);
        if (this.Game.EditObj.ProdFlap)
        {
          this.WRight = this.AddWindow((WindowClass) new ProdFlapWindowClass( tgame, 305, this.OwnBackground, 0, 0), 0, 35, this.Game.ScreenWidth, this.Game.ScreenHeight - 305);
        }
        else
        {
          this.WLeft = this.AddWindow((WindowClass) new PlayMainWindowClass( tgame), this.Game.ScreenWidth - 220, 0, 220, this.Game.ScreenHeight - 270);
          if (this.Game.EditObj.Layout == 1)
            this.Wleft2 = this.AddWindow((WindowClass) new PlaySecondMainWindowClass( tgame), 0, 35, 220, this.Game.ScreenHeight - 305);
          this.WRight = this.Game.EditObj.Layout != 1 ? this.AddWindow((WindowClass) new MapWindowClass( tgame, 305, 220), 0, 35, this.Game.ScreenWidth - 220, this.Game.ScreenHeight - 305) : this.AddWindow((WindowClass) new MapWindowClass( tgame, 305, 440), 220, 35, this.Game.ScreenWidth - 440, this.Game.ScreenHeight - 305);
        }
        this.WUp = this.AddWindow((WindowClass) new InfoWindowClass( tgame), 0, this.Game.ScreenHeight - 270, this.Game.ScreenWidth, 20);
        this.WRes = !(this.Game.EditObj.OrderType == 24 | this.Game.EditObj.ProdFlap) ? this.AddWindow((WindowClass) new ResourceWindowClass( tgame, 0), 0, 0, this.Game.ScreenWidth - 220, 35) : this.AddWindow((WindowClass) new ResourceWindowClass( tgame, 220), 0, 0, this.Game.ScreenWidth, 35);
        this.WOrder = this.AddWindow((WindowClass) new OrderWindowClass( tgame), 0, this.Game.ScreenHeight - 250, this.Game.ScreenWidth, 40);
        this.Game.EditObj.CameFrom = -1;
      }
      else if (this.Game.EditObj.CameFrom == 65)
      {
        this.WDown = this.AddWindow((WindowClass) new SFDesignWindowClass( tgame, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200, tgame.EditObj.TempProdList1, tgame.EditObj.TempProdList2), this.OffSetX, this.Game.ScreenHeight - 200, 1000, 200);
        this.WLeft = this.AddWindow((WindowClass) new PlayMainWindowClass( tgame), this.Game.ScreenWidth - 220, 0, 220, this.Game.ScreenHeight - 270);
        if (this.Game.EditObj.Layout == 1)
          this.Wleft2 = this.AddWindow((WindowClass) new PlaySecondMainWindowClass( tgame), 0, 35, 220, this.Game.ScreenHeight - 305);
        this.WRight = this.Game.EditObj.Layout != 1 ? this.AddWindow((WindowClass) new MapWindowClass( tgame, 305, 220), 0, 35, this.Game.ScreenWidth - 220, this.Game.ScreenHeight - 305) : this.AddWindow((WindowClass) new MapWindowClass( tgame, 305, 440), 220, 35, this.Game.ScreenWidth - 440, this.Game.ScreenHeight - 305);
        this.WUp = this.AddWindow((WindowClass) new InfoWindowClass( tgame), 0, this.Game.ScreenHeight - 270, this.Game.ScreenWidth, 20);
        this.WRes = !(this.Game.EditObj.OrderType == 24 | this.Game.EditObj.ProdFlap) ? this.AddWindow((WindowClass) new ResourceWindowClass( tgame, 0), 0, 0, this.Game.ScreenWidth - 220, 35) : this.AddWindow((WindowClass) new ResourceWindowClass( tgame, 220), 0, 0, this.Game.ScreenWidth, 35);
        this.WOrder = this.AddWindow((WindowClass) new OrderWindowClass( tgame), 0, this.Game.ScreenHeight - 250, this.Game.ScreenWidth, 40);
        this.Game.EditObj.CameFrom = -1;
      }
      else
      {
        this.WDown = this.Game.EditObj.CameFrom != 45 ? this.AddWindow((WindowClass) new PlayExtraWindowClass( tgame, this.OwnBackground, 0, this.Game.ScreenHeight - 210), 0, this.Game.ScreenHeight - 210, this.Game.ScreenWidth, 210) : this.AddWindow((WindowClass) new PrefsWindowClass( this.Game, this.OwnBackground, 0, this.Game.ScreenHeight - 200),  Math.Round( (this.Game.ScreenWidth - 1024) / 2.0), this.Game.ScreenHeight - 200, 1024, 200);
        this.WLeft = this.AddWindow((WindowClass) new PlayMainWindowClass( tgame), this.Game.ScreenWidth - 220, 0, 220, this.Game.ScreenHeight - 270);
        if (this.Game.EditObj.Layout == 1)
          this.Wleft2 = this.AddWindow((WindowClass) new PlaySecondMainWindowClass( tgame), 0, 35, 220, this.Game.ScreenHeight - 305);
        this.WRight = this.Game.EditObj.Layout != 1 ? this.AddWindow((WindowClass) new MapWindowClass( tgame, 305, 220), 0, 35, this.Game.ScreenWidth - 220, this.Game.ScreenHeight - 305) : this.AddWindow((WindowClass) new MapWindowClass( tgame, 305, 440), 220, 35, this.Game.ScreenWidth - 440, this.Game.ScreenHeight - 305);
        this.WUp = this.AddWindow((WindowClass) new InfoWindowClass( tgame), 0, this.Game.ScreenHeight - 270, this.Game.ScreenWidth, 20);
        this.WRes = !(this.Game.EditObj.OrderType == 24 | this.Game.EditObj.ProdFlap) ? this.AddWindow((WindowClass) new ResourceWindowClass( tgame, 0), 0, 0, this.Game.ScreenWidth - 220, 35) : this.AddWindow((WindowClass) new ResourceWindowClass( tgame, 220), 0, 0, this.Game.ScreenWidth, 35);
        this.WOrder = this.AddWindow((WindowClass) new OrderWindowClass( tgame), 0, this.Game.ScreenHeight - 250, this.Game.ScreenWidth, 40);
      }
      this.Game.EditObj.CameFrom = -1;
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
        if (this.Game.Data.UseAI == 1)
          this.Game.NewAIObj.LastRegime = -1;
        this.Game.EditObj.ShowInitialMenu = true;
        screenReturnClass.NewScreen = 1;
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
              if (windowReturnClass.CommandType[index2] == 4)
              {
                if (windowReturnClass.CommandData[index2] == 18)
                {
                  this.WindowList[this.GetNr(this.WLeft)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WLeft)] = true;
                  this.WindowList[this.GetNr(this.Wleft2)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.Wleft2)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 40)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 20)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 12)
                {
                  this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WRight)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 53)
                {
                  this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WRight)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 52)
                {
                  this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WRight)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 44)
                {
                  this.WindowList[this.GetNr(this.WOrder)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WOrder)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 39)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 29)
                {
                  this.WindowList[this.GetNr(this.WUp)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WUp)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 66)
                {
                  this.WindowList[this.GetNr(this.WRes)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WRes)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 25)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 30)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 35)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 61)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
                if (windowReturnClass.CommandData[index2] == 63)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                }
              }
              if (windowReturnClass.CommandType[index2] == 1)
              {
                if (windowReturnClass.CommandData[index2] == 3)
                  this.RemoveWindow(this.WRight);
                if (windowReturnClass.CommandData[index2] == 8 | windowReturnClass.CommandData[index2] == 66)
                  this.RemoveWindow(this.WRes);
                if (windowReturnClass.CommandData[index2] == 2)
                {
                  this.RemoveWindow(this.WLeft);
                  this.RemoveWindow(this.Wleft2);
                }
                if (windowReturnClass.CommandData[index2] == 5)
                  this.RemoveWindow(this.WDown);
              }
              if (windowReturnClass.CommandType[index2] == 2)
              {
                if (windowReturnClass.CommandData[index2] == 18)
                {
                  this.WLeft = this.AddWindow((WindowClass) new PlayMainWindowClass( this.Game), this.Game.ScreenWidth - 220, 0, 220, this.Game.ScreenHeight - 270);
                  if (this.Game.EditObj.Layout == 1)
                    this.Wleft2 = this.AddWindow((WindowClass) new PlaySecondMainWindowClass( DrawMod.TGame), 0, 35, 220, this.Game.ScreenHeight - 305);
                }
                if (windowReturnClass.CommandData[index2] == 12)
                  this.WRight = this.Game.EditObj.Layout != 1 ? this.AddWindow((WindowClass) new MapWindowClass( DrawMod.TGame, 305, 220), 0, 35, this.Game.ScreenWidth - 220, this.Game.ScreenHeight - 305) : this.AddWindow((WindowClass) new MapWindowClass( DrawMod.TGame, 305, 440), 220, 35, this.Game.ScreenWidth - 440, this.Game.ScreenHeight - 305);
                if (windowReturnClass.CommandData[index2] == 53)
                  this.WRight = this.AddWindow((WindowClass) new BigMiniMapWindowClass( this.Game, 305, 0), 0, 35, this.Game.ScreenWidth, this.Game.ScreenHeight - 305);
                if (windowReturnClass.CommandData[index2] == 52)
                  this.WRight = this.AddWindow((WindowClass) new ProdFlapWindowClass( DrawMod.TGame, 305, this.OwnBackground, 0, 0), 0, 35, this.Game.ScreenWidth, this.Game.ScreenHeight - 305);
                if (windowReturnClass.CommandData[index2] == 66)
                  this.WRes = !(this.Game.EditObj.OrderType == 24 | this.Game.EditObj.ProdFlap) ? this.AddWindow((WindowClass) new ResourceWindowClass( DrawMod.TGame, 0), 0, 0, this.Game.ScreenWidth - 220, 35) : this.AddWindow((WindowClass) new ResourceWindowClass( DrawMod.TGame, 220), 0, 0, this.Game.ScreenWidth, 35);
                if (windowReturnClass.CommandData[index2] == 25)
                  this.WDown = this.AddWindow((WindowClass) new ProdWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 48)
                  this.WDown = this.AddWindow((WindowClass) new BuildWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 28)
                  this.WDown = this.AddWindow((WindowClass) new NewUnitWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 59)
                  this.WDown = this.AddWindow((WindowClass) new NewUnit2WindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 30)
                  this.WDown = this.AddWindow((WindowClass) new TransferWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 61)
                  this.WDown = this.AddWindow((WindowClass) new OfficerPoolWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 63)
                  this.WDown = this.AddWindow((WindowClass) new ChangeModelWindowClass2( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 51)
                  this.WDown = this.AddWindow((WindowClass) new AirSupplyWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 35)
                  this.WDown = this.AddWindow((WindowClass) new StrategicWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 20)
                  this.WDown = this.AddWindow((WindowClass) new PlayExtraWindowClass( this.Game, this.OwnBackground, 0, this.Game.ScreenHeight - 210), 0, this.Game.ScreenHeight - 210, this.Game.ScreenWidth, 210);
                if (windowReturnClass.CommandData[index2] == 45)
                  this.WDown = this.AddWindow((WindowClass) new PrefsWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 37)
                  this.WDown = this.AddWindow((WindowClass) new OldResearchWindowClass( this.Game), this.OffSetX, this.OffSetY, 1024, 768);
                if (windowReturnClass.CommandData[index2] == 65)
                  this.WDown = this.AddWindow((WindowClass) new SFDesignWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                if (windowReturnClass.CommandData[index2] == 39)
                  this.WDown = this.AddWindow((WindowClass) new DipWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
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
                this.WindowList[this.GetNr(this.WUp)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WUp)] = true;
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

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        windowReturnClass: WindowReturnClass = this.WindowList[windowCounter].handleTimer();
        if (!this.WindowFlag[windowCounter] & windowReturnClass.Flag)
          this.WindowFlag[windowCounter] = windowReturnClass.Flag;
        if (windowReturnClass.Flag)
        {
          if (windowReturnClass.Counter > -1)
          {
            let mut counter: i32 =  windowReturnClass.Counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (windowReturnClass.CommandType[index] == 3)
              {
                screenReturnClass.NewScreen = windowReturnClass.CommandData[index];
                return screenReturnClass;
              }
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
              if (windowReturnClass.CommandType[index] == 4)
              {
                if (windowReturnClass.CommandData[index] == 20)
                {
                  this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WDown)] = true;
                  screenReturnClass.flag = true;
                }
                if (windowReturnClass.CommandData[index] == 18)
                {
                  this.WindowList[this.GetNr(this.WLeft)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WLeft)] = true;
                  screenReturnClass.flag = true;
                }
                if (windowReturnClass.CommandData[index] == 12)
                {
                  this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                  this.WindowFlag[this.GetNr(this.WRight)] = true;
                  screenReturnClass.flag = true;
                }
              }
              if (windowReturnClass.CommandType[index] == 1)
              {
                if (windowReturnClass.CommandData[index] == 3)
                {
                  this.RemoveWindow(this.WRight);
                  screenReturnClass.flag = true;
                }
                if (windowReturnClass.CommandData[index] == 2)
                {
                  this.RemoveWindow(this.WLeft);
                  screenReturnClass.flag = true;
                  this.RemoveWindow(this.Wleft2);
                  screenReturnClass.flag = true;
                }
                if (windowReturnClass.CommandData[index] == 5)
                {
                  this.RemoveWindow(this.WDown);
                  screenReturnClass.flag = true;
                }
              }
              if (windowReturnClass.CommandType[index] == 2 && windowReturnClass.CommandData[index] == 35)
              {
                this.WDown = this.AddWindow((WindowClass) new StrategicWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
                screenReturnClass.flag = true;
                windowReturnClass.Flag = true;
              }
            }
          }
          if (windowReturnClass.Flag)
            screenReturnClass.flag = windowReturnClass.Flag;
        }
      }
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
            if (windowReturnClass2.CommandType[index] == 5)
            {
              screenReturnClass.OpenPopUp = true;
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index];
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index] == 6)
            {
              screenReturnClass.ClosePopUp = true;
              screenReturnClass.NewScreen = 0;
              return screenReturnClass;
            }
            if (windowReturnClass2.CommandType[index] == 4)
            {
              if (windowReturnClass2.CommandData[index] == 18)
              {
                this.WindowList[this.GetNr(this.WLeft)].DoRefresh();
                this.WindowList[this.GetNr(this.Wleft2)].DoRefresh();
                this.WindowFlag[this.GetNr(this.Wleft2)] = true;
                this.WindowFlag[this.GetNr(this.WLeft)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 40)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 20)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 12)
              {
                this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WRight)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 53)
              {
                this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WRight)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 52)
              {
                this.WindowList[this.GetNr(this.WRight)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WRight)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 44)
              {
                this.WindowList[this.GetNr(this.WOrder)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WOrder)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 39)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 29)
              {
                this.WindowList[this.GetNr(this.WUp)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WUp)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 66)
              {
                this.WindowList[this.GetNr(this.WRes)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WRes)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 25)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 30)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 35)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 61)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
              if (windowReturnClass2.CommandData[index] == 63)
              {
                this.WindowList[this.GetNr(this.WDown)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WDown)] = true;
              }
            }
            if (windowReturnClass2.CommandType[index] == 1)
            {
              if (windowReturnClass2.CommandData[index] == 3)
                this.RemoveWindow(this.WRight);
              if (windowReturnClass2.CommandData[index] == 8)
                this.RemoveWindow(this.WRes);
              if (windowReturnClass2.CommandData[index] == 8 | windowReturnClass2.CommandData[index] == 66)
                this.RemoveWindow(this.WRes);
              if (windowReturnClass2.CommandData[index] == 2)
              {
                this.RemoveWindow(this.WLeft);
                this.RemoveWindow(this.Wleft2);
              }
              if (windowReturnClass2.CommandData[index] == 5)
                this.RemoveWindow(this.WDown);
            }
            if (windowReturnClass2.CommandType[index] == 2)
            {
              if (windowReturnClass2.CommandData[index] == 18)
              {
                this.WLeft = this.AddWindow((WindowClass) new PlayMainWindowClass( this.Game), this.Game.ScreenWidth - 220, 0, 220, this.Game.ScreenHeight - 270);
                if (this.Game.EditObj.Layout == 1)
                  this.Wleft2 = this.AddWindow((WindowClass) new PlaySecondMainWindowClass( DrawMod.TGame), 0, 35, 220, this.Game.ScreenHeight - 305);
              }
              if (windowReturnClass2.CommandData[index] == 12)
                this.WRight = this.Game.EditObj.Layout != 1 ? this.AddWindow((WindowClass) new MapWindowClass( DrawMod.TGame, 305, 220), 0, 35, this.Game.ScreenWidth - 220, this.Game.ScreenHeight - 305) : this.AddWindow((WindowClass) new MapWindowClass( DrawMod.TGame, 305, 440), 220, 35, this.Game.ScreenWidth - 440, this.Game.ScreenHeight - 305);
              if (windowReturnClass2.CommandData[index] == 53)
                this.WRight = this.AddWindow((WindowClass) new BigMiniMapWindowClass( this.Game, 305, 0), 0, 35, this.Game.ScreenWidth, this.Game.ScreenHeight - 305);
              if (windowReturnClass2.CommandData[index] == 52)
                this.WRight = this.AddWindow((WindowClass) new ProdFlapWindowClass( DrawMod.TGame, 305, this.OwnBackground, 0, 0), 0, 35, this.Game.ScreenWidth, this.Game.ScreenHeight - 305);
              if (windowReturnClass2.CommandData[index] == 66)
                this.WRes = !(this.Game.EditObj.OrderType == 24 | this.Game.EditObj.ProdFlap) ? this.AddWindow((WindowClass) new ResourceWindowClass( DrawMod.TGame, 0), 0, 0, this.Game.ScreenWidth - 220, 35) : this.AddWindow((WindowClass) new ResourceWindowClass( DrawMod.TGame, 220), 0, 0, this.Game.ScreenWidth, 35);
              if (windowReturnClass2.CommandData[index] == 25)
                this.WDown = this.AddWindow((WindowClass) new ProdWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 48)
                this.WDown = this.AddWindow((WindowClass) new BuildWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 28)
                this.WDown = this.AddWindow((WindowClass) new NewUnitWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 59)
                this.WDown = this.AddWindow((WindowClass) new NewUnit2WindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 30)
                this.WDown = this.AddWindow((WindowClass) new TransferWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 61)
                this.WDown = this.AddWindow((WindowClass) new OfficerPoolWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 63)
                this.WDown = this.AddWindow((WindowClass) new ChangeModelWindowClass2( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 51)
                this.WDown = this.AddWindow((WindowClass) new AirSupplyWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 35)
                this.WDown = this.AddWindow((WindowClass) new StrategicWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 20)
                this.WDown = this.AddWindow((WindowClass) new PlayExtraWindowClass( this.Game, this.OwnBackground, 0, this.Game.ScreenHeight - 210), 0, this.Game.ScreenHeight - 210, this.Game.ScreenWidth, 210);
              if (windowReturnClass2.CommandData[index] == 45)
                this.WDown = this.AddWindow((WindowClass) new PrefsWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 38)
                this.WDown = this.AddWindow((WindowClass) new OldResearchWindowClass( this.Game), this.OffSetX, this.OffSetY, 1024, 768);
              if (windowReturnClass2.CommandData[index] == 65)
                this.WDown = this.AddWindow((WindowClass) new SFDesignWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
              if (windowReturnClass2.CommandData[index] == 39)
                this.WDown = this.AddWindow((WindowClass) new DipWindowClass( this.Game, this.OwnBackground, this.OffSetX, this.Game.ScreenHeight - 200), this.OffSetX, this.Game.ScreenHeight - 200, 1024, 200);
            }
          }
        }
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }
  }
}
