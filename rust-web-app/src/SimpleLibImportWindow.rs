// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleLibImportWindow
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleLibImportWindow : WindowClass
  {
     TData: DataClass;
     but1id: i32;
     but2id: i32;
     LibListId: i32;
     ListClass LibListObj;
     LibId: i32;
     ImpId: i32;
     but2idb: i32;
     textid: i32;
     switchid: i32;
     bool[] import;
     impCount: i32;
     OpListId: i32;
     ListClass OpListObj;
     OpId: i32;
     ChangeId: i32;
     Op2ListId: i32;
     ListClass Op2ListObj;
     Op2Id: i32;
     Change2Id: i32;
     Op3ListId: i32;
     ListClass Op3ListObj;
     Op3Id: i32;
     Change3Id: i32;
     int[] subReg;
     int[] subPpl;
     int[] subLib;
     int[] subLibAtStart;
     bool[] regActive;
     bool[] regSubObligatoire;
     bool quitNow;

    pub SimpleLibImportWindow( tGame: GameClass)
      : base( tGame, 1024, 768, 9)
    {
      self.import = new bool[2];
      self.subReg = new int[2];
      self.subPpl = new int[2];
      self.subLib = new int[2];
      self.subLibAtStart = new int[2];
      self.regActive = new bool[1];
      self.regSubObligatoire = new bool[1];
      self.game.FormRef.Cursor = Cursors.WaitCursor;
      self.game.HandyFunctionsObj.LoadLibrary( self.TData);
      self.game.FormRef.Cursor = Cursors.Default;
      self.import = new bool[self.TData.LibraryCounter + 1];
      self.subReg = new int[self.TData.RegimeCounter + 1];
      self.subPpl = new int[self.TData.PeopleCounter + 1];
      self.subLib = new int[self.TData.LibraryCounter + 1];
      self.subLibAtStart = new int[self.TData.LibraryCounter + 1];
      let mut regimeCounter1: i32 = self.TData.RegimeCounter;
      for (let mut index1: i32 = 0; index1 <= regimeCounter1; index1 += 1)
      {
        self.subReg[index1] = -1;
        if (self.TData.RegimeObj[index1].libId.libSlot > -1)
        {
          let mut regimeCounter2: i32 = self.game.Data.RegimeCounter;
          for (let mut index2: i32 = 0; index2 <= regimeCounter2; index2 += 1)
          {
            if (self.game.Data.RegimeObj[index2].libId.libSlot > -1 && Operators.CompareString(self.TData.LibraryObj[self.TData.RegimeObj[index1].libId.libSlot].name, self.game.Data.LibraryObj[self.game.Data.RegimeObj[index2].libId.libSlot].name, false) == 0 && self.TData.RegimeObj[index1].id == self.game.Data.RegimeObj[index2].libId.id)
              self.subReg[index1] = index2;
          }
        }
      }
      let mut peopleCounter1: i32 = self.TData.PeopleCounter;
      for (let mut index3: i32 = 0; index3 <= peopleCounter1; index3 += 1)
      {
        self.subPpl[index3] = -1;
        if (self.TData.PeopleObj[index3].LibId.libSlot > -1)
        {
          let mut peopleCounter2: i32 = self.game.Data.PeopleCounter;
          for (let mut index4: i32 = 0; index4 <= peopleCounter2; index4 += 1)
          {
            if (self.game.Data.PeopleObj[index4].LibId.libSlot > -1 && Operators.CompareString(self.TData.LibraryObj[self.TData.PeopleObj[index3].LibId.libSlot].name, self.game.Data.LibraryObj[self.game.Data.PeopleObj[index4].LibId.libSlot].name, false) == 0 && self.TData.PeopleObj[index3].id == self.game.Data.PeopleObj[index4].LibId.id)
              self.subPpl[index3] = index4;
          }
        }
      }
      let mut libraryCounter1: i32 = self.TData.LibraryCounter;
      for (let mut index5: i32 = 0; index5 <= libraryCounter1; index5 += 1)
      {
        self.subLib[index5] = -1;
        self.subLibAtStart[index5] = -1;
        let mut libraryCounter2: i32 = self.game.Data.LibraryCounter;
        for (let mut index6: i32 = 0; index6 <= libraryCounter2; index6 += 1)
        {
          if (Operators.CompareString(self.game.Data.LibraryObj[index6].name, self.TData.LibraryObj[index5].name, false) == 0)
          {
            self.subLib[index5] = index6;
            self.subLibAtStart[index5] = index6;
          }
        }
      }
      self.OpId = -1;
      self.Op2Id = -1;
      self.Op3Id = -1;
      self.game.EditObj.TempRegimeSlot = -1;
      self.game.EditObj.TempPeopleSlot = -1;
      if (Operators.CompareString(Strings.Trim(Strings.LCase(self.TData.RuleSetName)), Strings.Trim(Strings.LCase(self.game.Data.RuleSetName)), false) != 0)
      {
        if (Interaction.MsgBox( ("You are attempting to load a library defined with ruleset '" + self.TData.RuleSetName + "'. It is not the same as our currents scenario ruleset '" + self.game.Data.RuleSetName + "'. Combining libraries and simple editor masterfiles from different rulesets can cause problems. Proceed at own risk. Do you want to proceed?"), MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
          self.quitNow = true;
      }
      self.DoStuff();
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!self.quitNow)
        return windowReturnClass;
      self.TData = (DataClass) null;
      windowReturnClass.AddCommand(6, 0);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn DoRefresh()
    {
      if (self.game.EditObj.TempRegimeSlot != -1)
      {
        if (self.game.EditObj.TempRegimeSlot <= -2)
          self.game.EditObj.TempRegimeSlot = -1;
        self.subReg[self.OpId] = self.game.EditObj.TempRegimeSlot;
        self.game.EditObj.TempRegimeSlot = -1;
      }
      if (self.game.EditObj.TempPeopleSlot != -1)
      {
        if (self.game.EditObj.TempPeopleSlot <= -2)
          self.game.EditObj.TempPeopleSlot = -1;
        self.subPpl[self.Op2Id] = self.game.EditObj.TempPeopleSlot;
        self.game.EditObj.TempPeopleSlot = -1;
      }
      self.DoStuff();
    }

    pub fn DoStuff()
    {
      self.NewBackGroundAndClearAll(1024, 768, 9);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.HighQuality;
      graphics.InterpolationMode = InterpolationMode.HighQualityBicubic;
      DrawMod.DrawMessFrameSimplePopup( self.OwnBitmap,  graphics, 0, 0, 1024, 768, "Loading libs from: '" + self.game.EditObj.TempFileName.Replace(self.game.AppPath, "") + "'");
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      if (self.quitNow)
        return;
      if (self.but1id > 0)
        self.RemoveSubPart(self.but1id);
      if (self.but2id > 0)
        self.RemoveSubPart(self.but2id);
      if (self.but2idb > 0)
        self.RemoveSubPart(self.but2idb);
      if (self.OpListId > 0)
        self.RemoveSubPart(self.OpListId);
      if (self.ChangeId > 0)
        self.RemoveSubPart(self.ChangeId);
      if (self.Op2ListId > 0)
        self.RemoveSubPart(self.Op2ListId);
      if (self.Change2Id > 0)
        self.RemoveSubPart(self.Change2Id);
      if (self.Op3ListId > 0)
        self.RemoveSubPart(self.Op3ListId);
      if (self.Change3Id > 0)
        self.RemoveSubPart(self.Change3Id);
      if (self.LibListId > 0)
        self.RemoveSubPart(self.LibListId);
      self.LibListObj = ListClass::new();
      let mut num1: i32 = -1;
      let mut num2: i32 = -1;
      let mut libraryCounter1: i32 = self.TData.LibraryCounter;
      for (let mut index1: i32 = 0; index1 <= libraryCounter1; index1 += 1)
      {
        bool flag1 = true;
        bool flag2 = false;
        bool flag3 = false;
        bool flag4 = false;
        bool flag5 = false;
        bool flag6 = false;
        let mut sfTypeCounter: i32 = self.TData.SFTypeCounter;
        for (let mut index2: i32 = 0; index2 <= sfTypeCounter; index2 += 1)
        {
          if (self.TData.SFTypeObj[index2].LibId.libSlot == index1)
            flag2 = true;
        }
        let mut actionCardCounter: i32 = self.TData.ActionCardCounter;
        for (let mut index3: i32 = 0; index3 <= actionCardCounter; index3 += 1)
        {
          if (self.TData.ActionCardObj[index3].LibId.libSlot == index1)
            flag3 = true;
        }
        let mut historicalUnitCounter1: i32 = self.TData.HistoricalUnitCounter;
        for (let mut index4: i32 = 0; index4 <= historicalUnitCounter1; index4 += 1)
        {
          if (self.TData.HistoricalUnitObj[index4].LibId.libSlot == index1 && self.TData.HistoricalUnitObj[index4].CommanderName.Length < 1)
            flag4 = true;
        }
        let mut historicalUnitCounter2: i32 = self.TData.HistoricalUnitCounter;
        for (let mut index5: i32 = 0; index5 <= historicalUnitCounter2; index5 += 1)
        {
          if (self.TData.HistoricalUnitObj[index5].LibId.libSlot == index1 && self.TData.HistoricalUnitObj[index5].CommanderName.Length > 0)
            flag5 = true;
        }
        let mut eventCounter: i32 = self.TData.EventCounter;
        for (let mut index6: i32 = 0; index6 <= eventCounter; index6 += 1)
        {
          if (self.TData.EventObj[index6].LibId.libSlot == index1)
            flag6 = true;
        }
        if (self.game.EditObj.TempFileType == NewEnums.LibFileType.LoadEvents)
        {
          if (flag2)
            flag1 = false;
          if (flag4)
            flag1 = false;
          if (flag5)
            flag1 = false;
        }
        if (self.game.EditObj.TempFileType == NewEnums.LibFileType.LoadHistoricals)
        {
          if (flag6)
            flag1 = false;
          if (flag2)
            flag1 = false;
          if (flag5)
            flag1 = false;
          if (flag3)
            flag1 = false;
        }
        if (self.game.EditObj.TempFileType == NewEnums.LibFileType.LoadOfficerCards)
        {
          if (flag2)
            flag1 = false;
          if (flag4)
            flag1 = false;
          if (flag5)
            flag1 = false;
        }
        if (self.game.EditObj.TempFileType == NewEnums.LibFileType.LoadOfficers)
        {
          if (flag6)
            flag1 = false;
          if (flag2)
            flag1 = false;
          if (flag4)
            flag1 = false;
          if (flag3)
            flag1 = false;
        }
        if (self.game.EditObj.TempFileType == NewEnums.LibFileType.LoadTroops)
        {
          if (flag6)
            flag1 = false;
          if (flag5)
            flag1 = false;
          if (flag4)
            flag1 = false;
          if (flag3)
            flag1 = false;
        }
        if (flag1)
        {
          num2 += 1;
          self.LibListObj.add(Conversion.Str( index1) + ") " + self.TData.LibraryObj[index1].name, index1);
          if (self.LibId == index1)
            num1 = num2;
        }
      }
      if (num1 == -1)
        self.LibId = -1;
      ListClass libListObj = self.LibListObj;
      let mut tlistselect1: i32 = num1;
      let mut game1: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font1: Font =  null;
       local2: Font =  font1;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 10, 400, tlistselect1, game1, true, "Libraries", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 50, bby: 125, tMarcStyle: true, overruleFont: ( local2));
      self.LibListId = self.AddSubPart( tsubpart1, 50, 125, 400, 208, 0);
      if (self.textid > 0)
        self.RemoveSubPart(self.textid);
      if (self.switchid > 0)
        self.RemoveSubPart(self.switchid);
      if (self.ImpId > 0)
        self.RemoveSubPart(self.ImpId);
      let mut num3: i32 = -1;
      SubPartClass tsubpart2;
      if (self.LibId > -1)
      {
        DependencyClass dependencyClass1 = DependencyClass::new();
        DependencyClass dependencyClass2 = self.game.HandyFunctionsObj.Libraries_CheckDependency( self.TData, self.LibId, true);
        bool flag7 = true;
        let mut libraryCounter2: i32 = self.game.Data.LibraryCounter;
        for (let mut index: i32 = 0; index <= libraryCounter2; index += 1)
        {
          if (Operators.CompareString(self.game.Data.LibraryObj[index].name, self.TData.LibraryObj[self.LibId].name, false) == 0)
          {
            flag7 = false;
            num3 = self.game.Data.LibraryObj[index].version;
          }
        }
        let mut libraryCounter3: i32 = self.game.Data.LibraryCounter;
        for (let mut index: i32 = 0; index <= libraryCounter3; index += 1)
        {
          if (Operators.CompareString(self.game.Data.LibraryObj[index].name, self.TData.LibraryObj[self.LibId].name, false) == 0)
          {
            flag7 = false;
            num3 = self.game.Data.LibraryObj[index].version;
          }
        }
        self.regActive = new bool[self.TData.RegimeCounter + 1];
        self.regSubObligatoire = new bool[self.TData.RegimeCounter + 1];
        let mut num4: i32 = 0;
        let mut regimeCounter1: i32 = self.TData.RegimeCounter;
        for (let mut index7: i32 = 0; index7 <= regimeCounter1; index7 += 1)
        {
          if (self.TData.RegimeObj[index7].libId.libSlot == self.LibId)
            self.regActive[index7] = true;
          let mut historicalUnitCounter: i32 = self.TData.HistoricalUnitCounter;
          for (let mut index8: i32 = 0; index8 <= historicalUnitCounter; index8 += 1)
          {
            if (self.TData.HistoricalUnitObj[index8].LibId.libSlot == self.LibId & self.TData.HistoricalUnitObj[index8].TempRegime == index7 & !self.regActive[index7])
            {
              self.regActive[self.TData.HistoricalUnitObj[index8].TempRegime] = true;
              self.regSubObligatoire[self.TData.HistoricalUnitObj[index8].TempRegime] = true;
            }
          }
          if (self.regActive[index7])
            num4 += 1;
        }
        let mut y1: i32 = 390;
        bool flag8;
        if (num4 > 0)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Substitute regimes in this library by existing ones?", self.game.MarcFont4, 510, y1, Color.White);
          y1 += 30;
          self.OpListObj = ListClass::new();
          let mut num5: i32 = -1;
          let mut num6: i32 = -1;
          let mut regimeCounter2: i32 = self.TData.RegimeCounter;
          for (let mut index: i32 = 0; index <= regimeCounter2; index += 1)
          {
            if (self.regActive[index])
            {
              num6 += 1;
              tvalue: String = "Import this regime";
              if (self.regSubObligatoire[index])
                tvalue = "Not yet substituted";
              if (self.subReg[index] > -1)
                tvalue = "Subst. with " + self.game.Data.RegimeObj[self.subReg[index]].Name;
              else if (Strings.InStr(Strings.LCase(self.TData.RegimeObj[index].Name), "only") > 0 & self.subReg[index] == -1)
                flag8 = true;
              if (self.game.Data.Product == 6 & self.subReg[index] == -1)
                flag8 = true;
              self.OpListObj.add(Conversion.Str( index) + ") " + self.TData.RegimeObj[index].Name, index, tvalue);
              if (self.OpId == -1)
                self.OpId = index;
              if (self.OpId == index)
                num5 = num6;
            }
          }
          ListClass opListObj = self.OpListObj;
          let mut tlistselect2: i32 = num5;
          let mut game2: GameClass = self.game;
           local3: Bitmap =  self.OwnBitmap;
          let mut bby: i32 = y1;
          font2: Font =  null;
           local4: Font =  font2;
          let mut tsubpart3: SubPartClass =  new ListSubPartClass(opListObj, 5, 350, tlistselect2, game2, true, "Regimes", false, tShowPair: true, tValueWidth: 225, tdotopandbottom: false, tbackbitmap: ( local3), bbx: 500, bby: bby, tMarcStyle: true, overruleFont: ( local4));
          self.OpListId = self.AddSubPart( tsubpart3, 500, y1, 350, 96, 0);
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change", 130, tBackbitmap: ( self.OwnBitmap), bbx: 870, bby: (y1 + 10), theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.ChangeId = self.AddSubPart( tsubpart4, 870, y1 + 10, 130, 45, 1);
        }
        let mut num7: i32 = 0;
        let mut peopleCounter1: i32 = self.TData.PeopleCounter;
        for (let mut index: i32 = 0; index <= peopleCounter1; index += 1)
        {
          if (self.TData.PeopleObj[index].LibId.libSlot == self.LibId)
            num7 += 1;
        }
        let mut y2: i32 = y1 + 110;
        bool flag9;
        if (num7 > 0)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Substitute peoples in this library by existing ones?", self.game.MarcFont4, 510, y2, Color.White);
          y2 += 30;
          self.Op2ListObj = ListClass::new();
          let mut num8: i32 = -1;
          let mut num9: i32 = -1;
          let mut peopleCounter2: i32 = self.TData.PeopleCounter;
          for (let mut index: i32 = 0; index <= peopleCounter2; index += 1)
          {
            if (self.TData.PeopleObj[index].LibId.libSlot == self.LibId)
            {
              num9 += 1;
              tvalue: String = "Import this people";
              if (self.subPpl[index] > -1)
                tvalue = "Subst. with " + self.game.Data.PeopleObj[self.subPpl[index]].Name;
              else if (Strings.InStr(Strings.LCase(self.TData.PeopleObj[index].Name), "only") > 0 & self.subPpl[index] == -1)
                flag9 = true;
              self.Op2ListObj.add(Conversion.Str( index) + ") " + self.TData.PeopleObj[index].Name, index, tvalue);
              if (self.Op2Id == -1)
                self.Op2Id = index;
              if (self.Op2Id == index)
                num8 = num9;
            }
          }
          ListClass op2ListObj = self.Op2ListObj;
          let mut tlistselect3: i32 = num8;
          let mut game3: GameClass = self.game;
           local5: Bitmap =  self.OwnBitmap;
          let mut bby: i32 = y2;
          font3: Font =  null;
           local6: Font =  font3;
          let mut tsubpart5: SubPartClass =  new ListSubPartClass(op2ListObj, 5, 350, tlistselect3, game3, true, "Peoples", false, tShowPair: true, tValueWidth: 225, tdotopandbottom: false, tbackbitmap: ( local5), bbx: 500, bby: bby, tMarcStyle: true, overruleFont: ( local6));
          self.Op2ListId = self.AddSubPart( tsubpart5, 500, y2, 350, 128, 0);
          let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Change", 130, tBackbitmap: ( self.OwnBitmap), bbx: 870, bby: (y2 + 10), theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.Change2Id = self.AddSubPart( tsubpart6, 870, y2 + 10, 130, 45, 1);
        }
        DrawMod.DrawTextColouredMarc( graphics, "Substitute existing library", self.game.MarcFont4, 50, 330, Color.White);
        let mut num10: i32 = y2 + 30;
        self.Op3ListObj = ListClass::new();
        let mut num11: i32 = -1;
        let mut num12: i32 = -1;
        bool flag10 = true;
        let mut libraryCounter4: i32 = self.game.Data.LibraryCounter;
        for (let mut index: i32 = 0; index <= libraryCounter4; index += 1)
        {
          num12 += 1;
          tvalue: String = "-";
          if (self.subLib[self.LibId] == index)
          {
            tvalue = "Replace this lib";
            if (self.subLibAtStart[self.LibId] > -1)
              flag10 = false;
          }
          self.Op3ListObj.add(Conversion.Str( index) + ") " + self.game.Data.LibraryObj[index].name, index, tvalue);
          if (self.Op3Id == index)
            num11 = num12;
          if (self.Op3Id == -1 & self.subLib[self.LibId] == index)
          {
            num11 = num12;
            self.Op3Id = index;
          }
        }
        ListClass op3ListObj = self.Op3ListObj;
        let mut tlistselect4: i32 = num11;
        let mut game4: GameClass = self.game;
         local7: Bitmap =  self.OwnBitmap;
        font4: Font =  null;
         local8: Font =  font4;
        let mut tsubpart7: SubPartClass =  new ListSubPartClass(op3ListObj, 13, 420, tlistselect4, game4, true, "Library Replacement", false, tShowPair: true, tValueWidth: 125, tdotopandbottom: false, tbackbitmap: ( local7), bbx: 50, bby: 360, tMarcStyle: true, overruleFont: ( local8));
        self.Op3ListId = self.AddSubPart( tsubpart7, 50, 360, 420, 256, 0);
        tsubpart2 =  new TextButtonPartClass("Change", 130, tBackbitmap: ( self.OwnBitmap), bbx: 50, bby: 620, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.Change3Id = self.AddSubPart( tsubpart2, 50, 620, 130, 45, 1);
        DrawMod.DrawTextColouredMarc( graphics, self.TData.LibraryObj[self.LibId].name, self.game.MarcFont1, 510, 125, Color.White);
        if (dependencyClass2.ok)
        {
          let mut regimeCounter3: i32 = self.TData.RegimeCounter;
          for (let mut index: i32 = 0; index <= regimeCounter3; index += 1)
          {
            if (self.regActive[index] & self.regSubObligatoire[index] & self.subReg[index] == -1)
            {
              dependencyClass2.ok = false;
              dependencyClass2.text = "You must substitute the regimes in the substitute list for this import.";
            }
          }
        }
        if (!dependencyClass2.ok)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Dependency problem", self.game.MarcFont3, 510, 170, Color.Salmon);
          tsubpart2 =  new TextAreaClass2(self.game, 500, 3, self.game.MarcFont4, "Dependency problem details:\r\n" + dependencyClass2.text, 27,  self.OwnBitmap, 500, 245, true, true);
          self.textid = self.AddSubPart( tsubpart2, 500, 245, 500, 108, 0);
        }
        else
        {
          if (flag7)
          {
            tsubpart2 =  new MarcRadioPartClass(0, self.import[self.LibId], tBackbitmap: ( self.OwnBitmap), bbx: 510, bby: 160);
            self.ImpId = self.AddSubPart( tsubpart2, 510, 160, 35, 35, 1);
            DrawMod.DrawTextColouredMarc( graphics, "Import this library", self.game.MarcFont3, 550, 165, Color.White);
          }
          else
          {
            if (self.TData.LibraryObj[self.LibId].version > num3)
              DrawMod.DrawTextColouredMarc( graphics, "You already have this library. But this is newer version v" + self.TData.LibraryObj[self.LibId].version.ToString(), self.game.MarcFont3, 510, 170, Color.GreenYellow);
            else if (self.TData.LibraryObj[self.LibId].version == num3)
              DrawMod.DrawTextColouredMarc( graphics, "You already have this library. Seems to be same version v" + self.TData.LibraryObj[self.LibId].version.ToString(), self.game.MarcFont3, 510, 170, Color.Yellow);
            else if (self.TData.LibraryObj[self.LibId].version == num3)
              DrawMod.DrawTextColouredMarc( graphics, "You already have this library. But this is older version v" + self.TData.LibraryObj[self.LibId].version.ToString(), self.game.MarcFont3, 510, 170, Color.Salmon);
            tsubpart2 =  new MarcRadioPartClass(0, self.import[self.LibId], tBackbitmap: ( self.OwnBitmap), bbx: 510, bby: 200);
            self.ImpId = self.AddSubPart( tsubpart2, 510, 200, 35, 35, 1);
            DrawMod.DrawTextColouredMarc( graphics, "Reload this library", self.game.MarcFont3, 550, 205, Color.White);
          }
          let mut num13: i32 = 0;
          if (self.OpListId < 1 & self.Op2ListId < 1)
            num13 += 9;
          tsubpart2 =  new TextAreaClass2(self.game, 500, 3 + num13, self.game.MarcFont4, self.TData.LibraryObj[self.LibId].information, 27,  self.OwnBitmap, 500, 245);
          self.textid = self.AddSubPart( tsubpart2, 500, 245, 500, (4 + num13) * 27, 0);
        }
        if (flag9 | flag8)
        {
          tsubpart2 =  new TextButtonPartClass("Import " + self.impCount.ToString() + " libs", 200, "You cannot import if there are peoples with 'only' in name that have not yet been substituted. Or if there are Regimes that have not yet been substituted.",  self.OwnBitmap, 524, 680, true, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.but2idb = self.AddSubPart( tsubpart2, 524, 680, 200, 45, 1);
        }
        else if (self.impCount > 0)
        {
          tsubpart2 =  new TextButtonPartClass("Import " + self.impCount.ToString() + " libs", 200, tBackbitmap: ( self.OwnBitmap), bbx: 524, bby: 680, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.but2id = self.AddSubPart( tsubpart2, 524, 680, 200, 45, 1);
        }
        else
        {
          tsubpart2 =  new TextButtonPartClass("Import " + self.impCount.ToString() + " libs", 200, "You have to flag a library in order to import it.",  self.OwnBitmap, 524, 680, true, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.but2idb = self.AddSubPart( tsubpart2, 524, 680, 200, 45, 1);
        }
      }
      if (Operators.CompareString(self.TData.RuleSetName, self.game.Data.RuleSetName, false) != 0)
        DrawMod.DrawTextColouredMarcCenter( graphics, "Lib uses: '" + self.TData.RuleSetName + "', is different from scn ruleset: '" + self.game.Data.RuleSetName + "'", self.game.MarcFont3, 512, 70, Color.Salmon);
      else
        DrawMod.DrawTextColouredMarcCenter( graphics, "Library uses same ruleset as your scenario: '" + self.game.Data.RuleSetName + "'", self.game.MarcFont3, 512, 70, Color.GreenYellow);
      tsubpart2 =  new TextButtonPartClass("Cancel", 200, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 680, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.but1id = self.AddSubPart( tsubpart2, 300, 680, 200, 45, 1);
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
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.LibListId)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num2 > -1)
                self.LibId = num2;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.textid)
            {
              self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OpListId)
            {
              let mut num3: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num3 > -1)
                self.OpId = num3;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Op2ListId)
            {
              let mut num4: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num4 > -1)
                self.Op2Id = num4;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Op3ListId)
            {
              let mut num5: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num5 > -1)
              {
                self.Op3Id = num5;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.Change3Id)
              {
                if (self.subLib[self.LibId] == self.subLibAtStart[self.LibId] & self.subLibAtStart[self.LibId] > -1 && Interaction.MsgBox( "Are you sure? This will cause a duplicate library. ", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
                {
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.subLib[self.LibId] = self.subLib[self.LibId] == self.Op3Id ? -1 : self.Op3Id;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.ChangeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 98, -1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.Change2Id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 99, -1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but1id)
              {
                self.TData = (DataClass) null;
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.but2id)
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.HandyFunctionsObj.ActuallyImportLibs( self.TData,  self.import,  self.subLib,  self.subPpl,  self.subReg);
                self.game.FormRef.Cursor = Cursors.Default;
                self.TData = (DataClass) null;
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.ImpId)
              {
                if (!self.import[self.LibId])
                  this += 1.impCount;
                else
                  --self.impCount;
                self.import[self.LibId] = !self.import[self.LibId];
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
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
  }
}
