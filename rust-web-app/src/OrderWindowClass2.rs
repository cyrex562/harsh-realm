// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OrderWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Diagnostics;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class OrderWindowClass2 : WindowClass
  {
     bool TimerUsed;
     w: i32;
     h: i32;
     CurrentView: i32;
     bool EndingTurn;
     bool Surrendering;
     bool RealSurrendering;
     bool JustAMessage;
     bool AskingAboutMetrics;
     Info1Id: i32;
     Cancelid: i32;
     OkId: i32;
     BattleId: i32;
     LeftId: i32;
     RightId: i32;
     KillId: i32;
     AllId: i32;
     NoneId: i32;
     Ok2Id: i32;
     Battle2Id: i32;
     Kill2Id: i32;
     All2Id: i32;
     None2Id: i32;
     NotOkText: String;
     tab1: i32;
     tab2: i32;
     tab3: i32;
     tab4: i32;
     tab51: i32;
     tab52: i32;
     tab53: i32;
     tab6: i32;
     tab101: i32;
     lastorderx: i32;
     lastordery: i32;
     LastAirSupplyNeed: i32;
     LastAirSupplyMax: i32;
     LastAirSupplyTarget: i32;
     MoveButtonId: i32;
     StatisticsButtonId: i32;
     GroupMoveButtonId: i32;
     NextButtonId: i32;
     PopupButtonId: i32;
     NewUnitButtonId: i32;
     HqUnitButtonId: i32;
     NewUnitButton2Id: i32;
     AttackButtonId: i32;
     seaAttackButtonId: i32;
     PrefsButtonId: i32;
     ArtAttackButtonId: i32;
     SeaArtAttackButtonId: i32;
     TransferButtonId: i32;
     AirAttackButtonId: i32;
     InterdictButtonId: i32;
     StrategicButtonId: i32;
     GroupStrategicButtonId: i32;
     MakeHQButtonID: i32;
     AirReconButtonId: i32;
     ParadropButtonId: i32;
     LoadButtonId: i32;
     UnLoadButtonID: i32;
     ResearchId: i32;
     DipId: i32;
     HistoryId: i32;
     SaveId: i32;
     QuitID: i32;
     HqProdButtonId: i32;
     PeopleTransferButtonId: i32;
     ProdButtonId: i32;
     RecruitButtonId: i32;
     SupplyLayerButtonId: i32;
     AirSupplyButtonId: i32;
     OfficerId: i32;
     BlowBridgeButtonId: i32;
     BlowLocationButtonId: i32;
     ACapButtonId: i32;
     InfraButtonId: i32;
     BuildButtonId: i32;
     HexUnitButtonId: i32;
     HexUnitButtonId2: i32;
     GiveUnitId: i32;
     GiveHexId: i32;
     ShowAsId: i32;
     ShowAsId2: i32;
     FakeBackButtonId: i32;
     ChangeModelId: i32;
     ChangeModelId2: i32;
     ModelDesignerId: i32;
     ModelDesignerId2: i32;
     SFDesignButtonId: i32;
     GuiDownId: i32;
     MoveButtonId2: i32;
     GroupMoveButtonId2: i32;
     StatisticsButtonId2: i32;
     NextButtonId2: i32;
     NewUnitButtonId2: i32;
     HqUnitButtonId2: i32;
     AttackButtonId2: i32;
     seaAttackButtonId2: i32;
     PrefsButtonId2: i32;
     NewUnitButton2Id2: i32;
     ArtAttackButtonId2: i32;
     SeaArtAttackButtonId2: i32;
     TransferButtonId2: i32;
     AirAttackButtonId2: i32;
     InterdictButtonId2: i32;
     StrategicButtonId2: i32;
     GroupStrategicButtonId2: i32;
     MakeHQButtonID2: i32;
     AirReconButtonId2: i32;
     ParadropButtonId2: i32;
     LoadButtonId2: i32;
     UnLoadButtonID2: i32;
     ResearchId2: i32;
     DipId2: i32;
     HistoryId2: i32;
     SaveId2: i32;
     QuitID2: i32;
     HqProdButtonId2: i32;
     OfficerId2: i32;
     PeopleTransferButtonId2: i32;
     ProdButtonId2: i32;
     RecruitButtonId2: i32;
     SupplyLayerButtonId2: i32;
     AirSupplyButtonId2: i32;
     OrderSurrenderButtonId: i32;
     ButtonZoomInId: i32;
     ButtonZoomOutId: i32;
     ButtonStackedUnitId: i32;
     OrderSurrenderButtonId2: i32;
     ButtonZoomInId2: i32;
     ButtonZoomOutId2: i32;
     ButtonStackedUnitId2: i32;
     BlowBridgeButtonId2: i32;
     BlowLocationButtonId2: i32;
     GiveUnitId2: i32;
     GiveHexId2: i32;
     ACapButtonId2: i32;
     InfraButtonId2: i32;
     BuildButtonId2: i32;
     TransportButtonId: i32;
     TransportButtonId2: i32;
     BattleGroupButtonId: i32;
     BattleGroupButtonId2: i32;
     SFDesignButtonText: String;
     MoveButtonText: String;
     ButtonZoomInText: String;
     ButtonZoomOutText: String;
     ButtonStackedUnitText: String;
     GroupMoveButtonText: String;
     StatisticsButtonText: String;
     NextButtonText: String;
     GiveUnitText: String;
     GiveHexText: String;
     NewUnitButtonText: String;
     HqUnitButtonText: String;
     AttackButtonText: String;
     ChangeModelText: String;
     OfficerText: String;
     SeaAttackButtonText: String;
     PrefsButtonText: String;
     ArtAttackButtonText: String;
     SeaArtAttackButtonText: String;
     TransferButtonText: String;
     AirAttackButtonText: String;
     AirReconButtonText: String;
     newunitbutton2text: String;
     paradropbuttontext: String;
     loadbuttontext: String;
     unloadbuttontext: String;
     researchbuttontext: String;
     diptext: String;
     constructtext: String;
     historytext: String;
     savetext: String;
     quittext: String;
     hqprodbuttontext: String;
     ordersurrendertext: String;
     battlegroupText: String;
     transportButtonText: String;
     supplylayerbuttontext: String;
     blowlocationtext: String;
     disbandtext: String;
     interdictbuttontext: String;
     prodbuttontext: String;
     researchtext: String;
     groupstrategictext: String;
     strategicbuttontext: String;
     airsupplybuttontext: String;
     blowbridgebuttontext: String;
     infrabuttontext: String;
     buildbuttontext: String;
     int[] ActionButtonId;
     int[] ActionButtonId2;
     int[] ActionButtonCardSlot;
     disbandid: i32;
     disbandid2: i32;
     bool pdfAsked;
     lastHideUnit: i32;

    pub OrderWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect)
      : base( tGame, tGame.ScreenWidth, 90)
    {
      this.ActionButtonId = new int[11];
      this.ActionButtonId2 = new int[11];
      this.ActionButtonCardSlot = new int[11];
      this.pdfAsked = false;
      this.NewGfx = true;
      this.w = tGame.ScreenWidth;
      this.h = 90;
      this.LowerWindow = tLowerWindow;
      this.LowerRect = tLowerRect;
      this.lastorderx = -1;
      this.lastordery = -1;
      this.lastHideUnit = -1;
      if (this.game.SelectX > -1 && this.game.EditObj.UnitSelected == -1 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1 && this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
        this.game.EditObj.UnitSelected = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0];
      this.game.EditObj.PurelyOrderRedrawRefresh = false;
      this.dostuff();
    }

    pub fn DoRefresh()
    {
      if (this.game.EditObj.OrderType == 0)
      {
        this.lastorderx = -1;
        this.lastordery = -1;
      }
      this.dostuff();
    }

    pub handleTimerWheel: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass.Flag = false;
      if (this.game.EditObj.MouseWheel > 0 & this.ButtonZoomInId > 0 & this.game.EditObj.Zoom < 1 & this.game.EditObj.TutOrder == -1)
      {
        if (this.game.EditObj.MouseOverX > -1 & this.game.EditObj.MouseOverY > -1)
          this.game.HandyFunctionsObj.CenterOnXY(this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, true);
        this.game.EditObj.MouseWheel = 0;
        this.game.EditObj.MouseWheelWait = 4;
        return this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonZoomInId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonZoomInId)] + 1, 1);
      }
      if (this.game.EditObj.MouseWheel < 0 & this.ButtonZoomOutId > 0 & this.game.EditObj.Zoom > -1 & this.game.EditObj.TutOrder == -1)
      {
        if (this.game.EditObj.MouseOverX > -1 & this.game.EditObj.MouseOverY > -1)
          this.game.HandyFunctionsObj.CenterOnXY(this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, true);
        this.game.EditObj.MouseWheel = 0;
        this.game.EditObj.MouseWheelWait = 4;
        return this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonZoomOutId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonZoomOutId)] + 1, 1);
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (Information.IsNothing( this.game.EditObj.pdfGenerated))
        this.game.EditObj.pdfGenerated = "";
      if (this.EndingTurn)
        return this.DoEndTurnStuff();
      if (!this.pdfAsked && this.game.EditObj.pdfGenerated.Length > 2)
      {
        str: String = "REPORT GENERATED\r\n" + "A PDF document has been generated. Open it?";
        this.pdfAsked = true;
        this.game.EditObj.PopupValue = 10;
        this.game.EditObj.QuestionText = str;
        this.game.EditObj.AnswerCount = 2;
        this.game.EditObj.AnswerText[1] = "Yes";
        this.game.EditObj.AnswerTextMouseOver[1] = "Try to open the PDF: ''documentation/" + this.game.EditObj.pdfGenerated + "'. ";
        this.game.EditObj.AnswerText[2] = "No";
        this.game.EditObj.AnswerTextMouseOver[2] = "Stay in the game. You can always look up the PDF later";
        this.game.EditObj.warningshown = true;
        windowReturnClass1.AddCommand(5, 10);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (this.pdfAsked & this.game.EditObj.pdfGenerated.Length > 2)
      {
        str: String = "documentation/" + this.game.EditObj.pdfGenerated;
        this.game.EditObj.pdfGenerated = "";
        if (this.game.EditObj.AnswerChosen == 1)
        {
          windowReturnClass2: WindowReturnClass;
          try
          {
            Process.Start(AppDomain.CurrentDomain.BaseDirectory + str);
            this.game.FormRef.SendToBack();
            goto label_10;
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            let mut num: i32 =   Interaction.MsgBox( "PROBLEM", Title: ( "Sadly there was a problem trying to let your Windows system open this PDF. Please check the game forums for possible causes."));
            this.dostuff();
            windowReturnClass2 = windowReturnClass1;
            ProjectData.ClearProjectError();
          }
          return windowReturnClass2;
        }
      }
label_10:
      if (this.Surrendering)
      {
        if ( this.game.Data.RuleVar[950] > 0.0)
        {
          this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[950]));
          this.RealSurrendering = true;
          this.Surrendering = false;
          DrawMod.TGame.EditObj.PopupValue = 0;
          this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
          windowReturnClass1.AddCommand(5, 14);
          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
        this.RealSurrendering = false;
        return this.DoSurrenderStuff();
      }
      if (this.RealSurrendering)
      {
        this.RealSurrendering = false;
        return this.DoSurrenderStuff();
      }
      if (!this.TimerUsed && this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49 && this.game.EditObj.OrderTarget > -1)
      {
        this.game.EditObj.TempCoordList = CoordList::new();
        this.game.EditObj.TargetX = this.game.EditObj.OrderX;
        this.game.EditObj.TargetY = this.game.EditObj.OrderY;
        this.game.SelectX = this.game.EditObj.OrderX;
        this.game.SelectY = this.game.EditObj.OrderY;
        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectX, this.game.EditObj.MapSelected);
        windowReturnClass1.AddCommand(1, 5);
        windowReturnClass1.AddCommand(2, 35);
        windowReturnClass1.AddCommand(4, 12);
        this.TimerUsed = true;
        windowReturnClass1.SetFlag(true);
      }
      return windowReturnClass1;
    }

    pub fn dostuff()
    {
      this.CurrentView = this.game.EditObj.SetViewMode;
      this.ClearMouse();
      this.NewBackGroundAndClearAll(this.w, this.h, -1);
      Graphics g = Graphics.FromImage((Image) this.OwnBitmap);
      if (!this.game.EditObj.useLeftRightClickMode |  this.game.Data.RuleVar[701] < 1.0 | this.game.EditObj.OrderType == 26)
      {
        let mut num1: i32 =   Math.Round( this.game.ScreenWidth / 116.0);
        bitmap: Bitmap;
        for (let mut index: i32 =  0; index <= num1; index += 1)
        {
           let mut local1: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(this.game.MARCBUTBARFRAME);
           let mut local2: &Bitmap = &bitmap;
          let mut x: i32 =  index * 116;
          DrawMod.DrawSimple( local1,  local2, x, 87);
        }
        let mut num2: i32 =  0;
        if (this.game.EditObj.OrderType == 26)
        {
          let mut num3: i32 =   Math.Round( (this.game.ScreenWidth - 1024) / 2.0);
           let mut local3: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(this.game.MARCBUTBARHISTORY);
           let mut local4: &Bitmap = &bitmap;
          let mut x: i32 =  num3;
          let mut y: i32 =  num2;
          DrawMod.DrawSimple( local3,  local4, x, y);
        }
        else
        {
          let mut num4: i32 =   Math.Round( (this.game.ScreenWidth - BitmapStore.GetWidth(this.game.MARCBUTBAR)) / 2.0);
           let mut local5: &Graphics = &g;
          bitmap = BitmapStore.GetBitmap(this.game.MARCBUTBAR);
           let mut local6: &Bitmap = &bitmap;
          let mut x: i32 =  num4;
          let mut y: i32 =  num2;
          DrawMod.DrawSimple( local5,  local6, x, y);
        }
      }
      else
      {
        DrawMod.DrawBlock( g, this.w - 290, 0, 290, 15, 0, 0, 0, 128);
        DrawMod.DrawBlock( g, 0, 15, this.w, 20, 0, 0, 0, 128);
        DrawMod.DrawBlock( g, 0, 33, this.w, this.h - 33, 0, 0, 0,  byte.MaxValue);
        let mut num: i32 =   Math.Round( this.game.ScreenWidth / 116.0);
        for (let mut index: i32 =  0; index <= num; index += 1)
        {
           let mut local7: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBUTBARFRAME);
           let mut local8: &Bitmap = &bitmap;
          let mut x: i32 =  index * 116;
          DrawMod.DrawSimple( local7,  local8, x, 87);
        }
      }
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      if (Information.IsNothing( this.game.EditObj.TempUnitList))
        this.game.EditObj.TempUnitList = UnitList::new();
      this.DoTabs( g);
      if (!this.game.EditObj.PurelyOrderRedrawRefresh)
      {
        this.dostuff2(g);
      }
      else
      {
        this.game.EditObj.PurelyOrderRedrawRefresh = false;
        if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
          this.dostuff2(g);
        this.FlagAll();
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.BattleTimerActive)
        return windowReturnClass1;
      let mut num1: i32 =  0;
      if (this.game.EditObj.TutMode)
        num1 = 1;
      if (this.game.EditObj.OrderType == 1 & this.game.EditObj.OrderUnit > -1 && nr == 71 & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1 & this.GroupMoveButtonId > 0)
      {
        windowReturnClass2: WindowReturnClass;
        if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].StartSize > 1)
        {
          this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
          windowReturnClass2 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass3: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.GroupMoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.GroupMoveButtonId)] + 1, 1);
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass3.SetFlag(true);
          return windowReturnClass3;
        }
        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
        windowReturnClass2 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
        windowReturnClass4: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
        this.game.EditObj.TempCoordList = CoordList::new();
        windowReturnClass4.SetFlag(true);
        return windowReturnClass4;
      }
      if (nr == 121 & this.tab1 > -1)
      {
        windowReturnClass5: WindowReturnClass = this.HandleMouseClick(this.MouseRect[this.tab1].X + 66, this.MouseRect[this.tab1].Y + 1, 1);
        windowReturnClass5.SetFlag(true);
        return windowReturnClass5;
      }
      if (nr == 122 & this.tab2 > -1)
      {
        windowReturnClass6: WindowReturnClass = this.HandleMouseClick(this.MouseRect[this.tab2].X + 66, this.MouseRect[this.tab2].Y + 1, 1);
        windowReturnClass6.SetFlag(true);
        return windowReturnClass6;
      }
      if (nr == 123 & this.tab3 > -1)
      {
        windowReturnClass7: WindowReturnClass = this.HandleMouseClick(this.MouseRect[this.tab3].X + 66, this.MouseRect[this.tab3].Y + 1, 1);
        windowReturnClass7.SetFlag(true);
        return windowReturnClass7;
      }
      if (nr == 120 & this.tab4 > -1)
      {
        windowReturnClass8: WindowReturnClass = this.HandleMouseClick(this.MouseRect[this.tab4].X + 66, this.MouseRect[this.tab4].Y + 1, 1);
        windowReturnClass8.SetFlag(true);
        return windowReturnClass8;
      }
      if ((nr == 97 | nr == 49) & this.HexUnitButtonId > 0 + num1 & this.game.EditObj.TutOrder == -1)
      {
        this.game.EditObj.HideUnit = 0;
        windowReturnClass9: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HexUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HexUnitButtonId)] + 1, 1);
        windowReturnClass9.SetFlag(true);
        return windowReturnClass9;
      }
      if (nr == 34 & !this.game.EditObj.GuiDown & this.GuiDownId > 0 + num1)
      {
        windowReturnClass10: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.GuiDownId)] + 1, this.SubPartY[this.SubpartNr(this.GuiDownId)] + 1, 1);
        windowReturnClass10.SetFlag(true);
        return windowReturnClass10;
      }
      if (nr == 33 & this.game.EditObj.GuiDown & this.GuiDownId > 0)
      {
        windowReturnClass11: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.GuiDownId)] + 1, this.SubPartY[this.SubpartNr(this.GuiDownId)] + 1, 1);
        windowReturnClass11.SetFlag(true);
        return windowReturnClass11;
      }
      if ((nr == 98 | nr == 50) & this.HexUnitButtonId > 0 + num1 &  this.game.Data.RuleVar[344] >  (0 + num1) & this.game.EditObj.TutOrder == -1)
      {
        this.game.EditObj.HideUnit = 1;
        windowReturnClass12: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HexUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HexUnitButtonId)] + 1, 1);
        windowReturnClass12.SetFlag(true);
        return windowReturnClass12;
      }
      if ((nr == 96 | nr == 48) & this.HexUnitButtonId > 0 + num1 & this.game.EditObj.TutOrder == -1)
      {
        let mut num2: i32 =  0;
        if ( this.game.Data.RuleVar[344] >  (0 + num1))
          num2 = 0;
        if (this.lastHideUnit > -1 & this.lastHideUnit != num2)
        {
          this.game.EditObj.HideUnit = this.lastHideUnit;
          this.lastHideUnit = -1;
        }
        else
        {
          this.lastHideUnit = this.game.EditObj.HideUnit;
          if (this.lastHideUnit == 0)
            this.lastHideUnit = 1;
          this.game.EditObj.HideUnit = num2;
        }
        this.game.EditObj.TempCoordList = CoordList::new();
        windowReturnClass1.AddCommand(4, 12);
        windowReturnClass1.AddCommand(4, 9);
        windowReturnClass1.AddCommand(4, 67);
        windowReturnClass1.AddCommand(4, 68);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if ((nr == 99 | nr == 51) & this.ButtonStackedUnitId > 0 + num1 & this.game.EditObj.TutOrder == -1)
      {
        windowReturnClass13: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonStackedUnitId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonStackedUnitId)] + 1, 1);
        windowReturnClass13.SetFlag(true);
        return windowReturnClass13;
      }
      if ((nr == 100 | nr == 52) & this.ShowAsId > 0 + num1 & this.game.EditObj.TutOrder == -1)
      {
        windowReturnClass14: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ShowAsId)] + 1, this.SubPartY[this.SubpartNr(this.ShowAsId)] + 1, 1);
        windowReturnClass14.SetFlag(true);
        return windowReturnClass14;
      }
      if ((nr == 101 | nr == 53) & this.ShowAsId > 0 + num1 & this.game.EditObj.TutOrder == -1)
      {
        this.game.EditObj.ShowHQPower = !this.game.EditObj.ShowHQPower;
        this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
        windowReturnClass1.AddCommand(4, 12);
        windowReturnClass1.AddCommand(4, 67);
        windowReturnClass1.AddCommand(4, 68);
        windowReturnClass1.AddCommand(4, 9);
        return windowReturnClass1;
      }
      if ((nr == 102 | nr == 54) & this.ShowAsId > 0 + num1 & this.game.EditObj.TutOrder == -1)
      {
        this.game.EditObj.ShowAirRange = !this.game.EditObj.ShowAirRange;
        this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
        windowReturnClass1.AddCommand(4, 12);
        windowReturnClass1.AddCommand(4, 67);
        windowReturnClass1.AddCommand(4, 68);
        windowReturnClass1.AddCommand(4, 9);
        return windowReturnClass1;
      }
      if (this.game.EditObj.OrderType == 48 & this.Cancelid > 0 && nr == 77)
      {
        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
        windowReturnClass15: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
        if (this.MoveButtonId > 0 + num1)
          windowReturnClass15 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
        this.game.EditObj.TempCoordList = CoordList::new();
        windowReturnClass15.SetFlag(true);
        return windowReturnClass15;
      }
      if (nr == 67 & this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X > -1)
      {
        this.game.HandyFunctionsObj.CenterOnXY(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y);
        this.game.EditObj.TempCoordList = CoordList::new();
        windowReturnClass1.AddCommand(4, 12);
        windowReturnClass1.AddCommand(4, 9);
        windowReturnClass1.AddCommand(4, 67);
        windowReturnClass1.AddCommand(4, 68);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      let mut num3: i32 =  0;
      if (this.game.EditObj.GuiDown)
        num3 = 222;
      if ((nr == 187 | nr == 191 | nr == 107) & this.game.EditObj.Zoom < 1 & this.game.EditObj.TutOrder == -1)
      {
        if (this.ButtonZoomInId > 0 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonZoomInId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonZoomInId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        else
        {
          let mut num4: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 53.0));
          let mut num5: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 106.0));
          let mut num6: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num3)) / 53.0));
          let mut num7: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num3)) / 106.0));
          num8: i32;
          num9: i32;
          if (this.game.EditObj.Zoom == 0)
          {
            this.game.EditObj.Zoom = 1;
            this.game.CornerX +=  Math.Round(Conversion.Int( num5 / 2.0));
            this.game.CornerY +=  Math.Round(Conversion.Int( num7 / 2.0));
            num8 = 106;
            num9 = 96;
          }
          else
          {
            this.game.EditObj.Zoom = 0;
            this.game.CornerX +=  Math.Round(Conversion.Int( num4 / 2.0));
            this.game.CornerY +=  Math.Round(Conversion.Int( num6 / 2.0));
            num8 = 53;
            num9 = 48;
          }
          if ( this.game.CornerX +  this.game.ScreenWidth /  num8 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
            this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num8);
          if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num3)) /  num9 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
            this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num3)) /  num9);
          if (this.game.CornerX < 0)
            this.game.CornerX = 0;
          if (this.game.CornerY < 0)
            this.game.CornerY = 0;
          this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass1.AddCommand(1, 12);
          windowReturnClass1.AddCommand(2, 12);
          windowReturnClass1.AddCommand(4, 9);
          windowReturnClass1.AddCommand(4, 67);
          windowReturnClass1.AddCommand(4, 68);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      if ((nr == 189 | nr == 219 | nr == 109) & this.game.EditObj.Zoom > -1 & this.game.EditObj.TutOrder == -1)
      {
        if (this.ButtonZoomOutId > 0 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ButtonZoomOutId)] + 1, this.SubPartY[this.SubpartNr(this.ButtonZoomOutId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        else
        {
          let mut num10: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 53.0));
          let mut num11: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 106.0));
          let mut num12: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num3)) / 53.0));
          let mut num13: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num3)) / 106.0));
          num14: i32;
          num15: i32;
          if (this.game.EditObj.Zoom == 0)
          {
            this.game.EditObj.Zoom = -1;
            this.game.CornerX -=  Math.Round(Conversion.Int( num10 / 2.0));
            this.game.CornerY -=  Math.Round(Conversion.Int( num12 / 2.0));
            num14 = 27;
            num15 = 24;
          }
          else
          {
            this.game.EditObj.Zoom = 0;
            this.game.CornerX -=  Math.Round(Conversion.Int( num11 / 2.0));
            this.game.CornerY -=  Math.Round(Conversion.Int( num13 / 2.0));
            num14 = 53;
            num15 = 48;
          }
          if ( this.game.CornerX +  this.game.ScreenWidth /  num14 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
            this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num14);
          if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num3)) /  num15 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
            this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num3)) /  num15);
          if (this.game.CornerX < 0)
            this.game.CornerX = 0;
          if (this.game.CornerY < 0)
            this.game.CornerY = 0;
          this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass1.AddCommand(1, 12);
          windowReturnClass1.AddCommand(2, 12);
          windowReturnClass1.AddCommand(4, 9);
          windowReturnClass1.AddCommand(4, 67);
          windowReturnClass1.AddCommand(4, 68);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      if (this.game.EditObj.OrderType == 0)
      {
        if (nr == 190 & this.game.EditObj.UnitSelected > -1)
        {
          this.game.HandyFunctionsObj.NextUnit();
          this.game.HandyFunctionsObj.CenterOnXY(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y);
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass1.AddCommand(4, 12);
          windowReturnClass1.AddCommand(4, 9);
          windowReturnClass1.AddCommand(4, 67);
          windowReturnClass1.AddCommand(4, 68);
          windowReturnClass1.AddCommand(4, 69);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
        if (nr == 188 & this.game.EditObj.UnitSelected > -1)
        {
          this.game.HandyFunctionsObj.NextUnit2();
          this.game.HandyFunctionsObj.CenterOnXY(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y);
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass1.AddCommand(4, 12);
          windowReturnClass1.AddCommand(4, 9);
          windowReturnClass1.AddCommand(4, 67);
          windowReturnClass1.AddCommand(4, 68);
          windowReturnClass1.AddCommand(4, 69);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
        if (nr == 77 & this.MoveButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        else if (nr == 77 &  this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
        {
          let mut mouseCounter: i32 =  this.MouseCounter;
          for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
          {
            if (this.MouseData[index] == 201)
            {
              windowReturnClass16: WindowReturnClass = this.HandleMouseClick(this.MouseRect[index].X + 1, this.MouseRect[index].Y + 1, 1);
              windowReturnClass16.SetFlag(true);
              return windowReturnClass16;
            }
          }
        }
        if (nr == 71)
        {
          if (this.GroupMoveButtonId > 1 + num1)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.GroupMoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.GroupMoveButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          else if (this.MoveButtonId > 1 + num1)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.MoveButtonId)] + 1, this.SubPartY[this.SubpartNr(this.MoveButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
          else if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
          {
            let mut mouseCounter: i32 =  this.MouseCounter;
            for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
            {
              if (this.MouseData[index] == 202)
              {
                windowReturnClass17: WindowReturnClass = this.HandleMouseClick(this.MouseRect[index].X + 1, this.MouseRect[index].Y + 1, 1);
                windowReturnClass17.SetFlag(true);
                return windowReturnClass17;
              }
            }
          }
        }
        if (nr == 83 & this.StrategicButtonId > 1 + num1 &  this.game.Data.RuleVar[520] == 0.0)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.StrategicButtonId)] + 1, this.SubPartY[this.SubpartNr(this.StrategicButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 72 & this.HqUnitButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HqUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HqUnitButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 78 & this.NewUnitButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.NewUnitButtonId)] + 1, this.SubPartY[this.SubpartNr(this.NewUnitButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 79 & this.HqProdButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.HqProdButtonId)] + 1, this.SubPartY[this.SubpartNr(this.HqProdButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 80 & this.ProdButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ProdButtonId)] + 1, this.SubPartY[this.SubpartNr(this.ProdButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (this.game.SelectX > -1)
        {
          if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
          {
            if (nr == 65 & this.AttackButtonId > 1 + num1 && this.SubpartNr(this.AttackButtonId) > -1)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.AttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.AttackButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
            if (nr == 66 & this.ArtAttackButtonId > 1 + num1)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ArtAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.ArtAttackButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
          }
          else
          {
            if (nr == 65 & this.seaAttackButtonId > 1 + num1)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.seaAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.seaAttackButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
            if (nr == 66 & this.ArtAttackButtonId > 1 + num1)
            {
              windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.ArtAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.ArtAttackButtonId)] + 1, 1);
              windowReturnClass1.SetFlag(true);
            }
          }
          if (nr == 90 & this.AirAttackButtonId > 1 + num1)
          {
            windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.AirAttackButtonId)] + 1, this.SubPartY[this.SubpartNr(this.AirAttackButtonId)] + 1, 1);
            windowReturnClass1.SetFlag(true);
          }
        }
        if (nr == 76 & this.LoadButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.LoadButtonId)] + 1, this.SubPartY[this.SubpartNr(this.LoadButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 85 & this.UnLoadButtonID > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.UnLoadButtonID)] + 1, this.SubPartY[this.SubpartNr(this.UnLoadButtonID)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 82 & this.InfraButtonId > 1 + num1)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.InfraButtonId)] + 1, this.SubPartY[this.SubpartNr(this.InfraButtonId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 27 & this.game.EditObj.LayerSupplyOn)
        {
          windowReturnClass1.AddCommand(4, 12);
          windowReturnClass1.AddCommand(4, 9);
          windowReturnClass1.AddCommand(4, 67);
          windowReturnClass1.AddCommand(4, 68);
          this.game.EditObj.LayerSupplyOn = false;
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      if (this.game.EditObj.OrderType > 0)
      {
        if (nr == 77 &  this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
        {
          let mut mouseCounter: i32 =  this.MouseCounter;
          for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
          {
            if (this.MouseData[index] == 201)
            {
              windowReturnClass18: WindowReturnClass = this.HandleMouseClick(this.MouseRect[index].X + 1, this.MouseRect[index].Y + 1, 1);
              windowReturnClass18.SetFlag(true);
              return windowReturnClass18;
            }
          }
        }
        if (nr == 71 &  this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
        {
          let mut mouseCounter: i32 =  this.MouseCounter;
          for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
          {
            if (this.MouseData[index] == 202)
            {
              windowReturnClass19: WindowReturnClass = this.HandleMouseClick(this.MouseRect[index].X + 1, this.MouseRect[index].Y + 1, 1);
              windowReturnClass19.SetFlag(true);
              return windowReturnClass19;
            }
          }
        }
      }
      if (this.game.EditObj.OrderType > 0 & this.Cancelid > 0 + num1)
      {
        if (nr == 27)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
      }
      else if (this.game.EditObj.OrderType > 0 & (nr == 27 | nr == 32) &  this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode & this.game.EditObj.udsUnitOrderMode > 0)
      {
        let mut mouseCounter: i32 =  this.MouseCounter;
        for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
        {
          if (this.MouseData[index] == 203)
          {
            windowReturnClass20: WindowReturnClass = this.HandleMouseClick(this.MouseRect[index].X + 1, this.MouseRect[index].Y + 1, 1);
            windowReturnClass20.SetFlag(true);
            return windowReturnClass20;
          }
        }
      }
      if (this.game.EditObj.OrderType > 0 & this.OkId > 0 + num1 && nr == 32)
      {
        windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.OkId)] + 1, this.SubPartY[this.SubpartNr(this.OkId)] + 1, 1);
        windowReturnClass1.SetFlag(true);
      }
      if (this.game.EditObj.OrderType > 0 & this.OkId == 0 & this.BattleId > 0 + num1)
      {
        if (this.game.SelectX == this.game.EditObj.TargetX & this.game.SelectY == this.game.EditObj.TargetY && nr == 32)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.BattleId)] + 1, this.SubPartY[this.SubpartNr(this.BattleId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
        if (nr == 65)
        {
          windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.BattleId)] + 1, this.SubPartY[this.SubpartNr(this.BattleId)] + 1, 1);
          windowReturnClass1.SetFlag(true);
        }
      }
      if (this.game.EditObj.OrderType == 1 & this.Cancelid > 0 && nr == 32)
      {
        windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
        windowReturnClass1.SetFlag(true);
      }
      if (this.game.EditObj.OrderType == 48 & this.Cancelid > 0 && nr == 32)
      {
        windowReturnClass1 = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
        windowReturnClass1.SetFlag(true);
      }
      return windowReturnClass1;
    }

    pub fn dostuff2(Graphics g)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num1: i32 =  31;
      let mut num2: i32 =   Math.Round( (this.game.ScreenWidth - 1024) / 2.0);
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
      {
        num2 =  Math.Round( (this.game.ScreenWidth - 1024) / 2.0 - 128.0) + 240;
        num1 += 16;
      }
      if (this.Info1Id > 0)
        this.RemoveSubPart(this.Info1Id);
      if (this.Cancelid > 0)
      {
        this.RemoveSubPart(this.Cancelid);
        this.Cancelid = 0;
      }
      if (this.OkId > 0)
      {
        this.RemoveSubPart(this.OkId);
        this.OkId = 0;
      }
      if (this.KillId > 0)
      {
        this.RemoveSubPart(this.KillId);
        this.KillId = 0;
      }
      if (this.Ok2Id > 0)
      {
        this.RemoveSubPart(this.Ok2Id);
        this.Ok2Id = 0;
      }
      if (this.Kill2Id > 0)
      {
        this.RemoveSubPart(this.Kill2Id);
        this.Kill2Id = 0;
      }
      if (this.GuiDownId > 0)
      {
        this.RemoveSubPart(this.GuiDownId);
        this.GuiDownId = 0;
      }
      if (this.PopupButtonId > 0)
      {
        this.RemoveSubPart(this.PopupButtonId);
        this.PopupButtonId = 0;
      }
      if (this.SupplyLayerButtonId > 0)
      {
        this.RemoveSubPart(this.SupplyLayerButtonId);
        this.SupplyLayerButtonId = 0;
      }
      if (this.BlowBridgeButtonId > 0)
      {
        this.RemoveSubPart(this.BlowBridgeButtonId);
        this.BlowBridgeButtonId = 0;
      }
      if (this.TransportButtonId > 0)
      {
        this.RemoveSubPart(this.TransportButtonId);
        this.TransportButtonId = 0;
      }
      if (this.TransportButtonId2 > 0)
      {
        this.RemoveSubPart(this.TransportButtonId2);
        this.TransportButtonId2 = 0;
      }
      if (this.BattleGroupButtonId > 0)
      {
        this.RemoveSubPart(this.BattleGroupButtonId);
        this.BattleGroupButtonId = 0;
      }
      if (this.BattleGroupButtonId2 > 0)
      {
        this.RemoveSubPart(this.BattleGroupButtonId2);
        this.BattleGroupButtonId2 = 0;
      }
      if (this.InfraButtonId > 0)
      {
        this.RemoveSubPart(this.InfraButtonId);
        this.InfraButtonId = 0;
      }
      if (this.BuildButtonId > 0)
      {
        this.RemoveSubPart(this.BuildButtonId);
        this.BuildButtonId = 0;
      }
      if (this.AirReconButtonId > 0)
      {
        this.RemoveSubPart(this.AirReconButtonId);
        this.AirReconButtonId = 0;
      }
      if (this.MoveButtonId > 0)
      {
        this.RemoveSubPart(this.MoveButtonId);
        this.MoveButtonId = 0;
      }
      if (this.GroupMoveButtonId > 0)
      {
        this.RemoveSubPart(this.GroupMoveButtonId);
        this.GroupMoveButtonId = 0;
      }
      if (this.NextButtonId > 0)
      {
        this.RemoveSubPart(this.NextButtonId);
        this.NextButtonId = 0;
      }
      if (this.HqUnitButtonId > 0)
      {
        this.RemoveSubPart(this.HqUnitButtonId);
        this.HqUnitButtonId = 0;
      }
      if (this.NewUnitButtonId > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId);
        this.NewUnitButtonId = 0;
      }
      if (this.NewUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId2);
        this.NewUnitButtonId2 = 0;
      }
      if (this.NewUnitButton2Id2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id2);
        this.NewUnitButton2Id2 = 0;
      }
      if (this.NewUnitButton2Id > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id);
        this.NewUnitButton2Id = 0;
      }
      if (this.TransferButtonId > 0)
      {
        this.RemoveSubPart(this.TransferButtonId);
        this.TransferButtonId = 0;
      }
      if (this.AttackButtonId > 0)
      {
        this.RemoveSubPart(this.AttackButtonId);
        this.AttackButtonId = 0;
      }
      if (this.seaAttackButtonId > 0)
      {
        this.RemoveSubPart(this.seaAttackButtonId);
        this.seaAttackButtonId = 0;
      }
      if (this.ArtAttackButtonId > 0)
      {
        this.RemoveSubPart(this.ArtAttackButtonId);
        this.ArtAttackButtonId = 0;
      }
      if (this.SeaArtAttackButtonId > 0)
      {
        this.RemoveSubPart(this.SeaArtAttackButtonId);
        this.SeaArtAttackButtonId = 0;
      }
      if (this.AirAttackButtonId > 0)
      {
        this.RemoveSubPart(this.AirAttackButtonId);
        this.AirAttackButtonId = 0;
      }
      if (this.InterdictButtonId > 0)
      {
        this.RemoveSubPart(this.InterdictButtonId);
        this.InterdictButtonId = 0;
      }
      if (this.StrategicButtonId > 0)
      {
        this.RemoveSubPart(this.StrategicButtonId);
        this.StrategicButtonId = 0;
      }
      if (this.GroupStrategicButtonId > 0)
      {
        this.RemoveSubPart(this.GroupStrategicButtonId);
        this.GroupStrategicButtonId = 0;
      }
      if (this.ParadropButtonId > 0)
      {
        this.RemoveSubPart(this.ParadropButtonId);
        this.ParadropButtonId = 0;
      }
      if (this.LoadButtonId > 0)
      {
        this.RemoveSubPart(this.LoadButtonId);
        this.LoadButtonId = 0;
      }
      if (this.UnLoadButtonID > 0)
      {
        this.RemoveSubPart(this.UnLoadButtonID);
        this.UnLoadButtonID = 0;
      }
      if (this.ResearchId > 0)
      {
        this.RemoveSubPart(this.ResearchId);
        this.ResearchId = 0;
      }
      if (this.DipId > 0)
      {
        this.RemoveSubPart(this.DipId);
        this.DipId = 0;
      }
      if (this.HistoryId > 0)
      {
        this.RemoveSubPart(this.HistoryId);
        this.HistoryId = 0;
      }
      if (this.SaveId > 0)
      {
        this.RemoveSubPart(this.SaveId);
        this.SaveId = 0;
      }
      if (this.QuitID > 0)
      {
        this.RemoveSubPart(this.QuitID);
        this.QuitID = 0;
      }
      if (this.HqProdButtonId > 0)
      {
        this.RemoveSubPart(this.HqProdButtonId);
        this.HqProdButtonId = 0;
      }
      if (this.ProdButtonId > 0)
      {
        this.RemoveSubPart(this.ProdButtonId);
        this.ProdButtonId = 0;
      }
      if (this.disbandid > 0)
      {
        this.RemoveSubPart(this.disbandid);
        this.disbandid = 0;
      }
      if (this.PrefsButtonId > 0)
      {
        this.RemoveSubPart(this.PrefsButtonId);
        this.PrefsButtonId = 0;
      }
      if (this.AirSupplyButtonId > 0)
      {
        this.RemoveSubPart(this.AirSupplyButtonId);
        this.AirSupplyButtonId = 0;
      }
      if (this.StatisticsButtonId > 0)
      {
        this.RemoveSubPart(this.StatisticsButtonId);
        this.StatisticsButtonId = 0;
      }
      if (this.BlowLocationButtonId > 0)
      {
        this.RemoveSubPart(this.BlowLocationButtonId);
        this.BlowLocationButtonId = 0;
      }
      if (this.OrderSurrenderButtonId > 0)
      {
        this.RemoveSubPart(this.OrderSurrenderButtonId);
        this.OrderSurrenderButtonId = 0;
      }
      if (this.HexUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.HexUnitButtonId2);
        this.HexUnitButtonId2 = 0;
      }
      if (this.HexUnitButtonId > 0)
      {
        this.RemoveSubPart(this.HexUnitButtonId);
        this.HexUnitButtonId = 0;
      }
      if (this.ShowAsId2 > 0)
      {
        this.RemoveSubPart(this.ShowAsId2);
        this.ShowAsId2 = 0;
      }
      if (this.ShowAsId > 0)
      {
        this.RemoveSubPart(this.ShowAsId);
        this.ShowAsId = 0;
      }
      if (this.NewUnitButton2Id > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id);
        this.NewUnitButton2Id = 0;
      }
      if (this.NewUnitButton2Id2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id2);
        this.NewUnitButton2Id2 = 0;
      }
      if (this.GiveUnitId > 0)
      {
        this.RemoveSubPart(this.GiveUnitId);
        this.GiveUnitId = 0;
      }
      if (this.GiveHexId > 0)
      {
        this.RemoveSubPart(this.GiveHexId);
        this.GiveHexId = 0;
      }
      if (this.SFDesignButtonId > 0)
      {
        this.RemoveSubPart(this.SFDesignButtonId);
        this.SFDesignButtonId = 0;
      }
      if (this.OfficerId > 0)
      {
        this.RemoveSubPart(this.OfficerId);
        this.OfficerId = 0;
      }
      if (this.OfficerId2 > 0)
      {
        this.RemoveSubPart(this.OfficerId2);
        this.OfficerId2 = 0;
      }
      if (this.ButtonZoomInId > 0)
      {
        this.RemoveSubPart(this.ButtonZoomInId);
        this.ButtonZoomInId = 0;
      }
      if (this.ButtonZoomInId2 > 0)
      {
        this.RemoveSubPart(this.ButtonZoomInId2);
        this.ButtonZoomInId2 = 0;
      }
      if (this.ButtonZoomOutId > 0)
      {
        this.RemoveSubPart(this.ButtonZoomOutId);
        this.ButtonZoomOutId = 0;
      }
      if (this.ButtonZoomOutId2 > 0)
      {
        this.RemoveSubPart(this.ButtonZoomOutId2);
        this.ButtonZoomOutId2 = 0;
      }
      if (this.ButtonStackedUnitId > 0)
      {
        this.RemoveSubPart(this.ButtonStackedUnitId);
        this.ButtonStackedUnitId = 0;
      }
      if (this.ButtonStackedUnitId2 > 0)
      {
        this.RemoveSubPart(this.ButtonStackedUnitId2);
        this.ButtonStackedUnitId2 = 0;
      }
      if (this.SupplyLayerButtonId2 > 0)
      {
        this.RemoveSubPart(this.SupplyLayerButtonId2);
        this.SupplyLayerButtonId2 = 0;
      }
      if (this.BlowBridgeButtonId2 > 0)
      {
        this.RemoveSubPart(this.BlowBridgeButtonId2);
        this.BlowBridgeButtonId2 = 0;
      }
      if (this.InfraButtonId2 > 0)
      {
        this.RemoveSubPart(this.InfraButtonId2);
        this.InfraButtonId2 = 0;
      }
      if (this.BuildButtonId2 > 0)
      {
        this.RemoveSubPart(this.BuildButtonId2);
        this.BuildButtonId2 = 0;
      }
      if (this.AirReconButtonId2 > 0)
      {
        this.RemoveSubPart(this.AirReconButtonId2);
        this.AirReconButtonId2 = 0;
      }
      if (this.MoveButtonId2 > 0)
      {
        this.RemoveSubPart(this.MoveButtonId2);
        this.MoveButtonId2 = 0;
      }
      if (this.GroupMoveButtonId2 > 0)
      {
        this.RemoveSubPart(this.GroupMoveButtonId2);
        this.GroupMoveButtonId2 = 0;
      }
      if (this.NextButtonId2 > 0)
      {
        this.RemoveSubPart(this.NextButtonId2);
        this.NextButtonId2 = 0;
      }
      if (this.HqUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.HqUnitButtonId2);
        this.HqUnitButtonId2 = 0;
      }
      if (this.NewUnitButtonId > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId2);
        this.NewUnitButtonId2 = 0;
      }
      if (this.NewUnitButtonId2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId2);
        this.NewUnitButtonId2 = 0;
      }
      if (this.TransferButtonId2 > 0)
      {
        this.RemoveSubPart(this.TransferButtonId2);
        this.TransferButtonId2 = 0;
      }
      if (this.AttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.AttackButtonId2);
        this.AttackButtonId2 = 0;
      }
      if (this.seaAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.seaAttackButtonId2);
        this.seaAttackButtonId2 = 0;
      }
      if (this.ArtAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.ArtAttackButtonId2);
        this.ArtAttackButtonId2 = 0;
      }
      if (this.SeaArtAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.SeaArtAttackButtonId2);
        this.SeaArtAttackButtonId2 = 0;
      }
      if (this.AirAttackButtonId2 > 0)
      {
        this.RemoveSubPart(this.AirAttackButtonId2);
        this.AirAttackButtonId2 = 0;
      }
      if (this.InterdictButtonId2 > 0)
      {
        this.RemoveSubPart(this.InterdictButtonId2);
        this.InterdictButtonId2 = 0;
      }
      if (this.StrategicButtonId2 > 0)
      {
        this.RemoveSubPart(this.StrategicButtonId2);
        this.StrategicButtonId2 = 0;
      }
      if (this.GroupStrategicButtonId2 > 0)
      {
        this.RemoveSubPart(this.GroupStrategicButtonId2);
        this.GroupStrategicButtonId2 = 0;
      }
      if (this.ParadropButtonId2 > 0)
      {
        this.RemoveSubPart(this.ParadropButtonId2);
        this.ParadropButtonId2 = 0;
      }
      if (this.LoadButtonId2 > 0)
      {
        this.RemoveSubPart(this.LoadButtonId2);
        this.LoadButtonId2 = 0;
      }
      if (this.UnLoadButtonID2 > 0)
      {
        this.RemoveSubPart(this.UnLoadButtonID2);
        this.UnLoadButtonID2 = 0;
      }
      if (this.ResearchId2 > 0)
      {
        this.RemoveSubPart(this.ResearchId2);
        this.ResearchId2 = 0;
      }
      if (this.DipId2 > 0)
      {
        this.RemoveSubPart(this.DipId2);
        this.DipId2 = 0;
      }
      if (this.HistoryId2 > 0)
      {
        this.RemoveSubPart(this.HistoryId2);
        this.HistoryId2 = 0;
      }
      if (this.SaveId2 > 0)
      {
        this.RemoveSubPart(this.SaveId2);
        this.SaveId2 = 0;
      }
      if (this.QuitID2 > 0)
      {
        this.RemoveSubPart(this.QuitID2);
        this.QuitID2 = 0;
      }
      if (this.HqProdButtonId2 > 0)
      {
        this.RemoveSubPart(this.HqProdButtonId2);
        this.HqProdButtonId2 = 0;
      }
      if (this.ProdButtonId2 > 0)
      {
        this.RemoveSubPart(this.ProdButtonId2);
        this.ProdButtonId2 = 0;
      }
      if (this.disbandid2 > 0)
      {
        this.RemoveSubPart(this.disbandid2);
        this.disbandid2 = 0;
      }
      if (this.PrefsButtonId2 > 0)
      {
        this.RemoveSubPart(this.PrefsButtonId2);
        this.PrefsButtonId2 = 0;
      }
      if (this.AirSupplyButtonId2 > 0)
      {
        this.RemoveSubPart(this.AirSupplyButtonId2);
        this.AirSupplyButtonId2 = 0;
      }
      if (this.StatisticsButtonId2 > 0)
      {
        this.RemoveSubPart(this.StatisticsButtonId2);
        this.StatisticsButtonId2 = 0;
      }
      if (this.BlowLocationButtonId2 > 0)
      {
        this.RemoveSubPart(this.BlowLocationButtonId2);
        this.BlowLocationButtonId2 = 0;
      }
      if (this.OrderSurrenderButtonId2 > 0)
      {
        this.RemoveSubPart(this.OrderSurrenderButtonId2);
        this.OrderSurrenderButtonId2 = 0;
      }
      if (this.GiveUnitId2 > 0)
      {
        this.RemoveSubPart(this.GiveUnitId2);
        this.GiveUnitId2 = 0;
      }
      if (this.GiveHexId2 > 0)
      {
        this.RemoveSubPart(this.GiveHexId2);
        this.GiveHexId2 = 0;
      }
      if (this.ChangeModelId > 0)
      {
        this.RemoveSubPart(this.ChangeModelId);
        this.ChangeModelId = 0;
      }
      if (this.ChangeModelId2 > 0)
      {
        this.RemoveSubPart(this.ChangeModelId2);
        this.ChangeModelId = 0;
      }
      if (this.ModelDesignerId > 0)
      {
        this.RemoveSubPart(this.ModelDesignerId);
        this.ChangeModelId = 0;
      }
      if (this.NewUnitButtonId > 0)
      {
        this.RemoveSubPart(this.NewUnitButtonId);
        this.NewUnitButtonId = 0;
      }
      if (this.NewUnitButton2Id > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id);
        this.NewUnitButton2Id = 0;
      }
      if (this.NewUnitButton2Id2 > 0)
      {
        this.RemoveSubPart(this.NewUnitButton2Id2);
        this.NewUnitButton2Id2 = 0;
      }
      let mut index1: i32 =  0;
      do
      {
        if (this.ActionButtonId[index1] > 0)
        {
          this.RemoveSubPart(this.ActionButtonId[index1]);
          this.ActionButtonId[index1] = 0;
        }
        if (this.ActionButtonId2[index1] > 0)
        {
          this.RemoveSubPart(this.ActionButtonId2[index1]);
          this.ActionButtonId2[index1] = 0;
        }
        index1 += 1;
      }
      while (index1 <= 10);
      if (this.FakeBackButtonId > 0)
      {
        this.RemoveSubPart(this.FakeBackButtonId);
        this.FakeBackButtonId = 0;
      }
      if (this.BattleId > 0)
      {
        this.RemoveSubPart(this.BattleId);
        this.BattleId = 0;
      }
      if (this.LeftId > 0)
      {
        this.RemoveSubPart(this.LeftId);
        this.LeftId = 0;
      }
      if (this.RightId > 0)
      {
        this.RemoveSubPart(this.RightId);
        this.RightId = 0;
      }
      if (this.AllId > 0)
      {
        this.RemoveSubPart(this.AllId);
        this.AllId = 0;
      }
      if (this.NoneId > 0)
      {
        this.RemoveSubPart(this.NoneId);
        this.NoneId = 0;
      }
      if (this.BattleId > 0)
      {
        this.RemoveSubPart(this.BattleId);
        this.BattleId = 0;
      }
      if (this.All2Id > 0)
      {
        this.RemoveSubPart(this.All2Id);
        this.All2Id = 0;
      }
      if (this.None2Id > 0)
      {
        this.RemoveSubPart(this.None2Id);
        this.None2Id = 0;
      }
      if (this.Battle2Id > 0)
      {
        this.RemoveSubPart(this.Battle2Id);
        this.Battle2Id = 0;
      }
      if (Information.IsNothing( this.game.EditObj.TempUnitList))
        this.game.EditObj.TempUnitList = UnitList::new();
      if ( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode)
        this.game.EditObj.udsUnitOrderMode = 0;
      Rectangle rectangle;
      Rectangle trect1;
      str1: String;
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
      {
        let mut width: i32 =  75;
        let mut num3: i32 =  num2 - 200 + 12;
        let mut height: i32 =  70;
        let mut x: i32 =  num3;
        if (this.game.EditObj.udsUnitOrderMode == 1)
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBIGBOTTOMTAB);
           let mut local2: &Bitmap = &bitmap;
          Rectangle srcrect = Rectangle::new(0, 0, 75, height - 3);
          rectangle = Rectangle::new(x, 20, 75, height - 3);
          let mut destrect: &Rectangle = &rectangle
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          rectangle = Rectangle::new(x, 20 + (70 - height), width, height);
          trect1 = rectangle;
          this.AddMouse( trect1, "", "Currently in Move Mode");
        }
        else
        {
          height = 40;
           let mut local3: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBIGBOTTOMTAB);
           let mut local4: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 75, height - 3);
          let mut srcrect: &Rectangle = &rectangle
          Rectangle destrect = Rectangle::new(x, 50, 75, height - 3);
          DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
          rectangle = Rectangle::new(x, 20 + (70 - height), width, height);
          trect1 = rectangle;
          this.AddMouse( trect1, "", "Switch to Move Mode [M]", 201);
        }
        str1 = "MOVE";
        SizeF sizeF2 = g.MeasureString(str1, this.game.MarcFont16);
        DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont16,  Math.Round( x +  width / 2.0 -  sizeF2.Width / 2.0 - 2.0), 37 +  Math.Round( (90 - height) / 2.0), Color.White);
      }
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
      {
        let mut width: i32 =  75;
        let mut num4: i32 =  num2 - 200 + 12;
        let mut height: i32 =  70;
        let mut x: i32 =  num4 + 75;
        if (this.game.EditObj.udsUnitOrderMode == 48)
        {
           let mut local5: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBIGBOTTOMTAB);
           let mut local6: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 75, height - 3);
          let mut srcrect: &Rectangle = &rectangle
          trect1 = Rectangle::new(x, 20, 75, height - 3);
          let mut destrect: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
          rectangle = Rectangle::new(x, 20 + (70 - height), width, height);
          trect1 = rectangle;
          this.AddMouse( trect1, "", "Currently in Group Move Mode", 202);
        }
        else
        {
          height = 40;
           let mut local7: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBIGBOTTOMTAB);
           let mut local8: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 75, height - 3);
          let mut srcrect: &Rectangle = &rectangle
          trect1 = Rectangle::new(x, 50, 75, height - 3);
          let mut destrect: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
          rectangle = Rectangle::new(x, 20 + (70 - height), width, height);
          trect1 = rectangle;
          this.AddMouse( trect1, "", "Switch to Group Move Mode [G]", 202);
        }
        str1 = "GROUP";
        SizeF sizeF3 = g.MeasureString(str1, this.game.MarcFont16);
        DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont16,  Math.Round( x +  width / 2.0 -  sizeF3.Width / 2.0 - 2.0), 37 +  Math.Round( (90 - height) / 2.0), Color.White);
      }
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
      {
        let mut width: i32 =  75;
        let mut num5: i32 =  num2 - 200 + 12;
        let mut height: i32 =  70;
        let mut x: i32 =  num5 + 150;
        if (this.game.EditObj.udsUnitOrderMode < 1)
        {
           let mut local9: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBIGBOTTOMTAB);
           let mut local10: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 75, height - 3);
          let mut srcrect: &Rectangle = &rectangle
          trect1 = Rectangle::new(x, 20, 75, height - 3);
          let mut destrect: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
          rectangle = Rectangle::new(x, 20 + (70 - height), width, height);
          let mut trect2: &Rectangle = &rectangle
          this.AddMouse( trect2, "", "Currently in Order Mode", 203);
        }
        else
        {
          height = 40;
           let mut local11: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBIGBOTTOMTAB);
           let mut local12: &Bitmap = &bitmap;
          rectangle = Rectangle::new(0, 0, 75, height - 3);
          let mut srcrect: &Rectangle = &rectangle
          trect1 = Rectangle::new(x, 50, 75, height - 3);
          let mut destrect: &Rectangle = &trect1
          DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
          rectangle = Rectangle::new(x, 20 + (70 - height), width, height);
          let mut trect3: &Rectangle = &rectangle
          this.AddMouse( trect3, "", "Switch to Order Mode [Space/Escape]", 203);
        }
        str1 = "ORDER";
        SizeF sizeF4 = g.MeasureString(str1, this.game.MarcFont16);
        DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont16,  Math.Round( x +  width / 2.0 -  sizeF4.Width / 2.0 - 2.0), 37 +  Math.Round( (90 - height) / 2.0), Color.White);
      }
      SubPartClass tsubpart;
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode & this.game.EditObj.OrderType == 0 & this.game.EditObj.udsUnitOrderMode == 0)
      {
        this.GuiDownId = 1;
        if (this.game.Data.Round == 0)
          this.GuiDownId = 0;
        let mut num6: i32 =  5;
        if (this.GuiDownId > 0)
        {
          if (!this.game.EditObj.GuiDown)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONGUIDOWN, tDescript: "Move Bottom GUI down & Get Larger Map [PgDown]", tBackbitmap: ( this.OwnBitmap), bbx: num6, bby: num1, totherback: 1, tsize: 29);
            this.GuiDownId = this.AddSubPart( tsubpart, num6, num1, 29, 29, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONGUIUP, tDescript: "Move Bottm GUI up & Get unit info", tBackbitmap: ( this.OwnBitmap), bbx: num6, bby: num1, totherback: 1, tsize: 29);
            this.GuiDownId = this.AddSubPart( tsubpart, num6, num1, 29, 29, 1);
          }
        }
      }
      if ( this.game.Data.RuleVar[419] > 0.0 & this.game.Data.Product >= 6 & this.game.EditObj.MouseOverX > -1 & !Information.IsNothing( this.game.EditObj.airRangeTempLos))
      {
        if (this.game.EditObj.UnitSelected > -1)
        {
          let mut num7: i32 =  this.game.EditObj.airRangeTempLos.Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY];
          let mut num8: i32 =  this.game.EditObj.airRangeMaxObstruct.Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY];
          let mut num9: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0, this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, 0, 19);
          float num10 = this.game.Data.RuleVar[420];
          let mut num11: i32 =  num9;
          for (let mut index2: i32 =  2; index2 <= num11; index2 += 1)
          {
            if ( index2 >  this.game.Data.RuleVar[422])
              num10 = 0.0f;
            else
              num10 *= this.game.Data.RuleVar[421];
          }
          let mut num12: i32 =  this.game.ScreenWidth - 290;
          str2: String;
          if (num7 > 0)
          {
            str3: String = this.game.EditObj.MouseOverX.ToString() + "," + this.game.EditObj.MouseOverY.ToString() + " - Los:" + num7.ToString() + " - Obs:" + num8.ToString();
            str2 =  num10 <= 0.0 ? str3 + " - To far" : str3 + " - Dis: /" + num10.ToString();
          }
          else
            str2 = this.game.EditObj.MouseOverX.ToString() + "," + this.game.EditObj.MouseOverY.ToString() + " - No Los";
          str1 = str2 + " - Hid: " + Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY].LandscapeType].HidePts));
          SizeF sizeF5 = g.MeasureString(str1, this.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont4,  Math.Round( (num12 + 150) -  sizeF5.Width / 2.0 - 2.0), 8, Color.LightGray);
        }
        else
        {
          let mut num13: i32 =  this.game.ScreenWidth - 290;
          str1 = "No Unit selected";
          SizeF sizeF6 = g.MeasureString(str1, this.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont4,  Math.Round( (num13 + 150) -  sizeF6.Width / 2.0 - 2.0), 8, Color.LightGray);
        }
      }
      if (this.game.EditObj.OrderType == 0 & this.game.EditObj.udsUnitOrderMode < 1 & !this.game.EditObj.LayerSupplyOn)
      {
        if (!( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode))
        {
          this.GuiDownId = 1;
          if (this.game.Data.Round == 0)
            this.GuiDownId = 0;
          let mut num14: i32 =  num2 + 40;
          if (this.GuiDownId > 0)
          {
            if (!this.game.EditObj.GuiDown)
            {
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONGUIDOWN, tDescript: "Move Bottom GUI down & Get Larger Map [PgDown]", tBackbitmap: ( this.OwnBitmap), bbx: num14, bby: num1, totherback: 1, tsize: 29);
              this.GuiDownId = this.AddSubPart( tsubpart, num14, num1, 29, 29, 1);
            }
            else
            {
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONGUIUP, tDescript: "Move Bottm GUI up & Get unit info", tBackbitmap: ( this.OwnBitmap), bbx: num14, bby: num1, totherback: 1, tsize: 29);
              this.GuiDownId = this.AddSubPart( tsubpart, num14, num1, 29, 29, 1);
            }
          }
        }
        this.PrefsButtonId = 1;
        if (this.game.Data.Round > 0)
        {
          this.ResearchId = 1;
          this.SFDesignButtonId = 1;
          if (this.game.Data.Turn > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1 &&  this.game.Data.RuleVar[531] == 1.0)
            this.ModelDesignerId = 1;
          this.DipId = 1;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].HistoryStepCounter > -1)
            this.HistoryId = 1;
          else
            this.historytext = "No history records to report!";
          this.NextButtonId = 1;
          if (this.game.Data.Winner == -1)
          {
            if (this.game.Data.RegimeObj[this.game.Data.Turn].Sleep)
              this.ordersurrendertext = "You already surrendered.";
            else
              this.OrderSurrenderButtonId = 1;
          }
          else
            this.ordersurrendertext = "Somebody already won the game.";
          this.StatisticsButtonId = 1;
        }
        this.ButtonStackedUnitId = 0;
        if (this.game.EditObj.Zoom == 1)
          this.ButtonStackedUnitId = 1;
        if (this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          this.HexUnitButtonId = 1;
          this.ShowAsId = 1;
          this.ButtonZoomInId = 1;
          this.ButtonZoomOutId = 1;
          this.OfficerId = 0;
          if (this.game.Data.Round > 0 &&  this.game.Data.RuleVar[343] == 1.0)
            this.OfficerId = 1;
          if (this.game.SelectX <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth && this.game.SelectY <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
          {
            if (!this.game.Data.ShrowdOn)
            {
              if (this.game.EditObj.UnitSelected > -1)
              {
                if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn) && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ |  this.game.Data.RuleVar[887] == 1.0)
                  this.SupplyLayerButtonId = 1;
              }
              else if ( this.game.Data.RuleVar[887] == 1.0)
                this.SupplyLayerButtonId = 1;
              if ( this.game.Data.RuleVar[887] == 1.0)
                this.SupplyLayerButtonId = 1;
            }
            else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1 && this.game.Data.UnitObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0]].Regime == this.game.Data.Turn && this.game.Data.Round > 0)
              this.SupplyLayerButtonId = 1;
          }
        }
        if (this.game.SelectX > -1 && this.game.Data.Round == 0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 &&  this.game.Data.RuleVar[512] == 0.0)
          this.NewUnitButtonId = 1;
        if (this.game.EditObj.UnitSelected > -1)
        {
          if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn && !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].ModelMaster > -1)
          {
            if (!this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].ModelMaster].Fixed)
            {
              if ( this.game.Data.RuleVar[531] == 1.0)
                this.ChangeModelId = 1;
            }
            else
              this.ChangeModelText = "This unit has a fixed model. You cannot change it.";
          }
          if (this.game.Data.Round == 0)
            this.ChangeModelId = 0;
          if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
          {
            if (this.game.Data.Turn > -1 &&  this.game.Data.RuleVar[528] == 1.0 && this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
            {
              if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true))
                this.GiveUnitId = 1;
              else
                this.GiveUnitText = "You have no allies.";
            }
            if (this.game.Data.Round == 0)
              this.MoveButtonId = 1;
            else if (this.game.HandyFunctionsObj.CanUnitMove(this.game.EditObj.UnitSelected))
            {
              this.MoveButtonId = 1;
              if ( this.game.Data.RuleVar[954] == 1.0 | this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1)
                this.GroupMoveButtonId = 1;
            }
            else
            {
              this.MoveButtonId = 0;
              this.MoveButtonText = "Unit does not have enough Action Points to move.";
              this.GroupMoveButtonId = 0;
              this.GroupMoveButtonText = "Unit does not have enough Action Points to move.";
            }
            if (this.game.Data.Round > 0 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].Type > 7 &&  this.game.Data.RuleVar[894] > 0.0)
            {
              this.MoveButtonId = 0;
              this.MoveButtonText = "High command cannot move";
              this.GroupMoveButtonId = 0;
              this.GroupMoveButtonText = "High command cannot move";
            }
            if ( this.game.Data.RuleVar[521] == 0.0)
              this.HqUnitButtonId = 1;
            if (this.game.Data.Turn > -1 && this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false) > this.game.Data.RegimeObj[this.game.Data.Turn].ResPts & !this.game.EditObj.TutMode)
            {
              this.HqUnitButtonId = 0;
              this.HqUnitButtonText = "You dont have the required " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false)) + " PP to switch the HQ of this unit.";
            }
            if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].ModelMaster > -1)
              {
                if (!this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].ModelMaster].Fixed)
                {
                  if ( this.game.Data.RuleVar[531] == 1.0)
                    this.ChangeModelId = 1;
                }
                else
                  this.ChangeModelText = "This unit has a fixed model. You cannot change it.";
              }
            }
            else
              this.ChangeModelText = "You cannot change Model of a HQ unit.";
            if (this.game.Data.Round == 0)
              this.ChangeModelId = 0;
            if (this.game.Data.Round > 0)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount == -1)
              {
                this.disbandid = 1;
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].ResPts + this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].PP < 0)
                {
                  this.disbandid = 0;
                  this.disbandtext = "You do not have the PP to disband the unit.";
                }
              }
              else
              {
                this.disbandid = 0;
                this.disbandtext = "You can only disband a unit with no subformations in it.";
              }
            }
            if (this.game.Data.Round > 0 & this.game.SelectX > -1)
            {
              let mut num15: i32 =  0;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].StructuralPts < this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].StructuralPts && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].EPCost > 0)
                num15 = 1;
              if (this.game.HandyFunctionsObj.CanUnitBuild(this.game.EditObj.UnitSelected) | num15 == 1)
              {
                if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 0)
                  this.BuildButtonId = 1;
                else
                  this.buildbuttontext = "You cannot build/repair something if you have zero action points.";
              }
              else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
              {
                this.BuildButtonId = 0;
                this.buildbuttontext = "Location is not damaged / Unit cannot repair this location";
              }
              else
              {
                this.BuildButtonId = 1;
                this.buildbuttontext = "Not enough Engineer Pts to build a location.";
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.UnitSelected) > 0)
              {
                if (this.game.HandyFunctionsObj.CanUnitInfra(this.game.EditObj.UnitSelected))
                {
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 0)
                  {
                    this.InfraButtonId = 1;
                  }
                  else
                  {
                    this.InfraButtonId = 0;
                    this.infrabuttontext = "Unit has 0 action points.";
                  }
                }
                else
                {
                  this.InfraButtonId = 0;
                  this.infrabuttontext = !( this.game.Data.RuleVar[483] > 0.0 & this.game.Data.Product >= 6) ? "Unit does not have EP required or other side of river is enemy hex." : "Unit does not have the EP required.";
                }
              }
              else
              {
                this.InfraButtonId = 0;
                this.infrabuttontext = "Unit has no engineer points.";
              }
            }
            if (this.game.Data.Round > 0 & this.game.SelectX > -1 &&  this.game.Data.RuleVar[474] > 0.0 & this.game.Data.Product >= 6)
            {
              if (this.game.HandyFunctionsObj.CanTransport(this.game.EditObj.UnitSelected))
                this.TransportButtonId = 1;
              else
                this.transportButtonText = "Unit is not capable of Transporting other Units.";
            }
            if (this.game.Data.Round > 0 & this.game.SelectX > -1 &&  this.game.Data.RuleVar[475] > 0.0 & this.game.Data.Product >= 6)
              this.BattleGroupButtonId = 1;
            if (this.game.Data.Round > 0 & this.game.SelectX > -1)
            {
              if (this.game.HandyFunctionsObj.CanWeBlowBridge(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected))
              {
                if (this.game.HandyFunctionsObj.GetBlowBridgePts(this.game.EditObj.UnitSelected) > 0)
                {
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 49)
                  {
                    this.BlowBridgeButtonId = 1;
                  }
                  else
                  {
                    this.BlowBridgeButtonId = 0;
                    this.blowbridgebuttontext = "You need at least 50 Action Pts for an attempt.";
                  }
                }
                else
                {
                  this.BlowBridgeButtonId = 0;
                  this.blowbridgebuttontext = "Unit does not have capability to blow bridge.";
                }
              }
              else
              {
                this.BlowBridgeButtonId = 0;
                this.blowbridgebuttontext = "No bridge is present in this hex.";
              }
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
              {
                if (!this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].Invincible)
                {
                  if (this.game.HandyFunctionsObj.GetBlowBridgePts(this.game.EditObj.UnitSelected) > 0)
                  {
                    if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) > 49)
                    {
                      this.BlowLocationButtonId = 1;
                    }
                    else
                    {
                      this.blowlocationtext = "You need at least 50 Action Pts for an attempt.";
                      this.BlowLocationButtonId = 0;
                    }
                  }
                  else
                  {
                    this.blowlocationtext = "Unit does not have capability to raze location.";
                    this.BlowLocationButtonId = 0;
                  }
                }
                else
                {
                  this.blowlocationtext = "This locationtype cannot be damaged.";
                  this.BlowLocationButtonId = 0;
                }
              }
              else
              {
                this.blowlocationtext = "No Location in hex.";
                this.BlowLocationButtonId = 0;
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X == -1)
                this.game.EditObj.UnitSelected = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].OnBoard;
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter == -1)
              {
                if ( this.game.Data.RuleVar[315] == 1.0 & !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                {
                  this.TransferButtonId = 0;
                  this.TransferButtonText = "You can only transfer from a HQ.";
                }
                else
                  this.TransferButtonId = 1;
                if (!this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
                {
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].get_APPenalty(this.game.Data.Turn) > 0)
                  {
                    this.strategicbuttontext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                    this.groupstrategictext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                  }
                  else if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].DidAttack)
                  {
                    this.StrategicButtonId = 1;
                    if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].StartSize > -1)
                    {
                      let mut num16: i32 =  1;
                      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                      for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                      {
                        if (this.game.Data.UnitObj[index3].Historical == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical & this.game.Data.UnitObj[index3].Regime == this.game.Data.Turn && this.game.Data.UnitObj[index3].DidAttack)
                          num16 = 0;
                      }
                      if (num16 == 1)
                        this.GroupStrategicButtonId = 1;
                    }
                  }
                  else
                  {
                    this.strategicbuttontext = "You cannot strategic transfer a unit that participated in an attack.";
                    this.groupstrategictext = "You cannot strategic transfer a unit that participated in an attack.";
                  }
                }
                else
                {
                  this.groupstrategictext = "You cannot strategic transfer units with air or navy subformations.";
                  this.strategicbuttontext = "You cannot strategic transfer units with air or navy subformations.";
                }
              }
              else
              {
                this.TransferButtonText = "You cannot transfer while carrying passengers.";
                this.strategicbuttontext = "You cannot strategicly transfer a unit carrying passengers.";
                this.groupstrategictext = "You cannot strategicly transfer a unit carrying passengers.";
              }
            }
            if (this.game.Data.Round > 0)
            {
              if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                {
                  if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].IsAirfield)
                  {
                    if (this.game.HandyFunctionsObj.GetUnitCarryCap(this.game.EditObj.UnitSelected, 2) > 0)
                      this.ParadropButtonId = 1;
                    else
                      this.paradropbuttontext = "Unit has no aircraft that can carry paratroopers.";
                    this.AirReconButtonId = 1;
                    if (this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.UnitSelected, 2) > 0)
                    {
                      if (this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.UnitSelected) >= 40)
                        this.AirSupplyButtonId = 1;
                      else
                        this.airsupplybuttontext = "You need a minimum of 40 ap for an airsupply mission.";
                    }
                    else
                      this.airsupplybuttontext = "Units air subformations have no carry points.";
                  }
                  else
                  {
                    this.paradropbuttontext = "Action must start from airfield. No airfield in this hex.";
                    this.AirReconButtonText = "Action must start from airfield. No airfield in this hex.";
                    this.airsupplybuttontext = "Action must start from airfield. No airfield in this hex.";
                  }
                }
                else
                {
                  this.paradropbuttontext = "Action must start from airfield. No airfield in this hex.";
                  this.AirReconButtonText = "Action must start from airfield. No airfield in this hex.";
                  this.airsupplybuttontext = "Action must start from airfield. No airfield in this hex.";
                }
                if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
                  this.AirReconButtonId = 1;
              }
              else
              {
                this.paradropbuttontext = "Unit has no aircraft.";
                this.AirReconButtonText = "Unit has no aircraft.";
                this.airsupplybuttontext = "Unit has no aircraft";
              }
            }
            if (this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
            {
              if (this.game.HandyFunctionsObj.GetUnitCarryCap(this.game.EditObj.UnitSelected, 1) > 0)
              {
                if (this.game.HandyFunctionsObj.CanUnitLoadaUnit(this.game.EditObj.UnitSelected))
                  this.LoadButtonId = 1;
                else
                  this.loadbuttontext = "No unit around to be loaded by this unit.";
              }
              else
                this.loadbuttontext = "Unit has no ships that can carry cargo.";
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter > -1)
              {
                if (this.game.EditObj.SFSelected > this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount)
                {
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerList[this.game.EditObj.SFSelected - (1 + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount)]) > 0 | this.game.Data.Round == 0)
                    this.UnLoadButtonID = 1;
                  else if ( this.game.Data.RuleVar[880] > 0.0)
                  {
                    if (this.game.HandyFunctionsObj.IsHexPort(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0))
                      this.UnLoadButtonID = 1;
                    else
                      this.unloadbuttontext = "Passenger must have more then 0 ap to unload outside a port";
                  }
                  else
                    this.unloadbuttontext = "Passenger must have more then 0 ap to unload.";
                }
                else
                  this.unloadbuttontext = "Unit has passengers, but no unit is selected to unload";
              }
              else
                this.unloadbuttontext = "Unit has no passenger(s).";
            }
            else
            {
              this.loadbuttontext = "Unit has no navy subformations.";
              this.unloadbuttontext = "Unit has no navy subformations.";
            }
          }
          else
          {
            this.HqUnitButtonText = "Not a friendly unit.";
            if (this.game.Data.Turn > -1 && this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
            {
              this.HqUnitButtonId = 1;
              if (this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false) > this.game.Data.RegimeObj[this.game.Data.Turn].ResPts)
              {
                this.HqUnitButtonId = 0;
                this.HqUnitButtonText = "You dont have the required " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.UnitSelected, false)) + " PP to switch the HQ of this unit.";
              }
            }
            this.supplylayerbuttontext = "Not a friendly unit.";
            this.MoveButtonText = "Not a friendly unit.";
            this.GroupMoveButtonText = "Not a friendly unit.";
            this.disbandtext = "Not a friendly unit.";
            this.buildbuttontext = "Not a friendly unit.";
            this.infrabuttontext = "Not a friendly unit.";
            this.blowbridgebuttontext = "Not a friendly unit.";
            this.transportButtonText = "Not a friendly unit.";
            this.battlegroupText = "Not a friendly unit.";
            this.blowlocationtext = "Not a friendly unit.";
            this.TransferButtonText = "Not a friendly unit.";
            this.strategicbuttontext = "Not a friendly unit.";
            this.groupstrategictext = "Not a friendly unit.";
            this.paradropbuttontext = "Not a friendly unit.";
            this.AirReconButtonText = "Not a friendly unit.";
            this.airsupplybuttontext = "Not a friendly unit.";
            this.loadbuttontext = "Not a friendly unit.";
            this.unloadbuttontext = "Not a friendly unit.";
            this.GiveUnitText = "Not a friendly unit.";
            this.OfficerText = "Not a friendly unit.";
            this.ChangeModelText = "Not a friendly unit.";
          }
        }
        else
        {
          this.groupstrategictext = "No unit selected.";
          this.supplylayerbuttontext = "You must select a friendly HQ first.";
          this.MoveButtonText = "No unit selected.";
          this.GroupMoveButtonText = "No unit selected.";
          this.HqUnitButtonText = "No unit selected.";
          this.disbandtext = "No unit selected.";
          this.buildbuttontext = "No unit selected.";
          this.infrabuttontext = "No unit selected.";
          this.blowbridgebuttontext = "No unit selected.";
          this.transportButtonText = "No unit selected.";
          this.battlegroupText = "No unit selected.";
          this.blowlocationtext = "No unit selected.";
          this.TransferButtonText = "No unit selected.";
          this.strategicbuttontext = "No unit selected.";
          this.paradropbuttontext = "No unit selected.";
          this.AirReconButtonText = "No unit selected.";
          this.airsupplybuttontext = "No unit selected.";
          this.loadbuttontext = "No unit selected.";
          this.unloadbuttontext = "No unit selected.";
          this.GiveUnitText = "Not a friendly unit.";
          this.OfficerText = "Not a friendly unit.";
          this.ChangeModelText = "Not a friendly unit.";
          if (this.game.SelectX > -1 && this.game.Data.Round > 0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location == -1)
          {
            let mut locTypeCounter: i32 =  this.game.Data.LocTypeCounter;
            for (let mut index4: i32 =  0; index4 <= locTypeCounter; index4 += 1)
            {
              if (this.game.Data.LocTypeObj[index4].Buildable && this.game.Data.LocTypeObj[index4].HumanCanBuild)
                this.BuildButtonId = 1;
            }
          }
        }
        if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.Turn > -1 &&  this.game.Data.RuleVar[529] == 1.0)
        {
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
          {
            if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, false))
              this.GiveHexId = 1;
            else
              this.GiveHexText = "You have no allies.";
          }
          else
            this.GiveHexText = !this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, false) ? "You have no allies." : "Not a friendly hex.";
        }
        if (this.game.EditObj.LayerSupplyOn)
          this.SupplyLayerButtonId = 1;
        if (this.game.Data.Round > 0)
        {
          if (this.game.SelectX > -1 & this.game.SelectY > -1)
          {
            if (this.game.EditObj.UnitSelected > -1)
            {
              if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                this.HqUnitButtonId = 1;
              if (this.game.Data.Turn > -1)
              {
                if ( this.game.Data.RuleVar[528] == 1.0 && this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                  this.GiveUnitId = 1;
                if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                {
                  if (!this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].get_APPenalty(this.game.Data.Turn) > 0)
                    {
                      this.strategicbuttontext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                      this.groupstrategictext = "You cant strategically transfer a unit from a hex you conquered in this turn.";
                    }
                    else if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].DidAttack)
                    {
                      this.StrategicButtonId = 1;
                      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].StartSize > -1)
                      {
                        let mut num17: i32 =  1;
                        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                        for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
                        {
                          if (this.game.Data.UnitObj[index5].Historical == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical & this.game.Data.UnitObj[index5].Regime == this.game.Data.Turn && this.game.Data.UnitObj[index5].DidAttack)
                            num17 = 0;
                        }
                        if (num17 == 1)
                          this.GroupStrategicButtonId = 1;
                      }
                    }
                    else
                    {
                      this.strategicbuttontext = "You cannot strategic transfer a unit that participated in an attack.";
                      this.groupstrategictext = "You cannot strategic transfer a unit that participated in an attack.";
                    }
                  }
                  else
                  {
                    this.groupstrategictext = "You cannot strategic transfer units with air or navy subformations.";
                    this.strategicbuttontext = "You cannot strategic transfer units with air or navy subformations.";
                  }
                }
              }
              if ( this.game.Data.RuleVar[894] > 0.0 && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].Type == 8)
              {
                this.StrategicButtonId = 0;
                this.GroupStrategicButtonId = 0;
                this.strategicbuttontext = "High command cannot move";
                this.groupstrategictext = "High command cannot move";
              }
            }
            if (this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.Data.Turn, testingforattack: true) & this.game.Data.Round > 0)
            {
              this.AttackButtonId = 1;
              this.ArtAttackButtonId = 1;
              if (this.game.HandyFunctionsObj.IsHexHarbourOrSea(this.game.SelectX, this.game.SelectY, 0))
              {
                this.seaAttackButtonId = 1;
              }
              else
              {
                this.seaAttackButtonId = 0;
                this.SeaAttackButtonText = "Only enemy vessels on sea hexes or enemy ports can be targetted.";
              }
              this.SeaArtAttackButtonId = 1;
              this.AirAttackButtonId = 1;
              if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
              {
                this.SeaArtAttackButtonId = 0;
                this.SeaArtAttackButtonText = "You can only shore bombardment land hexes.";
              }
            }
            else
            {
              str4: String = "No enemy unit in hex.";
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime] >= 1)
                str4 = "Cannot attack. You need to declare war before you can attack";
              this.AttackButtonText = str4;
              this.ArtAttackButtonText = str4;
              this.SeaAttackButtonText = str4;
              this.SeaArtAttackButtonText = str4;
              this.AirAttackButtonText = str4;
              if ( this.game.Data.RuleVar[318] > 0.0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime] == 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn)
                this.AirAttackButtonId = 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime] == 0)
            {
              this.ArtAttackButtonId = 1;
              this.SeaArtAttackButtonId = 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_SeeNow(this.game.Data.Turn) > 0)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn)
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 | this.game.HandyFunctionsObj.HasHexBridge(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected))
                {
                  if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                    this.InterdictButtonId = 1;
                  else
                    this.interdictbuttontext = "You are not at war with this regime. So you cannot bomb.";
                }
                else
                  this.interdictbuttontext = "No location or bridge in this hex to bomb.";
              }
              else
                this.interdictbuttontext = "You cannot bomb your own territory.";
            }
            else
              this.InterdictButtonId = 1;
          }
          else
          {
            this.AttackButtonText = "No hex selected.";
            this.ArtAttackButtonText = "No hex selected.";
            this.SeaAttackButtonText = "No hex selected.";
            this.SeaArtAttackButtonText = "No hex selected.";
            this.AirAttackButtonText = "No hex selected.";
            this.interdictbuttontext = "No hex selected.";
          }
        }
        if (this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime, this.game.Data.Turn) & this.game.Data.Round != 0)
          {
            if ( this.game.Data.RuleVar[512] == 0.0)
              this.NewUnitButtonId = 1;
            if ( this.game.Data.RuleVar[527] > 0.0)
              this.NewUnitButton2Id = 1;
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_ReconPts(this.game.Data.Turn) < 1)
            {
              this.NewUnitButtonId = 0;
              this.NewUnitButton2Id = 0;
              this.NewUnitButtonText = "You must have recon on the hex you want to create a unit.";
              this.newunitbutton2text = "You must have recon on the hex you want to create a unit.";
            }
          }
          else
          {
            this.NewUnitButtonText = "You can only make a new unit on a friendly hex.";
            this.newunitbutton2text = "You can only make a new unit on a friendly hex.";
          }
        }
        else
        {
          this.NewUnitButtonText = "No hex selected";
          this.newunitbutton2text = "No hex selected";
        }
        if (this.game.Data.Round == 0)
        {
          this.NewUnitButtonId = 0;
          this.NewUnitButton2Id = 0;
        }
        if ( this.game.Data.RuleVar[527] == 0.0)
        {
          this.newunitbutton2text = "";
          this.NewUnitButton2Id = 0;
        }
        if (this.game.Data.Round > 0)
          this.QuitID = 1;
        this.SaveId = 1;
        if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.Round == 0 && this.game.HandyFunctionsObj.CanLocProduce(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location, this.game.Data.Turn))
        {
          this.HqProdButtonId = 1;
          this.ProdButtonId = 1;
        }
        if (this.HqProdButtonId == 0)
          this.hqprodbuttontext = "You have to select a friendly location that can produce stuff.";
        if (this.ProdButtonId == 0)
          this.prodbuttontext = "You have to select a friendly location that can produce stuff.";
        num18: i32;
        if (this.game.Data.Round > 0 && this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter == -1)
          {
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn)
            {
              num18 = 1;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime].UberRegime == this.game.Data.Turn)
                num18 = 2;
            }
          }
          else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1 & this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
            {
              num18 = 0;
            }
            else
            {
              num18 = 1;
              if (this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime == this.game.Data.Turn)
                num18 = 2;
            }
          }
          else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.Turn)
          {
            num18 = 1;
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime].UberRegime == this.game.Data.Turn)
              num18 = 2;
          }
        }
        if (this.game.EditObj.GuiDown)
        {
          if (this.StrategicButtonId > 0)
          {
            this.StrategicButtonId = 0;
            this.StrategicButtonId2 = 1;
            this.strategicbuttontext = "Cannot use this order with the Gui down.";
          }
          if (this.GroupStrategicButtonId > 0)
          {
            this.GroupStrategicButtonId = 0;
            this.GroupStrategicButtonId2 = 1;
            this.groupstrategictext = "Cannot use this order with the Gui down.";
          }
          if (this.NewUnitButtonId > 0)
          {
            this.NewUnitButtonId = 0;
            this.NewUnitButtonId2 = 1;
            this.NewUnitButtonText = "Cannot use this order with the Gui down.";
          }
          if (this.NewUnitButton2Id > 0)
          {
            this.NewUnitButton2Id = 0;
            this.NewUnitButton2Id2 = 1;
            this.newunitbutton2text = "Cannot use this order with the Gui down.";
          }
          if (this.ChangeModelId > 0)
          {
            this.ChangeModelId = 0;
            this.ChangeModelId2 = 1;
            this.ChangeModelText = "Cannot use this order with the Gui down.";
          }
          if (this.OfficerId > 0)
          {
            this.OfficerId = 0;
            this.OfficerId2 = 1;
            this.OfficerText = "Cannot use this order with the Gui down.";
          }
        }
        if ( this.game.Data.RuleVar[814] == 1.0)
        {
          this.OfficerId = 0;
          this.OfficerId2 = 1;
          this.OfficerText = "Cannot use this order in free movement phase.";
          this.NewUnitButtonId = 0;
          this.NewUnitButtonId2 = 1;
          this.NewUnitButtonText = "Cannot use this order in free movement phase.";
          this.NewUnitButton2Id = 0;
          this.NewUnitButton2Id2 = 1;
          this.newunitbutton2text = "Cannot use this order in free movement phase.";
          this.ChangeModelId = 0;
          this.ChangeModelId2 = 1;
          this.ChangeModelText = "Cannot use this order in free movement phase.";
        }
        let mut num19: i32 =  800 + num2;
        num20: i32;
        num21: i32;
        if ( this.game.Data.RuleVar[343] == 1.0)
        {
          if (this.OfficerId > 0)
          {
            num21 = num20 + 1;
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONOFFICER, tDescript: "Click to go to officerpool", tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num21 * 33), bby: num1, totherback: 4, tsize: 29);
            this.OfficerId = this.AddSubPart( tsubpart, num19 + num21 * 33, num1, 29, 29, 1);
          }
          else
          {
            num21 = num20 + 1;
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONOFFICER, 1, this.OfficerText,  this.OwnBitmap, num19 + num21 * 33, num1, 4, 29);
            this.OfficerId2 = this.AddSubPart( tsubpart, num19 + num21 * 33, num1, 29, 29, 0);
          }
        }
        num22: i32;
        if (this.HistoryId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 26))
        {
          num22 = num21 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONHISTORY, tDescript: "History", tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num22 * 33), bby: num1, totherback: 4, tsize: 29);
          this.HistoryId = this.AddSubPart( tsubpart, num19 + num22 * 33, num1, 29, 29, 1);
        }
        else
        {
          num22 = num21 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.historytext = "";
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONHISTORY, 1, this.historytext,  this.OwnBitmap, num19 + num22 * 33, num1, 4, 29);
          this.HistoryId2 = this.AddSubPart( tsubpart, num19 + num22 * 33, num1, 29, 29, 0);
        }
        num23: i32;
        if (this.OrderSurrenderButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 41))
        {
          num23 = num22 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONSURRENDER, tDescript: "Surrender", tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num23 * 33), bby: num1, totherback: 4, tsize: 29);
          this.OrderSurrenderButtonId = this.AddSubPart( tsubpart, num19 + num23 * 33, num1, 29, 29, 1);
        }
        else
        {
          num23 = num22 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.historytext = "";
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONSURRENDER, 1, this.ordersurrendertext,  this.OwnBitmap, num19 + num23 * 33, num1, 4, 29);
          this.OrderSurrenderButtonId2 = this.AddSubPart( tsubpart, num19 + num23 * 33, num1, 29, 29, 0);
        }
        num24: i32;
        if (this.SaveId > 0 & !this.game.Data.PBEM & this.game.EditObj.TutOrder == -1)
        {
          num24 = num23 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONSAVE, tDescript: "Save", tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num24 * 33), bby: num1, totherback: 4, tsize: 29);
          this.SaveId = this.AddSubPart( tsubpart, num19 + num24 * 33, num1, 29, 29, 1);
        }
        else
        {
          num24 = num23 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONSAVE, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num24 * 33), bby: num1, totherback: 4, tsize: 29);
          this.SaveId2 = this.AddSubPart( tsubpart, num19 + num24 * 33, num1, 29, 29, 0);
        }
        if (this.NextButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 50))
        {
          let mut num25: i32 =  num24 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONNEXT, tDescript: "End your Turn", tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num25 * 33), bby: num1, totherback: 4, tsize: 29);
          this.NextButtonId = this.AddSubPart( tsubpart, num19 + num25 * 33, num1, 29, 29, 1);
        }
        else
        {
          let mut num26: i32 =  num24 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONNEXT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num19 + num26 * 33), bby: num1, totherback: 4, tsize: 29);
          this.NextButtonId = this.AddSubPart( tsubpart, num19 + num26 * 33, num1, 29, 29, 0);
        }
        let mut num27: i32 =  40 + num2;
        num28: i32;
        num29: i32;
        if ( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode)
        {
          if ( this.game.Data.RuleVar[458] > 0.0 & this.game.Data.Product >= 6)
          {
            let mut num30: i32 =   Math.Round(Conversion.Val( this.game.Data.RuleVar[458]));
            if (num30 > 3)
              num30 = 3;
            let mut num31: i32 =  num30 * 33;
            if (num18 == 0)
            {
              if (this.MoveButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 1))
              {
                let mut num32: i32 =  num28 + 1;
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONMOVE, "Move Unit", tDescript: "Move Unit [M]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num32 * 33), bby: num1, totherback: 1, tWidth: (29 + num31), tHeight: 29);
                this.MoveButtonId = this.AddSubPart( tsubpart, num27 + num32 * 33, num1, 29 + num31, 29, 1);
                num29 = num32 + num30;
              }
              else
              {
                let mut num33: i32 =  num28 + 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.MoveButtonText = "";
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONMOVE, "Move Unit", 1, this.MoveButtonText,  this.OwnBitmap, num27 + num33 * 33, num1, 1, 29 + num31, 29);
                this.MoveButtonId2 = this.AddSubPart( tsubpart, num27 + num33 * 33, num1, 29 + num31, 29, 0);
                num29 = num33 + num30;
              }
              if ( this.game.Data.RuleVar[533] == 0.0 &  this.game.Data.RuleVar[344] > 0.0)
              {
                if ( this.game.Data.RuleVar[344] > 0.0 & this.GroupMoveButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 48))
                {
                  let mut num34: i32 =  num29 + 1;
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONMOVE2, "Group Move", tDescript: "Group Move [G]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num34 * 33), bby: num1, totherback: 1, tWidth: (29 + num31), tHeight: 29);
                  this.GroupMoveButtonId = this.AddSubPart( tsubpart, num27 + num34 * 33, num1, 29 + num31, 29, 1);
                  num29 = num34 + num30;
                }
                else
                {
                  let mut num35: i32 =  num29 + 1;
                  if (this.game.EditObj.TutOrder > -1)
                    this.GroupMoveButtonText = "";
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONMOVE2, "Group Move", 1, this.GroupMoveButtonText,  this.OwnBitmap, num27 + num35 * 33, num1, 1, 29 + num31, 29);
                  this.GroupMoveButtonId2 = this.AddSubPart( tsubpart, num27 + num35 * 33, num1, 29 + num31, 29, 0);
                  num29 = num35 + num30;
                }
              }
            }
          }
          else if (num18 == 0)
          {
            if (this.MoveButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 1))
            {
              num29 = num28 + 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONMOVE, tDescript: "Move Unit [M]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.MoveButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 = num28 + 1;
              if (this.game.EditObj.TutOrder > -1)
                this.MoveButtonText = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONMOVE, 1, this.MoveButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.MoveButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
            if ( this.game.Data.RuleVar[533] == 0.0 &  this.game.Data.RuleVar[344] > 0.0)
            {
              if ( this.game.Data.RuleVar[344] > 0.0 & this.GroupMoveButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 48))
              {
                num29 += 1;
                tsubpart =  new MarcButtonPartClass(this.game.BUTTONMOVE2, tDescript: "Group Move Unit [G]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
                this.GroupMoveButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
              }
              else
              {
                num29 += 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.GroupMoveButtonText = "";
                tsubpart =  new MarcButtonPartClass(this.game.BUTTONMOVE2, 1, this.GroupMoveButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
                this.GroupMoveButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
              }
            }
          }
        }
        if (num18 == 0 | num18 == 2)
        {
          if ( this.game.Data.RuleVar[458] > 0.0 & this.game.Data.Product >= 6)
          {
            let mut num36: i32 =   Math.Round(Conversion.Val( this.game.Data.RuleVar[458]));
            if (( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode) & num36 > 1)
              num36 = 1;
            if (num36 > 2)
              num36 = 2;
            let mut num37: i32 =  num36 * 33;
            if ( this.game.Data.RuleVar[521] == 0.0 | this.game.Data.Round == 0)
            {
              if (this.HqUnitButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 3))
              {
                let mut num38: i32 =  num29 + 1;
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONHQUNIT, "HQ", tDescript: "Set Units HQ [H]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num38 * 33), bby: num1, totherback: 1, tWidth: (29 + num37), tHeight: 29);
                this.HqUnitButtonId = this.AddSubPart( tsubpart, num27 + num38 * 33, num1, 29 + num37, 29, 1);
                num29 = num38 + num36;
              }
              else
              {
                let mut num39: i32 =  num29 + 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.HqUnitButtonText = "";
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONHQUNIT, "HQ", 1, this.HqUnitButtonText,  this.OwnBitmap, num27 + num39 * 33, num1, 1, 29 + num37, 29);
                this.HqUnitButtonId2 = this.AddSubPart( tsubpart, num27 + num39 * 33, num1, 29 + num37, 29, 0);
                num29 = num39 + num36;
              }
            }
            else
            {
              this.HqUnitButtonId = 0;
              this.HqUnitButtonId2 = 0;
            }
          }
          else if ( this.game.Data.RuleVar[521] == 0.0 | this.game.Data.Round == 0)
          {
            if (this.HqUnitButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 3))
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONHQUNIT, tDescript: "Set Units HQ [H]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.HqUnitButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.HqUnitButtonText = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONHQUNIT, 1, this.HqUnitButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.HqUnitButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          else
          {
            this.HqUnitButtonId = 0;
            this.HqUnitButtonId2 = 0;
          }
          if ( this.game.Data.RuleVar[520] == 0.0)
          {
            if (this.StrategicButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 18))
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONSTRATEGIC, tDescript: "Strategic Transfer [S]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.StrategicButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.strategicbuttontext = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONSTRATEGIC, 1, this.strategicbuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.StrategicButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
            if ( this.game.Data.RuleVar[533] == 0.0 &  this.game.Data.RuleVar[344] > 0.0)
            {
              if (this.GroupStrategicButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 49))
              {
                num29 += 1;
                tsubpart =  new MarcButtonPartClass(this.game.BUTTONSTRATEGIC2, tDescript: "Group Strategic Transfer", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
                this.GroupStrategicButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
              }
              else
              {
                num29 += 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.groupstrategictext = "";
                tsubpart =  new MarcButtonPartClass(this.game.BUTTONSTRATEGIC2, 1, this.groupstrategictext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
                this.GroupStrategicButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
              }
            }
          }
          else
          {
            this.StrategicButtonId = 0;
            this.StrategicButtonId2 = 0;
            this.GroupStrategicButtonId = 0;
            this.GroupStrategicButtonId2 = 0;
          }
          if ( this.game.Data.RuleVar[528] == 1.0)
          {
            if (this.GiveUnitId > 0)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONGIVEUNIT, tDescript: "Click to give a unit to an ally (or whole HQ group)", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.GiveUnitId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutMode)
                this.GiveUnitText = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONGIVEUNIT, 1, this.GiveUnitText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.GiveUnitId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          else
          {
            this.GiveUnitId = 0;
            this.GiveUnitId2 = 0;
          }
          if ( this.game.Data.RuleVar[512] == 0.0)
          {
            if (this.NewUnitButtonId > 0)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONNEWUNIT, tDescript: "Make New Unit [N]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.NewUnitButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONNEWUNIT, 1, this.NewUnitButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.NewUnitButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          else
          {
            this.NewUnitButtonId = 0;
            this.NewUnitButtonId2 = 0;
          }
          if ( this.game.Data.RuleVar[527] > 0.0)
          {
            if (this.NewUnitButton2Id > 0)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONNEWUNIT2, tDescript: "Sub Unit Options (add)", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.NewUnitButton2Id = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONNEWUNIT2, 1, this.NewUnitButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.NewUnitButton2Id2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[531] == 1.0)
          {
            if (this.ChangeModelId > 0)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONCHANGEMODEL, tDescript: "Click to change model of this unit", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.ChangeModelId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONCHANGEMODEL, 1, this.ChangeModelText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.ChangeModelId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
        }
        if (num18 == 0)
        {
          if ( this.game.Data.RuleVar[522] == 0.0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 33))
          {
            if (this.AirReconButtonId > 0)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONAIRRECON, tDescript: "Click to do an Air Recon Mission", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.AirReconButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.AirReconButtonText = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONAIRRECON, 1, this.AirReconButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.AirReconButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[505] == 0.0)
          {
            if ( this.game.Data.RuleVar[458] > 0.0 & this.game.Data.Product >= 6)
            {
              let mut num40: i32 =   Math.Round(Conversion.Val( this.game.Data.RuleVar[458]));
              if (num40 > 2)
                num40 = 2;
              if (( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode) & num40 > 1)
                num40 = 1;
              let mut num41: i32 =  num40 * 33;
              if (this.BlowBridgeButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 35))
              {
                let mut num42: i32 =  num29 + 1;
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONBLOWBRIDGE, "Blow", tDescript: "Click to blow a bridge", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num42 * 33), bby: num1, totherback: 1, tWidth: (29 + num41), tHeight: 29);
                this.BlowBridgeButtonId = this.AddSubPart( tsubpart, num27 + num42 * 33, num1, 29 + num41, 29, 1);
                num29 = num42 + num40;
              }
              else
              {
                let mut num43: i32 =  num29 + 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.blowbridgebuttontext = "";
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONBLOWBRIDGE, "Blow", 1, this.blowbridgebuttontext,  this.OwnBitmap, num27 + num43 * 33, num1, 1, 29 + num41, 29);
                this.BlowBridgeButtonId2 = this.AddSubPart( tsubpart, num27 + num43 * 33, num1, 29 + num41, 29, 0);
                num29 = num43 + num40;
              }
              if ( this.game.Data.RuleVar[474] > 0.0)
              {
                if (this.TransportButtonId > 0)
                {
                  let mut num44: i32 =  num29 + 1;
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONLOAD, "Trnp", tDescript: "Click to attach Units for Transport", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num44 * 33), bby: num1, totherback: 1, tWidth: (29 + num41), tHeight: 29);
                  this.TransportButtonId = this.AddSubPart( tsubpart, num27 + num44 * 33, num1, 29 + num41, 29, 1);
                  num29 = num44 + num40;
                }
                else
                {
                  let mut num45: i32 =  num29 + 1;
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONLOAD, "Trnp", 1, this.transportButtonText,  this.OwnBitmap, num27 + num45 * 33, num1, 1, 29 + num41, 29);
                  this.TransportButtonId2 = this.AddSubPart( tsubpart, num27 + num45 * 33, num1, 29 + num41, 29, 0);
                  num29 = num45 + num40;
                }
              }
              if ( this.game.Data.RuleVar[475] > 0.0)
              {
                if (this.BattleGroupButtonId > 0)
                {
                  let mut num46: i32 =  num29 + 1;
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONTRANSFER, "Trnf", tDescript: "Click to create Battlegroup or Transfer", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num46 * 33), bby: num1, totherback: 1, tWidth: (29 + num41), tHeight: 29);
                  this.BattleGroupButtonId = this.AddSubPart( tsubpart, num27 + num46 * 33, num1, 29 + num41, 29, 1);
                  num29 = num46 + num40;
                }
                else
                {
                  let mut num47: i32 =  num29 + 1;
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONTRANSFER, "Trnf", 1, this.transportButtonText,  this.OwnBitmap, num27 + num47 * 33, num1, 1, 29 + num41, 29);
                  this.BattleGroupButtonId2 = this.AddSubPart( tsubpart, num27 + num47 * 33, num1, 29 + num41, 29, 0);
                  num29 = num47 + num40;
                }
              }
            }
            else if (this.BlowBridgeButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 35))
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONBLOWBRIDGE, tDescript: "Click to blow a bridge", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.BlowBridgeButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.blowbridgebuttontext = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONBLOWBRIDGE, 1, this.blowbridgebuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.BlowBridgeButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[503] == 0.0)
          {
            if ( this.game.Data.RuleVar[458] > 0.0 & this.game.Data.Product >= 6)
            {
              let mut num48: i32 =   Math.Round(Conversion.Val( this.game.Data.RuleVar[458]));
              if (num48 > 2)
                num48 = 2;
              if (( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode) & num48 > 1)
                num48 = 1;
              let mut num49: i32 =  num48 * 33;
              if (this.InfraButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 37))
              {
                let mut num50: i32 =  num29 + 1;
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONBUILDROAD, "Repair", tDescript: "Click to build bridge [R]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num50 * 33), bby: num1, totherback: 1, tWidth: (29 + num49), tHeight: 29);
                this.InfraButtonId = this.AddSubPart( tsubpart, num27 + num50 * 33, num1, 29 + num49, 29, 1);
                num29 = num50 + num48;
              }
              else
              {
                let mut num51: i32 =  num29 + 1;
                if (this.game.EditObj.TutMode)
                  this.infrabuttontext = "";
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONBUILDROAD, "Repair", 1, this.infrabuttontext,  this.OwnBitmap, num27 + num51 * 33, num1, 1, 29 + num49, 29);
                this.InfraButtonId2 = this.AddSubPart( tsubpart, num27 + num51 * 33, num1, 29 + num49, 29, 0);
                num29 = num51 + num48;
              }
            }
            else if (this.InfraButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 37))
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONBUILDROAD, tDescript: "Click to build bridge [R]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.InfraButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutMode)
                this.infrabuttontext = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONBUILDROAD, 1, this.infrabuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.InfraButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[515] == 0.0)
          {
            if (this.ParadropButtonId > 0 & this.game.EditObj.TutOrder == -1)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONPARADROP, tDescript: "Use this Air Unit to do a paradrop", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.ParadropButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONPARADROP, 1, this.paradropbuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.ParadropButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[516] == 0.0)
          {
            if (this.AirSupplyButtonId > 0 & this.game.EditObj.TutOrder == -1)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONAIRSUPPLY, tDescript: "Use this Air Unit to do an airsupply", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.AirSupplyButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONAIRSUPPLY, 1, this.airsupplybuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.AirSupplyButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[507] == 0.0)
          {
            if (this.LoadButtonId > 0 & this.game.EditObj.TutOrder == -1)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONLOAD, tDescript: "Load Unit aboard this Naval Unit [L]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.LoadButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONLOAD, 1, this.loadbuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.LoadButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          if ( this.game.Data.RuleVar[507] == 0.0)
          {
            if (this.UnLoadButtonID > 0 & this.game.EditObj.TutOrder == -1)
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONUNLOAD, tDescript: "Unload this Unit [U]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 1, tsize: 29);
              this.UnLoadButtonID = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONUNLOAD, 1, this.unloadbuttontext,  this.OwnBitmap, num27 + num29 * 33, num1, 1, 29);
              this.UnLoadButtonID2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
        }
        if (num18 == 1)
        {
          if ( this.game.Data.RuleVar[458] > 0.0 & this.game.Data.Product >= 6)
          {
            let mut num52: i32 =   Math.Round(Conversion.Val( this.game.Data.RuleVar[458]));
            if (num52 > 3)
              num52 = 3;
            let mut num53: i32 =  num52 * 33;
            if (this.AttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 2))
            {
              let mut num54: i32 =  num29 + 1;
              tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONATTACK, "Regular Attack", tDescript: "Do a Land Attack on this Hex [A]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num54 * 33), bby: num1, totherback: 2, tWidth: (29 + num53), tHeight: 29);
              this.AttackButtonId = this.AddSubPart( tsubpart, num27 + num54 * 33, num1, 29 + num53, 29, 1);
              num29 = num54 + num52;
            }
            else
            {
              let mut num55: i32 =  num29 + 1;
              if (this.game.EditObj.TutOrder > -1)
                this.AttackButtonText = "";
              tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONATTACK, "Regular Attack", 1, this.AttackButtonText,  this.OwnBitmap, num27 + num55 * 33, num1, 2, 29 + num53, 29);
              this.AttackButtonId2 = this.AddSubPart( tsubpart, num27 + num55 * 33, num1, 29 + num53, 29, 0);
              num29 = num55 + num52;
            }
            if ( this.game.Data.RuleVar[953] == 0.0)
            {
              if (this.ArtAttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 11))
              {
                let mut num56: i32 =  num29 + 1;
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONARTATTACK, "Ranged Attack", tDescript: "Do an Artillery Barrage on this Hex [B]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num56 * 33), bby: num1, totherback: 2, tWidth: (29 + num53), tHeight: 29);
                this.ArtAttackButtonId = this.AddSubPart( tsubpart, num27 + num56 * 33, num1, 29 + num53, 29, 1);
                num29 = num56 + num52;
              }
              else
              {
                let mut num57: i32 =  num29 + 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.ArtAttackButtonText = "";
                tsubpart =  new MarcButtonFlexPartClass(this.game.BUTTONARTATTACK, "Ranged Attack", 1, this.ArtAttackButtonText,  this.OwnBitmap, num27 + num57 * 33, num1, 2, 29 + num53, 29);
                this.ArtAttackButtonId2 = this.AddSubPart( tsubpart, num27 + num57 * 33, num1, 29 + num53, 29, 0);
                num29 = num57 + num52;
              }
            }
            else
            {
              this.ArtAttackButtonId = 0;
              this.ArtAttackButtonId2 = 0;
            }
          }
          else
          {
            if (this.AttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 2))
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONATTACK, tDescript: "Do a Land Attack on this Hex [A]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 2, tsize: 29);
              this.AttackButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.AttackButtonText = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONATTACK, 1, this.AttackButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 2, 29);
              this.AttackButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
            if ( this.game.Data.RuleVar[953] == 0.0)
            {
              if (this.ArtAttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 11))
              {
                num29 += 1;
                tsubpart =  new MarcButtonPartClass(this.game.BUTTONARTATTACK, tDescript: "Do an Artillery Barrage on this Hex [B]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 2, tsize: 29);
                this.ArtAttackButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
              }
              else
              {
                num29 += 1;
                if (this.game.EditObj.TutOrder > -1)
                  this.ArtAttackButtonText = "";
                tsubpart =  new MarcButtonPartClass(this.game.BUTTONARTATTACK, 1, this.ArtAttackButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 2, 29);
                this.ArtAttackButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
              }
            }
            else
            {
              this.ArtAttackButtonId = 0;
              this.ArtAttackButtonId2 = 0;
            }
          }
          if ( this.game.Data.RuleVar[952] == 0.0)
          {
            if (this.AirAttackButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 14))
            {
              num29 += 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONAIRATTACK, tDescript: "Do an Airstrike on this Hex [Z]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 2, tsize: 29);
              this.AirAttackButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 += 1;
              if (this.game.EditObj.TutOrder > -1)
                this.AirAttackButtonText = "";
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONAIRATTACK, 1, this.AirAttackButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 2, 29);
              this.AirAttackButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          else
          {
            this.AirAttackButtonId = 0;
            this.AirAttackButtonId2 = 0;
          }
          if ( this.game.Data.RuleVar[511] == 0.0)
          {
            num58: i32;
            if (this.seaAttackButtonId > 0 & this.game.EditObj.TutOrder == -1)
            {
              num58 = num29 + 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONSEAATTACK, tDescript: "Do a Sea Attack on this Hex [A]", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num58 * 33), bby: num1, totherback: 2, tsize: 29);
              this.seaAttackButtonId = this.AddSubPart( tsubpart, num27 + num58 * 33, num1, 29, 29, 1);
            }
            else
            {
              num58 = num29 + 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONSEAATTACK, 1, this.SeaAttackButtonText,  this.OwnBitmap, num27 + num58 * 33, num1, 2, 29);
              this.seaAttackButtonId2 = this.AddSubPart( tsubpart, num27 + num58 * 33, num1, 29, 29, 0);
            }
            if (this.SeaArtAttackButtonId > 0 & this.game.EditObj.TutOrder == -1)
            {
              num29 = num58 + 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONSEAARTATTACK, tDescript: "Do Shorebombardment on this Hex", tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 2, tsize: 29);
              this.SeaArtAttackButtonId = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
            }
            else
            {
              num29 = num58 + 1;
              tsubpart =  new MarcButtonPartClass(this.game.BUTTONSEAARTATTACK, 1, this.SeaArtAttackButtonText,  this.OwnBitmap, num27 + num29 * 33, num1, 2, 29);
              this.SeaArtAttackButtonId2 = this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 0);
            }
          }
          else
          {
            this.SeaArtAttackButtonId = 0;
            this.SeaArtAttackButtonId2 = 0;
            this.seaAttackButtonId = 0;
            this.SeaArtAttackButtonId2 = 0;
          }
        }
        let mut index6: i32 =  0;
        let mut actionCardCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter;
        for (let mut index7: i32 =  0; index7 <= actionCardCounter; index7 += 1)
        {
          let mut index8: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index7];
          if (index6 < 10 && this.game.Data.ActionCardObj[index8].quickButton > 0 & this.game.Data.Round > 0)
          {
            bool flag = false;
            this.game.EditObj.AreaX = this.game.SelectX;
            this.game.EditObj.AreaY = this.game.SelectY;
            if (this.game.Data.ActionCardObj[index8].PPCost <= this.game.Data.RegimeObj[this.game.Data.Turn].ResPts)
            {
              if (this.game.Data.ActionCardObj[index8].PreExecuteEvent > -1)
              {
                this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.ActionCardObj[index8].PreExecuteEvent, this.game.Data.ActionCardObj[index8].TempVar0, this.game.Data.ActionCardObj[index8].TempVar1, tv3: this.game.EditObj.UnitSelected);
                if (this.game.Data.ActionCardObj[index8].UnitSelect)
                {
                  if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].TempUnitSelectable)
                    flag = true;
                }
                else if (this.game.Data.ActionCardObj[index8].AreaValue > 0 && this.game.SelectX > -1 && this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].AreaCode[this.game.Data.ActionCardObj[index8].AreaSlot] == this.game.Data.ActionCardObj[index8].AreaValue)
                  flag = true;
              }
              else
                flag = true;
            }
            if (flag)
            {
              index6 += 1;
              if (index6 == 1 & num29 < 3)
                num29 += 1;
              this.ActionButtonCardSlot[index6] = index7;
              num29 += 1;
              int[] actionButtonId = this.ActionButtonId;
              let mut index9: i32 =  index6;
              tsubpart =  new MarcButtonPartClass(this.game.Data.SmallPicNr[this.game.Data.ActionCardObj[index8].quickSmall], tDescript: this.game.Data.ActionCardObj[index8].Title, tBackbitmap: ( this.OwnBitmap), bbx: (num27 + num29 * 33), bby: num1, totherback: 2, tsize: 29);
              let mut num59: i32 =  this.AddSubPart( tsubpart, num27 + num29 * 33, num1, 29, 29, 1);
              actionButtonId[index9] = num59;
            }
            this.game.EditObj.AreaX = -1;
            this.game.EditObj.AreaY = -1;
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut index10: i32 =  0; index10 <= unitCounter; index10 += 1)
              this.game.Data.UnitObj[index10].TempUnitSelectable = false;
          }
        }
        let mut num60: i32 =  num2 + 600;
        let mut num61: i32 =  -1;
        num62: i32;
        if (this.HexUnitButtonId > 0 & this.game.EditObj.TutOrder == -1)
        {
          num62 = num61 + 1;
          if (this.game.EditObj.HideUnit == 0)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEX, tDescript: "Click to show siluet counters [1]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num62 * 33), bby: num1, totherback: 3, tsize: 29);
            this.HexUnitButtonId = this.AddSubPart( tsubpart, num60 + 36 * num62, num1, 29, 29, 1);
          }
          else if (this.game.EditObj.HideUnit == 1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEXUNIT, tDescript: "Click to show nato counters [2]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num62 * 33), bby: num1, totherback: 3, tsize: 29);
            this.HexUnitButtonId = this.AddSubPart( tsubpart, num60 + 36 * num62, num1, 29, 29, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEXUNIT2, tDescript: "Click to hide units [0]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num62 * 33), bby: num1, totherback: 3, tsize: 29);
            this.HexUnitButtonId = this.AddSubPart( tsubpart, num60 + 36 * num62, num1, 29, 29, 1);
          }
        }
        else
        {
          num62 = num61 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEX, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num62 * 33), bby: num1, totherback: 3, tsize: 29);
          this.HexUnitButtonId2 = this.AddSubPart( tsubpart, num60 + 36 * num62, num1, 29, 29, 0);
        }
        num63: i32;
        if (this.ShowAsId > 0 & this.game.EditObj.TutOrder == -1)
        {
          num63 = num62 + 1;
          if (this.game.EditObj.HideAS)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEXINFO2, tDescript: "Click to show extra hex info [4]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num63 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ShowAsId = this.AddSubPart( tsubpart, num60 + 36 * num63, num1, 29, 29, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEXINFO, tDescript: "Click to hide extra hex info [4]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num63 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ShowAsId = this.AddSubPart( tsubpart, num60 + 36 * num63, num1, 29, 29, 1);
          }
        }
        else
        {
          num63 = num62 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONHEXINFO, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num63 * 33), bby: num1, totherback: 3, tsize: 29);
          this.ShowAsId2 = this.AddSubPart( tsubpart, num60 + 36 * num63, num1, 29, 29, 0);
        }
        num64: i32;
        if (this.ButtonZoomOutId > 0 & this.game.EditObj.TutOrder == -1)
        {
          num64 = num63 + 1;
          if (this.game.EditObj.Zoom > -1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONZOOMOUT, tDescript: "Click to zoom out [-]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num64 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ButtonZoomOutId = this.AddSubPart( tsubpart, num60 + 36 * num64, num1, 29, 29, 1);
          }
          else if (this.game.EditObj.Zoom == -1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONZOOMOUT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num64 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ButtonZoomOutId2 = this.AddSubPart( tsubpart, num60 + 36 * num64, num1, 29, 29, 0);
          }
        }
        else
        {
          num64 = num63 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONZOOMOUT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num64 * 33), bby: num1, totherback: 3, tsize: 29);
          this.ButtonZoomOutId2 = this.AddSubPart( tsubpart, num60 + 36 * num64, num1, 29, 29, 0);
        }
        num65: i32;
        if (this.ButtonZoomInId > 0 & this.game.EditObj.TutOrder == -1)
        {
          num65 = num64 + 1;
          if (this.game.EditObj.Zoom < 1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONZOOMIN, tDescript: "Click to zoom in [+]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num65 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ButtonZoomInId = this.AddSubPart( tsubpart, num60 + 36 * num65, num1, 29, 29, 1);
          }
          else if (this.game.EditObj.Zoom == 1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONZOOMIN, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num65 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ButtonZoomInId2 = this.AddSubPart( tsubpart, num60 + 36 * num65, num1, 29, 29, 0);
          }
        }
        else
        {
          num65 = num64 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONZOOMIN, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num65 * 33), bby: num1, totherback: 3, tsize: 29);
          this.ButtonZoomInId2 = this.AddSubPart( tsubpart, num60 + 36 * num65, num1, 29, 29, 0);
        }
        num66: i32;
        if (this.ButtonStackedUnitId > 0 & this.game.EditObj.TutOrder == -1 & this.game.EditObj.Zoom == 1)
        {
          num66 = num65 + 1;
          if (!this.game.EditObj.SpreadUnit)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONSPREADUNIT, tDescript: "Click to spread out units in zoomed in mode [3]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num66 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ButtonStackedUnitId = this.AddSubPart( tsubpart, num60 + 36 * num66, num1, 29, 29, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONSTACKEDUNIT, tDescript: "Click to stack units in zoomed in mode [3]", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num66 * 33), bby: num1, totherback: 3, tsize: 29);
            this.ButtonStackedUnitId = this.AddSubPart( tsubpart, num60 + 36 * num66, num1, 29, 29, 1);
          }
        }
        else
        {
          num66 = num65 + 1;
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONSPREADUNIT, 1, tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num66 * 33), bby: num1, totherback: 3, tsize: 29);
          this.ButtonStackedUnitId2 = this.AddSubPart( tsubpart, num60 + 36 * num66, num1, 29, 29, 0);
        }
        if ( this.game.Data.RuleVar[523] != 0.0)
          return;
        if (this.SupplyLayerButtonId > 0 & (this.game.EditObj.TutOrder == -1 | this.game.EditObj.TutOrder == 51))
        {
          let mut num67: i32 =  num66 + 1;
          if (this.game.EditObj.LayerSupplyOn)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONSUPPLYON, tDescript: "Click to turn off Supply layer", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num67 * 33), bby: num1, totherback: 3, tsize: 29);
            this.SupplyLayerButtonId = this.AddSubPart( tsubpart, num60 + 36 * num67, num1, 29, 29, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.BUTTONSUPPLYON, tDescript: "Click to turn on supply layer", tBackbitmap: ( this.OwnBitmap), bbx: (num60 + num67 * 33), bby: num1, totherback: 3, tsize: 29);
            this.SupplyLayerButtonId = this.AddSubPart( tsubpart, num60 + 36 * num67, num1, 29, 29, 1);
          }
        }
        else
        {
          let mut num68: i32 =  num66 + 1;
          if (this.game.EditObj.TutOrder > -1)
            this.supplylayerbuttontext = "";
          tsubpart =  new MarcButtonPartClass(this.game.BUTTONSUPPLYON, 1, this.supplylayerbuttontext,  this.OwnBitmap, num60 + num68 * 33, num1, 3, 29);
          this.SupplyLayerButtonId2 = this.AddSubPart( tsubpart, num60 + 36 * num68, num1, 29, 29, 0);
        }
      }
      else
      {
        if (this.game.EditObj.udsUnitOrderMode > 0 & !this.game.EditObj.BattleTimerActive)
          this.game.EditObj.OrderType = this.game.EditObj.udsUnitOrderMode;
        if (this.game.EditObj.OrderType == 1)
          str1 = "Movement";
        if (this.game.EditObj.OrderType == 48)
          str1 = "Group Movement";
        if (this.game.EditObj.OrderType == 14)
          str1 = "Airstrike";
        if (this.game.EditObj.OrderType == 11)
          str1 = "Land Artillery Attack";
        if (this.game.EditObj.OrderType == 2)
          str1 = "Land Attack";
        if (this.game.EditObj.OrderType == 15)
          str1 = "Bombing Raid";
        if (this.game.EditObj.OrderType == 25)
          str1 = "Construction Menu";
        if (this.game.EditObj.OrderType == 24)
          str1 = "Strategic Information";
        if (this.game.EditObj.OrderType == 26)
          str1 = "History";
        if (this.game.EditObj.OrderType == 20)
          str1 = "Embark Unit";
        if (this.game.EditObj.OrderType == 8)
          str1 = "Create new Subformation/HQ";
        if (this.game.EditObj.OrderType == 7)
          str1 = "New Unit";
        if (this.game.EditObj.OrderType == 19)
          str1 = "Paradrop/Airlift";
        if (this.game.EditObj.OrderType == 22)
          str1 = "Officerpool";
        if (this.game.EditObj.OrderType == 6)
          str1 = "Production";
        if (this.game.EditObj.OrderType == 4)
          str1 = "Locations HQ";
        if (this.game.EditObj.OrderType == 5)
          str1 = "Recruitment";
        if (this.game.EditObj.OrderType == 23)
          str1 = "Decision Room";
        if (this.game.EditObj.OrderType == 52)
          str1 = "Subformation Model Design";
        if (this.game.EditObj.OrderType == 13)
          str1 = "Shore Bombardment";
        if (this.game.EditObj.OrderType == 12)
          str1 = "Sea Attack";
        if (this.game.EditObj.OrderType == 18)
          str1 = "Strategic Transfer";
        if (this.game.EditObj.OrderType == 49)
          str1 = "Group Strategic Transfer";
        if (this.game.EditObj.OrderType == 9)
          str1 = "Transfer";
        if (this.game.EditObj.OrderType == 3)
          str1 = "Set Unit's HQ";
        if (this.game.EditObj.OrderType == 21)
          str1 = "Disembark Unit";
        if (this.game.EditObj.OrderType == 30)
          str1 = "System Options";
        if (this.game.EditObj.OrderType == 33)
          str1 = "Air Recon";
        if (this.game.EditObj.OrderType == 35)
          str1 = "Blow Bridge";
        if (this.game.EditObj.OrderType == 36)
          str1 = "Build Road/Bridge";
        if (this.game.EditObj.OrderType == 37)
          str1 = "Build/Repair Location";
        if (this.game.EditObj.OrderType == 39)
          str1 = "Select Anti-Supply Target";
        if (this.game.EditObj.OrderType == 40)
          str1 = "Air Supply";
        if (this.game.EditObj.OrderType == 43)
          str1 = "AI Testing, Pick Marker for unit";
        if (this.game.EditObj.OrderType == 44)
          str1 = "Make new subunit";
        if (this.game.EditObj.OrderType == 45)
          str1 = "Officer Pool";
        if (this.game.EditObj.OrderType == 46)
          str1 = "Change Model";
        if (this.game.EditObj.OrderType == 47)
          str1 = "Model Designer";
        if (this.game.EditObj.LayerSupplyOn)
        {
          let mut layerSupplyAp: i32 =  this.game.EditObj.LayerSupplyAP;
          if ( this.game.Data.RuleVar[471] > 0.0 & this.game.Data.Product >= 6)
          {
            let mut location2: i32 =  this.game.Data.MapObj[0].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Location2;
            str5: String;
            if (location2 > -1)
            {
              if (this.game.Data.LocTypeObj[this.game.Data.LocObj[location2].Type].isSupplySource)
                str5 = "Supply flow from Supply Source (" + this.game.EditObj.OrderX.ToString() + "," + this.game.EditObj.OrderY.ToString() + "). ";
              else if (this.game.Data.LocTypeObj[this.game.Data.LocObj[location2].Type].isSupplyBase)
                str5 = "If 'Deplete' the flow from Supply Base (" + this.game.EditObj.OrderX.ToString() + "," + this.game.EditObj.OrderY.ToString() + "). ";
              else
                str5 = "Cumulative supply flow from all Supply Sources. ";
            }
            else
              str5 = "Cumulative supply flow from all Supply Sources. ";
            str1 = str5 + "Click a hex for path. Selected Hex: " + Strings.Trim(Conversion.Str( this.game.EditObj.TempSup[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY])) + "ap/" + Strings.Trim(Conversion.Str( layerSupplyAp)) + "ap";
          }
          else
            str1 = "Viewing Supply layer. Click a hex to see path from HQ. Selected Hex: " + Strings.Trim(Conversion.Str( this.game.EditObj.TempSup[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY])) + "ap/" + Strings.Trim(Conversion.Str( layerSupplyAp)) + "ap";
        }
        if (this.game.EditObj.udsUnitOrderMode == 1)
          str1 = "Movement Mode";
        if (this.game.EditObj.udsUnitOrderMode == 48)
          str1 = "Group Movement Mode";
        bool flag1 = true;
        this.NotOkText = "Option not available";
        let mut num69: i32 =  this.game.EditObj.LayerSupplyOn ? 1 : 0;
        if (this.game.EditObj.OrderType == 6)
        {
          let mut Number: i32 =  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].Type].MaxProd;
          if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].Type].StructuralPts > 0)
            Number =  Math.Round(Conversion.Int( Number * ( this.game.Data.LocObj[this.game.EditObj.OrderLoc].StructuralPts /  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].Type].StructuralPts)));
          if (this.game.Data.Round <= 0)
            ;
          str1 = "Production for " + this.game.Data.LocObj[this.game.EditObj.OrderLoc].Name + " (" + Strings.Trim(Conversion.Str( Number)) + "points)";
          if (this.game.Data.Round != 0)
            str1 = str1 + " (" + Conversion.Str( Conversion.Int(this.game.Data.PeopleObj[this.game.Data.LocObj[this.game.EditObj.OrderLoc].People].ProdMod[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.Turn].People].PeopleGroup] * 100f)) + "%)";
          if (this.game.HandyFunctionsObj.GetProdTotalPercent(this.game.EditObj.OrderLoc) > 100)
            flag1 = false;
        }
        if (this.game.EditObj.OrderType == 9)
          str1 = this.game.EditObj.OrderTarget != -1 ? "Transfer subformations" : "Select target for transfers";
        bool flag2;
        if (this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49)
        {
          if (this.game.EditObj.OrderTarget == -1)
          {
            this.PopupButtonId = 1;
            str1 = "Strategic Transfer. Select HQ to provide Movement Capacity";
            if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
              flag2 = true;
          }
          else
            str1 = "Strategic Transfer. Select destination Hex";
        }
        tDescript: String;
        if (this.game.EditObj.OrderType == 3)
        {
          this.PopupButtonId = 1;
          str1 = "Select HQ for Unit";
          if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
          {
            if (this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, true) > 0)
              str1 = str1 + ". Costs " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, false)) + " PP - " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, true)) + " PP to switch due to current commander.";
          }
          else if (this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, false) > 0)
            str1 = str1 + ". Costs " + Conversion.Str( this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(this.game.EditObj.OrderUnit, false)) + " PP due to current commander.";
          if (this.game.EditObj.UnitSelected > -1)
          {
            this.NotOkText = "Unit is not a HQ";
            if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
            {
              this.NotOkText = "You cannot select self as HQ";
              if (this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit)
              {
                this.NotOkText = "That is already the HQ";
                if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HQ != this.game.EditObj.UnitSelected)
                {
                  this.NotOkText = "Unit is not a HQ";
                  if (this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit | this.game.Data.UnitObj[this.game.EditObj.OrderUnit].IsHQ)
                  {
                    this.NotOkText = "Unit cannot be set as HQ";
                    if (this.game.HandyFunctionsObj.CanUnitBecomeHQfor(this.game.EditObj.UnitSelected, this.game.EditObj.OrderUnit))
                    {
                      flag2 = true;
                      if ( this.game.Data.RuleVar[304] > 0.0)
                      {
                        let mut num70: i32 =  0;
                        if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].IsHQ)
                          num70 = 1;
                        if ( (this.game.HandyFunctionsObj.HowmanyHQsAbove(this.game.EditObj.UnitSelected) + this.game.HandyFunctionsObj.HowmanyHQsBelow(this.game.EditObj.OrderUnit) + 1 + num70) >  this.game.Data.RuleVar[304])
                        {
                          this.NotOkText = "Cannot select as HQ because it would exceed the maximum amount of HQs in HQ chain.";
                          flag2 = false;
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          this.KillId = 1;
          tDescript = "Set Unit to No HQ";
        }
        if (this.game.EditObj.OrderType == 35)
        {
          str1 = "Select Bridge Direction to blow";
          if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] == 0)
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 36)
        {
          str1 = !(this.game.SelectX == this.game.EditObj.OrderX & this.game.SelectY == this.game.EditObj.OrderY) ? this.game.HandyFunctionsObj.GetRoadOrBridgeBuildCostString(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) : "Select Direction to build road/bridge";
          if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] == 0)
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 19 | this.game.EditObj.OrderType == 42)
        {
          if (this.game.EditObj.OrderTarget == -1)
          {
            str1 = "Select Unit to paradrop or airlift";
            if (this.game.EditObj.UnitSelected > -1 & this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit)
            {
              this.NotOkText = "Unit is empty";
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount > -1)
              {
                this.NotOkText = "Unit is a HQ. Only non-HQs can be dropped.";
                if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                {
                  this.NotOkText = "Unit must have at least 50ap";
                  if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) >= 50)
                  {
                    this.NotOkText = "Unit must be in same hex.";
                    if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X == this.game.SelectX & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y == this.game.SelectY)
                    {
                      this.NotOkText = "Unit is not yours.";
                      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                      {
                        this.NotOkText = "Unit may only have land theater subformations";
                        if (!this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected))
                        {
                          this.NotOkText = "Unit is to heavy to be carried.";
                          if (this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.OrderUnit, 2) >= this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.UnitSelected))
                            flag2 = true;
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          else if (this.game.EditObj.TargetX == -1)
          {
            str1 = this.game.EditObj.OrderType != 19 ? "Select airfield to airlift too" : "Select Hex to paradrop on";
            if (!(this.game.EditObj.OrderX == this.game.SelectX & this.game.EditObj.OrderY == this.game.SelectY & this.game.EditObj.OrderMap == this.game.EditObj.MapSelected))
            {
              this.NotOkText = "Hex cannot be reached.";
              if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] <= this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit))
              {
                this.NotOkText = "Option not available..";
                if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                {
                  this.NotOkText = "Unit needs at least " + Conversion.Str( this.game.Data.RuleVar[307]) + " power points to capture enemy territory.";
                  if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.EditObj.OrderTarget))
                  {
                    this.NotOkText = "Unit does not only have paratroopers.";
                    if (this.game.HandyFunctionsObj.HasUnitOnlyParaLandSF(this.game.EditObj.OrderTarget))
                      flag2 = true;
                  }
                }
                else
                {
                  this.NotOkText = "Not on sea";
                  if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
                  {
                    if (!this.game.HandyFunctionsObj.HasUnitOnlyParaLandSF(this.game.EditObj.OrderTarget))
                    {
                      this.NotOkText = "Must have location";
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                      {
                        this.NotOkText = "Must have location with airfield";
                        if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].IsAirfield)
                          flag2 = true;
                      }
                    }
                    else
                      flag2 = true;
                  }
                }
              }
            }
          }
        }
        if (this.game.EditObj.OrderType == 33)
        {
          str1 = "Select target hex for recon mission";
          if (!(this.game.EditObj.OrderX == this.game.SelectX & this.game.EditObj.OrderY == this.game.SelectY & this.game.EditObj.MapSelected == this.game.EditObj.OrderMap) && this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] <= this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit))
            flag2 = true;
        }
        if (this.game.EditObj.OrderType != 40)
          this.LastAirSupplyTarget = -1;
        if (this.game.EditObj.OrderType == 40)
        {
          str1 = "Select target unit for air supply.";
          if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HQ > -1 & this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit) >= 40)
          {
            let mut supply: i32 =  this.game.Data.UnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HQ].Supply;
            let mut Number1: i32 =   Math.Round( Conversion.Int( this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.OrderUnit, 2) / this.game.Data.RuleVar[33]));
            if (Number1 > supply)
              Number1 = supply;
            str6: String = (this.game.EditObj.TempValue2[0].Value[this.game.SelectX, this.game.SelectY] >= 9999 ? str1 + " Ap: out of range" : str1 + " Ap: " + this.game.EditObj.TempValue2[0].Value[this.game.SelectX, this.game.SelectY].ToString()) + ", Max: " + Strings.Trim(Conversion.Str( Number1)) + ".";
            if (this.game.EditObj.UnitSelected > -1)
            {
              if (this.LastAirSupplyTarget == this.game.EditObj.UnitSelected)
              {
                str1 = str6 + ", target area needs: " + Strings.Trim(Conversion.Str( this.LastAirSupplyNeed));
                let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
                for (let mut index11: i32 =  0; index11 <= mapWidth; index11 += 1)
                {
                  let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                  for (let mut index12: i32 =  0; index12 <= mapHeight; index12 += 1)
                    this.game.EditObj.TempValue[0].Value[index11, index12] = this.game.EditObj.TempValue2[0].Value[index11, index12];
                }
              }
              else
              {
                this.game.HandyFunctionsObj.RedimTempValue3(9999);
                let mut Number2: i32 =  this.game.HandyFunctionsObj.AirSupplyNeeded(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0);
                let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
                for (let mut index13: i32 =  0; index13 <= mapWidth; index13 += 1)
                {
                  let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                  for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
                    this.game.EditObj.TempValue[0].Value[index13, index14] = this.game.EditObj.TempValue2[0].Value[index13, index14];
                }
                str1 = str6 + ", target area needs: " + Strings.Trim(Conversion.Str( Number2));
                this.LastAirSupplyMax = Number1;
                this.LastAirSupplyNeed = Number2;
                this.LastAirSupplyTarget = this.game.EditObj.UnitSelected;
              }
            }
            else
            {
              str1 = str6 + ", no target unit selected";
              this.LastAirSupplyTarget = -1;
              this.game.HandyFunctionsObj.RedimTempValue3(9999);
            }
          }
          else
          {
            this.LastAirSupplyTarget = -1;
            this.game.HandyFunctionsObj.RedimTempValue3(9999);
          }
          if (!(this.game.EditObj.OrderX == this.game.SelectX & this.game.EditObj.OrderY == this.game.SelectY & this.game.EditObj.MapSelected == this.game.EditObj.OrderMap) && this.game.EditObj.TempValue2[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] <= this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.OrderUnit) && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn && this.game.EditObj.TargetX == -1 & this.LastAirSupplyMax > 0 & this.LastAirSupplyNeed > 0)
            flag2 = true;
          this.game.EditObj.TempCoordList = CoordList::new();
        }
        Coordinate target;
        bool flag3;
        if (this.game.EditObj.OrderType == 2)
        {
          this.PopupButtonId = 1;
          let mut unitCounter: i32 =  this.game.Data.MapObj[0].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].UnitCounter;
          num71: i32;
          for (let mut index15: i32 =  0; index15 <= unitCounter; index15 += 1)
          {
            let mut unit: i32 =  this.game.Data.MapObj[0].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].UnitList[index15];
            num71 += this.game.HandyFunctionsObj.GetStackWithFOW(unit, this.game.Data.Turn);
          }
          str7: String = "Select attackers. Stack: " + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStack() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStack(this.game.Data.Turn))) + " /" + Conversion.Str( this.game.HandyFunctionsObj.maxAttackStack());
          if (num71 > 0)
            str7 = str7 + " vs " + num71.ToString();
          str1 = str7 + ". Concentric: +" + Conversion.Str( Conversion.Int( (( this.game.HandyFunctionsObj.GetConcentricBonus2() - 1.0) * 100.0))) + "%";
          let mut divBonusForAttack: i32 =  this.game.HandyFunctionsObj.GetDivBonusForAttack(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.TargetMap);
          if (divBonusForAttack > 0)
            str1 = str1 + ". Divisional: +" + Conversion.Str( Conversion.Int(divBonusForAttack)) + "%";
          if (this.game.EditObj.UnitSelected > -1)
          {
            target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            target.map = this.game.EditObj.MapSelected;
            this.NotOkText = "Unit needs at least " + Conversion.Str( this.game.Data.RuleVar[307]) + " power points to join in attack.";
            if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.EditObj.UnitSelected))
            {
              this.NotOkText = "Unit does not have enough AP to join in attack.";
              if (this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, target.x, target.y, target.map) <= 1)
              {
                let mut x: i32 =  this.game.HandyFunctionsObj.MoveApCostPreview(this.game.EditObj.UnitSelected, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, target.x, target.y, target.map, true).x;
                this.NotOkText = x < 9999 ? "Unit does not have the " + x.ToString() + " AP needed to join in attack." : "Unit can never join in this attack. Landscape or river might be impassible for it.";
              }
              if (this.game.HandyFunctionsObj.CanDoLandAttack(this.game.EditObj.UnitSelected, target))
                flag2 = true;
            }
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 14)
        {
          this.PopupButtonId = 1;
          str1 = "Preparing Air Strike. Select attackers. ";
          if ( this.game.Data.RuleVar[833] > 0.0)
            str1 = str1 + "Stack:" + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackAir() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackAir(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[833]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            if (this.game.HandyFunctionsObj.CanDoAirStrike(this.game.EditObj.UnitSelected, target))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48)
        {
          if (this.game.EditObj.OrderUnit == -1)
            str1 += " - Please select a Unit";
          else if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.udsUnitOrderMode > 0 & this.game.EditObj.useLeftRightClickMode)
          {
            bool flag4 = false;
            bool flag5 = false;
            if (this.game.EditObj.MouseOverX > -1)
            {
              let mut index16: i32 =  0;
              do
              {
                if (this.game.EditObj.TempAttack[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, index16])
                {
                  flag4 = true;
                  Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, 0, index16 + 1);
                  if (!this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y].onmap)
                    flag5 = true;
                }
                index16 += 1;
              }
              while (index16 <= 5);
              str1 = !flag4 ? ( this.game.Data.RuleVar[419] <= 0.0 ? str1 + " - AP Cost: " + this.game.EditObj.CurrentDescript : (this.game.EditObj.TempLos[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] <= 0 ? str1 + " - AP Cost: " + this.game.EditObj.CurrentDescript : str1 + " - Right Click to do Ranged Attack")) : (!flag5 ? (!( this.game.Data.RuleVar[419] > 0.0 & this.game.EditObj.TempLos[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] > 0) ? str1 + " - Move next to target todo Regular Attack" : str1 + " - Right Click to do Ranged Attack") : str1 + " - Right Click to Attack");
              if (this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, 0, this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, 0) == 1 && Strings.InStr(str1, "AP") < 1)
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.MoveApCostPreview(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, 0, this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY, 0, true, ismove: true);
                str1 = str1 + " - AP Cost: " + coordinate.x.ToString();
              }
            }
          }
          else
            str1 = str1 + " - AP Cost: " + this.game.EditObj.CurrentDescript;
        }
        if (this.game.EditObj.OrderType == 15)
        {
          this.PopupButtonId = 1;
          str1 = "Preparing Bombing Run. Select participants." + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackAir() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackAir(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[833]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            if (this.game.HandyFunctionsObj.CanDoAirStrike(this.game.EditObj.UnitSelected, target))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 11)
        {
          str1 = "Preparing Art Attack. Select attackers. ";
          this.PopupButtonId = 1;
          if ( this.game.Data.RuleVar[834] > 0.0)
            str1 = str1 + "Stack" + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackart() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackArt(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[834]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            if (this.game.HandyFunctionsObj.CanDoArtAttack(this.game.EditObj.UnitSelected, target, false))
              flag2 = true;
            else if ( this.game.Data.RuleVar[419] > 0.0 && this.game.HandyFunctionsObj.CanDoDirectAttack(this.game.EditObj.UnitSelected, target, false))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 13)
        {
          str1 = "Preparing Shore Bombardment. Select participants. " + Conversion.Str( (this.game.HandyFunctionsObj.CurrentAttackStackart() + this.game.Data.MapObj[this.game.EditObj.TargetMap].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_BattleStackArt(this.game.Data.Turn))) + "/" + Conversion.Str( this.game.Data.RuleVar[834]);
          if (this.game.EditObj.UnitSelected > -1)
          {
            target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            if (this.game.HandyFunctionsObj.CanDoSeaArtAttack(this.game.EditObj.UnitSelected, target))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 12)
        {
          str1 = "Preparing Sea Attack. Select participants.";
          if (this.game.EditObj.UnitSelected > -1)
          {
            target = Coordinate::new();
            target.x = this.game.EditObj.TargetX;
            target.y = this.game.EditObj.TargetY;
            if (this.game.HandyFunctionsObj.CanDoSeaAttack(this.game.EditObj.UnitSelected, target))
              flag2 = true;
          }
          this.AllId = 1;
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            flag3 = true;
            this.NoneId = 1;
          }
        }
        if (this.game.EditObj.OrderType == 4)
        {
          str1 = "Select a HQ for " + this.game.Data.LocObj[this.game.EditObj.OrderLoc].Name;
          if (this.game.EditObj.UnitSelected > -1 && this.game.HandyFunctionsObj.CanUnitBecomeHQforLoc(this.game.EditObj.UnitSelected, this.game.EditObj.OrderLoc))
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 20)
        {
          str1 = "Select cargo for " + this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Name;
          if (this.game.EditObj.UnitSelected > -1 && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit)
          {
            this.NotOkText = "Unit already has 8 subformations.";
            if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].SFCount + this.game.Data.UnitObj[this.game.EditObj.OrderUnit].PassengerCounter + 1 < 7)
            {
              this.NotOkText = "Unit is empty.";
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount > -1)
              {
                this.NotOkText = "Unit is not yours";
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
                {
                  let mut num72: i32 =  0;
                  if (this.game.HandyFunctionsObj.IsHexPort(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap))
                  {
                    if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) > 0)
                    {
                      num72 = 1;
                      this.NotOkText = "Unit is not in port.";
                    }
                  }
                  else
                  {
                    if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) > 1)
                    {
                      num72 = 1;
                      this.NotOkText = "Unit is not next to naval unit.";
                    }
                    if (((ulong) -(this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) <= 0 ? 1 : 0) & (ulong) (long) Math.Round( this.game.Data.RuleVar[882])) > 0UL)
                    {
                      num72 = 1;
                      this.NotOkText = "Unit needs more then 0 ap in order to be able to get of the beaches.";
                    }
                  }
                  if (num72 == 0)
                  {
                    this.NotOkText = "Unit may not have naval or aerial subformations.";
                    if (!this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.UnitSelected) & !this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
                    {
                      this.NotOkText = "Unit is to heavy.";
                      if (this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.UnitSelected, true) <= this.game.HandyFunctionsObj.GetUnitCarryCap(this.game.EditObj.OrderUnit, 1, true))
                        flag2 = true;
                    }
                  }
                }
              }
            }
          }
        }
        if (this.game.EditObj.OrderType == 21)
        {
          str1 = "Select disembark hex for " + this.game.Data.UnitObj[this.game.EditObj.OrderTarget].Name;
          if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
          {
            if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) == 0)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter < 15)
                flag2 = true;
            }
            else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) <= 1)
            {
              this.NotOkText = "Landscape not suitable";
              if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].CanAmph && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].IsSea)
              {
                if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                {
                  this.NotOkText = "Unit needs at least " + Conversion.Str( this.game.Data.RuleVar[307]) + " power points to do amphibious invasion.";
                  if ( this.game.Data.RuleVar[307] <=  this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.EditObj.OrderTarget))
                    flag2 = true;
                }
                else if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime, this.game.Data.Turn) && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter < 15)
                  flag2 = true;
              }
            }
          }
        }
        if (!this.game.AIThreadRunning && this.game.EditObj.BattleTimerActive)
        {
          tstring: String = "BATTLE IS COMMENCING!!";
          DrawMod.DrawTextColouredMarc( g, tstring, this.game.MarcFont3, num2 + 82, num1 + 3, Color.White);
        }
        else if (this.game.EditObj.udsUnitOrderMode > 0)
        {
          let mut num73: i32 =  0;
          if (!( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode))
          {
            if ( this.game.Data.RuleVar[458] > 0.0 & this.game.Data.Product >= 6)
            {
              let mut num74: i32 =   Math.Round(Conversion.Val( this.game.Data.RuleVar[458]));
              if (num74 > 3)
                num74 = 3;
              num73 = (num74 + 1) * 33;
              if (this.game.EditObj.udsUnitOrderMode == 1)
              {
                if (flag1)
                {
                  tsubpart =  new MarcButtonFlexPartClass(this.game.BACKBUTTON, "Exit Move Mode", tDescript: "Exit Move Mode [Esc]", tBackbitmap: ( this.OwnBitmap), bbx: (num2 + 42), bby: (num1 - 3), tWidth: (35 + num73));
                  this.Cancelid = this.AddSubPart( tsubpart, num2 + 42, num1 - 3, 35 + num73, 35, 1);
                }
              }
              else if (this.game.EditObj.udsUnitOrderMode == 48 && flag1)
              {
                tsubpart =  new MarcButtonFlexPartClass(this.game.BACKBUTTON, "Exit Group Move", tDescript: "Exit Group Move Mode [Esc]", tBackbitmap: ( this.OwnBitmap), bbx: (num2 + 42), bby: (num1 - 3), tWidth: (35 + num73));
                this.Cancelid = this.AddSubPart( tsubpart, num2 + 42, num1 - 3, 35 + num73, 35, 1);
              }
            }
            else if (flag1)
            {
              tsubpart =  new MarcButtonPartClass(this.game.BACKBUTTON, tDescript: "Exit Move Mode", tBackbitmap: ( this.OwnBitmap), bbx: (num2 + 42), bby: num1);
              this.Cancelid = this.AddSubPart( tsubpart, num2 + 42, num1, 32, 32, 1);
            }
          }
          DrawMod.DrawTextColouredMarcCenter( g, str1, this.game.MarcFont3,  Math.Round( num73 / 2.0) +  Math.Round( this.w / 2.0), num1 + 3, Color.White);
        }
        else
        {
          if (this.game.EditObj.udsUnitOrderMode >= 1)
            return;
          if (this.AllId > 0 & this.lastorderx == this.game.EditObj.OrderX & this.lastordery == this.game.EditObj.OrderY)
            this.AllId = 0;
          let mut num75: i32 =  num2 + 1024 - 300;
          if (flag1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.BACKBUTTON, tDescript: "Back to Main Menu [ESC]", tBackbitmap: ( this.OwnBitmap), bbx: (num2 + 42), bby: num1);
            this.Cancelid = this.AddSubPart( tsubpart, num2 + 42, num1, 32, 32, 1);
          }
          DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont3, num2 + 82, num1 + 3, Color.White);
          if (flag2)
          {
            tsubpart =  new MarcButtonPartClass(this.game.OKBALL, tDescript: "Select / Deselect [SPACE]", tBackbitmap: ( this.OwnBitmap), bbx: num75, bby: num1);
            this.OkId = this.AddSubPart( tsubpart, num75, num1, 32, 32, 1);
          }
          else if (!this.game.EditObj.LayerSupplyOn)
          {
            tsubpart =  new MarcButtonPartClass(this.game.OKBALL, 1, this.NotOkText,  this.OwnBitmap, num75, num1);
            this.Ok2Id = this.AddSubPart( tsubpart, num75, num1, 32, 32, 0);
          }
          let mut num76: i32 =  num75 + 40;
          if (this.PopupButtonId > 0)
          {
            tsubpart =  new TextButtonPartClass("LIST", 48, "Select from list",  this.OwnBitmap, num76, num1, tMarcStyle: true);
            this.PopupButtonId = this.AddSubPart( tsubpart, num76, num1, 48, 32, 1);
            num76 += 50;
          }
          if (this.KillId == 1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.CANCELBALL, tDescript: tDescript, tBackbitmap: ( this.OwnBitmap), bbx: num76, bby: num1, totherback: 3);
            this.KillId = this.AddSubPart( tsubpart, num76, num1, 32, 32, 1);
            num76 += 50;
          }
          if (!(this.game.EditObj.OrderType == 2 | this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 13 | this.game.EditObj.OrderType == 12 | this.game.EditObj.OrderType == 15))
            return;
          if (flag3)
          {
            tsubpart =  new TextButtonPartClass("ATTACK", 96, "Start Battle!!",  this.OwnBitmap, num76, num1, tMarcStyle: true);
            this.BattleId = this.AddSubPart( tsubpart, num76, num1, 96, 32, 1);
          }
          else
          {
            tsubpart =  new TextButtonPartClass("ATTACK", 96, "You cannot start a battle",  this.OwnBitmap, num76, num1, true, tMarcStyle: true);
            this.Battle2Id = this.AddSubPart( tsubpart, num76, num1, 96, 32, 1);
          }
          let mut num77: i32 =  num76 + 104;
          if (this.NoneId == 1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.NONEBUTTON, tDescript: "Remove all selected units from attack", tBackbitmap: ( this.OwnBitmap), bbx: num77, bby: (num1 + 2), totherback: 3);
            this.NoneId = this.AddSubPart( tsubpart, num77, num1 + 2, 32, 32, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.NONEBUTTON, 1, "Option not available",  this.OwnBitmap, num77, num1 + 2, 3);
            this.None2Id = this.AddSubPart( tsubpart, num77, num1 + 2, 32, 32, 0);
          }
          let mut num78: i32 =  num77 + 40;
          if (this.AllId == 1)
          {
            tsubpart =  new MarcButtonPartClass(this.game.ALLBUTTON, tDescript: "Select all eligble units to join attack", tBackbitmap: ( this.OwnBitmap), bbx: num78, bby: (num1 + 2), totherback: 3);
            this.AllId = this.AddSubPart( tsubpart, num78, num1 + 2, 32, 32, 1);
          }
          else
          {
            tsubpart =  new MarcButtonPartClass(this.game.ALLBUTTON, 1, "Option not available",  this.OwnBitmap, num78, num1 + 2, 3);
            this.All2Id = this.AddSubPart( tsubpart, num78, num1 + 2, 32, 32, 0);
          }
        }
      }
    }

    pub fn DoTabs( Graphics g)
    {
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
        return;
      SizeF sizeF1 = SizeF::new();
      bool flag1 = false;
      if (this.game.Data.Product > 5 &&  this.game.Data.RuleVar[461] > 0.0 && this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Location2 > -1)
      {
        if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Location2].Type].isSupplyBase)
          flag1 = true;
        if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Location2].Type].isSupplySource)
          flag1 = true;
      }
      this.tab1 = -1;
      this.tab2 = -1;
      this.tab3 = -1;
      this.tab4 = -1;
      this.tab51 = -1;
      this.tab52 = -1;
      this.tab53 = -1;
      this.tab6 = -1;
      this.tab101 = -1;
      if (this.game.EditObj.GuiDown)
        return;
      num1: i32;
      if (this.game.EditObj.UnitSelected == -1)
      {
        if (flag1)
        {
          let mut width: i32 =  182;
          let mut num2: i32 =  180;
          if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
            num2 += 240;
          let mut x1: i32 =   Math.Round( num2 +  this.game.ScreenWidth / 2.0 - 480.0) - (width - 0);
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
           let mut local2: &Bitmap = &bitmap;
          let mut x2: i32 =  x1;
          let mut w: i32 =  width;
          DrawMod.DrawScaledColorized( local1,  local2, x2, 66, w, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
          str: String = "SUPPLY BASE";
          SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16,  Math.Round( x1 +  width / 2.0 -  sizeF2.Width / 2.0), 70, Color.White);
          Rectangle trect = Rectangle::new(x1, 66, width, 24);
          this.AddMouse( trect, "", "Information on the Supply Base in this Hex");
        }
        else
        {
          if (this.game.Data.ExtraTabName.Length <= 0)
            return;
          let mut width: i32 =  182;
          let mut num3: i32 =  180;
          let mut num4: i32 =  1;
          if (this.game.Data.ExtraTabName2.Length > 0)
          {
            width = 140;
            num3 = 260;
            num4 = 2;
          }
          if (this.game.Data.ExtraTabName3.Length > 0)
          {
            width = 112;
            num3 = 300;
            num4 = 3;
          }
          if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
            num3 += 240;
          let mut num5: i32 =  num3 - width;
          if (this.game.EditObj.SetViewModeExtraNr == 0)
          {
            let mut x3: i32 =   Math.Round( num5 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            bitmap: Bitmap;
            Rectangle rectangle;
            if (this.game.Data.ExtraTabName3.Length > 0)
            {
              x3 -= width - 12;
               let mut local3: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local4: &Bitmap = &bitmap;
              let mut x4: i32 =  x3;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local3,  local4, x4, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName3.ToUpper();
              SizeF sizeF3 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x3 +  width / 2.0 -  sizeF3.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x3, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 53);
              this.tab53 = this.MouseCounter;
            }
            if (this.game.Data.ExtraTabName2.Length > 0)
            {
              x3 -= width - 12;
               let mut local5: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local6: &Bitmap = &bitmap;
              let mut x5: i32 =  x3;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local5,  local6, x5, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName2.ToUpper();
              SizeF sizeF4 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x3 +  width / 2.0 -  sizeF4.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x3, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 52);
              this.tab52 = this.MouseCounter;
            }
            let mut x6: i32 =  x3 - (width - 12);
             let mut local7: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local8: &Bitmap = &bitmap;
            let mut x7: i32 =  x6;
            let mut w1: i32 =  width;
            DrawMod.DrawScaledColorized( local7,  local8, x7, 66, w1, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            upper1: String = this.game.Data.ExtraTabName.ToUpper();
            SizeF sizeF5 = g.MeasureString(upper1, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper1, this.game.MarcFont16,  Math.Round( x6 +  width / 2.0 -  sizeF5.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x6, 66, width, 24);
            let mut trect1: &Rectangle = &rectangle
            this.AddMouse( trect1, "", "Extra data sheet.", 51);
            this.tab51 = this.MouseCounter;
          }
          else
          {
            let mut x8: i32 =   Math.Round( num5 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            bitmap: Bitmap;
            upper: String;
            Rectangle rectangle;
            if (this.game.Data.ExtraTabName3.Length > 0 & this.game.EditObj.SetViewModeExtraNr != 3)
            {
              x8 -= width - 12;
               let mut local9: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local10: &Bitmap = &bitmap;
              let mut x9: i32 =  x8;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local9,  local10, x9, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper = this.game.Data.ExtraTabName3.ToUpper();
              SizeF sizeF6 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x8 +  width / 2.0 -  sizeF6.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x8, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 53);
              this.tab53 = this.MouseCounter;
            }
            else if (this.game.Data.ExtraTabName3.Length > 0)
              x8 -= width - 12;
            if (this.game.Data.ExtraTabName2.Length > 0 & this.game.EditObj.SetViewModeExtraNr != 2)
            {
              x8 -= width - 12;
               let mut local11: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local12: &Bitmap = &bitmap;
              let mut x10: i32 =  x8;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local11,  local12, x10, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper = this.game.Data.ExtraTabName2.ToUpper();
              SizeF sizeF7 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x8 +  width / 2.0 -  sizeF7.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x8, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 52);
              this.tab52 = this.MouseCounter;
            }
            else if (this.game.Data.ExtraTabName2.Length > 0)
              x8 -= width - 12;
            if (this.game.Data.ExtraTabName.Length > 0 & this.game.EditObj.SetViewModeExtraNr != 1)
            {
              let mut x11: i32 =  x8 - (width - 12);
               let mut local13: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local14: &Bitmap = &bitmap;
              let mut x12: i32 =  x11;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local13,  local14, x12, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper = this.game.Data.ExtraTabName.ToUpper();
              SizeF sizeF8 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x11 +  width / 2.0 -  sizeF8.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x11, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 51);
              this.tab51 = this.MouseCounter;
            }
            else if (this.game.Data.ExtraTabName.Length > 0)
              num1 = x8 - (width - 12);
            let mut x13: i32 =   Math.Round( num5 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            if (this.game.EditObj.SetViewModeExtraNr == 1)
            {
              upper = this.game.Data.ExtraTabName.ToUpper();
              x13 -= (width - 12) * (num4 - 0);
            }
            if (this.game.EditObj.SetViewModeExtraNr == 2)
            {
              upper = this.game.Data.ExtraTabName2.ToUpper();
              x13 -= (width - 12) * (num4 - 1);
            }
            if (this.game.EditObj.SetViewModeExtraNr == 3)
            {
              upper = this.game.Data.ExtraTabName3.ToUpper();
              x13 -= (width - 12) * (num4 - 2);
            }
             let mut local15: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local16: &Bitmap = &bitmap;
            let mut x14: i32 =  x13;
            let mut w2: i32 =  width;
            DrawMod.DrawScaledColorized( local15,  local16, x14, 66, w2, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            SizeF sizeF9 = g.MeasureString(upper, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x13 +  width / 2.0 -  sizeF9.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x13, 66, width, 24);
            let mut trect2: &Rectangle = &rectangle
            this.AddMouse( trect2, "", "Extra data sheet.", 50 + this.game.EditObj.SetViewModeExtraNr);
            if (this.game.EditObj.SetViewModeExtraNr == 1)
              this.tab51 = this.MouseCounter;
            if (this.game.EditObj.SetViewModeExtraNr == 2)
              this.tab52 = this.MouseCounter;
            if (this.game.EditObj.SetViewModeExtraNr != 3)
              return;
            this.tab53 = this.MouseCounter;
          }
        }
      }
      else
      {
        if (this.game.EditObj.UnitSelected <= -1)
          return;
        object obj =  true;
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1)
        {
          if (Information.IsNothing( this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].CommanderName))
            obj =  false;
          else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].CommanderName.Length < 1)
            obj =  false;
          if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
            ;
        }
        else
          obj =  false;
        if (this.game.EditObj.OrderType == 18 || this.game.EditObj.OrderType == 7 || this.game.EditObj.OrderType == 44 || this.game.EditObj.OrderType == 45 || this.game.EditObj.OrderType == 46 || this.game.EditObj.OrderType == 49)
          return;
        if (flag1)
        {
          let mut width: i32 =  182;
          let mut num6: i32 =  180;
          if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
            num6 += 120;
          let mut num7: i32 =   Math.Round( num6 +  this.game.ScreenWidth / 2.0 - 480.0);
          if (this.game.EditObj.SetViewModeExtraNr == 0)
          {
            let mut x15: i32 =  num7 + (width - 34) - (width - 24);
             let mut local17: &Graphics = &g;
            bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local18: &Bitmap = &bitmap1;
            let mut x16: i32 =  x15;
            let mut w3: i32 =  width;
            DrawMod.DrawScaledColorized( local17,  local18, x16, 66, w3, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            str1: String = "SUPPLY BASE";
            SizeF sizeF10 = g.MeasureString(str1, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str1, this.game.MarcFont16,  Math.Round( x15 +  width / 2.0 -  sizeF10.Width / 2.0), 70, Color.White);
            Rectangle rectangle = Rectangle::new(x15, 66, width, 24);
            let mut trect3: &Rectangle = &rectangle
            this.AddMouse( trect3, "", "Information on the Supply Base in this Hex", 101);
            this.tab101 = this.MouseCounter;
            let mut x17: i32 =  x15 - (width - 12);
             let mut local19: &Graphics = &g;
            bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local20: &Bitmap = &bitmap2;
            let mut x18: i32 =  x17;
            let mut w4: i32 =  width;
            DrawMod.DrawScaledColorized( local19,  local20, x18, 66, w4, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            str2: String = "UNIT BASE INFO";
            SizeF sizeF11 = g.MeasureString(str2, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str2, this.game.MarcFont16,  Math.Round( x17 +  width / 2.0 -  sizeF11.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x17, 66, width, 24);
            let mut trect4: &Rectangle = &rectangle
            this.AddMouse( trect4, "", "The base info of the unit is always shown.");
          }
          else
          {
            let mut x19: i32 =  num7 - (width - 0);
             let mut local21: &Graphics = &g;
            bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local22: &Bitmap = &bitmap;
            let mut x20: i32 =  x19;
            let mut w5: i32 =  width;
            DrawMod.DrawScaledColorized( local21,  local22, x20, 66, w5, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            str3: String = "UNIT BASE INFO";
            SizeF sizeF12 = g.MeasureString(str3, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str3, this.game.MarcFont16,  Math.Round( x19 +  width / 2.0 -  sizeF12.Width / 2.0), 70, Color.White);
            Rectangle rectangle = Rectangle::new(x19, 66, width, 24);
            let mut trect5: &Rectangle = &rectangle
            this.AddMouse( trect5, "", "The base info of the unit is always shown.", 6);
            this.tab6 = this.MouseCounter;
            let mut x21: i32 =  x19 + (width - 12);
             let mut local23: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local24: &Bitmap = &bitmap;
            let mut x22: i32 =  x21;
            let mut w6: i32 =  width;
            DrawMod.DrawScaledColorized( local23,  local24, x22, 66, w6, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            str4: String = "SUPPLY BASE";
            SizeF sizeF13 = g.MeasureString(str4, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str4, this.game.MarcFont16,  Math.Round( x21 +  width / 2.0 -  sizeF13.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x21, 66, width, 24);
            let mut trect6: &Rectangle = &rectangle
            this.AddMouse( trect6, "", "Information on the Supply Base in this Hex");
          }
        }
        else if (this.game.Data.ExtraTabName.Length > 0)
        {
          let mut width: i32 =  182;
          let mut num8: i32 =  180;
          let mut num9: i32 =  1;
          if (this.game.Data.ExtraTabName2.Length > 0)
          {
            width = 140;
            num8 = 260;
            num9 = 2;
          }
          if (this.game.Data.ExtraTabName3.Length > 0)
          {
            width = 112;
            num8 = 300;
            num9 = 3;
          }
          if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
            num8 += 120;
          if (this.game.EditObj.SetViewModeExtraNr == 0)
          {
            let mut x23: i32 =   Math.Round( num8 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            bitmap: Bitmap;
            Rectangle rectangle;
            if (this.game.Data.ExtraTabName3.Length > 0)
            {
              x23 -= width - 12;
               let mut local25: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local26: &Bitmap = &bitmap;
              let mut x24: i32 =  x23;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local25,  local26, x24, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName3.ToUpper();
              SizeF sizeF14 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x23 +  width / 2.0 -  sizeF14.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x23, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 53);
              this.tab53 = this.MouseCounter;
            }
            if (this.game.Data.ExtraTabName2.Length > 0)
            {
              x23 -= width - 12;
               let mut local27: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local28: &Bitmap = &bitmap;
              let mut x25: i32 =  x23;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local27,  local28, x25, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName2.ToUpper();
              SizeF sizeF15 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x23 +  width / 2.0 -  sizeF15.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x23, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 52);
              this.tab52 = this.MouseCounter;
            }
            let mut x26: i32 =  x23 - (width - 12);
             let mut local29: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local30: &Bitmap = &bitmap;
            let mut x27: i32 =  x26;
            let mut w7: i32 =  width;
            DrawMod.DrawScaledColorized( local29,  local30, x27, 66, w7, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            upper2: String = this.game.Data.ExtraTabName.ToUpper();
            SizeF sizeF16 = g.MeasureString(upper2, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, upper2, this.game.MarcFont16,  Math.Round( x26 +  width / 2.0 -  sizeF16.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x26, 66, width, 24);
            let mut trect7: &Rectangle = &rectangle
            this.AddMouse( trect7, "", "Extra data sheet.", 51);
            this.tab51 = this.MouseCounter;
            let mut x28: i32 =  x26 - (width - 12);
             let mut local31: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local32: &Bitmap = &bitmap;
            let mut x29: i32 =  x28;
            let mut w8: i32 =  width;
            DrawMod.DrawScaledColorized( local31,  local32, x29, 66, w8, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            str: String = "UNIT INFO";
            SizeF sizeF17 = g.MeasureString(str, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16,  Math.Round( x28 +  width / 2.0 -  sizeF17.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x28, 66, width, 24);
            let mut trect8: &Rectangle = &rectangle
            this.AddMouse( trect8, "", "The base info of the unit is always shown.");
          }
          else
          {
            let mut x30: i32 =   Math.Round( num8 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            bitmap: Bitmap;
            Rectangle rectangle;
            if (this.game.Data.ExtraTabName3.Length > 0 & this.game.EditObj.SetViewModeExtraNr != 3)
            {
              x30 -= width - 12;
               let mut local33: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local34: &Bitmap = &bitmap;
              let mut x31: i32 =  x30;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local33,  local34, x31, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName3.ToUpper();
              SizeF sizeF18 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x30 +  width / 2.0 -  sizeF18.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x30, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 53);
              this.tab53 = this.MouseCounter;
            }
            else if (this.game.Data.ExtraTabName3.Length > 0)
              x30 -= width - 12;
            if (this.game.Data.ExtraTabName2.Length > 0 & this.game.EditObj.SetViewModeExtraNr != 2)
            {
              x30 -= width - 12;
               let mut local35: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local36: &Bitmap = &bitmap;
              let mut x32: i32 =  x30;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local35,  local36, x32, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName2.ToUpper();
              SizeF sizeF19 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x30 +  width / 2.0 -  sizeF19.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x30, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 52);
              this.tab52 = this.MouseCounter;
            }
            else if (this.game.Data.ExtraTabName2.Length > 0)
              x30 -= width - 12;
            if (this.game.Data.ExtraTabName.Length > 0 & this.game.EditObj.SetViewModeExtraNr != 1)
            {
              let mut x33: i32 =  x30 - (width - 12);
               let mut local37: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
               let mut local38: &Bitmap = &bitmap;
              let mut x34: i32 =  x33;
              let mut w: i32 =  width;
              DrawMod.DrawScaledColorized( local37,  local38, x34, 66, w, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
              upper: String = this.game.Data.ExtraTabName.ToUpper();
              SizeF sizeF20 = g.MeasureString(upper, this.game.MarcFont16);
              DrawMod.DrawTextColouredMarc( g, upper, this.game.MarcFont16,  Math.Round( x33 +  width / 2.0 -  sizeF20.Width / 2.0), 70, Color.White);
              rectangle = Rectangle::new(x33, 66, width, 24);
              let mut trect: &Rectangle = &rectangle
              this.AddMouse( trect, "", "Extra data sheet.", 51);
              this.tab51 = this.MouseCounter;
            }
            else if (this.game.Data.ExtraTabName.Length > 0)
              num1 = x30 - (width - 12);
            let mut num10: i32 =   Math.Round( num8 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            if (this.game.Data.ExtraTabName3.Length > 0)
              num10 -= width - 12;
            if (this.game.Data.ExtraTabName2.Length > 0)
              num10 -= width - 12;
            if (this.game.Data.ExtraTabName.Length > 0)
              num10 -= width - 12;
            let mut x35: i32 =  num10 - (width - 12);
             let mut local39: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local40: &Bitmap = &bitmap;
            let mut x36: i32 =  x35;
            let mut w9: i32 =  width;
            DrawMod.DrawScaledColorized( local39,  local40, x36, 66, w9, 24, 182, 24, -0.1f, -0.1f, -0.1f, 1f);
            str: String = "UNIT INFO";
            SizeF sizeF21 = g.MeasureString(str, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16,  Math.Round( x35 +  width / 2.0 -  sizeF21.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x35, 66, width, 24);
            let mut trect9: &Rectangle = &rectangle
            this.AddMouse( trect9, "", "The base info of the unit is always shown.", 6);
            this.tab6 = this.MouseCounter;
            let mut x37: i32 =   Math.Round( num8 +  this.game.ScreenWidth / 2.0 - 480.0) + (width - 12);
            if (this.game.EditObj.SetViewModeExtraNr == 1)
            {
              str = this.game.Data.ExtraTabName.ToUpper();
              x37 -= (width - 12) * (num9 - 0);
            }
            if (this.game.EditObj.SetViewModeExtraNr == 2)
            {
              str = this.game.Data.ExtraTabName2.ToUpper();
              x37 -= (width - 12) * (num9 - 1);
            }
            if (this.game.EditObj.SetViewModeExtraNr == 3)
            {
              str = this.game.Data.ExtraTabName3.ToUpper();
              x37 -= (width - 12) * (num9 - 2);
            }
             let mut local41: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
             let mut local42: &Bitmap = &bitmap;
            let mut x38: i32 =  x37;
            let mut w10: i32 =  width;
            DrawMod.DrawScaledColorized( local41,  local42, x38, 66, w10, 24, 182, 24, 0.0f, 0.0f, 0.0f, 1f);
            SizeF sizeF22 = g.MeasureString(str, this.game.MarcFont16);
            DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16,  Math.Round( x37 +  width / 2.0 -  sizeF22.Width / 2.0), 70, Color.White);
            rectangle = Rectangle::new(x37, 66, width, 24);
            let mut trect10: &Rectangle = &rectangle
            this.AddMouse( trect10, "", "Extra data sheet.", 50 + this.game.EditObj.SetViewModeExtraNr);
            if (this.game.EditObj.SetViewModeExtraNr == 1)
              this.tab51 = this.MouseCounter;
            if (this.game.EditObj.SetViewModeExtraNr == 2)
              this.tab52 = this.MouseCounter;
            if (this.game.EditObj.SetViewModeExtraNr == 3)
              this.tab53 = this.MouseCounter;
          }
        }
        else
        {
          let mut num11: i32 =  0;
          if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
            num11 += 120;
          let mut x39: i32 =   Math.Round( num11 +  this.game.ScreenWidth / 2.0 - 480.0);
           let mut local43: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
           let mut local44: &Bitmap = &bitmap;
          let mut x40: i32 =  x39;
          DrawMod.DrawSimple( local43,  local44, x40, 66);
          str: String = "UNIT BASE INFO";
          SizeF sizeF23 = g.MeasureString(str, this.game.MarcFont16);
          DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16,  Math.Round( ( (x39 + 91) - sizeF23.Width / 2f)), 70, Color.White);
          Rectangle trect = Rectangle::new(x39, 66, BitmapStore.GetWidth(this.game.MARCLARGETAB), 24);
          this.AddMouse( trect, "", "The base info of the unit is always shown.");
        }
        if (!(this.game.EditObj.SetViewModeExtraNr == 0 | !flag1 & this.game.EditObj.SetViewModeExtraNr == 101))
          return;
        bool flag2;
        if (this.game.EditObj.OrderType == 14)
          flag2 = true;
        if (this.game.EditObj.OrderType == 15)
          flag2 = true;
        if (this.game.EditObj.OrderType == 2)
          flag2 = true;
        if (this.game.EditObj.OrderType == 12)
          flag2 = true;
        if (this.game.EditObj.OrderType == 11)
          flag2 = true;
        if (this.game.EditObj.OrderType == 13)
          flag2 = true;
        if (this.CurrentView == 2 & !flag2)
        {
          this.CurrentView = 0;
          this.game.EditObj.SetViewMode = 0;
        }
        if (Conversions.ToBoolean(Operators.AndObject( (this.CurrentView == 3), Operators.OrObject( !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ, Operators.CompareObjectEqual(obj,  false, false)))))
        {
          this.CurrentView = 0;
          this.game.EditObj.SetViewMode = 0;
          this.game.EditObj.SetViewMode3 = false;
        }
        if (Conversions.ToBoolean(Operators.AndObject( (this.CurrentView == 0), Operators.AndObject( this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ, obj))) && !this.game.EditObj.SetViewMode3)
        {
          this.CurrentView = 3;
          this.game.EditObj.SetViewMode = 3;
          this.game.EditObj.SetViewMode3 = true;
        }
        if (Conversions.ToBoolean(Operators.AndObject( this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ, Operators.CompareObjectEqual(obj,  true, false))))
        {
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn) | !this.game.Data.FOWOn)
          {
            if (this.CurrentView == 3)
            {
              if (flag2)
                this.DoTabs3( g);
              else
                this.DoTabs2B( g);
              this.DoTabs1B( g);
              this.DoTabs4( g, true);
            }
            else if (this.CurrentView == 0)
            {
              if (flag2)
                this.DoTabs3( g);
              else
                this.DoTabs2B( g);
              this.DoTabs4( g);
              this.DoTabs1B( g, true);
            }
            else if (this.CurrentView == 1)
            {
              this.DoTabs4( g);
              this.DoTabs1B( g);
              if (flag2)
                this.DoTabs3( g, true);
              else
                this.DoTabs2B( g, true);
            }
            else
            {
              if (this.CurrentView != 2)
                return;
              this.DoTabs4( g);
              if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn))
                this.DoTabs1B( g);
              if (flag2)
                this.DoTabs3( g, true);
              else
                this.DoTabs2B( g, true);
            }
          }
          else if (this.CurrentView != 3)
          {
            this.DoTabs4( g);
            this.DoTabs1B( g, true);
          }
          else
          {
            this.DoTabs1B( g);
            this.DoTabs4( g, true);
          }
        }
        else if (this.CurrentView == 0)
        {
          if (flag2)
            this.DoTabs3( g);
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn) | !this.game.Data.FOWOn)
            this.DoTabs2( g);
          this.DoTabs1( g, true);
        }
        else if (this.CurrentView == 1)
        {
          if (flag2)
            this.DoTabs3( g);
          this.DoTabs1( g);
          if (!(this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn) | !this.game.Data.FOWOn))
            return;
          this.DoTabs2( g, true);
        }
        else
        {
          if (this.CurrentView != 2)
            return;
          this.DoTabs1( g);
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn) | !this.game.Data.FOWOn)
            this.DoTabs2( g);
          if (!flag2)
            return;
          this.DoTabs3( g, true);
        }
      }
    }

    pub fn DoTabs1( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "UNIT TROOPS";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + 1 + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter > 7)
      {
        DrawMod.DrawBlock( g,  Math.Round( x1 +  sizeF2.Width + 1.0 + 2.0), y + 8 + 2, 9, 9, 0, 0, 0, 196);
        DrawMod.DrawBlock( g,  Math.Round( x1 +  sizeF2.Width + 1.0), y + 8, 9, 9,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      }
      Rectangle trect = Rectangle::new(num + 420, y, 182, 24);
      this.AddMouse( trect, "", "Click to get more stats on selected unit. [F10]", 1);
      this.tab1 = this.MouseCounter;
    }

    pub fn DoTabs1B( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 420 + 170;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 420 + 170;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "UNIT TROOPS";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 170 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + 1 + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter > 7)
      {
        DrawMod.DrawBlock( g,  Math.Round( x1 +  sizeF2.Width + 1.0 + 2.0), y + 8 + 2, 9, 9, 0, 0, 0, 196);
        DrawMod.DrawBlock( g,  Math.Round( x1 +  sizeF2.Width + 1.0), y + 8, 9, 9,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      }
      Rectangle trect = Rectangle::new(num + 420 + 170, y, 182, 24);
      this.AddMouse( trect, "", "Click to get more stats on selected unit. [F10]", 1);
      this.tab1 = this.MouseCounter;
    }

    pub fn DoTabs2( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      if (this.game.Data.Round == 0)
        return;
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 170 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 170 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "UNIT DETAILS";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 170 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 170 + 420, y, 182, 24);
      this.AddMouse( trect, "", "Click to see detailed stats on the unit. [F11]", 2);
      this.tab2 = this.MouseCounter;
    }

    pub fn DoTabs2B( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      if (this.game.Data.Round == 0)
        return;
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 340 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 340 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "UNIT DETAILS";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 340 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 340 + 420, y, 182, 24);
      this.AddMouse( trect, "", "Click to see detailed stats on the unit. [F11]", 2);
      this.tab2 = this.MouseCounter;
    }

    pub fn DoTabs3( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      if (this.game.Data.Round == 0)
        return;
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 340 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 340 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "COMBAT SETUP";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 340 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 340 + 420, y, 182, 24);
      this.AddMouse( trect, "", "Click to see the combat setup. [F12]", 3);
      this.tab3 = this.MouseCounter;
    }

    pub fn DoTabs4( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 0 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 0 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "OFFICER INFO";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 0 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 0 + 420, y, 182, 24);
      this.AddMouse( trect, "", "Click to see the info on the officer with this unit. [F9]", 4);
      this.tab4 = this.MouseCounter;
    }

    pub fn DoTabs4B( Graphics g, bool Active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 480.0);
      if (!Active)
      {
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  num + 170 + 420;
        DrawMod.Draw( local1,  local2, x, 66, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
         let mut local3: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  num + 170 + 420;
        DrawMod.DrawSimple( local3,  local4, x, 66);
      }
      str: String = "OFFICER INFO";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x1: i32 =   Math.Round( ( (num + 420 + 170 + 91) - sizeF2.Width / 2f));
      let mut y: i32 =  66;
      DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont16, x1, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num + 170 + 420, y, 182, 24);
      this.AddMouse( trect, "", "Click to see the info on the officer with this unit. [F9]", 4);
      this.tab4 = this.MouseCounter;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter <= -1)
        return;
      let mut subPartCounter: i32 =  this.SubPartCounter;
      for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
      {
        if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index] && Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
        {
          this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = "";
          this.game.EditObj.TipText = this.SubPartList[index].Descript;
          break;
        }
      }
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = base.HandleMouseMove(x, y);
      if (y > 18 &&  this.w / 2.0 - 500.0 <  x &  x <  this.w / 2.0 + 500.0)
        windowReturnClass.NoMouseClickBelow = true;
      if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode && y > 0 &&  this.w / 2.0 - 640.0 <  x &  x <  this.w / 2.0 + 640.0)
        windowReturnClass.NoMouseClickBelow = true;
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      numArray: Vec<i32> = new int[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.BattleTimerActive)
        return windowReturnClass1;
      if (y > 18 &&  this.w / 2.0 - 500.0 <  x &  x <  this.w / 2.0 + 500.0)
        windowReturnClass1.NoMouseClickBelow = true;
      for (let mut mouseCounter: i32 =  this.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (this.MouseData[mouseCounter] > 0 && x > this.MouseRect[mouseCounter].X & x < this.MouseRect[mouseCounter].X + this.MouseRect[mouseCounter].Width && y > this.MouseRect[mouseCounter].Y & y < this.MouseRect[mouseCounter].Y + this.MouseRect[mouseCounter].Height)
        {
          switch (this.MouseData[mouseCounter])
          {
            case 1:
              if (this.game.EditObj.SetViewMode == 0)
                this.game.EditObj.SubformationListMode = !this.game.EditObj.SubformationListMode;
              this.game.EditObj.SetViewMode = 0;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 2:
              this.game.EditObj.SetViewMode = 1;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 3:
              this.game.EditObj.SetViewMode = 2;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 4:
              this.game.EditObj.SetViewMode = 3;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 6:
              this.game.EditObj.SetViewModeExtraNr = 0;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 51:
              this.game.EditObj.SetViewModeExtraNr = 1;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 52:
              this.game.EditObj.SetViewModeExtraNr = 2;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 53:
              this.game.EditObj.SetViewModeExtraNr = 3;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 101:
              this.game.EditObj.SetViewModeExtraNr = 101;
              this.dostuff();
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 201:
              if (this.Cancelid > 0)
                this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
              this.game.EditObj.udsUnitOrderMode = 1;
              ScreenClass screeny1 = this.formref.Screeny;
              System.Type type1 = typeof (MapWindowClass2);
               System.Type local1 =  type1;
              MapWindowClass2 window1 = (MapWindowClass2) screeny1.GetWindow( local1);
              if (!Information.IsNothing( window1))
              {
                this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
                if (this.game.EditObj.UnitSelected > -1)
                  window1.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, true);
                else
                  window1.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
              }
              if (this.Cancelid > 0)
                this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass1.AddCommand(4, 12);
              windowReturnClass1.AddCommand(4, 9);
              windowReturnClass1.AddCommand(4, 67);
              windowReturnClass1.AddCommand(4, 68);
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 202:
              if (this.Cancelid > 0)
                this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
              this.game.EditObj.udsUnitOrderMode = 48;
              ScreenClass screeny2 = this.formref.Screeny;
              System.Type type2 = typeof (MapWindowClass2);
               System.Type local2 =  type2;
              MapWindowClass2 window2 = (MapWindowClass2) screeny2.GetWindow( local2);
              if (!Information.IsNothing( window2))
              {
                this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
                if (this.game.EditObj.UnitSelected > -1)
                  window2.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, true);
                else
                  window2.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
              }
              if (this.Cancelid > 0)
                this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass1.AddCommand(4, 12);
              windowReturnClass1.AddCommand(4, 9);
              windowReturnClass1.AddCommand(4, 67);
              windowReturnClass1.AddCommand(4, 68);
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            case 203:
              if (this.Cancelid > 0)
                this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.Cancelid)] + 1, this.SubPartY[this.SubpartNr(this.Cancelid)] + 1, 1);
              this.game.EditObj.OrderType = 0;
              if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
              {
                this.game.EditObj.udsUnitOrderMode = 0;
                ScreenClass screeny3 = this.formref.Screeny;
                System.Type type3 = typeof (MapWindowClass2);
                 System.Type local3 =  type3;
                MapWindowClass2 window3 = (MapWindowClass2) screeny3.GetWindow( local3);
                if (!Information.IsNothing( window3))
                {
                  this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
                  if (this.game.EditObj.UnitSelected > -1)
                    window3.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, true);
                  else
                    window3.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
                }
              }
              else
              {
                this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                this.game.EditObj.MapSelected = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map;
              }
              if (this.Cancelid > 0)
                this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass1.AddCommand(4, 12);
              windowReturnClass1.AddCommand(4, 67);
              windowReturnClass1.AddCommand(4, 69);
              windowReturnClass1.AddCommand(4, 68);
              windowReturnClass1.AddCommand(4, 9);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            default:
              continue;
          }
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut index2: i32 =  1;
            while (this.ActionButtonId[index2] != this.SubPartID[index1])
            {
              index2 += 1;
              if (index2 > 10)
              {
                let mut num1: i32 =  this.SubPartID[index1];
                if (num1 == this.GuiDownId)
                {
                  if (!this.game.EditObj.GuiDown)
                    this.game.EditObj.GuiDown = true;
                  else
                    this.game.EditObj.GuiDown = false;
                  windowReturnClass1.AddCommand(3, 11);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == this.SaveId)
                {
                  str1: String;
                  if (this.game.Data.Round == 0)
                  {
                    str2: String = this.game.AppPath + "scenarios\\";
                    if (!Information.IsNothing( this.game.Data.ScenarioDir))
                    {
                      if (this.game.Data.ScenarioDir.Length > 1)
                        str1 = str2.Replace("scenarios", this.game.Data.ScenarioDir);
                      else if (this.game.ModScenarioDir.Length > 1)
                        str1 = str2.Replace("scenarios", this.game.ModScenarioDir);
                    }
                    else if (this.game.ModScenarioDir.Length > 1)
                      str1 = str2.Replace("scenarios", this.game.ModScenarioDir);
                  }
                  else
                    str1 = "savedgames";
                  str3: String = this.game.Data.Round != 0 ? this.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", this.game.AppPath_SAVEGAMES, false) : this.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", this.game.AppPath_SAVEGAMES, false);
                  if (Strings.Len(str3) >= 2)
                  {
                    this.game.FormRef.Cursor = Cursors.WaitCursor;
                    this.game.Data.serialize(str3);
                    this.game.HandyFunctionsObj.ZipFile(str3);
                    windowReturnClass1.SetFlag(true);
                    this.game.FormRef.Cursor = Cursors.Default;
                  }
                }
                else if (num1 == this.BattleId)
                {
                  if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
                  {
                    this.game.EditObj.PopupValue = 22;
                    windowReturnClass1.AddCommand(5, 14);
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    windowReturnClass1.SetFlag(true);
                  }
                  else
                  {
                    this.game.EditObj.BattleTimerActive = true;
                    this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
                  }
                }
                else if (num1 == this.PopupButtonId)
                {
                  switch (this.game.EditObj.OrderType)
                  {
                    case 2:
                      this.TimerUsed = false;
                      this.game.EditObj.PopupValue = 18;
                      windowReturnClass1.AddCommand(5, 14);
                      this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    case 3:
                      this.TimerUsed = false;
                      Form3::new( this.formref).Initialize(this.game.Data, 82, this.game.EditObj.OrderUnit, tGame: this.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    case 11:
                      this.TimerUsed = false;
                      this.game.EditObj.PopupValue = 12;
                      windowReturnClass1.AddCommand(5, 14);
                      this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    case 14:
                    case 15:
                      this.TimerUsed = false;
                      this.game.EditObj.PopupValue = 11;
                      windowReturnClass1.AddCommand(5, 14);
                      this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    case 18:
                    case 49:
                      this.TimerUsed = false;
                      Form3::new( this.formref).Initialize(this.game.Data, 60, this.game.EditObj.UnitSelected, tGame: this.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                  }
                }
                else if (num1 == this.KillId)
                {
                  if (this.game.EditObj.OrderType == 3)
                  {
                    if (this.game.ProcessingObj.SetUnitHq(this.game.EditObj.OrderUnit, -1).OK)
                    {
                      this.game.EditObj.OrderType = 0;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.AddCommand(4, 9);
                      this.game.SelectX = this.game.EditObj.OrderX;
                      this.game.SelectY = this.game.EditObj.OrderY;
                      this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    }
                    this.dostuff();
                    windowReturnClass1.SetFlag(true);
                  }
                }
                else if (num1 == this.NoneId)
                {
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.lastorderx = -1;
                  this.lastordery = -1;
                  let mut counter: i32 =  this.game.EditObj.TempUnitList.counter;
                  for (let mut index3: i32 =  0; index3 <= counter; index3 += 1)
                    this.game.EditObj.TempCoordList.AddCoord(this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index3]].X, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index3]].Y, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index3]].Map);
                  this.game.EditObj.TempUnitList = UnitList::new();
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.AddCommand(4, 9);
                  windowReturnClass1.AddCommand(4, 69);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.AllId)
                {
                  Coordinate target;
                  target.x = this.game.EditObj.OrderX;
                  target.y = this.game.EditObj.OrderY;
                  this.lastorderx = target.x;
                  this.lastordery = target.y;
                  target.onmap = true;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempUnitList = UnitList::new();
                  if (this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 13 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 15)
                  {
                    let mut mapCounter: i32 =  this.game.Data.MapCounter;
                    for (let mut index4: i32 =  0; index4 <= mapCounter; index4 += 1)
                    {
                      let mut mapWidth: i32 =  this.game.Data.MapObj[index4].MapWidth;
                      for (let mut index5: i32 =  0; index5 <= mapWidth; index5 += 1)
                      {
                        let mut mapHeight: i32 =  this.game.Data.MapObj[index4].MapHeight;
                        for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
                        {
                          Coordinate coordinate;
                          coordinate.x = index5;
                          coordinate.y = index6;
                          coordinate.map = index4;
                          coordinate.onmap = true;
                          if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter > -1 && this.game.Data.UnitObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[0]].Regime == this.game.Data.Turn)
                          {
                            let mut unitCounter: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter;
                            for (let mut index7: i32 =  0; index7 <= unitCounter; index7 += 1)
                            {
                              let mut unit: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[index7];
                              if (this.game.EditObj.TempCoordList.counter < 15)
                              {
                                if (this.game.EditObj.OrderType == 11)
                                {
                                  if (this.game.HandyFunctionsObj.CanDoArtAttack(unit, target, false))
                                  {
                                    if ( this.game.Data.RuleVar[899] < 1.0)
                                    {
                                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                      this.game.EditObj.TempUnitList.add(unit);
                                    }
                                    else if ( this.game.HandyFunctionsObj.GetArtPercent(unit, true) >=  this.game.Data.RuleVar[899])
                                    {
                                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                      this.game.EditObj.TempUnitList.add(unit);
                                    }
                                  }
                                  else if ( this.game.Data.RuleVar[419] > 0.0 && this.game.HandyFunctionsObj.CanDoDirectAttack(unit, target, false))
                                  {
                                    if ( this.game.Data.RuleVar[899] < 1.0)
                                    {
                                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                      this.game.EditObj.TempUnitList.add(unit);
                                    }
                                    else if ( this.game.HandyFunctionsObj.GetDirectPercent(unit, true) >=  this.game.Data.RuleVar[899])
                                    {
                                      this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                      this.game.EditObj.TempUnitList.add(unit);
                                    }
                                  }
                                }
                                else if (this.game.EditObj.OrderType == 13)
                                {
                                  if (this.game.HandyFunctionsObj.CanDoSeaArtAttack(unit, target))
                                  {
                                    this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                    this.game.EditObj.TempUnitList.add(unit);
                                  }
                                }
                                else if (this.game.HandyFunctionsObj.CanDoAirStrike(unit, target))
                                {
                                  this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                  this.game.EditObj.TempUnitList.add(unit);
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                  else
                  {
                    let mut num2: i32 =  this.game.HandyFunctionsObj.HexNeighbourCount(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                    for (let mut tfacing: i32 =  1; tfacing <= num2; tfacing += 1)
                    {
                      Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, tfacing);
                      if (coordinate.onmap && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter > -1 && this.game.Data.UnitObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[0]].Regime == this.game.Data.Turn)
                      {
                        let mut unitCounter: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter;
                        for (let mut index8: i32 =  0; index8 <= unitCounter; index8 += 1)
                        {
                          if (this.game.EditObj.TempCoordList.counter < 15)
                          {
                            let mut unit: i32 =  this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitList[index8];
                            if (this.game.EditObj.OrderType == 2)
                            {
                              if (this.game.HandyFunctionsObj.CanDoLandAttack(unit, target))
                              {
                                this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                                this.game.EditObj.TempUnitList.add(unit);
                              }
                            }
                            else if (this.game.HandyFunctionsObj.CanDoSeaAttack(unit, target))
                            {
                              this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                              this.game.EditObj.TempUnitList.add(unit);
                            }
                          }
                        }
                      }
                    }
                  }
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 9);
                  windowReturnClass1.AddCommand(4, 69);
                  windowReturnClass1.AddCommand(4, 67);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.OkId)
                {
                  switch (this.game.EditObj.OrderType)
                  {
                    case 2:
                    case 11:
                    case 12:
                    case 13:
                    case 14:
                    case 15:
                      if (this.game.EditObj.TempUnitList.CheckIfPresent(this.game.EditObj.UnitSelected))
                        this.game.EditObj.TempUnitList.remove(this.game.EditObj.UnitSelected);
                      else
                        this.game.EditObj.TempUnitList.add(this.game.EditObj.UnitSelected);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 69);
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 3:
                      let mut historical: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical;
                      if (historical > -1 && this.game.Data.HistoricalUnitObj[historical].HandCardCounter > -1 &  this.game.Data.RuleVar[897] == 0.0 && Interaction.MsgBox( "This action will cause the HQ to lose all handcards. Are you sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
                      {
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 9);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.dostuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      OrderResult orderResult1 = this.game.ProcessingObj.SetUnitHq(this.game.EditObj.OrderUnit, this.game.EditObj.UnitSelected);
                      if (this.game.EditObj.SoundOn)
                        SoundMod.PlayAWave(this.game.AppPath + "sound/radio.wav",  this.game.EditObj);
                      if (orderResult1.OK)
                      {
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 18:
                    case 49:
                      if (this.game.EditObj.OrderTarget == -1)
                      {
                        this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(1, 69);
                        windowReturnClass1.AddCommand(2, 35);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    case 19:
                    case 42:
                      if (this.game.EditObj.OrderTarget == -1)
                      {
                        this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                        if (this.game.HandyFunctionsObj.HasUnitOnlyParaLandSF(this.game.EditObj.OrderTarget))
                        {
                          if (Interaction.MsgBox( "Do you want to paradrop?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                          {
                            this.game.EditObj.OrderType = 19;
                            this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, ClearSea: true, attack: true, isparadrop: true);
                          }
                          else
                          {
                            this.game.EditObj.OrderType = 42;
                            this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, PredictAirOnly: true, ClearSea: true, ismove: true);
                          }
                        }
                        else
                        {
                          this.game.EditObj.OrderType = 42;
                          this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, PredictAirOnly: true, ClearSea: true, ismove: true);
                        }
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 9);
                        windowReturnClass1.AddCommand(4, 67);
                        this.dostuff();
                        windowReturnClass1.SetFlag(true);
                        break;
                      }
                      let mut num3: i32 =  0;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
                      {
                        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > 14)
                        {
                          num3 = 1;
                          let mut num4: i32 =   Interaction.MsgBox( "Already 16 units in that hex.");
                          this.game.EditObj.OrderType = 0;
                          windowReturnClass1.AddCommand(4, 12);
                          windowReturnClass1.AddCommand(4, 68);
                          windowReturnClass1.AddCommand(4, 69);
                          windowReturnClass1.AddCommand(4, 9);
                          windowReturnClass1.AddCommand(4, 67);
                          this.game.SelectX = this.game.EditObj.OrderX;
                          this.game.SelectY = this.game.EditObj.OrderY;
                          this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                          this.dostuff();
                          windowReturnClass1.SetFlag(true);
                        }
                        this.game.HandyFunctionsObj.IsHexAirfield(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      }
                      if (num3 == 0)
                      {
                        this.game.EditObj.TargetX = this.game.SelectX;
                        this.game.EditObj.TargetY = this.game.SelectY;
                        this.game.TempCombat = new CombatClass(this.game);
                        Coordinate Target = Coordinate::new();
                        Target.x = this.game.EditObj.TargetX;
                        Target.y = this.game.EditObj.TargetY;
                        this.game.SelectX = this.game.EditObj.TargetX;
                        this.game.SelectY = this.game.EditObj.TargetY;
                        this.game.EditObj.TempUnitList = UnitList::new();
                        this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                        if (this.game.EditObj.OrderType != 42)
                          this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderTarget, 1);
                        if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                        {
                          this.game.EditObj.PopupValue = 7;
                          windowReturnClass1.AddCommand(5, 14);
                          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                        windowReturnClass1.SetFlag(true);
                        break;
                      }
                      break;
                    case 20:
                      OrderResult orderResult2 = this.game.ProcessingObj.LoadUnit(this.game.EditObj.UnitSelected, this.game.EditObj.OrderUnit);
                      if (this.game.EditObj.SoundOn)
                        SoundMod.PlayAWave(this.game.AppPath + "sound/load.wav",  this.game.EditObj);
                      if (orderResult2.OK)
                      {
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 9);
                        if (this.game.SelectX == this.game.EditObj.OrderX & this.game.SelectY == this.game.EditObj.OrderY)
                        {
                          let mut num5: i32 =  0;
                          while (this.game.EditObj.OrderUnit != this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, 0, false, 1))
                          {
                            num5 += 1;
                            if (num5 > 99)
                              break;
                          }
                        }
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        if (this.game.EditObj.MapSelected != this.game.EditObj.OrderMap)
                        {
                          this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                          this.game.EditObj.TempCoordList = CoordList::new();
                        }
                        else
                          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 21:
                      OrderResult orderResult3 = this.game.ProcessingObj.unLoadUnit(this.game.EditObj.OrderTarget, this.game.EditObj.OrderUnit, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      if (this.game.EditObj.SoundOn)
                        SoundMod.PlayAWave(this.game.AppPath + "sound/unload.wav",  this.game.EditObj);
                      if (orderResult3.OK)
                      {
                        if (!this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.Data.Turn, true) & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].CardUponConquest == -1)
                        {
                          this.game.EditObj.OrderType = 0;
                          windowReturnClass1.AddCommand(4, 12);
                          windowReturnClass1.AddCommand(4, 68);
                          windowReturnClass1.AddCommand(4, 69);
                          windowReturnClass1.AddCommand(4, 9);
                          this.game.EditObj.TempCoordList = CoordList::new();
                          this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                          this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X;
                          this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y;
                          this.game.EditObj.OrderType = 0;
                          this.game.EditObj.TempCoordList = CoordList::new();
                          this.dostuff();
                          windowReturnClass1.SetFlag(true);
                          break;
                        }
                        this.game.EditObj.TargetX = this.game.SelectX;
                        this.game.EditObj.TargetY = this.game.SelectY;
                        this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                        this.game.TempCombat = new CombatClass(this.game);
                        Coordinate Target = Coordinate::new();
                        Target.x = this.game.EditObj.TargetX;
                        Target.y = this.game.EditObj.TargetY;
                        Target.map = this.game.EditObj.TargetMap;
                        this.game.EditObj.TempUnitList = UnitList::new();
                        this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderTarget);
                        if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                        {
                          this.game.EditObj.BattleTimerActive = true;
                          this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
                        }
                        windowReturnClass1.SetFlag(true);
                        break;
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 33:
                      this.game.TempCombat = new CombatClass(this.game);
                      Coordinate Target1 = Coordinate::new();
                      Target1.x = this.game.SelectX;
                      Target1.y = this.game.SelectY;
                      this.game.EditObj.TempUnitList = UnitList::new();
                      this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                      if (this.game.TempCombat.Init(Target1, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                      {
                        if ( this.game.Data.RuleVar[839] == 1.0)
                        {
                          this.game.EditObj.PopupValue = 7;
                          windowReturnClass1.AddCommand(5, 14);
                          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                        windowReturnClass1.AddCommand(3, 5);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 35:
                      if (this.game.ProcessingObj.BlowBridge(this.game.EditObj.OrderUnit, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) - 1).OK)
                      {
                        SoundMod.PlayAWave(this.game.AppPath + "sound/blow.wav",  this.game.EditObj);
                        this.game.EditObj.OrderType = 0;
                        DrawMod.TGame.EditObj.PopupValue = 5;
                        windowReturnClass1.AddCommand(5, 14);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.OrderMap);
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      }
                      else
                      {
                        this.game.EditObj.OrderType = 0;
                        DrawMod.TGame.EditObj.PopupValue = 5;
                        windowReturnClass1.AddCommand(5, 14);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        this.game.SelectX = this.game.EditObj.OrderX;
                        this.game.SelectY = this.game.EditObj.OrderY;
                        this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      }
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 36:
                      OrderResult orderResult4 = this.game.ProcessingObj.BuildInfra(this.game.EditObj.OrderUnit, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected) - 1);
                      if (this.game.EditObj.SoundOn)
                        SoundMod.PlayAWave(this.game.AppPath + "sound/building.wav",  this.game.EditObj);
                      if (orderResult4.OK)
                      {
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.ProcessingObj.LocationProductionPrognosis();
                        DrawMod.TGame.EditObj.PopupValue = 5;
                        windowReturnClass1.AddCommand(5, 14);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                      break;
                    case 40:
                      this.game.EditObj.TempUnitList = UnitList::new();
                      this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                      Coordinate Target2;
                      Target2.x = this.game.SelectX;
                      Target2.y = this.game.SelectY;
                      this.game.EditObj.AirSupplyPts = Math.Min(this.LastAirSupplyMax, this.LastAirSupplyNeed);
                      this.game.EditObj.AirSupplyHq = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HQ;
                      this.game.EditObj.TargetX = this.game.SelectX;
                      this.game.EditObj.TargetY = this.game.SelectY;
                      this.game.EditObj.AirSupplyCarry = this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.OrderUnit, 2);
                      Target2.onmap = true;
                      this.game.TempCombat = new CombatClass(this.game);
                      if (!this.game.TempCombat.Init(Target2, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                        return windowReturnClass1;
                      this.game.EditObj.PopupValue = 7;
                      windowReturnClass1.AddCommand(5, 14);
                      this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                  }
                }
                else if (num1 == this.ChangeModelId)
                {
                  this.game.EditObj.OrderType = 46;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  windowReturnClass1.AddCommand(1, 5);
                  windowReturnClass1.AddCommand(2, 90);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.NewUnitButtonId)
                {
                  this.game.EditObj.OrderType = 7;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  windowReturnClass1.AddCommand(1, 5);
                  windowReturnClass1.AddCommand(2, 88);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.NewUnitButton2Id)
                {
                  this.game.EditObj.OrderType = 44;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  windowReturnClass1.AddCommand(1, 5);
                  windowReturnClass1.AddCommand(2, 89);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.UnLoadButtonID)
                {
                  this.game.EditObj.OrderTarget = -1;
                  this.game.EditObj.OrderType = 21;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderTarget = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerList[this.game.EditObj.SFSelected - (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + 1)];
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempValue = new MapMatrix2[this.game.Data.MapCounter + 1];
                  let mut mapCounter1: i32 =  this.game.Data.MapCounter;
                  for (let mut index9: i32 =  0; index9 <= mapCounter1; index9 += 1)
                    this.game.EditObj.TempValue[index9] = new MapMatrix2(this.game.Data.MapObj[index9].MapWidth, this.game.Data.MapObj[index9].MapHeight);
                  let mut mapCounter2: i32 =  this.game.Data.MapCounter;
                  for (let mut index10: i32 =  0; index10 <= mapCounter2; index10 += 1)
                  {
                    let mut mapWidth: i32 =  this.game.Data.MapObj[index10].MapWidth;
                    for (let mut index11: i32 =  0; index11 <= mapWidth; index11 += 1)
                    {
                      let mut mapHeight: i32 =  this.game.Data.MapObj[index10].MapHeight;
                      for (let mut index12: i32 =  0; index12 <= mapHeight; index12 += 1)
                        this.game.EditObj.TempValue[index10].Value[index11, index12] = 9999;
                    }
                  }
                  let mut num6: i32 =  this.game.HandyFunctionsObj.HexNeighbourCount(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  for (let mut tfacing: i32 =  1; tfacing <= num6; tfacing += 1)
                  {
                    Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, tfacing);
                    if (coordinate.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                    {
                      if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) == 0)
                      {
                        if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                        }
                      }
                      else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) <= 1 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].CanAmph && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].IsSea)
                      {
                        if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime))
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                        }
                        else if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime == this.game.Data.Turn && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                        }
                      }
                    }
                  }
                  Coordinate coordinate1 = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
                  if (coordinate1.onmap)
                  {
                    this.game.EditObj.MapSelected = coordinate1.map;
                    this.game.SelectX = coordinate1.x;
                    this.game.SelectY = coordinate1.y;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                    windowReturnClass1.AddCommand(4, 18);
                  }
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 29);
                  windowReturnClass1.AddCommand(4, 20);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.LoadButtonId)
                {
                  this.game.EditObj.OrderTarget = -1;
                  this.game.EditObj.OrderType = 20;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempValue = new MapMatrix2[this.game.Data.MapCounter + 1];
                  let mut mapCounter3: i32 =  this.game.Data.MapCounter;
                  for (let mut index13: i32 =  0; index13 <= mapCounter3; index13 += 1)
                    this.game.EditObj.TempValue[index13] = new MapMatrix2(this.game.Data.MapObj[index13].MapWidth, this.game.Data.MapObj[index13].MapHeight);
                  let mut mapCounter4: i32 =  this.game.Data.MapCounter;
                  for (let mut index14: i32 =  0; index14 <= mapCounter4; index14 += 1)
                  {
                    let mut mapWidth: i32 =  this.game.Data.MapObj[index14].MapWidth;
                    for (let mut index15: i32 =  0; index15 <= mapWidth; index15 += 1)
                    {
                      let mut mapHeight: i32 =  this.game.Data.MapObj[index14].MapHeight;
                      for (let mut index16: i32 =  0; index16 <= mapHeight; index16 += 1)
                        this.game.EditObj.TempValue[index14].Value[index15, index16] = 9999;
                    }
                  }
                  let mut num7: i32 =  this.game.HandyFunctionsObj.HexNeighbourCount(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  for (let mut tfacing: i32 =  1; tfacing <= num7; tfacing += 1)
                  {
                    Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, tfacing);
                    if (coordinate.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                    {
                      if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) == 0)
                      {
                        if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                        }
                      }
                      else if (this.game.HandyFunctionsObj.Distance(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map) <= 1 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].CanAmph && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].LandscapeType].IsSea)
                      {
                        if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime))
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                        }
                        else if (this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime == this.game.Data.Turn && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].UnitCounter < 15)
                        {
                          this.game.EditObj.TempCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                          this.game.EditObj.TempValue[coordinate.map].Value[coordinate.x, coordinate.y] = 0;
                        }
                      }
                    }
                  }
                  Coordinate coordinate2 = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
                  if (coordinate2.onmap)
                  {
                    this.game.EditObj.MapSelected = coordinate2.map;
                    this.game.SelectX = coordinate2.x;
                    this.game.SelectY = coordinate2.y;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                  }
                  this.game.EditObj.TempCoordList = CoordList::new();
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 68);
                  windowReturnClass1.AddCommand(4, 69);
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.AirSupplyButtonId)
                {
                  this.game.EditObj.OrderTarget = -1;
                  this.game.EditObj.TargetX = -1;
                  this.game.EditObj.TargetY = -1;
                  this.game.EditObj.OrderType = 40;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, attack: true);
                  let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
                  for (let mut index17: i32 =  0; index17 <= mapWidth; index17 += 1)
                  {
                    let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                    for (let mut index18: i32 =  0; index18 <= mapHeight; index18 += 1)
                      this.game.EditObj.TempValue2[0].Value[index17, index18] = this.game.EditObj.TempValue[0].Value[index17, index18];
                  }
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 68);
                  windowReturnClass1.AddCommand(4, 69);
                  windowReturnClass1.AddCommand(4, 9);
                  windowReturnClass1.AddCommand(4, 67);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.seaAttackButtonId)
                {
                  this.game.EditObj.OrderTarget = -1;
                  this.game.EditObj.TempUnitList = UnitList::new();
                  this.game.EditObj.OrderType = 12;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TargetX = this.game.SelectX;
                  this.game.EditObj.TargetY = this.game.SelectY;
                  this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  this.game.EditObj.SetViewMode = 2;
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 69);
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.AddCommand(4, 9);
                  windowReturnClass1.AddCommand(4, 68);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.SeaArtAttackButtonId)
                {
                  this.game.EditObj.OrderTarget = -1;
                  this.game.EditObj.TempUnitList = UnitList::new();
                  this.game.EditObj.OrderType = 13;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TargetX = this.game.SelectX;
                  this.game.EditObj.TargetY = this.game.SelectY;
                  this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  this.game.EditObj.SetViewMode = 2;
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 69);
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.AddCommand(4, 9);
                  windowReturnClass1.AddCommand(4, 68);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else if (num1 == this.ParadropButtonId)
                {
                  this.game.EditObj.OrderTarget = -1;
                  this.game.EditObj.TargetX = -1;
                  this.game.EditObj.TargetY = -1;
                  this.game.EditObj.OrderType = 19;
                  this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                  this.game.EditObj.OrderX = this.game.SelectX;
                  this.game.EditObj.OrderY = this.game.SelectY;
                  this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  windowReturnClass1.AddCommand(4, 12);
                  windowReturnClass1.AddCommand(4, 20);
                  this.dostuff();
                  windowReturnClass1.SetFlag(true);
                }
                else
                {
                  if (num1 == this.NextButtonId)
                  {
                    if (this.game.Data.Round == 0)
                    {
                      windowReturnClass2: WindowReturnClass;
                      return windowReturnClass2;
                    }
                    let mut messCounter1: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                    num8: i32;
                    for (let mut index19: i32 =  0; index19 <= messCounter1; index19 += 1)
                    {
                      if (Strings.InStr(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[index19], "YOUR MESSAGE TO") > 0)
                        num8 += 1;
                    }
                    if (num8 == 0 & !this.game.EditObj.NextTurnButtonPress & this.game.Data.PbemGameID > 0 & this.game.Data.Round == 1)
                    {
                      this.game.EditObj.NextTurnButtonPress = true;
                      this.game.EditObj.OrderType = 0;
                      this.game.EditObj.QuestionText = "It is a good custom to send a friendly in-game text message to your opponent. You can do this with the 'message' button in the strategic map tab.";
                      this.game.EditObj.AnswerCount = 2;
                      this.game.EditObj.AnswerText[1] = "End turn";
                      this.game.EditObj.AnswerText[2] = "Good idea";
                      this.game.EditObj.AnswerTextMouseOver[1] = "End your turn.";
                      this.game.EditObj.AnswerTextMouseOver[2] = "Continue your turn.";
                      DrawMod.TGame.EditObj.PopupValue = 10;
                      windowReturnClass1.AddCommand(5, 14);
                      this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                      this.EndingTurn = true;
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    this.game.EditObj.OrderType = 0;
                    bool flag1 = false;
                    let mut num9: i32 =  0;
                    let mut messCounter2: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                    for (let mut index20: i32 =  0; index20 <= messCounter2; index20 += 1)
                    {
                      bool flag2 = false;
                      if (this.game.Data.RegimeObj[this.game.Data.Turn].MesStyle[index20] == 3)
                      {
                        DynamicData dynamicData = new DynamicData(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[index20]);
                        let mut elementCounter: i32 =  dynamicData.elementCounter;
                        for (let mut index21: i32 =  0; index21 <= elementCounter; index21 += 1)
                        {
                          if (dynamicData.element[index21].type == DynamicType.OptionField)
                            flag2 = true;
                        }
                      }
                      if (flag2 && this.game.Data.RegimeObj[this.game.Data.Turn].MesChosen[index20] == 0)
                      {
                        flag1 = true;
                        num9 += 1;
                      }
                    }
                    this.game.EditObj.QuestionText = "Are you sure you want to end your turn?";
                    if (flag1)
                      this.game.EditObj.QuestionText = "You have delegated " + num9.ToString() + " decisions. " + this.game.EditObj.QuestionText;
                    this.game.EditObj.AnswerCount = 2;
                    this.game.EditObj.AnswerText[1] = "Yes";
                    this.game.EditObj.AnswerText[2] = "No";
                    this.game.EditObj.AnswerTextMouseOver[1] = "End your turn.";
                    this.game.EditObj.AnswerTextMouseOver[2] = "Continue your turn.";
                    DrawMod.TGame.EditObj.PopupValue = 10;
                    windowReturnClass1.AddCommand(5, 14);
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    this.EndingTurn = true;
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (num1 == this.ShowAsId)
                  {
                    if (this.game.EditObj.HideAS)
                      this.game.EditObj.HideAS = false;
                    else
                      this.game.EditObj.HideAS = true;
                    this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass1.AddCommand(4, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.HexUnitButtonId)
                  {
                    if (this.game.EditObj.HideUnit == 0)
                      this.game.EditObj.HideUnit = 1;
                    else if (this.game.EditObj.HideUnit == 1)
                    {
                      if ( this.game.Data.RuleVar[344] == 1.0)
                        this.game.EditObj.HideUnit = 2;
                      else
                        this.game.EditObj.HideUnit = 0;
                    }
                    else
                      this.game.EditObj.HideUnit = 0;
                    this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass1.AddCommand(4, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.ButtonZoomOutId)
                  {
                    let mut num10: i32 =  0;
                    if (this.game.EditObj.GuiDown)
                      num10 = 222;
                    let mut num11: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 53.0));
                    let mut num12: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 106.0));
                    let mut num13: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num10)) / 53.0));
                    let mut num14: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num10)) / 106.0));
                    num15: i32;
                    num16: i32;
                    if (this.game.EditObj.Zoom == 0)
                    {
                      this.game.EditObj.Zoom = -1;
                      this.game.CornerX -=  Math.Round(Conversion.Int( num11 / 2.0));
                      this.game.CornerY -=  Math.Round(Conversion.Int( num13 / 2.0));
                      num15 = 27;
                      num16 = 24;
                    }
                    else
                    {
                      this.game.EditObj.Zoom = 0;
                      this.game.CornerX -=  Math.Round(Conversion.Int( num12 / 2.0));
                      this.game.CornerY -=  Math.Round(Conversion.Int( num14 / 2.0));
                      num15 = 53;
                      num16 = 48;
                    }
                    if ( this.game.CornerX +  this.game.ScreenWidth /  num15 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                      this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num15);
                    if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num10)) /  num16 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
                      this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num10)) /  num16);
                    if (this.game.CornerX < 0)
                      this.game.CornerX = 0;
                    if (this.game.CornerY < 0)
                      this.game.CornerY = 0;
                    this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass1.AddCommand(1, 12);
                    windowReturnClass1.AddCommand(2, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.ButtonZoomInId)
                  {
                    let mut num17: i32 =  0;
                    if (this.game.EditObj.GuiDown)
                      num17 = 222;
                    let mut num18: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 53.0));
                    let mut num19: i32 =   Math.Round(Conversion.Int( this.game.ScreenWidth / 106.0));
                    let mut num20: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num17)) / 53.0));
                    let mut num21: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num17)) / 106.0));
                    num22: i32;
                    num23: i32;
                    if (this.game.EditObj.Zoom == 0)
                    {
                      this.game.EditObj.Zoom = 1;
                      this.game.CornerX +=  Math.Round(Conversion.Int( num19 / 2.0));
                      this.game.CornerY +=  Math.Round(Conversion.Int( num21 / 2.0));
                      num22 = 106;
                      num23 = 96;
                    }
                    else
                    {
                      this.game.EditObj.Zoom = 0;
                      this.game.CornerX +=  Math.Round(Conversion.Int( num18 / 2.0));
                      this.game.CornerY +=  Math.Round(Conversion.Int( num20 / 2.0));
                      num22 = 53;
                      num23 = 48;
                    }
                    if ( this.game.CornerX +  this.game.ScreenWidth /  num22 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                      this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num22);
                    if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num17)) /  num23 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
                      this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num17)) /  num23);
                    if (this.game.CornerX < 0)
                      this.game.CornerX = 0;
                    if (this.game.CornerY < 0)
                      this.game.CornerY = 0;
                    this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass1.AddCommand(1, 12);
                    windowReturnClass1.AddCommand(2, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.ButtonStackedUnitId)
                  {
                    this.game.EditObj.SpreadUnit = !this.game.EditObj.SpreadUnit;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass1.AddCommand(4, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.SupplyLayerButtonId)
                  {
                    if (this.game.EditObj.LayerSupplyOn)
                    {
                      this.game.EditObj.LayerSupplyOn = false;
                    }
                    else
                    {
                      this.game.EditObj.LayerSupplyOn = true;
                      let mut unr: i32 =  this.game.EditObj.UnitSelected;
                      if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[471] > 0.0)
                      {
                        let mut index22: i32 =  this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Location2;
                        if (index22 > -1)
                        {
                          if (this.game.Data.LocTypeObj[this.game.Data.LocObj[index22].Type].isSupplySource | this.game.Data.LocTypeObj[this.game.Data.LocObj[index22].Type].isSupplyBase)
                            this.game.HandyFunctionsObj.MakeSupplyLayer2(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                          else
                            index22 = -1;
                        }
                        if (index22 == -1)
                          this.game.HandyFunctionsObj.MakeSupplyLayer3(this.game.SelectX, this.game.SelectY, 0);
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                      }
                      else
                      {
                        if (unr != -1)
                        {
                          if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                          {
                            if (!this.game.Data.UnitObj[unr].IsHQ)
                              unr = this.game.Data.UnitObj[unr].HQ;
                            this.game.EditObj.LayerSupplyHQ = unr;
                            if ( this.game.Data.RuleVar[887] == 1.0)
                            {
                              this.game.EditObj.LayerSupplyHQ = this.game.HandyFunctionsObj.GetTopHQ(unr);
                              unr = this.game.EditObj.LayerSupplyHQ;
                            }
                          }
                          else
                            this.game.EditObj.LayerSupplyHQ = -1;
                        }
                        else
                        {
                          let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                          for (let mut index23: i32 =  0; index23 <= unitCounter; index23 += 1)
                          {
                            if (this.game.Data.UnitObj[index23].Historical > -1 & this.game.Data.UnitObj[index23].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index23].X > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index23].Historical].Type == 8)
                            {
                              this.game.EditObj.LayerSupplyHQ = index23;
                              break;
                            }
                          }
                          this.game.EditObj.LayerSupplyHQ = -1;
                        }
                        if ( this.game.Data.RuleVar[462] > 0.0 && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn && !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].Location].Type].isSupplyBase)
                          this.game.EditObj.LayerSupplyHQ = -1;
                        if (this.game.EditObj.LayerSupplyHQ == -1)
                          this.game.HandyFunctionsObj.MakeSupplyLayer2(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        else
                          this.game.HandyFunctionsObj.MakeSupplyLayer(unr);
                      }
                    }
                    windowReturnClass1.AddCommand(4, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.OfficerId)
                  {
                    this.game.EditObj.OrderType = 45;
                    this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                    this.game.EditObj.OrderX = this.game.SelectX;
                    this.game.EditObj.OrderY = this.game.SelectY;
                    this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                    windowReturnClass1.AddCommand(1, 5);
                    windowReturnClass1.AddCommand(2, 87);
                    this.dostuff();
                    windowReturnClass1.SetFlag(true);
                  }
                  else if (num1 == this.GroupMoveButtonId)
                  {
                    if ( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode)
                    {
                      this.game.EditObj.OrderType = 48;
                      this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePredictionGroup(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, attackoptions: true, ismove: true);
                      this.game.EditObj.TempCoordList.RemoveCoord(0);
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      Coordinate coordinate = this.game.HandyFunctionsObj.SetTempCanSee(this.game.EditObj.TempCoordList);
                      if (coordinate.onmap)
                      {
                        this.game.EditObj.MapSelected = coordinate.map;
                        this.game.SelectX = coordinate.x;
                        this.game.SelectY = coordinate.y;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.UnitSelected = this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, 0);
                        windowReturnClass1.AddCommand(4, 20);
                      }
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    }
                    else
                    {
                      this.game.EditObj.udsUnitOrderMode = 48;
                      ScreenClass screeny = this.formref.Screeny;
                      System.Type type = typeof (MapWindowClass2);
                       System.Type local =  type;
                      MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                      if (!Information.IsNothing( window))
                      {
                        this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
                        if (this.game.EditObj.UnitSelected > -1)
                          window.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, true);
                        else
                          window.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
                      }
                    }
                    windowReturnClass1.AddCommand(4, 12);
                    windowReturnClass1.AddCommand(4, 9);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.SetFlag(true);
                  }
                  else
                  {
                    if (num1 == this.MoveButtonId)
                    {
                      if ( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode)
                      {
                        if (this.game.EditObj.UnitSelected == -1)
                          return windowReturnClass1;
                        this.game.EditObj.OrderType = 1;
                        this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, attackoptions: true, ismove: true);
                        this.game.EditObj.TempCoordList.RemoveCoord(0);
                        this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      }
                      else
                      {
                        this.game.EditObj.udsUnitOrderMode = 1;
                        ScreenClass screeny = this.formref.Screeny;
                        System.Type type = typeof (MapWindowClass2);
                         System.Type local =  type;
                        MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                        if (!Information.IsNothing( window))
                        {
                          this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
                          if (this.game.EditObj.UnitSelected > -1)
                            window.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, true);
                          else
                            window.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
                        }
                      }
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (num1 == this.StrategicButtonId)
                    {
                      this.game.EditObj.OrderTarget = -1;
                      this.game.EditObj.TargetX = -1;
                      this.game.EditObj.TargetY = -1;
                      this.game.EditObj.OrderType = 18;
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      let mut singleCapHq: i32 =  this.game.HandyFunctionsObj.GetSingleCapHQ();
                      if (singleCapHq > -1)
                      {
                        this.game.EditObj.OrderTarget = singleCapHq;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                        windowReturnClass1.AddCommand(1, 69);
                        windowReturnClass1.AddCommand(2, 35);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                    }
                    else if (num1 == this.GroupStrategicButtonId)
                    {
                      this.game.EditObj.OrderTarget = -1;
                      this.game.EditObj.TargetX = -1;
                      this.game.EditObj.TargetY = -1;
                      this.game.EditObj.OrderType = 49;
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      let mut singleCapHq: i32 =  this.game.HandyFunctionsObj.GetSingleCapHQ();
                      if (singleCapHq > -1)
                      {
                        this.game.EditObj.OrderTarget = singleCapHq;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                        windowReturnClass1.AddCommand(1, 69);
                        windowReturnClass1.AddCommand(2, 35);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.SetFlag(true);
                      }
                      this.dostuff();
                      windowReturnClass1.SetFlag(true);
                    }
                    else if (num1 == this.HqUnitButtonId)
                    {
                      this.game.EditObj.OrderType = 3;
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.SetFlag(true);
                      this.game.HandyFunctionsObj.AllHQToTopOutside(this.game.SelectX, this.game.SelectY);
                    }
                    else if (num1 == this.BlowBridgeButtonId)
                    {
                      this.game.EditObj.OrderType = 35;
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      this.game.HandyFunctionsObj.BlowBridgeHexHighlight(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.SetFlag(true);
                    }
                    else if (num1 == this.InfraButtonId)
                    {
                      this.game.EditObj.OrderType = 36;
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.game.HandyFunctionsObj.InfraHexHighlight(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.UnitSelected);
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.SetFlag(true);
                    }
                    else if (num1 == this.AirReconButtonId)
                    {
                      this.game.EditObj.OrderTarget = -1;
                      this.game.EditObj.TargetX = -1;
                      this.game.EditObj.TargetY = -1;
                      this.game.EditObj.OrderType = 33;
                      this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, attack: true, OnlyFrontline: true);
                      this.game.EditObj.TempValue3 = (MapMatrix2[]) this.game.EditObj.TempValue.Clone();
                      this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, false, PredictAirOnly: true, attack: true);
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.SetFlag(true);
                    }
                    else if (num1 == this.AirAttackButtonId)
                    {
                      this.game.EditObj.OrderTarget = -1;
                      this.game.EditObj.SetViewMode = 2;
                      this.game.EditObj.TempUnitList = UnitList::new();
                      this.game.EditObj.OrderType = 14;
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      this.game.EditObj.TargetX = this.game.SelectX;
                      this.game.EditObj.TargetY = this.game.SelectY;
                      this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      this.game.EditObj.SetViewMode = 2;
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.SetFlag(true);
                    }
                    else
                    {
                      if (num1 == this.OrderSurrenderButtonId)
                      {
                        this.game.EditObj.OrderType = 0;
                        this.game.EditObj.QuestionText = "Are you sure you want to surrender?";
                        this.game.EditObj.AnswerCount = 2;
                        this.game.EditObj.AnswerText[1] = "Yes";
                        this.game.EditObj.AnswerText[2] = "No";
                        this.game.EditObj.AnswerTextMouseOver[1] = "Surrender now.";
                        this.game.EditObj.AnswerTextMouseOver[2] = "No don't! Continue turn.";
                        DrawMod.TGame.EditObj.PopupValue = 10;
                        windowReturnClass1.AddCommand(5, 14);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        this.Surrendering = true;
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (num1 == this.TransportButtonId)
                      {
                        DrawMod.TGame.EditObj.PopupValue = 26;
                        windowReturnClass1.AddCommand(5, 14);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (num1 == this.BattleGroupButtonId)
                      {
                        DrawMod.TGame.EditObj.PopupValue = 27;
                        windowReturnClass1.AddCommand(5, 14);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (num1 == this.AttackButtonId)
                      {
                        this.game.EditObj.OrderTarget = -1;
                        this.game.EditObj.SetViewMode = 2;
                        this.game.EditObj.TempUnitList = UnitList::new();
                        this.game.EditObj.OrderType = 2;
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                        this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                        this.game.EditObj.TargetX = this.game.SelectX;
                        this.game.EditObj.TargetY = this.game.SelectY;
                        this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.SetFlag(true);
                      }
                      else if (num1 == this.ArtAttackButtonId)
                      {
                        this.game.EditObj.OrderTarget = -1;
                        this.game.EditObj.SetViewMode = 2;
                        this.game.EditObj.OrderType = 11;
                        this.game.EditObj.TempUnitList = UnitList::new();
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                        this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                        this.game.EditObj.TargetX = this.game.SelectX;
                        this.game.EditObj.TargetY = this.game.SelectY;
                        this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        if (num1 == this.GiveUnitId)
                        {
                          Form3::new( this.formref).Initialize(this.game.Data, 52, this.game.EditObj.UnitSelected);
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                        if (num1 == this.HistoryId)
                        {
                          this.game.EditObj.LayerSupplyOn = false;
                          this.game.EditObj.OrderType = 26;
                          windowReturnClass1.AddCommand(3, 16);
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                        if (num1 == this.Cancelid)
                        {
                          this.game.HandyFunctionsObj.RedimTempValue(9999);
                          if (this.game.EditObj.LayerSupplyOn)
                          {
                            this.game.EditObj.LayerSupplyOn = false;
                            windowReturnClass1.AddCommand(4, 12);
                            windowReturnClass1.AddCommand(4, 9);
                            windowReturnClass1.AddCommand(4, 67);
                            windowReturnClass1.AddCommand(4, 68);
                            windowReturnClass1.SetFlag(true);
                          }
                          switch (this.game.EditObj.OrderType)
                          {
                            case 1:
                            case 43:
                            case 48:
                              this.game.EditObj.OrderType = 0;
                              if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
                              {
                                this.game.EditObj.udsUnitOrderMode = 0;
                                ScreenClass screeny = this.formref.Screeny;
                                System.Type type = typeof (MapWindowClass2);
                                 System.Type local =  type;
                                MapWindowClass2 window = (MapWindowClass2) screeny.GetWindow( local);
                                if (!Information.IsNothing( window))
                                {
                                  this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
                                  if (this.game.EditObj.UnitSelected > -1)
                                    window.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Map, true);
                                  else
                                    window.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
                                }
                              }
                              else
                              {
                                this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                                this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                                this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                                this.game.EditObj.MapSelected = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map;
                              }
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 9);
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 2:
                            case 11:
                            case 12:
                            case 13:
                            case 14:
                            case 15:
                              this.game.EditObj.OrderType = 0;
                              this.lastorderx = -1;
                              this.lastordery = -1;
                              this.game.EditObj.TempCoordList = CoordList::new();
                              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.TargetX, this.game.EditObj.TargetY, this.game.EditObj.MapSelected);
                              if (this.game.EditObj.TempUnitList.counter > -1)
                              {
                                let mut counter: i32 =  this.game.EditObj.TempUnitList.counter;
                                for (let mut index24: i32 =  0; index24 <= counter; index24 += 1)
                                  this.game.EditObj.TempCoordList.AddCoord(this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index24]].X, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index24]].Y, this.game.Data.UnitObj[this.game.EditObj.TempUnitList.unr[index24]].Map);
                              }
                              this.game.EditObj.TargetX = -1;
                              this.game.EditObj.TargetY = -1;
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 9);
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 3:
                            case 4:
                            case 5:
                              this.game.EditObj.OrderType = 0;
                              this.game.EditObj.TempCoordList = CoordList::new();
                              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 9);
                              this.game.SelectX = this.game.EditObj.OrderX;
                              this.game.SelectY = this.game.EditObj.OrderY;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              if (this.game.Data.Round == 0)
                                this.game.Data.Turn = -1;
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 7:
                            case 18:
                            case 44:
                            case 45:
                            case 46:
                            case 49:
                              this.game.EditObj.ShowTransfer = false;
                              this.game.EditObj.TempCoordList = CoordList::new();
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(1, 5);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(2, 69);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 9);
                              this.game.EditObj.OrderType = 0;
                              this.game.EditObj.TargetX = -1;
                              this.game.EditObj.TargetY = -1;
                              this.game.SelectX = this.game.EditObj.OrderX;
                              this.game.SelectY = this.game.EditObj.OrderY;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              this.dostuff();
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 19:
                            case 42:
                              this.game.EditObj.OrderType = 0;
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 18);
                              windowReturnClass1.AddCommand(4, 20);
                              this.game.SelectX = this.game.EditObj.OrderX;
                              this.game.SelectY = this.game.EditObj.OrderY;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              this.dostuff();
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 20:
                            case 21:
                              this.game.EditObj.TempCoordList = CoordList::new();
                              let mut mapCounter: i32 =  this.game.Data.MapCounter;
                              for (let mut index25: i32 =  0; index25 <= mapCounter; index25 += 1)
                              {
                                let mut mapWidth: i32 =  this.game.Data.MapObj[index25].MapWidth;
                                for (let mut index26: i32 =  0; index26 <= mapWidth; index26 += 1)
                                {
                                  let mut mapHeight: i32 =  this.game.Data.MapObj[index25].MapHeight;
                                  for (let mut index27: i32 =  0; index27 <= mapHeight; index27 += 1)
                                    this.game.EditObj.TempValue[index25].Value[index26, index27] = 9999;
                                }
                              }
                              this.game.EditObj.OrderType = 0;
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 9);
                              windowReturnClass1.AddCommand(4, 69);
                              this.game.SelectX = this.game.EditObj.OrderX;
                              this.game.SelectY = this.game.EditObj.OrderY;
                              this.game.EditObj.MapSelected = this.game.EditObj.OrderMap;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              this.dostuff();
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 33:
                            case 35:
                            case 36:
                            case 39:
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 9);
                              this.game.EditObj.OrderType = 0;
                              this.game.EditObj.TargetX = -1;
                              this.game.EditObj.TargetY = -1;
                              this.game.SelectX = this.game.EditObj.OrderX;
                              this.game.SelectY = this.game.EditObj.OrderY;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              windowReturnClass1.SetFlag(true);
                              break;
                            case 40:
                              this.game.EditObj.OrderType = 0;
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 9);
                              this.game.SelectX = this.game.EditObj.OrderX;
                              this.game.SelectY = this.game.EditObj.OrderY;
                              this.game.EditObj.TargetX = -1;
                              this.game.EditObj.TargetY = -1;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              this.game.EditObj.TempCoordList = CoordList::new();
                              this.dostuff();
                              windowReturnClass1.SetFlag(true);
                              break;
                          }
                        }
                      }
                    }
                  }
                }
                return windowReturnClass1;
              }
            }
            let mut cardinhandnr: i32 =  this.ActionButtonCardSlot[index2];
            this.game.EditObj.AreaX = this.game.SelectX;
            this.game.EditObj.AreaY = this.game.SelectY;
            let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
            this.game.ProcessingObj.PlayCard(this.game.Data.Turn, cardinhandnr);
            this.game.EditObj.AreaX = -1;
            this.game.EditObj.AreaY = -1;
            this.game.EditObj.TempCoordList = CoordList::new();
            windowReturnClass1.AddCommand(4, 12);
            windowReturnClass1.AddCommand(4, 69);
            if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
            {
              this.JustAMessage = true;
              this.game.EditObj.PopupValue = 0;
              this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass1.AddCommand(5, 14);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn PopUpRefresh()
    {
      if (this.game.Data.Product >= 6 &  this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode && !this.game.EditObj.battleTimerPopupRefreshDoesntStartIt)
      {
        if (this.game.EditObj.OrderType == 2)
        {
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            this.game.EditObj.BattleTimerActive = true;
            this.game.EditObj.battleTimerInterceptFire = false;
            this.game.EditObj.BattleAnimNr = 0;
            this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
            return;
          }
          this.game.EditObj.TempUnitList = UnitList::new();
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.OrderX = -1;
          this.game.EditObj.OrderY = -1;
          this.game.EditObj.TargetX = -1;
          this.game.EditObj.TargetY = -1;
          this.game.EditObj.TempCoordList = CoordList::new();
        }
        if (this.game.EditObj.OrderType == 11)
        {
          if (this.game.EditObj.TempUnitList.counter > -1)
          {
            this.game.EditObj.BattleTimerActive = true;
            this.game.EditObj.battleTimerInterceptFire = false;
            this.game.EditObj.BattleAnimNr = 0;
            this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
            return;
          }
          this.game.EditObj.TempUnitList = UnitList::new();
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.OrderX = -1;
          this.game.EditObj.OrderY = -1;
          this.game.EditObj.TargetX = -1;
          this.game.EditObj.TargetY = -1;
          this.game.EditObj.TempCoordList = CoordList::new();
        }
      }
      this.game.EditObj.battleTimerPopupRefreshDoesntStartIt = false;
      if (this.JustAMessage)
      {
        this.JustAMessage = false;
        this.DoRefresh();
      }
      else
      {
        this.game.EditObj.HandCard = -1;
        this.EndingTurn &= this.game.EditObj.AnswerChosen == 1;
        this.Surrendering &= this.game.EditObj.AnswerChosen == 1;
        if (this.AskingAboutMetrics)
        {
          if (this.game.EditObj.AnswerChosen == 1)
            this.game.EditObj.allowMetrics = true;
          else
            this.game.EditObj.allowMetrics = false;
          this.game.EditObj.askedMetricsPermission = true;
          this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
        }
        this.lastorderx = -1;
        this.lastordery = -1;
        this.DoRefresh();
      }
    }

    pub DoSurrenderStuff: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      this.game.EditObj.UnitSelected = -1;
      this.game.EditObj.OrderUnit = -1;
      this.game.EditObj.OrderTarget = -1;
      this.game.EditObj.OldUnit = -1;
      let mut humanPlayers: i32 =  this.game.HandyFunctionsObj.GetHumanPlayers();
      switch (humanPlayers)
      {
        case 1:
          this.game.Data.Winner = this.game.HandyFunctionsObj.FindAIPlayer();
          break;
        case 2:
          this.game.Data.Winner = this.game.HandyFunctionsObj.FindOtherHumanPlayer(this.game.Data.Turn);
          break;
        default:
          if (this.game.Data.PbemGameID < 1)
          {
            this.game.EventRelatedObj.ExecJoinRegime(this.game.Data.Turn, -1, 0, 0, "");
            break;
          }
          break;
      }
      if (humanPlayers > 2)
      {
        for (let mut unitCounter: i32 =  this.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
        {
          if (this.game.Data.UnitObj[unitCounter].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unitCounter].PreDef == -1)
          {
            data: DataClass = this.game.Data;
            let mut nr: i32 =  unitCounter;
            let mut gameClass: GameClass = (GameClass) null;
             let mut local: GameClass =  gameClass;
            data.RemoveUnit(nr,  local);
          }
        }
      }
      this.game.EventRelatedObj.ExecMessage2(-1, -1, -1, -1, this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has surrendered.");
      if ( this.game.Data.RuleVar[978] < 1.0)
      {
        this.game.Data.LastWinner = this.game.Data.Winner;
        if (this.game.Data.PbemGameID < 1)
          this.game.Data.RegimeObj[this.game.Data.Turn].Sleep = true;
      }
      this.EndingTurn = true;
      this.dostuff();
      windowReturnClass.SetFlag(true);
      if (humanPlayers > 1 |  this.game.Data.RuleVar[978] > 0.0 | this.game.Data.PbemGameID > 0)
      {
        windowReturnClass.AddCommand(3, 13);
      }
      else
      {
        this.game.Data = DataClass::new();
        this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
        if (this.game.Data.UseAI == 1)
        {
          if (Information.IsNothing( this.game.NewAIObj))
            this.game.NewAIObj = new NewAIClass(this.game);
          this.game.NewAIObj.LastRegime = -1;
        }
        this.game.EditObj.ShowInitialMenu = true;
        windowReturnClass.AddCommand(3, 12);
      }
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub DoEndTurnStuff: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
      for (let mut regnr: i32 =  0; regnr <= regimeCounter; regnr += 1)
      {
        if (this.game.Data.RegimeObj[regnr].Sleep | this.game.Data.RegimeObj[regnr].AI)
          this.game.HandyFunctionsObj.ClearHistory( regnr);
      }
      let mut humanPlayers: i32 =  this.game.HandyFunctionsObj.GetHumanPlayers();
      this.game.EditObj.DoCardSlot = -1;
      this.game.EditObj.HandCard = -1;
      if (this.game.Data.UseAI == 1 && Information.IsNothing( this.game.NewAIObj))
        this.game.NewAIObj = new NewAIClass(this.game);
      if (humanPlayers < 1)
      {
        this.game.Data = DataClass::new();
        this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
        if (this.game.Data.UseAI == 1)
          this.game.NewAIObj.LastRegime = -1;
        this.game.EditObj.ShowInitialMenu = true;
        windowReturnClass.AddCommand(3, 12);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      this.game.FormRef.Cursor = Cursors.WaitCursor;
      if (this.game.EditObj.Screenshoton)
        this.game.HandyFunctionsObj.doscreenshot("b", 0);
      if (this.game.EditObj.AutoSave & !this.game.Data.PBEM)
      {
        str: String = this.game.AppPath_SAVEGAMES + "autosave.se1";
        this.game.Data.serialize(str);
        this.game.HandyFunctionsObj.ZipFile(str);
      }
      if (this.game.Data.Turn != -1 && !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
        this.game.EventRelatedObj.DoCheckEvents(5);
      windowReturnClass.AddCommand(3, 13);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn HighLightAItest()
    {
      let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
      this.game.EditObj.TempCoordList = CoordList::new();
      let mut num: i32 =  -1;
      let mut moveMatrixCounter: i32 =  this.game.NewAIObj.MoveMatrixCounter;
      for (let mut index: i32 =  0; index <= moveMatrixCounter; index += 1)
      {
        if (this.game.NewAIObj.MoveMatrixUnit[index] == unitSelected)
        {
          num = index;
          break;
        }
      }
      this.game.HandyFunctionsObj.RedimTempValue(9999);
      if (num <= -1)
        return;
      if (this.game.Data.UnitObj[unitSelected].TempCategory == 1)
      {
        let mut counter: i32 =  this.game.NewAIObj.MarkerList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.MarkerList.Data1[index], this.game.NewAIObj.MarkerList.Data2[index]] = 0;
      }
      else if (this.game.Data.UnitObj[unitSelected].TempCategory == 2)
      {
        let mut counter: i32 =  this.game.NewAIObj.ArtMarkerList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.ArtMarkerList.Data1[index], this.game.NewAIObj.ArtMarkerList.Data2[index]] = 0;
      }
      else if (this.game.Data.UnitObj[unitSelected].TempCategory == 3)
      {
        let mut counter: i32 =  this.game.NewAIObj.AirMarkerList.Counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
          this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.AirMarkerList.Data1[index], this.game.NewAIObj.AirMarkerList.Data2[index]] = 0;
      }
      else
      {
        if (this.game.Data.UnitObj[unitSelected].TempCategory != 4)
          return;
        if (this.game.NewAIObj.EngineerMarkerList.Counter > -1)
        {
          let mut counter: i32 =  this.game.NewAIObj.EngineerMarkerList.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
            this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.EngineerMarkerList.Data1[index], this.game.NewAIObj.EngineerMarkerList.Data2[index]] = 0;
        }
        else
        {
          let mut counter: i32 =  this.game.NewAIObj.MarkerList.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
            this.game.EditObj.TempValue[0].Value[this.game.NewAIObj.MarkerList.Data1[index], this.game.NewAIObj.MarkerList.Data2[index]] = 0;
        }
      }
    }
  }
}
