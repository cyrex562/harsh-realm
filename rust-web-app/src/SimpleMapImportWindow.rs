// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleMapImportWindow
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleMapImportWindow : WindowClass
  {
     TData: DataClass;
     but1id: i32;
     but2id: i32;
     LibListId: i32;
     ListClass LibListObj;
     LibId: i32;
     ImpId: i32;
     but2idb: i32;
     but3id: i32;
     but3idb: i32;
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
     int[] subReg;
     int[] subPpl;
     int[] subLib;
     bool[] usePpl;

    pub SimpleMapImportWindow( tGame: GameClass)
      : base( tGame, 1024, 768, 9)
    {
      self.import = new bool[2];
      self.subReg = new int[2];
      self.subPpl = new int[2];
      self.subLib = new int[2];
      self.usePpl = new bool[2];
      self.game.FormRef.Cursor = Cursors.WaitCursor;
      self.LoadMap();
      self.game.FormRef.Cursor = Cursors.Default;
      self.subPpl = new int[self.TData.PeopleCounter + 1];
      self.usePpl = new bool[self.TData.PeopleCounter + 1];
      let mut peopleCounter1: i32 = self.TData.PeopleCounter;
      for (let mut index1: i32 = 0; index1 <= peopleCounter1; index1 += 1)
      {
        self.subPpl[index1] = -1;
        self.usePpl[index1] = false;
        if (self.TData.PeopleObj[index1].LibId.libSlot > -1)
        {
          let mut peopleCounter2: i32 = self.game.Data.PeopleCounter;
          for (let mut index2: i32 = 0; index2 <= peopleCounter2; index2 += 1)
          {
            if (self.game.Data.PeopleObj[index2].LibId.libSlot > -1)
            {
              if (Operators.CompareString(self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name, self.game.Data.LibraryObj[self.game.Data.PeopleObj[index2].LibId.libSlot].name, false) == 0 && self.TData.PeopleObj[index1].id == self.game.Data.PeopleObj[index2].LibId.id)
                self.subPpl[index1] = index2;
            }
            else if (Operators.CompareString(self.TData.PeopleObj[index1].Name, self.game.Data.PeopleObj[index2].Name, false) == 0)
              self.subPpl[index1] = index2;
          }
        }
        else
        {
          let mut peopleCounter3: i32 = self.game.Data.PeopleCounter;
          for (let mut index3: i32 = 0; index3 <= peopleCounter3; index3 += 1)
          {
            if (self.game.Data.PeopleObj[index3].LibId.libSlot == -1 & self.TData.PeopleObj[index1].LibId.libSlot == -1 && Operators.CompareString(self.TData.PeopleObj[index1].Name, self.game.Data.PeopleObj[index3].Name, false) == 0)
              self.subPpl[index1] = index3;
          }
        }
      }
      self.OpId = -1;
      self.Op2Id = -1;
      self.game.EditObj.TempRegimeSlot = -1;
      self.game.EditObj.TempPeopleSlot = -1;
      self.DoStuff();
    }

    pub fn DoRefresh()
    {
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
      DrawMod.DrawMessFrameSimplePopup( self.OwnBitmap,  graphics, 0, 0, 1024, 768, "Loading map: '" + self.TData.MapName + "'");
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      if (self.but1id > 0)
        self.RemoveSubPart(self.but1id);
      if (self.but2id > 0)
        self.RemoveSubPart(self.but2id);
      if (self.but2idb > 0)
        self.RemoveSubPart(self.but2idb);
      if (self.but3id > 0)
        self.RemoveSubPart(self.but3id);
      if (self.but3idb > 0)
        self.RemoveSubPart(self.but3idb);
      if (self.OpListId > 0)
        self.RemoveSubPart(self.OpListId);
      if (self.ChangeId > 0)
        self.RemoveSubPart(self.ChangeId);
      if (self.Op2ListId > 0)
        self.RemoveSubPart(self.Op2ListId);
      if (self.Change2Id > 0)
        self.RemoveSubPart(self.Change2Id);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Cancel", 200, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 680, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.but1id = self.AddSubPart( tsubpart1, 300, 680, 200, 45, 1);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Load Map", 200, tBackbitmap: ( self.OwnBitmap), bbx: 524, bby: 680, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.but2id = self.AddSubPart( tsubpart2, 524, 680, 200, 45, 1);
      if (self.TData.MapObj[0].MapWidth + 1 < self.game.Data.MapObj[0].MapWidth & self.TData.MapObj[0].MapHeight + 1 < self.game.Data.MapObj[0].MapHeight)
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Insert Map", 200, tBackbitmap: ( self.OwnBitmap), bbx: 748, bby: 680, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.but3id = self.AddSubPart( tsubpart3, 748, 680, 200, 45, 1);
      }
      else
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Insert Map", 200, "Cannot insert since the map we attempt to load is either (almost) wider of higher.",  self.OwnBitmap, 748, 680, true, theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.but3idb = self.AddSubPart( tsubpart4, 748, 680, 200, 45, 0);
      }
      num1: i32;
      let mut y1: i32 = num1 + 70;
      tstring1: String = "Current map: " + self.game.Data.MapObj[0].MapWidth.ToString() + "x" + self.game.Data.MapObj[0].MapHeight.ToString() + " : " + self.game.Data.MapName + " by " + self.game.Data.MapDesigner + ", version " + self.game.Data.MapVersion.ToString();
      DrawMod.DrawTextColouredMarcCenter( graphics, tstring1, self.game.MarcFont4, 512, y1, Color.White);
      let mut y2: i32 = y1 + 19;
      tstring2: String = "Map attempting to load: " + self.TData.MapObj[0].MapWidth.ToString() + "x" + self.TData.MapObj[0].MapHeight.ToString() + " : " + self.TData.MapName + " by " + self.TData.MapDesigner + ", version " + self.TData.MapVersion.ToString();
      DrawMod.DrawTextColouredMarcCenter( graphics, tstring2, self.game.MarcFont4, 512, y2, Color.White);
      let mut y3: i32 = y2 + 25;
      if (self.TData.MapObj[0].MapWidth == self.game.Data.MapObj[0].MapWidth & self.TData.MapObj[0].MapHeight == self.game.Data.MapObj[0].MapHeight)
      {
        if (Operators.CompareString(self.TData.MapName, self.game.Data.MapName, false) == 0)
        {
          if (self.TData.MapVersion > self.game.Data.MapVersion)
            DrawMod.DrawTextColouredMarcCenter( graphics, "You already have map with this name loaded. But this is newer version.", self.game.MarcFont3, 512, y3, Color.GreenYellow);
          else if (self.TData.MapVersion == self.game.Data.MapVersion)
            DrawMod.DrawTextColouredMarcCenter( graphics, "You already have map with this name loaded. Seems to be same version.", self.game.MarcFont3, 512, y3, Color.Yellow);
          else if (self.TData.MapVersion < self.game.Data.MapVersion)
            DrawMod.DrawTextColouredMarcCenter( graphics, "You already have map with this name loaded. Be careful loading, this is an older version.", self.game.MarcFont3, 512, y3, Color.Salmon);
        }
        else
          DrawMod.DrawTextColouredMarcCenter( graphics, "Be carefull. The name of your current map loaded is different from this one.", self.game.MarcFont3, 512, y3, Color.Salmon);
      }
      else if (self.game.Data.MapObj[0].MapWidth > 0 & self.game.Data.MapObj[0].MapHeight > 0)
        DrawMod.DrawTextColouredMarcCenter( graphics, "The size dimensions of the current map and the one you are preparing to load are not exactly the same.", self.game.MarcFont3, 512, y3, Color.Salmon);
      let mut num2: i32 = 0;
      let mut mapWidth: i32 = self.TData.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 = self.TData.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (self.TData.MapObj[0].HexObj[index1, index2].Location > -1)
            self.usePpl[self.TData.LocObj[self.TData.MapObj[0].HexObj[index1, index2].Location].People] = true;
        }
      }
      let mut peopleCounter1: i32 = self.TData.PeopleCounter;
      for (let mut index: i32 = 0; index <= peopleCounter1; index += 1)
      {
        if (self.usePpl[index])
          num2 += 1;
      }
      let mut y4: i32 = y3 + 60;
      if (num2 <= 0)
        return;
      DrawMod.DrawTextColouredMarc( graphics, "Substitute peoples in locations on this map?", self.game.MarcFont4, 110, y4, Color.White);
      let mut y5: i32 = y4 + 30;
      self.Op2ListObj = ListClass::new();
      let mut num3: i32 = -1;
      let mut num4: i32 = -1;
      let mut peopleCounter2: i32 = self.TData.PeopleCounter;
      for (let mut index: i32 = 0; index <= peopleCounter2; index += 1)
      {
        if (self.usePpl[index])
        {
          num4 += 1;
          tvalue: String = "Import this people";
          if (self.subPpl[index] > -1)
            tvalue = "Subst. with " + self.game.Data.PeopleObj[self.subPpl[index]].Name;
          self.Op2ListObj.add(Conversion.Str( index) + ") " + self.TData.PeopleObj[index].Name, index, tvalue);
          if (self.Op2Id == -1)
            self.Op2Id = index;
          if (self.Op2Id == index)
            num3 = num4;
        }
      }
      ListClass op2ListObj = self.Op2ListObj;
      let mut tlistselect: i32 = num3;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      let mut bby: i32 = y5;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart5: SubPartClass =  new ListSubPartClass(op2ListObj, 9, 750, tlistselect, game, true, "Peoples", false, tShowPair: true, tValueWidth: 225, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 100, bby: bby, tMarcStyle: true, overruleFont: ( local2));
      self.Op2ListId = self.AddSubPart( tsubpart5, 100, y5, 750, 192, 0);
      let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Change", 130, tBackbitmap: ( self.OwnBitmap), bbx: 870, bby: (y5 + 10), theight: 45, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.Change2Id = self.AddSubPart( tsubpart6, 870, y5 + 10, 130, 45, 1);
    }

    pub LoadMap: bool()
    {
      tempFileName: String = self.game.EditObj.TempFileName;
      if (!File.Exists(tempFileName))
        return false;
      self.game.HandyFunctionsObj.Unzip(tempFileName);
      self.TData = new DataClass(DontLoadGraphics: true);
      self.TData = DataClass.deserialize(tempFileName);
      self.game.HandyFunctionsObj.ZipFile(tempFileName);
      GC.Collect();
      Application.DoEvents();
      return true;
    }

    pub fn ActuallyImportLibs()
    {
      let mut peopleCounter: i32 = self.TData.PeopleCounter;
      for (let mut index1: i32 = 0; index1 <= peopleCounter; index1 += 1)
      {
        if (self.usePpl[index1] & self.subPpl[index1] == -1)
        {
          bool flag = false;
          let mut index2: i32 = -1;
          if (self.TData.PeopleObj[index1].LibId.libSlot > -1)
            index2 = self.game.Data.FindPeople( self.TData.PeopleObj[index1], self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name);
          if (index2 > -1)
          {
            flag = true;
            self.game.Data.PeopleObj[index2] = self.TData.PeopleObj[index1].Clone();
            this += 1.game.Data.PeopleIdCounter;
            self.game.Data.PeopleObj[index2].id = self.game.Data.PeopleIdCounter;
            self.game.Data.PeopleObj[index2].LibId.id = self.TData.PeopleObj[index1].id;
          }
          if (!flag)
          {
            self.game.Data.AddPeople();
            self.game.Data.PeopleObj[self.game.Data.PeopleCounter] = self.TData.PeopleObj[index1].Clone();
            this += 1.game.Data.PeopleIdCounter;
            self.game.Data.PeopleObj[self.game.Data.PeopleCounter].id = self.game.Data.PeopleIdCounter;
            if (self.TData.PeopleObj[index1].LibId.libSlot > -1)
            {
              num: i32;
              if (self.game.Data.FindLibrary(self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name) == -1)
              {
                self.game.Data.AddLibrary();
                self.game.Data.LibraryObj[self.game.Data.LibraryCounter] = self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].Clone();
                num = self.game.Data.LibraryCounter;
              }
              else
                num = self.game.Data.FindLibrary(self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name);
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.libSlot = num;
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.id = self.TData.PeopleObj[index1].id;
            }
            else
            {
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.libSlot = -1;
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.id = -1;
            }
            self.subPpl[index1] = self.game.Data.PeopleCounter;
          }
        }
      }
      bool flag1 = false;
      if (self.game.Data.MapObj[0].MapWidth > 0 & self.game.Data.MapObj[0].MapHeight > 0)
        flag1 = true;
      data: DataClass = self.game.Data;
      data.MapDesigner = self.TData.MapDesigner;
      data.MapVersion = self.TData.MapVersion;
      data.MapName = self.TData.MapName;
      bool flag2 = false;
      bool flag3 = false;
      bool flag4 = false;
      data.MapObj[0].MapLoop = self.TData.MapObj[0].MapLoop;
      if (flag1 && !(data.MapObj[0].MapWidth == self.TData.MapObj[0].MapWidth & data.MapObj[0].MapHeight == self.TData.MapObj[0].MapHeight))
      {
        if (Interaction.MsgBox( "Map not same size. Reload is cancelled. Are you sure you want to continue", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
          return;
        flag4 = true;
      }
      if (flag1 && Interaction.MsgBox( "Do you want to overwrite the current Victory Posettings: i32 on your current map?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
        flag2 = true;
      if (self.TData.LocTypeCounter > data.LocTypeCounter && Interaction.MsgBox( "Do you want to add extra locationtypes present in the map file (that are not present in the master)?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
        flag3 = true;
      if (flag3 && self.TData.LocTypeCounter > data.LocTypeCounter)
      {
        data.LocTypeObj = (LocationTypeClass[]) Utils.CopyArray((Array) data.LocTypeObj, (Array) new LocationTypeClass[self.TData.LocTypeCounter + 1]);
        let mut locTypeCounter: i32 = self.TData.LocTypeCounter;
        for (let mut index: i32 = 0; index <= locTypeCounter; index += 1)
        {
          if (index > data.LocTypeCounter)
            data.LocTypeObj[index] = self.TData.LocTypeObj[index];
        }
        data.LocTypeCounter = self.TData.LocTypeCounter;
      }
      if (!flag1)
      {
        let mut mapWidth: i32 = data.MapObj[0].MapWidth;
        for (let mut index3: i32 = 0; index3 <= mapWidth; index3 += 1)
        {
          let mut mapHeight: i32 = data.MapObj[0].MapHeight;
          for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
          {
            while (data.MapObj[0].HexObj[index3, index4].UnitCounter > -1)
            {
              dataClass: DataClass = data;
              let mut unit: i32 = data.MapObj[0].HexObj[index3, index4].UnitList[0];
              let mut gameClass: GameClass = (GameClass) null;
               let mut local: GameClass =  gameClass;
              dataClass.RemoveUnit(unit,  local);
            }
            if (data.MapObj[0].HexObj[index3, index4].Location > -1)
              data.RemoveLoc(data.MapObj[0].HexObj[index3, index4].Location);
          }
        }
        data.MapObj[0].MapWidth = self.TData.MapObj[0].MapWidth;
        data.MapObj[0].MapHeight = self.TData.MapObj[0].MapHeight;
        data.MapObj[0].HexObj = new HexClass[self.TData.MapObj[0].MapWidth + 1, self.TData.MapObj[0].MapHeight + 1];
      }
      else
      {
        let mut mapWidth: i32 = data.MapObj[0].MapWidth;
        for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
        {
          let mut mapHeight: i32 = data.MapObj[0].MapHeight;
          for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
          {
            if (data.MapObj[0].HexObj[index5, index6].Location > -1)
              data.RemoveLoc(data.MapObj[0].HexObj[index5, index6].Location);
          }
        }
        if (flag4)
        {
          if (self.TData.MapObj[0].MapWidth < data.MapObj[0].MapWidth)
            self.game.HandyFunctionsObj.RemoveXToMap(data.MapObj[0].MapWidth - self.TData.MapObj[0].MapWidth);
          if (self.TData.MapObj[0].MapWidth > data.MapObj[0].MapWidth)
            self.game.HandyFunctionsObj.AddXToMap(self.TData.MapObj[0].MapWidth - data.MapObj[0].MapWidth);
          if (self.TData.MapObj[0].MapHeight < data.MapObj[0].MapHeight)
            self.game.HandyFunctionsObj.RemoveYToMap(data.MapObj[0].MapHeight - self.TData.MapObj[0].MapHeight);
          if (self.TData.MapObj[0].MapHeight > data.MapObj[0].MapHeight)
            self.game.HandyFunctionsObj.AddYToMap(self.TData.MapObj[0].MapHeight - data.MapObj[0].MapHeight);
        }
      }
      let mut mapWidth1: i32 = self.TData.MapObj[0].MapWidth;
      for (let mut index7: i32 = 0; index7 <= mapWidth1; index7 += 1)
      {
        let mut mapHeight: i32 = self.TData.MapObj[0].MapHeight;
        for (let mut index8: i32 = 0; index8 <= mapHeight; index8 += 1)
        {
          if (!flag1)
            data.MapObj[0].HexObj[index7, index8] = new HexClass(0, self.game.Data.RegimeCounter, self.game.Data.RegimeCounter);
          let mut index9: i32 = 0;
          do
          {
            data.MapObj[0].HexObj[index7, index8].AreaCode[index9] = self.TData.MapObj[0].HexObj[index7, index8].AreaCode[index9];
            index9 += 1;
          }
          while (index9 <= 9);
          let mut index10: i32 = 0;
          do
          {
            data.MapObj[0].HexObj[index7, index8].Bridge[index10] = self.TData.MapObj[0].HexObj[index7, index8].Bridge[index10];
            data.MapObj[0].HexObj[index7, index8].RiverType[index10] = self.TData.MapObj[0].HexObj[index7, index8].RiverType[index10];
            data.MapObj[0].HexObj[index7, index8].RoadType[index10] = self.TData.MapObj[0].HexObj[index7, index8].RoadType[index10];
            index10 += 1;
          }
          while (index10 <= 5);
          data.MapObj[0].HexObj[index7, index8].CardUponConquest = -1;
          data.MapObj[0].HexObj[index7, index8].LabelText1 = self.TData.MapObj[0].HexObj[index7, index8].LabelText1;
          data.MapObj[0].HexObj[index7, index8].LabelText2 = self.TData.MapObj[0].HexObj[index7, index8].LabelText2;
          data.MapObj[0].HexObj[index7, index8].LabelType1 = self.TData.MapObj[0].HexObj[index7, index8].LabelType1;
          data.MapObj[0].HexObj[index7, index8].LabelType2 = self.TData.MapObj[0].HexObj[index7, index8].LabelType2;
          data.MapObj[0].HexObj[index7, index8].LandscapeType = self.TData.MapObj[0].HexObj[index7, index8].LandscapeType;
          data.MapObj[0].HexObj[index7, index8].SmallLabel = self.TData.MapObj[0].HexObj[index7, index8].SmallLabel;
          data.MapObj[0].HexObj[index7, index8].SmallLabelType = self.TData.MapObj[0].HexObj[index7, index8].SmallLabelType;
          data.MapObj[0].HexObj[index7, index8].randomOverrule = self.TData.MapObj[0].HexObj[index7, index8].randomOverrule;
          data.MapObj[0].HexObj[index7, index8].Location = self.TData.MapObj[0].HexObj[index7, index8].Location;
          data.MapObj[0].HexObj[index7, index8].Name = self.TData.MapObj[0].HexObj[index7, index8].Name;
          data.MapObj[0].HexObj[index7, index8].SpecialSprite = self.TData.MapObj[0].HexObj[index7, index8].SpecialSprite;
          data.MapObj[0].HexObj[index7, index8].SpecialType = self.TData.MapObj[0].HexObj[index7, index8].SpecialType;
          data.MapObj[0].HexObj[index7, index8].SpriteNr = self.TData.MapObj[0].HexObj[index7, index8].SpriteNr;
          data.MapObj[0].HexObj[index7, index8].HeightLevel = self.TData.MapObj[0].HexObj[index7, index8].HeightLevel;
          if (!flag2)
            data.MapObj[0].HexObj[index7, index8].VP = self.TData.MapObj[0].HexObj[index7, index8].VP;
        }
      }
      data.LocCounter = self.TData.LocCounter;
      data.LocObj = (LocationClass[]) Utils.CopyArray((Array) data.LocObj, (Array) new LocationClass[data.LocCounter + 1]);
      let mut locCounter1: i32 = self.TData.LocCounter;
      for (let mut index: i32 = 0; index <= locCounter1; index += 1)
      {
        data.LocObj[index] = self.TData.LocObj[index];
        data.LocObj[index].People = self.subPpl[self.TData.LocObj[index].People];
      }
      let mut num1: i32 = 0;
      for (let mut locCounter2: i32 = data.LocCounter; locCounter2 >= 0; locCounter2 += -1)
      {
        if (data.LocObj[locCounter2].Type > data.LocTypeCounter)
        {
          data.RemoveLoc(locCounter2);
          num1 += 1;
        }
      }
      if (num1 > 0)
      {
        let mut num2: i32 =  Interaction.MsgBox( ("Removed " + num1.ToString() + " locations since they were set to a locationType that was not present in master."), Title: ( "Shadow Empire : Planetary Conquest"));
      }
      let mut num3: i32 = 0;
      for (let mut locTypeCounter: i32 = data.LocTypeCounter; locTypeCounter >= 0; locTypeCounter += -1)
      {
        if (data.LocTypeObj[locTypeCounter].SmallGraphic > data.SmallPicCounter)
        {
          data.LocTypeObj[locTypeCounter].SmallGraphic = data.SmallPicCounter;
          num3 += 1;
        }
      }
      if (num3 > 0)
      {
        let mut num4: i32 =  Interaction.MsgBox( ("Replaced " + num3.ToString() + " small graphics from locationTypes because they were not present in master."), Title: ( "Shadow Empire : Planetary Conquest"));
      }
      self.game.Data.LoadGraphics(self.formref);
    }

    pub fn ActuallyImportLibs2()
    {
      let mut peopleCounter: i32 = self.TData.PeopleCounter;
      for (let mut index1: i32 = 0; index1 <= peopleCounter; index1 += 1)
      {
        if (self.usePpl[index1] & self.subPpl[index1] == -1)
        {
          bool flag = false;
          let mut index2: i32 = -1;
          if (self.TData.PeopleObj[index1].LibId.libSlot > -1)
            index2 = self.game.Data.FindPeople( self.TData.PeopleObj[index1], self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name);
          if (index2 > -1)
          {
            flag = true;
            self.game.Data.PeopleObj[index2] = self.TData.PeopleObj[index1].Clone();
            this += 1.game.Data.PeopleIdCounter;
            self.game.Data.PeopleObj[index2].id = self.game.Data.PeopleIdCounter;
            self.game.Data.PeopleObj[index2].LibId.id = self.TData.PeopleObj[index1].id;
          }
          if (!flag)
          {
            self.game.Data.AddPeople();
            self.game.Data.PeopleObj[self.game.Data.PeopleCounter] = self.TData.PeopleObj[index1].Clone();
            this += 1.game.Data.PeopleIdCounter;
            self.game.Data.PeopleObj[self.game.Data.PeopleCounter].id = self.game.Data.PeopleIdCounter;
            if (self.TData.PeopleObj[index1].LibId.libSlot > -1)
            {
              num: i32;
              if (self.game.Data.FindLibrary(self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name) == -1)
              {
                self.game.Data.AddLibrary();
                self.game.Data.LibraryObj[self.game.Data.LibraryCounter] = self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].Clone();
                num = self.game.Data.LibraryCounter;
              }
              else
                num = self.game.Data.FindLibrary(self.TData.LibraryObj[self.TData.PeopleObj[index1].LibId.libSlot].name);
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.libSlot = num;
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.id = self.TData.PeopleObj[index1].id;
            }
            else
            {
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.libSlot = -1;
              self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.id = -1;
            }
            self.subPpl[index1] = self.game.Data.PeopleCounter;
          }
        }
      }
      data: DataClass = self.game.Data;
      let mut integer1: i32 = Conversions.ToInteger(Interaction.InputBox("Give X coordinate to insert 0,0 coordinate of import map on. Make sure it is an EVEN coordinate", "Shadow Empire : Planetary Conquest"));
      let mut integer2: i32 = Conversions.ToInteger(Interaction.InputBox("Give Y coordinate to insert 0,0 coordinate of import map on.", "Shadow Empire : Planetary Conquest"));
      let mut num1: i32 = integer1 + self.TData.MapObj[0].MapWidth;
      let mut num2: i32 = integer2 + self.TData.MapObj[0].MapHeight;
      if ((integer1 + 2) % 2 > 0)
      {
        let mut num3: i32 =  Interaction.MsgBox( "You picked an UNEVEN X Coordinate. Aborting Insert Operation. ", (MsgBoxStyle) Conversions.ToInteger("Shadow Empire : Planetary Conquest"));
      }
      else if (num1 > self.game.Data.MapObj[0].MapWidth | num2 > self.game.Data.MapObj[0].MapHeight)
      {
        let mut num4: i32 =  Interaction.MsgBox( "Invalid insert coordinate. The inserted map will exceed the current map width or height. Aborting Insert Operation. ", (MsgBoxStyle) Conversions.ToInteger("Shadow Empire : Planetary Conquest"));
      }
      else if (Interaction.MsgBox( ("This operation requires you to be sure that the map you are inserting is using the same masterfile/ruleset as the one it is being inserted in. Are you sure to insert this map on the area " + integer1.ToString() + "," + integer2.ToString() + " - " + num1.ToString() + "," + num2.ToString() + " ?"), MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
      {
        let mut num5: i32 =  Interaction.MsgBox( "Aborting Insert Operation", (MsgBoxStyle) Conversions.ToInteger("Shadow Empire : Planetary Conquest"));
      }
      else
      {
        bool flag = false;
        if (Interaction.MsgBox( "Do you want to overwrite the current Victory Posettings: i32 on your current map?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.No)
          flag = true;
        let mut num6: i32 = integer1;
        let mut num7: i32 = num1;
        for (let mut x: i32 = num6; x <= num7; x += 1)
        {
          let mut num8: i32 = integer2;
          let mut num9: i32 = num2;
          for (let mut y: i32 = num8; y <= num9; y += 1)
          {
            let mut index3: i32 = x - integer1;
            let mut index4: i32 = y - integer2;
            if (x == 166 & y == 42)
              x = x;
            let mut index5: i32 = 0;
            do
            {
              data.MapObj[0].HexObj[x, y].Bridge[index5] = self.TData.MapObj[0].HexObj[index3, index4].Bridge[index5];
              data.MapObj[0].HexObj[x, y].RiverType[index5] = self.TData.MapObj[0].HexObj[index3, index4].RiverType[index5];
              data.MapObj[0].HexObj[x, y].RoadType[index5] = self.TData.MapObj[0].HexObj[index3, index4].RoadType[index5];
              index5 += 1;
            }
            while (index5 <= 5);
            data.MapObj[0].HexObj[x, y].CardUponConquest = -1;
            data.MapObj[0].HexObj[x, y].LabelText1 = self.TData.MapObj[0].HexObj[index3, index4].LabelText1;
            data.MapObj[0].HexObj[x, y].LabelText2 = self.TData.MapObj[0].HexObj[index3, index4].LabelText2;
            data.MapObj[0].HexObj[x, y].LabelType1 = self.TData.MapObj[0].HexObj[index3, index4].LabelType1;
            data.MapObj[0].HexObj[x, y].LabelType2 = self.TData.MapObj[0].HexObj[index3, index4].LabelType2;
            data.MapObj[0].HexObj[x, y].LandscapeType = self.TData.MapObj[0].HexObj[index3, index4].LandscapeType;
            data.MapObj[0].HexObj[x, y].SmallLabel = self.TData.MapObj[0].HexObj[index3, index4].SmallLabel;
            data.MapObj[0].HexObj[x, y].SmallLabelType = self.TData.MapObj[0].HexObj[index3, index4].SmallLabelType;
            data.MapObj[0].HexObj[x, y].randomOverrule = self.TData.MapObj[0].HexObj[index3, index4].randomOverrule;
            if (self.TData.MapObj[0].HexObj[index3, index4].Location > -1)
            {
              if (data.MapObj[0].HexObj[x, y].Location > -1)
              {
                self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].Name = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].Name;
                self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].Type = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].Type;
                self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].People = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].People;
                self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].StructuralPts = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].StructuralPts;
              }
              else
              {
                self.game.Data.AddLoc(x, y, 0);
                self.game.Data.LocObj[self.game.Data.LocCounter] = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location];
                self.game.Data.LocObj[self.game.Data.LocCounter].Name = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].Name;
                self.game.Data.LocObj[self.game.Data.LocCounter].Type = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].Type;
                self.game.Data.LocObj[self.game.Data.LocCounter].People = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].People;
                self.game.Data.LocObj[self.game.Data.LocCounter].StructuralPts = self.TData.LocObj[self.TData.MapObj[0].HexObj[index3, index4].Location].StructuralPts;
                self.game.Data.MapObj[0].HexObj[x, y].Location = self.game.Data.LocCounter;
              }
            }
            else if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1)
              self.game.Data.RemoveLoc(data.MapObj[0].HexObj[x, y].Location);
            data.MapObj[0].HexObj[x, y].Name = self.TData.MapObj[0].HexObj[index3, index4].Name;
            data.MapObj[0].HexObj[x, y].SpecialSprite = self.TData.MapObj[0].HexObj[index3, index4].SpecialSprite;
            data.MapObj[0].HexObj[x, y].SpecialType = self.TData.MapObj[0].HexObj[index3, index4].SpecialType;
            data.MapObj[0].HexObj[x, y].SpriteNr = self.TData.MapObj[0].HexObj[index3, index4].SpriteNr;
            data.MapObj[0].HexObj[x, y].HeightLevel = self.TData.MapObj[0].HexObj[index3, index4].HeightLevel;
            if (!flag)
              data.MapObj[0].HexObj[x, y].VP = self.TData.MapObj[0].HexObj[index3, index4].VP;
          }
        }
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
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.Op2ListId)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num2 > -1)
                self.Op2Id = num2;
              self.DoStuff();
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
              self.ActuallyImportLibs();
              self.game.FormRef.Cursor = Cursors.Default;
              let mut num3: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
              self.TData = (DataClass) null;
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 != self.but3id)
              return windowReturnClass;
            self.game.FormRef.Cursor = Cursors.WaitCursor;
            self.ActuallyImportLibs2();
            self.game.FormRef.Cursor = Cursors.Default;
            let mut num4: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
            self.TData = (DataClass) null;
            windowReturnClass.AddCommand(6, 0);
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
