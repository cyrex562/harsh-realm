// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ServerClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Net;
// usingSystem.Text;
// usingSystem.Threading;
// usingSystem.Web;

namespace WindowsApplication1
{
  pub class ServerClass
  {
     game: GameClass;
     e: EditClass;
     HttpWebRequest wr;
     url: String;
     baseurl: String;
     exchangebaseurl: String;
     pbempassword: String;
     bool debug;
     receive: String;
     AutoResetEvent auto;
     ManualResetEvent manual;

    pub ServerClass(tgame: GameClass)
    {
      self.baseurl = "defaultURL";
      self.exchangebaseurl = "defaultExchangeURL";
      self.pbempassword = "defaultPass";
      self.debug = true;
      self.game = tgame;
      self.e = self.game.EditObj;
      self.e.ServerLostConnect = false;
      self.e.ServerTextBuffer = "";
      if (self.game.EditObj.donequest != 0)
        return;
      self.game.EditObj.donequest = 1;
      self.game.EditObj.donequest = 2;
    }

     void Response(IAsyncResult ar)
    {
      ServerClass.WebRequestState asyncState = (ServerClass.WebRequestState) ar.AsyncState;
      HttpWebRequest request = (HttpWebRequest) asyncState.Request;
      asyncState.lastRead = DateAndTime.Now;
      if (asyncState.e.ServerOrderCancel)
      {
        if (!Information.IsNothing( asyncState.Response))
          asyncState.Response.Close();
        request.Abort();
        asyncState.e.ServerLostConnect = true;
      }
      else
      {
        try
        {
          asyncState.Response = request.EndGetResponse(ar);
          asyncState.RespStream = asyncState.Response.GetResponseStream();
          asyncState.RespStream.BeginRead(asyncState.buf, 0, 131072, new AsyncCallback(self.ReadResponse),  asyncState);
        }
        catch (WebException ex)
        {
          ProjectData.SetProjectError((Exception) ex);
          asyncState.e.ServerOrderCancel = true;
          if (!Information.IsNothing( asyncState.Response))
            asyncState.Response.Close();
          request.Abort();
          asyncState.e.ServerLostConnect = true;
          ProjectData.ClearProjectError();
        }
      }
    }

     void ReadResponse(IAsyncResult ar)
    {
      self.receive = "";
      ServerClass.WebRequestState asyncState = (ServerClass.WebRequestState) ar.AsyncState;
      HttpWebRequest request = (HttpWebRequest) asyncState.Request;
      WebResponse response = asyncState.Response;
      Stream respStream = asyncState.RespStream;
      try
      {
        asyncState.lastRead = DateAndTime.Now;
        let mut count: i32 = respStream.EndRead(ar);
        if (count > 0 & !asyncState.completed)
        {
          self.auto.WaitOne();
          if (self.e.ServerCommand == ServerCommandType.PlayTurn)
          {
            if (!asyncState.responseStarted)
            {
              asyncState.e.ServerDownloadDone = 0L;
              asyncState.uploaddone = 0L;
              asyncState.upload = new byte[1];
              asyncState.responseStarted = true;
            }
            if (asyncState.uploaddone + (long) count + 32368L > (long) asyncState.upload.GetUpperBound(0))
              asyncState.upload = (byte[]) Utils.CopyArray((Array) asyncState.upload, (Array) new byte[asyncState.upload.GetUpperBound(0) + 1045876 + 1]);
            asyncState.e.ServerStatus = ServerStatusType.Receiving;
            let mut num: i32 = count - 1;
            for (let mut index: i32 = 0; index <= num; index += 1)
              asyncState.upload[ (asyncState.uploaddone + (long) index)] = asyncState.buf[index];
            asyncState.uploaddone += (long) count;
            asyncState.e.ServerDownloadDone += (long) count;
          }
          else if (count > 0)
          {
            self.e.ServerStatus = ServerStatusType.Receiving;
            asyncState.tempreceive += Encoding.ASCII.GetString(asyncState.buf, 0, count);
            self.e = self.e;
          }
          while (!respStream.CanRead)
            Thread.Sleep(1);
          self.auto.Set();
          respStream.BeginRead(asyncState.buf, 0, 131072, new AsyncCallback(self.ReadResponse),  asyncState);
          if (self.e.ServerOrderCancel)
          {
            asyncState.e.ServerLostConnect = true;
            respStream.Close();
            request.Abort();
          }
          if (self.e.ServerCommand == ServerCommandType.PlayTurn)
          {
            if (!self.e.ServerOrderCancel)
              return;
            respStream.Close();
            asyncState.completed = true;
            asyncState.Response = response;
            asyncState.e.ServerLostConnect = true;
          }
          else
          {
            if (!(response.ContentLength <= (long) asyncState.tempreceive.Length | self.e.ServerOrderCancel))
              return;
            respStream.Close();
            asyncState.completed = true;
            asyncState.Response = response;
          }
        }
        else
        {
          respStream.Close();
          request.Abort();
          asyncState.completed = true;
          asyncState.Response = response;
          asyncState.e.ServerLostConnect = true;
          self.auto.Set();
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.e.ServerOrderCancel = true;
        if (!Information.IsNothing( respStream))
          respStream.Close();
        request.Abort();
        asyncState.completed = true;
        asyncState.Response = response;
        self.auto.Set();
        ProjectData.ClearProjectError();
      }
    }

    pub fn Go()
    {
      self.e.ServerMessages = MessageList::new();
      self.e.ServerCommandMaxStep = 1;
      if (self.e.ServerCommand == ServerCommandType.Refresh)
        self.e.ServerCommandMaxStep = 2;
      if (self.e.ServerCommand == ServerCommandType.UploadInstance)
        self.e.ServerCommandMaxStep = 3;
      if (self.e.ServerCommand == ServerCommandType.Initialize)
        self.e.ServerCommandMaxStep = 2;
      if (self.e.ServerCommand == ServerCommandType.Register)
        self.e.ServerCommandMaxStep = 3;
      self.e.ServerCommandMaxStepOrig = self.e.ServerCommandMaxStep;
      e: EditClass = self.e;
      let mut serverCommandMaxStep: i32 = self.e.ServerCommandMaxStep;
      for (e.ServerCommandStep = 1; self.e.ServerCommandStep <= serverCommandMaxStep; this += 1.e.ServerCommandStep)
      {
        if (self.e.ServerCommandStep <= self.e.ServerCommandMaxStep)
        {
          self.receive = "";
          self.e.ServerStatus = ServerStatusType.Sending;
          self.SetUrl();
          if (self.debug)
            self.WriteLog("SEND: " + self.url);
          self.e.ServerStatus = ServerStatusType.Sending;
          self.e.ServerStatusStartTime = DateTime.Now;
          self.wr = (HttpWebRequest) WebRequest.Create(self.url);
          self.wr.Timeout = 9000;
          if (Operators.CompareString(self.e.ServerCookieValue, "", false) != 0)
            self.wr.Headers.Add(HttpRequestHeader.Cookie, self.e.ServerCookieValue);
          DateTime now;
          if (self.e.ServerCommand == ServerCommandType.Challenge | self.e.ServerCommand == ServerCommandType.UploadInstance & self.e.ServerCommandStep == 3 && !Information.IsNothing( self.e.ServerUploadFile))
          {
            now = DateTime.Now;
            str: String = "----------------------------" + now.Ticks.ToString("x");
            self.wr.Method = "POST";
            self.wr.ContentType = "myHttpWebRequest.ContentType = \"application/x-www-form-urlencoded\"";
            self.wr.Timeout = 320000;
            self.wr.ContentLength = (long) (self.e.ServerUploadFile.GetUpperBound(0) + 1);
            self.e.ServerUploadSize = self.wr.ContentLength;
            Stream requestStream = self.wr.GetRequestStream();
            byte[] buffer = new byte[1025];
            num1: i32;
            while (num1 < self.e.ServerUploadFile.GetUpperBound(0) + 1)
            {
              let mut count: i32 = 0;
              let mut num2: i32 = num1;
              let mut num3: i32 = num1 + 1023;
              for (let mut index: i32 = num2; index <= num3; index += 1)
              {
                if (num1 + count <= self.e.ServerUploadFile.GetUpperBound(0))
                {
                  buffer[count] = self.e.ServerUploadFile[index];
                  count += 1;
                }
                else
                  count = count;
              }
              requestStream.Write(buffer, 0, count);
              num1 += count;
              self.e.ServerUploadDone = (long) num1;
              if (self.e.ServerOrderCancel)
              {
                self.wr.Abort();
                break;
              }
            }
            if (!self.e.ServerOrderCancel)
              requestStream.Close();
          }
          self.receive = "";
          byte[] numArray = new byte[8193];
          ServerClass.WebRequestState state = new ServerClass.WebRequestState((WebRequest) self.wr);
          state.e = self.e;
          if (!self.e.ServerOrderCancel)
          {
            self.auto = new AutoResetEvent(true);
            self.wr.BeginGetResponse(new AsyncCallback(self.Response),  state);
            state.lastRead = DateAndTime.Now;
            while (!state.completed)
            {
              Thread.Sleep(1);
              if (self.e.ServerOrderCancel)
              {
                now = DateAndTime.Now;
                if (now.Ticks - 20000000L > state.lastRead.Ticks)
                {
                  state.Request.Abort();
                  state.completed = true;
                }
              }
            }
            self.receive = state.tempreceive;
            if (self.e.ServerCommand == ServerCommandType.PlayTurn & self.e.ServerCommandStep == 1)
            {
              if (state.uploaddone < 99L)
              {
                self.receive = Encoding.ASCII.GetString(state.upload, 0,  state.uploaddone);
              }
              else
              {
                self.e.ServerDownloadFile = new byte[ (state.uploaddone - 1L) + 1];
                let mut num: i32 =  (state.uploaddone - 1L);
                for (let mut index: i32 = 0; index <= num; index += 1)
                  self.e.ServerDownloadFile[index] = state.upload[index];
              }
            }
            if (!self.e.ServerOrderCancel)
            {
              HttpWebResponse response = (HttpWebResponse) state.Response;
              if (response.Headers.Count > 0)
              {
                let mut num: i32 = response.Headers.Count - 1;
                for (let mut index: i32 = 0; index <= num; index += 1)
                {
                  if (Operators.CompareString(response.Headers.Keys[index], "Set-Cookie", false) == 0)
                  {
                    header: String = response.Headers[index];
                    self.e.ServerCookieValue = Strings.Mid(header, 1, header.IndexOf(';') + 1);
                  }
                }
              }
            }
            else
              self.receive = "99 ***CANCELLED BY CLIENT***";
          }
          else
            self.receive = "99 ***CANCELLED BY CLIENT***";
          self.e.ServerStatus = ServerStatusType.Received;
          if (self.debug)
            self.WriteLog("RECV: " + self.receive);
          self.ReadReceive();
        }
      }
      self.e.ServerUploadFile = (byte[]) null;
      self.e.ServerStatus = ServerStatusType.Completed;
    }

     void ScanTimeoutCallback( object state, bool timedOut)
    {
      if (!timedOut)
        return;
      ServerClass.WebRequestState Expression = (ServerClass.WebRequestState) state;
      if (Information.IsNothing( Expression))
        return;
      Expression.Request.Abort();
    }

    pub fn SetUrl()
    {
      if (self.game.EditObj.PbemSteam)
      {
        if (self.e.ServerCommand == ServerCommandType.Initialize)
        {
          self.url = self.baseurl;
          if (self.e.ServerCommandStep == 1)
          {
            self.e.ServerTextBuffer += "Checking username+password...\r\n";
            self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
            if (Operators.CompareString(self.game.EditObj.PbemSerial, "0000-0000-0000-0000", false) == 0)
              self.url = self.url + "auth_steam.php?client_serial=new&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
            else
              self.url = self.url + "auth_steam.php?client_serial=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
          }
        }
      }
      else if (!self.game.EditObj.PbemSteam && self.e.ServerCommand == ServerCommandType.Initialize)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Checking serial number...\r\n";
          self.e.ServerSerialReply = ServerSerialReplyType.SerialFailure;
          self.url = self.url + "serial_information.php?serial_num=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&password=" + self.pbempassword;
        }
        if (self.e.ServerCommandStep == 2)
        {
          self.e.ServerTextBuffer += "Checking username+password...\r\n";
          self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
          self.url = self.url + "auth_steam.php?client_serial=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
        }
      }
      if (self.game.EditObj.PbemSteam)
      {
        if (self.e.ServerCommand == ServerCommandType.Register)
        {
          self.url = self.baseurl;
          if (self.e.ServerCommandStep == 1)
          {
            self.e.ServerTextBuffer += "Registering serial number...\r\n";
            self.e.ServerRegisterReply = ServerRegisterReplyType.RegisterationFailure;
            self.url = self.url + "register_steam.php?login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword) + "&email=" + HttpUtility.UrlEncode(self.game.EditObj.PbemEmail);
          }
          else if (self.e.ServerCommandStep == 2)
          {
            self.e.ServerTextBuffer += "Checking user/pass...\r\n";
            self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
            self.url = self.url + "auth_steam.php?client_serial=new&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
          }
        }
      }
      else if (!self.game.EditObj.PbemSteam && self.e.ServerCommand == ServerCommandType.Register)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Checking serial number...\r\n";
          self.e.ServerSerialReply = ServerSerialReplyType.SerialFailure;
          self.url = self.url + "serial_information.php?serial_num=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&password=" + self.pbempassword;
        }
        else if (self.e.ServerCommandStep == 2)
        {
          self.e.ServerTextBuffer += "Registering serial number...\r\n";
          self.e.ServerRegisterReply = ServerRegisterReplyType.RegisterationFailure;
          self.url = self.url + "register.php?serial_num=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword) + "&email=" + HttpUtility.UrlEncode(self.game.EditObj.PbemEmail);
        }
        else if (self.e.ServerCommandStep == 3)
        {
          self.e.ServerTextBuffer += "Checking user/pass...\r\n";
          self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
          self.url = self.url + "auth_steam.php?client_serial=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
        }
      }
      if (self.game.EditObj.PbemSteam)
      {
        if (self.e.ServerCommand == ServerCommandType.Login)
        {
          self.url = self.baseurl;
          if (self.e.ServerCommandStep == 1)
          {
            self.e.ServerTextBuffer += "Checking user/pass...\r\n";
            self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
            if (Operators.CompareString(self.game.EditObj.PbemSerial, "0000-0000-0000-0000", false) == 0)
              self.url = self.url + "auth_steam.php?client_serial=new&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
            else
              self.url = self.url + "auth_steam.php?client_serial=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
          }
        }
      }
      else if (self.e.ServerCommand == ServerCommandType.Login)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Checking user/pass...\r\n";
          self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
          self.url = self.url + "auth.php?serial_num=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
        }
      }
      if (self.e.ServerCommand == ServerCommandType.CheckPlayer)
      {
        self.url = self.exchangebaseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Checking info on a player...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url = self.url + "rankings.php?login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemCheckPlayer);
        }
      }
      if (self.e.ServerCommand == ServerCommandType.Refresh)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Updating challenges...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url += "list_challenges.php";
        }
        if (self.e.ServerCommandStep == 2)
        {
          self.e.ServerTextBuffer += "Updating running games...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url += "games_list.php";
        }
      }
      if (self.e.ServerCommand == ServerCommandType.Logout)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Logging out...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url = self.url + "logout.php?password=" + self.pbempassword + "&serial_num=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial);
        }
      }
      if (self.e.ServerCommand == ServerCommandType.AcceptChallenge)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Accepting a challenge...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          if (self.game.EditObj.PbemPrivatePassword.Length > 0)
            self.url = self.url + "accept_challenge.php?ChallengeID=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.EditObj.PbemChallengeID)) + "&ChallengePassword=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPrivatePassword);
          else
            self.url = self.url + "accept_challenge.php?ChallengeID=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.EditObj.PbemChallengeID));
        }
      }
      if (self.e.ServerCommand == ServerCommandType.CancelChallenge)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Cancelling a challenge...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url = self.url + "cancel_challenge.php?ChallengeID=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.EditObj.PbemChallengeID));
        }
      }
      if (self.e.ServerCommand == ServerCommandType.PlayTurn)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Playing a turn (downloading game file)...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url = self.url + "download_instance.php?InstanceID=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.EditObj.PbemGameID));
        }
      }
      if (self.e.ServerCommand == ServerCommandType.Claim)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          self.e.ServerTextBuffer += "Claiming a game...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url = self.url + "claim_old_game.php?InstanceID=" + HttpUtility.UrlEncode(Conversions.ToString(self.e.PbemGameID));
        }
      }
      if (self.e.ServerCommand == ServerCommandType.Challenge)
      {
        self.url = self.baseurl;
        if (self.e.ServerCommandStep == 1)
        {
          let mut num1: i32 = 1;
          let mut num2: i32 = 1;
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
          {
            if (!self.game.Data.RegimeObj[index].Sleep & !self.game.Data.RegimeObj[index].AI)
            {
              if (self.game.Data.RegimeObj[index].PbemPlayer == 1)
              {
                num1 = 1;
                num2 = 1;
                break;
              }
              if (self.game.Data.RegimeObj[index].PbemPlayer == 2)
              {
                num1 = 1;
                num2 = 2;
                break;
              }
            }
          }
          self.e.ServerTextBuffer += "Issuing a challenge...\r\n";
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          self.url += "issue_challenge.php?";
          self.url = self.url + "GameName=" + HttpUtility.UrlEncode(self.game.EditObj.PbemTitle);
          if (self.game.EditObj.PbemPrivatePassword.Length > 0)
            self.url = self.url + "&ChallengePassword=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPrivatePassword);
          self.url = self.url + "&ChallengerSide=" + HttpUtility.UrlEncode(Conversions.ToString(num1));
          self.url = self.url + "&FirstPlayerSide=" + HttpUtility.UrlEncode(Conversions.ToString(num2));
          self.url = self.url + "&ProductID=" + HttpUtility.UrlEncode("0");
          self.url = self.url + "&ArmyName=" + HttpUtility.UrlEncode(self.game.Data.RegimeObj[0].Name);
          self.url = self.url + "&ArmyPoints=" + HttpUtility.UrlEncode("0");
          self.url = self.url + "&ShowName=" + HttpUtility.UrlEncode("1");
          self.url = self.url + "&MiscData=" + HttpUtility.UrlEncode(self.game.EditObj.PbemMiscString);
          self.url = self.url + "&FileSize=" + HttpUtility.UrlEncode(Strings.Trim(Conversion.Str( (self.game.EditObj.ServerUploadFile.GetUpperBound(0) + 1))));
        }
      }
      if (self.e.ServerCommand == ServerCommandType.UploadInstance)
      {
        self.url = self.baseurl;
        let mut num: i32 = 0;
        if (self.e.ServerCommandStep + num == 1)
        {
          self.e.ServerTextBuffer += "Checking serial number...\r\n";
          self.e.ServerSerialReply = ServerSerialReplyType.SerialFailure;
          self.url = self.url + "serial_information.php?serial_num=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&password=slith_exchange";
        }
        if (self.e.ServerCommandStep + num == 2)
        {
          self.e.ServerTextBuffer += "Checking username+password...\r\n";
          self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
          if (self.game.EditObj.PbemSteam)
            self.url = self.url + "auth_steam.php?client_serial=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
          else
            self.url = self.url + "auth_steam.php?client_serial=" + HttpUtility.UrlEncode(self.game.EditObj.PbemSerial) + "&login=" + HttpUtility.UrlEncode(self.game.EditObj.PbemUserName) + "&password=" + HttpUtility.UrlEncode(self.game.EditObj.PbemPassword);
        }
        if (self.e.ServerCommandStep + num == 3)
        {
          self.e.ServerGenericReply = ServerGenericReplyType.Failure;
          if (self.game.Data.PbemGameOver == 1)
          {
            self.e.ServerTextBuffer += "Marking finished game as viewed...\r\n";
            self.url += "mark_game_viewed.php?";
            self.url = self.url + "InstanceID=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.Data.PbemGameID));
            self.url = self.url + "&Viewed=" + HttpUtility.UrlEncode("1");
            self.e.ServerUploadFile = (byte[]) null;
          }
          else
          {
            self.e.ServerTextBuffer += "Uploading turn...\r\n";
            self.url += "upload_instance.php?";
            self.url = self.url + "GameInstanceID=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.Data.PbemGameID));
            self.url = self.game.Data.RegimeObj[self.game.Data.Turn].PbemPlayer != 1 ? self.url + "&PlayerToUserName=" + HttpUtility.UrlEncode(self.game.Data.PbemPlayer2) : self.url + "&PlayerToUserName=" + HttpUtility.UrlEncode(self.game.Data.PbemPlayer1);
            self.url = self.url + "&TurnNo=" + HttpUtility.UrlEncode(Conversions.ToString(self.game.Data.Round));
            self.url = !(self.game.Data.LastWinner > -1 | self.game.Data.PbemDrawGame == 1) ? self.url + "&GameOver=" + HttpUtility.UrlEncode("0") : self.url + "&GameOver=" + HttpUtility.UrlEncode("1");
            self.url = self.url + "&AccepterArmyName=" + HttpUtility.UrlEncode("n/a");
            self.url = self.url + "&FileSize=" + HttpUtility.UrlEncode(Strings.Trim(Conversion.Str( (self.game.EditObj.ServerUploadFile.GetUpperBound(0) + 1))));
            if (self.game.Data.LastWinner > -1 | self.game.Data.PbemDrawGame == 1)
            {
              if (self.game.Data.PbemDrawGame == 1)
              {
                self.url = self.url + "&Player1Losses=" + HttpUtility.UrlEncode("1");
                self.url = self.url + "&Player2Losses=" + HttpUtility.UrlEncode("1");
              }
              else if (self.game.Data.RegimeObj[self.game.Data.LastWinner].PbemPlayer == 1)
              {
                self.url = self.url + "&Player1Losses=" + HttpUtility.UrlEncode("0");
                self.url = self.url + "&Player2Losses=" + HttpUtility.UrlEncode("1");
              }
              else
              {
                self.url = self.url + "&Player1Losses=" + HttpUtility.UrlEncode("1");
                self.url = self.url + "&Player2Losses=" + HttpUtility.UrlEncode("0");
              }
              self.url = self.url + "&Resigned=" + HttpUtility.UrlEncode("0");
              self.url = self.url + "&ScorePlayer1" + HttpUtility.UrlEncode("0");
            }
          }
        }
      }
      if (!self.e.ServerOrderCancel)
        return;
      self.url = self.baseurl;
      self.e.ServerCommandMaxStep = 0;
    }

    pub fn ReadReceive()
    {
      self.WriteLog("READING");
      if (Information.IsNothing( self.receive))
      {
        self.receive = "99 ***ZERO LENGTH STRING RETURNED BY PBEM++ SERVER***";
        if (self.e.ServerCommand == ServerCommandType.PlayTurn)
          self.receive = "";
      }
      if (self.e.ServerCommand == ServerCommandType.PlayTurn & Operators.CompareString(self.receive, "", false) == 0)
      {
        self.e.ServerStatus = ServerStatusType.Completed;
        self.e.ServerGenericReply = ServerGenericReplyType.Succes;
      }
      else
      {
        strArray1: Vec<String> = self.receive.Split('\n');
        str: String = strArray1[0];
        let mut num1: i32 = !(Strings.AscW(Strings.Left(str, 1)) < 48 | Strings.AscW(Strings.Left(str, 1)) > 57) ?  Math.Round(Conversion.Val(Strings.Left(str, 2))) : 900;
        if (num1 == 99)
          num1 = 900;
        self.e.ServerExactErrorCode = num1;
        let mut upperBound: i32 = strArray1.GetUpperBound(0);
        while (strArray1[upperBound].Length == 0)
          --upperBound;
        self.WriteLog("READING: CODE: " + num1.ToString() + ", MAXLINE: " + upperBound.ToString() + ", SERVERCOMMAND: " + self.e.ServerCommand.ToString() + ", SERVERCOMMANDSTEP: " + self.e.ServerCommandStep.ToString());
        self.e.ServerLostConnect = false;
        if (self.e.ServerCommand == ServerCommandType.Initialize)
        {
          if (self.game.EditObj.PbemSteam & !self.game.EditObj.PbemAlreadyAccount)
          {
            if (self.e.ServerCommandStep == 1)
            {
              if (num1 == 0)
              {
                self.e.ServerAuthReply = ServerAuthReplyType.AuthSucces;
                self.e.ServerSerialReply = ServerSerialReplyType.SerialRegistered;
                self.game.EditObj.PbemSerial = Strings.Mid(strArray1[0], 3);
                self.game.EditObj.Save("editobj.txt");
              }
              else
              {
                self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
                self.e.ServerSerialReply = ServerSerialReplyType.SerialNotRegistered;
              }
            }
            self.e.ServerCommandMaxStep = 1;
          }
          else if (self.e.ServerCommandStep == 1)
          {
            switch (num1)
            {
              case 0:
                self.e.ServerSerialReply = ServerSerialReplyType.SerialRegistered;
                self.e.ServerRegisterUserNameReply = Strings.Mid(strArray1[0], 3);
                break;
              case 5:
                self.e.ServerSerialReply = ServerSerialReplyType.SerialNotRegistered;
                self.e.ServerCommandMaxStep = 1;
                break;
              case 900:
                self.e.ServerSerialReply = ServerSerialReplyType.NoConnection;
                self.e.ServerLostConnect = true;
                self.e.ServerCommandMaxStep = 1;
                break;
              default:
                self.e.ServerSerialReply = !self.game.EditObj.PbemSteam ? ServerSerialReplyType.SerialFailure : ServerSerialReplyType.SerialNotRegistered;
                self.e.ServerCommandMaxStep = 1;
                break;
            }
          }
          else if (self.e.ServerCommandStep == 2)
            self.e.ServerAuthReply = num1 != 0 ? ServerAuthReplyType.AuthFailure : ServerAuthReplyType.AuthSucces;
        }
        if (self.e.ServerCommand == ServerCommandType.Register)
        {
          if (self.game.EditObj.PbemSteam)
          {
            if (self.e.ServerCommandStep == 1)
            {
              if (num1 == 0)
              {
                self.e.ServerRegisterReply = ServerRegisterReplyType.RegisterationSucces;
              }
              else
              {
                self.e.ServerRegisterReply = ServerRegisterReplyType.RegisterationFailure;
                self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
                self.e.ServerCommandMaxStep = 1;
              }
            }
            else if (self.e.ServerCommandStep == 2)
            {
              if (num1 == 0)
              {
                self.e.ServerAuthReply = ServerAuthReplyType.AuthSucces;
                self.game.EditObj.PbemSerial = Strings.Mid(strArray1[0], 3);
                self.game.EditObj.Save("editobj.txt");
              }
              else
                self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
              self.e.ServerCommandMaxStep = 2;
            }
          }
          else if (!self.game.EditObj.PbemSteam)
          {
            if (self.e.ServerCommandStep == 1)
            {
              switch (num1)
              {
                case 0:
                  self.e.ServerSerialReply = ServerSerialReplyType.SerialRegistered;
                  self.e.ServerRegisterUserNameReply = Strings.Mid(strArray1[0], 3);
                  self.e.ServerCommandMaxStep = 1;
                  break;
                case 5:
                  self.e.ServerSerialReply = ServerSerialReplyType.SerialNotRegistered;
                  break;
                default:
                  self.e.ServerSerialReply = ServerSerialReplyType.SerialFailure;
                  self.e.ServerCommandMaxStep = 1;
                  break;
              }
            }
            else if (self.e.ServerCommandStep == 2)
            {
              if (num1 == 0)
              {
                self.e.ServerRegisterReply = ServerRegisterReplyType.RegisterationSucces;
              }
              else
              {
                self.e.ServerRegisterReply = ServerRegisterReplyType.RegisterationFailure;
                self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
                self.e.ServerCommandMaxStep = 1;
              }
            }
            else if (self.e.ServerCommandStep == 3)
              self.e.ServerAuthReply = num1 != 0 ? ServerAuthReplyType.AuthFailure : ServerAuthReplyType.AuthSucces;
          }
        }
        if (self.e.ServerCommand == ServerCommandType.Login && self.e.ServerCommandStep == 1)
        {
          if (num1 == 0)
          {
            self.e.ServerAuthReply = ServerAuthReplyType.AuthSucces;
            if (self.game.EditObj.PbemSteam)
            {
              self.game.EditObj.PbemSerial = Strings.Mid(strArray1[0], 3);
              self.game.EditObj.Save("editobj.txt");
            }
          }
          else
            self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
        }
        if (self.e.ServerCommand == ServerCommandType.Logout && self.e.ServerCommandStep == 1)
        {
          if (num1 == 0)
          {
            self.e.ServerGenericReply = ServerGenericReplyType.Succes;
            self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
            self.e.ServerRunningGameList = (RunningGameClass[]) null;
            self.e.ServerChallengeList = (ChallengeClass[]) null;
          }
          else
            self.e.ServerGenericReply = ServerGenericReplyType.Failure;
        }
        if (self.e.ServerCommand == ServerCommandType.CheckPlayer && self.e.ServerCommandStep == 1)
        {
          if (num1 == 0)
          {
            self.e.ServerGenericReply = ServerGenericReplyType.Succes;
          }
          else
          {
            self.e.ServerGenericReply = ServerGenericReplyType.Failure;
            self.e.ServerCommandMaxStep = 1;
          }
        }
        if (self.e.ServerCommand == ServerCommandType.Refresh)
        {
          if (self.e.ServerCommandStep == 1)
          {
            if (num1 == 0)
            {
              self.e.ServerGenericReply = ServerGenericReplyType.Succes;
              self.e.ServerChallengeList = new ChallengeClass[upperBound - 1 + 1];
              let mut num2: i32 = upperBound;
              for (let mut index: i32 = 1; index <= num2; index += 1)
              {
                self.e.ServerChallengeList[index - 1] = ChallengeClass::new();
                strArray2: Vec<String> = strArray1[index].Split(',');
                self.e.ServerChallengeList[index - 1].challengeID = Conversions.ToInteger(strArray2[0]);
                self.e.ServerChallengeList[index - 1].challengerUserName = Strings.Mid(strArray2[1], 2, Strings.Len(strArray2[1]) - 2);
                self.e.ServerChallengeList[index - 1].challengerSide = Conversions.ToInteger(strArray2[2]);
                self.e.ServerChallengeList[index - 1].firstPlayerSide = Conversions.ToInteger(strArray2[3]);
                self.e.ServerChallengeList[index - 1].gameName = Strings.Mid(strArray2[4], 2, Strings.Len(strArray2[4]) - 2);
                self.e.ServerChallengeList[index - 1].dateIssued = Conversions.ToDate(strArray2[5]);
                self.e.ServerChallengeList[index - 1].challengePrivate = Conversions.ToInteger(strArray2[6]);
                self.e.ServerChallengeList[index - 1].productID = Conversions.ToInteger(strArray2[7]);
                self.e.ServerChallengeList[index - 1].armyName = Strings.Mid(strArray2[8], 2, Strings.Len(strArray2[8]) - 2);
                self.e.ServerChallengeList[index - 1].armyPoints = Conversions.ToInteger(strArray2[9]);
                self.e.ServerChallengeList[index - 1].showName = Conversions.ToInteger(strArray2[10]);
                self.e.ServerChallengeList[index - 1].miscData = Strings.Mid(strArray2[11], 1, Strings.Len(strArray2[11]) - 0);
              }
            }
            else
            {
              self.e.ServerGenericReply = ServerGenericReplyType.Failure;
              self.e.ServerCommandMaxStep = 1;
            }
          }
          if (self.e.ServerCommandStep == 2)
          {
            if (num1 == 0)
            {
              self.e.ServerRunningGameList = new RunningGameClass[upperBound - 1 + 1];
              try
              {
                let mut num3: i32 = upperBound;
                for (let mut index: i32 = 1; index <= num3; index += 1)
                {
                  self.WriteLog("READING LINE " + index.ToString());
                  self.e.ServerRunningGameList[index - 1] = RunningGameClass::new();
                  strArray3: Vec<String> = strArray1[index].Split(',');
                  self.e.ServerRunningGameList[index - 1].gameInstanceID = Conversions.ToInteger(strArray3[0]);
                  self.e.ServerRunningGameList[index - 1].pairedGameID = Conversions.ToInteger(strArray3[1]);
                  self.e.ServerRunningGameList[index - 1].playerToUserName = Strings.Mid(strArray3[2], 2, Strings.Len(strArray3[2]) - 2);
                  self.e.ServerRunningGameList[index - 1].Player1UserName = Strings.Mid(strArray3[3], 2, Strings.Len(strArray3[3]) - 2);
                  self.e.ServerRunningGameList[index - 1].Player2UserName = Strings.Mid(strArray3[4], 2, Strings.Len(strArray3[4]) - 2);
                  self.e.ServerRunningGameList[index - 1].TurnNo = Conversions.ToInteger(strArray3[5]);
                  self.e.ServerRunningGameList[index - 1].GameName = strArray3[6];
                  self.e.ServerRunningGameList[index - 1].DateIssued = Conversions.ToDate(strArray3[7]);
                  self.e.ServerRunningGameList[index - 1].lastUploaded = Conversions.ToDate(strArray3[8]);
                  self.e.ServerRunningGameList[index - 1].GameOver = Conversions.ToBoolean(strArray3[9]);
                  self.e.ServerRunningGameList[index - 1].MiscData = strArray3[15];
                  self.WriteLog("." + index.ToString());
                  if (self.e.ServerRunningGameList[index - 1].GameOver)
                  {
                    self.WriteLog(".." + index.ToString());
                    self.e.ServerRunningGameList[index - 1].Player1Losses = Operators.CompareString(Strings.Left(strArray3[10], 1), "1", false) != 0 ? 0.0f : 1f;
                    self.e.ServerRunningGameList[index - 1].Player2Losses = Operators.CompareString(Strings.Left(strArray3[11], 1), "1", false) != 0 ? 0.0f : 1f;
                    self.e.ServerRunningGameList[index - 1].Viewed = Conversions.ToInteger(strArray3[12]);
                    self.e.ServerRunningGameList[index - 1].lastTurnUploaded = Conversions.ToDate(strArray3[16]);
                  }
                  self.WriteLog("..." + index.ToString());
                }
              }
              catch (Exception ex)
              {
                ProjectData.SetProjectError(ex);
                self.WriteLog("Error: " + ex.Message);
                ProjectData.ClearProjectError();
              }
              self.e.ServerGenericReply = ServerGenericReplyType.Succes;
            }
            else
              self.e.ServerGenericReply = ServerGenericReplyType.Failure;
            self.WriteLog("....");
          }
        }
        if (self.e.ServerCommand == ServerCommandType.Challenge && self.e.ServerCommandStep == 1)
          self.e.ServerGenericReply = num1 != 0 ? ServerGenericReplyType.Failure : ServerGenericReplyType.Succes;
        if (self.e.ServerCommand == ServerCommandType.CancelChallenge && self.e.ServerCommandStep == 1)
          self.e.ServerGenericReply = num1 != 0 ? ServerGenericReplyType.Failure : ServerGenericReplyType.Succes;
        if (self.e.ServerCommand == ServerCommandType.UploadInstance)
        {
          let mut num4: i32 = 0;
          if (self.e.ServerCommandStep + num4 == 1)
          {
            switch (num1)
            {
              case 0:
                self.e.ServerSerialReply = ServerSerialReplyType.SerialRegistered;
                self.e.ServerRegisterUserNameReply = Strings.Mid(strArray1[0], 3);
                break;
              case 5:
                self.e.ServerSerialReply = ServerSerialReplyType.SerialNotRegistered;
                self.e.ServerCommandMaxStep = 1;
                break;
              case 900:
                self.e.ServerSerialReply = ServerSerialReplyType.NoConnection;
                self.e.ServerLostConnect = true;
                self.e.ServerCommandMaxStep = 1;
                break;
              default:
                self.e.ServerSerialReply = ServerSerialReplyType.SerialFailure;
                self.e.ServerCommandMaxStep = 1;
                break;
            }
          }
          else if (self.e.ServerCommandStep + num4 == 2)
          {
            if (num1 == 0)
            {
              self.e.ServerAuthReply = ServerAuthReplyType.AuthSucces;
            }
            else
            {
              self.e.ServerAuthReply = ServerAuthReplyType.AuthFailure;
              self.e.ServerCommandMaxStep = 1;
            }
          }
          else if (self.e.ServerCommandStep + num4 == 3)
          {
            self.e.ServerGenericReply = num1 != 0 ? ServerGenericReplyType.Failure : ServerGenericReplyType.Succes;
            if (self.game.EditObj.PbemSteam)
              self.e.ServerCommandMaxStep = 2;
          }
        }
        if (self.e.ServerCommand == ServerCommandType.Claim && self.e.ServerCommandStep == 1)
          self.e.ServerGenericReply = num1 != 0 ? ServerGenericReplyType.Failure : ServerGenericReplyType.Succes;
        if (self.e.ServerCommand == ServerCommandType.AcceptChallenge && self.e.ServerCommandStep == 1)
          self.e.ServerGenericReply = num1 != 0 ? ServerGenericReplyType.Failure : ServerGenericReplyType.Succes;
        if (self.e.ServerCommand == ServerCommandType.PlayTurn && self.e.ServerCommandStep == 1)
          self.e.ServerGenericReply = num1 != 0 ? ServerGenericReplyType.Failure : ServerGenericReplyType.Succes;
        self.WriteLog("END READING. GENERICSERVERREPLY=" + self.e.ServerGenericReply.ToString());
      }
    }

    pub fn WriteLog(s: String)
    {
      if (Strings.InStr(s, "SEND:") > 0)
      {
        try
        {
          if (Strings.InStr(s, "serial_num=") <= 0)
            ;
          if (Strings.InStr(s, "password") <= 0)
            ;
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
      }
      StreamWriter streamWriter = System.IO.File.AppendText(self.game.AppPath + "logs/ServerLog.txt");
      streamWriter.WriteLine(DateTime.Now.ToString());
      streamWriter.WriteLine(s);
      streamWriter.Close();
    }

     class WebRequestState
    {
      pub WebRequest Request;
      pub WebResponse Response;
      pub Stream RespStream;
      pub byte[] buf;
      pub headerRead: bool;
      pub completed: bool;
      pub tempreceive: String;
      pub byte[] upload;
      pub long uploaddone;
      pub e: EditClass;
      pub responseStarted: bool;
      pub DateTime lastRead;

      pub WebRequestState(WebRequest newRequest)
      {
        self.buf = new byte[131073];
        self.upload = new byte[4194305];
        self.Request = newRequest;
        self.responseStarted = false;
      }
    }
  }
}
