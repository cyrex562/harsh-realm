// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CorePbemWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class CorePbemWindowClass : WindowClass
  {
     BackId: i32;
     PassId: i32;
     PassTextId: i32;
     selectedid: i32;
     RegisterId: i32;
     RegisterId2: i32;
     CancelId: i32;
     Cancel2Id: i32;
     ClaimId: i32;
     Claim2Id: i32;
     LoginId: i32;
     LoginId2: i32;
     Refresh2id: i32;
     RefreshId: i32;
     LogOutId: i32;
     LogOut2Id: i32;
     challengeId: i32;
     challenge2Id: i32;
     AcceptChallengeId: i32;
     AcceptChallengeId2: i32;
     PreviewId: i32;
     Preview2Id: i32;
     CheckPlayerId: i32;
     CheckPlayerId2: i32;
     PlayTurnId: i32;
     textid: i32;
     PlayTurn2id: i32;
     ch1: i32;
     ch1b: i32;
     ch2b: i32;
     ch3b: i32;
     ch4b: i32;
     ch5b: i32;
     ch2: i32;
     ch3: i32;
     ch4: i32;
     ch5: i32;
     ChallengeListId: i32;
     ListClass ChallengeListObj;
     gamenr: i32;
     bool AcceptUsePass;
     currentTab: i32;

    pub CorePbemWindowClass(ref tGame: GameClass)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND1MARC)
    {
      this.ChallengeListObj = ListClass::new();
      this.gamenr = -1;
      this.selectedid = -1;
      this.currentTab = 1;
      if (this.game.EditObj.PbemInspectReturnFromID > -1 & this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting)
      {
        this.gamenr = this.game.EditObj.PbemInspectReturnFromID;
        this.game.EditObj.PbemInspectReturnFromID = -1;
        this.currentTab = 3;
      }
      if (this.game.EditObj.PbemInspectReturnFromID > -1 & this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.CancelPlayTurn)
      {
        this.gamenr = this.game.EditObj.PbemInspectReturnFromID;
        this.game.EditObj.PbemInspectReturnFromID = -1;
        this.currentTab = 1;
      }
      if (this.game.EditObj.PbemGameSetup != PbemGameSetupPhase.Saved)
        ;
      if (this.game.EditObj.IntroSoundOn)
        SoundMod.PlayEventBackground(this.game.AppPath + "sound/" + this.game.ModOpeningSoundtrack, ref this.game.EditObj);
      this.DoStuff();
    }

    pub fn PopUpRefresh()
    {
      this.gamenr = -1;
      this.DoStuff();
    }

    pub fn DoStuff()
    {
      SizeF sizeF = SizeF::new();
      this.selectedid = -1;
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.PassId > 0)
      {
        this.RemoveSubPart(this.PassId);
        this.PassId = 0;
      }
      if (this.PassTextId > 0)
      {
        this.RemoveSubPart(this.PassTextId);
        this.PassTextId = 0;
      }
      if (this.CancelId > 0)
        this.RemoveSubPart(this.CancelId);
      if (this.ClaimId > 0)
        this.RemoveSubPart(this.ClaimId);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.Cancel2Id);
      if (this.Claim2Id > 0)
        this.RemoveSubPart(this.Claim2Id);
      if (this.BackId > 0)
        this.RemoveSubPart(this.BackId);
      if (this.RegisterId > 0)
        this.RemoveSubPart(this.RegisterId);
      if (this.LoginId > 0)
        this.RemoveSubPart(this.LoginId);
      if (this.RegisterId2 > 0)
        this.RemoveSubPart(this.RegisterId2);
      if (this.LoginId2 > 0)
        this.RemoveSubPart(this.LoginId2);
      if (this.RefreshId > 0)
        this.RemoveSubPart(this.RefreshId);
      if (this.Refresh2id > 0)
        this.RemoveSubPart(this.Refresh2id);
      if (this.LogOutId > 0)
        this.RemoveSubPart(this.LogOutId);
      if (this.LogOut2Id > 0)
        this.RemoveSubPart(this.LogOut2Id);
      if (this.challengeId > 0)
        this.RemoveSubPart(this.challengeId);
      if (this.ch1 > 0)
        this.RemoveSubPart(this.ch1);
      if (this.ch2 > 0)
        this.RemoveSubPart(this.ch2);
      if (this.ch3 > 0)
        this.RemoveSubPart(this.ch3);
      if (this.ch4 > 0)
        this.RemoveSubPart(this.ch4);
      if (this.ch5 > 0)
        this.RemoveSubPart(this.ch5);
      if (this.ch1b > 0)
        this.RemoveSubPart(this.ch1b);
      if (this.ch2b > 0)
        this.RemoveSubPart(this.ch2b);
      if (this.ch3b > 0)
        this.RemoveSubPart(this.ch3b);
      if (this.ch4b > 0)
        this.RemoveSubPart(this.ch4b);
      if (this.ch5b > 0)
        this.RemoveSubPart(this.ch5b);
      if (this.challenge2Id > 0)
        this.RemoveSubPart(this.challenge2Id);
      if (this.PlayTurnId > 0)
        this.RemoveSubPart(this.PlayTurnId);
      if (this.PlayTurn2id > 0)
        this.RemoveSubPart(this.PlayTurn2id);
      if (this.PreviewId > 0)
        this.RemoveSubPart(this.PreviewId);
      if (this.Preview2Id > 0)
        this.RemoveSubPart(this.Preview2Id);
      if (this.CheckPlayerId > 0)
        this.RemoveSubPart(this.CheckPlayerId);
      if (this.CheckPlayerId2 > 0)
        this.RemoveSubPart(this.CheckPlayerId2);
      if (this.textid > 0)
        this.RemoveSubPart(this.textid);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
      Graphics g = Graphics.FromImage((Image) this.OwnBitmap);
      g.SmoothingMode = SmoothingMode.AntiAlias;
      g.TextRenderingHint = TextRenderingHint.AntiAlias;
      g.TextContrast = 1;
      DrawMod.DrawTextColouredMarc(ref g, "PBEM++ MAIN MENU", this.game.MarcFont1, 55, 27, Color.White);
      DrawMod.DrawBlock(ref g, 40, 60, 925, 3, (int) this.game.MarcCol4.R, (int) this.game.MarcCol4.G, (int) this.game.MarcCol4.B, (int) this.game.MarcCol4.A);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("EXIT PBEM++", 160, "Go back to the main menu.", ref this.OwnBitmap, 800, 250, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
      this.BackId = this.AddSubPart(ref tsubpart1, 800, 250, 160, 50, 1);
      let mut num: i32 =  -1;
      if (this.game.EditObj.ServerLostConnect)
      {
        DrawMod.DrawTextColouredMarc(ref g, "Lost connection", this.game.MarcFont4, 800, 80, Color.White);
        DrawMod.DrawTextColouredMarc(ref g, "with server", this.game.MarcFont4, 800, 105, Color.White);
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("REGISTER", 160, "Click to register.", ref this.OwnBitmap, 800, 150, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.RegisterId = this.AddSubPart(ref tsubpart2, 800, 150, 160, 50, 1);
        tsubpart2 =  new TextButtonPartClass("LOGIN", 160, "Try to log-in with existing username/password.", ref this.OwnBitmap, 800, 200, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.LoginId = this.AddSubPart(ref tsubpart2, 800, 200, 160, 50, 1);
        tsubpart2 =  new TextButtonPartClass("REFRESH", 160, "cannot refresh because not registered yet.", ref this.OwnBitmap, 800, 300, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.Refresh2id = this.AddSubPart(ref tsubpart2, 800, 300, 160, 50, 1);
        tsubpart2 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 350, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "open file browser", tfont2: this.game.MarcFont4);
        this.challenge2Id = this.AddSubPart(ref tsubpart2, 800, 350, 160, 50, 1);
        switch (num)
        {
          case 3:
            tsubpart2 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 400, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Case Blue Short", tfont2: this.game.MarcFont4);
            this.ch1b = this.AddSubPart(ref tsubpart2, 800, 400, 160, 50, 1);
            tsubpart2 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 450, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Op. Uranus Short", tfont2: this.game.MarcFont4);
            this.ch2b = this.AddSubPart(ref tsubpart2, 800, 450, 160, 50, 1);
            tsubpart2 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 500, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "2nd Kharkov", tfont2: this.game.MarcFont4);
            this.ch3b = this.AddSubPart(ref tsubpart2, 800, 500, 160, 50, 1);
            tsubpart2 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 550, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Voronezh", tfont2: this.game.MarcFont4);
            this.ch4b = this.AddSubPart(ref tsubpart2, 800, 550, 160, 50, 1);
            break;
          case 4:
            tsubpart2 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 400, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Barbarossa", tfont2: this.game.MarcFont4);
            this.ch1b = this.AddSubPart(ref tsubpart2, 800, 400, 160, 50, 1);
            break;
        }
      }
      else if (this.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthFailure & this.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialNotRegistered)
      {
        DrawMod.DrawTextColouredMarc(ref g, "You still have to login", this.game.MarcFont4, 800, 80, Color.White);
        DrawMod.DrawTextColouredMarc(ref g, "or register a user/pass...", this.game.MarcFont4, 800, 105, Color.White);
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("REGISTER", 160, "Click to register.", ref this.OwnBitmap, 800, 150, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.RegisterId = this.AddSubPart(ref tsubpart3, 800, 150, 160, 50, 1);
        tsubpart3 =  new TextButtonPartClass("LOGIN", 160, "Try to log-in with existing username/password.", ref this.OwnBitmap, 800, 200, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.LoginId = this.AddSubPart(ref tsubpart3, 800, 200, 160, 50, 1);
        tsubpart3 =  new TextButtonPartClass("REFRESH", 160, "cannot refresh because not registered yet.", ref this.OwnBitmap, 800, 300, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.Refresh2id = this.AddSubPart(ref tsubpart3, 800, 300, 160, 50, 1);
        tsubpart3 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 350, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "open file browser", tfont2: this.game.MarcFont4);
        this.challenge2Id = this.AddSubPart(ref tsubpart3, 800, 350, 160, 50, 1);
        switch (num)
        {
          case 3:
            tsubpart3 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 400, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Case Blue Short", tfont2: this.game.MarcFont4);
            this.ch1b = this.AddSubPart(ref tsubpart3, 800, 400, 160, 50, 1);
            tsubpart3 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 450, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Op. Uranus Short", tfont2: this.game.MarcFont4);
            this.ch2b = this.AddSubPart(ref tsubpart3, 800, 450, 160, 50, 1);
            tsubpart3 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 500, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "2nd Kharkov", tfont2: this.game.MarcFont4);
            this.ch3b = this.AddSubPart(ref tsubpart3, 800, 500, 160, 50, 1);
            tsubpart3 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 550, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Voronezh", tfont2: this.game.MarcFont4);
            this.ch4b = this.AddSubPart(ref tsubpart3, 800, 550, 160, 50, 1);
            break;
          case 4:
            tsubpart3 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 400, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Barbarossa", tfont2: this.game.MarcFont4);
            this.ch1b = this.AddSubPart(ref tsubpart3, 800, 400, 160, 50, 1);
            break;
        }
      }
      else if (this.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthSucces)
      {
        DrawMod.DrawTextColouredMarc(ref g, "Welcome back,", this.game.MarcFont4, 800, 80, Color.White);
        DrawMod.DrawTextColouredMarc(ref g, this.game.EditObj.PbemUserName, this.game.MarcFont3, 800, 105, Color.White);
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("REGISTER", 160, "Cannot register because already logged-in.", ref this.OwnBitmap, 800, 150, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.RegisterId2 = this.AddSubPart(ref tsubpart4, 800, 150, 160, 50, 1);
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("LOGIN", 160, "Cannot login because you are already logged-in.", ref this.OwnBitmap, 800, 200, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.LoginId2 = this.AddSubPart(ref tsubpart5, 800, 200, 160, 50, 1);
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("REFRESH", 160, "Get new list of challenges and running games from server.", ref this.OwnBitmap, 800, 300, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.RefreshId = this.AddSubPart(ref tsubpart6, 800, 300, 160, 50, 1);
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("CHALLENGE", 160, "Allows you to choose a scenario from a file browser to issue a challenge for a PBEM++ game. Keep in mind if you select a custom or third party scenario that the other player needs to any possible graphics this scenarios uses as well.\r\nFurther keep in mind that the PBEM++ system only supports 2 player games.", ref this.OwnBitmap, 800, 350, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "open file browser", tfont2: this.game.MarcFont4);
        this.challengeId = this.AddSubPart(ref tsubpart7, 800, 350, 160, 50, 1);
        switch (num)
        {
          case 3:
            let mut tsubpart8: SubPartClass =  new TextButtonPartClass("CHALLENGE", 160, "Start a PBEM++ challenge to play the Case Blue Short campaign scenario.\r\nIt lasts about 30 rounds or less. ", ref this.OwnBitmap, 800, 400, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Case Blue Short", tfont2: this.game.MarcFont4);
            this.ch1 = this.AddSubPart(ref tsubpart8, 800, 400, 160, 50, 1);
            let mut tsubpart9: SubPartClass =  new TextButtonPartClass("CHALLENGE", 160, "Start a PBEM++ challenge to play the Uranus Short campaign scenario.\r\nIt lasts about 30 rounds or less. ", ref this.OwnBitmap, 800, 450, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Op. Uranus Short", tfont2: this.game.MarcFont4);
            this.ch2 = this.AddSubPart(ref tsubpart9, 800, 450, 160, 50, 1);
            let mut tsubpart10: SubPartClass =  new TextButtonPartClass("CHALLENGE", 160, "Start a PBEM++ challenge to play the 2nd Kharkov scenario.\r\nIt lasts about 8 rounds.", ref this.OwnBitmap, 800, 500, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "2nd Kharkov", tfont2: this.game.MarcFont4);
            this.ch3 = this.AddSubPart(ref tsubpart10, 800, 500, 160, 50, 1);
            tsubpart10 =  new TextButtonPartClass("CHALLENGE", 160, "Start a PBEM++ challenge to play the Drive on Voronezh scenario.\r\nIt lasts about 8 rounds.", ref this.OwnBitmap, 800, 550, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Voronezh", tfont2: this.game.MarcFont4);
            this.ch4 = this.AddSubPart(ref tsubpart10, 800, 550, 160, 50, 1);
            break;
          case 4:
            let mut tsubpart11: SubPartClass =  new TextButtonPartClass("CHALLENGE", 160, "Start a PBEM++ challenge to play the regular Barbarossa scenario.", ref this.OwnBitmap, 800, 400, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Barbarossa", tfont2: this.game.MarcFont4);
            this.ch1 = this.AddSubPart(ref tsubpart11, 800, 400, 160, 50, 1);
            break;
        }
      }
      else
      {
        DrawMod.DrawTextColouredMarc(ref g, "Your not logged in.", this.game.MarcFont4, 800, 80, Color.White);
        let mut tsubpart12: SubPartClass =  new TextButtonPartClass("REGISTER", 160, "Click to register.", ref this.OwnBitmap, 800, 150, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.RegisterId = this.AddSubPart(ref tsubpart12, 800, 150, 160, 50, 1);
        let mut tsubpart13: SubPartClass =  new TextButtonPartClass("LOGIN", 160, "Click to login with existing username/password.", ref this.OwnBitmap, 800, 200, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.LoginId = this.AddSubPart(ref tsubpart13, 800, 200, 160, 50, 1);
        let mut tsubpart14: SubPartClass =  new TextButtonPartClass("REFRESH", 160, "cannot refresh because not logged in yet.", ref this.OwnBitmap, 800, 300, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.Refresh2id = this.AddSubPart(ref tsubpart14, 800, 300, 160, 50, 1);
        tsubpart14 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 350, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "open file browser", tfont2: this.game.MarcFont4);
        this.challenge2Id = this.AddSubPart(ref tsubpart14, 800, 350, 160, 50, 1);
        switch (num)
        {
          case 3:
            tsubpart14 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 400, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Case Blue Short", tfont2: this.game.MarcFont4);
            this.ch1b = this.AddSubPart(ref tsubpart14, 800, 400, 160, 50, 1);
            tsubpart14 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 450, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Op. Uranus Short", tfont2: this.game.MarcFont4);
            this.ch2b = this.AddSubPart(ref tsubpart14, 800, 450, 160, 50, 1);
            tsubpart14 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 500, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "2nd Kharkov", tfont2: this.game.MarcFont4);
            this.ch3b = this.AddSubPart(ref tsubpart14, 800, 500, 160, 50, 1);
            tsubpart14 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 550, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Voronezh", tfont2: this.game.MarcFont4);
            this.ch4b = this.AddSubPart(ref tsubpart14, 800, 550, 160, 50, 1);
            break;
          case 4:
            tsubpart14 =  new TextButtonPartClass("CHALLENGE", 160, "Not logged in!", ref this.OwnBitmap, 800, 400, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true, textras: "Barbarossa", tfont2: this.game.MarcFont4);
            this.ch1b = this.AddSubPart(ref tsubpart14, 800, 400, 160, 50, 1);
            break;
        }
      }
      DrawMod.DrawFrame2(ref this.OwnBitmap, ref g, 40, 104, 720, 600);
      if (this.currentTab == 0)
      {
        this.DrawTab3(g);
        this.DrawTab2(g);
        this.DrawTab1(g);
        this.DrawTab0(g, true);
        this.DrawFinishedGamesTab(g);
      }
      else if (this.currentTab == 1)
      {
        this.DrawTab0(g);
        this.DrawTab3(g);
        this.DrawTab2(g);
        this.DrawTab1(g, true);
        this.DrawCurrentGamesTab(g);
      }
      else if (this.currentTab == 2)
      {
        this.DrawTab0(g);
        this.DrawTab1(g);
        this.DrawTab3(g);
        this.DrawTab2(g, true);
        this.DrawYourChallengesTab(g);
      }
      else
      {
        if (this.currentTab != 3)
          return;
        this.DrawTab0(g);
        this.DrawTab1(g);
        this.DrawTab2(g);
        this.DrawTab3(g, true);
        this.DrawOtherChallengesTab(g);
      }
    }

    pub fn DrawYourChallengesTab(Graphics g)
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.ChallengeListId > 0)
        this.RemoveSubPart(this.ChallengeListId);
      this.ChallengeListObj = ListClass::new();
      let mut num: i32 =  -1;
      if (Information.IsNothing( this.game.EditObj.ServerChallengeList))
        return;
      let mut upperBound: i32 =  this.game.EditObj.ServerChallengeList.GetUpperBound(0);
      for (let mut index: i32 =  0; index <= upperBound; index += 1)
      {
        if (Operators.CompareString(Strings.UCase(this.game.EditObj.ServerChallengeList[index].challengerUserName), Strings.UCase(this.game.EditObj.PbemUserName), false) == 0)
        {
          if (this.gamenr == this.game.EditObj.ServerChallengeList[index].challengeID)
          {
            this.game.EditObj.PbemChallengeID = this.game.EditObj.ServerChallengeList[index].challengeID;
            num = index;
          }
          tvalue3: String = " ";
          str1: String = "You".to_owned();
          tvalue: String = this.game.EditObj.ServerChallengeList[index].firstPlayerSide != 1 ? str1 + " (opponent starts)" : str1 + " (you start)";
          if (this.game.EditObj.ServerChallengeList[index].challengePrivate == 1)
            tvalue3 = "PASSWORD REQ";
          str2: String = this.game.EditObj.ServerChallengeList[index].gameName;
          if (str2.Length > 30)
            str2 = Strings.Left(str2, 30) + "...";
          this.ChallengeListObj.add(str2, this.game.EditObj.ServerChallengeList[index].challengeID, tvalue, Conversions.ToString(this.game.EditObj.ServerChallengeList[index].dateIssued), tvalue3);
        }
      }
      ListClass challengeListObj = this.ChallengeListObj;
      let mut tlistselect: i32 =  num;
      let mut game: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font: Font =  null;
      ref local2: Font = ref font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(challengeListObj, 18, 680, tlistselect, game, tHeaderCenter: false, tShowPair: true, tValueWidth: 490, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: 60, bby: 130, tMarcStyle: true, overruleFont: (ref local2));
      this.ChallengeListId = this.AddSubPart(ref tsubpart, 60, 130, 680, 304, 0);
      this.DrawYourChallengesTabRefresh();
    }

    pub fn DrawYourChallengesTabRefresh()
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.CancelId > 0)
        this.RemoveSubPart(this.CancelId);
      if (this.ClaimId > 0)
        this.RemoveSubPart(this.ClaimId);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.Cancel2Id);
      if (this.Claim2Id > 0)
        this.RemoveSubPart(this.Claim2Id);
      if (this.AcceptChallengeId > 0)
        this.RemoveSubPart(this.AcceptChallengeId);
      if (this.AcceptChallengeId2 > 0)
        this.RemoveSubPart(this.AcceptChallengeId2);
      if (this.PlayTurnId > 0)
        this.RemoveSubPart(this.PlayTurnId);
      if (this.PlayTurn2id > 0)
        this.RemoveSubPart(this.PlayTurn2id);
      if (this.PreviewId > 0)
        this.RemoveSubPart(this.PreviewId);
      if (this.Preview2Id > 0)
        this.RemoveSubPart(this.Preview2Id);
      if (this.textid > 0)
        this.RemoveSubPart(this.textid);
      if (this.PassId > 0)
      {
        this.RemoveSubPart(this.PassId);
        this.PassId = 0;
      }
      if (this.PassTextId > 0)
      {
        this.RemoveSubPart(this.PassTextId);
        this.PassTextId = 0;
      }
      if (this.CheckPlayerId > 0)
        this.RemoveSubPart(this.CheckPlayerId);
      if (this.CheckPlayerId2 > 0)
        this.RemoveSubPart(this.CheckPlayerId2);
      let mut index1: i32 =  -1;
      tText: String;
      if (this.gamenr > -1)
      {
        let mut upperBound: i32 =  this.game.EditObj.ServerChallengeList.GetUpperBound(0);
        for (let mut index2: i32 =  0; index2 <= upperBound; index2 += 1)
        {
          if (this.gamenr == this.game.EditObj.ServerChallengeList[index2].challengeID)
          {
            this.game.EditObj.PbemChallengeID = this.game.EditObj.ServerChallengeList[index2].challengeID;
            index1 = index2;
          }
        }
        tText = "-error with this challenge-";
        if (index1 > -1)
        {
          InputStr: String = Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, 33, 8);
          num: i32;
          try
          {
            num = (int) Math.Round(Conversion.Val(InputStr));
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            num = 0;
            ProjectData.ClearProjectError();
          }
          if (num > 999)
            num = 0;
          tText = "GAME NAME: " + this.game.EditObj.ServerChallengeList[index1].gameName + "\r\n" + "CHALLENGER: " + this.game.EditObj.ServerChallengeList[index1].challengerUserName + "\r\n" + "FILE NAME: " + Strings.Left(this.game.EditObj.ServerChallengeList[index1].miscData, 32) + "\r\n" + "VERSION: " + num.ToString() + "\r\n" + "SETTINGS: " + Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, 41, 4) + "\r\n" + "VARIANTS: " + Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, 45, 12) + "\r\n";
        }
      }
      else
        tText = "-no scenario selected-";
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(this.game, 680, 7, this.game.MarcFont4, tText, tbackbitmap: (ref this.OwnBitmap), bbx: 60, bby: 450);
      this.textid = this.AddSubPart(ref tsubpart1, 60, 450, 680, 160, 0);
      if (this.gamenr > -1)
      {
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("CANCEL CHALLENGE", 240, "Click to cancel this challenge", ref this.OwnBitmap, 60, 620, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.CancelId = this.AddSubPart(ref tsubpart2, 60, 620, 240, 50, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("CANCEL CHALLENGE", 240, "Cannot cancel any challenge because no challenge from the list above has been selected.", ref this.OwnBitmap, 60, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.CancelId = this.AddSubPart(ref tsubpart3, 60, 620, 240, 50, 1);
      }
    }

    pub fn DrawOtherChallengesTab(Graphics g)
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.ChallengeListId > 0)
        this.RemoveSubPart(this.ChallengeListId);
      if (Information.IsNothing( this.game.EditObj.ServerChallengeList))
        return;
      this.ChallengeListObj = ListClass::new();
      let mut num: i32 =  -1;
      let mut upperBound: i32 =  this.game.EditObj.ServerChallengeList.GetUpperBound(0);
      for (let mut index: i32 =  0; index <= upperBound; index += 1)
      {
        if (Operators.CompareString(Strings.UCase(this.game.EditObj.ServerChallengeList[index].challengerUserName), Strings.UCase(this.game.EditObj.PbemUserName), false) != 0)
        {
          if (this.gamenr == this.game.EditObj.ServerChallengeList[index].challengeID)
            num = index;
          tvalue3: String = " ";
          if (this.game.EditObj.ServerChallengeList[index].challengePrivate == 1)
            tvalue3 = "PASSWORD REQ";
          challengerUserName: String = this.game.EditObj.ServerChallengeList[index].challengerUserName;
          tvalue: String = this.game.EditObj.ServerChallengeList[index].firstPlayerSide != 1 ? challengerUserName + " (you start)" : challengerUserName + " (starts)";
          str: String = this.game.EditObj.ServerChallengeList[index].gameName;
          if (str.Length > 30)
            str = Strings.Left(str, 30) + "...";
          this.ChallengeListObj.add(str, this.game.EditObj.ServerChallengeList[index].challengeID, tvalue, Conversions.ToString(this.game.EditObj.ServerChallengeList[index].dateIssued), tvalue3);
        }
      }
      ListClass challengeListObj = this.ChallengeListObj;
      let mut tlistselect: i32 =  num;
      let mut game: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font: Font =  null;
      ref local2: Font = ref font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(challengeListObj, 18, 680, tlistselect, game, tHeaderCenter: false, tShowPair: true, tValueWidth: 490, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: 60, bby: 130, tMarcStyle: true, overruleFont: (ref local2));
      this.ChallengeListId = this.AddSubPart(ref tsubpart, 60, 130, 680, 304, 0);
      this.DrawOtherChallengesTabRefresh();
    }

    pub fn DrawOtherChallengesTabRefresh()
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.AcceptChallengeId > 0)
        this.RemoveSubPart(this.AcceptChallengeId);
      if (this.AcceptChallengeId2 > 0)
        this.RemoveSubPart(this.AcceptChallengeId2);
      if (this.PlayTurnId > 0)
        this.RemoveSubPart(this.PlayTurnId);
      if (this.PlayTurn2id > 0)
        this.RemoveSubPart(this.PlayTurn2id);
      if (this.PreviewId > 0)
        this.RemoveSubPart(this.PreviewId);
      if (this.Preview2Id > 0)
        this.RemoveSubPart(this.Preview2Id);
      if (this.textid > 0)
        this.RemoveSubPart(this.textid);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.CancelId);
      if (this.ClaimId > 0)
        this.RemoveSubPart(this.ClaimId);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.Cancel2Id);
      if (this.Claim2Id > 0)
        this.RemoveSubPart(this.Claim2Id);
      if (this.PassId > 0)
      {
        this.RemoveSubPart(this.PassId);
        this.PassId = 0;
      }
      if (this.PassTextId > 0)
      {
        this.RemoveSubPart(this.PassTextId);
        this.PassTextId = 0;
      }
      if (this.CheckPlayerId > 0)
        this.RemoveSubPart(this.CheckPlayerId);
      if (this.CheckPlayerId2 > 0)
        this.RemoveSubPart(this.CheckPlayerId2);
      let mut index1: i32 =  -1;
      this.selectedid = -1;
      tText: String;
      SubPartClass tsubpart1;
      if (this.gamenr > -1)
      {
        let mut upperBound: i32 =  this.game.EditObj.ServerChallengeList.GetUpperBound(0);
        for (let mut index2: i32 =  0; index2 <= upperBound; index2 += 1)
        {
          if (this.gamenr == this.game.EditObj.ServerChallengeList[index2].challengeID)
            index1 = index2;
        }
        tText = "-error with this challenge-";
        this.game.EditObj.PbemChallengeMiscData = this.game.EditObj.ServerChallengeList[index1].miscData;
        InputStr: String = Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 33, 8);
        num: i32;
        try
        {
          num = (int) Math.Round(Conversion.Val(InputStr));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          num = 0;
          ProjectData.ClearProjectError();
        }
        if (num > 999)
          num = 0;
        str1: String;
        if (index1 > -1)
        {
          str1 = "";
          let mut Start: i32 =  32;
          do
          {
            if (Operators.CompareString(Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, Start, 1), " ", false) != 0)
              str1 = Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, Start, 1) + str1;
            Start += -1;
          }
          while (Start >= 1);
          str2: String = "GAME NAME: " + this.game.EditObj.ServerChallengeList[index1].gameName + "\r\n" + "CHALLENGER: " + this.game.EditObj.ServerChallengeList[index1].challengerUserName + "\r\n";
          this.game.EditObj.PbemCheckPlayer = this.game.EditObj.ServerChallengeList[index1].challengerUserName;
          str3: String;
          if (File.Exists(this.game.AppPath + this.game.ModScenarioDir + "\\" + str1))
          {
            str3 = str2 + "FILE NAME: " + str1 + " : YOU HAVE THIS FILE AS WELL\r\n";
            if (Operators.CompareString(this.game.HandyFunctionsObj.GetFileCRC(this.game.ModScenarioDir + "\\" + str1), Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, 33, 8), false) != 0)
              ;
          }
          else
            str3 = str2 + "FILE NAME: " + str1 + " : YOU DO NOT HAVE THIS FILE!\r\n";
          tText = (num <= 0 ? str3 + "CHALLENGER GAME VERSION: older\r\n" : str3 + "CHALLENGER GAME VERSION: " + num.ToString() + "\r\n") + "SETTINGS: " + Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, 41, 4) + "\r\n" + "VARIANTS: " + Strings.Mid(this.game.EditObj.ServerChallengeList[index1].miscData, 45, 12) + "\r\n";
        }
        if (num > 110)
        {
          let mut tsubpart2: SubPartClass =  new TextButtonPartClass("ACCEPT CHALLENGE", 240, "Your game version is to low to play with this oppponent. Please upgrade your game to the latest version. ", ref this.OwnBitmap, 60, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.AcceptChallengeId2 = this.AddSubPart(ref tsubpart2, 60, 620, 240, 50, 1);
        }
        else
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("ACCEPT CHALLENGE", 240, "Accept the selected challenge and start up a PBEM++ game with the challenger.", ref this.OwnBitmap, 60, 620, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.AcceptChallengeId = this.AddSubPart(ref tsubpart3, 60, 620, 240, 50, 1);
        }
        this.AcceptUsePass = false;
        if (this.game.EditObj.ServerChallengeList[index1].challengePrivate == 1)
        {
          this.AcceptUsePass = true;
          Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
          if (Information.IsNothing( this.game.EditObj.PbemPrivatePassword))
            this.game.EditObj.PbemPrivatePassword = "";
          let mut tsubpart4: SubPartClass =  TextPartClass::new("Private Password:", this.game.MarcFont4, 160, 20, false, tMarc: true);
          this.PassTextId = this.AddSubPart(ref tsubpart4, 580, 620, 160, 20, 0);
          let mut tsubpart5: SubPartClass =  new InputTextClass(this.game.EditObj.PbemPrivatePassword, this.game.MarcFont4, 160, 36, false, 20, true);
          this.PassId = this.AddSubPart(ref tsubpart5, 580, 643, 160, 36, 0);
          graphics.Dispose();
        }
        if (File.Exists(this.game.AppPath + this.game.ModScenarioDir + "\\" + str1))
        {
          if (num <= 110)
          {
            let mut tsubpart6: SubPartClass =  new TextButtonPartClass("PREVIEW SCENARIO", 240, "Loads a local copy the challenge with the same filename\r\nand will show it with the challengers configuration.", ref this.OwnBitmap, 320, 620, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
            this.PreviewId = this.AddSubPart(ref tsubpart6, 320, 620, 240, 50, 1);
          }
          else
          {
            let mut tsubpart7: SubPartClass =  new TextButtonPartClass("PREVIEW SCENARIO", 240, "Your game version is to low to play with this opponent. Please upgrade your game to the latest version.", ref this.OwnBitmap, 320, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
            this.Preview2Id = this.AddSubPart(ref tsubpart7, 320, 620, 240, 50, 1);
          }
        }
        else
        {
          let mut tsubpart8: SubPartClass =  new TextButtonPartClass("PREVIEW SCENARIO", 240, "You dont have the same file as the challenger used.\r\nMaybe check the forums to find out where to get it.", ref this.OwnBitmap, 320, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.Preview2Id = this.AddSubPart(ref tsubpart8, 320, 620, 240, 50, 1);
        }
      }
      else
      {
        tText = "-no scenario selected-";
        let mut tsubpart9: SubPartClass =  new TextButtonPartClass("ACCEPT CHALLENGE", 240, "You have to select a challenge in order for you to be able to use this button.", ref this.OwnBitmap, 60, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.AcceptChallengeId2 = this.AddSubPart(ref tsubpart9, 60, 620, 240, 50, 1);
        tsubpart1 =  new TextButtonPartClass("PREVIEW SCENARIO", 240, "You have to select a challenge in order for you to be able to use this button.", ref this.OwnBitmap, 320, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.Preview2Id = this.AddSubPart(ref tsubpart1, 320, 620, 240, 50, 1);
      }
      tsubpart1 =  new TextAreaClass2(this.game, 680, 7, this.game.MarcFont4, tText, tbackbitmap: (ref this.OwnBitmap), bbx: 60, bby: 450);
      this.textid = this.AddSubPart(ref tsubpart1, 60, 450, 680, 160, 0);
    }

    pub fn DrawCurrentGamesTab(Graphics g)
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.ChallengeListId > 0)
        this.RemoveSubPart(this.ChallengeListId);
      if (Information.IsNothing( this.game.EditObj.ServerChallengeList) || Information.IsNothing( this.game.EditObj.ServerRunningGameList))
        return;
      this.ChallengeListObj = ListClass::new();
      let mut num: i32 =  -1;
      let mut upperBound: i32 =  this.game.EditObj.ServerRunningGameList.GetUpperBound(0);
      for (let mut index: i32 =  0; index <= upperBound; index += 1)
      {
        if (this.gamenr == this.game.EditObj.ServerRunningGameList[index].gameInstanceID)
        {
          num = index;
          this.game.EditObj.PbemGameID = this.game.EditObj.ServerRunningGameList[index].gameInstanceID;
          this.game.EditObj.PbemPlayer1 = this.game.EditObj.ServerRunningGameList[index].Player1UserName;
          this.game.EditObj.PbemPlayer2 = this.game.EditObj.ServerRunningGameList[index].Player2UserName;
          if (this.game.EditObj.ServerRunningGameList[index].GameOver)
            this.game.EditObj.PbemGameOver = 1;
          else
            this.game.EditObj.PbemGameOver = 0;
          this.game.EditObj.PbemChallengeMiscData = "";
        }
        bool flag = true;
        str1: String = "Round " + Strings.Trim(Conversion.Str( this.game.EditObj.ServerRunningGameList[index].TurnNo)) + ", Turn: ";
        tvalue2: String = Operators.CompareString(this.game.EditObj.ServerRunningGameList[index].playerToUserName, this.game.EditObj.ServerRunningGameList[index].Player1UserName, false) != 0 ? str1 + this.game.EditObj.ServerRunningGameList[index].Player2UserName : str1 + this.game.EditObj.ServerRunningGameList[index].Player1UserName;
        tvalue3: String = Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].lastUploaded);
        if (this.game.EditObj.ServerRunningGameList[index].GameOver)
        {
          tvalue3 =  this.game.EditObj.ServerRunningGameList[index].Player1Losses !=  this.game.EditObj.ServerRunningGameList[index].Player2Losses ? ( this.game.EditObj.ServerRunningGameList[index].Player1Losses >=  this.game.EditObj.ServerRunningGameList[index].Player2Losses ? this.game.EditObj.ServerRunningGameList[index].Player2UserName + " WON GAME " : this.game.EditObj.ServerRunningGameList[index].Player1UserName + " WON GAME ") : "DRAW GAME ";
          if (this.game.EditObj.ServerRunningGameList[index].Viewed == 1)
          {
            flag = false;
            tvalue3 += "(viewed)";
          }
          else if (Operators.CompareString(Strings.UCase(this.game.EditObj.ServerRunningGameList[index].playerToUserName), Strings.UCase(this.game.EditObj.PbemUserName), false) != 0)
          {
            flag = false;
            tvalue3 += "(viewed by you)";
          }
        }
        if (flag)
        {
          str2: String = this.game.EditObj.ServerRunningGameList[index].GameName;
          if (str2.Length > 22)
            str2 = Strings.Left(str2, 22) + "...";
          this.ChallengeListObj.add(str2, this.game.EditObj.ServerRunningGameList[index].gameInstanceID, this.game.EditObj.ServerRunningGameList[index].Player1UserName + " VS " + this.game.EditObj.ServerRunningGameList[index].Player2UserName, tvalue2, tvalue3);
        }
      }
      ListClass challengeListObj = this.ChallengeListObj;
      let mut tlistselect: i32 =  num;
      let mut game: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font: Font =  null;
      ref local2: Font = ref font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(challengeListObj, 18, 680, tlistselect, game, tHeaderCenter: false, tShowPair: true, tValueWidth: 510, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: 60, bby: 130, tMarcStyle: true, overruleFont: (ref local2));
      this.ChallengeListId = this.AddSubPart(ref tsubpart, 60, 130, 680, 304, 0);
      this.DrawCurrentGamesTabRefresh();
    }

    pub fn DrawCurrentGamesTabRefresh()
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.PlayTurnId > 0)
        this.RemoveSubPart(this.PlayTurnId);
      if (this.PlayTurn2id > 0)
        this.RemoveSubPart(this.PlayTurn2id);
      if (this.AcceptChallengeId > 0)
        this.RemoveSubPart(this.AcceptChallengeId);
      if (this.AcceptChallengeId2 > 0)
        this.RemoveSubPart(this.AcceptChallengeId2);
      if (this.PreviewId > 0)
        this.RemoveSubPart(this.PreviewId);
      if (this.Preview2Id > 0)
        this.RemoveSubPart(this.Preview2Id);
      if (this.textid > 0)
        this.RemoveSubPart(this.textid);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.CancelId);
      if (this.ClaimId > 0)
        this.RemoveSubPart(this.ClaimId);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.Cancel2Id);
      if (this.Claim2Id > 0)
        this.RemoveSubPart(this.Claim2Id);
      if (this.PassId > 0)
      {
        this.RemoveSubPart(this.PassId);
        this.PassId = 0;
      }
      if (this.PassTextId > 0)
      {
        this.RemoveSubPart(this.PassTextId);
        this.PassTextId = 0;
      }
      if (this.CheckPlayerId > 0)
        this.RemoveSubPart(this.CheckPlayerId);
      if (this.CheckPlayerId2 > 0)
        this.RemoveSubPart(this.CheckPlayerId2);
      bool flag1 = false;
      bool flag2;
      bool flag3;
      tText: String;
      if (this.gamenr > -1)
      {
        let mut upperBound: i32 =  this.game.EditObj.ServerRunningGameList.GetUpperBound(0);
        for (let mut index: i32 =  0; index <= upperBound; index += 1)
        {
          if (this.gamenr == this.game.EditObj.ServerRunningGameList[index].gameInstanceID)
          {
            this.game.EditObj.PbemGameID = this.game.EditObj.ServerRunningGameList[index].gameInstanceID;
            this.game.EditObj.PbemPlayer1 = this.game.EditObj.ServerRunningGameList[index].Player1UserName;
            this.game.EditObj.PbemPlayer2 = this.game.EditObj.ServerRunningGameList[index].Player2UserName;
            if (this.game.EditObj.ServerRunningGameList[index].GameOver)
            {
              this.game.EditObj.PbemGameOver = 1;
              flag2 = true;
            }
            else
              this.game.EditObj.PbemGameOver = 0;
            this.game.EditObj.PbemChallengeMiscData = "";
            if (Operators.CompareString(Strings.UCase(this.game.EditObj.ServerRunningGameList[index].playerToUserName), Strings.UCase(this.game.EditObj.PbemUserName), false) != 0)
              flag3 = true;
            num: i32;
            try
            {
              num = (int) Math.Round(Conversion.Val(Strings.Mid(this.game.EditObj.ServerRunningGameList[index].MiscData, 33, 8)));
            }
            catch (Exception ex)
            {
              ProjectData.SetProjectError(ex);
              num = 0;
              ProjectData.ClearProjectError();
            }
            if (num > 999)
              num = 0;
            if (num > 110)
              flag1 = true;
            if (this.gamenr > -1)
            {
              str1: String = "GAME NAME: " + this.game.EditObj.ServerRunningGameList[index].GameName + "\r\n" + "CHALLENGER: " + this.game.EditObj.ServerRunningGameList[index].Player1UserName + "\r\n" + "OPPONENT: " + this.game.EditObj.ServerRunningGameList[index].Player2UserName + "\r\n";
              str2: String = num <= 0 ? str1 + "VERSION: older\r\n" : str1 + "VERSION: " + num.ToString() + "\r\n";
              if (Operators.CompareString(Strings.UCase(this.game.EditObj.PbemUserName), Strings.UCase(this.game.EditObj.ServerRunningGameList[index].Player1UserName), false) == 0)
                this.game.EditObj.PbemCheckPlayer = this.game.EditObj.ServerRunningGameList[index].Player2UserName;
              else
                this.game.EditObj.PbemCheckPlayer = this.game.EditObj.ServerRunningGameList[index].Player1UserName;
              tText = str2 + "CURRENT TURN: " + this.game.EditObj.ServerRunningGameList[index].playerToUserName + "\r\n" + "DATE STARTED: " + Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].DateIssued) + ", DATE LAST TURN: " + Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].lastUploaded) + "\r\n";
              if (this.game.EditObj.ServerRunningGameList[index].GameOver)
              {
                str3: String = tText + "GAME IS OVER. GAME ENDED ON: " + Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].lastTurnUploaded) + "\r\n";
                str4: String =  this.game.EditObj.ServerRunningGameList[index].Player1Losses >=  this.game.EditObj.ServerRunningGameList[index].Player2Losses ? str3 + "WINNER: " + this.game.EditObj.ServerRunningGameList[index].Player2UserName + " WON GAME " : str3 + "WINNER: " + this.game.EditObj.ServerRunningGameList[index].Player1UserName + " WON GAME ";
                tText = (this.game.EditObj.ServerRunningGameList[index].Viewed != 1 ? str4 + ", VIEWED: NO" : str4 + ", VIEWED: YES") + "\r\n";
              }
            }
          }
        }
      }
      else
        tText = "-no scenario selected-";
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(this.game, 680, 7, this.game.MarcFont4, tText, tbackbitmap: (ref this.OwnBitmap), bbx: 60, bby: 450);
      this.textid = this.AddSubPart(ref tsubpart1, 60, 450, 680, 160, 0);
      if (this.gamenr > -1)
      {
        if (flag3)
        {
          let mut tsubpart2: SubPartClass =  new TextButtonPartClass("PLAY TURN", 240, "Cannot play turn because it is not your turn.", ref this.OwnBitmap, 60, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.PlayTurn2id = this.AddSubPart(ref tsubpart2, 60, 620, 240, 50, 1);
        }
        else if (flag1)
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("PLAY TURN", 240, "Cannot play turn because your version is lower than that of your opponent. Please upgrade your game to the lowest version. ", ref this.OwnBitmap, 60, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.PlayTurn2id = this.AddSubPart(ref tsubpart3, 60, 620, 240, 50, 1);
        }
        else
        {
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("PLAY TURN", 240, "Click to play your turn in this PBEM game.", ref this.OwnBitmap, 60, 620, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.PlayTurnId = this.AddSubPart(ref tsubpart4, 60, 620, 240, 50, 1);
        }
        if (!flag2)
        {
          let mut tsubpart5: SubPartClass =  new TextButtonPartClass("CLAIM GAME", 240, "If other side has not played its turn for 30 days you can claim victory.\r\nand will show it with the challengers configuration.", ref this.OwnBitmap, 320, 620, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.ClaimId = this.AddSubPart(ref tsubpart5, 320, 620, 240, 50, 1);
        }
        else
        {
          let mut tsubpart6: SubPartClass =  new TextButtonPartClass("CLAIM GAME", 240, "You cannot attempt to claim a game thats already over.", ref this.OwnBitmap, 320, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.Claim2Id = this.AddSubPart(ref tsubpart6, 320, 620, 240, 50, 1);
        }
        if (this.game.EditObj.PbemCheckPlayer.Length > 0)
          ;
      }
      else
      {
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("PLAY TURN", 240, "Cannot play turn because you have not selected a game from the list.", ref this.OwnBitmap, 60, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.PlayTurn2id = this.AddSubPart(ref tsubpart7, 60, 620, 240, 50, 1);
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("CLAIM GAME", 240, "Cannot claim because no running game from the list above has been selected.", ref this.OwnBitmap, 320, 620, true, theight: 50, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.Claim2Id = this.AddSubPart(ref tsubpart8, 320, 620, 240, 50, 1);
      }
    }

    pub fn DrawFinishedGamesTab(Graphics g)
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.ChallengeListId > 0)
        this.RemoveSubPart(this.ChallengeListId);
      if (Information.IsNothing( this.game.EditObj.ServerChallengeList) || Information.IsNothing( this.game.EditObj.ServerRunningGameList))
        return;
      this.ChallengeListObj = ListClass::new();
      let mut num: i32 =  -1;
      let mut upperBound: i32 =  this.game.EditObj.ServerRunningGameList.GetUpperBound(0);
      for (let mut index: i32 =  0; index <= upperBound; index += 1)
      {
        bool flag = false;
        if (this.gamenr == this.game.EditObj.ServerRunningGameList[index].gameInstanceID)
        {
          num = index;
          this.game.EditObj.PbemGameID = this.game.EditObj.ServerRunningGameList[index].gameInstanceID;
          this.game.EditObj.PbemPlayer1 = this.game.EditObj.ServerRunningGameList[index].Player1UserName;
          this.game.EditObj.PbemPlayer2 = this.game.EditObj.ServerRunningGameList[index].Player2UserName;
          if (this.game.EditObj.ServerRunningGameList[index].GameOver)
            this.game.EditObj.PbemGameOver = 1;
          else
            this.game.EditObj.PbemGameOver = 0;
          this.game.EditObj.PbemChallengeMiscData = "";
        }
        str1: String = "Round " + Strings.Trim(Conversion.Str( this.game.EditObj.ServerRunningGameList[index].TurnNo)) + ", Turn: ";
        tvalue2: String = Operators.CompareString(this.game.EditObj.ServerRunningGameList[index].playerToUserName, this.game.EditObj.ServerRunningGameList[index].Player1UserName, false) != 0 ? str1 + this.game.EditObj.ServerRunningGameList[index].Player2UserName : str1 + this.game.EditObj.ServerRunningGameList[index].Player1UserName;
        Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].lastUploaded);
        if (this.game.EditObj.ServerRunningGameList[index].GameOver)
        {
          tvalue3: String =  this.game.EditObj.ServerRunningGameList[index].Player1Losses !=  this.game.EditObj.ServerRunningGameList[index].Player2Losses ? ( this.game.EditObj.ServerRunningGameList[index].Player1Losses >=  this.game.EditObj.ServerRunningGameList[index].Player2Losses ? this.game.EditObj.ServerRunningGameList[index].Player2UserName + " WON GAME " : this.game.EditObj.ServerRunningGameList[index].Player1UserName + " WON GAME ") : "DRAW GAME ";
          if (this.game.EditObj.ServerRunningGameList[index].Viewed == 1)
          {
            tvalue3 += "(viewed)";
            flag = true;
          }
          else if (Operators.CompareString(Strings.UCase(this.game.EditObj.ServerRunningGameList[index].playerToUserName), Strings.UCase(this.game.EditObj.PbemUserName), false) != 0)
          {
            tvalue3 += "(viewed by you)";
            flag = true;
          }
          if (flag)
          {
            str2: String = this.game.EditObj.ServerRunningGameList[index].GameName;
            if (str2.Length > 22)
              str2 = Strings.Left(str2, 22) + "...";
            this.ChallengeListObj.add(str2, this.game.EditObj.ServerRunningGameList[index].gameInstanceID, this.game.EditObj.ServerRunningGameList[index].Player1UserName + " VS " + this.game.EditObj.ServerRunningGameList[index].Player2UserName, tvalue2, tvalue3);
          }
        }
      }
      ListClass challengeListObj = this.ChallengeListObj;
      let mut tlistselect: i32 =  num;
      let mut game: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font: Font =  null;
      ref local2: Font = ref font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(challengeListObj, 18, 680, tlistselect, game, tHeaderCenter: false, tShowPair: true, tValueWidth: 510, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: 60, bby: 130, tMarcStyle: true, overruleFont: (ref local2));
      this.ChallengeListId = this.AddSubPart(ref tsubpart, 60, 130, 680, 304, 0);
      this.DrawFinishedGamesTabRefresh();
    }

    pub fn DrawFinishedGamesTabRefresh()
    {
      this.game.EditObj.PbemCheckPlayer = "";
      if (this.PlayTurnId > 0)
        this.RemoveSubPart(this.PlayTurnId);
      if (this.PlayTurn2id > 0)
        this.RemoveSubPart(this.PlayTurn2id);
      if (this.AcceptChallengeId > 0)
        this.RemoveSubPart(this.AcceptChallengeId);
      if (this.AcceptChallengeId2 > 0)
        this.RemoveSubPart(this.AcceptChallengeId2);
      if (this.PreviewId > 0)
        this.RemoveSubPart(this.PreviewId);
      if (this.Preview2Id > 0)
        this.RemoveSubPart(this.Preview2Id);
      if (this.textid > 0)
        this.RemoveSubPart(this.textid);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.CancelId);
      if (this.ClaimId > 0)
        this.RemoveSubPart(this.ClaimId);
      if (this.CancelId > 0)
        this.RemoveSubPart(this.Cancel2Id);
      if (this.Claim2Id > 0)
        this.RemoveSubPart(this.Claim2Id);
      if (this.PassId > 0)
      {
        this.RemoveSubPart(this.PassId);
        this.PassId = 0;
      }
      if (this.PassTextId > 0)
      {
        this.RemoveSubPart(this.PassTextId);
        this.PassTextId = 0;
      }
      if (this.CheckPlayerId > 0)
        this.RemoveSubPart(this.CheckPlayerId);
      if (this.CheckPlayerId2 > 0)
        this.RemoveSubPart(this.CheckPlayerId2);
      tText: String;
      if (this.gamenr > -1)
      {
        let mut upperBound: i32 =  this.game.EditObj.ServerRunningGameList.GetUpperBound(0);
        for (let mut index: i32 =  0; index <= upperBound; index += 1)
        {
          if (this.gamenr == this.game.EditObj.ServerRunningGameList[index].gameInstanceID)
          {
            this.game.EditObj.PbemGameID = this.game.EditObj.ServerRunningGameList[index].gameInstanceID;
            this.game.EditObj.PbemPlayer1 = this.game.EditObj.ServerRunningGameList[index].Player1UserName;
            this.game.EditObj.PbemPlayer2 = this.game.EditObj.ServerRunningGameList[index].Player2UserName;
            if (this.game.EditObj.ServerRunningGameList[index].GameOver)
              this.game.EditObj.PbemGameOver = 1;
            else
              this.game.EditObj.PbemGameOver = 0;
            this.game.EditObj.PbemChallengeMiscData = "";
            if (Operators.CompareString(Strings.UCase(this.game.EditObj.ServerRunningGameList[index].playerToUserName), Strings.UCase(this.game.EditObj.PbemUserName), false) != 0)
              ;
            if (this.gamenr > -1)
            {
              str1: String = "GAME NAME: " + this.game.EditObj.ServerRunningGameList[index].GameName + "\r\n" + "CHALLENGER: " + this.game.EditObj.ServerRunningGameList[index].Player1UserName + "\r\n" + "OPPONENT: " + this.game.EditObj.ServerRunningGameList[index].Player2UserName + "\r\n";
              if (Operators.CompareString(Strings.UCase(this.game.EditObj.PbemUserName), Strings.UCase(this.game.EditObj.ServerRunningGameList[index].Player1UserName), false) == 0)
                this.game.EditObj.PbemCheckPlayer = this.game.EditObj.ServerRunningGameList[index].Player2UserName;
              else
                this.game.EditObj.PbemCheckPlayer = this.game.EditObj.ServerRunningGameList[index].Player1UserName;
              tText = str1 + "CURRENT TURN: " + this.game.EditObj.ServerRunningGameList[index].playerToUserName + "\r\n" + "DATE STARTED: " + Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].DateIssued) + ", DATE LAST TURN: " + Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].lastUploaded) + "\r\n";
              if (this.game.EditObj.ServerRunningGameList[index].GameOver)
              {
                str2: String = tText + "GAME IS OVER. GAME ENDED ON: " + Conversions.ToString(this.game.EditObj.ServerRunningGameList[index].lastTurnUploaded) + "\r\n";
                str3: String =  this.game.EditObj.ServerRunningGameList[index].Player1Losses >=  this.game.EditObj.ServerRunningGameList[index].Player2Losses ? str2 + "WINNER: " + this.game.EditObj.ServerRunningGameList[index].Player2UserName + " WON GAME " : str2 + "WINNER: " + this.game.EditObj.ServerRunningGameList[index].Player1UserName + " WON GAME ";
                tText = (this.game.EditObj.ServerRunningGameList[index].Viewed != 1 ? str3 + ", VIEWED BY BOTH: NO" : str3 + ", VIEWED BY BOTH: YES") + "\r\n";
              }
            }
          }
        }
      }
      else
        tText = "-no scenario selected-";
      let mut tsubpart: SubPartClass =  new TextAreaClass2(this.game, 680, 7, this.game.MarcFont4, tText, tbackbitmap: (ref this.OwnBitmap), bbx: 60, bby: 450);
      this.textid = this.AddSubPart(ref tsubpart, 60, 450, 680, 160, 0);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (nr != 27)
        return windowReturnClass1;
      windowReturnClass2: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.BackId)] + 1, this.SubPartY[this.SubpartNr(this.BackId)] + 1, 1);
      windowReturnClass2.SetFlag(true);
      return windowReturnClass2;
    }

    pub fn DrawTab0(Graphics g, bool active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut x1: i32 =  40;
      let mut y1: i32 =  80;
      if (active)
      {
        ref Graphics local1 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local2: Bitmap = ref bitmap;
        let mut x2: i32 =  x1;
        let mut y2: i32 =  y1;
        DrawMod.DrawSimple(ref local1, ref local2, x2, y2);
      }
      else
      {
        ref Graphics local3 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local4: Bitmap = ref bitmap;
        let mut x3: i32 =  x1;
        let mut y3: i32 =  y1;
        DrawMod.Draw(ref local3, ref local4, x3, y3, -0.1f, -0.1f, -0.1f, 1f);
      }
      str: String = "FINISHED GAMES";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x4: i32 =  (int) Math.Round( ( (x1 + 91) - sizeF2.Width / 2f));
      DrawMod.DrawTextColouredMarc(ref g, str, this.game.MarcFont16, x4, y1 + 4, Color.White);
      Rectangle trect = Rectangle::new(x1, y1, 182, 24);
      this.AddMouse(ref trect, "", "See PBEM games you are playing in that are finished.", 100);
    }

    pub fn DrawTab1(Graphics g, bool active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut x1: i32 =  210;
      let mut y1: i32 =  80;
      if (active)
      {
        ref Graphics local1 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local2: Bitmap = ref bitmap;
        let mut x2: i32 =  x1;
        let mut y2: i32 =  y1;
        DrawMod.DrawSimple(ref local1, ref local2, x2, y2);
      }
      else
      {
        ref Graphics local3 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local4: Bitmap = ref bitmap;
        let mut x3: i32 =  x1;
        let mut y3: i32 =  y1;
        DrawMod.Draw(ref local3, ref local4, x3, y3, -0.1f, -0.1f, -0.1f, 1f);
      }
      str: String = "RUNNING GAMES";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x4: i32 =  (int) Math.Round( ( (x1 + 91) - sizeF2.Width / 2f));
      DrawMod.DrawTextColouredMarc(ref g, str, this.game.MarcFont16, x4, y1 + 4, Color.White);
      Rectangle trect = Rectangle::new(x1, y1, 182, 24);
      this.AddMouse(ref trect, "", "See PBEM games you are playing in that are currently running.", 101);
    }

    pub fn DrawTab2(Graphics g, bool active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut x1: i32 =  380;
      let mut y1: i32 =  80;
      if (active)
      {
        ref Graphics local1 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local2: Bitmap = ref bitmap;
        let mut x2: i32 =  x1;
        let mut y2: i32 =  y1;
        DrawMod.DrawSimple(ref local1, ref local2, x2, y2);
      }
      else
      {
        ref Graphics local3 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local4: Bitmap = ref bitmap;
        let mut x3: i32 =  x1;
        let mut y3: i32 =  y1;
        DrawMod.Draw(ref local3, ref local4, x3, y3, -0.1f, -0.1f, -0.1f, 1f);
      }
      str: String = "YOUR CHALLENGES";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x4: i32 =  (int) Math.Round( ( (x1 + 91) - sizeF2.Width / 2f));
      DrawMod.DrawTextColouredMarc(ref g, str, this.game.MarcFont16, x4, y1 + 4, Color.White);
      Rectangle trect = Rectangle::new(x1, y1, 182, 24);
      this.AddMouse(ref trect, "", "See challenges for PBEM games that you issued..", 102);
    }

    pub fn DrawTab3(Graphics g, bool active = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut x1: i32 =  550;
      let mut y1: i32 =  80;
      if (active)
      {
        ref Graphics local1 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local2: Bitmap = ref bitmap;
        let mut x2: i32 =  x1;
        let mut y2: i32 =  y1;
        DrawMod.DrawSimple(ref local1, ref local2, x2, y2);
      }
      else
      {
        ref Graphics local3 = ref g;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCLARGETAB);
        ref local4: Bitmap = ref bitmap;
        let mut x3: i32 =  x1;
        let mut y3: i32 =  y1;
        DrawMod.Draw(ref local3, ref local4, x3, y3, -0.1f, -0.1f, -0.1f, 1f);
      }
      str: String = "OTHER CHALLENGES";
      SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont16);
      let mut x4: i32 =  (int) Math.Round( ( (x1 + 91) - sizeF2.Width / 2f));
      DrawMod.DrawTextColouredMarc(ref g, str, this.game.MarcFont16, x4, y1 + 4, Color.White);
      Rectangle trect = Rectangle::new(x1, y1, 182, 24);
      this.AddMouse(ref trect, "", "See challenges that other players have issued.", 103);
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.LoadingResult == LoadType.FirstScreen)
      {
        if (this.game.EditObj.ServerCommand == ServerCommandType.PlayTurn)
        {
          this.game.Data.PbemGameID = this.game.EditObj.PbemGameID;
          this.game.Data.PbemPlayer1 = this.game.EditObj.PbemPlayer1;
          this.game.Data.PbemPlayer2 = this.game.EditObj.PbemPlayer2;
          this.game.Data.PbemGameOver = this.game.EditObj.PbemGameOver;
          this.game.EditObj.LoadNoNewEdit = false;
          this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.PlayTurn;
          this.game.EditObj.PbemInspectReturnFromID = this.gamenr;
        }
        this.game.EditObj.LoadingResult = LoadType.None;
        this.game.EditObj.ShownWelcome = true;
        this.game.EditObj.ShowInitialMenu = false;
        windowReturnClass.AddCommand(3, 12);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (this.game.EditObj.LoadingResult == LoadType.PlayScreen)
      {
        if (this.game.EditObj.ServerCommand == ServerCommandType.PlayTurn)
        {
          this.game.Data.PbemGameID = this.game.EditObj.PbemGameID;
          this.game.Data.PbemPlayer1 = this.game.EditObj.PbemPlayer1;
          this.game.Data.PbemPlayer2 = this.game.EditObj.PbemPlayer2;
          this.game.Data.PbemGameOver = this.game.EditObj.PbemGameOver;
          this.game.EditObj.LoadingResult = LoadType.None;
          windowReturnClass.AddCommand(3, 11);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        this.game.EditObj.LoadingResult = LoadType.None;
        let mut num: i32 =  (int) Interaction.MsgBox( "You cannot use the selected file for making a challenge since this game has already begun.", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      if (this.game.EditObj.LoadingResult == LoadType.GameLoop)
      {
        if (this.game.EditObj.ServerCommand == ServerCommandType.PlayTurn)
        {
          this.game.Data.PbemGameID = this.game.EditObj.PbemGameID;
          this.game.Data.PbemPlayer1 = this.game.EditObj.PbemPlayer1;
          this.game.Data.PbemPlayer2 = this.game.EditObj.PbemPlayer2;
          this.game.Data.PbemGameOver = this.game.EditObj.PbemGameOver;
          this.game.EditObj.LoadingResult = LoadType.None;
          windowReturnClass.AddCommand(3, 13);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        this.game.EditObj.LoadingResult = LoadType.None;
        let mut num: i32 =  (int) Interaction.MsgBox( "You cannot use the selected file for making a challenge since this game has already begun.", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.ChallengeMade)
      {
        this.game.FormRef.Cursor = Cursors.WaitCursor;
        this.game.HandyFunctionsObj.SetPbemMiscString();
        this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.Saving;
        str: String = this.game.AppPath + "savedgames\\uploadfile.se1";
        this.game.Data.serialize(str);
        this.game.HandyFunctionsObj.ZipFile(str);
        this.game.FormRef.Cursor = Cursors.Default;
        this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.Saved;
        return windowReturnClass;
      }
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Saved)
      {
        this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.None;
        this.game.EditObj.ServerCommand = ServerCommandType.Challenge;
        this.game.HandyFunctionsObj.SetUploadFile("savedgames\\uploadfile.se1");
        this.game.EditObj.PopupValue = 15;
        windowReturnClass.AddCommand(5, 14);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.TurnPlayed)
      {
        this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.None;
        this.game.EditObj.ServerCommand = ServerCommandType.UploadInstance;
        this.game.HandyFunctionsObj.SetUploadFile("savedgames\\uploadfile.se1");
        this.game.EditObj.PopupValue = 15;
        windowReturnClass.AddCommand(5, 14);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Cancel)
      {
        this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.None;
        this.game.EditObj.FeedBackString = "Cancelled the setting up of a challenge";
        this.game.EditObj.OrderType = 0;
        DrawMod.TGame.EditObj.PopupValue = 5;
        windowReturnClass.AddCommand(5, 10);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.CancelPlayTurn)
      {
        this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.None;
        this.game.EditObj.FeedBackString = "Cancelled the playing of your turn.";
        this.game.EditObj.OrderType = 0;
        DrawMod.TGame.EditObj.PopupValue = 5;
        windowReturnClass.AddCommand(5, 10);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (!Information.IsNothing( this.game.EditObj.TextInputString) && this.game.EditObj.TextInputString.Length > 0)
      {
        if (this.selectedid == this.PassId)
        {
          this.SubPartList[this.SubpartNr(this.selectedid)].Refresh(this.game.EditObj.TextInputString);
          this.SubPartFlag[this.SubpartNr(this.selectedid)] = true;
        }
        windowReturnClass.SetFlag(true);
      }
      this.game.EditObj.TextInputString = "";
      if (this.PassId > 0)
        this.game.EditObj.PbemPrivatePassword = this.SubPartList[this.SubpartNr(this.PassId)].GetText();
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      for (let mut mouseCounter: i32 =  this.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (this.MouseData[mouseCounter] > 0 && x > this.MouseRect[mouseCounter].X & x < this.MouseRect[mouseCounter].X + this.MouseRect[mouseCounter].Width && y > this.MouseRect[mouseCounter].Y & y < this.MouseRect[mouseCounter].Y + this.MouseRect[mouseCounter].Height)
        {
          switch (this.MouseData[mouseCounter])
          {
            case 100:
              this.currentTab = 0;
              this.gamenr = -1;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 101:
              this.currentTab = 1;
              this.gamenr = -1;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 102:
              this.currentTab = 2;
              this.gamenr = -1;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            case 103:
              this.currentTab = 3;
              this.gamenr = -1;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            default:
              continue;
          }
        }
      }
      if (this.SubPartCounter > -1 & b == 1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.PassId)
            {
              this.selectedid = this.PassId;
              this.SubPartList[this.SubpartNr(this.PassId)].Descript = "select".to_owned();
              this.SubPartFlag[this.SubpartNr(this.PassId)] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BackId)
            {
              this.game.EditObj.ShowInitialMenu = true;
              windowReturnClass.AddCommand(3, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.RegisterId)
            {
              this.game.EditObj.PopupValue = 14;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.LoginId)
            {
              this.game.EditObj.PopupValue = 16;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.RefreshId)
            {
              this.game.EditObj.PopupValue = 15;
              this.game.EditObj.ServerCommand = ServerCommandType.Refresh;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.LogOutId)
            {
              this.game.EditObj.PopupValue = 15;
              this.game.EditObj.ServerCommand = ServerCommandType.Logout;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.ClaimId)
            {
              this.game.EditObj.PopupValue = 15;
              this.game.EditObj.ServerCommand = ServerCommandType.Claim;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.CancelId)
            {
              this.game.EditObj.PopupValue = 15;
              this.game.EditObj.ServerCommand = ServerCommandType.CancelChallenge;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.CheckPlayerId)
            {
              this.game.EditObj.PopupValue = 15;
              this.game.EditObj.ServerCommand = ServerCommandType.CheckPlayer;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.PreviewId)
            {
              this.game.EditObj.LoadNoNewEdit = true;
              this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.Inspecting;
              this.game.EditObj.PbemInspectReturnFromID = this.gamenr;
              str: String = "";
              let mut Start: i32 =  32;
              do
              {
                if (Operators.CompareString(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, Start, 1), " ", false) != 0)
                  str = Strings.Mid(this.game.EditObj.PbemChallengeMiscData, Start, 1) + str;
                Start += -1;
              }
              while (Start >= 1);
              this.game.EditObj.LoadFileName = this.game.AppPath + this.game.ModScenarioDir + "\\" + str;
              this.game.EditObj.PopupValue = 17;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.ch1 || num1 == this.ch2 || num1 == this.ch3 || num1 == this.ch4)
            {
              let mut num2: i32 =  -1;
              str: String = this.game.AppPath + this.game.ModScenarioDir + "/";
              switch (num2)
              {
                case 3:
                  if (this.SubPartID[index] == this.ch1)
                    str += "campaign2s.dc2";
                  if (this.SubPartID[index] == this.ch2)
                    str += "campaign3s.dc2";
                  if (this.SubPartID[index] == this.ch3)
                    str += "2ndkharkov.dc2";
                  if (this.SubPartID[index] == this.ch4)
                  {
                    str += "voronezh.dc2";
                    break;
                  }
                  break;
                case 4:
                  if (this.SubPartID[index] == this.ch1)
                  {
                    str += "barbarossa.dc3";
                    break;
                  }
                  break;
                default:
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
              }
              if (File.Exists(str))
              {
                if (str.Replace(this.game.AppPath + this.game.ModScenarioDir + "/", "").Length <= 36)
                {
                  this.game.EditObj.LoadNoNewEdit = true;
                  this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.MakingChallenge;
                  this.game.EditObj.ServerCommand = ServerCommandType.None;
                  this.game.EditObj.LoadFileName = str;
                  this.game.EditObj.PopupValue = 17;
                  windowReturnClass.AddCommand(5, 14);
                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num3: i32 =  (int) Interaction.MsgBox( ("The actual scenario name and location must be 36 characters or less. The current path '" + str.Replace(this.game.AppPath + this.game.ModScenarioDir, "") + "' is " + str.Replace(this.game.AppPath + this.game.ModScenarioDir, "").Length.ToString() + " characters long."), Title: ( "Shadow Empire : Planetary Conquest"));
                return windowReturnClass;
              }
              if (Strings.Len(str) > 1)
              {
                let mut num4: i32 =  (int) Interaction.MsgBox( "File could not be found or op. is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == this.challengeId)
            {
              str: String = this.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to use for your challenge...", this.game.AppPath + this.game.ModScenarioDir, false);
              if (File.Exists(str))
              {
                if (str.Replace(this.game.AppPath + this.game.ModScenarioDir, "").Length <= 36)
                {
                  this.game.EditObj.LoadNoNewEdit = true;
                  this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.MakingChallenge;
                  this.game.EditObj.ServerCommand = ServerCommandType.None;
                  this.game.EditObj.LoadFileName = str;
                  this.game.EditObj.PopupValue = 17;
                  windowReturnClass.AddCommand(5, 14);
                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num5: i32 =  (int) Interaction.MsgBox( ("The actual scenario name and location must be 36 characters or less. The current path '" + str.Replace(this.game.AppPath + this.game.ModScenarioDir, "") + "' is " + str.Replace(this.game.AppPath + this.game.ModScenarioDir, "").Length.ToString() + " characters long."), Title: ( "Shadow Empire : Planetary Conquest"));
                return windowReturnClass;
              }
              if (Strings.Len(str) > 1)
              {
                let mut num6: i32 =  (int) Interaction.MsgBox( "File could not be found or op. is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == this.AcceptChallengeId)
            {
              if (!this.AcceptUsePass)
                this.game.EditObj.PbemPrivatePassword = "";
              this.game.EditObj.ServerCommand = ServerCommandType.AcceptChallenge;
              this.game.EditObj.PbemChallengeID = this.gamenr;
              this.game.EditObj.PopupValue = 15;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.PlayTurnId)
            {
              this.game.EditObj.LoadNoNewEdit = true;
              this.game.EditObj.ServerCommand = ServerCommandType.PlayTurn;
              this.game.EditObj.PbemGameID = this.gamenr;
              this.game.EditObj.PopupValue = 15;
              windowReturnClass.AddCommand(5, 14);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 != this.ChallengeListId)
              return windowReturnClass;
            let mut num7: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (num7 > -1)
              this.gamenr = num7;
            if (this.currentTab == 0)
              this.DrawFinishedGamesTabRefresh();
            if (this.currentTab == 1)
              this.DrawCurrentGamesTabRefresh();
            if (this.currentTab == 2)
              this.DrawYourChallengesTabRefresh();
            if (this.currentTab == 3)
              this.DrawOtherChallengesTabRefresh();
            this.SubPartFlag[index] = true;
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
