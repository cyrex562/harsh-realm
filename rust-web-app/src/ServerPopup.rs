// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ServerPopup
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Globalization;
// usingSystem.Threading;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class ServerPopup : WindowClass
  {
     okid: i32;
     cancelid: i32;
     cancel2id: i32;
     stateid: i32;
     textid: i32;
     ServerClass Server;
     bool ServerThRunning;
     ServerThread: Thread;
     bool doLog;
     bool Finished;
     newWindow: i32;
     animTimer: i32;
     TextSoFar: String;
     bool RequestRefresh;

    pub ServerPopup( tGame: GameClass)
      : base( tGame, 600, 400, 8)
    {
      self.doLog = true;
      self.Finished = false;
      self.newWindow = -1;
      self.game.EditObj.OrigServerCommand = self.game.EditObj.ServerCommand;
      self.Ready();
      self.View();
    }

    pub fn Ready()
    {
      self.game.EditObj.ServerUploadSize = 0L;
      self.game.EditObj.ServerDownloadSize = 0L;
      self.game.EditObj.ServerOrderCancel = false;
      self.game.EditObj.ServerDownloadFile = (byte[]) null;
      self.game.EditObj.ServerCommandMaxStepOrig = 0;
      self.game.EditObj.ServerStatus = ServerStatusType.Sending;
      self.Finished = false;
      self.Server = new ServerClass(self.game);
      self.ServerThread = new Thread(new ThreadStart(self.Server.Go));
      CultureInfo cultureInfo = new CultureInfo("en-US");
      Thread.CurrentThread.CurrentCulture = cultureInfo;
      Thread.CurrentThread.CurrentUICulture = cultureInfo;
      self.ServerThread.Start();
      self.game.FormRef.Cursor = Cursors.WaitCursor;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      this += 1.animTimer;
      if (self.game.EditObj.ServerStatus == ServerStatusType.Completed & !self.Finished)
      {
        self.ServerThread.Abort();
        self.ServerThread.Join();
        self.game.FormRef.Cursor = Cursors.Default;
        self.RequestRefresh = false;
        if (self.game.EditObj.ServerCommand == ServerCommandType.Initialize)
        {
          self.ViewInitialize();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.Register)
        {
          self.ViewRegister();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.CheckPlayer)
          self.ViewCheckPlayer();
        else if (self.game.EditObj.ServerCommand == ServerCommandType.Login)
        {
          self.ViewLogin();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.Refresh)
          self.ViewRefresh();
        else if (self.game.EditObj.ServerCommand == ServerCommandType.Challenge)
        {
          self.ViewChallenge();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.UploadInstance)
        {
          self.ViewUploadInstance();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.AcceptChallenge)
        {
          self.ViewAcceptChallenge();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.Logout)
          self.ViewLogOut();
        else if (self.game.EditObj.ServerCommand == ServerCommandType.CancelChallenge)
        {
          self.ViewCancelChallenge();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.Claim)
        {
          self.ViewClaim();
          if (self.RequestRefresh)
          {
            self.game.EditObj.ServerCommand = ServerCommandType.Refresh;
            self.Ready();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        else if (self.game.EditObj.ServerCommand == ServerCommandType.PlayTurn)
        {
          if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
            self.game.HandyFunctionsObj.SaveDownloadedFile("savedgames/downloadedfile.se1");
          self.ViewPlayTurn();
        }
        else
        {
          self.RemoveSubPart(self.cancelid);
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("OK", 200, tBackbitmap: ( self.OwnBitmap), bbx: 200, bby: 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.okid = self.AddSubPart( tsubpart, 200, 340, 200, 36, 1);
          Rectangle trect = Rectangle::new(200, 530, 200, 35);
          self.AddMouse( trect, "", "Back to main PBEM++ screen");
        }
        windowReturnClass.SetFlag(true);
        self.Finished = true;
      }
      else if (self.game.EditObj.ServerStatus == ServerStatusType.TimeOut & !self.Finished)
      {
        self.ServerThread.Abort();
        self.ServerThread.Join();
        self.game.FormRef.Cursor = Cursors.Default;
        self.ViewTimeOut();
        self.Finished = true;
        windowReturnClass.SetFlag(true);
      }
      else if (!self.Finished)
      {
        self.View();
        windowReturnClass.SetFlag(true);
      }
      return windowReturnClass;
    }

    pub fn Clear()
    {
    }

    pub fn View()
    {
      Graphics g;
      self.DoHeader(g);
      if (!self.game.EditObj.ServerOrderCancel)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Cancel", 200, "Click to break off the current communication with server.\r\nMaybe because the connection seems to have died for instance.",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.cancelid = self.AddSubPart( tsubpart, 200, 340, 200, 36, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Cancel", 200, "You already gave cancel request! Cancellation should be in progress.",  self.OwnBitmap, 200, 340, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.cancel2id = self.AddSubPart( tsubpart, 200, 340, 200, 36, 1);
      }
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      if (self.game.EditObj.ServerTextBuffer.Length > 0)
      {
        self.TextSoFar += self.game.EditObj.ServerTextBuffer;
        self.game.EditObj.ServerTextBuffer = "";
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
    }

    pub fn ViewRegister()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialNotRegistered)
      {
        str2 = str1 + "Your serial is verified." + "\r\n";
        if (self.game.EditObj.ServerRegisterReply == ServerRegisterReplyType.RegisterationSucces)
        {
          str2 += "You have been succesfully registered.";
          self.RequestRefresh = true;
        }
        else if (self.game.EditObj.ServerRegisterReply == ServerRegisterReplyType.RegisterationFailure)
        {
          str3: String = str2 + "You have not been registered.";
          self.game.EditObj.ServerAuthReply = ServerAuthReplyType.AuthFailure;
          str2 = str3 + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
        }
      }
      else if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialRegistered)
      {
        str2 = str1 + "You are already registered. Please try to use the login button in the next screen.";
        self.game.EditObj.ServerAuthReply = ServerAuthReplyType.AuthFailure;
      }
      else if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialFailure)
      {
        str4: String = str1 + "Registration Failed.  \r\n";
        self.game.EditObj.ServerAuthReply = ServerAuthReplyType.AuthFailure;
        str2 = str4 + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      }
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewInitialize()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialRegistered)
      {
        str2 = (Operators.CompareString(self.game.EditObj.PbemSerial, "0000-0000-0000-0000", false) != 0 ? str1 + "Your serial is verified as registered already." : str1 + "Your still need to login a first time or register to get a serial code.") + "\r\n";
        if (self.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthSucces)
        {
          str2 += "You have been logged in.";
          self.RequestRefresh = true;
        }
        else if (self.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthFailure)
          str2 += "You still need to login though.\r\n";
      }
      else if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialNotRegistered)
      {
        str2 = Operators.CompareString(self.game.EditObj.PbemSerial, "0000-0000-0000-0000", false) != 0 ? str1 + "Your serial is registered already." : str1 + "Your still need to login a first time or register to get a serial code.";
        self.game.EditObj.ServerAuthReply = ServerAuthReplyType.AuthFailure;
      }
      else if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.NoConnection)
      {
        str2 = str1 + "No (valid) connection with server.";
        self.game.EditObj.ServerAuthReply = ServerAuthReplyType.AuthFailure;
      }
      else if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialFailure)
      {
        str3: String = Operators.CompareString(self.game.EditObj.PbemSerial, "0000-0000-0000-0000", false) != 0 ? str1 + "Serial verification failed. You'll need to login manually with the correct serial. \r\n" : str1 + "No valid serial present at the moment. You will need to register first. \r\n";
        self.game.EditObj.ServerAuthReply = ServerAuthReplyType.AuthFailure;
        str2 = str3 + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      }
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewLogin()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthSucces)
      {
        str2 = str1 + "You have been logged in.";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthFailure)
        str2 = str1 + "You could not be logged in.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewTimeOut()
    {
      Graphics g;
      self.DoHeader(g);
      self.TextSoFar += "No connection with Server!";
      self.TextSoFar += "\r\n";
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewLogOut()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
        str2 = str1 + "Logout succesfull.";
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Logout did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewRefresh()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
        str2 = str1 + "Refresh succesfull.";
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Refresh request did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewChallenge()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
      {
        str2 = str1 + "Challenge succesfully issued.";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Challenge request did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewUploadInstance()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.Data.PbemGameOver == 1)
      {
        if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
        {
          str2 = str1 + "You have reviewed this finished game. Your opponent already did. This concludes this PBEM++ match.";
          self.RequestRefresh = true;
        }
        else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
          str2 = str1 + "Your turn upload/reviewing of finished game has failed.\r\nKeep in mind you can still find your file in 'savedgames/uploadfile.se1'\r\nYou can load the file from the main menu again and retry sending it.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
      {
        str2 = str1 + "Your turn is succesfully uploaded\r\nYour opponent will be notified he can play his/her turn.";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Your turn upload has failed.\r\nKeep in mind you can still find your file in 'savedgames/uploadfile.se1'\r\nYou can load the file from the main menu again and retry sending it.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewPlayTurn()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
        str2 = str1 + "Succesfully downloaded your turn.";
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Download did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewCancelChallenge()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
      {
        str2 = str1 + "Succesfully cancelled your challenge.";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Cancel challenge did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewClaim()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
      {
        str2 = str1 + "Succesfully claimed victory.";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Claim victory did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewCheckPlayer()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
      {
        str2 = str1 + "Player info on ";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Requesting player info did not work.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn ViewAcceptChallenge()
    {
      Graphics g;
      self.DoHeader(g);
      str1: String;
      str2: String;
      if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
      {
        str2 = str1 + "Challenge succesfully accepted.";
        self.RequestRefresh = true;
      }
      else if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Failure)
        str2 = str1 + "Challenge could not be accepted by you.\r\n" + self.GetExactErrorString(self.game.EditObj.ServerExactErrorCode);
      self.TextSoFar += str2;
      self.TextSoFar += "\r\n";
      if (self.RequestRefresh)
        return;
      if (self.textid > 0)
      {
        self.RemoveSubPart(self.textid);
        self.textid = 0;
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 440, 7, self.game.MarcFont4, self.TextSoFar, 24,  self.OwnBitmap, 80, 80);
      self.textid = self.AddSubPart( tsubpart1, 80, 80, 440, 192, 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, "Back to main PBEM++ screen",  self.OwnBitmap, 200, 340, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.okid = self.AddSubPart( tsubpart2, 200, 340, 200, 36, 1);
    }

    pub fn DoHeader(Graphics g)
    {
      if (self.cancelid > 0)
        self.RemoveSubPart(self.cancelid);
      if (self.okid > 0)
        self.RemoveSubPart(self.okid);
      if (self.cancel2id > 0)
        self.RemoveSubPart(self.cancel2id);
      self.NewBackGroundAndClearAll(600, 400, -1);
      g = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawMessFrame( self.OwnBitmap,  g, 0, 0, 600, 400);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      DrawMod.DrawTextColouredMarcCenter( g, "SERVER COMMUNICATION", self.game.MarcFont1, 300, 27, Color.White);
      self.ClearMouse();
      Rectangle trect1;
      if (self.game.EditObj.ServerUploadSize > 0L)
      {
        g = Graphics.FromImage((Image) self.OwnBitmap);
        let mut num1: i32 = 480;
        let mut num2: i32 = 20;
        let mut w: i32 =  Math.Round(100.0 * ( self.game.EditObj.ServerUploadDone /  self.game.EditObj.ServerUploadSize));
        let mut num3: i32 = 100 - w;
        DrawMod.DrawBlock( g, num1, num2, 100, 15, 0, 0, 0, 64);
        DrawMod.DrawBlock( g, num1, num2, w, 15, 0,  byte.MaxValue, 0, 100);
        DrawMod.DrawRectangle( g, num1, num2, 100, 15,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
        DrawMod.DrawTextColouredMarc( g, Conversion.Str(  Math.Round( self.game.EditObj.ServerUploadDone / 1000.0)) + " KB", self.game.MarcFont14, num1 + 2, num2 + 2, Color.White);
        trect1 = Rectangle::new(num1, num2, 100, 15);
        self.AddMouse( trect1, "FILE UPLOAD PROGRESS", "Currently uploaded " + self.game.EditObj.ServerUploadDone.ToString() + " bytes of " + self.game.EditObj.ServerUploadSize.ToString());
      }
      if (self.game.EditObj.ServerDownloadDone > 0L)
      {
        g = Graphics.FromImage((Image) self.OwnBitmap);
        let mut num4: i32 = 480;
        let mut num5: i32 = 20;
        DrawMod.DrawBlock( g, num4, num5, 100, 15, 0, 0, 0, 64);
        DrawMod.DrawRectangle( g, num4, num5, 100, 15,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
        DrawMod.DrawTextColouredMarc( g, Conversion.Str(  Math.Round( self.game.EditObj.ServerDownloadDone / 1000.0)) + " KB", self.game.MarcFont14, num4 + 2, num5 + 2, Color.White);
        trect1 = Rectangle::new(num4, num5, 100, 15);
        let mut trect2: &Rectangle = &trect1
        self.AddMouse( trect2, "FILE DOWNLOAD PROGRESS", "Currently downloaded " + self.game.EditObj.ServerDownloadDone.ToString() + " bytes.");
      }
      if (self.game.EditObj.ServerCommandMaxStepOrig <= 0)
        return;
      let mut num: i32 = 20;
      let mut y1: i32 = 360;
      let mut commandMaxStepOrig: i32 = self.game.EditObj.ServerCommandMaxStepOrig;
      for (let mut index: i32 = 1; index <= commandMaxStepOrig; index += 1)
      {
        if (index < self.game.EditObj.ServerCommandStep)
          DrawMod.DrawBlock( g, num + 40 * (index - 1), y1, 15, 8, 0,  byte.MaxValue, 0, 150);
        else if (index == self.game.EditObj.ServerCommandStep)
        {
          if (self.animTimer % 10 < 5)
            DrawMod.DrawBlock( g, num + 40 * (index - 1), y1, 15, 8, 0,  byte.MaxValue, 0, 150);
          else
            DrawMod.DrawBlock( g, num + 40 * (index - 1), y1, 15, 8, 0, 0, 0, 150);
        }
        else
          DrawMod.DrawBlock( g, num + 40 * (index - 1), y1, 15, 8, 0, 0, 0, 150);
        DrawMod.DrawRectangle( g, num + 40 * (index - 1), y1, 15, 8,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
      }
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
            if (num == self.okid)
            {
              self.game.EditObj.ServerUploadDone = 0L;
              self.game.EditObj.ServerDownloadDone = 0L;
              self.game.EditObj.ServerDownloadSize = 0L;
              if (self.game.EditObj.OrigServerCommand == ServerCommandType.Initialize)
              {
                self.game.EditObj.ServerStatusInitializeTried = true;
                if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialFailure)
                {
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialRegistered & self.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthFailure)
                {
                  self.game.EditObj.PopupValue = 16;
                  windowReturnClass.AddCommand(5, 14);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (self.game.EditObj.ServerSerialReply == ServerSerialReplyType.SerialNotRegistered | self.game.EditObj.ServerAuthReply == ServerAuthReplyType.AuthFailure)
                {
                  if (self.game.EditObj.PbemAlreadyAccount)
                  {
                    windowReturnClass.AddCommand(6, 0);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  self.game.EditObj.PopupValue = 14;
                  windowReturnClass.AddCommand(5, 14);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.EditObj.OrigServerCommand == ServerCommandType.PlayTurn)
              {
                if (self.game.EditObj.ServerGenericReply == ServerGenericReplyType.Succes)
                {
                  self.game.EditObj.LoadNoNewEdit = true;
                  self.game.EditObj.LoadFileName = self.game.AppPath + "savedgames\\downloadedfile.se1";
                  self.game.EditObj.PopupValue = 17;
                  windowReturnClass.AddCommand(5, 14);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.textid)
            {
              self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.cancelid)
            {
              self.game.EditObj.ServerOrderCancel = true;
              self.View();
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

    pub GetExactErrorString: String(err: i32)
    {
      str: String = "";
      if (err == 900)
        str = "Lost connection with server.";
      if (err == 1)
        str = "Did not transfer enough data. [1]";
      if (err == 2)
        str = "Serial number is blacklisted. [2]";
      if (err == 3)
        str = "Serial number is non-existant. [3]";
      if (err == 4)
        str = "Serial number is registered. [4]";
      if (err == 5)
        str = "Serial number is not registered. [5]";
      if (err == 6)
        str = "User has not logged in. [6]";
      if (err == 7)
        str = "Login name already exists, pick other one please. [7]";
      if (err == 8)
        str = "Login/pass is not correct. Or username is to short or already exists. [8]";
      if (err == 9)
        str = "Login is not activated. [9]";
      if (err == 10)
        str = "Login/pass is wrong for serial number. [10]";
      if (err == 11)
        str = "User has one or more serials. [11]";
      if (err == 12)
        str = "Email already exists with other user. Pick other email please. [12]";
      if (err == 13)
        str = "Email is not correct. [13]";
      if (err == 14)
        str = "Password length is short (5-15 characters). [14]";
      if (err == 15)
        str = "Script/Database error on server [15]";
      if (err == 20)
        str = "Game folder is undefined. [20]";
      if (err == 21)
        str = "Game folder is non-existent. [21]";
      if (err == 22)
        str = "The game is not active for serials-exchange. [22]";
      if (err == 23)
        str = "Wrong Turn! [23]";
      if (err == 24)
        str = "IP adress locked out for 10 minutes. [24]";
      if (err == 25)
        str = "Too many challenges issued already! [25]";
      if (err == 26)
        str = "Too many challenges accepted today! [26]";
      if (err == 27)
        str = "Too many total games or challenges in progress. [27]";
      if (err == 28)
        str = "Game already accepted. [28]";
      if (err == 29)
        str = "You are not in this game [29]";
      if (err == 30)
        str = "Game already finished [30]";
      if (err == 31)
        str = "The game has been updated in the last 30 days [31]";
      if (err == 32)
        str = "Game is not finished yet. [32]";
      if (err == 33)
        str = "Email not sent. [33]";
      if (err == 34)
        str = "Invalid challenge password [34]";
      if (err == 35)
        str = "Password length is too long (5-15 characters). [35]";
      if (err == 36)
        str = "The file-size is different from what client posted to server [36].";
      if (err == 37)
        str = "The Cookie seems to be corrupted [37].";
      if (err == 38)
        str = "The server could not write the file [38].";
      if (err == 39)
        str = "File size limit exceeded [39].";
      if (err == 40)
        str = "User disk-space limit exceeded [40].";
      if (err == 41)
        str = "CRC32 check failed upon upload. [41]";
      if (err == 42)
        str = "CRC32 check failed writing too server. [42]";
      if (err == 43)
        str = "The server could not write the file. [43]";
      if (err == 44)
        str = "CRC32 check failed writing into the server. [44]";
      if (err == 45)
        str = "You are not allowed to see the message [45].";
      if (err == 46)
        str = "Authorization Cookie is missing. [46].";
      if (str.Length < 1)
        str = "Unknown Error [" + err.ToString() + "]";
      return "Error Report: '" + str + "'";
    }
  }
}
