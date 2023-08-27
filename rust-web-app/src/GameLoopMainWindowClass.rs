// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.GameLoopMainWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Threading;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class GameLoopMainWindowClass : WindowClass
  {
     TempText: i32;
     TempText2: i32;
     HeadingText: i32;
     Heading2Text: i32;
     EnterTurnId: i32;
     EnterTurnTextId: i32;
     LoginId: i32;
     QuitId: i32;
     okid: i32;
     oktextid: i32;
     Phase: i32;
     PhaseData: i32;
     int[] TextId;
     int[] ButId;
     int[] ButEvent;
     Pic1Id: i32;
     Pic2Id: i32;
     SaveId: i32;
     bool saved;
     bool loggedin;
     TAid: i32;
     refrcount: i32;
     opt9: i32;
     opt3: i32;
     ATListClass ListObj;
     ATListClass ListObj2;
     Listid1: i32;
     listid2: i32;
     cloudid: i32;
     noteid: i32;
     note2id: i32;
     DateTime showtime;
     bool DoingAI;

    pub GameLoopMainWindowClass(ref tGame: GameClass)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.TextId = new int[21];
      this.ButId = new int[21];
      this.ButEvent = new int[21];
      this.Setup();
    }

    pub GameLoopMainWindowClass(ref tGame: GameClass, bool NewGfx)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND1MARC)
    {
      this.TextId = new int[21];
      this.ButId = new int[21];
      this.ButEvent = new int[21];
      this.Setup();
    }

    pub fn Setup()
    {
      this.game.EditObj.Test = -1;
      this.Phase = this.game.EditObj.Phase;
      this.PhaseData = -1;
      this.saved = false;
      this.showtime = DateAndTime.Now;
      this.game.EditObj.AIMoving = false;
      this.game.EditObj.HumanPlayer = -1;
      if (!(this.game.Data.Round == 0 &  this.game.Data.RuleVar[319] < 1.0))
        return;
      let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
      for (let mut index1: i32 =  0; index1 <= sfTypeCounter; index1 += 1)
      {
        let mut killPercent: i32 =  this.game.Data.SFTypeObj[index1].KillPercent;
        if (killPercent > 0)
        {
          let mut num: i32 =   Math.Round( Conversion.Int( killPercent * this.game.Data.RuleVar[319]));
          SFTypeClass[] sfTypeObj = this.game.Data.SFTypeObj;
          SFTypeClass[] sfTypeClassArray = sfTypeObj;
          let mut index2: i32 =  index1;
          let mut index3: i32 =  index2;
          sfTypeClassArray[index3].RetreatPercent = sfTypeObj[index2].RetreatPercent + (this.game.Data.SFTypeObj[index1].KillPercent - num);
          this.game.Data.SFTypeObj[index1].KillPercent = num;
        }
      }
    }

    pub handleTimer: WindowReturnClass()
    {
      if (this.opt9 > 0)
        this.RemoveSubPart(this.opt9);
      if (this.opt3 > 0)
        this.RemoveSubPart(this.opt3);
      if (!Information.IsNothing( this.game.AIThread))
      {
        if (!this.game.AIRunning & this.game.AIThreadRunning & (this.game.EditObj.Test == 10 | this.game.EditObj.Test == 9))
        {
          this.game.AIThreadRunning = false;
          this.game.AIThread.Abort();
          this.game.AIThread.Join();
          this.DoingAI = false;
        }
        else if (this.game.AIThreadRunning & (this.game.EditObj.Test == 9 | this.game.EditObj.Test == 10))
          this.DoingAI = true;
      }
      if (this.DoingAI)
      {
        windowReturnClass: WindowReturnClass = WindowReturnClass::new();
        if (DateAndTime.Now.Subtract(this.showtime).Milliseconds > 200)
        {
          this.showtime = DateAndTime.Now;
          this.game.FormRef.Cursor = Cursors.WaitCursor;
          if (this.TempText > 0)
          {
            this.RemoveSubPart(this.TempText);
            this.TempText = 0;
          }
          if (this.TempText2 > 0)
          {
            this.RemoveSubPart(this.TempText2);
            this.TempText2 = 0;
          }
          this += 1.refrcount;
          if ( this.game.Data.RuleVar[839] == 1.0)
            this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
          else
            this.NewBackGroundAndClearAll(1024, 768, DrawMod.TGame.BACKGROUND2MARC);
          f: Font;
          bool tBlackBack;
          if ( this.game.Data.RuleVar[839] == 1.0)
          {
            f = this.game.MarcFont1;
            tBlackBack = false;
          }
          else
          {
            f = this.game.VicFont1;
            tBlackBack = true;
          }
          if ( this.game.Data.RuleVar[839] == 1.0 & this.game.Data.UseAI == 1)
          {
            this.DrawDateAndRegime();
            let mut tsubpart: SubPartClass =  TextPartClass::new(this.game.EditObj.TempAIString, f, 800, 30, true, tBlackBack: tBlackBack, tMarc: (!tBlackBack));
            this.TempText = this.AddSubPart(ref tsubpart, 120, 330, 800, 30, 0);
            if (this.game.EditObj.AIProgressMax > 0)
            {
              tsubpart =  TextPartClass::new(Strings.Trim(Conversion.Str(  Math.Round( this.game.EditObj.AIProgressNow /  this.game.EditObj.AIProgressMax * 100.0))) + "% completed", this.game.MarcFont1, 600, 40, true, tBlackBack: true, tProgress: ( Math.Round( this.game.EditObj.AIProgressNow /  this.game.EditObj.AIProgressMax * 100.0)), tMarc: true);
              this.TempText2 = this.AddSubPart(ref tsubpart, 220, 375, 600, 100, 0);
            }
          }
          else
          {
            let mut tsubpart: SubPartClass =  new ATTextPartClass("AI step " + Strings.Trim(Conversion.Str( this.refrcount)), f, 800, 30, true, tBlackBack: tBlackBack, tMarc: (!tBlackBack));
            this.TempText = this.AddSubPart(ref tsubpart, 110, 365, 800, 30, 0);
          }
          windowReturnClass.Flag = true;
        }
        return windowReturnClass;
      }
      this.game.EditObj.TempCoordList = CoordList::new();
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (this.Phase == -1)
      {
        this.Phase = 1;
        this.game.EditObj.Phase = 0;
        this.saved = true;
        this.game.FormRef.Cursor = Cursors.Default;
        this.EnterTurn();
        windowReturnClass1.Flag = true;
        return windowReturnClass1;
      }
      if (this.Phase > 0)
      {
        windowReturnClass1.Flag = false;
        return windowReturnClass1;
      }
      this.game.EditObj.FromMessage = -1;
      this.game.EditObj.Phase = 0;
      this.game.EditObj.OrderType = 0;
      this += 1.game.EditObj.Test;
      txt: String;
      if (this.game.EditObj.Test == 0)
      {
        this.game.FormRef.Cursor = Cursors.WaitCursor;
        if (this.game.Data.Turn > -1)
          txt = "End of turn";
        if (this.game.Data.Round == 0)
          txt = "Setting up scenario.";
        this.game.EditObj.LayerSupplyOn = false;
      }
      if (this.game.EditObj.Test == 1)
      {
        this.game.FormRef.Cursor = Cursors.WaitCursor;
        if (this.game.Data.Turn > -1 & this.game.Data.Round != 0)
        {
          this.game.ProcessingObj.SetInitialReconAndZOC(this.game.Data.Turn);
          this.game.HandyFunctionsObj.ClearHistory( this.game.Data.Turn);
        }
        if (this.game.Data.Round == 0)
        {
          if (this.game.Data.UseAI == 1)
            this.game.NewAIObj.tempextraaivp = false;
          if ( this.game.Data.RuleVar[840] == 1.0)
          {
            let mut mapCounter: i32 =  this.game.Data.MapCounter;
            for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
            {
              let mut mapWidth: i32 =  this.game.Data.MapObj[index1].MapWidth;
              for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
              {
                let mut mapHeight: i32 =  this.game.Data.MapObj[index1].MapHeight;
                for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
                  this.game.Data.MapObj[index1].HexObj[index2, index3].OrigOwner = this.game.Data.MapObj[index1].HexObj[index2, index3].Regime;
              }
            }
          }
          if ( this.game.Data.RuleVar[501] == 1.0)
            this.game.Data.RuleVar[226] = 0.0f;
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut regnr: i32 =  0; regnr <= regimeCounter; regnr += 1)
          {
            this.game.ProcessingObj.InitialAPPenalty(regnr, true);
            this.game.EditObj.firstroundcheck = true;
            if (this.game.Data.ShrowdPeek)
              this.game.ProcessingObj.SetInitialPeek(regnr);
            let mut sfCounter: i32 =  this.game.Data.SFCounter;
            for (let mut index: i32 =  0; index <= sfCounter; index += 1)
              this.game.Data.SFObj[index].Ap = this.game.Data.SFObj[index].Rdn;
            let mut turn: i32 =  this.game.Data.Turn;
            this.game.Data.Turn = regnr;
            this.game.Data.Round = 1;
            this.game.ProcessingObj.SetInitialReconAndZOC(regnr);
            this.game.HandyFunctionsObj.ClearHistory( regnr);
            this.game.Data.Round = 0;
            this.game.Data.Turn = turn;
          }
        }
        txt = "Add turn";
      }
      if (this.game.EditObj.Test == 2)
      {
        txt = "Next round";
        if (this.game.Data.Turn != -1)
          this.game.ProcessingObj.InitialAPPenalty(this.game.Data.Turn, true);
        if (this.game.Data.Turn != -1)
          this.game.EventRelatedObj.DoCheckEvents(5);
        this += 1.game.Data.Turn;
      }
      bool sleep;
      if (this.game.EditObj.Test == 3)
      {
        txt = "Round events";
        if (this.game.Data.Round == 0 | this.game.Data.Turn > this.game.Data.RegimeCounter)
        {
          this += 1.game.Data.Round;
          if (this.game.Data.Turn > this.game.Data.RegimeCounter)
            this.game.Data.Turn = 0;
          if (this.game.Data.Round > 1)
            this.game.Data.Turn = 0;
          this.game.HandyFunctionsObj.RedimStats();
          this.game.ProcessingObj.CheckForWinner();
          this.game.EventRelatedObj.DoCheckEvents(1);
          this.DoPresentStats();
          if (this.game.Data.Round == 1 & this.game.Data.Turn == 0)
            this.game.ProcessingObj.MakeInitialModels();
        }
        if (this.game.Data.RegimeObj[this.game.Data.Turn].Sleep)
        {
          let mut num: i32 =  0;
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
          {
            if (this.game.Data.RegimeObj[index].UberRegime == this.game.Data.Turn)
              num += 1;
          }
          if (num == 0)
          {
            this.game.ProcessingObj.SetInitialReconAndZOC(this.game.Data.Turn);
            this.game.HandyFunctionsObj.ClearHistory( this.game.Data.Turn);
            this.game.EditObj.Test = 1;
          }
        }
        sleep = this.game.Data.RegimeObj[this.game.Data.Turn].Sleep;
      }
      if (this.game.Data.Round > 1)
        this.game.EditObj.CombatSim = false;
      if (this.game.EditObj.Test == 4)
      {
        if (this.game.Data.Round > 1)
          this.game.EditObj.CombatSim = false;
        txt = "Supply system";
        this.game.ProcessingObj.InitialAPPenalty(this.game.Data.Turn, false);
        this.game.HandyFunctionsObj.DoAntiInfraDammage();
        this.game.ProcessingObj.SetUberOn();
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.EventRelatedObj.DoCheckEvents(2);
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.LocationProduction();
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.SetCapForUnitS();
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1 &  this.game.Data.RuleVar[337] == 1.0)
          this.game.ProcessingObj.DoAutoReinforce();
      }
      if (this.game.EditObj.Test == 5)
        txt = "Supply system";
      if (this.game.EditObj.Test == 6)
      {
        txt = "Supply consumption";
        if ( this.game.Data.RuleVar[333] == 0.0 && this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.DoSupplySystem();
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.ApToSf();
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.DoEntrench(this.game.Data.Turn);
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.DoMorale(this.game.Data.Turn);
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.DoAutoRecoverLocations(this.game.Data.Turn);
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1)
          this.game.ProcessingObj.DoTraining(this.game.Data.Turn);
        if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1 &  this.game.Data.RuleVar[79] == 1.0)
          this.game.ProcessingObj.AutoConquerNeutral(this.game.Data.Turn);
        this.game.ProcessingObj.SetUberOff();
      }
      if (this.game.EditObj.Test == 7)
      {
        txt = "Miscellaneous calculations";
        if (this.game.Data.RegimeObj[this.game.Data.Turn].Sleep & sleep)
        {
          this.game.ProcessingObj.SetInitialReconAndZOC(this.game.Data.Turn);
          this.game.EditObj.Test = 1;
        }
      }
      if (this.game.EditObj.Test == 8)
      {
        this.game.ProcessingObj.SetInitialReconAndZOC(this.game.Data.Turn);
        this.game.EventRelatedObj.DoCheckEvents(3);
        this.game.ProcessingObj.DoDeckCards();
        this.game.ProcessingObj.DoAutoEvents();
        this.game.ProcessingObj.SetExtraStat(this.game.Data.Turn);
        this.game.ProcessingObj.IntialZOCConquestCheck(this.game.Data.Turn);
        this.game.EditObj.MiniMap = new Bitmap(205, 110);
        this.game.EditObj.MiniMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 205, 110, false);
        this.game.EditObj.StratMap = new Bitmap(this.game.ScreenWidth, this.game.ScreenHeight - 305);
        this.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.StratMap, this.game.ScreenWidth, this.game.ScreenHeight - 305, false, true, false);
        if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI)
          this.game.HandyFunctionsObj.SetInitialXY(this.game.Data.Turn);
        if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI)
        {
          this.game.EditObj.UnitSelected = -1;
          this.game.EditObj.OrderType = 0;
          this.Phase = 1;
          this.game.Data.InTurn = false;
          this.loggedin = false;
          this.game.EditObj.LastHistoryStep = -1;
          this.game.EditObj.TempAIInitialized = false;
          this.game.FormRef.Cursor = Cursors.Default;
          this.game.EditObj.SFSelected = -1;
          if (this.game.Data.CampaignRoom > -1)
          {
            windowReturnClass1.Flag = true;
            windowReturnClass1.AddCommand(3, 8);
            return windowReturnClass1;
          }
          if (this.game.EditObj.SoundOn & this.game.Data.Winner == -1)
            SoundMod.PlayAWave(this.game.AppPath + "sound/startturn.wav", ref this.game.EditObj);
          else if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Winner, this.game.Data.Turn))
            SoundMod.PlayAWave(this.game.AppPath + "sound/victory.wav", ref this.game.EditObj);
          else if (this.game.Data.Winner > -1)
            SoundMod.PlayAWave(this.game.AppPath + "sound/defeat.wav", ref this.game.EditObj);
          if (!this.game.Data.PBEM & !this.game.Data.PasswordsOn)
          {
            this.loggedin = true;
            this.game.EventRelatedObj.DoCheckEvents(4);
          }
          this.EnterTurn();
          windowReturnClass1.Flag = true;
          return windowReturnClass1;
        }
        this.game.EventRelatedObj.DoCheckEvents(4);
        txt = "Setting up initialization";
      }
      if (this.game.EditObj.Test == 9)
      {
        txt = "AI for " + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " is playing. Executing";
        if ( this.game.Data.RuleVar[814] == 0.0)
        {
          if (this.game.Data.UseAI == 0)
          {
            this.DoingAI = true;
            this.game.AIObj.InitAI();
            this.DoingAI = false;
          }
          if (this.game.Data.UseAI == 1)
          {
            this.game.AIRunning = true;
            this.game.AIThreadRunning = true;
            this.game.AIThread = new Thread(new ThreadStart(this.game.NewAIObj.InitAI));
            this.game.AIThread.Name = "AI Init Thread";
            this.game.AIThread.Start();
          }
        }
      }
      if (this.game.EditObj.Test == 10 &&  this.game.Data.RuleVar[814] == 0.0)
      {
        if (this.game.HandyFunctionsObj.GetHumanPlayers() == 1 & !this.game.Data.DontShowAIMove)
        {
          this.game.AIRunning = true;
          this.game.EditObj.TempAIWatch = true;
          this.game.AIThreadRunning = true;
          if (this.game.Data.UseAI == 0)
            this.game.AIThread = new Thread(new ThreadStart(this.game.AIObj.ExecuteAI));
          if (this.game.Data.UseAI == 1)
            this.game.AIThread = new Thread(new ThreadStart(this.game.NewAIObj.ExecuteAI));
          this.game.AIThread.Name = "AI Execute Thread";
          this.game.AIThread.Start();
          windowReturnClass1.Flag = true;
          windowReturnClass1.AddCommand(3, 6);
          this.game.FormRef.Cursor = Cursors.Default;
          return windowReturnClass1;
        }
        if (this.game.Data.UseAI == 0)
        {
          txt = "AI for " + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " is playing...";
          this.DoingAI = true;
          this.game.AIObj.ExecuteAI();
          this.DoingAI = false;
        }
        else
        {
          this.game.AIRunning = true;
          this.game.EditObj.TempAIWatch = true;
          this.game.AIThreadRunning = true;
          if (this.game.Data.UseAI == 0)
            this.game.AIThread = new Thread(new ThreadStart(this.game.AIObj.ExecuteAI));
          if (this.game.Data.UseAI == 1)
            this.game.AIThread = new Thread(new ThreadStart(this.game.NewAIObj.ExecuteAI));
          this.game.AIThread.Name = "AI Execute Thread";
          this.game.AIThread.Start();
        }
      }
      if (this.game.EditObj.Test >= 11)
      {
        this.game.EditObj.Test = 0;
        txt = "Finished".to_owned();
        this.DoingAI = true;
        if ( this.game.Data.RuleVar[814] == 0.0)
        {
          if (this.game.Data.UseAI == 0)
            this.game.AIObj.CloseAI();
          if (this.game.Data.UseAI == 1)
            this.game.NewAIObj.CloseAI();
        }
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          this.game.Data.UnitObj[index].TempUnitSelectable = false;
        this.DoingAI = false;
        if (this.game.HandyFunctionsObj.GetHumanPlayers() == 0 && this.game.EditObj.Screenshoton)
          this.game.HandyFunctionsObj.doscreenshot("ai", 0);
      }
      if (this.TempText > 0)
        this.RemoveSubPart(this.TempText);
      if (this.TempText2 > 0)
        this.RemoveSubPart(this.TempText2);
      f1: Font;
      bool tBlackBack1;
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        f1 = this.game.MarcFont1;
        tBlackBack1 = false;
      }
      else
      {
        f1 = this.game.VicFont1;
        tBlackBack1 = true;
      }
      if ( this.game.Data.RuleVar[839] == 1.0)
        this.DrawDateAndRegime();
      let mut tsubpart1: SubPartClass =  new ATTextPartClass(txt, f1, 800, 30, true, tBlackBack: tBlackBack1, tMarc: (!tBlackBack1));
      this.TempText = this.AddSubPart(ref tsubpart1, 110, 360, 800, 30, 0);
      windowReturnClass1.Flag = true;
      return windowReturnClass1;
    }

    pub fn RemoveEnterTurnViewMessageShit()
    {
      if (this.HeadingText > 0)
        this.RemoveSubPart(this.HeadingText);
      if (this.Heading2Text > 0)
        this.RemoveSubPart(this.Heading2Text);
      if (this.EnterTurnId > 0)
        this.RemoveSubPart(this.EnterTurnId);
      if (this.EnterTurnTextId > 0)
        this.RemoveSubPart(this.EnterTurnTextId);
      if (this.TempText > 0)
        this.RemoveSubPart(this.TempText);
      if (this.TempText2 > 0)
        this.RemoveSubPart(this.TempText2);
      if (this.SaveId > 0)
        this.RemoveSubPart(this.SaveId);
      if (this.SaveId > 0)
        this.RemoveSubPart(this.SaveId);
      if (this.HeadingText > 0)
        this.RemoveSubPart(this.HeadingText);
      if (this.Heading2Text > 0)
        this.RemoveSubPart(this.Heading2Text);
      if (this.EnterTurnId > 0)
        this.RemoveSubPart(this.EnterTurnId);
      if (this.EnterTurnTextId > 0)
        this.RemoveSubPart(this.EnterTurnTextId);
      if (this.LoginId > 0)
        this.RemoveSubPart(this.LoginId);
      if (this.QuitId > 0)
        this.RemoveSubPart(this.QuitId);
      let mut index: i32 =  0;
      do
      {
        if (this.TextId[index] > 0)
          this.RemoveSubPart(this.TextId[index]);
        if (this.ButId[index] > 0)
          this.RemoveSubPart(this.ButId[index]);
        this.ButEvent[index] = -1;
        index += 1;
      }
      while (index <= 20);
      if (this.Pic1Id > 0)
        this.RemoveSubPart(this.Pic1Id);
      if (this.Pic2Id > 0)
        this.RemoveSubPart(this.Pic2Id);
      if (this.okid > 0)
        this.RemoveSubPart(this.okid);
      if (this.oktextid <= 0)
        return;
      this.RemoveSubPart(this.oktextid);
    }

    pub fn DrawDateAndRegime()
    {
      SizeF sizeF1 = SizeF::new();
      if (this.game.Data.Turn == -1 | this.game.Data.Turn > this.game.Data.RegimeCounter)
        return;
      GC.Collect();
      str1: String;
      str2: String;
      name: String;
      if (this.game.Data.AlternateRound > -1)
      {
        DateTime dateTime1 = DateTime::new();
        dateTime1 = dateTime1.AddYears(this.game.Data.StartData.Year - 1);
        DateTime dateTime2 = dateTime1.AddMonths(this.game.Data.StartData.Month - 1).AddDays( (this.game.Data.StartData.Day - 1));
        DateTime dateTime3;
        if (this.game.Data.AlternateRound == 31)
        {
          dateTime3 = dateTime2.AddMonths((this.game.Data.Round - 1) * 1);
        }
        else
        {
          TimeSpan timeSpan = new TimeSpan((this.game.Data.Round - 1) * this.game.Data.AlternateRound, 0, 0, 0);
          dateTime3 = dateTime2.Add(timeSpan);
        }
        str3: String = "";
        str1 = "";
        str2 = "Date: " + (str3 + this.game.HandyFunctionsObj.GetMonth(dateTime3.Month) + " " + Strings.Trim(Conversion.Str( dateTime3.Day)) + " " + Strings.Trim(Conversion.Str( dateTime3.Year)));
        name = this.game.Data.RegimeObj[this.game.Data.Turn].Name;
      }
      else
      {
        name = this.game.Data.RegimeObj[this.game.Data.Turn].Name;
        str2 = "Round " + Strings.Trim(Conversion.Str( this.game.Data.Round));
        if (this.game.Data.AlternateRound2 > -1)
        {
          DateTime dateTime4 = DateTime::new();
          dateTime4 = dateTime4.AddYears(this.game.Data.StartData.Year - 1);
          DateTime dateTime5 = dateTime4.AddMonths(this.game.Data.StartData.Month - 1).AddDays( (this.game.Data.StartData.Day - 1)).AddHours( this.game.Data.StartData.Hour).Add(new TimeSpan(0, (this.game.Data.Round - 1) * this.game.Data.AlternateRound2, 0, 0));
          str4: String = "";
          str1 = "";
          str2 = "Date: " + (str4 + this.game.HandyFunctionsObj.GetMonth(dateTime5.Month) + " " + Strings.Trim(Conversion.Str( dateTime5.Day)) + " " + Strings.Trim(Conversion.Str( dateTime5.Hour)) + ":00" + " " + Strings.Trim(Conversion.Str( dateTime5.Year)));
          name = this.game.Data.RegimeObj[this.game.Data.Turn].Name;
        }
      }
      if ( this.game.Data.RuleVar[839] == 1.0)
        this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
      else
        this.NewBackGroundAndClearAll(1024, 768, DrawMod.TGame.BACKGROUND2MARC);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        ref Graphics local1 = ref Expression;
        bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.LOGOFLATTINY);
        ref local2: Bitmap = ref bitmap1;
        let mut x: i32 =  this.OwnBitmap.Width - (BitmapStore.GetWidth(this.game.LOGOFLATTINY) + 50);
        DrawMod.DrawSimple(ref local1, ref local2, x, 33);
        if (this.game.Data.Turn > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RoundelIconSprite > -1)
        {
          ref Graphics local3 = ref Expression;
          bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.Data.RegimeObj[this.game.Data.Turn].RoundelIconSprite);
          ref local4: Bitmap = ref bitmap2;
          DrawMod.DrawScaled(ref local3, ref local4, 4, -2, 90, 90);
        }
      }
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        DrawMod.DrawTextColouredMarc(ref Expression, name, this.game.MarcFont1, 85, 15, Color.White);
        sizeF1 = Expression.MeasureString(str2, this.game.MarcFont3);
        DrawMod.DrawTextColouredMarc(ref Expression, str2, this.game.MarcFont3, 90, 45, Color.White);
        DrawMod.DrawBlock(ref Expression, 92, 82, 878, 3,  this.game.MarcCol4.R,  this.game.MarcCol4.G,  this.game.MarcCol4.B,  this.game.MarcCol4.A);
        DrawMod.DrawBlock(ref Expression, 92, 675, 878, 3,  this.game.MarcCol4.R,  this.game.MarcCol4.G,  this.game.MarcCol4.B,  this.game.MarcCol4.A);
      }
      else
      {
        DrawMod.DrawSteveBlock(ref Expression, 225, 15, 690, 28);
        DrawMod.DrawTextVic2(ref Expression, name, this.game.VicFont1, 225, 15, this.game.VicColor2, this.game.VicColor2Shade);
        SizeF sizeF2 = Expression.MeasureString(str2, this.game.VicFont1);
        DrawMod.DrawTextVic2(ref Expression, str2, this.game.VicFont1,  Math.Round( (900f - sizeF2.Width)), 15, this.game.VicColor2, this.game.VicColor2Shade);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub fn EnterTurn()
    {
      SizeF sizeF = SizeF::new();
      if (this.HeadingText > 0)
        this.RemoveSubPart(this.HeadingText);
      if (this.Heading2Text > 0)
        this.RemoveSubPart(this.Heading2Text);
      if (this.EnterTurnId > 0)
        this.RemoveSubPart(this.EnterTurnId);
      if (this.EnterTurnTextId > 0)
        this.RemoveSubPart(this.EnterTurnTextId);
      if (this.TempText > 0)
        this.RemoveSubPart(this.TempText);
      if (this.TempText2 > 0)
        this.RemoveSubPart(this.TempText2);
      if (this.SaveId > 0)
        this.RemoveSubPart(this.SaveId);
      if (this.Listid1 > 0)
        this.RemoveSubPart(this.Listid1);
      if (this.listid2 > 0)
        this.RemoveSubPart(this.listid2);
      if (this.LoginId > 0)
      {
        this.RemoveSubPart(this.LoginId);
        this.LoginId = 0;
      }
      if (this.QuitId > 0)
        this.RemoveSubPart(this.QuitId);
      this.game.FormRef.Cursor = Cursors.Default;
      if (this.game.EditObj.TutStep < 1)
        this.game.EditObj.TutStep = 1;
      if ( this.game.Data.RuleVar[839] == 1.0)
        this.DrawDateAndRegime();
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      txt: String;
      if (!this.loggedin)
        txt = this.game.Data.RegimeObj[this.game.Data.Turn].Name + ", Its your turn. Please login";
      else if (this.game.Data.VPWin == -1)
        txt = this.game.Data.Winner != -1 ? (!(this.game.Data.Winner == this.game.Data.Turn | this.game.Data.Winner == this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime) ? this.game.Data.RegimeObj[this.game.Data.Winner].Name + " has won this game. You have lost!" : this.game.Data.RegimeObj[this.game.Data.Turn].Name + ", You have won this game!") : this.game.Data.RegimeObj[this.game.Data.Turn].Name + ", Game is in progress. Please start turn.";
      else if (this.game.Data.Winner == -1)
        txt = this.game.Data.RegimeObj[this.game.Data.Turn].Name + ", You have" + Conversion.Str( this.game.HandyFunctionsObj.GetRegimeVP(this.game.Data.Turn)) + " VP of the" + Conversion.Str( this.game.Data.VPWin) + " VP required to win.";
      else
        txt = this.game.Data.Winner != this.game.Data.Turn ? this.game.Data.RegimeObj[this.game.Data.Winner].Name + " has won this game. You have lost!" : this.game.Data.RegimeObj[this.game.Data.Turn].Name + ", You have won this game!";
      bool flag;
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        marcFont1: Font = this.game.MarcFont1;
        flag = false;
        let mut tsubpart: SubPartClass =  TextPartClass::new(txt, marcFont1, 724, 40, true, tMarc: (!flag));
        this.Heading2Text = this.AddSubPart(ref tsubpart, 150, 360, 724, 30, 0);
      }
      else
      {
        vicFont1: Font = this.game.VicFont1;
        flag = true;
        let mut tsubpart: SubPartClass =  new ATTextPartClass(txt, vicFont1, 724, 40, true, tMarc: (!flag));
        this.Heading2Text = this.AddSubPart(ref tsubpart, 150, 150, 724, 30, 0);
      }
      usefont: Font =  this.game.Data.RuleVar[839] != 1.0 ?  null : this.game.MarcFont4;
      SubPartClass tsubpart1;
      if (!this.loggedin)
      {
        buttontext: String =  this.game.Data.RuleVar[839] != 1.0 ? "Login" : "LOGIN".to_owned();
        if (!this.saved)
        {
          tsubpart1 =  new TextButtonPartClass(buttontext, 100, "You have to login if playing PBEM and/or using passwords.", ref this.OwnBitmap, 400, 710, usefont: usefont, useshadow: (!flag), tMarcStyle: (!flag));
          this.LoginId = this.AddSubPart(ref tsubpart1, 400, 710, 100, 35, 1);
        }
        else
        {
          tsubpart1 =  new TextButtonPartClass(buttontext, 100, "You have to login if playing PBEM and/or using passwords.", ref this.OwnBitmap, 455, 710, usefont: usefont, useshadow: (!flag), tMarcStyle: (!flag));
          this.LoginId = this.AddSubPart(ref tsubpart1, 455, 710, 100, 35, 1);
        }
      }
      buttontext1: String =  this.game.Data.RuleVar[839] != 1.0 ? "Save & Quit" : "SAVE + QUIT";
      if (!this.loggedin & !this.saved)
      {
        tsubpart1 =  new TextButtonPartClass(buttontext1, 100, "If your playing a PBEM and your not playing\r\nthis regime this is a good place to save and quit.", ref this.OwnBitmap, 530, 710, usefont: usefont, useshadow: (!flag), tMarcStyle: (!flag));
        this.SaveId = this.AddSubPart(ref tsubpart1, 530, 710, 100, 35, 1);
      }
      buttontext2: String =  this.game.Data.RuleVar[839] != 1.0 ? "Start Turn" : "START TURN";
      if (this.loggedin)
      {
        tsubpart1 =  new TextButtonPartClass(buttontext2, 100, "Click to start the turn.", ref this.OwnBitmap, 455, 710, usefont: usefont, useshadow: (!flag), tMarcStyle: (!flag));
        this.EnterTurnId = this.AddSubPart(ref tsubpart1, 455, 710, 100, 35, 1);
      }
      if ( this.game.Data.RuleVar[839] == 0.0 && this.loggedin)
      {
        this.ListObj = ATListClass::new();
        let mut itemTypeCounter: i32 =  this.game.Data.ItemTypeCounter;
        tdata1: i32;
        for (tdata1 = 0; tdata1 <= itemTypeCounter; tdata1 += 1)
        {
          if (this.game.Data.RegimeObj[this.game.Data.Turn].SProd[tdata1, this.game.Data.Round] > 0 | this.game.Data.RegimeObj[this.game.Data.Turn].SASProdLost[tdata1] > 0)
          {
            name: String = this.game.Data.ItemTypeObj[tdata1].Name;
            tvalue2: String = "-";
            let mut Number1: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].SProd[tdata1, this.game.Data.Round];
            if (this.game.Data.ItemTypeObj[tdata1].IsSFType > -1)
              Number1 *= this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[tdata1].IsSFType].Ratio;
            tvalue: String = Strings.Trim(Conversion.Str( Number1));
            if (this.game.Data.ASOn && this.game.Data.RegimeObj[this.game.Data.Turn].SASProdLost[tdata1] > 0)
            {
              let mut Number2: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].SASProdLost[tdata1];
              if (this.game.Data.ItemTypeObj[tdata1].IsSFType > -1)
                Number2 *= this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[tdata1].IsSFType].Ratio;
              tvalue2 = Strings.Trim(Conversion.Str( Number2));
            }
            this.ListObj.add(name, tdata1, tvalue, tvalue2);
          }
        }
        if (!this.game.Data.ASOn)
        {
          tsubpart1 =  new ATListSubPartClass(this.ListObj, 18, 250, -1, this.game, tHeader: "Type                                 Prod", tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tbackbitmap: (ref this.OwnBitmap), bbx: 225, bby: 210);
          this.Listid1 = this.AddSubPart(ref tsubpart1, 185, 210, 250, 336, 0);
        }
        else
        {
          tsubpart1 =  new ATListSubPartClass(this.ListObj, 18, 250, -1, this.game, tHeader: "Type                                 Prod    AS Loss", tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tbackbitmap: (ref this.OwnBitmap), bbx: 185, bby: 210);
          this.Listid1 = this.AddSubPart(ref tsubpart1, 225, 210, 250, 336, 0);
        }
        this.ListObj2 = ATListClass::new();
        if (this.game.Data.ASOn)
        {
          tname: String = "Supply".to_owned();
          tvalue: String = "-";
          tvalue2: String = "-";
          tvalue3: String = "-";
          if (this.game.Data.RegimeObj[this.game.Data.Turn].SASSupplyLost > 0)
            tvalue = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].SASSupplyLost));
          if (this.game.Data.RegimeObj[this.game.Data.Turn].SASSupplyKilled > 0)
            tvalue3 = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].SASSupplyKilled));
          this.ListObj2.add(tname, tdata1, tvalue, tvalue2, tvalue3);
        }
        int[] numArray1 = new int[100];
        int[] numArray2 = new int[100];
        int[] numArray3 = new int[100];
        let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
        for (let mut index1: i32 =  0; index1 <= sfTypeCounter; index1 += 1)
        {
          if (this.game.Data.RegimeObj[this.game.Data.Turn].SLoss[index1, this.game.Data.Round] > 0 | this.game.Data.RegimeObj[this.game.Data.Turn].SKills[index1, this.game.Data.Round] > 0 | this.game.Data.RegimeObj[this.game.Data.Turn].SASKilled[index1] > 0)
          {
            int[] numArray4 = numArray1;
            int[] numArray5 = numArray4;
            let mut unitGroup1: i32 =  this.game.Data.SFTypeObj[index1].UnitGroup;
            let mut index2: i32 =  unitGroup1;
            let mut num1: i32 =  numArray4[unitGroup1] + this.game.Data.RegimeObj[this.game.Data.Turn].SLoss[index1, this.game.Data.Round] * this.game.Data.SFTypeObj[index1].Ratio;
            numArray5[index2] = num1;
            int[] numArray6 = numArray2;
            int[] numArray7 = numArray6;
            let mut unitGroup2: i32 =  this.game.Data.SFTypeObj[index1].UnitGroup;
            let mut index3: i32 =  unitGroup2;
            let mut num2: i32 =  numArray6[unitGroup2] + this.game.Data.RegimeObj[this.game.Data.Turn].SKills[index1, this.game.Data.Round] * this.game.Data.SFTypeObj[index1].Ratio;
            numArray7[index3] = num2;
            int[] numArray8 = numArray3;
            int[] numArray9 = numArray8;
            let mut unitGroup3: i32 =  this.game.Data.SFTypeObj[index1].UnitGroup;
            let mut index4: i32 =  unitGroup3;
            let mut num3: i32 =  numArray8[unitGroup3] + this.game.Data.RegimeObj[this.game.Data.Turn].SASKilled[index1] * this.game.Data.SFTypeObj[index1].Ratio;
            numArray9[index4] = num3;
          }
        }
        let mut tdata2: i32 =  0;
        do
        {
          if (numArray1[tdata2] > 0 | numArray2[tdata2] > 0 | numArray3[tdata2] > 0)
          {
            str: String = this.game.Data.TempString[400 + tdata2];
            tvalue: String = "-";
            tvalue2: String = "-";
            tvalue3: String = "-";
            if (Strings.Len(str) > 15)
              str = Strings.Left(str, 15);
            if (numArray1[tdata2] > 0)
              tvalue = Strings.Trim(Conversion.Str( numArray1[tdata2]));
            if (numArray2[tdata2] > 0)
              tvalue2 = Strings.Trim(Conversion.Str( numArray2[tdata2]));
            if (this.game.Data.ASOn && numArray3[tdata2] > 0)
              tvalue3 = Strings.Trim(Conversion.Str( numArray3[tdata2]));
            this.ListObj2.add(str, tdata2, tvalue, tvalue2, tvalue3);
          }
          tdata2 += 1;
        }
        while (tdata2 <= 99);
        if (this.game.Data.ASOn)
        {
          tsubpart1 =  new ATListSubPartClass(this.ListObj2, 18, 350, -1, this.game, tHeader: "Type                                                Losses Kills   AS Kill", tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 150, tbackbitmap: (ref this.OwnBitmap), bbx: 490, bby: 210);
          this.listid2 = this.AddSubPart(ref tsubpart1, 490, 210, 350, 336, 0);
        }
        else
        {
          tsubpart1 =  new ATListSubPartClass(this.ListObj2, 18, 350, -1, this.game, tHeader: "Type                                                Losses Kills", tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 150, tbackbitmap: (ref this.OwnBitmap), bbx: 490, bby: 210);
          this.listid2 = this.AddSubPart(ref tsubpart1, 490, 210, 350, 336, 0);
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub fn ViewMessage()
    {
      strArray: Vec<String> = new string[1000];
      if (this.SaveId > 0)
        this.RemoveSubPart(this.SaveId);
      if (this.HeadingText > 0)
        this.RemoveSubPart(this.HeadingText);
      if (this.Heading2Text > 0)
        this.RemoveSubPart(this.Heading2Text);
      if (this.EnterTurnId > 0)
      {
        this.RemoveSubPart(this.EnterTurnId);
        this.EnterTurnId = 0;
      }
      if (this.EnterTurnTextId > 0)
        this.RemoveSubPart(this.EnterTurnTextId);
      if (this.Listid1 > 0)
        this.RemoveSubPart(this.Listid1);
      if (this.listid2 > 0)
        this.RemoveSubPart(this.listid2);
      let mut index1: i32 =  0;
      do
      {
        if (this.TextId[index1] > 0)
          this.RemoveSubPart(this.TextId[index1]);
        if (this.ButId[index1] > 0)
          this.RemoveSubPart(this.ButId[index1]);
        this.ButEvent[index1] = -1;
        index1 += 1;
      }
      while (index1 <= 20);
      if (this.Pic1Id > 0)
        this.RemoveSubPart(this.Pic1Id);
      if (this.Pic2Id > 0)
        this.RemoveSubPart(this.Pic2Id);
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.oktextid > 0)
        this.RemoveSubPart(this.oktextid);
      if (this.TAid > 0)
        this.RemoveSubPart(this.TAid);
      if (this.cloudid > 0)
      {
        this.RemoveSubPart(this.cloudid);
        this.cloudid = 0;
      }
      if (this.noteid > 0)
      {
        this.RemoveSubPart(this.noteid);
        this.noteid = 0;
      }
      if (this.note2id > 0)
      {
        this.RemoveSubPart(this.note2id);
        this.note2id = 0;
      }
      this += 1.PhaseData;
      this.game.FormRef.Cursor = Cursors.Default;
      if (this.game.Data.RegimeObj[this.game.Data.Turn].MessBackPic[this.PhaseData] > -1)
        this.NewBackGroundAndClearAll(1024, 768, this.game.Data.EventPicNr[this.game.Data.RegimeObj[this.game.Data.Turn].MessBackPic[this.PhaseData]]);
      else if ( this.game.Data.RuleVar[839] == 1.0)
        this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
      else
        this.NewBackGroundAndClearAll(1024, 768, DrawMod.TGame.BACKGROUND2MARC);
      if ( this.game.Data.RuleVar[839] > 0.0)
        this.DrawDateAndRegime();
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.game.Data.RegimeObj[this.game.Data.Turn].MessBackPic[this.PhaseData] != -2)
      {
        let mut index2: i32 =  -1;
        str1: String = this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.PhaseData];
        let mut num1: i32 =  0;
        if (Strings.Len(str1) > 0)
        {
          do
          {
            let mut Length: i32 =  Strings.InStr(str1, "#", CompareMethod.Text);
            if (Length > 0)
            {
              str2: String = Strings.Left(str1, Length);
              str3: String = Strings.Left(str2, Strings.Len(str2) - 1);
              index2 += 1;
              strArray[index2] = str3;
              if (Strings.Len(str1) > Length)
                str1 = Strings.Mid(str1, Length + 1);
              else
                num1 = 1;
            }
            else
            {
              if (Strings.Len(str1) > 0)
              {
                index2 += 1;
                strArray[index2] = str1;
              }
              num1 = 1;
            }
          }
          while (num1 == 0);
        }
        if (index2 > -1)
        {
          int[] textId = this.TextId;
          let mut tsubpart: SubPartClass =  TextPartClass::new(strArray[0], Font::new("Times New Roman", 25f, FontStyle.Bold, GraphicsUnit.Pixel), 800, 30, true);
          let mut num2: i32 =  this.AddSubPart(ref tsubpart, 100, 55, 800, 30, 0);
          textId[0] = num2;
        }
        let mut y: i32 =  95;
        SubPartClass tsubpart1;
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.PhaseData] > -1)
        {
          let mut num3: i32 =  this.game.Data.EventPicNr[this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.PhaseData]];
          if (BitmapStore.Getheight(num3) <= 300)
          {
            tsubpart1 =  ButtonPartClass::new(num3);
            this.Pic1Id = this.AddSubPart(ref tsubpart1,  Math.Round(512.0 -  BitmapStore.GetWidth(num3) / 2.0), y, BitmapStore.GetWidth(num3), BitmapStore.Getheight(num3), 0);
            y = y + BitmapStore.Getheight(num3) + 10;
          }
          else
          {
            let mut tsubpart2: SubPartClass =  ButtonPartClass::new(num3, tResizeX: BitmapStore.GetWidth(num3), tresizeY: 300);
            this.Pic1Id = this.AddSubPart(ref tsubpart2,  Math.Round(512.0 -  BitmapStore.GetWidth(num3) / 2.0), y, BitmapStore.GetWidth(num3), 300, 0);
            y = y + 300 + 10;
          }
        }
        let mut num4: i32 =  0;
        if (index2 > 0)
        {
          let mut num5: i32 =  index2;
          for (let mut index3: i32 =  1; index3 <= num5; index3 += 1)
          {
            let mut num6: i32 =  Strings.InStr(strArray[index3], "*", CompareMethod.Text);
            str4: String = Strings.Mid(strArray[index3], num6 + 1);
            let mut num7: i32 =  Strings.InStr(str4, "@", CompareMethod.Text);
            if (num6 > 0 & num7 > 0)
            {
              str5: String = Strings.Mid(str4, num7 + 1);
              txt: String = Strings.Left(str4, Strings.Len(str4) - (1 + Strings.Len(str5)));
              let mut num8: i32 =   Math.Round(Conversion.Val(str5));
              this.ButEvent[index3] = num8;
              let mut num9: i32 =  y + 15;
              int[] butId = this.ButId;
              let mut index4: i32 =  index3;
              tsubpart1 =  ButtonPartClass::new(this.game.OKBALL);
              let mut num10: i32 =  this.AddSubPart(ref tsubpart1, 500, num9 + (index3 - 1) * 25 + 5, 35, 35, 1);
              butId[index4] = num10;
              y = num9 + 40;
              num4 += 1;
              int[] textId = this.TextId;
              let mut index5: i32 =  index3;
              tsubpart1 =  TextPartClass::new(txt, Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), 800, 25, true);
              let mut num11: i32 =  this.AddSubPart(ref tsubpart1, 110, y + (index3 - 1) * 25, 800, 20, 0);
              textId[index5] = num11;
            }
            else
            {
              int[] textId = this.TextId;
              let mut index6: i32 =  index3;
              tsubpart1 =  TextPartClass::new(strArray[index3], Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), 800, 25, true);
              let mut num12: i32 =  this.AddSubPart(ref tsubpart1, 110, y + (index3 - 1) * 25, 800, 20, 0);
              textId[index6] = num12;
            }
          }
        }
        if (num4 == 0)
        {
          tsubpart1 =  new TextButtonPartClass("Acknowledge", 100, tBackbitmap: (ref this.OwnBitmap), bbx: 455, bby: 690);
          this.okid = this.AddSubPart(ref tsubpart1, 455, 690, 100, 35, 1);
        }
      }
      else
      {
        if (Strings.Len(this.game.Data.RegimeObj[this.game.Data.Turn].MessWav[this.PhaseData]) > 0)
        {
          SoundMod.STopEventWave();
          SoundMod.PlayEventWave(this.game.AppPath + "sound/" + this.game.Data.RegimeObj[this.game.Data.Turn].MessWav[this.PhaseData], ref this.game.EditObj);
        }
        if ( this.game.Data.RuleVar[839] == 0.0)
        {
          Color.FromArgb( byte.MaxValue, 180, 200, 240);
          Color.FromArgb( byte.MaxValue, 120, 120, 160);
          Color.FromArgb( byte.MaxValue, 200, 200, 200);
          Color.FromArgb( byte.MaxValue, 130, 130, 130);
          white: Color = Color.White;
          black: Color = Color.Black;
          vicFont3: Font = this.game.VicFont3;
          let mut num13: i32 =  0;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.PhaseData] > -1)
          {
            let mut index7: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.PhaseData];
            let mut nr: i32 =  index7 < 10000 ? this.game.Data.EventPicNr[index7] : this.game.Data.HistoricalUnitObj[index7 - 10000].CommanderSpriteID;
            if (nr > -1)
            {
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.PhaseData] >= 10000)
              {
                DrawMod.DrawOfficer(graphics, this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.PhaseData] - 10000,  Math.Round(512.0 -  BitmapStore.GetWidth(nr) / 2.0), 70, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr));
                num13 = BitmapStore.Getheight(nr) + 20;
              }
              else if (BitmapStore.Getheight(nr) <= 200)
              {
                ref Graphics local1 = ref graphics;
                bitmap: Bitmap = BitmapStore.GetBitmap(nr);
                ref local2: Bitmap = ref bitmap;
                let mut x: i32 =   Math.Round(512.0 -  BitmapStore.GetWidth(nr) / 2.0);
                let mut width: i32 =  BitmapStore.GetWidth(nr);
                let mut h: i32 =  BitmapStore.Getheight(nr);
                DrawMod.DrawScaled(ref local1, ref local2, x, 70, width, h);
                num13 = BitmapStore.Getheight(nr) + 20;
                DrawMod.DrawRectangle(ref graphics,  Math.Round(512.0 -  BitmapStore.GetWidth(nr) / 2.0), 70, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr),  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
              }
              else
              {
                ref Graphics local3 = ref graphics;
                bitmap: Bitmap = BitmapStore.GetBitmap(nr);
                ref local4: Bitmap = ref bitmap;
                let mut x: i32 =   Math.Round(512.0 -  BitmapStore.GetWidth(nr) * (200.0 /  BitmapStore.Getheight(nr)) / 2.0);
                let mut w: i32 =   Math.Round( BitmapStore.GetWidth(nr) * (200.0 /  BitmapStore.Getheight(nr)));
                DrawMod.DrawScaled(ref local3, ref local4, x, 70, w, 200);
                num13 = 220;
                DrawMod.DrawRectangle(ref graphics,  Math.Round(510.0 -  BitmapStore.GetWidth(nr) * (200.0 /  BitmapStore.Getheight(nr)) / 2.0), 68,  Math.Round( BitmapStore.GetWidth(nr) * (200.0 /  BitmapStore.Getheight(nr)) + 4.0), 204,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
              }
            }
          }
          let mut trows: i32 =   Math.Round(30.0 -  num13 / 16.0);
          if (7 > trows)
            trows = 7;
          let mut num14: i32 =  150;
          let mut num15: i32 =  110 + num13;
          let mut num16: i32 =  760;
          let mut num17: i32 =  (trows + 1) * 16;
          DrawMod.DrawPaperSheet(ref graphics, num14 - 20, num15 - 10, num16 + 40, num17 + 20);
          let mut tsubpart3: SubPartClass =  new PaperAreaClass(this.game, num16, trows,  null, "Description", false, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.PhaseData], this.game.VicColor8, tbackbitmap: (ref this.OwnBitmap), bbx: num14, bby: num15);
          this.TAid = this.AddSubPart(ref tsubpart3, num14, num15, num16, 16 * (trows + 1), 0);
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Acknowledge", 100, tBackbitmap: (ref this.OwnBitmap), bbx: 455, bby: 690);
          this.okid = this.AddSubPart(ref tsubpart4, 455, 690, 100, 35, 1);
        }
        else
        {
          let mut phaseData: i32 =  this.PhaseData;
          Rectangle rectangle = Rectangle::new();
          num18: i32;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[phaseData] > -1)
          {
            let mut index8: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[phaseData];
            let mut nr: i32 =  index8 < 10000 ? this.game.Data.EventPicNr[index8] : this.game.Data.HistoricalUnitObj[index8 - 10000].CommanderSpriteID;
            if (nr > -1)
            {
              let mut width: i32 =  BitmapStore.GetWidth(nr);
              let mut height: i32 =  BitmapStore.Getheight(nr);
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MesStyle[phaseData] == 2)
              {
                if (width > 194)
                {
                  height =  Math.Round( height * (194.0 /  width));
                  width =  Math.Round( width * (194.0 /  width));
                }
                if (height > 200)
                {
                  width =  Math.Round( width * (200.0 /  height));
                  height =  Math.Round( height * (200.0 /  height));
                }
                rectangle = Rectangle::new( Math.Round(500.0 -  width / 2.0), 20, width, height);
                num18 = height + 10;
              }
              else
              {
                if (width > 515)
                {
                  height =  Math.Round( height * (515.0 /  width));
                  width =  Math.Round( width * (515.0 /  width));
                }
                if (height > 250)
                {
                  width =  Math.Round( width * (250.0 /  height));
                  height =  Math.Round( height * (250.0 /  height));
                }
                rectangle = Rectangle::new( Math.Round(512.0 -  width / 2.0), 80, width, height);
                num18 = height + 10;
              }
            }
          }
          else
            num18 = 0;
          let mut tsubpart: SubPartClass =  new TextAreaClass2(this.game, 500, 15, this.game.MarcFont3, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[phaseData], 24, ref this.BackBitmap, 175, 100, true, true, tcenterit: true);
          this.TAid = this.AddSubPart(ref tsubpart, 260, 100, 500, 384, 0);
          let mut num19: i32 =  this.SubPartList[this.SubpartNr(this.TAid)].HeightUsed();
          let mut num20: i32 =   Math.Round( (this.OwnBitmap.Height - 100 - (num18 + num19)) / 2.0);
          this.RemoveSubPart(this.TAid);
          tsubpart =  new TextAreaClass2(this.game, 500,  Math.Round(Conversion.Int( num19 / 24.0)), this.game.MarcFont3, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[phaseData], 24, ref this.BackBitmap, 260, num20 + num18, true, true, tcenterit: true);
          this.TAid = this.AddSubPart(ref tsubpart, 260, num20 + num18, 500, 16 * ( Math.Round( num19 / 36.0) + 1), 0);
          tsubpart =  new TextButtonPartClass("PRESS ANY KEY", 160, "Click button to continue", ref this.OwnBitmap, 420, 710, theight: 20, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          this.okid = this.AddSubPart(ref tsubpart, 420, 710, 160, 20, 1);
          rectangle.Y = num20;
          if (rectangle.Y < 80)
            rectangle.Y = 80;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[phaseData] > -1)
          {
            let mut index9: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[phaseData];
            let mut num21: i32 =  index9 < 10000 ? this.game.Data.EventPicNr[index9] : index9 - 10000;
            if (num21 > -1)
            {
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[phaseData] > 10000)
              {
                DrawMod.DrawOfficer(graphics, num21, rectangle.X, rectangle.Y, rectangle.Width, rectangle.Height);
                if (this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[phaseData].Length > 0)
                {
                  SizeF sizeF1 = SizeF::new();
                  SizeF sizeF2 = graphics.MeasureString(this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[phaseData], this.game.MarcFont7);
                  DrawMod.DrawTextColouredMarc(ref graphics, this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[phaseData], this.game.MarcFont7,  Math.Round( rectangle.X +  rectangle.Width / 2.0 -  sizeF2.Width / 2.0), rectangle.Y + rectangle.Height - 8, Color.White);
                }
              }
              else
              {
                ref Graphics local5 = ref graphics;
                bitmap: Bitmap = BitmapStore.GetBitmap(num21);
                ref local6: Bitmap = ref bitmap;
                let mut x: i32 =  rectangle.X;
                let mut y: i32 =  rectangle.Y;
                let mut width: i32 =  rectangle.Width;
                let mut height: i32 =  rectangle.Height;
                DrawMod.DrawScaled(ref local5, ref local6, x, y, width, height);
                DrawMod.DrawFrame(ref this.OwnBitmap, ref this.BackBitmap, ref graphics, rectangle.X, rectangle.Y, rectangle.Width, rectangle.Height, -1, -1);
              }
            }
          }
        }
      }
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
      graphics = (Graphics) null;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 32 | nr == 13)
        {
          if (this.LoginId > 0)
          {
            windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.LoginId)] + 1, this.SubPartY[this.SubpartNr(this.LoginId)] + 1, 1);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (this.EnterTurnId > 0)
          {
            windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.EnterTurnId)] + 1, this.SubPartY[this.SubpartNr(this.EnterTurnId)] + 1, 1);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (this.okid > 0)
          {
            windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.okid)] + 1, this.SubPartY[this.SubpartNr(this.okid)] + 1, 1);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.Listid1)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.TAid)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.listid2)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.QuitId)
            {
              if (Interaction.MsgBox( "Are you Sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                this.game.Data = DataClass::new();
                this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
                if (this.game.Data.UseAI == 1)
                  this.game.NewAIObj.LastRegime = -1;
                this.game.EditObj.ShowInitialMenu = true;
                windowReturnClass.AddCommand(3, 1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.LoginId)
              {
                if (this.game.Data.PasswordsOn)
                {
                  if (Strings.Len(this.game.Data.RegimeObj[this.game.Data.Turn].PassWord) == 0)
                    this.game.Data.RegimeObj[this.game.Data.Turn].PassWord = Interaction.InputBox("First turn for this regime. Please give a password. DON''T FORGET IT!!", "Shadow Empire : Planetary Conquest");
                  else if (Operators.CompareString(Strings.UCase(Interaction.InputBox("Please give your password...", "Shadow Empire : Planetary Conquest")), Strings.UCase(this.game.Data.RegimeObj[this.game.Data.Turn].PassWord), false) == 0)
                  {
                    this.loggedin = true;
                    this += 1.game.EditObj.TutStep;
                    if (this.game.EditObj.Screenshoton)
                    {
                      this.game.FormRef.Cursor = Cursors.WaitCursor;
                      this.game.HandyFunctionsObj.doscreenshot("a", 0);
                      this.game.FormRef.Cursor = Cursors.Default;
                    }
                  }
                  else
                  {
                    let mut num2: i32 =   Interaction.MsgBox( "Wrong password.", Title: ( "Shadow Empire : Planetary Conquest"));
                    this.loggedin = false;
                    this.EnterTurn();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else
                {
                  this.loggedin = true;
                  this += 1.game.EditObj.TutStep;
                  if (this.game.EditObj.Screenshoton)
                  {
                    this.game.FormRef.Cursor = Cursors.WaitCursor;
                    this.game.HandyFunctionsObj.doscreenshot("a", 0);
                    this.game.FormRef.Cursor = Cursors.Default;
                  }
                }
                if (this.game.Data.PBEM && this.game.Data.GameID > -1)
                {
                  MasterLogClass masterLogClass = MasterLogClass::new();
                  str: String = this.game.AppPath + "masterlog.mlc";
                  DirectoryInfo Expression;
                  if (Directory.Exists(this.game.AppPath + "masterlogdir"))
                  {
                    Directory.CreateDirectory(this.game.AppPath + "masterlogdir");
                    Expression = new DirectoryInfo(this.game.AppPath + "masterlogdir");
                  }
                  else
                    Expression = (DirectoryInfo) null;
                  if (File.Exists(str))
                  {
                    this.game.HandyFunctionsObj.Unzip(str);
                    masterLogClass = MasterLogClass.deserialize(str);
                    this.game.HandyFunctionsObj.ZipFile(str);
                  }
                  if (this.game.Data.RegimeObj[this.game.Data.Turn].FirstRound < 1)
                    this.game.Data.RegimeObj[this.game.Data.Turn].FirstRound = this.game.Data.Round;
                  if (this.game.Data.Round == 1 | this.game.Data.RegimeObj[this.game.Data.Turn].FirstRound == this.game.Data.Round)
                  {
                    if (Operators.ConditionalCompareObjectLess(masterLogClass.ReturnQty(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, -1),  1, false))
                    {
                      this.game.Data.RegimeObj[this.game.Data.Turn].RandomCode =  Math.Round( Conversion.Int(VBMath.Rnd() * 9999f));
                      masterLogClass.SetEntry(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].RandomCode);
                    }
                  }
                  else
                  {
                    integer1: i32;
                    if (Operators.ConditionalCompareObjectLess(masterLogClass.ReturnQty(this.game.Data.GameID, this.game.Data.Round - 1, this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].RandomCode),  1, false))
                    {
                      if (this.game.Data.TerrorMode)
                      {
                        this.loggedin = false;
                        let mut num3: i32 =   Interaction.MsgBox( "Wiped Logbook. Terror Mode aborts opening this turn.", Title: ( "Shadow Empire : Planetary Conquest"));
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                      for (let mut index2: i32 =  0; index2 <= regimeCounter; index2 += 1)
                      {
                        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut index3: i32 =  index2;
                        let mut index4: i32 =  index3;
                        regimeClassArray[index4].MessCounter = regimeObj[index3].MessCounter + 1;
                        let mut messCounter: i32 =  this.game.Data.RegimeObj[index2].MessCounter;
                        this.game.Data.RegimeObj[index2].MessString = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MessString, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MessBackPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MessBackPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MessFrontPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MessFrontPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesStyle = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesStyle, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesNote = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesNote, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesNote2 = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesNote2, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MessWav = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MessWav, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesGroup = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesGroup, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesName = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesName, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesChosen = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesChosen, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesHideFromStart = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesHideFromStart, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesHideFromTab = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index2].MesHideFromTab, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index2].MesStyle[messCounter] = 0;
                        this.game.Data.RegimeObj[index2].MessString[messCounter] = "Possible PBEM cheat\r\n\r\n" + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has wiped his pbem logbook.";
                        this.game.Data.RegimeObj[index2].MessBackPic[messCounter] = -2;
                        this.game.Data.RegimeObj[index2].MessFrontPic[messCounter] = -1;
                      }
                    }
                    else if (Operators.ConditionalCompareObjectGreater(masterLogClass.ReturnQty(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, -1),  0, false))
                    {
                      let mut integer2: i32 =  Conversions.ToInteger(Operators.AddObject(masterLogClass.ReturnQty(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, -1),  1));
                      if (this.game.Data.TerrorMode)
                      {
                        this.loggedin = false;
                        let mut num4: i32 =   Interaction.MsgBox( "Turn has already been opened before. Terror Mode aborts opening this turn.", Title: ( "Shadow Empire : Planetary Conquest"));
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                      for (let mut index5: i32 =  0; index5 <= regimeCounter; index5 += 1)
                      {
                        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut index6: i32 =  index5;
                        let mut index7: i32 =  index6;
                        regimeClassArray[index7].MessCounter = regimeObj[index6].MessCounter + 1;
                        let mut messCounter: i32 =  this.game.Data.RegimeObj[index5].MessCounter;
                        this.game.Data.RegimeObj[index5].MessString = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MessString, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MessBackPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MessBackPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MessFrontPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MessFrontPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesStyle = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesStyle, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesNote = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesNote, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesNote2 = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesNote2, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MessWav = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MessWav, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesGroup = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesGroup, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesName = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesName, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesChosen = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesChosen, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesHideFromStart = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesHideFromStart, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesHideFromTab = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index5].MesHideFromTab, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index5].MesStyle[messCounter] = 0;
                        this.game.Data.RegimeObj[index5].MessString[messCounter] = "Possible PBEM cheat\r\n\r\n" + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has opened round " + Conversion.Str( this.game.Data.Round) + " for the " + Conversion.Str( integer2) + "th time.";
                        this.game.Data.RegimeObj[index5].MessBackPic[messCounter] = -2;
                        this.game.Data.RegimeObj[index5].MessFrontPic[messCounter] = -1;
                      }
                    }
                    else if (Information.IsNothing( Expression))
                    {
                      integer1 = Conversions.ToInteger(Operators.AddObject(masterLogClass.ReturnQty(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, -1),  1));
                      if (this.game.Data.TerrorMode)
                      {
                        this.loggedin = false;
                        let mut num5: i32 =   Interaction.MsgBox( "Unknown operations done to logfiles by user. Terror Mode aborts opening this turn.", Title: ( "Shadow Empire : Planetary Conquest"));
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                      for (let mut index8: i32 =  0; index8 <= regimeCounter; index8 += 1)
                      {
                        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut index9: i32 =  index8;
                        let mut index10: i32 =  index9;
                        regimeClassArray[index10].MessCounter = regimeObj[index9].MessCounter + 1;
                        let mut messCounter: i32 =  this.game.Data.RegimeObj[index8].MessCounter;
                        this.game.Data.RegimeObj[index8].MessString = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MessString, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MessBackPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MessBackPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MessFrontPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MessFrontPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesStyle = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesStyle, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesNote = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesNote, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesNote2 = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesNote2, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MessWav = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MessWav, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesGroup = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesGroup, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesName = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesName, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesChosen = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesChosen, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesHideFromStart = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesHideFromStart, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesHideFromTab = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index8].MesHideFromTab, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index8].MesStyle[messCounter] = 0;
                        this.game.Data.RegimeObj[index8].MessString[messCounter] = "Possible PBEM cheat\r\n\r\n" + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has been making changes to log files. He has deleted a directory. ";
                        this.game.Data.RegimeObj[index8].MessBackPic[messCounter] = -2;
                        this.game.Data.RegimeObj[index8].MessFrontPic[messCounter] = -1;
                      }
                    }
                    else if (DateTime.Compare(masterLogClass.LastSave, Expression.CreationTime) != 0)
                    {
                      integer1 = Conversions.ToInteger(Operators.AddObject(masterLogClass.ReturnQty(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, -1),  1));
                      if (this.game.Data.TerrorMode)
                      {
                        this.loggedin = false;
                        let mut num6: i32 =   Interaction.MsgBox( "Unknown operations done to logfiles by user. Terror Mode aborts opening this turn.", Title: ( "Shadow Empire : Planetary Conquest"));
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                      for (let mut index11: i32 =  0; index11 <= regimeCounter; index11 += 1)
                      {
                        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut index12: i32 =  index11;
                        let mut index13: i32 =  index12;
                        regimeClassArray[index13].MessCounter = regimeObj[index12].MessCounter + 1;
                        let mut messCounter: i32 =  this.game.Data.RegimeObj[index11].MessCounter;
                        this.game.Data.RegimeObj[index11].MessString = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MessString, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MessBackPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MessBackPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MessFrontPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MessFrontPic, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesStyle = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesStyle, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesNote = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesNote, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesNote2 = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesNote2, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MessWav = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MessWav, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesGroup = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesGroup, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesName = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesName, (Array) new string[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesChosen = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesChosen, (Array) new int[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesHideFromStart = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesHideFromStart, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesHideFromTab = (bool[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index11].MesHideFromTab, (Array) new bool[messCounter + 1]);
                        this.game.Data.RegimeObj[index11].MesStyle[messCounter] = 0;
                        this.game.Data.RegimeObj[index11].MessString[messCounter] = "Possible PBEM cheat\r\n\r\n" + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has been making changes to log files. He has been attempting to copy logs. ";
                        this.game.Data.RegimeObj[index11].MessBackPic[messCounter] = -2;
                        this.game.Data.RegimeObj[index11].MessFrontPic[messCounter] = -1;
                      }
                    }
                    this.game.Data.RegimeObj[this.game.Data.Turn].RandomCode =  Math.Round( Conversion.Int(VBMath.Rnd() * 9999f));
                    masterLogClass.SetEntry(this.game.Data.GameID, this.game.Data.Round, this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].RandomCode);
                  }
                  DirectoryInfo directoryInfo1 = (DirectoryInfo) null;
                  if (Directory.Exists(this.game.AppPath + "masterlogdir"))
                    Directory.Delete(this.game.AppPath + "masterlogdir");
                  Directory.CreateDirectory(this.game.AppPath + "masterlogdir");
                  DirectoryInfo directoryInfo2 = new DirectoryInfo(this.game.AppPath + "masterlogdir");
                  masterLogClass.LastSave = directoryInfo2.CreationTime;
                  directoryInfo1 = (DirectoryInfo) null;
                  if (File.Exists(str))
                    File.Delete(str);
                  masterLogClass.serialize(str);
                  this.game.HandyFunctionsObj.ZipFile(str);
                }
                this.game.EventRelatedObj.DoCheckEvents(4);
                this.EnterTurn();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.EnterTurnId)
              {
                if (this.loggedin & !this.game.Data.PBEM & !this.game.Data.PasswordsOn && this.game.EditObj.Screenshoton)
                {
                  this.game.FormRef.Cursor = Cursors.WaitCursor;
                  this.game.HandyFunctionsObj.doscreenshot("a", 0);
                  this.game.FormRef.Cursor = Cursors.Default;
                }
                if (this.loggedin)
                {
                  this.Phase = 2;
                  this += 1.game.EditObj.TutStep;
                  this.game.Data.RegimeObj[this.game.Data.Turn].Version = 110;
                  this.game.Data.RegimeObj[this.game.Data.Turn].subVersion = ".04b";
                  if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > -1)
                  {
                    this.ViewMessage();
                    windowReturnClass.SetFlag(true);
                  }
                  else
                  {
                    this.game.Data.InTurn = true;
                    if (Strings.Len(this.game.Data.LoadGame) > 0)
                    {
                      this.game.FormRef.Cursor = Cursors.WaitCursor;
                      this.game.HandyFunctionsObj.LoadGameNow();
                      this.game.FormRef.Cursor = Cursors.Default;
                      windowReturnClass.AddCommand(3, 4);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (this.game.EditObj.StartWithHistory)
                    {
                      this.game.HandyFunctionsObj.SetInitialXY(this.game.Data.Turn);
                      this.game.EditObj.OrderType = 26;
                      windowReturnClass.AddCommand(3, 6);
                    }
                    else
                    {
                      this.game.ProcessingObj.LocationProductionPrognosis();
                      this.game.HandyFunctionsObj.SetInitialXY(this.game.Data.Turn);
                      if ( this.game.Data.RuleVar[839] == 0.0)
                        windowReturnClass.AddCommand(3, 3);
                      else
                        windowReturnClass.AddCommand(3, 11);
                    }
                    windowReturnClass.SetFlag(true);
                  }
                }
                return windowReturnClass;
              }
              if (num1 == this.okid)
              {
                if (this.PhaseData >= this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter)
                {
                  this.game.Data.InTurn = true;
                  if (Strings.Len(this.game.Data.LoadGame) > 0)
                  {
                    this.game.FormRef.Cursor = Cursors.WaitCursor;
                    this.game.HandyFunctionsObj.LoadGameNow();
                    this.game.FormRef.Cursor = Cursors.Default;
                    windowReturnClass.AddCommand(3, 4);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (this.game.EditObj.StartWithHistory)
                  {
                    this.game.HandyFunctionsObj.SetInitialXY(this.game.Data.Turn);
                    this.game.EditObj.OrderType = 26;
                    windowReturnClass.AddCommand(3, 6);
                  }
                  else
                  {
                    this.game.HandyFunctionsObj.SetInitialXY(this.game.Data.Turn);
                    this.game.ProcessingObj.LocationProductionPrognosis();
                    if ( this.game.Data.RuleVar[839] == 0.0)
                      windowReturnClass.AddCommand(3, 3);
                    else
                      windowReturnClass.AddCommand(3, 11);
                  }
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  this.ViewMessage();
                  windowReturnClass.SetFlag(true);
                }
                return windowReturnClass;
              }
              if (num1 == this.SaveId)
              {
                str: String = this.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", this.game.AppPath_SAVEGAMES, false);
                if (Strings.Len(str) < 2)
                {
                  let mut num7: i32 =   Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  this.game.Data.serialize(str);
                  this.game.HandyFunctionsObj.ZipFile(str);
                  this.saved = true;
                  this.game.Data = DataClass::new();
                  this.game.EditObj.ShowInitialMenu = true;
                  windowReturnClass.AddCommand(3, 1);
                  windowReturnClass.SetFlag(true);
                }
                return windowReturnClass;
              }
            }
            let mut index14: i32 =  0;
            while (this.SubPartID[index1] != this.ButId[index14])
            {
              index14 += 1;
              if (index14 > 20)
                return windowReturnClass;
            }
            this.game.EventRelatedObj.DoCheckSpecificEvent(this.ButEvent[index14]);
            if (this.PhaseData >= this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter)
            {
              this.game.Data.InTurn = true;
              if (this.game.EditObj.StartWithHistory)
              {
                this.game.EditObj.OrderType = 26;
                windowReturnClass.AddCommand(3, 6);
              }
              else
              {
                this.game.ProcessingObj.LocationProductionPrognosis();
                if ( this.game.Data.RuleVar[839] == 0.0)
                  windowReturnClass.AddCommand(3, 3);
                else
                  windowReturnClass.AddCommand(3, 11);
              }
              windowReturnClass.SetFlag(true);
            }
            else
            {
              this.ViewMessage();
              windowReturnClass.SetFlag(true);
            }
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn DoPresentStats()
    {
      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
      for (let mut index1: i32 =  0; index1 <= regimeCounter; index1 += 1)
      {
        this.game.Data.RegimeObj[index1].SPresent = (int[,]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index1].SPresent, (Array) new int[this.game.Data.SFTypeCounter + 1, this.game.Data.Round + 1 + 1]);
        let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
        for (let mut index2: i32 =  0; index2 <= sfTypeCounter; index2 += 1)
        {
          this.game.Data.RegimeObj[index1].SPresent[index2, 0] = 0;
          this.game.Data.RegimeObj[index1].SPresent[index2, this.game.Data.Round] = 0;
        }
      }
      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
      for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
      {
        if (this.game.Data.UnitObj[index3].PreDef == -1)
        {
          let mut regime: i32 =  this.game.Data.UnitObj[index3].Regime;
          let mut sfCount: i32 =  this.game.Data.UnitObj[index3].SFCount;
          for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[index3].SFList[index4];
            spresent: Vec<i32> = this.game.Data.RegimeObj[regime].SPresent;
            numArray: Vec<i32> = spresent;
            let mut type: i32 =  this.game.Data.SFObj[sf].Type;
            let mut index5: i32 =  type;
            let mut round: i32 =  this.game.Data.Round;
            let mut index6: i32 =  round;
            let mut num: i32 =  spresent[type, round] + this.game.Data.SFObj[sf].Qty;
            numArray[index5, index6] = num;
          }
        }
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index] && Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "";
            this.game.EditObj.TipText = this.SubPartList[index].Descript;
            return;
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }
  }
}
