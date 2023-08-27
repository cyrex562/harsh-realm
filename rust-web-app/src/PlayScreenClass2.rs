// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlayScreenClass2
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
  pub class PlayScreenClass2 : ScreenClass
  {
     WMap: i32;
     WPlay: i32;
     WOrder: i32;
     WRes: i32;
     WTabs: i32;
     WLeft: i32;
     wRight: i32;
     wAdvice: i32;
     wOps: i32;
     OffSetX: i32;
     lastx: i32;
     lasty: i32;
     playExtraHeight: i32;
     rightSideBarWidth: i32;
     leftSideBarWidth: i32;

    pub PlayScreenClass2( tgame: GameClass, tformref: Form1)
      : base( tgame, -3, tformref)
    {
      this.playExtraHeight = 222;
      this.rightSideBarWidth = 185;
      this.leftSideBarWidth = 185;
      if (this.Game.Data.Product == 6)
      {
        this.rightSideBarWidth = 320;
        this.leftSideBarWidth = 250;
        this.playExtraHeight = 150;
      }
      if (this.Game.Data.Product == 6 && this.Game.EditObj.SetViewMode2 == 7)
        this.Game.EditObj.SetViewMode2 = 0;
      if (this.Game.Data.Product >= 5)
      {
        this.Game.HandyFunctionsObj.RedimTempValue(9999);
        this.Game.HandyFunctionsObj.RedimTempLosValue(0);
        this.Game.HandyFunctionsObj.RedimTempValue(9999);
        this.Game.HandyFunctionsObj.RedimTempValueSpecial(0);
        this.Game.HandyFunctionsObj.RedimTempValueSpecial2(0);
        this.Game.HandyFunctionsObj.RedimTempValue2(0);
        this.Game.HandyFunctionsObj.RedimTempCameFrom();
        this.Game.HandyFunctionsObj.RedimTempAttack(false);
      }
      this.Game.HandyFunctionsObj.RedimTempValue3(9999);
      if ( this.Game.Data.RuleVar[701] > 0.0)
        this.Game.HandyFunctionsObj.RedimTempValue4(9999);
      this.OffSetX =  Math.Round( (this.Game.ScreenWidth - 1024) / 2.0);
      this.Game.HandyFunctionsObj.SetGameColors();
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (this.Game.EditObj.GuiDown)
      {
        if ( this.Game.Data.RuleVar[408] > 0.0)
        {
          if (this.Game.EditObj.RightDown)
          {
            MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, tminwidth: 0);
            let mut screenWidth: i32 =  this.Game.ScreenWidth;
            let mut h: i32 =  this.Game.ScreenHeight - 0;
            Rectangle rectangle3 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 165);
            let mut tShowRectangle: &Rectangle = &rectangle3
            this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
             let mut local1: GameClass =  tgame;
             WindowClass local2 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle3 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
            rectangle1 = rectangle3;
             Rectangle local3 =  rectangle1;
            this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local1,  local2,  local3), 0, 0, this.Game.ScreenWidth, 75);
            if (!this.Game.EditObj.LeftDown)
            {
               let mut local4: GameClass =  tgame;
              let mut theight: i32 =  this.Game.ScreenHeight - 165;
               WindowClass local5 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle1 = Rectangle::new(0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - 165);
              rectangle2 = rectangle1;
               Rectangle local6 =  rectangle2;
              this.WLeft = this.AddWindow((WindowClass) new LeftSideBar( local4, theight,  local5,  local6), 0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - 165);
            }
          }
          else
          {
            MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, tminwidth: 0);
            let mut screenWidth: i32 =  this.Game.ScreenWidth;
            let mut h: i32 =  this.Game.ScreenHeight - 0;
            rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 165);
            let mut tShowRectangle: &Rectangle = &rectangle1
            this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
             let mut local7: GameClass =  tgame;
             WindowClass local8 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
            let mut rectangle4: &Rectangle = &rectangle1
             Rectangle local9 =  rectangle4;
            this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local7,  local8,  local9), 0, 0, this.Game.ScreenWidth, 75);
             let mut local10: GameClass =  tgame;
            let mut theight1: i32 =  this.Game.ScreenHeight - 165;
             WindowClass local11 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - 165);
            rectangle2 = rectangle1;
             Rectangle local12 =  rectangle2;
            this.WLeft = this.AddWindow((WindowClass) new LeftSideBar( local10, theight1,  local11,  local12), 0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - 165);
             let mut local13: GameClass =  tgame;
            let mut theight2: i32 =  this.Game.ScreenHeight - 165;
             WindowClass local14 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - 165);
            rectangle2 = rectangle1;
             Rectangle local15 =  rectangle2;
            this.wRight = this.AddWindow((WindowClass) new RightSideBar( local13, theight2,  local14,  local15), this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - 165);
          }
          if (!this.Game.EditObj.BlockAdvice & !this.Game.EditObj.TempBlockAdvice)
          {
             let mut local16: GameClass =  tgame;
             WindowClass local17 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(185, 75, 620, 240);
            rectangle2 = rectangle1;
             Rectangle local18 =  rectangle2;
            this.wAdvice = this.AddWindow((WindowClass) new AdviceWindow( local16,  local17,  local18), this.rightSideBarWidth, 75, 620, 240);
          }
        }
        else
        {
          MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, tminwidth: 0);
          let mut screenWidth: i32 =  this.Game.ScreenWidth;
          let mut h: i32 =  this.Game.ScreenHeight - 0;
          rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 165);
          let mut tShowRectangle: &Rectangle = &rectangle1
          this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
           let mut local19: GameClass =  tgame;
           WindowClass local20 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
          let mut rectangle5: &Rectangle = &rectangle1
           Rectangle local21 =  rectangle5;
          this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local19,  local20,  local21), 0, 0, this.Game.ScreenWidth, 75);
          if (!this.Game.EditObj.LeftDown)
          {
             let mut local22: GameClass =  tgame;
            let mut theight: i32 =  this.Game.ScreenHeight - 165;
             WindowClass local23 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - 165);
            rectangle2 = rectangle1;
             Rectangle local24 =  rectangle2;
            this.WLeft = this.AddWindow((WindowClass) new LeftSideBar( local22, theight,  local23,  local24), 0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - 165);
          }
          if (!this.Game.EditObj.RightDown)
          {
             let mut local25: GameClass =  tgame;
            let mut theight: i32 =  this.Game.ScreenHeight - 165;
             WindowClass local26 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - 165);
            rectangle2 = rectangle1;
             Rectangle local27 =  rectangle2;
            this.wRight = this.AddWindow((WindowClass) new RightSideBar( local25, theight,  local26,  local27), this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - 165);
          }
        }
        if ( this.Game.Data.RuleVar[408] > 0.0)
        {
           let mut local28: GameClass =  tgame;
           WindowClass local29 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, this.Game.ScreenHeight - 90, this.Game.ScreenWidth, 90);
          rectangle2 = rectangle1;
           Rectangle local30 =  rectangle2;
          this.WOrder = this.AddWindow((WindowClass) new UdsOrderWindowClass( local28,  local29,  local30), 0, this.Game.ScreenHeight - 90, this.Game.ScreenWidth, 90);
        }
        else
        {
           let mut local31: GameClass =  tgame;
           WindowClass local32 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, this.Game.ScreenHeight - 90, this.Game.ScreenWidth, 90);
          rectangle2 = rectangle1;
           Rectangle local33 =  rectangle2;
          this.WOrder = this.AddWindow((WindowClass) new OrderWindowClass2( local31,  local32,  local33), 0, this.Game.ScreenHeight - 90, this.Game.ScreenWidth, 90);
        }
        this.WPlay = -1;
      }
      else
      {
        if ( this.Game.Data.RuleVar[408] > 0.0)
        {
          MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, this.playExtraHeight, 0);
          let mut screenWidth: i32 =  this.Game.ScreenWidth;
          let mut h: i32 =  this.Game.ScreenHeight - this.playExtraHeight;
          rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          let mut tShowRectangle: &Rectangle = &rectangle1
          this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
           let mut local34: GameClass =  tgame;
           WindowClass local35 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
          let mut rectangle6: &Rectangle = &rectangle1
           Rectangle local36 =  rectangle6;
          this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local34,  local35,  local36), 0, 0, this.Game.ScreenWidth, 75);
           let mut local37: GameClass =  tgame;
          let mut theight3: i32 =  this.Game.ScreenHeight - (165 + this.playExtraHeight);
           WindowClass local38 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          rectangle2 = rectangle1;
           Rectangle local39 =  rectangle2;
          this.WLeft = this.AddWindow((WindowClass) new LeftSideBar( local37, theight3,  local38,  local39), 0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          if (!this.Game.EditObj.RightDown)
          {
             let mut local40: GameClass =  tgame;
            let mut theight4: i32 =  this.Game.ScreenHeight - (165 + this.playExtraHeight);
             WindowClass local41 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
            rectangle2 = rectangle1;
             Rectangle local42 =  rectangle2;
            this.wRight = this.AddWindow((WindowClass) new RightSideBar( local40, theight4,  local41,  local42), this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          }
          if (!this.Game.EditObj.BlockAdvice & !this.Game.EditObj.TempBlockAdvice)
          {
             let mut local43: GameClass =  tgame;
             WindowClass local44 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(185, 75, 820, 240);
            rectangle2 = rectangle1;
             Rectangle local45 =  rectangle2;
            this.wAdvice = this.AddWindow((WindowClass) new AdviceWindow( local43,  local44,  local45), 185, 75, 820, 240);
          }
        }
        else
        {
          MapWindowClass2 tmpWindow = new MapWindowClass2( tgame, this.playExtraHeight, 0);
          let mut screenWidth: i32 =  this.Game.ScreenWidth;
          let mut h: i32 =  this.Game.ScreenHeight - this.playExtraHeight;
          rectangle1 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          let mut tShowRectangle: &Rectangle = &rectangle1
          this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
           let mut local46: GameClass =  tgame;
           WindowClass local47 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
          let mut rectangle7: &Rectangle = &rectangle1
           Rectangle local48 =  rectangle7;
          this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local46,  local47,  local48), 0, 0, this.Game.ScreenWidth, 75);
          if (!this.Game.EditObj.LeftDown & this.Game.Data.Product >= 6)
          {
             let mut local49: GameClass =  tgame;
            let mut theight: i32 =  this.Game.ScreenHeight - (165 + this.playExtraHeight);
             WindowClass local50 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
            rectangle2 = rectangle1;
             Rectangle local51 =  rectangle2;
            this.WLeft = this.AddWindow((WindowClass) new LeftSideBar( local49, theight,  local50,  local51), 0, 75, this.leftSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          }
          if (!this.Game.EditObj.RightDown & this.Game.Data.Product >= 6)
          {
             let mut local52: GameClass =  tgame;
            let mut theight: i32 =  this.Game.ScreenHeight - (165 + this.playExtraHeight);
             WindowClass local53 =  this.WindowList[this.GetNr(this.WMap)];
            rectangle1 = Rectangle::new(this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
            rectangle2 = rectangle1;
             Rectangle local54 =  rectangle2;
            this.wRight = this.AddWindow((WindowClass) new RightSideBar( local52, theight,  local53,  local54), this.Game.ScreenWidth - this.rightSideBarWidth, 75, this.rightSideBarWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
          }
        }
        if ( this.Game.Data.RuleVar[408] > 0.0)
        {
           let mut local55: GameClass =  tgame;
           WindowClass local56 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, this.Game.ScreenHeight - 312, this.Game.ScreenWidth, 90);
          rectangle2 = rectangle1;
           Rectangle local57 =  rectangle2;
          this.WOrder = this.AddWindow((WindowClass) new UdsOrderWindowClass( local55,  local56,  local57), 0, this.Game.ScreenHeight - 312, this.Game.ScreenWidth, 90);
        }
        else
        {
           let mut local58: GameClass =  tgame;
           WindowClass local59 =  this.WindowList[this.GetNr(this.WMap)];
          rectangle1 = Rectangle::new(0, this.Game.ScreenHeight - (90 + this.playExtraHeight), this.Game.ScreenWidth, 90);
          rectangle2 = rectangle1;
           Rectangle local60 =  rectangle2;
          this.WOrder = this.AddWindow((WindowClass) new OrderWindowClass2( local58,  local59,  local60), 0, this.Game.ScreenHeight - (90 + this.playExtraHeight), this.Game.ScreenWidth, 90);
        }
        this.WPlay = !(this.Game.EditObj.OrderType == 51 | this.Game.EditObj.LayerSupplyOn) ? this.AddWindow((WindowClass) new PlayExtraWindowClass2( tgame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight) : this.AddWindow((WindowClass) new SupplyLayerWindowClass( tgame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
      }
      Rectangle rectForTab;
      if (this.Game.EditObj.SetViewMode2 == 1)
      {
        rectForTab = DrawMod.GetRectForTab(1);
         let mut local61: GameClass =  tgame;
         WindowClass local62 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local63 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabBriefingWindowClass2( local61,  local62,  local63, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 2)
      {
        rectForTab = DrawMod.GetRectForTab(2);
         let mut local64: GameClass =  tgame;
         WindowClass local65 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local66 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabStatsWindowClass2( local64,  local65,  local66, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 3)
      {
        rectForTab = DrawMod.GetRectForTab(3);
         let mut local67: GameClass =  tgame;
         WindowClass local68 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local69 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabOOBWindowClass2( local67,  local68,  local69, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 4)
      {
        rectForTab = DrawMod.GetRectForTab(4);
         let mut local70: GameClass =  tgame;
         WindowClass local71 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local72 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabReportsWindowClass2( local70,  local71,  local72, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 5)
      {
        rectForTab = DrawMod.GetRectForTab(5);
         let mut local73: GameClass =  tgame;
         WindowClass local74 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local75 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabCardsWindowClass2( local73,  local74,  local75, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 6)
      {
        rectForTab = DrawMod.GetRectForTab(6);
         let mut local76: GameClass =  tgame;
         WindowClass local77 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local78 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabSMapWindowClass2( local76,  local77,  local78, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 7)
      {
        rectForTab = DrawMod.GetRectForTab(7);
         let mut local79: GameClass =  tgame;
         WindowClass local80 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local81 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabMiniMapWindowClass2( local79,  local80,  local81, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 8)
      {
        rectForTab = DrawMod.GetRectForTab(8);
         let mut local82: GameClass =  tgame;
         WindowClass local83 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local84 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabPrefsWindowClass2( local82,  local83,  local84, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (this.Game.EditObj.SetViewMode2 == 9)
      {
        rectForTab = DrawMod.GetRectForTab(9);
         let mut local85: GameClass =  tgame;
         WindowClass local86 =  this.WindowList[this.GetNr(this.WMap)];
        rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
        rectangle2 = rectangle1;
         Rectangle local87 =  rectangle2;
        let mut trect: &Rectangle = &rectForTab
        this.WTabs = this.AddWindow((WindowClass) new TabHelpWindowClass2( local85,  local86,  local87, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      }
      if (!(this.Game.EditObj.SetViewMode2 >= 11 & this.Game.EditObj.SetViewMode2 <= 13))
        return;
      rectForTab = DrawMod.GetRectForTab(this.Game.EditObj.SetViewMode2);
       let mut local88: GameClass =  tgame;
       WindowClass local89 =  this.WindowList[this.GetNr(this.WMap)];
      rectangle1 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
      rectangle2 = rectangle1;
       Rectangle local90 =  rectangle2;
      let mut trect1: &Rectangle = &rectForTab
      this.WTabs = this.AddWindow((WindowClass) new TabManagementWindowClass2( local88,  local89,  local90, trect1), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
    }

    pub ScreenReturnClass HandleWR(
      wr: WindowReturnClass,
      bool setflag,
       ScreenReturnClass obj)
    {
      if (wr.Counter > -1)
      {
        let mut counter: i32 =  wr.Counter;
        for (let mut index1: i32 =  0; index1 <= counter; index1 += 1)
        {
          if (wr.CommandType[index1] == 3)
          {
            obj.NewScreen = wr.CommandData[index1];
            return obj;
          }
          if (wr.CommandType[index1] == 5)
          {
            obj.OpenPopUp = true;
            obj.NewScreen = wr.CommandData[index1];
            return obj;
          }
          if (wr.CommandType[index1] == 6)
          {
            obj.ClosePopUp = true;
            obj.NewScreen = 0;
            return obj;
          }
          if (wr.CommandType[index1] == 4)
          {
            if (wr.CommandData[index1] == 69 & this.WPlay > -1)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
            if (wr.CommandData[index1] == 87 & this.WPlay > -1)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
            if (wr.CommandData[index1] == 88 & this.WPlay > -1)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
            if (wr.CommandData[index1] == 90 & this.WPlay > -1)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
            if (wr.CommandData[index1] == 89 & this.WPlay > -1)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
            if (wr.CommandData[index1] == 12)
            {
              this.WindowList[this.GetNr(this.WMap)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WMap)] = true;
            }
            if (wr.CommandData[index1] == 68)
            {
              this.WindowList[this.GetNr(this.WOrder)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WOrder)] = true;
            }
            if (wr.CommandData[index1] == 3 & this.wRight > 0)
            {
              this.WindowList[this.GetNr(this.wRight)].DoRefresh();
              this.WindowFlag[this.GetNr(this.wRight)] = true;
            }
            if (wr.CommandData[index1] == 2 & this.WLeft > 0)
            {
              this.WindowList[this.GetNr(this.wRight)].DoRefresh();
              this.WindowFlag[this.GetNr(this.wRight)] = true;
            }
            if (wr.CommandData[index1] == 67)
            {
              this.WindowList[this.GetNr(this.WRes)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WRes)] = true;
              if (this.WLeft > 0)
              {
                this.WindowList[this.GetNr(this.WLeft)].DoRefresh();
                this.WindowFlag[this.GetNr(this.WLeft)] = true;
              }
              if (this.wRight > 0)
              {
                this.WindowList[this.GetNr(this.wRight)].DoRefresh();
                this.WindowFlag[this.GetNr(this.wRight)] = true;
              }
              if (this.wAdvice > 0 & this.Game.EditObj.SetViewMode2 < 1)
              {
                this.WindowList[this.GetNr(this.wAdvice)].DoRefresh();
                this.WindowFlag[this.GetNr(this.wAdvice)] = true;
              }
            }
            if (wr.CommandData[index1] == 9 & this.WTabs > 0)
            {
              this.WindowList[this.GetNr(this.WTabs)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WTabs)] = true;
            }
            if (wr.CommandData[index1] == 35 & this.WPlay > -1)
            {
              this.WindowList[this.GetNr(this.WPlay)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WPlay)] = true;
            }
          }
          if (wr.CommandType[index1] == 7)
          {
            this.WindowFlag[this.GetNr(this.WMap)] = true;
            if (this.WLeft > 0)
            {
              this.WindowList[this.GetNr(this.WLeft)].DoRefresh();
              this.WindowFlag[this.GetNr(this.WLeft)] = true;
            }
            if (this.wRight > 0)
            {
              this.WindowList[this.GetNr(this.wRight)].DoRefresh();
              this.WindowFlag[this.GetNr(this.wRight)] = true;
            }
            if (this.wAdvice > 0)
            {
              this.WindowList[this.GetNr(this.wAdvice)].DoRefresh();
              this.WindowFlag[this.GetNr(this.wAdvice)] = true;
            }
          }
          if (wr.CommandType[index1] == 1)
          {
            if (wr.CommandData[index1] == 12)
            {
              this.RemoveWindow(this.WMap);
              this.WMap = 0;
            }
            if (wr.CommandData[index1] == 67)
            {
              this.RemoveWindow(this.WRes);
              this.WRes = 0;
            }
            if (wr.CommandData[index1] == 69 | wr.CommandData[index1] == 5)
            {
              this.RemoveWindow(this.WPlay);
              this.WPlay = 0;
            }
            if (wr.CommandData[index1] == 68)
            {
              this.RemoveWindow(this.WOrder);
              this.WOrder = 0;
            }
            if (wr.CommandData[index1] == 119 & this.wAdvice > 0)
            {
              this.RemoveWindow(this.wAdvice);
              this.wAdvice = 0;
            }
            if (wr.CommandData[index1] == 118)
            {
              let mut windowCounter: i32 =  this.WindowCounter;
              for (let mut index2: i32 =  0; index2 <= windowCounter; index2 += 1)
                this.WindowInputBlock[index2] = false;
              this.RemoveWindow(this.wOps);
              this.wOps = 0;
            }
            if (wr.CommandData[index1] == 9)
            {
              this.RemoveWindow(this.WTabs);
              this.WTabs = 0;
              this.WTabs = 0;
            }
          }
          if (wr.CommandType[index1] == 2)
          {
            Rectangle rectangle1;
            Rectangle rectangle2;
            if (wr.CommandData[index1] == 119 && !this.Game.EditObj.BlockAdvice & !this.Game.EditObj.TempBlockAdvice)
            {
               let mut local1: GameClass =  DrawMod.TGame;
               WindowClass local2 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle1 = Rectangle::new(185, 75, 820, 240);
              rectangle2 = rectangle1;
               Rectangle local3 =  rectangle2;
              this.wAdvice = this.AddWindow((WindowClass) new AdviceWindow( local1,  local2,  local3), 185, 75, 820, 240);
            }
            if (wr.CommandData[index1] == 12)
            {
              if (this.Game.EditObj.GuiDown)
              {
                MapWindowClass2 tmpWindow = new MapWindowClass2( DrawMod.TGame, tminwidth: 0);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut h: i32 =  this.Game.ScreenHeight - 0;
                rectangle2 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - 165);
                let mut tShowRectangle: &Rectangle = &rectangle2
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
              }
              else
              {
                MapWindowClass2 tmpWindow = new MapWindowClass2( DrawMod.TGame, this.playExtraHeight, 0);
                let mut screenWidth: i32 =  this.Game.ScreenWidth;
                let mut h: i32 =  this.Game.ScreenHeight - this.playExtraHeight;
                rectangle2 = Rectangle::new(0, 75, this.Game.ScreenWidth, this.Game.ScreenHeight - (165 + this.playExtraHeight));
                let mut tShowRectangle: &Rectangle = &rectangle2
                this.WMap = this.AddWindow((WindowClass) tmpWindow, 0, 0, screenWidth, h, tShowRectangle);
              }
              if (this.WRes > 0)
                this.WindowList[this.GetNr(this.WRes)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.WOrder > 0)
                this.WindowList[this.GetNr(this.WOrder)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.WTabs > 0)
                this.WindowList[this.GetNr(this.WTabs)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.wRight > 0)
                this.WindowList[this.GetNr(this.wRight)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.WLeft > 0)
                this.WindowList[this.GetNr(this.WLeft)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
              if (this.wAdvice > 0)
                this.WindowList[this.GetNr(this.wAdvice)].LowerWindow = this.WindowList[this.GetNr(this.WMap)];
            }
            if (wr.CommandData[index1] == 118)
            {
              let mut num1: i32 =  this.Game.ScreenHeight - (165 + this.playExtraHeight);
              if (this.Game.EditObj.GuiDown)
                num1 = this.Game.ScreenHeight - 165;
              let mut x: i32 =  this.FormRef.LastMouseX - 120;
              let mut y: i32 =  this.FormRef.LastMouseY <= 375 ? this.FormRef.LastMouseY : this.FormRef.LastMouseY - 300;
              if (x < 100)
                x = 100;
              if (x > this.Game.ScreenWidth - 350)
                x = this.Game.ScreenWidth - 350;
              let mut windowCounter: i32 =  this.WindowCounter;
              for (let mut index3: i32 =  0; index3 <= windowCounter; index3 += 1)
                this.WindowInputBlock[index3] = true;
              let mut num2: i32 =  340;
              if (this.Game.Data.Product == 7 && this.Game.EventRelatedObj.Helper_AirEnabled())
                num2 = 430;
               let mut local4: GameClass =  DrawMod.TGame;
               WindowClass local5 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(x, y, 200, num2);
              rectangle1 = rectangle2;
               Rectangle local6 =  rectangle1;
               let mut local7: i32 =   num2;
              this.wOps = this.AddWindow((WindowClass) new UdsUnitOpsWindowClass( local4,  local5,  local6,  local7), x, y, 200, num2);
            }
            if (wr.CommandData[index1] == 67)
            {
               let mut local8: GameClass =  DrawMod.TGame;
               WindowClass local9 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(0, 0, this.Game.ScreenWidth, 75);
              rectangle1 = rectangle2;
               Rectangle local10 =  rectangle1;
              this.WRes = this.AddWindow((WindowClass) new ResourceWindowClass2( local8,  local9,  local10), 0, 0, this.Game.ScreenWidth, 75);
            }
            Rectangle rectForTab;
            if (wr.CommandData[index1] == 70)
            {
              rectForTab = DrawMod.GetRectForTab(1);
               let mut local11: GameClass =  DrawMod.TGame;
               WindowClass local12 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local13 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabBriefingWindowClass2( local11,  local12,  local13, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 71)
            {
              rectForTab = DrawMod.GetRectForTab(2);
               let mut local14: GameClass =  DrawMod.TGame;
               WindowClass local15 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local16 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabStatsWindowClass2( local14,  local15,  local16, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 113)
            {
              rectForTab = DrawMod.GetRectForTab(this.Game.EditObj.SetViewMode2);
               let mut local17: GameClass =  DrawMod.TGame;
               WindowClass local18 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local19 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabManagementWindowClass2( local17,  local18,  local19, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 72)
            {
              rectForTab = DrawMod.GetRectForTab(3);
               let mut local20: GameClass =  DrawMod.TGame;
               WindowClass local21 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local22 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabOOBWindowClass2( local20,  local21,  local22, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 73)
            {
              rectForTab = DrawMod.GetRectForTab(4);
               let mut local23: GameClass =  DrawMod.TGame;
               WindowClass local24 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local25 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabReportsWindowClass2( local23,  local24,  local25, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 74)
            {
              rectForTab = DrawMod.GetRectForTab(5);
               let mut local26: GameClass =  DrawMod.TGame;
               WindowClass local27 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local28 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabCardsWindowClass2( local26,  local27,  local28, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 75)
            {
              rectForTab = DrawMod.GetRectForTab(6);
               let mut local29: GameClass =  DrawMod.TGame;
               WindowClass local30 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local31 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabSMapWindowClass2( local29,  local30,  local31, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 76)
            {
              rectForTab = DrawMod.GetRectForTab(7);
               let mut local32: GameClass =  DrawMod.TGame;
               WindowClass local33 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local34 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabMiniMapWindowClass2( local32,  local33,  local34, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 77)
            {
              rectForTab = DrawMod.GetRectForTab(8);
               let mut local35: GameClass =  DrawMod.TGame;
               WindowClass local36 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local37 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabPrefsWindowClass2( local35,  local36,  local37, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 110)
            {
              rectForTab = DrawMod.GetRectForTab(9);
               let mut local38: GameClass =  DrawMod.TGame;
               WindowClass local39 =  this.WindowList[this.GetNr(this.WMap)];
              rectangle2 = Rectangle::new(rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
              rectangle1 = rectangle2;
               Rectangle local40 =  rectangle1;
              let mut trect: &Rectangle = &rectForTab
              this.WTabs = this.AddWindow((WindowClass) new TabHelpWindowClass2( local38,  local39,  local40, trect), rectForTab.X, rectForTab.Y, rectForTab.Width, rectForTab.Height);
            }
            if (wr.CommandData[index1] == 35)
              this.WPlay = this.AddWindow((WindowClass) new StrategicWindowClass2( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
            if (wr.CommandData[index1] == 69)
              this.WPlay = this.AddWindow((WindowClass) new PlayExtraWindowClass2( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
            if (wr.CommandData[index1] == 87)
              this.WPlay = this.AddWindow((WindowClass) new OfficerPoolWindowClass2( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
            if (wr.CommandData[index1] == 88)
              this.WPlay = this.AddWindow((WindowClass) new NewUnitWindowClass2( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
            if (wr.CommandData[index1] == 121)
              this.WPlay = this.AddWindow((WindowClass) new SupplyLayerWindowClass( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
            if (wr.CommandData[index1] == 90)
              this.WPlay = this.AddWindow((WindowClass) new ChangeModelWindowClass2( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
            if (wr.CommandData[index1] == 89)
              this.WPlay = this.AddWindow((WindowClass) new NewUnit2WindowClass2( DrawMod.TGame), 0, this.Game.ScreenHeight - this.playExtraHeight, this.Game.ScreenWidth, this.playExtraHeight);
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
      if (this.Game.Data.Product < 6)
      {
        if (x > this.Game.ScreenWidth - 52 & x < this.Game.ScreenWidth - 28 & y < 25)
          this.Game.FormRef.WindowState = FormWindowState.Minimized;
        if (x > this.Game.ScreenWidth - 28 & x < this.Game.ScreenWidth - 4 & y < 25 && Interaction.MsgBox( "Are you sure you want to exit your current game?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
        {
          this.Game.Data = DataClass::new();
          this.Game.EditObj = new EditClass(this.Game.AppPath + "editobj.txt");
          if (this.Game.Data.UseAI == 1)
          {
            if (Information.IsNothing( this.Game.NewAIObj))
              this.Game.NewAIObj = new NewAIClass(this.Game);
            this.Game.NewAIObj.LastRegime = -1;
          }
          this.Game.EditObj.ShowInitialMenu = true;
          screenReturnClass.NewScreen = 12;
          return screenReturnClass;
        }
      }
      if (this.WindowCounter > -1)
      {
        for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Operators.CompareString(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 & !this.WindowInputBlock[windowCounter])
          {
            bool flag = false;
            if (x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
            {
              flag = true;
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
            if (!flag)
            {
              wr: WindowReturnClass = this.WindowList[windowCounter].HandleMouseClickOutsideWindow(x - this.WindowX[windowCounter], y - this.WindowY[windowCounter], b);
              if (wr.Flag | wr.alwaysExecuteWR)
              {
                this.HandleWR(wr, true,  screenReturnClass);
                screenReturnClass.flag = true;
                return screenReturnClass;
              }
            }
          }
        }
        for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (Operators.CompareString(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 & !this.WindowInputBlock[windowCounter])
          {
            bool flag = false;
            if (x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
            {
              flag = true;
              wr: WindowReturnClass = this.WindowList[windowCounter].HandleMouseClick(x - this.WindowX[windowCounter], y - this.WindowY[windowCounter], b);
              this.WindowFlag[windowCounter] = wr.Flag;
              if (wr.Flag | wr.alwaysExecuteWR)
              {
                this.HandleWR(wr, true,  screenReturnClass);
                screenReturnClass.flag = true;
                return screenReturnClass;
              }
              if (wr.NoMouseClickBelow)
              {
                screenReturnClass.flag = false;
                return screenReturnClass;
              }
            }
            if (!flag)
            {
              wr: WindowReturnClass = this.WindowList[windowCounter].HandleMouseClickOutsideWindow(x - this.WindowX[windowCounter], y - this.WindowY[windowCounter], b);
              this.WindowFlag[windowCounter] = wr.Flag;
              if (wr.Flag | wr.alwaysExecuteWR)
              {
                this.HandleWR(wr, true,  screenReturnClass);
                screenReturnClass.flag = true;
                return screenReturnClass;
              }
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
      if (this.WindowCounter > -1)
      {
        for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
        {
          if (x > this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] & !this.WindowInputBlock[windowCounter] && y > this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter] && !Information.IsNothing( this.WindowList[windowCounter]))
          {
            wr: WindowReturnClass = this.WindowList[windowCounter].HandleMouseUp(x - this.WindowX[windowCounter], y - this.WindowY[windowCounter], b);
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
      let mut num1: i32 =  -1;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        if (!Information.IsNothing( this.WindowList[windowCounter]))
        {
          this.WindowList[windowCounter].MouseInThisWindow = false;
          if (x >= this.WindowX[windowCounter] & x < this.WindowX[windowCounter] + this.WindowW[windowCounter] && y >= this.WindowY[windowCounter] & y < this.WindowY[windowCounter] + this.WindowH[windowCounter])
          {
            if (num1 == -1)
              num1 = windowCounter;
            if (num1 > -1 && !Information.IsNothing( this.WindowList[windowCounter].LowerWindow))
              num1 = windowCounter;
          }
        }
      }
      let mut windowCounter1: i32 =  this.WindowCounter;
      for (let mut index: i32 =  0; index <= windowCounter1; index += 1)
      {
        this.WindowList[index].MouseInThisWindow = false;
        if (x >= this.WindowX[index] & x < this.WindowX[index] + this.WindowW[index] && y >= this.WindowY[index] & y < this.WindowY[index] + this.WindowH[index])
          this.WindowList[index].MouseInThisWindow = true;
      }
      let mut windowCounter2: i32 =  this.WindowCounter;
      for (let mut index: i32 =  0; index <= windowCounter2; index += 1)
      {
        if (index == num1 & Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 & !this.WindowInputBlock[index] && !(x >= this.WindowX[index] & x <= this.WindowX[index] + this.WindowW[index] & y >= this.WindowY[index] & y <= this.WindowY[index] + this.WindowH[index]) && this.LastOverlayWindow == this.WindowID[index])
        {
          this.ClearOverlaySpecificWindow(this.LastOverlayWindow);
          this.LastOverlayWindow = -1;
          screenReturnClass.flag = true;
          return screenReturnClass;
        }
      }
      bool flag = false;
      let mut windowCounter3: i32 =  this.WindowCounter;
      windowReturnClass: WindowReturnClass;
      for (let mut index: i32 =  0; index <= windowCounter3; index += 1)
      {
        if (index == num1 & Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 & !this.WindowInputBlock[index] && x >= this.WindowX[index] & x <= this.WindowX[index] + this.WindowW[index] & y >= this.WindowY[index] & y <= this.WindowY[index] + this.WindowH[index])
        {
          windowReturnClass = this.WindowList[index].HandleMouseMove(x - this.WindowX[index], y - this.WindowY[index]);
          this.WindowFlag[index] = windowReturnClass.Flag;
          if (windowReturnClass.allowOtherMouseOverWindow)
            flag = true;
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
      num2: i32;
      if (Information.IsNothing( windowReturnClass))
        num2 = 1;
      else if (!windowReturnClass.NoMouseClickBelow)
        num2 = 1;
      if (num2 == 1)
      {
        let mut windowCounter4: i32 =  this.WindowCounter;
        for (let mut index: i32 =  0; index <= windowCounter4; index += 1)
        {
          if ((index == num1 | flag) & Operators.CompareString(this.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 & !this.WindowInputBlock[index] && x >= this.WindowX[index] & x <= this.WindowX[index] + this.WindowW[index] & y >= this.WindowY[index] & y <= this.WindowY[index] + this.WindowH[index])
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

    pub ScreenReturnClass HandleTimerWheel(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass1 = ScreenReturnClass::new();
      if (this.WindowCounter <= -1)
      {
        ScreenReturnClass screenReturnClass2;
        return screenReturnClass2;
      }
      let mut num: i32 =  1;
      do
      {
        let mut windowCounter1: i32 =  this.WindowCounter;
        for (let mut index1: i32 =  0; index1 <= windowCounter1; index1 += 1)
        {
          if (!this.WindowInputBlock[index1])
          {
            bool flag;
            if (x > this.WindowX[index1] & y > this.WindowY[index1] & x < this.WindowX[index1] + this.WindowW[index1] & y < this.WindowY[index1] + this.WindowH[index1])
              flag = true;
            if (Operators.CompareString(this.WindowList[index1].GetType().FullName, "WindowsApplication1.OrderWindowClass2", false) == 0 | Operators.CompareString(this.WindowList[index1].GetType().FullName, "WindowsApplication1.UdsOrderWindowClass", false) == 0)
            {
              if (num == 2)
              {
                let mut windowCounter2: i32 =  this.WindowCounter;
                for (let mut index2: i32 =  0; index2 <= windowCounter2; index2 += 1)
                {
                  if (Operators.CompareString(this.WindowList[index2].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 && x > this.WindowX[index2] & y > this.WindowY[index2] & x < this.WindowX[index2] + this.WindowW[index2] & y < this.WindowY[index2] + this.WindowH[index2])
                    flag = true;
                }
              }
              else
                flag = false;
            }
            else if (num == 2)
              flag = false;
            if (Operators.CompareString(this.WindowList[index1].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0)
              flag = false;
            if (flag)
            {
              wr: WindowReturnClass = this.WindowList[index1].handleTimerWheel(x - this.WindowX[index1], y - this.WindowY[index1]);
              if (wr.Flag)
              {
                screenReturnClass1.flag = true;
                this.WindowFlag[index1] = true;
                if (wr.Counter > -1)
                  this.HandleWR(wr, false,  screenReturnClass1);
                return screenReturnClass1;
              }
              this.Game.EditObj.MouseWheel = 0;
              screenReturnClass1.flag = false;
              return screenReturnClass1;
            }
          }
        }
        num += 1;
      }
      while (num <= 2);
      this.Game.EditObj.MouseWheel = 0;
      screenReturnClass1.flag = false;
      return screenReturnClass1;
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      bool flag = false;
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        if (Operators.CompareString(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0 & !this.WindowInputBlock[windowCounter])
        {
          wr: WindowReturnClass = this.WindowList[windowCounter].handleTimer();
          if (!this.WindowFlag[windowCounter] & wr.Flag)
            this.WindowFlag[windowCounter] = wr.Flag;
          if (wr.Flag)
          {
            if (wr.Counter > -1)
              this.HandleWR(wr, false,  screenReturnClass);
            if (wr.Flag)
              flag = true;
          }
        }
      }
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        if (Operators.CompareString(this.WindowList[windowCounter].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 & !this.WindowInputBlock[windowCounter])
        {
          wr: WindowReturnClass = this.WindowList[windowCounter].handleTimer();
          if (!this.WindowFlag[windowCounter] & wr.Flag)
            this.WindowFlag[windowCounter] = wr.Flag;
          if (wr.Flag)
          {
            if (wr.Counter > -1)
              this.HandleWR(wr, false,  screenReturnClass);
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
      if (this.WindowCounter <= -1)
        return screenReturnClass;
      for (let mut windowCounter: i32 =  this.WindowCounter; windowCounter >= 0; windowCounter += -1)
      {
        if (!this.WindowInputBlock[windowCounter])
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
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }
  }
}
