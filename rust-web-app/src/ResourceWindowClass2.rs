// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ResourceWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Threading;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class ResourceWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     bool AskingAboutMetrics;
     CurrentView: i32;
     tab1: i32;
     tab2: i32;
     tab3: i32;
     tab4: i32;
     tab5: i32;
     tab6: i32;
     tab7: i32;
     tab8: i32;
     tab9: i32;
     tab1a: i32;
     tab2a: i32;
     tab3a: i32;
     tab4a: i32;
     tab5a: i32;
     tab6a: i32;
     tab7a: i32;
     tab8a: i32;
     tab9a: i32;
     tab13: i32;
     tab11: i32;
     tab12: i32;
     tab13name: String;
     tab11name: String;
     tab12name: String;
     butNextTurnId: i32;
     butNextTurnId2: i32;
     butHistoryId: i32;
     butPlayId: i32;
     currentPlayerId: i32;
     cinButId: i32;
     MouseOverWhichTab: i32;
     smallAiProgress: i32;
     prevAiProgress: i32;
     bool startedInHistoryMode;
     bool surrendering;
     special1: i32;
     special2: i32;
     special3: i32;
     advice: i32;
     adviceB: i32;
     screenHis: i32;
     screenVid: i32;
     screenMan: i32;
     screenMap: i32;

    pub ResourceWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect)
      : base( tGame, tGame.ScreenWidth, 75, 8)
    {
      self.NewGfx = true;
      self.w = tGame.ScreenWidth;
      self.h = 75;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.CurrentView = 0;
      self.startedInHistoryMode = false;
      if (self.game.EditObj.OrderType == 26)
        self.startedInHistoryMode = true;
      if (!self.game.EditObj.AIMoving)
      {
        self.game.EditObj.RealRound = self.game.Data.Round;
        self.game.EditObj.RealTurn = self.game.Data.Turn;
      }
      if (!self.game.AIRunning & !self.game.EditObj.AIMoving & self.game.EditObj.OrderType != 26 && !self.game.EditObj.helpAlreadyOpened & self.game.EditObj.RealRound == 1)
      {
        self.game.EditObj.helpAlreadyOpened = true;
        self.CurrentView = 11;
        self.game.EditObj.SetViewMode2 = 11;
        self.game.EditObj.rightSideBarMode = 2;
        self.game.EditObj.leftSideBarMode = 2;
      }
      self.tab11name = self.game.HandyFunctionsObj.GetUDSmanagementTabName(1);
      self.tab12name = self.game.HandyFunctionsObj.GetUDSmanagementTabName(2);
      self.tab13name = self.game.HandyFunctionsObj.GetUDSmanagementTabName(3);
      if (self.game.EditObj.se1_ManagementTab < 1)
        self.game.EditObj.se1_ManagementTab = 54;
      self.ShowTime = DateAndTime.Now;
      self.dostuff();
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.HumanPlayer > -1 && self.game.EditObj.AIMoving &&  DateAndTime.Now.Subtract(self.ShowTime).Ticks > 2000000.0)
      {
        self.ShowTime = DateAndTime.Now;
        self.dostuff();
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (self.game.EditObj.udsManagementTabOverrideId > 0)
      {
        let mut mouseCounter: i32 = self.MouseCounter;
        for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
        {
          if (self.MouseData[index] == self.game.EditObj.SetViewMode2)
          {
            self.game.EditObj.SetViewMode2 = 0;
            windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.MouseRect[index].X + 1, self.MouseRect[index].Y + 1, 1);
            windowReturnClass2.SetFlag(true);
            return windowReturnClass2;
          }
        }
      }
      if (self.game.EditObj.udsViewMode2Override > 0)
      {
        if (self.game.EditObj.SetViewMode2 != self.game.EditObj.udsViewMode2Override)
        {
          let mut mouseCounter: i32 = self.MouseCounter;
          for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
          {
            if (self.MouseData[index] == self.game.EditObj.udsViewMode2Override)
            {
              self.game.EditObj.udsViewMode2Override = -1;
              windowReturnClass3: WindowReturnClass = self.HandleMouseClick(self.MouseRect[index].X + 1, self.MouseRect[index].Y + 1, 1);
              windowReturnClass3.SetFlag(true);
              return windowReturnClass3;
            }
          }
          self.game.EditObj.udsViewMode2Override = -1;
        }
        else
        {
          self.game.EditObj.udsViewMode2Override = -1;
          windowReturnClass1.AddCommand(4, 9);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      if (self.MouseOverWhichTab > 0 && !self.MouseInThisWindow)
      {
        self.MouseOverWhichTab = 0;
        self.dostuff();
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (!self.game.EditObj.askedMetricsPermission & !self.game.EditObj.allowMetrics & !self.AskingAboutMetrics & self.game.Data.Round >= 2)
      {
        ScreenClass screeny = self.game.FormRef.Screeny;
        System.Type type = typeof (PlayExtraWindowClass2);
         System.Type local =  type;
        if (screeny.WindowPresent( local))
        {
          str: String = "Collecting metrics data on your gameplay really helps with fine-tuning the game.\r\nWill you allow this game to share some metrics data with the developers?";
          self.game.EditObj.PopupValue = 10;
          self.game.EditObj.QuestionText = str;
          self.game.EditObj.AnswerCount = 2;
          self.game.EditObj.AnswerText[1] = "Sure!";
          self.game.EditObj.AnswerTextMouseOver[1] = "This means that now and then the game will use your internet connection to send some minimal quantities (1K or less) of game statistics to our server. You will always remain anonymous. Data is used to improve game balance and provide feedback to the community. ";
          self.game.EditObj.AnswerText[2] = "No thanks";
          self.game.EditObj.AnswerTextMouseOver[2] = "This means the game will not attempt to send any metrics data to the developers.";
          self.AskingAboutMetrics = true;
          windowReturnClass1.AddCommand(5, 10);
          self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      return windowReturnClass1;
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = base.HandleMouseMove(x, y);
      if (y < 60 && x > 100 & x < self.game.ScreenWidth - 175)
        windowReturnClass.NoMouseClickBelow = true;
      let mut num: i32 = -1;
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
          num = self.MouseData[mouseCounter];
      }
      if (num > 0)
      {
        if (self.MouseOverWhichTab != num)
        {
          if (self.game.EmpireStyle)
            SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
          self.MouseOverWhichTab = num;
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      else if (self.MouseOverWhichTab > 0)
      {
        self.MouseOverWhichTab = -1;
        self.dostuff();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      return windowReturnClass;
    }

    pub fn DoRefresh()
    {
      if (self.cinButId > 0)
      {
        self.RemoveSubPart(self.cinButId);
        self.cinButId = 0;
      }
      if (self.butHistoryId > 0)
      {
        self.RemoveSubPart(self.butHistoryId);
        self.butHistoryId = 0;
      }
      if (self.butPlayId > 0)
      {
        self.RemoveSubPart(self.butPlayId);
        self.butPlayId = 0;
      }
      if (self.butNextTurnId > 0)
      {
        self.RemoveSubPart(self.butNextTurnId);
        self.butNextTurnId = 0;
      }
      if (self.butNextTurnId2 > 0)
      {
        self.RemoveSubPart(self.butNextTurnId2);
        self.butNextTurnId2 = 0;
      }
      if (self.currentPlayerId > 0)
      {
        self.RemoveSubPart(self.currentPlayerId);
        self.currentPlayerId = 0;
      }
      self.dostuff();
    }

    pub fn dostuff()
    {
      self.game.Data.DontShowAIMove = self.game.EditObj.dontShowAImoves;
      if (self.butNextTurnId > 0)
      {
        self.RemoveSubPart(self.butNextTurnId);
        self.butNextTurnId = 0;
      }
      if (self.butNextTurnId2 > 0)
      {
        self.RemoveSubPart(self.butNextTurnId2);
        self.butNextTurnId2 = 0;
      }
      if (self.special1 > 0)
      {
        self.RemoveSubPart(self.special1);
        self.special1 = 0;
      }
      if (self.special2 > 0)
      {
        self.RemoveSubPart(self.special2);
        self.special2 = 0;
      }
      if (self.special3 > 0)
      {
        self.RemoveSubPart(self.special3);
        self.special3 = 0;
      }
      if (self.advice > 0)
      {
        self.RemoveSubPart(self.advice);
        self.advice = 0;
      }
      if (self.adviceB > 0)
      {
        self.RemoveSubPart(self.adviceB);
        self.adviceB = 0;
      }
      if (!(!self.game.AIRunning & !self.game.EditObj.AIMoving & self.game.EditObj.OrderType != 26) && self.game.EditObj.SetViewMode2 != 7)
        self.game.EditObj.SetViewMode2 = 0;
      self.CurrentView = self.game.EditObj.SetViewMode2;
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (!self.game.EditObj.se1_ManagementMode)
        self.DrawOpenTab( graphics);
      let mut tx1: i32 = 312;
      bool flag1 = false;
      width: i32;
      if ( self.game.Data.RuleVar[971] > 0.0)
      {
        let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[971]));
        if (stringListById > -1 && self.game.Data.StringListObj[stringListById].Length > -1)
          flag1 = true;
        width = 100;
        if (!flag1)
          ;
      }
      bool flag2 = false;
      let mut num1: i32 = 68;
      if (self.game.ScreenWidth <= 1320)
      {
        tx1 -= 10;
        num1 = 58;
      }
      self.screenHis = -1;
      self.screenMan = -1;
      self.screenVid = -1;
      self.screenMap = -1;
      num2: i32;
      if (self.game.se1Running | self.game.se1ThreadRunning | self.game.se1Running & !self.game.se1ThreadRunning & self.game.EditObj.AIMoving)
      {
        flag2 = true;
        Rectangle trect1 = self.DrawOneTab(graphics, false, tx1, "MAP", -1, grayedOut: true) with
        {
          Y = 36
        };
        trect1.Height -= 36;
        self.AddMouse( trect1, "", "Inaccesible during the AIs turn.");
        let mut tx2: i32 = tx1 + num1;
        Rectangle trect2 = self.DrawOneTab(graphics, true, tx2, "HIS", -1) with
        {
          Y = 36
        };
        trect2.Height -= 36;
        self.AddMouse( trect2, "", "You are currently in HISTORY mode. While the AI is playing.");
        let mut tx3: i32 = tx2 + num1;
        trect2 = self.DrawOneTab(graphics, false, tx3, "VID", -1, grayedOut: true) with
        {
          Y = 36
        };
        trect2.Height -= 36;
        self.AddMouse( trect2, "", "Inaccesible during the AIs turn.");
        let mut tx4: i32 = tx3 + num1;
        trect2 = self.DrawOneTab(graphics, false, tx4, "MNG", -1, grayedOut: true) with
        {
          Y = 36
        };
        trect2.Height -= 36;
        self.AddMouse( trect2, "", "Inaccesible during the AIs turn.");
      }
      else if ( self.game.Data.RuleVar[408] > 1.0)
      {
        if (self.game.EditObj.OrderType == 26 | self.startedInHistoryMode)
        {
          Rectangle trect = self.DrawOneTab(graphics, false, tx1, "MAP", -1, MousingOverNow: (self.MouseOverWhichTab == 2001)) with
          {
            Y = 36
          };
          trect.Height -= 36;
          self.AddMouse( trect, "", "Click to got to MAP mode [Shortkey Escape]", 2001);
          self.screenMap = self.MouseCounter;
          let mut tx5: i32 = tx1 + num1;
          trect = self.DrawOneTab(graphics, true, tx5, "HIS", -1, MousingOverNow: (self.MouseOverWhichTab == 2002)) with
          {
            Y = 36
          };
          trect.Height -= 36;
          self.AddMouse( trect, "", "You are currently in HISTORY mode", 2002);
          let mut tx6: i32 = tx5 + num1;
          tx7: i32;
          if (flag1)
          {
            trect = self.DrawOneTab(graphics, false, tx6, "VID", -1, MousingOverNow: (self.MouseOverWhichTab == 2003)) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "Click to go to VIDCOM mode [Shortkey V]", 2003);
            self.screenVid = self.MouseCounter;
            tx7 = tx6 + num1;
          }
          else
          {
            trect = self.DrawOneTab(graphics, false, tx6, "VID", -1) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "There are no VIDCOM messages or reports available");
            tx7 = tx6 + num1;
          }
          trect = self.DrawOneTab(graphics, false, tx7, "MNG", -1, MousingOverNow: (self.MouseOverWhichTab == 2004)) with
          {
            Y = 36
          };
          trect.Height -= 36;
          self.AddMouse( trect, "", "Go to MANAGEMENT SCREEN mode  [Shortkey N]", 2004);
          self.screenMan = self.MouseCounter;
          num2 = tx7 + num1;
        }
        else
        {
          Rectangle trect;
          tx8: i32;
          if (self.game.EditObj.se1_ManagementMode)
          {
            trect = self.DrawOneTab(graphics, false, tx1, "MAP", -1, MousingOverNow: (self.MouseOverWhichTab == 2001)) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "Click to got to MAP mode [Shortkey Escape]", 2001);
            self.screenMap = self.MouseCounter;
            tx8 = tx1 + num1;
          }
          else
          {
            trect = self.DrawOneTab(graphics, true, tx1, "MAP", -1) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "You are currently in MAP mode");
            tx8 = tx1 + num1;
          }
          trect = self.DrawOneTab(graphics, false, tx8, "HIS", -1, MousingOverNow: (self.MouseOverWhichTab == 2002)) with
          {
            Y = 36
          };
          trect.Height -= 36;
          self.AddMouse( trect, "", "Click to go to HISTORY mode [Shortkey H]", 2002);
          self.screenHis = self.MouseCounter;
          let mut tx9: i32 = tx8 + num1;
          tx10: i32;
          if (flag1)
          {
            trect = self.DrawOneTab(graphics, false, tx9, "VID", -1, MousingOverNow: (self.MouseOverWhichTab == 2003)) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "Click to go to VIDCOM mode [Shortkey V]", 2003);
            self.screenVid = self.MouseCounter;
            tx10 = tx9 + num1;
          }
          else
          {
            trect = self.DrawOneTab(graphics, false, tx9, "VID", -1) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "There are no VIDCOM messages or reports available");
            tx10 = tx9 + num1;
          }
          if (self.game.EditObj.se1_ManagementMode)
          {
            trect = self.DrawOneTab(graphics, true, tx10, "MNG", -1) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "You are currently in MANAGEMENT SCREEN mode");
            num2 = tx10 + num1;
          }
          else
          {
            trect = self.DrawOneTab(graphics, false, tx10, "MNG", -1, MousingOverNow: (self.MouseOverWhichTab == 2004)) with
            {
              Y = 36
            };
            trect.Height -= 36;
            self.AddMouse( trect, "", "Go to MANAGEMENT SCREEN mode [Shortkey N]", 2004);
            self.screenMan = self.MouseCounter;
            num2 = tx10 + num1;
          }
        }
      }
       let mut local1: &Graphics = &graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
       let mut local2: &Bitmap = &bitmap;
      Rectangle trect3 = Rectangle::new(0, 140, 300, 63);
      let mut srcrect1: &Rectangle = &trect3
      Rectangle rectangle = Rectangle::new(0, 0, 300, 63);
      let mut destrect1: &Rectangle = &rectangle
      DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
      if (self.game.ScreenWidth > 2600)
      {
        width =  Math.Round( self.game.ScreenWidth / 2.0);
         let mut local3: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
         let mut local4: &Bitmap = &bitmap;
        rectangle = Rectangle::new(300, 140, width, 32);
        let mut srcrect2: &Rectangle = &rectangle
        trect3 = Rectangle::new(300, 0, width, 32);
        let mut destrect2: &Rectangle = &trect3
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
         let mut local6: &Bitmap = &bitmap;
        rectangle = Rectangle::new(0, 140, width, 32);
        let mut srcrect3: &Rectangle = &rectangle
        trect3 = Rectangle::new(300 + width, 0, width, 32);
        let mut destrect3: &Rectangle = &trect3
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
        if (!flag2)
        {
           let mut local7: &Graphics = &graphics;
          bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
           let mut local8: &Bitmap = &bitmap;
          rectangle = Rectangle::new(self.w - width - 150, 140, 150, 75);
          let mut srcrect4: &Rectangle = &rectangle
          trect3 = Rectangle::new(self.w - 150, 0, 150, 75);
          let mut destrect4: &Rectangle = &trect3
          DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
        }
        if (flag2)
        {
           let mut local9: &Graphics = &graphics;
          bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
           let mut local10: &Bitmap = &bitmap;
          rectangle = Rectangle::new(self.w - width - 150, 140, 150, 32);
          let mut srcrect5: &Rectangle = &rectangle
          trect3 = Rectangle::new(self.w - 150, 0, 150, 32);
          let mut destrect5: &Rectangle = &trect3
          DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
        }
      }
      else
      {
         let mut local11: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
         let mut local12: &Bitmap = &bitmap;
        rectangle = Rectangle::new(300, 140, self.w - 440, 32);
        let mut srcrect6: &Rectangle = &rectangle
        trect3 = Rectangle::new(300, 0, self.w - 440, 32);
        let mut destrect6: &Rectangle = &trect3
        DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
        if (!flag2)
        {
           let mut local13: &Graphics = &graphics;
          bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
           let mut local14: &Bitmap = &bitmap;
          rectangle = Rectangle::new(self.w - 150, 140, 150, 75);
          let mut srcrect7: &Rectangle = &rectangle
          trect3 = Rectangle::new(self.w - 150, 0, 150, 75);
          let mut destrect7: &Rectangle = &trect3
          DrawMod.DrawSimplePart2( local13,  local14, srcrect7, destrect7);
        }
        if (flag2)
        {
           let mut local15: &Graphics = &graphics;
          bitmap = BitmapStore.GetBitmap(self.game.MARCBOTBAR);
           let mut local16: &Bitmap = &bitmap;
          rectangle = Rectangle::new(self.w - width - 150, 140, 150, 32);
          let mut srcrect8: &Rectangle = &rectangle
          trect3 = Rectangle::new(self.w - 150, 0, 150, 32);
          let mut destrect8: &Rectangle = &trect3
          DrawMod.DrawSimplePart2( local15,  local16, srcrect8, destrect8);
        }
      }
      let mut x1: i32 = 290;
      if (flag2)
      {
        for (; x1 < self.game.ScreenWidth; x1 += 50)
        {
           let mut local17: &Graphics = &graphics;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_BOTTOM);
           let mut local18: &Bitmap = &bitmap;
          rectangle = Rectangle::new(15, 22, 50, 20);
          let mut srcrect9: &Rectangle = &rectangle
          trect3 = Rectangle::new(x1, 22, 50, 20);
          let mut destrect9: &Rectangle = &trect3
          DrawMod.DrawSimplePart2( local17,  local18, srcrect9, destrect9);
        }
      }
      else
      {
        for (; x1 < self.game.ScreenWidth - 120; x1 += 50)
        {
           let mut local19: &Graphics = &graphics;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_BOTTOM);
           let mut local20: &Bitmap = &bitmap;
          rectangle = Rectangle::new(15, 22, 50, 20);
          let mut srcrect10: &Rectangle = &rectangle
          trect3 = Rectangle::new(x1, 22, 50, 20);
          let mut destrect10: &Rectangle = &trect3
          DrawMod.DrawSimplePart2( local19,  local20, srcrect10, destrect10);
        }
         let mut local21: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_BOTTOM);
         let mut local22: &Bitmap = &bitmap;
        rectangle = Rectangle::new(0, 22, 15, 20);
        let mut srcrect11: &Rectangle = &rectangle
        trect3 = Rectangle::new(300, 22, 15, 20);
        let mut destrect11: &Rectangle = &trect3
        DrawMod.DrawSimplePart2( local21,  local22, srcrect11, destrect11);
         let mut local23: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_BOTTOM);
         let mut local24: &Bitmap = &bitmap;
        rectangle = Rectangle::new(65, 22, 15, 20);
        let mut srcrect12: &Rectangle = &rectangle
        trect3 = Rectangle::new(self.w - 187 - 15, 22, 15, 20);
        let mut destrect12: &Rectangle = &trect3
        DrawMod.DrawSimplePart2( local23,  local24, srcrect12, destrect12);
      }
      if (self.game.EditObj.se1_ManagementMode)
        self.DrawTabs_ManagementScreen( graphics);
      else
        self.DrawTabs( graphics);
       let mut local25: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_LEFT);
       let mut local26: &Bitmap = &bitmap;
      DrawMod.DrawSimple( local25,  local26, 0, 0);
      bool flag3 = true;
      if ( Math.Round(Conversion.Val(self.game.Data.Designer)) >= 43)
      {
        let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 79, 0, 0));
        let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 191, 0, 0));
        if (self.game.Data.StringListObj[stringListById2].Width >= 27)
        {
          for (let mut length: i32 = self.game.Data.StringListObj[stringListById1].Length; length >= 0; length += -1)
          {
            let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[length, 11]));
            width =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 27)));
            if (width > 0)
              flag3 = false;
          }
        }
      }
      if (!flag2)
      {
         let mut local27: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_RIGHT);
         let mut local28: &Bitmap = &bitmap;
        let mut x2: i32 = self.w - 190;
        DrawMod.DrawSimple( local27,  local28, x2, 0);
        if (flag3)
        {
          if (self.butNextTurnId == 0)
          {
            let mut tsubpart: SubPartClass =  new SEButtonPartClass(self.game.SE1_ARROW1, "End your turn. Let the other players make their moves.", 54, 39);
            self.butNextTurnId = self.AddSubPart( tsubpart, self.w - 190 + 126, 9, 54, 39, 1);
          }
        }
        else if (self.butNextTurnId2 == 0)
        {
          let mut tsubpart: SubPartClass =  new SEButtonPartClass(self.game.CANCELBALL, "You cannot end your move as there is still a Decision left that is OBLIGATORY to make.", 54, 39);
          self.butNextTurnId2 = self.AddSubPart( tsubpart, self.w - 190 + 126, 9, 54, 39, 1);
        }
      }
      if (!flag2)
      {
        self.DrawDate( graphics);
        self.DrawPP( graphics);
      }
      let mut index: i32 = self.game.EditObj.RealTurn;
      bool flag4 = false;
      if (self.game.EditObj.RealTurn == -1)
        width = width;
      if (self.game.EditObj.RealTurn > self.game.Data.RegimeCounter)
        ;
      if (flag2 && self.game.EditObj.RealTurn > -1 & self.game.EditObj.RealTurn != self.game.Data.Turn & self.game.Data.Turn > -1)
      {
        let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 275, 0, 0));
        try
        {
          if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData3(0, self.game.Data.RegimeObj[self.game.EditObj.RealTurn].id, 1, self.game.Data.RegimeObj[self.game.Data.Turn].id, 2, "recon", 3))) > 0)
          {
            index = self.game.Data.Turn;
          }
          else
          {
            index = self.game.EditObj.RealTurn;
            if (index != self.game.Data.Turn)
              flag4 = true;
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          index = self.game.EditObj.RealTurn;
          flag4 = true;
          ProjectData.ClearProjectError();
        }
      }
      str: String = self.game.Data.RegimeObj[index].Name;
      if (flag4)
        str = "Unknown Regime";
      graphics.MeasureString(str, DrawMod.TGame.MarcFont16);
      if (flag2)
      {
        self.smallAiProgress += 5;
        if (self.game.EditObj.AIProgressNow != self.prevAiProgress)
          self.smallAiProgress = 0;
        self.prevAiProgress = self.game.EditObj.AIProgressNow;
        let mut num3: i32 =  Math.Round( self.game.EditObj.AIProgressNow /  self.game.EditObj.AIProgressMax * 100.0);
        if (num3 > 100)
          num3 = 100;
        let mut num4: i32 =  Math.Round( (84 * num3) / 100.0);
        let mut num5: i32 = self.smallAiProgress;
        if (num5 > 50)
          num5 =  Math.Round(Math.Sqrt( (num5 - 50))) + 50;
        if (num5 > 70)
          num5 =  Math.Round(Math.Sqrt( (num5 - 70))) + 70;
        if (num5 > 90)
          num5 =  Math.Round(Math.Sqrt( (num5 - 70))) + 90;
        if (num5 > 100)
          num5 = 100;
        let mut w: i32 = num4 +  Math.Round( (83 * num5) / 100.0);
        DrawMod.DrawBlock( graphics, 111, 18, w, 16, 200, 0, 0, 150);
      }
      DrawMod.DrawTextColouredConsoleCenter( graphics, str, self.game.MarcFont16, 193, 15, self.game.seColWhite);
      if (self.currentPlayerId < 1)
      {
        if (flag2)
        {
          rectangle = Rectangle::new(108, 15, 173, 21);
          trect3 = rectangle;
          self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[index].Name, "This AI is currently making its moves.");
        }
        else if (self.game.EditObj.se1_ManagementMode)
        {
          rectangle = Rectangle::new(108, 15, 173, 21);
          trect3 = rectangle;
          self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[index].Name, "Yes, this is you.");
        }
        else
        {
          rectangle = Rectangle::new(108, 15, 173, 21);
          trect3 = rectangle;
          self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[index].Name, "Click to go to capitol of this regime.", 501);
        }
      }
      let mut num6: i32 = self.game.Data.RegimeObj[index].Red;
      let mut num7: i32 = self.game.Data.RegimeObj[index].Green;
      let mut num8: i32 = self.game.Data.RegimeObj[index].Blue;
      let mut num9: i32 = self.game.Data.RegimeObj[index].Red2;
      let mut num10: i32 = self.game.Data.RegimeObj[index].Green2;
      let mut num11: i32 = self.game.Data.RegimeObj[index].Blue2;
      if (self.MouseOverWhichTab == 501)
      {
        num6 =  Math.Round( num6 * 1.1);
        num7 =  Math.Round( num7 * 1.1);
        num8 =  Math.Round( num8 * 1.1);
        num9 =  Math.Round( num9 * 1.1);
        num10 =  Math.Round( num10 * 1.1);
        num11 =  Math.Round( num11 * 1.1);
        if (num6 >  byte.MaxValue)
          num6 =  byte.MaxValue;
        if (num7 >  byte.MaxValue)
          num7 =  byte.MaxValue;
        if (num8 >  byte.MaxValue)
          num8 =  byte.MaxValue;
        if (num9 >  byte.MaxValue)
          num9 =  byte.MaxValue;
        if (num10 >  byte.MaxValue)
          num10 =  byte.MaxValue;
        if (num11 >  byte.MaxValue)
          num11 =  byte.MaxValue;
      }
      if (flag4)
      {
        num6 =  Math.Round( num6 / 2.0);
        num7 =  Math.Round( num7 / 2.0);
        num8 =  Math.Round( num8 / 2.0);
        num9 =  Math.Round( num9 / 2.0);
        num10 =  Math.Round( num10 / 2.0);
        num11 =  Math.Round( num11 / 2.0);
      }
      let mut bannerSpriteNr: i32 = self.game.Data.RegimeObj[index].BannerSpriteNr;
       let mut local29: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(bannerSpriteNr);
       let mut local30: &Bitmap = &bitmap;
      double r1 =  ( num6 /  byte.MaxValue);
      double g1 =  ( num7 /  byte.MaxValue);
      double b1 =  ( num8 /  byte.MaxValue);
      DrawMod.DrawScaledColorized2( local29,  local30, 13, 15, 80, 60, 124, 210,  r1,  g1,  b1, 1f);
      let mut bannerSpriteNr2: i32 = self.game.Data.RegimeObj[index].BannerSpriteNr2;
      if (bannerSpriteNr2 > 0)
      {
         let mut local31: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(bannerSpriteNr2);
         let mut local32: &Bitmap = &bitmap;
        double r2 =  ( num9 /  byte.MaxValue);
        double g2 =  ( num10 /  byte.MaxValue);
        double b2 =  ( num11 /  byte.MaxValue);
        DrawMod.DrawScaledColorized2( local31,  local32, 13, 15, 80, 60, 124, 210,  r2,  g2,  b2, 1f);
      }
      let mut hqSpriteNr2: i32 = self.game.Data.RegimeObj[index].HQSpriteNr2;
      if (hqSpriteNr2 > 0)
      {
         let mut local33: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
         let mut local34: &Bitmap = &bitmap;
        double r3 =  ( self.game.Data.RegimeObj[index].Red3 /  byte.MaxValue) - 1.0;
        double g3 =  ( self.game.Data.RegimeObj[index].Green3 /  byte.MaxValue) - 1.0;
        double b3 =  ( self.game.Data.RegimeObj[index].Blue3 /  byte.MaxValue) - 1.0;
        DrawMod.Draw( local33,  local34, 30, 27,  r3,  g3,  b3, 0.95f);
      }
      if (self.currentPlayerId < 1)
      {
        if (flag2)
        {
          if (flag4)
          {
            rectangle = Rectangle::new(0, 0, 100, 75);
            trect3 = rectangle;
            self.AddMouse( trect3, "Current AI player is unknown. ", "This AI is currently making its moves.");
          }
          else if (self.game.EditObj.se1_ManagementMode)
          {
            rectangle = Rectangle::new(0, 0, 100, 75);
            trect3 = rectangle;
            self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[index].Name, "Yes, this is you.");
          }
          else
          {
            rectangle = Rectangle::new(0, 0, 100, 75);
            trect3 = rectangle;
            self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[index].Name, "This AI is currently making its moves.");
          }
        }
        else if (self.game.EditObj.se1_ManagementMode)
        {
          rectangle = Rectangle::new(0, 0, 100, 75);
          trect3 = rectangle;
          self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[index].Name, "Yes, this is you.");
        }
        else
        {
          rectangle = Rectangle::new(0, 0, 100, 75);
          trect3 = rectangle;
          self.AddMouse( trect3, "Current player is " + self.game.Data.RegimeObj[self.game.EditObj.RealTurn].Name, "Click to go to capitol of this regime.", 501);
        }
      }
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
      graphics = (Graphics) null;
    }

    pub Rectangle DrawOneTab(
      Graphics g,
      bool active,
      tx: i32,
      s: String,
      iconSlot: i32,
      let mut smallNumber: i32 = -1,
      bool grayedOut = false,
      bool MousingOverNow = false)
    {
      let mut y1: i32 = 0;
      if (!active)
        y1 = -12;
      bitmap: Bitmap;
      if (MousingOverNow)
      {
         let mut local1: &Graphics = &g;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_TAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = tx;
        let mut y2: i32 = y1;
        DrawMod.Draw( local1,  local2, x, y2, 0.05f, 0.05f, 0.05f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_TAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 = tx;
        let mut y3: i32 = y1;
        DrawMod.DrawSimple( local3,  local4, x, y3);
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (iconSlot > -1)
      {
        if (grayedOut)
        {
           let mut local5: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
           let mut local6: &Bitmap = &bitmap;
          rectangle1 = Rectangle::new(iconSlot * 42, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2Coloured( local5,  local6, srcrect, destrect, 0.5f, 0.5f, 0.5f, 1f);
        }
        else if (MousingOverNow & !active)
        {
           let mut local7: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
           let mut local8: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
        }
        else
        {
          if (!active)
          {
             let mut local9: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local10: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 0, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
          }
          if (active)
          {
             let mut local11: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local12: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
          }
        }
      }
      c: Color;
      if (smallNumber > 0)
      {
        if (!active)
        {
           let mut local13: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONHIGHLIGHT);
           let mut local14: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local13,  local14, srcrect, destrect);
          c = Color.FromArgb( byte.MaxValue, 225, 215, 215);
        }
        if (active)
        {
           let mut local15: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONHIGHLIGHT);
           let mut local16: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
          c = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 245, 245);
        }
        str: String = smallNumber.ToString();
        if (smallNumber > 9)
          str = "X";
        SizeF sizeF = g.MeasureString(str, DrawMod.TGame.MarcFont5);
        DrawMod.DrawTextColouredConsole( g, str, self.game.MarcFont5, tx +  Math.Round((68.0 -  sizeF.Width) / 2.0) + 11, y1 + 22, c);
      }
      SizeF sizeF1 = g.MeasureString(s, DrawMod.TGame.MarcFont16);
      if (active)
        c = self.game.seColWhite;
      if (!active)
        c = self.game.seColGray;
      if (grayedOut)
        c = Color.FromArgb( byte.MaxValue, 128, 128, 128);
      DrawMod.DrawTextColouredConsole( g, s, self.game.MarcFont16, tx +  Math.Round((68.0 -  sizeF1.Width) / 2.0), y1 + 48, c);
      Rectangle rectangle3 = Rectangle::new(tx, y1, 68, 75);
      tx += 68;
      return rectangle3;
    }

    pub fn DrawOpenTab(object g)
    {
      if (self.CurrentView <= 0)
        return;
      Rectangle rectForTab = DrawMod.GetRectForTab(self.CurrentView);
      Graphics graphics = (Graphics) g;
       let mut local1: &Graphics = &graphics;
      bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.MARCTAB);
       let mut local2: &Bitmap = &bitmap1;
      Rectangle rectangle1 = Rectangle::new(0, 20, 8, 43);
      let mut srcrect1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(rectForTab.X, 32, 8, 43);
      let mut destrect1: &Rectangle = &rectangle2
      DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
      g =  graphics;
      graphics = (Graphics) g;
       let mut local3: &Graphics = &graphics;
      bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.MARCTAB);
       let mut local4: &Bitmap = &bitmap2;
      rectangle2 = Rectangle::new(170, 20, 16, 43);
      let mut srcrect2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(rectForTab.X + rectForTab.Width - 16, 32, 16, 43);
      let mut destrect2: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
      g =  graphics;
      let mut x: i32 = rectForTab.X + 8;
      let mut width: i32 = 160;
      for (; x < rectForTab.X + rectForTab.Width; x += 160)
      {
        if (x + width > rectForTab.X + rectForTab.Width - 16)
          width = rectForTab.X + rectForTab.Width - 16 - x;
        graphics = (Graphics) g;
         let mut local5: &Graphics = &graphics;
        bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.MARCTAB);
         let mut local6: &Bitmap = &bitmap3;
        rectangle2 = Rectangle::new(10, 20, width, 43);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, 32, width, 43);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
        g =  graphics;
      }
    }

    pub fn DrawTabs(object g)
    {
      self.tab1 = -1;
      self.tab2 = -1;
      self.tab3 = -1;
      self.tab4 = -1;
      self.tab5 = -1;
      self.tab6 = -1;
      self.tab7 = -1;
      self.tab8 = -1;
      self.tab9 = -1;
      self.tab13 = -1;
      self.tab11 = -1;
      self.tab12 = -1;
      Rectangle trect1;
      if (!self.game.AIRunning & !self.game.EditObj.AIMoving & self.game.EditObj.OrderType != 26)
      {
        let mut tx1: i32 = self.game.ScreenWidth - 734;
        Rectangle trect2 = self.DrawOneTab((Graphics) g, self.CurrentView == 8, tx1, "PREFS", 0, MousingOverNow: (self.MouseOverWhichTab == 8));
        self.AddMouse( trect2, "PREFERENCES", "Sound and other customizable settings. [F1]", 8);
        self.tab1 = self.MouseCounter;
        let mut tx2: i32 = tx1 + 68;
        Rectangle trect3 = self.DrawOneTab((Graphics) g, self.CurrentView == 2, tx2, "STATS", 2, MousingOverNow: (self.MouseOverWhichTab == 2));
        self.AddMouse( trect3, "STATISTICS", "Review kills, losses and Regime information [F3]", 2);
        self.tab3 = self.MouseCounter;
        let mut tx3: i32 = tx2 + 68;
        Rectangle trect4 = self.DrawOneTab((Graphics) g, self.CurrentView == 3, tx3, "OOB", 1, MousingOverNow: (self.MouseOverWhichTab == 3));
        self.AddMouse( trect4, "ORDER OF BATTLE", "Review all your units. [F4]", 3);
        self.tab4 = self.MouseCounter;
        let mut tx4: i32 = tx3 + 68;
        Rectangle trect5 = self.DrawOneTab((Graphics) g, self.CurrentView == 6, tx4, "S.MAP", 3, MousingOverNow: (self.MouseOverWhichTab == 6));
        self.AddMouse( trect5, "STRATEGIC MAP", "View the strategic situation and send messages. [F7]", 6);
        self.tab7 = self.MouseCounter;
        let mut tx5: i32 = tx4 + 68;
        tx6: i32;
        if (self.game.Data.RegimeObj[self.game.EditObj.RealTurn].ActionCardCounter == -1)
        {
          self.DrawOneTab((Graphics) g, false, tx5, "STRAT.", 4);
          self.tab6 = -1;
          tx6 = tx5 + 68;
        }
        else
        {
          Rectangle trect6 = self.DrawOneTab((Graphics) g, self.CurrentView == 5, tx5, "STRAT.", 4, MousingOverNow: (self.MouseOverWhichTab == 5));
          self.AddMouse( trect6, "STRATAGEMS", "View your stratagems and play them. [F6]", 5);
          self.tab6 = self.MouseCounter;
          tx6 = tx5 + 68;
        }
        if (self.tab11name.Length > 1)
        {
          let mut smanagementTabPageCount: i32 = self.game.HandyFunctionsObj.GetUDSmanagementTabPageCount(1);
          if (smanagementTabPageCount > 0)
          {
            trect1 = self.DrawOneTab((Graphics) g, self.CurrentView == 11, tx6, self.tab11name, 5, smanagementTabPageCount, MousingOverNow: (self.MouseOverWhichTab == 11));
            self.AddMouse( trect1, self.tab11name, "Make decisions awaiting you", 11);
            self.tab11 = self.MouseCounter;
            tx6 += 68;
          }
          else
          {
            trect1 = self.DrawOneTab((Graphics) g, self.CurrentView == 11, tx6, self.tab11name, 5, grayedOut: true);
            self.AddMouse( trect1, self.tab11name, "No decisions to be taken", -2);
            self.tab11 = self.MouseCounter;
            tx6 += 68;
          }
        }
        if (self.tab12name.Length > 1 && self.game.HandyFunctionsObj.GetUDSmanagementTabPageCount(2) > 0)
        {
          trect1 = self.DrawOneTab((Graphics) g, self.CurrentView == 12, tx6, self.tab12name, 6, MousingOverNow: (self.MouseOverWhichTab == 12));
          self.AddMouse( trect1, self.tab12name, "Reports and letters received this turn", 12);
          self.tab12 = self.MouseCounter;
          let mut num: i32 = tx6 + 68;
        }
      }
      let mut tx: i32 = self.game.ScreenWidth - 258;
      trect1 = self.DrawOneTab((Graphics) g, self.CurrentView == 7, tx, "MINI", 7, MousingOverNow: (self.MouseOverWhichTab == 7));
      self.AddMouse( trect1, "MINIMAP", "View the mini-map. [F8]", 7);
      self.tab8 = self.MouseCounter;
    }

    pub fn DrawTabs_ManagementScreen(object g)
    {
      self.tab1a = -1;
      self.tab2a = -1;
      self.tab3a = -1;
      self.tab4a = -1;
      self.tab5a = -1;
      self.tab6a = -1;
      self.tab7a = -1;
      self.tab8a = -1;
      self.tab9a = -1;
      let mut tx1: i32 = 650;
      Rectangle trect1 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab == 54, tx1, "ASSET", 46, MousingOverNow: (self.MouseOverWhichTab == 54));
      self.AddMouse( trect1, "ASSET MANAGEMENT WINDOW", "Inspect and give orders to all your Assets", 54);
      self.tab4a = self.MouseCounter;
      let mut tx2: i32 = tx1 + 68;
      Rectangle trect2 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab == 55, tx2, "MODEL", 47, MousingOverNow: (self.MouseOverWhichTab == 55));
      self.AddMouse( trect2, "MODEL & QUALITY LEVEL MANAGEMENT WINDOW", "Inspect and set Quality Levels for all your Models and Units", 55);
      self.tab5a = self.MouseCounter;
      let mut tx3: i32 = tx2 + 68;
      Rectangle trect3 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab == 56, tx3, "LEADER", 43, MousingOverNow: (self.MouseOverWhichTab == 56));
      self.AddMouse( trect3, "LEADER MANAGEMENT WINDOW", "Inspect and review all your Leaders", 56);
      self.tab6a = self.MouseCounter;
      let mut tx4: i32 = tx3 + 68;
      Rectangle trect4 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab <= 51, tx4, "PROF", 12, MousingOverNow: (self.MouseOverWhichTab <= 51));
      self.AddMouse( trect4, "PROFILE INFO", "Inspect your Regime Feats progress", 51);
      self.tab1a = self.MouseCounter;
      let mut tx5: i32 = tx4 + 68;
      Rectangle trect5 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab == 52, tx5, "TECH", 1, MousingOverNow: (self.MouseOverWhichTab == 52));
      self.AddMouse( trect5, "TECH TREE INFO", "Inspect your Tech tree progress", 52);
      self.tab2a = self.MouseCounter;
      let mut tx6: i32 = tx5 + 68;
      Rectangle trect6 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab == 53, tx6, "TYPE", 41, MousingOverNow: (self.MouseOverWhichTab == 53));
      self.AddMouse( trect6, "MODEL TYPE INFO", "Inspect your Model Type tree progress", 53);
      self.tab3a = self.MouseCounter;
      let mut tx7: i32 = tx6 + 68;
      if (!self.game.UserDebugger || !(self.game.HandyFunctionsObj.GetHumanPlayers() <= 1 |  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0))].GetData(0, 17, 2))) > 0))
        return;
      Rectangle trect7 = self.DrawOneTab((Graphics) g, self.game.EditObj.se1_ManagementTab == 57, tx7, "DEBUG", 6, MousingOverNow: (self.MouseOverWhichTab == 57));
      self.AddMouse( trect7, "MODEL TYPE INFO", "Inspect your Model Type tree progress", 57);
      self.tab7a = self.MouseCounter;
      let mut num: i32 = tx7 + 68;
    }

    pub fn DrawHexStats( Graphics g)
    {
      let mut x1: i32 =  Math.Round( self.game.ScreenWidth / 2.0 + 158.0);
      if (!(self.game.EditObj.RealRound > 0 & self.game.SelectX > -1))
        return;
      SizeF sizeF1 = SizeF::new();
      str1: String = "REC";
      SizeF sizeF2 = g.MeasureString(str1, self.game.MarcFont5);
      let mut x2: i32 =  Math.Round( ( x1 +  (15.0 -  sizeF2.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont5, x2, 2, Color.White);
      str2: String = Strings.Trim(Conversion.Str( self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].MaxRecon));
      color: Color =  self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].MaxRecon >=  self.game.Data.RuleVar[55] ? ( self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].MaxRecon >=  self.game.Data.RuleVar[56] ? Color.FromArgb( byte.MaxValue, 0,  byte.MaxValue, 0) : Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 0)) : Color.FromArgb( byte.MaxValue,  byte.MaxValue, 0, 0);
      SizeF sizeF3 = g.MeasureString(str2, self.game.MarcFont5);
      let mut x3: i32 =  Math.Round( ( x1 +  (15.0 -  sizeF3.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont5, x3, 15, Color.White);
      Rectangle trect1 = Rectangle::new(x1, 2, 30, 28);
      self.AddMouse( trect1, "RECON POINTS", "How much recon points you have on hex.\r\n" + Strings.Trim(Conversion.Str( self.game.Data.RuleVar[55])) + " points needed to spot a unit.\r\n" + Strings.Trim(Conversion.Str( self.game.Data.RuleVar[56])) + " points needed for full info on unit.");
      let mut x4: i32 = x1 + 30;
      DrawMod.DrawBlockGradient2( g, x4 - 2, 0, 3, 32, Color.FromArgb(200, 80, 90, 110), Color.FromArgb(50, 80, 90, 110));
      str3: String = "ZOC";
      SizeF sizeF4 = g.MeasureString(str3, self.game.MarcFont5);
      let mut x5: i32 =  Math.Round( ( x4 +  (15.0 -  sizeF4.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str3, self.game.MarcFont5, x5, 2, Color.White);
      str4: String = Strings.Trim(Conversion.Str( self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].get_ZocPts(self.game.EditObj.RealTurn)));
      SizeF sizeF5 = g.MeasureString(str4, self.game.MarcFont5);
      let mut x6: i32 =  Math.Round( ( x4 +  (15.0 -  sizeF5.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str4, self.game.MarcFont5, x6, 15, Color.White);
      trect1 = Rectangle::new(x4, 2, 30, 28);
      let mut trect2: &Rectangle = &trect1
      self.AddMouse( trect2, "ZONE OF CONTROL POINTS", "Shows how many ZOC points you are exerting on the hex.\r\n" + Strings.Trim(Conversion.Str( self.game.Data.RuleVar[40])) + "x more ZOC needed then enemy to capture unoccupied hex.");
      let mut x7: i32 = x4 + 30;
      DrawMod.DrawBlockGradient2( g, x7 - 2, 0, 3, 32, Color.FromArgb(200, 80, 90, 110), Color.FromArgb(50, 80, 90, 110));
      str5: String = "AP";
      SizeF sizeF6 = g.MeasureString(str5, self.game.MarcFont5);
      let mut x8: i32 =  Math.Round( ( x7 +  (15.0 -  sizeF6.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str5, self.game.MarcFont5, x8, 2, Color.White);
      str6: String = Strings.Trim(Conversion.Str( (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].get_APPenalty(self.game.EditObj.RealTurn) + self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].get_BattlePenalty(self.game.EditObj.RealTurn))));
      sizeF6 = g.MeasureString(str6, self.game.MarcFont5);
      let mut x9: i32 =  Math.Round( ( x7 +  (15.0 -  sizeF6.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str6, self.game.MarcFont5, x9, 15, Color.White);
      ttext: String = "How much extra AP it costs to move into hex.\r\n" + Strings.Trim(Conversion.Str( self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].get_APPenalty(self.game.EditObj.RealTurn))) + " points for enemy owned hex rule." + "\r\n" + Strings.Trim(Conversion.Str( self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].get_BattlePenalty(self.game.EditObj.RealTurn))) + " points for previous battles fought in hex.";
      trect1 = Rectangle::new(x7, 2, 30, 28);
      let mut trect3: &Rectangle = &trect1
      self.AddMouse( trect3, "ACTION POPENALTIES: i32", ttext);
      let mut x10: i32 = x7 + 30;
      DrawMod.DrawBlockGradient2( g, x10 - 2, 0, 3, 32, Color.FromArgb(200, 80, 90, 110), Color.FromArgb(50, 80, 90, 110));
      str7: String = "STK";
      sizeF6 = g.MeasureString(str7, self.game.MarcFont5);
      let mut x11: i32 =  Math.Round( ( x10 +  (15.0 -  sizeF6.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str7, self.game.MarcFont5, x11, 2, Color.White);
      str8: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetHexStackPts(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected)));
      if (self.game.Data.FOWOn)
      {
        if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime > -1 && !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime, self.game.EditObj.RealTurn))
          str8 = "?";
        if (self.game.EditObj.UnitSelected > -1 && !self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.EditObj.RealTurn))
          str8 = "?";
      }
      if (Operators.CompareString(str8, "?", false) == 0)
      {
        let mut unitCounter: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter;
        num: i32;
        for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        {
          let mut unit: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[index];
          num += self.game.HandyFunctionsObj.GetStackWithFOW(unit, self.game.EditObj.RealTurn);
        }
        if (num > 0)
          str8 = num.ToString();
      }
      sizeF6 = g.MeasureString(str8, self.game.MarcFont5);
      let mut x12: i32 =  Math.Round( ( x10 +  (15.0 -  sizeF6.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str8, self.game.MarcFont5, x12, 15, Color.White);
      trect1 = Rectangle::new(x10, 2, 30, 28);
      let mut trect4: &Rectangle = &trect1
      self.AddMouse( trect4, "STACK POINTS", "How much stack points are in this hex.\r\nAbove " + Strings.Trim(Conversion.Str( self.game.Data.RuleVar[30])) + " points the hex becomes overstacked.");
      let mut x13: i32 = x10 + 30;
      DrawMod.DrawBlockGradient2( g, x13 - 2, 0, 3, 32, Color.FromArgb(200, 80, 90, 110), Color.FromArgb(50, 80, 90, 110));
      str9: String = "VP";
      sizeF6 = g.MeasureString(str9, self.game.MarcFont5);
      let mut x14: i32 =  Math.Round( ( x13 +  (15.0 -  sizeF6.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str9, self.game.MarcFont5, x14, 2, Color.White);
      str10: String = Strings.Trim(Conversion.Str( self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].VP));
      sizeF6 = g.MeasureString(str10, self.game.MarcFont5);
      let mut x15: i32 =  Math.Round( ( x13 +  (15.0 -  sizeF6.Width / 2.0)));
      DrawMod.DrawTextColouredMarc( g, str10, self.game.MarcFont5, x15, 15, Color.White);
      trect1 = Rectangle::new(x13, 2, 30, 28);
      let mut trect5: &Rectangle = &trect1
      self.AddMouse( trect5, "VICTORY POINTS", "How much VP does hex have.");
    }

    pub fn DrawScope( Graphics g)
    {
      let mut num1: i32 =  Math.Round( self.game.ScreenWidth / 2.0 - 158.0);
      if (!(self.game.SelectX > -1 & self.game.SelectY > -1))
        return;
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 370, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      let mut num2: i32 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 125, 0, 0))].Length + 1;
      let mut landscapeType: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType;
      let mut spriteNr: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].SpriteNr;
      data: DataClass = DrawMod.TGame.Data;
      str1: String = "Zones";
       local1: String =  str1;
      let mut libVar: i32 = data.FindLibVar( local1, "SE_Data");
      let mut num3: i32 = 0;
      let mut hexLibVarValue: i32 = DrawMod.TGame.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].GetHexLibVarValue(libVar);
      if (hexLibVarValue > 0)
        num3 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, hexLibVarValue, 13)));
      let mut num4: i32 =  Math.Round( num3 /  num2);
      eventPicOrigSlot1: i32;
      eventPicOrigSlot2: i32;
      if (stringListById1 > -1)
      {
        eventPicOrigSlot1 = num4 >= 50 ? (num4 >= 500 ?  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, landscapeType, 3))) :  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, landscapeType, 2)))) :  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, landscapeType, 1)));
        eventPicOrigSlot2 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, landscapeType, 6)));
      }
      if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].MaxRecon < 1)
        eventPicOrigSlot2 = 61;
      let mut eventPic1: i32 = self.game.Data.FindEventPic(eventPicOrigSlot1, "SE_Present");
      num5: i32;
      num6: i32;
      Rectangle trect;
      Rectangle rectangle;
      if (eventPic1 > -1)
      {
        let mut nr: i32 = self.game.Data.EventPicNr[eventPic1];
        num5 = 256;
        num6 = 80;
         let mut local2: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local3: &Bitmap = &bitmap;
        trect = Rectangle::new(0, 0, 256, 65);
        let mut srcrect: &Rectangle = &trect
        rectangle = Rectangle::new(num1 + 31, 0, 256, 65);
        let mut destrect: &Rectangle = &rectangle
        DrawMod.DrawSimplePart2( local2,  local3, srcrect, destrect);
      }
      let mut eventPic2: i32 = self.game.Data.FindEventPic(eventPicOrigSlot2, "SE_Present");
      if (eventPic2 > -1)
      {
        let mut nr: i32 = self.game.Data.EventPicNr[eventPic2];
        num5 = 256;
        num6 = 80;
         let mut local4: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local5: &Bitmap = &bitmap;
        rectangle = Rectangle::new(0, 0, 256, 65);
        let mut srcrect: &Rectangle = &rectangle
        trect = Rectangle::new(num1 + 31, 0, 256, 65);
        let mut destrect: &Rectangle = &trect
        DrawMod.DrawSimplePart2( local4,  local5, srcrect, destrect);
      }
       let mut local6: &Graphics = &g;
      bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.MARCSCOPE);
       let mut local7: &Bitmap = &bitmap1;
      let mut x1: i32 = num1;
      DrawMod.DrawSimple( local6,  local7, x1, 0);
      if (!(landscapeType > -1 & spriteNr > -1))
        return;
      str2: String = self.game.HandyFunctionsObj.GetHexName(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected);
      SizeF sizeF1 = SizeF::new();
      SizeF sizeF2 = g.MeasureString(str2, self.game.MarcFont5);
      if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location > -1)
      {
        let mut pictureLt: i32 = self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location].Type].PictureLT;
        if (pictureLt > -1)
          str2 = self.game.Data.LandscapeTypeObj[pictureLt].Name.Replace("(System)", "");
      }
      str3: String = str2;
      if ( sizeF2.Width < 135.0 & self.w >= 1600)
        str2 = str2 + " (" + Strings.Trim(Conversion.Str( self.game.SelectX)) + "," + Strings.Trim(Conversion.Str( self.game.SelectY)) + ") ";
      str3 + " (" + Strings.Trim(Conversion.Str( self.game.SelectX)) + "," + Strings.Trim(Conversion.Str( self.game.SelectY)) + ") ";
      sizeF1 = g.MeasureString(str2, self.game.MarcFont5);
      str4: String;
      regime: i32;
      if (self.game.EditObj.OrderType == 26)
      {
        if (self.game.EditObj.HisOwner[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY] > -1)
        {
          str4 = self.game.Data.RegimeObj[self.game.EditObj.HisOwner[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY]].Name;
          regime = self.game.EditObj.HisOwner[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY];
        }
      }
      else if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime > -1)
      {
        str4 = self.game.EventRelatedObj.Helper_GetOwnerNameForGuiDisplay(self.game.SelectX, self.game.SelectY);
        regime = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Regime;
      }
      landscapeMouseOverText: String = self.game.HandyFunctionsObj.GetLandscapeMouseOverText();
      if (1600 > self.w)
      {
        str5: String = Strings.UCase(Strings.Left(str2, 1)) + Strings.Mid(str2, 2);
        let mut x2: i32 =  Math.Round( self.w / 2.0 - 115.0 - 75.0 -  g.MeasureString(str5, self.game.MarcFont5).Width / 2.0);
        DrawMod.DrawTextColouredMarc( g, str5, self.game.MarcFont5, x2, 9, Color.White);
        rectangle = Rectangle::new( Math.Round( self.game.ScreenWidth / 2.0 - 130.0), 0, 100, 62);
        trect = rectangle;
        self.AddMouse( trect, str5, landscapeMouseOverText);
        str6: String = str4;
        let mut x3: i32 =  Math.Round( self.w / 2.0 + 125.0 + 75.0 -  g.MeasureString(str6, self.game.MarcFont5).Width / 2.0);
        DrawMod.DrawTextColouredMarc( g, str6, self.game.MarcFont5, x3, 9, Color.White);
      }
      else
      {
        str7: String = Strings.UCase(Strings.Left(str2, 1)) + Strings.Mid(str2, 2);
        let mut x4: i32 =  Math.Round( self.w / 2.0 - 175.0 - 75.0 -  g.MeasureString(str7, self.game.MarcFont4).Width / 2.0);
        DrawMod.DrawTextColouredMarc( g, str7, self.game.MarcFont4, x4, 4, Color.White);
        rectangle = Rectangle::new( Math.Round( self.game.ScreenWidth / 2.0 - 130.0), 0, 260, 62);
        trect = rectangle;
        self.AddMouse( trect, str7, landscapeMouseOverText);
        str8: String = str4;
        let mut x5: i32 =  Math.Round( self.w / 2.0 + 175.0 + 75.0 -  g.MeasureString(str8, self.game.MarcFont4).Width / 2.0);
        if (Operators.CompareString(str8, "Unknown", false) != 0 & Operators.CompareString(str8, "Unclear", false) != 0)
        {
           let mut local8: &Graphics = &g;
          bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[regime].SymbolSpriteNr);
           let mut local9: &Bitmap = &bitmap2;
          let mut x6: i32 = x5 - 24;
          let mut width: i32 = BitmapStore.GetWidth(self.game.Data.RegimeObj[regime].SymbolSpriteNr);
          let mut origh: i32 = BitmapStore.Getheight(self.game.Data.RegimeObj[regime].SymbolSpriteNr);
          double r =  ( self.game.Data.RegimeObj[regime].Red3 /  byte.MaxValue);
          double g1 =  ( self.game.Data.RegimeObj[regime].Green3 /  byte.MaxValue);
          double b =  ( self.game.Data.RegimeObj[regime].Blue3 /  byte.MaxValue);
          DrawMod.DrawScaledColorized2( local8,  local9, x6, 0, 24, 24, width, origh,  r,  g1,  b,  byte.MaxValue);
        }
        DrawMod.DrawTextColouredMarc( g, str8, self.game.MarcFont4, x5, 4, Color.White);
      }
    }

    pub fn DrawDate( Graphics g)
    {
      if (self.game.EditObj.RealTurn <= -1)
        return;
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 125, 0, 0));
      let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, 14, 2)));
      let mut num2: i32 = self.game.Data.StringListObj[stringListById2].Length + 1;
      let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, 12, 2)));
      self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 1);
      let mut num3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData(0, 47, 2)));
      let mut Month: i32 = (self.game.EditObj.RealRound + 6) % 6 * 2 - 1;
      if (Month < 1)
        Month = 11;
      if (Information.IsNothing( self.game.Data.TurnString))
        self.game.Data.TurnString = "";
      SizeF sizeF = SizeF::new();
      strArray: Vec<String> = self.game.Data.TurnString.Split(new char[1]
      {
        ','
      }, StringSplitOptions.RemoveEmptyEntries);
      if (strArray.GetUpperBound(0) <= 0)
        return;
      let mut x1: i32 = self.game.ScreenWidth - 190 + 65 -  Math.Round( (g.MeasureString(strArray[0], self.game.MarcFont16).Width / 2f));
      DrawMod.DrawTextColouredConsole( g, strArray[0], self.game.MarcFont16, x1, 11, Color.LightGray);
      let mut x2: i32 = self.game.ScreenWidth - 190 + 65 -  Math.Round( (g.MeasureString(strArray[1], self.game.MarcFont16).Width / 2f));
      DrawMod.DrawTextColouredConsole( g, strArray[1], self.game.MarcFont16, x2, 27, Color.LightGray);
      Rectangle trect = Rectangle::new(x2, 16, 106, 32);
      self.AddMouse( trect, "", "This is Round " + self.game.EditObj.RealRound.ToString() + ".\r\nThis is Local Year " + num3.ToString() + " AA, Season " + idValue.ToString() + " of " + num2.ToString() + ".\r\nThis is Earth Year " + num1.ToString() + "-" + DateAndTime.MonthName(Month, true) + ".");
    }

    pub fn DrawPP( Graphics g)
    {
      if (self.game.EditObj.RealTurn <= -1)
        return;
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
      let mut id: i32 = self.game.Data.RegimeObj[self.game.EditObj.RealTurn].id;
      if (self.game.Data.RegimeObj[self.game.EditObj.RealTurn].AI &  self.game.Data.RuleVar[976] == 1.0)
        return;
      SizeF sizeF1 = SizeF::new();
      SizeF sizeF2 = SizeF::new();
      SizeF sizeF3 = SizeF::new();
      SizeF sizeF4 = SizeF::new();
      SizeF sizeF5 = SizeF::new();
      data2: String = self.game.Data.StringListObj[stringListById1].GetData2(0, id, 1, "fp", 2);
      str1: String = self.game.Data.RegimeObj[self.game.EditObj.RealTurn].ResPts.ToString();
      if (self.game.Data.RegimeObj[self.game.EditObj.RealTurn].ResPts > 9999)
        str1 = ( Math.Round( self.game.Data.RegimeObj[self.game.EditObj.RealTurn].ResPts / 1000.0)).ToString() + "k";
      else if (self.game.Data.RegimeObj[self.game.EditObj.RealTurn].ResPts > 999)
        str1 = Math.Round( self.game.Data.RegimeObj[self.game.EditObj.RealTurn].ResPts / 1000.0, 1).ToString() + "k";
      let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, id, 1, "credits", 2)));
      str2: String;
      if (num1 > 9999)
      {
        double num2 = Math.Round( num1 / 1000.0, 1);
        str2 = num2.ToString() + "k";
        if (num1 >= 100000)
        {
          num2 = Math.Round( num1 / 1000.0, 0);
          str2 = num2.ToString() + "k";
        }
        if (num1 == 0)
          str2 = "0";
      }
      else
        str2 = num1.ToString();
      sizeF1 = g.MeasureString(data2, self.game.shadowFontConsole);
      sizeF2 = g.MeasureString(str1, self.game.shadowFontConsole);
      sizeF3 = g.MeasureString(str2, self.game.shadowFontConsole);
      let mut x1: i32 = 302;
      let mut y1: i32 = 0;
       let mut local1: &Graphics = &g;
      bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_VARBOX);
       let mut local2: &Bitmap = &bitmap1;
      let mut x2: i32 = x1;
      let mut y2: i32 = y1;
      DrawMod.DrawSimple( local1,  local2, x2, y2);
      Rectangle trect1 = Rectangle::new(x1, y1, 74, 28);
      self.AddMouse( trect1, "Fate Points", "You need FP’s to play powerful Fate Stratagems.");
      let mut eventPicSlotFor1: i32 = self.game.EventRelatedObj.GetEventPicSlotFor(0, "", "fp");
       let mut local3: &Graphics = &g;
      bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPicSlotFor1]);
       let mut local4: &Bitmap = &bitmap2;
      let mut x3: i32 = x1 + 2;
      DrawMod.DrawSimple( local3,  local4, x3, 6);
      DrawMod.DrawTextColouredConsole( g, data2, self.game.MarcFont16, x1 + 31, 4, self.game.seColWhite);
      let mut x4: i32 = x1 + 75;
       let mut local5: &Graphics = &g;
      bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_VARBOX);
       let mut local6: &Bitmap = &bitmap3;
      let mut x5: i32 = x4;
      let mut y3: i32 = y1;
      DrawMod.DrawSimple( local5,  local6, x5, y3);
      trect1 = Rectangle::new(x4, y1, 74, 28);
      let mut trect2: &Rectangle = &trect1
      self.AddMouse( trect2, "Political Points", "You need PP’s to play organisation-generated Stratagems and sometimes make Decisions.");
      let mut eventPicSlotFor2: i32 = self.game.EventRelatedObj.GetEventPicSlotFor(0, "", "pp");
       let mut local7: &Graphics = &g;
      bitmap4: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPicSlotFor2]);
       let mut local8: &Bitmap = &bitmap4;
      let mut x6: i32 = x4 + 2;
      DrawMod.DrawSimple( local7,  local8, x6, 6);
      DrawMod.DrawTextColouredConsole( g, str1, self.game.MarcFont16, x4 + 31, 4, self.game.seColWhite);
      let mut x7: i32 = x4 + 75;
       let mut local9: &Graphics = &g;
      bitmap5: Bitmap = BitmapStore.GetBitmap(self.game.SE1_RESOURCEBAR_VARBOX);
       let mut local10: &Bitmap = &bitmap5;
      let mut x8: i32 = x7;
      let mut y4: i32 = y1;
      DrawMod.DrawSimple( local9,  local10, x8, y4);
      trect1 = Rectangle::new(x7, y1, 74, 28);
      let mut trect3: &Rectangle = &trect1
      self.AddMouse( trect3, "Credits", "You need credits to buy with traders and to pay leaders, workers and others.");
      let mut eventPicSlotFor3: i32 = self.game.EventRelatedObj.GetEventPicSlotFor(0, "", "credits");
       let mut local11: &Graphics = &g;
      bitmap6: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPicSlotFor3]);
       let mut local12: &Bitmap = &bitmap6;
      let mut x9: i32 = x7 + 2;
      DrawMod.DrawSimple( local11,  local12, x9, 6);
      DrawMod.DrawTextColouredConsole( g, str2, self.game.MarcFont16, x7 + 31, 4, self.game.seColWhite);
      if (self.game.EditObj.se1_ManagementMode)
        return;
      if (self.game.EditObj.BlockAdvice & self.game.EditObj.showAdvice)
      {
        if (self.game.ScreenWidth >= 1353)
        {
          let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
          let mut num3: i32 = self.game.Data.StringListObj[stringListById2].Length + 1;
          if (stringListById2 <= -1 || self.game.Data.StringListObj[stringListById2].Length <= -1)
            return;
          let mut num4: i32 = x7 + 83;
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Adv.[" + num3.ToString() + "]", 80, "Click for opening advice window.",  self.OwnBitmap, num4, y1 + 2, theight: 26);
          self.advice = self.AddSubPart( tsubpart, num4, y1 + 2, 80, 26, 1);
        }
        else
        {
          let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
          if (stringListById3 <= -1 || self.game.Data.StringListObj[stringListById3].Length <= -1)
            return;
          let mut num5: i32 = x7 + 71;
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Ad", 25, "Click for opening advice window.",  self.OwnBitmap, num5, y1 + 2, theight: 26);
          self.advice = self.AddSubPart( tsubpart, num5, y1 + 2, 25, 26, 1);
        }
      }
      else
      {
        if (!(!self.game.EditObj.BlockAdvice & self.game.EditObj.showAdvice))
          return;
        let mut num6: i32 = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0))].Length + 1;
        if (self.game.ScreenWidth >= 1353)
        {
          if (num6 > 0)
          {
            let mut num7: i32 = x7 + 83;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Adv.[" + num6.ToString() + "]", 80, "Advice Window is open.",  self.OwnBitmap, num7, y1 + 2, true, theight: 26, tMarcStyle: true);
            self.adviceB = self.AddSubPart( tsubpart, num7, y1 + 2, 80, 26, 0);
          }
          else
          {
            let mut num8: i32 = x7 + 83;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Adv.[" + num6.ToString() + "]", 80, "No Advice left. Nothing given this round or everything has been dismissed already.",  self.OwnBitmap, num8, y1 + 2, true, theight: 26, tMarcStyle: true);
            self.adviceB = self.AddSubPart( tsubpart, num8, y1 + 2, 80, 26, 0);
          }
        }
        else if (num6 > 0)
        {
          let mut num9: i32 = x7 + 71;
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Ad", 25, "Advice Window is open.",  self.OwnBitmap, num9, y1 + 2, true, theight: 26, tMarcStyle: true);
          self.adviceB = self.AddSubPart( tsubpart, num9, y1 + 2, 25, 26, 0);
        }
        else
        {
          let mut num10: i32 = x7 + 71;
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Ad", 25, "No Advice left. Nothing given this round or everything has been dismissed already.",  self.OwnBitmap, num10, y1 + 2, true, theight: 26, tMarcStyle: true);
          self.adviceB = self.AddSubPart( tsubpart, num10, y1 + 2, 25, 26, 0);
        }
      }
    }

    pub fn DrawResources( Graphics g)
    {
      SizeF sizeF = SizeF::new();
      int[] numArray = new int[10];
      let mut num1: i32 = 3;
      numArray[1] =  Math.Round( self.game.ScreenWidth / 2.0 + 158.0 + 162.0);
      numArray[2] =  Math.Round( self.game.ScreenWidth / 2.0 + 158.0 + 162.0 + 75.0 + 5.0);
      numArray[3] = 165;
      let mut index1: i32 = 0;
      Right: String = "oil" + self.game.Data.RuleVar[949].ToString();
      if ( self.game.Data.RuleVar[949] < 1.0)
        Right = "1!impossible!!x423121";
      Rectangle trect1;
      if ( self.game.Data.RuleVar[411] > 0.0 && self.game.Data.TempString[731].Length > 1)
      {
        index1 += 1;
        let mut num2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[411]))].GetData2(0, self.game.Data.RegimeObj[self.game.EditObj.RealTurn].id, 1, self.game.Data.TempString[731], 2)));
        index2: i32;
        tstring: String = num2 <= 9999 ? self.game.Data.TempString[732] + " " + num2.ToString() : self.game.Data.TempString[732] + " " + (Conversion.Int( self.game.Data.GameSlot[index2] / 1000.0).ToString() + "k");
        let mut x1: i32 = 165;
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
         let mut local2: &Bitmap = &bitmap;
        let mut x2: i32 = x1;
        DrawMod.DrawSimple( local1,  local2, x2, 2);
        DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x1 + 2, 5, Color.White);
        trect1 = Rectangle::new(x1, 2, 75, 20);
        self.AddMouse( trect1, "", self.game.Data.TempString[733]);
      }
      if (self.game.Data.Product >= 7)
        return;
      let mut index3: i32 = 0;
      double num3;
      do
      {
        if (self.game.Data.GameSlotShow2[index3] & self.game.Data.GameSlot[index3] > -1)
        {
          index1 += 1;
          if (index1 <= num1)
          {
            tstring: String = Strings.Trim(Conversion.Str( self.game.Data.GameSlot[index3]));
            if (self.game.Data.GameSlot[index3] > 9999)
            {
              num3 = Conversion.Int( self.game.Data.GameSlot[index3] / 1000.0);
              tstring = num3.ToString() + "k";
            }
            if (self.game.Data.GameSlotSmallGfx[index3] > -1)
            {
               let mut local3: &Graphics = &g;
              bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
               let mut local4: &Bitmap = &bitmap1;
              let mut x3: i32 = numArray[index1];
              DrawMod.DrawSimple( local3,  local4, x3, 2);
               let mut local5: &Graphics = &g;
              bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.Data.SmallPicNr[self.game.Data.GameSlotSmallGfx[index3]]);
               let mut local6: &Bitmap = &bitmap2;
              let mut x4: i32 = numArray[index1];
              DrawMod.DrawSimple( local5,  local6, x4, 2);
              let mut x5: i32 = numArray[index1] + 24;
              DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x5, 5, Color.White);
              trect1 = Rectangle::new(numArray[index1], 2, 75, 20);
              let mut trect2: &Rectangle = &trect1
              self.AddMouse( trect2, "", self.game.Data.GameSlotName[index3]);
            }
            else if (self.game.Data.GameSlotNato[index3] > 0)
            {
              if (self.game.NATO[self.game.Data.GameSlotNato[index3]] > 0)
              {
                 let mut local7: &Graphics = &g;
                bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
                 let mut local8: &Bitmap = &bitmap3;
                let mut x6: i32 = numArray[index1];
                DrawMod.DrawSimple( local7,  local8, x6, 2);
                 let mut local9: &Graphics = &g;
                bitmap4: Bitmap = BitmapStore.GetBitmap(self.game.NATO[self.game.Data.GameSlotNato[index3]]);
                 let mut local10: &Bitmap = &bitmap4;
                let mut x7: i32 = numArray[index1];
                DrawMod.DrawSimple( local9,  local10, x7, 2);
                let mut x8: i32 = numArray[index1] + 24;
                DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x8, 5, Color.White);
                trect1 = Rectangle::new(numArray[index1], 2, 75, 20);
                let mut trect3: &Rectangle = &trect1
                self.AddMouse( trect3, "", self.game.Data.GameSlotName[index3]);
              }
            }
            else
            {
               let mut local11: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
               let mut local12: &Bitmap = &bitmap;
              let mut x9: i32 = numArray[index1];
              DrawMod.DrawSimple( local11,  local12, x9, 2);
              let mut x10: i32 = numArray[index1] + 24;
              DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x10, 5, Color.White);
              trect1 = Rectangle::new(numArray[index1], 2, 75, 20);
              let mut trect4: &Rectangle = &trect1
              self.AddMouse( trect4, "", self.game.Data.GameSlotName[index3]);
            }
          }
        }
        index3 += 1;
      }
      while (index3 <= 499);
      let mut index4: i32 = 0;
      do
      {
        if (self.game.Data.RegimeSlotShow[index4] &  self.game.Data.RuleVar[814] < 1.0)
        {
          let mut index5: i32 = index4;
          x11: i32;
          tstring: String;
          if (Operators.CompareString(self.game.Data.RegimeSlotName[index4], Right, false) == 0)
          {
            if (self.game.SelectX > -1 & self.game.SelectY > -1)
            {
              x11 = self.game.HandyFunctionsObj.GetFuelSlot949(-1, self.game.SelectX, self.game.SelectY);
              index5 = x11;
              if (x11 > -1)
              {
                x11 = self.game.Data.RegimeObj[self.game.EditObj.RealTurn].RegimeSlot[x11];
                tstring = x11.ToString();
                if (x11 > 9999)
                {
                  num3 = Conversion.Int( x11 / 1000.0);
                  tstring = num3.ToString() + "k";
                }
              }
              else
                tstring = "N/A";
            }
            else
              tstring = "N/A";
          }
          else
          {
            tstring = Strings.Trim(Conversion.Str( self.game.Data.RegimeObj[self.game.EditObj.RealTurn].RegimeSlot[index4]));
            if (self.game.Data.RegimeObj[self.game.EditObj.RealTurn].RegimeSlot[index4] > 9999)
            {
              num3 = Conversion.Int( self.game.Data.RegimeObj[self.game.EditObj.RealTurn].RegimeSlot[index4] / 1000.0);
              tstring = num3.ToString() + "k";
            }
          }
          if (index5 > -1 && self.game.Data.RegimeObj[self.game.EditObj.RealTurn].RegimeSlot[index5] > -1)
          {
            index1 += 1;
            if (index1 <= num1)
            {
              if (self.game.Data.RegimeSlotSmallGfx[index5] > 0)
              {
                 let mut local13: &Graphics = &g;
                bitmap5: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
                 let mut local14: &Bitmap = &bitmap5;
                let mut x12: i32 = numArray[index1];
                DrawMod.DrawSimple( local13,  local14, x12, 2);
                 let mut local15: &Graphics = &g;
                bitmap6: Bitmap = BitmapStore.GetBitmap(self.game.Data.SmallPicNr[self.game.Data.RegimeSlotSmallGfx[index5]]);
                 let mut local16: &Bitmap = &bitmap6;
                let mut x13: i32 = numArray[index1];
                DrawMod.DrawSimple( local15,  local16, x13, 2);
                x11 = numArray[index1] + 24;
                DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x11, 5, Color.White);
                trect1 = Rectangle::new(numArray[index1], 2, 75, 20);
                let mut trect5: &Rectangle = &trect1
                self.AddMouse( trect5, "", self.game.Data.RegimeSlotName[index5]);
              }
              else if (self.game.Data.RegimeSlotNato[index5] > 0)
              {
                 let mut local17: &Graphics = &g;
                bitmap7: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
                 let mut local18: &Bitmap = &bitmap7;
                let mut x14: i32 = numArray[index1];
                DrawMod.DrawSimple( local17,  local18, x14, 2);
                 let mut local19: &Graphics = &g;
                bitmap8: Bitmap = BitmapStore.GetBitmap(self.game.NATO[self.game.Data.RegimeSlotNato[index5]]);
                 let mut local20: &Bitmap = &bitmap8;
                let mut x15: i32 = numArray[index1];
                DrawMod.DrawSimple( local19,  local20, x15, 2);
                x11 = numArray[index1] + 24;
                DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x11, 5, Color.White);
                trect1 = Rectangle::new(numArray[index1], 2, 75, 20);
                let mut trect6: &Rectangle = &trect1
                self.AddMouse( trect6, "", self.game.Data.RegimeSlotName[index5]);
              }
              else
              {
                 let mut local21: &Graphics = &g;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCOPTSLOTS);
                 let mut local22: &Bitmap = &bitmap;
                let mut x16: i32 = numArray[index1];
                DrawMod.DrawSimple( local21,  local22, x16, 2);
                x11 = numArray[index1] + 24;
                DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont8, x11, 5, Color.White);
                trect1 = Rectangle::new(numArray[index1], 2, 75, 20);
                let mut trect7: &Rectangle = &trect1
                self.AddMouse( trect7, "", self.game.Data.RegimeSlotName[index5]);
              }
            }
          }
        }
        index4 += 1;
      }
      while (index4 <= 499);
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          if (Strings.InStr(self.game.EditObj.TipText, "MX-ENTR") <= 0)
            return;
          self.game.EditObj.TipTitle += "<FIXEDSYS>";
          return;
        }
      }
      if (self.SubPartCounter <= -1)
        return;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.game.EditObj.TipButton = false;
          self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (self.game.EditObj.TipButton)
            break;
          if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            break;
          }
        }
      }
    }

    pub fn PopUpRefresh()
    {
      self.surrendering &= self.game.EditObj.AnswerChosen == 1;
      if (self.AskingAboutMetrics)
      {
        if (self.game.EditObj.AnswerChosen == 1)
          self.game.EditObj.allowMetrics = true;
        else
          self.game.EditObj.allowMetrics = false;
        self.game.EditObj.askedMetricsPermission = true;
        self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
      }
      self.DoRefresh();
    }

    pub DoEndTurnStuff: WindowReturnClass(tMouseButPressed: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!self.game.Data.RegimeObj[self.game.Data.Turn].AI & !self.game.SuperAdminRights)
      {
        let mut locCounter: i32 = self.game.Data.LocCounter;
        num: i32;
        for (let mut index: i32 = 0; index <= locCounter; index += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index].X, self.game.Data.LocObj[index].Y].Regime == self.game.Data.Turn)
            num += 1;
        }
        if (self.game.Data.RegimeObj[self.game.Data.Turn].Sleep)
          num = 0;
        if (num < 1)
        {
          self.surrendering = true;
          windowReturnClass = self.DoSurrenderStuff();
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
          {
            if (!self.game.Data.RegimeObj[index].Sleep & !self.game.Data.RegimeObj[index].AI)
              self.game.EventRelatedObj.Helper_AddDetailedReport(DetailType.ForeignAffairs, self.game.Data.RegimeObj[self.game.Data.Turn].id, self.game.Data.RegimeObj[self.game.Data.Turn].Name + " has been eliminated from the game.", self.game.Data.RegimeObj[index].id);
          }
          if (windowReturnClass.Flag)
            return windowReturnClass;
        }
      }
      if (self.game.EventRelatedObj.Helper_IsDebug() & tMouseButPressed == 2)
      {
        let mut integer: i32 = Conversions.ToInteger(Interaction.InputBox("Run with AI only for howmany rounds?", "Shadow Empire : Planetary Conquest"));
        if (integer > 0)
          self.game.EditObj.debugAiOnlyTillRound = self.game.Data.Round + integer;
        else
          self.game.EditObj.debugAiOnlyTillRound = 0;
      }
      let mut humanPlayers: i32 = self.game.HandyFunctionsObj.GetHumanPlayers();
      self.game.EditObj.DoCardSlot = -1;
      self.game.EditObj.HandCard = -1;
      self.game.EditObj.se1_ManagementMode = false;
      if (humanPlayers < 1)
      {
        self.game.Data = DataClass::new();
        self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
        if (self.game.Data.UseAI == 1 & !Information.IsNothing( self.game.NewAIObj))
          self.game.NewAIObj.LastRegime = -1;
        self.game.EditObj.ShowInitialMenu = true;
        windowReturnClass.AddCommand(3, 12);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      self.game.FormRef.Cursor = Cursors.WaitCursor;
      if (self.game.EditObj.Screenshoton)
        self.game.HandyFunctionsObj.doscreenshot("b", 0);
      if (self.game.EditObj.AutoSave & !self.game.Data.PBEM)
      {
        self.game.EventRelatedObj.ExecSuperImposeMessage("Ending Turn", "Making an Auto-Save first", 0, 0, "");
        str: String = self.game.AppPath_SAVEGAMES + "autosave_round" + self.game.EditObj.RealRound.ToString() + ".se1";
        self.game.Data.serialize(str);
        self.game.HandyFunctionsObj.ZipFile(str);
        GC.Collect();
        Application.DoEvents();
      }
      if (self.game.EditObj.RealTurn != -1 && !self.game.Data.RegimeObj[self.game.EditObj.RealTurn].AI)
        self.game.EventRelatedObj.DoCheckEvents(5);
      let mut num1: i32 = 0;
      let mut index1: i32 = self.game.EditObj.RealTurn + 1;
      if (self.game.EditObj.RealTurn > self.game.Data.RegimeCounter)
        index1 = 0;
      if (!self.game.Data.RegimeObj[index1].AI & !self.game.Data.RegimeObj[index1].Sleep)
        num1 = 1;
      self.game.HandyFunctionsObj.ClearHistory( self.game.Data.Turn);
      if (num1 > 0 | self.game.EventRelatedObj.Helper_IsDebug() & self.game.EditObj.debugAiOnlyTillRound >= self.game.Data.Round | self.game.Data.DontShowAIMove)
      {
        self.game.EditObj.Test = -1;
        self.game.Data.DontShowAIMove = true;
        self.game.EditObj.HumanPlayer = -1;
        windowReturnClass.AddCommand(3, 13);
      }
      else
      {
        self.game.EditObj.TempAIWatch = true;
        self.game.EditObj.HumanPlayer = self.game.EditObj.RealTurn;
        self.game.se1GameLoop = new GameLoopClass2( self.game);
        self.game.se1GameLoop.Setup();
        self.game.se1Running = true;
        self.game.se1ThreadRunning = true;
        self.game.se1Thread = new Thread(new ThreadStart(self.game.se1GameLoop.handleTimer));
        self.game.se1Thread.Name = "Game Loop Thread";
        self.game.se1Thread.Start();
        windowReturnClass.AddCommand(3, 16);
      }
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.TutOrder > -1)
        return windowReturnClass;
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num: i32 = self.SubPartID[index];
            if (num == self.cinButId)
            {
              windowReturnClass.AddCommand(3, 22);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.advice)
            {
              self.game.EditObj.BlockAdvice = false;
              self.game.EditObj.TempBlockAdvice = false;
              if (self.game.EditObj.SetViewMode2 > 0)
              {
                self.game.EditObj.SetViewMode2 = 0;
                windowReturnClass.AddCommand(1, 9);
                windowReturnClass.AddCommand(7, 12);
                windowReturnClass.SetFlag(true);
              }
              self.dostuff();
              windowReturnClass.AddCommand(2, 119);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.special1)
            {
              self.game.EditObj.PopupValue = 23;
              windowReturnClass.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.special2)
            {
              self.game.EditObj.PopupValue = 24;
              windowReturnClass.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.special3)
            {
              self.game.EditObj.PopupValue = 25;
              windowReturnClass.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.butNextTurnId)
            {
              self.game.Data.DontShowAIMove = self.game.EditObj.dontShowAImoves;
              self.game.EditObj.se1_ManagementMode = false;
              self.game.Data.RegimeObj[self.game.Data.Turn].Version = 110;
              self.game.Data.RegimeObj[self.game.Data.Turn].subVersion = ".04b";
              if (self.game.EventRelatedObj.Helper_IsDebug())
                self.game.Data.DontShowAIMove = true;
              return self.DoEndTurnStuff(b);
            }
            if (num == self.butNextTurnId2)
            {
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.butHistoryId)
            {
              self.game.EditObj.LayerSupplyOn = false;
              self.game.EditObj.OrderType = 26;
              windowReturnClass.AddCommand(3, 16);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.butPlayId)
            {
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(3, 11);
              self.game.EditObj.OrderType = 0;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0)
        {
          if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
          {
            let mut num: i32 = self.MouseData[index];
            switch (num)
            {
              case 2001:
                self.game.EditObj.se1_ManagementMode = false;
                self.game.EditObj.TempCoordList = CoordList::new();
                self.game.EditObj.OrderType = 0;
                windowReturnClass.AddCommand(3, 11);
                self.game.EditObj.OrderType = 0;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              case 2002:
                self.game.EditObj.se1_ManagementMode = false;
                self.game.EditObj.LayerSupplyOn = false;
                self.game.EditObj.OrderType = 26;
                windowReturnClass.AddCommand(3, 16);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              case 2003:
                self.game.EditObj.OrderType = 0;
                windowReturnClass.AddCommand(3, 22);
                self.game.EditObj.se1_ManagementMode = false;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              case 2004:
                self.game.EditObj.se1_ManagementMode = true;
                windowReturnClass.AddCommand(3, 24);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              default:
                if (num == self.game.EditObj.SetViewMode2)
                {
                  if (self.game.EditObj.GuiDown | self.game.EditObj.RightDown)
                  {
                    self.game.EditObj.GuiDown = false;
                    self.game.EditObj.RightDown = false;
                    self.game.EditObj.SetViewMode2 = 0;
                    windowReturnClass.SetFlag(true);
                    windowReturnClass.AddCommand(3, 11);
                    return windowReturnClass;
                  }
                  self.game.EditObj.SetViewMode2 = 0;
                  self.dostuff();
                  windowReturnClass.AddCommand(1, 9);
                  windowReturnClass.AddCommand(7, 12);
                  windowReturnClass.SetFlag(true);
                  windowReturnClass.NoMouseClickBelow = true;
                  return windowReturnClass;
                }
                switch (num)
                {
                  case 1:
                    if (self.game.EditObj.SetViewMode2 == 1)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 1;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 70);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 2:
                    if (self.game.EditObj.SetViewMode2 == 2)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 2;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 71);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 3:
                    if (self.game.EditObj.SetViewMode2 == 3)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 3;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 72);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 4:
                    if (self.game.EditObj.SetViewMode2 == 4)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 4;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 73);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 5:
                    if (self.game.EditObj.SetViewMode2 == 5)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 5;
                    self.DoRefresh();
                    if (self.game.ScreenHeight < 920)
                    {
                      self.game.EditObj.GuiDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else
                    {
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.AddCommand(2, 74);
                    }
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 6:
                    if (self.game.EditObj.SetViewMode2 == 6)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 6;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 75);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 7:
                    if (self.game.EditObj.SetViewMode2 == 7)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 7;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 76);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 8:
                    if (self.game.EditObj.SetViewMode2 == 8)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 8;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 77);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 9:
                    if (self.game.EditObj.SetViewMode2 == 9)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 9;
                    self.DoRefresh();
                    windowReturnClass.AddCommand(1, 9);
                    windowReturnClass.AddCommand(7, 12);
                    windowReturnClass.AddCommand(2, 110);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 11:
                    if (self.game.EditObj.SetViewMode2 == 11)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 11;
                    self.DoRefresh();
                    if (self.game.ScreenHeight < 920 & self.game.ScreenWidth < 1465)
                    {
                      self.game.EditObj.GuiDown = true;
                      self.game.EditObj.RightDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else if (self.game.ScreenHeight < 920)
                    {
                      self.game.EditObj.GuiDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else if (self.game.ScreenWidth < 1465)
                    {
                      self.game.EditObj.RightDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else
                    {
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.AddCommand(2, 113);
                    }
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 12:
                    if (self.game.EditObj.SetViewMode2 == 12)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 12;
                    self.DoRefresh();
                    if (self.game.ScreenHeight < 920 & self.game.ScreenWidth < 1465)
                    {
                      self.game.EditObj.GuiDown = true;
                      self.game.EditObj.RightDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else if (self.game.ScreenHeight < 920)
                    {
                      self.game.EditObj.GuiDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else if (self.game.ScreenWidth < 1465)
                    {
                      self.game.EditObj.RightDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else
                    {
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.AddCommand(2, 113);
                    }
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 13:
                    if (self.game.EditObj.SetViewMode2 == 13)
                    {
                      self.game.EditObj.SetViewMode2 = 0;
                      self.dostuff();
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.SetFlag(true);
                      windowReturnClass.NoMouseClickBelow = true;
                      return windowReturnClass;
                    }
                    self.game.EditObj.SetViewMode2 = 13;
                    self.DoRefresh();
                    if (self.game.ScreenHeight < 920 & self.game.ScreenWidth < 1465)
                    {
                      self.game.EditObj.GuiDown = true;
                      self.game.EditObj.RightDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else if (self.game.ScreenHeight < 920)
                    {
                      self.game.EditObj.GuiDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else if (self.game.ScreenWidth < 1465)
                    {
                      self.game.EditObj.RightDown = true;
                      windowReturnClass.AddCommand(3, 11);
                    }
                    else
                    {
                      windowReturnClass.AddCommand(1, 9);
                      windowReturnClass.AddCommand(7, 12);
                      windowReturnClass.AddCommand(2, 113);
                    }
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 51:
                    self.game.EditObj.se1_ManagementTab = 51;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 52:
                    self.game.EditObj.se1_ManagementTab = 52;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.AddCommand(7, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 53:
                    self.game.EditObj.se1_ManagementTab = 53;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.AddCommand(7, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 54:
                    self.game.EditObj.se1_ManagementTab = 54;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.AddCommand(7, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 55:
                    self.game.EditObj.se1_ManagementTab = 55;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.AddCommand(7, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 56:
                    self.game.EditObj.se1_ManagementTab = 56;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.AddCommand(7, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 57:
                    self.game.EditObj.se1_ManagementTab = 57;
                    windowReturnClass.AddCommand(1, 12);
                    windowReturnClass.AddCommand(1, 67);
                    windowReturnClass.AddCommand(2, 12);
                    windowReturnClass.AddCommand(2, 67);
                    windowReturnClass.AddCommand(7, 67);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  case 501:
                    if (self.game.EditObj.OrderType == 26)
                    {
                      self.game.EditObj.TempCoordList = CoordList::new();
                      self.game.HandyFunctionsObj.SetInitialXY(self.game.EditObj.RealTurn);
                      self.game.EditObj.UnitSelected = -1;
                      if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1)
                        self.game.EditObj.UnitSelected = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
                      windowReturnClass.AddCommand(4, 12);
                      windowReturnClass.AddCommand(4, 80);
                      windowReturnClass.AddCommand(4, 67);
                      windowReturnClass.AddCommand(4, 81);
                      windowReturnClass.AddCommand(4, 9);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    self.game.EditObj.TempCoordList = CoordList::new();
                    self.game.HandyFunctionsObj.SetInitialXY(self.game.EditObj.RealTurn);
                    self.game.EditObj.UnitSelected = -1;
                    if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > -1)
                      self.game.EditObj.UnitSelected = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0];
                    windowReturnClass.AddCommand(3, 11);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  default:
                    continue;
                }
            }
          }
        }
        else if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] == -2)
        {
          if (self.game.EditObj.SetViewMode2 > 0)
          {
            self.game.EditObj.SetViewMode2 = 0;
            self.dostuff();
            windowReturnClass.AddCommand(1, 9);
            windowReturnClass.AddCommand(7, 12);
            windowReturnClass.SetFlag(true);
          }
          windowReturnClass.NoMouseClickBelow = true;
          return windowReturnClass;
        }
      }
      if (y < 32)
      {
        if (self.game.EditObj.SetViewMode2 > 0)
        {
          self.game.EditObj.SetViewMode2 = 0;
          self.dostuff();
          windowReturnClass.AddCommand(1, 9);
          windowReturnClass.AddCommand(7, 12);
          windowReturnClass.SetFlag(true);
        }
        windowReturnClass.NoMouseClickBelow = true;
        return windowReturnClass;
      }
      if (y < 64 &&  x >  self.w / 2.0 - 158.0 &  x <  self.w / 2.0 + 158.0)
        windowReturnClass.NoMouseClickBelow = true;
      if (x < 583 & y <= 70)
        windowReturnClass.NoMouseClickBelow = true;
      if (x > self.game.ScreenWidth - 735 & y <= 60)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.screenVid > -1 & nr == 86)
      {
        windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.screenVid].X + 1, self.MouseRect[self.screenVid].Y + 1, 1);
        windowReturnClass2.SetFlag(true);
        return windowReturnClass2;
      }
      if (self.screenHis > -1 & nr == 72)
      {
        windowReturnClass3: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.screenHis].X + 1, self.MouseRect[self.screenHis].Y + 1, 1);
        windowReturnClass3.SetFlag(true);
        return windowReturnClass3;
      }
      if (self.screenMan > -1 & nr == 78)
      {
        windowReturnClass4: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.screenMan].X + 1, self.MouseRect[self.screenMan].Y + 1, 1);
        windowReturnClass4.SetFlag(true);
        return windowReturnClass4;
      }
      if (self.screenMap > -1 & nr == 27)
      {
        windowReturnClass5: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.screenMap].X + 1, self.MouseRect[self.screenMap].Y + 1, 1);
        windowReturnClass5.SetFlag(true);
        return windowReturnClass5;
      }
      if (self.game.EditObj.OrderType != 26 && self.screenMap < 0 && nr == 67)
      {
        let mut integer: i32 = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(self.game.SelectX, self.game.SelectY, "SE_Data", "Zones"));
        let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
        if (integer > 0)
        {
          let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, integer, 6)));
          if (self.game.EventRelatedObj.CheckRegimeSlot( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, integer, 8))), 0, 0, 0) == self.game.Data.Turn)
          {
            let mut index: i32 = -1;
            if (id > 0)
              index = self.game.HandyFunctionsObj.GetLocationByID(id);
            let mut num: i32 = -1;
            if (index > -1)
              num = self.game.Data.LocObj[index].HQ;
            if (num > -1)
            {
              self.game.EditObj.UDSpopupText = "";
              self.formref.Cursor = Cursors.WaitCursor;
              self.game.EditObj.UDSClearInput();
              self.game.EventRelatedObj.SetUDSKey("ZONE", integer);
              let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 106, 0, 0);
              self.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
              self.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
              self.formref.Cursor = Cursors.Default;
              self.game.EditObj.PopupValue = 21;
              windowReturnClass1.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
          }
        }
      }
      if (nr == 112 & self.tab1 > -1)
      {
        windowReturnClass6: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab1].X + 1, self.MouseRect[self.tab1].Y + 1, 1);
        windowReturnClass6.SetFlag(true);
        return windowReturnClass6;
      }
      if (nr == 112 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 113 & self.tab3 > -1)
      {
        windowReturnClass7: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab3].X + 1, self.MouseRect[self.tab3].Y + 1, 1);
        windowReturnClass7.SetFlag(true);
        return windowReturnClass7;
      }
      if (nr == 113 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 114 & self.tab4 > -1)
      {
        windowReturnClass8: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab4].X + 1, self.MouseRect[self.tab4].Y + 1, 1);
        windowReturnClass8.SetFlag(true);
        return windowReturnClass8;
      }
      if (nr == 114 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 115 & self.tab7 > -1)
      {
        windowReturnClass9: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab7].X + 1, self.MouseRect[self.tab7].Y + 1, 1);
        windowReturnClass9.SetFlag(true);
        return windowReturnClass9;
      }
      if (nr == 115 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 116 & self.tab6 > -1)
      {
        windowReturnClass10: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab6].X + 1, self.MouseRect[self.tab6].Y + 1, 1);
        windowReturnClass10.SetFlag(true);
        return windowReturnClass10;
      }
      if (nr == 116 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 117 & self.tab11 > -1)
      {
        windowReturnClass11: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab11].X + 1, self.MouseRect[self.tab11].Y + 1, 1);
        windowReturnClass11.SetFlag(true);
        return windowReturnClass11;
      }
      if (nr == 117 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 118 & self.tab12 > -1)
      {
        windowReturnClass12: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab12].X + 1, self.MouseRect[self.tab12].Y + 1, 1);
        windowReturnClass12.SetFlag(true);
        return windowReturnClass12;
      }
      if (nr == 118 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 119 & self.tab8 > -1)
      {
        windowReturnClass13: WindowReturnClass = self.HandleMouseClick(self.MouseRect[self.tab8].X + 1, self.MouseRect[self.tab8].Y + 1, 1);
        windowReturnClass13.SetFlag(true);
        return windowReturnClass13;
      }
      if (nr == 119 & self.game.EditObj.SetViewMode2 > 0)
      {
        self.game.EditObj.SetViewMode2 = 0;
        self.dostuff();
        windowReturnClass1.AddCommand(1, 9);
        windowReturnClass1.AddCommand(7, 12);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 27 & self.game.EditObj.SetViewMode2 > 0 && self.game.EditObj.OrderType != 26)
      {
        self.game.EditObj.SetViewMode2 = 0;
        if (self.game.EditObj.GuiDown | self.game.EditObj.RightDown)
        {
          self.game.EditObj.GuiDown = false;
          self.game.EditObj.RightDown = false;
          self.game.EditObj.SetViewMode2 = 0;
          windowReturnClass1.AddCommand(3, 11);
        }
        else
        {
          self.dostuff();
          windowReturnClass1.AddCommand(1, 9);
          windowReturnClass1.AddCommand(7, 12);
        }
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (self.game.EventRelatedObj.Helper_IsDebug() && nr == 68)
        self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0))].SetData2(0, self.game.Data.RegimeObj[self.game.EditObj.RealTurn].id, 1, Interaction.InputBox("Change which regimekey?"), 2,  Math.Round(Conversion.Val(Interaction.InputBox("What new value?"))));
      return windowReturnClass1;
    }

    pub DoSurrenderStuff: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      self.game.EditObj.UnitSelected = -1;
      self.game.EditObj.OrderUnit = -1;
      self.game.EditObj.OrderTarget = -1;
      self.game.EditObj.OldUnit = -1;
      let mut humanPlayers: i32 = self.game.HandyFunctionsObj.GetHumanPlayers();
      if (!self.game.Data.RegimeObj[self.game.Data.Turn].AI & self.game.Data.RegimeObj[self.game.Data.Turn].Sleep)
        humanPlayers += 1;
      if (humanPlayers != 2 && humanPlayers != 1 && self.game.Data.Product < 7 && self.game.Data.PbemGameID < 1)
        self.game.EventRelatedObj.ExecJoinRegime(self.game.Data.Turn, -1, 0, 0, "");
      if (humanPlayers > 2)
      {
        for (let mut unitCounter: i32 = self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
        {
          if (self.game.Data.UnitObj[unitCounter].Regime == self.game.Data.Turn & self.game.Data.UnitObj[unitCounter].PreDef == -1)
          {
            data: DataClass = self.game.Data;
            let mut nr: i32 = unitCounter;
            let mut gameClass: GameClass = (GameClass) null;
             let mut local: GameClass =  gameClass;
            data.RemoveUnit(nr,  local);
          }
        }
      }
      if ( self.game.Data.RuleVar[978] < 1.0)
      {
        self.game.Data.LastWinner = self.game.Data.Winner;
        if (self.game.Data.PbemGameID < 1)
          self.game.Data.RegimeObj[self.game.Data.Turn].Sleep = true;
      }
      if (humanPlayers > 1 |  self.game.Data.RuleVar[978] > 0.0 | self.game.Data.PbemGameID > 0 | self.game.Data.Product == 7 & humanPlayers > 1)
      {
        windowReturnClass.SetFlag(false);
      }
      else
      {
        self.game.Data = DataClass::new();
        self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
        if (self.game.Data.UseAI == 1)
        {
          if (Information.IsNothing( self.game.NewAIObj))
            self.game.NewAIObj = new NewAIClass(self.game);
          self.game.NewAIObj.LastRegime = -1;
        }
        self.game.EditObj.ShowInitialMenu = true;
        windowReturnClass.AddCommand(3, 12);
        windowReturnClass.SetFlag(true);
      }
      return windowReturnClass;
    }
  }
}
