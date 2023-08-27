// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PbemInitializeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;

namespace WindowsApplication1
{
  pub class PbemInitializeWindowClass : WindowClass
  {
    pub Started: bool;
    pub object SteamQuestionPosed;
    pub object SteamQuestionAnswered;

    pub PbemInitializeWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND1MARC)
    {
      this.Started = false;
      this.SteamQuestionPosed =  false;
      this.SteamQuestionAnswered =  false;
    }

    pub fn PopUpRefresh()
    {
      if (!Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(this.SteamQuestionAnswered,  false, false), Operators.CompareObjectEqual(this.SteamQuestionPosed,  true, false))))
        return;
      if (this.game.EditObj.AnswerChosen == 1)
        this.game.EditObj.PbemAlreadyAccount = false;
      else
        this.game.EditObj.PbemAlreadyAccount = true;
      this.SteamQuestionAnswered =  true;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      this.game.EditObj.PbemSteam = true;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(this.SteamQuestionPosed,  false, false),  (Operators.CompareString(this.game.EditObj.PbemSerial, "0000-0000-0000-0000", false) == 0))))
      {
        this.game.EditObj.PbemAlreadyAccount = false;
        this.game.EditObj.QuestionText = "You do not have an account associated yet with this game.\r\nDo you want to use an existing PBEM++ account or create a new one?";
        this.game.EditObj.AnswerCount = 2;
        this.game.EditObj.AnswerText[1] = "Create new";
        this.game.EditObj.AnswerText[2] = "Use existing";
        this.game.EditObj.AnswerTextMouseOver[1] = "Will create a new PBEM++ user account for you.";
        this.game.EditObj.AnswerTextMouseOver[2] = "Will allow you to input an existing PBEM++ username and password.";
        this.SteamQuestionPosed =  true;
        this.game.EditObj.PopupValue = 10;
        windowReturnClass.AddCommand(5, 14);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (Operators.ConditionalCompareObjectEqual(this.SteamQuestionPosed,  false, false))
        this.game.EditObj.PbemAlreadyAccount = false;
      if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(this.SteamQuestionPosed,  false, false), Operators.CompareObjectEqual(this.SteamQuestionAnswered,  true, false))))
      {
        if (!this.Started)
        {
          this.Started = true;
          this.game.EditObj.ServerCommand = ServerCommandType.Initialize;
          this.game.EditObj.PopupValue = 15;
          this.game.EditObj.ServerStatusInitializeTried = false;
          windowReturnClass.AddCommand(5, 14);
          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (this.game.EditObj.ServerStatusInitializeTried)
        {
          windowReturnClass.AddCommand(1, 55);
          windowReturnClass.AddCommand(2, 91);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      return windowReturnClass;
    }
  }
}
