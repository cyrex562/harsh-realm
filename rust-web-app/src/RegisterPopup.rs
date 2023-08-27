// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RegisterPopup
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class RegisterPopup : WindowClass
  {
     okid: i32;
     cancelid: i32;
     userid: i32;
     passid: i32;
     serialid: i32;
     emailid: i32;
     selectedid: i32;

    pub RegisterPopup( tGame: GameClass)
      : base( tGame, 600, 480, 8)
    {
      self.View();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          let mut num: i32 = self.SubPartID[index];
          if (num == self.userid)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "USERNAME";
            self.game.EditObj.TipText = "Enter a username of choice here.";
          }
          else if (num == self.passid)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "PASSWORD";
            self.game.EditObj.TipText = "Enter a password of choice here.";
          }
          else if (num == self.serialid)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "SERIAL CODE";
            self.game.EditObj.TipText = "Your serial code. You cannot change this field.";
          }
          else if (num == self.emailid)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "EMAIL";
            self.game.EditObj.TipText = "Enter your email adress. This will probably be of use to notify you of new PBEM turns.";
          }
        }
      }
    }

    pub fn View()
    {
      self.ClearMouse();
      self.NewBackGroundAndClearAll(600, 480, -1);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 600, 480);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      DrawMod.DrawTextColouredMarc( graphics, "REGISTER WITH PBEM++ SERVER", self.game.MarcFont1, 98, 27, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, "Username:", self.game.MarcFont4, 80, 75, Color.White);
      let mut tsubpart1: SubPartClass =  new InputTextClass("", self.game.MarcFont4, 440, 36, false, 20, true);
      self.userid = self.AddSubPart( tsubpart1, 80, 100, 440, 36, 0);
      DrawMod.DrawTextColouredMarc( graphics, "Password:", self.game.MarcFont4, 80, 155, Color.White);
      let mut tsubpart2: SubPartClass =  new InputTextClass("", self.game.MarcFont4, 440, 36, false, 20, true);
      self.passid = self.AddSubPart( tsubpart2, 80, 180, 440, 36, 0);
      DrawMod.DrawTextColouredMarc( graphics, "Serial code:", self.game.MarcFont4, 80, 235, Color.White);
      let mut tsubpart3: SubPartClass =  new InputTextClass("", self.game.MarcFont4, 440, 36, true, 19, true);
      self.serialid = self.AddSubPart( tsubpart3, 80, 260, 440, 36, 0);
      DrawMod.DrawTextColouredMarc( graphics, "Email:", self.game.MarcFont4, 80, 315, Color.White);
      let mut tsubpart4: SubPartClass =  new InputTextClass("", self.game.MarcFont4, 440, 36, false, 40, true);
      self.emailid = self.AddSubPart( tsubpart4, 80, 340, 440, 36, 0);
      if (Information.IsNothing( self.game.EditObj.PbemEmail))
        self.game.EditObj.PbemEmail = "";
      if (Information.IsNothing( self.game.EditObj.PbemPassword))
        self.game.EditObj.PbemPassword = "";
      if (Information.IsNothing( self.game.EditObj.PbemUserName))
        self.game.EditObj.PbemUserName = "";
      if (Information.IsNothing( self.game.EditObj.PbemSerial))
        self.game.EditObj.PbemSerial = "";
      self.SubPartList[self.SubpartNr(self.userid)].Refresh(self.game.EditObj.PbemUserName);
      self.SubPartList[self.SubpartNr(self.passid)].Refresh(self.game.EditObj.PbemPassword);
      self.SubPartList[self.SubpartNr(self.serialid)].Refresh(self.game.EditObj.PbemSerial);
      self.SubPartList[self.SubpartNr(self.emailid)].Refresh(self.game.EditObj.PbemEmail);
      let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Already Registered user", 200, "Click to go directly to PBEM++ screen for login with an existing account.",  self.OwnBitmap, 80, 410, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.cancelid = self.AddSubPart( tsubpart5, 80, 410, 200, 36, 1);
      let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Register User", 200, "Register with the PBEM++ server if you do not have an existing user account yet.",  self.OwnBitmap, 320, 410, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart6, 320, 410, 200, 36, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27)
        {
          windowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.cancelid)] + 1, self.SubPartY[self.SubpartNr(self.cancelid)] + 1, 1);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!Information.IsNothing( self.game.EditObj.TextInputString) && self.game.EditObj.TextInputString.Length > 0)
      {
        if (self.selectedid == self.userid)
        {
          self.SubPartList[self.SubpartNr(self.selectedid)].Refresh(self.game.EditObj.TextInputString);
          self.SubPartFlag[self.SubpartNr(self.selectedid)] = true;
        }
        else if (self.selectedid == self.passid)
        {
          self.SubPartList[self.SubpartNr(self.passid)].Refresh(self.game.EditObj.TextInputString);
          self.SubPartFlag[self.SubpartNr(self.passid)] = true;
        }
        else if (self.selectedid == self.emailid)
        {
          self.SubPartList[self.SubpartNr(self.emailid)].Refresh(self.game.EditObj.TextInputString);
          self.SubPartFlag[self.SubpartNr(self.emailid)] = true;
        }
        windowReturnClass.SetFlag(true);
      }
      self.game.EditObj.TextInputString = "";
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num: i32 = self.SubPartID[index];
            if (num == self.cancelid)
            {
              self.game.EditObj.Save("editobj.txt");
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.okid)
            {
              self.game.EditObj.PbemUserName = self.SubPartList[self.SubpartNr(self.userid)].GetText();
              self.game.EditObj.PbemPassword = self.SubPartList[self.SubpartNr(self.passid)].GetText();
              self.game.EditObj.PbemSerial = self.SubPartList[self.SubpartNr(self.serialid)].GetText();
              self.game.EditObj.PbemEmail = self.SubPartList[self.SubpartNr(self.emailid)].GetText();
              self.game.EditObj.ServerCommand = ServerCommandType.Register;
              self.game.EditObj.Save("editobj.txt");
              self.game.EditObj.PopupValue = 15;
              windowReturnClass.AddCommand(5, 14);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.userid)
            {
              self.selectedid = self.userid;
              self.SubPartList[self.SubpartNr(self.userid)].Descript = "select";
              self.SubPartList[self.SubpartNr(self.serialid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.passid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.emailid)].Descript = "";
              self.SubPartFlag[self.SubpartNr(self.userid)] = true;
              self.SubPartFlag[self.SubpartNr(self.serialid)] = true;
              self.SubPartFlag[self.SubpartNr(self.passid)] = true;
              self.SubPartFlag[self.SubpartNr(self.emailid)] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.emailid)
            {
              self.selectedid = self.emailid;
              self.SubPartList[self.SubpartNr(self.emailid)].Descript = "select";
              self.SubPartList[self.SubpartNr(self.userid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.serialid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.passid)].Descript = "";
              self.SubPartFlag[self.SubpartNr(self.userid)] = true;
              self.SubPartFlag[self.SubpartNr(self.serialid)] = true;
              self.SubPartFlag[self.SubpartNr(self.passid)] = true;
              self.SubPartFlag[self.SubpartNr(self.emailid)] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.serialid)
            {
              self.selectedid = self.serialid;
              self.SubPartList[self.SubpartNr(self.emailid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.userid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.serialid)].Descript = "select";
              self.SubPartList[self.SubpartNr(self.passid)].Descript = "";
              self.SubPartFlag[self.SubpartNr(self.userid)] = true;
              self.SubPartFlag[self.SubpartNr(self.serialid)] = true;
              self.SubPartFlag[self.SubpartNr(self.passid)] = true;
              self.SubPartFlag[self.SubpartNr(self.emailid)] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.passid)
            {
              self.selectedid = self.passid;
              self.SubPartList[self.SubpartNr(self.passid)].Descript = "select";
              self.SubPartList[self.SubpartNr(self.serialid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.userid)].Descript = "";
              self.SubPartList[self.SubpartNr(self.emailid)].Descript = "";
              self.SubPartFlag[self.SubpartNr(self.userid)] = true;
              self.SubPartFlag[self.SubpartNr(self.serialid)] = true;
              self.SubPartFlag[self.SubpartNr(self.passid)] = true;
              self.SubPartFlag[self.SubpartNr(self.emailid)] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
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
