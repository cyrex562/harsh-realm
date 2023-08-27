// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UDSMessageWindowClass
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
  pub class UDSMessageWindowClass : WindowClass
  {
     pageId: i32;
     okId: i32;
     bool noBackground;
     bool exitFound;
     useExitEventNr: i32;
     useExitKey: String;
     useExitValue: String;
     useExitElementSlot: i32;
     UDSPartClass udsPartObj;
     SimpleStringList[] backupUdsKeys;
     int[] backupEvent;
     int[] backupY;
     StringListClass[] backupStringlist;
     backupTempVars: Vec<i32>;
     backupCounter: i32;
     StringListClass[] backupStrl;
     UDSData[] backupUdsData;
     bool currentBlockedForBackup;
     bool exitPopupCommand;
     useExtraHeight: i32;

    pub UDSMessageWindowClass( tGame: GameClass, extraHeight: i32)
      : base( tGame, 1260, 750 + extraHeight, 8)
    {
      self.backupUdsKeys = new SimpleStringList[1];
      self.backupEvent = new int[1];
      self.backupY = new int[1];
      self.backupStringlist = new StringListClass[1];
      self.backupTempVars = new int[400, 1];
      self.backupStrl = new StringListClass[1];
      self.backupUdsData = new UDSData[1];
      self.noBackground = false;
      self.useExtraHeight = extraHeight;
      self.useExitEventNr = -1;
      self.backupCounter = -1;
      self.SetBackup();
      self.ViewMessage();
      self.SetBackup(true);
    }

    pub UDSMessageWindowClass( tGame: GameClass, bool tnoBackground, extraHeight: i32)
      : base( tGame, 1260, 750 + extraHeight, 8)
    {
      self.backupUdsKeys = new SimpleStringList[1];
      self.backupEvent = new int[1];
      self.backupY = new int[1];
      self.backupStringlist = new StringListClass[1];
      self.backupTempVars = new int[400, 1];
      self.backupStrl = new StringListClass[1];
      self.backupUdsData = new UDSData[1];
      self.noBackground = tnoBackground;
      self.backupCounter = -1;
      self.useExtraHeight = extraHeight;
      self.useExitEventNr = -1;
      self.SetBackup();
      self.ViewMessage();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.game.EditObj.TipButton = false;
          self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (self.game.EditObj.TipButton)
            break;
          let mut num: i32 = self.SubPartID[index];
        }
      }
    }

    pub fn ViewMessage(bool dontDrawPage = false)
    {
      if (self.pageId > 0)
      {
        self.RemoveSubPart(self.pageId);
        self.pageId = 0;
      }
      if (self.okId > 0)
      {
        self.RemoveSubPart(self.okId);
        self.okId = 0;
      }
      self.ClearMouse();
      if (!self.noBackground)
      {
        self.NewBackGroundAndClearAll(1260, 750 + self.useExtraHeight, 8);
        Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
        DrawMod.DrawMessFrame( self.OwnBitmap,  g, 0, 0, 1260, 750 + self.useExtraHeight);
        self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      }
      self.exitFound = false;
      UDSData udsData = new UDSData(Strings.LCase(self.game.EditObj.UDSpopupText), false);
      buttontext: String = "exit";
      self.exitPopupCommand = false;
      let mut elementCounter: i32 = udsData.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter; index += 1)
      {
        if (udsData.element[index].type == UDSType.Button && udsData.element[index].hidden)
        {
          self.exitFound = true;
          self.useExitEventNr = udsData.element[index].eventNr;
          self.useExitKey = udsData.element[index].key;
          self.useExitValue = udsData.element[index].value;
          self.useExitElementSlot = index;
          buttontext = Strings.Trim(Strings.UCase(udsData.element[index].texty));
        }
        if (udsData.element[index].type == UDSType.Hidden && Operators.CompareString(udsData.element[index].key.ToLower(), "CLOSEPOPUP".ToLower(), false) == 0 && Conversions.ToDouble(udsData.element[index].value) > 0.0)
          self.exitPopupCommand = true;
      }
      if ((Strings.InStr(Strings.LCase(self.game.EditObj.UDSpopupText), "[type]button[/type]") < 1 | self.exitFound) & !self.noBackground)
      {
        self.udsPartObj = new UDSPartClass(self.game, 1184, 650 + self.useExtraHeight, self.game.EditObj.UDSpopupText,  self.OwnBitmap, 42, 25, noBitmapDraw: true);
        let mut udsPartObj: SubPartClass =  self.udsPartObj;
        let mut num: i32 = self.AddSubPart( udsPartObj, 42, 25, 1184, 650 + self.useExtraHeight, 0);
        self.udsPartObj = (UDSPartClass) udsPartObj;
        self.pageId = num;
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext, 190, "Click to " + buttontext.ToLower() + ".",  self.OwnBitmap, 535, 688 + self.useExtraHeight, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.okId = self.AddSubPart( tsubpart, 535, 688 + self.useExtraHeight, 190, 35, 1);
      }
      else if (self.noBackground)
      {
        self.udsPartObj = new UDSPartClass(self.game, 1280, 700 + self.useExtraHeight, self.game.EditObj.UDSpopupText,  self.OwnBitmap, 0, 25, tNoBackground: true, noBitmapDraw: true);
        let mut udsPartObj: SubPartClass =  self.udsPartObj;
        let mut num: i32 = self.AddSubPart( udsPartObj, 0, 25, 1280, 700 + self.useExtraHeight, 0);
        self.udsPartObj = (UDSPartClass) udsPartObj;
        self.pageId = num;
      }
      else
      {
        self.udsPartObj = new UDSPartClass(self.game, 1184, 700 + self.useExtraHeight, self.game.EditObj.UDSpopupText,  self.OwnBitmap, 42, 25, noBitmapDraw: true);
        let mut udsPartObj: SubPartClass =  self.udsPartObj;
        let mut num: i32 = self.AddSubPart( udsPartObj, 42, 25, 1184, 700 + self.useExtraHeight, 0);
        self.udsPartObj = (UDSPartClass) udsPartObj;
        self.pageId = num;
      }
      if (!(self.pageId > 0 & !self.currentBlockedForBackup) || ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).curY == self.backupY[self.backupCounter] || self.backupEvent[self.backupCounter] == self.game.EditObj.udsLastCalledPopupEventNr)
        return;
      ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).curY = self.backupY[self.backupCounter];
      if (dontDrawPage)
        return;
      ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).MakeBitmap();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          if (self.okId > 0)
          {
            let mut index: i32 = self.SubpartNr(self.okId);
            windowReturnClass1 = self.HandleMouseClick(self.SubPartX[index] + 1, self.SubPartY[index] + 1, 1);
            self.game.EditObj.udsReturnFromPopup = true;
            return windowReturnClass1;
          }
          if (self.pageId > 0)
          {
            UDSData udsData = new UDSData(Strings.LCase(self.game.EditObj.UDSpopupText), false);
            let mut elementCounter: i32 = udsData.elementCounter;
            for (let mut index1: i32 = 0; index1 <= elementCounter; index1 += 1)
            {
              if (udsData.element[index1].type == UDSType.Button && !udsData.element[index1].hidden && Operators.CompareString(Strings.LCase(udsData.element[index1].texty), "ok", false) == 0)
              {
                let mut num1: i32 = udsData.element[index1].x + 20;
                let mut num2: i32 = udsData.element[index1].y + 10;
                let mut index2: i32 = self.SubpartNr(self.pageId);
                windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
                windowReturnClass3: WindowReturnClass = self.HandleMouseClick(self.SubPartX[index2] + num1, self.SubPartY[index2] + num2, 1);
                if (windowReturnClass3.Counter > -1)
                {
                  self.game.EditObj.udsReturnFromPopup = true;
                  return windowReturnClass3;
                }
              }
            }
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass1;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      return self.game.EditObj.WINDOW_DEBUG_MODE ? self.HandleKeyPress(32, false) : windowReturnClass;
    }

    pub fn SetBackup(bool updateCurrent = false)
    {
      if (updateCurrent)
      {
        if (self.currentBlockedForBackup)
          return;
      }
      else
        self.currentBlockedForBackup = false;
      bool flag1;
      if (self.backupCounter == -1)
        flag1 = true;
      if (self.backupCounter > -1 & !updateCurrent)
        flag1 = self.backupEvent[self.backupCounter] != self.game.EditObj.udsLastCalledPopupEventNr;
      let mut udSinputCounter1: i32 = self.game.EditObj.UDSinputCounter;
      bool flag2;
      for (let mut index: i32 = 0; index <= udSinputCounter1; index += 1)
      {
        if (Operators.CompareString(self.game.EditObj.UDSinputKey[index], "BACKUPSKIP", false) == 0 &  Math.Round(Conversion.Val(self.game.EditObj.UDSinputValue[index])) == 1)
        {
          flag2 = true;
          self.currentBlockedForBackup = true;
          flag1 = false;
        }
      }
      if (updateCurrent)
        flag1 = false;
      if (flag1)
      {
        self += 1.backupCounter;
        self.backupUdsKeys = (SimpleStringList[]) Utils.CopyArray((Array) self.backupUdsKeys, (Array) new SimpleStringList[self.backupCounter + 1]);
        self.backupEvent = (int[]) Utils.CopyArray((Array) self.backupEvent, (Array) new int[self.backupCounter + 1]);
        self.backupY = (int[]) Utils.CopyArray((Array) self.backupY, (Array) new int[self.backupCounter + 1]);
        self.backupStringlist = (StringListClass[]) Utils.CopyArray((Array) self.backupStringlist, (Array) new StringListClass[self.backupCounter + 1]);
        self.backupTempVars = (int[,]) Utils.CopyArray((Array) self.backupTempVars, (Array) new int[400, self.backupCounter + 1]);
        self.backupStringlist = (StringListClass[]) Utils.CopyArray((Array) self.backupStringlist, (Array) new StringListClass[self.backupCounter + 1]);
        self.backupUdsData = (UDSData[]) Utils.CopyArray((Array) self.backupUdsData, (Array) new UDSData[self.backupCounter + 1]);
        self.backupEvent[self.backupCounter] = self.game.EditObj.udsLastCalledPopupEventNr;
      }
      if (!flag2)
      {
        self.backupUdsKeys[self.backupCounter] = SimpleStringList::new();
        let mut udSinputCounter2: i32 = self.game.EditObj.UDSinputCounter;
        for (let mut index: i32 = 0; index <= udSinputCounter2; index += 1)
          self.backupUdsKeys[self.backupCounter].Add(self.game.EditObj.UDSinputKey[index],  Math.Round(Conversion.Val(self.game.EditObj.UDSinputValue[index])));
        let mut index1: i32 = 0;
        do
        {
          self.backupTempVars[index1, self.backupCounter] = self.game.Data.TempVar[index1];
          index1 += 1;
        }
        while (index1 <= 399);
        if (self.game.Data.Product == 7)
          self.backupStringlist[self.backupCounter] = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 165, 0, 0))].Clone();
        if (self.pageId > 0)
          self.backupY[self.backupCounter] = ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).curY;
        if (self.pageId > 0)
        {
          UDSPartClass subPart = (UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)];
          self.backupUdsData[self.backupCounter] = subPart.dyn;
          let mut elementCounter: i32 = subPart.dyn.elementCounter;
          for (let mut index2: i32 = 0; index2 <= elementCounter; index2 += 1)
          {
            if (subPart.dyn.element[index2].type == UDSType.Table)
            {
              StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(subPart.dyn.element[index2].texty))];
              self.backupStrl = (StringListClass[]) Utils.CopyArray((Array) self.backupStrl, (Array) new StringListClass[self.backupCounter + 1]);
              self.backupStrl[self.backupCounter] = stringListClass.Clone();
              let mut length: i32 = stringListClass.Length;
              for (let mut index3: i32 = 0; index3 <= length; index3 += 1)
              {
                let mut width: i32 = stringListClass.Width;
                for (let mut index4: i32 = 0; index4 <= width; index4 += 1)
                {
                  self.backupStrl[self.backupCounter].TempDesc[index3, index4] = stringListClass.TempDesc[index3, index4];
                  self.backupStrl[self.backupCounter].TempBmp[index3, index4] = stringListClass.TempBmp[index3, index4];
                }
              }
            }
          }
        }
      }
      self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
    }

    pub HandleMouseUp: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          let mut enr: i32 = self.SubPartList[index].HandleMouseUp(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (enr > 1)
          {
            self.game.EditObj.UDSpopupText = "";
            self.formref.Cursor = Cursors.WaitCursor;
            self.game.EventRelatedObj.DoCheckSpecificEvent(enr);
            self.formref.Cursor = Cursors.Default;
            if (Strings.InStr(Strings.LCase(self.game.EditObj.UDSpopupText), "[key]nobackground[/key]") > 0)
            {
              self.noBackground = true;
              Graphics.FromImage((Image) self.OwnBitmap).Clear(Color.Transparent);
              self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
            }
            else
              self.noBackground = false;
            self.game.EditObj.udsLastCalledPopupEventNr = enr;
            self.SetBackup();
            self.ViewMessage();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (enr > -1)
          {
            if (self.backupCounter > -1)
              self.SetBackup(true);
            windowReturnClass.SetFlag(true);
            self.SubPartFlag[index] = true;
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            num2: i32;
            if (num1 == self.okId)
            {
              if (self.exitPopupCommand)
              {
                self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                self.game.EditObj.udsReturnFromPopup = true;
                self.game.EditObj.udsLastCalledPopupEventNr = -1;
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.backupCounter == 0 && self.backupEvent[0] != self.game.EditObj.udsLastCalledPopupEventNr)
                self += 1.backupCounter;
              if (self.backupCounter > 0)
              {
                --self.backupCounter;
                self.game.EditObj.UDSpopupText = "";
                self.currentBlockedForBackup = false;
                self.formref.Cursor = Cursors.WaitCursor;
                let mut enr: i32 = self.backupEvent[self.backupCounter];
                self.game.EditObj.UDSClearInput();
                let mut counter: i32 = self.backupUdsKeys[self.backupCounter].Counter;
                index2: i32;
                for (index2 = 0; index2 <= counter; index2 += 1)
                  self.game.EditObj.UDSAddInput(self.backupUdsKeys[self.backupCounter].Id[index2], self.backupUdsKeys[self.backupCounter].Weight[index2]);
                if (self.game.Data.Product == 7)
                {
                  self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 165, 0, 0))] = self.backupStringlist[self.backupCounter];
                  num2 = index2;
                }
                let mut index3: i32 = 0;
                do
                {
                  self.game.Data.TempVar[index3] = self.backupTempVars[index3, self.backupCounter];
                  index3 += 1;
                }
                while (index3 <= 399);
                self.game.EventRelatedObj.DoCheckSpecificEvent(enr, skipSettingTempVars: true);
                self.formref.Cursor = Cursors.Default;
                if (self.game.EditObj.UDSpopupText.Length > 1)
                {
                  if (Strings.InStr(Strings.LCase(self.game.EditObj.UDSpopupText), "[key]nobackground[/key]") > 0)
                  {
                    self.noBackground = true;
                    Graphics.FromImage((Image) self.OwnBitmap).Clear(Color.Transparent);
                    self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
                  }
                  else
                    self.noBackground = false;
                  self.ViewMessage(true);
                  self.game.EditObj.udsLastCalledPopupEventNr = enr;
                  if (self.pageId > 0)
                  {
                    UDSPartClass subPart = (UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)];
                    UDSData Expression = self.backupUdsData[self.backupCounter];
                    if (!Information.IsNothing( Expression))
                    {
                      let mut elementCounter: i32 = subPart.dyn.elementCounter;
                      for (let mut index4: i32 = 0; index4 <= elementCounter; index4 += 1)
                      {
                        if (Expression.elementCounter >= index4)
                        {
                          if (subPart.dyn.element[index4].type == Expression.element[index4].type && subPart.dyn.element[index4].x == Expression.element[index4].x & subPart.dyn.element[index4].y == Expression.element[index4].y)
                          {
                            subPart.dyn.element[index4].topRow = Expression.element[index4].topRow;
                            if (subPart.dyn.element[index4].parentElement > -1)
                            {
                              subPart.dyn.element[index4].grayed = Expression.element[index4].grayed;
                              subPart.dyn.element[index4].texty = Expression.element[index4].texty;
                            }
                          }
                          if (subPart.dyn.element[index4].type == UDSType.Table & !Information.IsNothing( self.backupStrl))
                          {
                            StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(subPart.dyn.element[index4].texty))];
                            self.game.Data.StringListObj[ Math.Round(Conversion.Val(subPart.dyn.element[index4].texty))] = self.backupStrl[self.backupCounter];
                          }
                        }
                      }
                    }
                    ((UDSPartClass) self.SubPartList[self.SubpartNr(self.pageId)]).MakeBitmap();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.game.EditObj.udsLastCalledPopupEventNr = enr;
                self.game.EditObj.udsLastCalledPopupEventNr = -1;
                self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
              self.game.EditObj.udsReturnFromPopup = true;
              self.game.EditObj.udsLastCalledPopupEventNr = -1;
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == 9999992)
            {
              if (self.useExitEventNr > 0)
              {
                self.game.EditObj.udsReturnFromPopup = true;
                self.udsPartObj.SetHiddenAndBaseData(self.useExitElementSlot, self.useExitKey, self.useExitValue);
                let mut useExitEventNr: i32 = self.useExitEventNr;
                if (useExitEventNr > 0)
                {
                  self.game.EditObj.UDSpopupText = "";
                  self.formref.Cursor = Cursors.WaitCursor;
                  self.game.EventRelatedObj.DoCheckSpecificEvent(useExitEventNr);
                  self.formref.Cursor = Cursors.Default;
                  if (self.game.EditObj.UDSpopupText.Length > 1)
                  {
                    if (Strings.InStr(Strings.LCase(self.game.EditObj.UDSpopupText), "[key]nobackground[/key]") > 0)
                    {
                      self.noBackground = true;
                      Graphics.FromImage((Image) self.OwnBitmap).Clear(Color.Transparent);
                      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
                    }
                    else
                      self.noBackground = false;
                    self.ViewMessage();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  self.game.EditObj.udsLastCalledPopupEventNr = -1;
                  self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (useExitEventNr == -999)
                {
                  self.game.EditObj.udsLastCalledPopupEventNr = -1;
                  self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.EditObj.udsLastCalledPopupEventNr = -1;
              self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
              self.game.EditObj.udsReturnFromPopup = true;
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.pageId)
            {
              let mut enr1: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (enr1 > 0)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EventRelatedObj.DoCheckSpecificEvent(enr1);
                self.formref.Cursor = Cursors.Default;
                if (self.game.EditObj.UDSpopupText.Length > 1)
                {
                  if (Strings.InStr(Strings.LCase(self.game.EditObj.UDSpopupText), "[key]nobackground[/key]") > 0)
                  {
                    self.noBackground = true;
                    Graphics.FromImage((Image) self.OwnBitmap).Clear(Color.Transparent);
                    self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
                  }
                  else
                    self.noBackground = false;
                  self.game.EditObj.udsLastCalledPopupEventNr = enr1;
                  self.SetBackup();
                  self.ViewMessage();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                self.game.EditObj.udsReturnFromPopup = true;
                self.game.EditObj.udsLastCalledPopupEventNr = -1;
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (enr1 == -999)
              {
                if (Strings.InStr(self.game.EditObj.UDSpopupText.ToLower(), "[key]closepopup[/key]") > 0)
                {
                  self.game.EditObj.udsLastCalledPopupEventNr = -1;
                  self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                  self.game.EditObj.udsReturnFromPopup = true;
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (self.backupCounter > 0)
                {
                  --self.backupCounter;
                  self.game.EditObj.UDSpopupText = "";
                  self.formref.Cursor = Cursors.WaitCursor;
                  let mut enr2: i32 = self.backupEvent[self.backupCounter];
                  self.game.EditObj.UDSClearInput();
                  let mut counter: i32 = self.backupUdsKeys[self.backupCounter].Counter;
                  index5: i32;
                  for (index5 = 0; index5 <= counter; index5 += 1)
                    self.game.EditObj.UDSAddInput(self.backupUdsKeys[self.backupCounter].Id[index5], self.backupUdsKeys[self.backupCounter].Weight[index5]);
                  if (self.game.Data.Product == 7)
                  {
                    self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 165, 0, 0))] = self.backupStringlist[self.backupCounter];
                    num2 = index5;
                  }
                  let mut index6: i32 = 0;
                  do
                  {
                    self.game.Data.TempVar[index6] = self.backupTempVars[index6, self.backupCounter];
                    index6 += 1;
                  }
                  while (index6 <= 399);
                  self.game.EventRelatedObj.DoCheckSpecificEvent(enr2, skipSettingTempVars: true);
                  self.game.EditObj.udsLastCalledPopupEventNr = enr2;
                  self.formref.Cursor = Cursors.Default;
                  if (self.game.EditObj.UDSpopupText.Length > 1)
                  {
                    if (Strings.InStr(Strings.LCase(self.game.EditObj.UDSpopupText), "[key]nobackground[/key]") > 0)
                    {
                      self.noBackground = true;
                      Graphics.FromImage((Image) self.OwnBitmap).Clear(Color.Transparent);
                      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
                    }
                    else
                      self.noBackground = false;
                    self.ViewMessage();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  self.game.EditObj.udsLastCalledPopupEventNr = -1;
                  self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.game.EditObj.udsLastCalledPopupEventNr = -1;
                self.game.EventRelatedObj.SetUDSKey("BACKUPSKIP", 0);
                self.game.EditObj.udsReturnFromPopup = true;
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.backupCounter > -1)
                self.SetBackup(true);
              self.SubPartFlag[index1] = true;
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
