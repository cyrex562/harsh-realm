// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LoadClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class LoadClass
  {
    pub game: GameClass;

    pub LoadClass( tgame: GameClass) => this.game = tgame;

    pub fn Go()
    {
      str1: String = this.game.EditObj.LoadFileName;
      if (!this.game.EditObj.LoadNoNewEdit)
      {
        EditClass.AfterPopUpRefresh afterPopUpRefresh = this.game.EditObj.MyDelegate;
        pbemUserName: String = this.game.EditObj.PbemUserName;
        pbemPassword: String = this.game.EditObj.PbemPassword;
        bool tutMode = this.game.EditObj.TutMode;
        bool buttonLoadMode = this.game.EditObj.ButtonLoadMode;
        pbemSerial: String = this.game.EditObj.PbemSerial;
        this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
        this.game.EditObj.PbemUserName = pbemUserName;
        this.game.EditObj.PbemPassword = pbemPassword;
        this.game.EditObj.PbemSerial = pbemSerial;
        this.game.EditObj.MyDelegate = afterPopUpRefresh;
        this.game.EditObj.TutMode = tutMode;
        this.game.EditObj.ButtonLoadMode = buttonLoadMode;
      }
      this.game.EditObj.LoadNoNewEdit = false;
      if (this.game.Data.UseAI == 1)
      {
        if (Information.IsNothing( this.game.NewAIObj))
          this.game.NewAIObj = new NewAIClass(this.game);
        this.game.NewAIObj.LastRegime = -1;
      }
      this.game.SelectX = -1;
      this.game.SelectY = -1;
      this.game.EditObj.LoadString = "Unzipping".to_owned();
      this.game.HandyFunctionsObj.Unzip(str1);
      this.game.EditObj.LoadString = "Deserializing".to_owned();
      this.game.Data = DataClass.deserialize(str1);
      this.game.EditObj.LoadString = "Zipping".to_owned();
      this.game.HandyFunctionsObj.ZipFile(str1);
      let mut specialSaveMode: i32 =  this.game.Data.specialSaveMode;
      this.game.Data.specialSaveMode = 0;
      if (Strings.Len(this.game.Data.MasterFile) > 0 & this.game.Data.Round == 0 & specialSaveMode < 1)
      {
        this.game.Data.MasterfileReadPeople = false;
        masterFile: String = this.game.Data.MasterFile;
        filename: String = this.game.HandyFunctionsObj.ReturnLongMaster(str1, masterFile);
        this.game.EditObj.LoadString = "Loading Masterfile";
        this.game.HandyFunctionsObj.LoadMasterFile(filename);
        this.game.Data.MasterfileReadPeople = false;
      }
      if (this.game.useModLib)
      {
        if (Strings.InStr(str1, "evlib") < 1 & this.game.EditObj.ButtonLoadMode)
        {
          SimpleStringList simpleStringList = SimpleStringList::new();
          str2: String = "";
          tempFileName: String = this.game.EditObj.TempFileName;
          let mut modlibCounter: i32 =  this.game.modlib_Counter;
          for (let mut index1: i32 =  0; index1 <= modlibCounter; index1 += 1)
          {
            if (this.game.modlib_Flagged[index1])
            {
              if (simpleStringList.FindNr(this.game.modlib_Name[index1].ToLower()) == -1)
              {
                simpleStringList.Add(this.game.modlib_Name[index1].ToLower(), 1);
                this.game.EditObj.TempFileName = this.game.AppPath + this.game.ModScenarioDir + "/" + this.game.modlib_Filename[index1];
                dataClass: DataClass;
                if (File.Exists(this.game.EditObj.TempFileName))
                {
                  try
                  {
                    this.game.HandyFunctionsObj.LoadLibrary( dataClass);
                    this.game.HandyFunctionsObj.modlib_Import( dataClass, 0);
                    bool[] import = new bool[dataClass.LibraryCounter + 1];
                    int[] subreg = new int[dataClass.RegimeCounter + 1];
                    int[] subPpl = new int[dataClass.PeopleCounter + 1];
                    int[] sublib = new int[dataClass.LibraryCounter + 1];
                    int[] numArray = new int[dataClass.LibraryCounter + 1];
                    let mut regimeCounter: i32 =  dataClass.RegimeCounter;
                    for (let mut index2: i32 =  0; index2 <= regimeCounter; index2 += 1)
                      subreg[index2] = -1;
                    let mut peopleCounter: i32 =  dataClass.PeopleCounter;
                    for (let mut index3: i32 =  0; index3 <= peopleCounter; index3 += 1)
                      subPpl[index3] = -1;
                    let mut libraryCounter: i32 =  dataClass.LibraryCounter;
                    for (let mut index4: i32 =  0; index4 <= libraryCounter; index4 += 1)
                    {
                      sublib[index4] = -1;
                      numArray[index4] = -1;
                    }
                    import[0] = true;
                    this.game.EditObj.TempFileType = NewEnums.LibFileType.LoadEvents;
                    this.game.HandyFunctionsObj.ActuallyImportLibs( dataClass,  import,  sublib,  subPpl,  subreg);
                    this.game.EditObj.TempFileType = NewEnums.LibFileType.None;
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    str2 = str2 + "Did not load Mod Library file '" + this.game.modlib_Filename[index1] + "' because it was misconfigured/corrupted.\r\n";
                    ProjectData.ClearProjectError();
                  }
                }
                else
                  str2 = str2 + "Did not load Mod Library file '" + this.game.modlib_Filename[index1] + "' because we were not able to find the file.\r\n";
                this.game.EditObj.TempFileName = "";
                dataClass = (DataClass) null;
              }
              else
                str2 = str2 + "Did not load Mod Library file '" + this.game.modlib_Filename[index1] + "' because another version of '" + this.game.modlib_Name[index1] + "' has already been loaded.\r\n";
            }
          }
          this.game.EditObj.TempFileName = tempFileName;
          if (str2.Length > 1)
          {
            let mut num: i32 =   Interaction.MsgBox( ("MOD LIBRARY PROBLEMS ENCOUNTERED: \r\n" + str2 + "\r\n\r\nYou are advised to to back to Main Menu and reconfigure your Mod Libraries activations in the Mod Library Picker."), Title: ( "Shadow Empire : Planetary Conquest"));
          }
        }
        let mut stringListCounter: i32 =  this.game.Data.StringListCounter;
        for (let mut index5: i32 =  0; index5 <= stringListCounter; index5 += 1)
        {
          if (Operators.CompareString(this.game.Data.StringListObj[index5].Name, "Modify existing table", false) == 0)
          {
            let mut length: i32 =  this.game.Data.StringListObj[index5].Length;
            for (let mut index6: i32 =  0; index6 <= length; index6 += 1)
            {
              let mut num: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[index5].Data[index6, 1]));
              libName: String = this.game.Data.StringListObj[index5].Data[index6, 2];
              str3: String = Conversions.ToString( Math.Round(Conversion.Val(this.game.Data.StringListObj[index5].Data[index6, 3])));
              str4: String = Conversions.ToString( Math.Round(Conversion.Val(this.game.Data.StringListObj[index5].Data[index6, 4])));
              str5: String = Conversions.ToString( Math.Round(Conversion.Val(this.game.Data.StringListObj[index5].Data[index6, 5])));
              if (this.game.Data.FindLibrary(libName) > -1)
              {
                let mut id: i32 =  this.game.EventRelatedObj.CheckStringlistID(libName, Conversions.ToInteger(str3), 0, 0);
                if (id > -1)
                {
                  let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID(id);
                  if (num == 1 && Conversions.ToDouble(str4) > -1.0 & Conversions.ToDouble(str5) > -1.0 & Conversions.ToDouble(str4) <=  this.game.Data.StringListObj[stringListById].Length & Conversions.ToDouble(str5) <=  this.game.Data.StringListObj[stringListById].Width)
                  {
                    if (Information.IsNothing( this.game.Data.StringListObj[index5].Data[index6, 6]))
                      this.game.Data.StringListObj[index5].Data[index6, 6] = "";
                    if (Operators.CompareString(this.game.Data.StringListObj[index5].Data[index6, 6], "*SKIP*", false) != 0)
                      this.game.Data.StringListObj[stringListById].Data[Conversions.ToInteger(str4), Conversions.ToInteger(str5)] = this.game.Data.StringListObj[index5].Data[index6, 6];
                  }
                  if (num == 2 && Conversions.ToDouble(str4) > -1.0 & Conversions.ToDouble(str4) <=  this.game.Data.StringListObj[stringListById].Length)
                  {
                    let mut width: i32 =  this.game.Data.StringListObj[stringListById].Width;
                    for (let mut index7: i32 =  0; index7 <= width; index7 += 1)
                    {
                      if (index7 + 6 <= this.game.Data.StringListObj[index5].Data.GetUpperBound(1))
                      {
                        if (Information.IsNothing( this.game.Data.StringListObj[index5].Data[index6, index7 + 6]))
                          this.game.Data.StringListObj[index5].Data[index6, index7 + 6] = "";
                        if (Operators.CompareString(this.game.Data.StringListObj[index5].Data[index6, index7 + 6], "*SKIP*", false) != 0)
                          this.game.Data.StringListObj[stringListById].Data[Conversions.ToInteger(str4), index7] = this.game.Data.StringListObj[index5].Data[index6, index7 + 6];
                      }
                    }
                  }
                  if (num == 3)
                  {
                    this.game.Data.StringListObj[stringListById].AddRow(this.game.Data.StringListObj[stringListById].Length);
                    str6: String = Conversions.ToString(this.game.Data.StringListObj[stringListById].Length);
                    let mut width: i32 =  this.game.Data.StringListObj[stringListById].Width;
                    for (let mut index8: i32 =  0; index8 <= width; index8 += 1)
                    {
                      if (index8 + 6 <= this.game.Data.StringListObj[index5].Data.GetUpperBound(1))
                      {
                        if (Information.IsNothing( this.game.Data.StringListObj[index5].Data[index6, index8 + 6]))
                          this.game.Data.StringListObj[index5].Data[index6, index8 + 6] = "";
                        if (Operators.CompareString(this.game.Data.StringListObj[index5].Data[index6, index8 + 6], "*SKIP*", false) != 0)
                          this.game.Data.StringListObj[stringListById].Data[Conversions.ToInteger(str6), index8] = this.game.Data.StringListObj[index5].Data[index6, index8 + 6];
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      if ( this.game.Data.RuleVar[344] == 1.0 & this.game.EditObj.HideUnit == 0)
        this.game.EditObj.HideUnit = 2;
      this.game.EditObj.TempValue = new MapMatrix2[this.game.Data.MapCounter + 1];
      this.game.EditObj.TempValue2 = new MapMatrix2[this.game.Data.MapCounter + 1];
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut index: i32 =  0; index <= mapCounter; index += 1)
      {
        this.game.EditObj.TempValue[index] = new MapMatrix2(this.game.Data.MapObj[index].MapWidth, this.game.Data.MapObj[index].MapHeight);
        this.game.EditObj.TempValue2[index] = new MapMatrix2(this.game.Data.MapObj[index].MapWidth, this.game.Data.MapObj[index].MapHeight);
      }
      bool[] flagArray = new bool[this.game.Data.SFCounter + 1];
      let mut unitCounter1: i32 =  this.game.Data.UnitCounter;
      for (let mut index9: i32 =  0; index9 <= unitCounter1; index9 += 1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[index9].SFCount;
        for (let mut index10: i32 =  0; index10 <= sfCount; index10 += 1)
          flagArray[this.game.Data.UnitObj[index9].SFList[index10]] = true;
      }
      for (let mut sfCounter: i32 =  this.game.Data.SFCounter; sfCounter >= 0; sfCounter += -1)
      {
        if (!flagArray[sfCounter])
          this.game.Data.RemoveSF(sfCounter);
      }
      if (this.game.Data.Product == 7)
      {
        let mut unitCounter2: i32 =  this.game.Data.UnitCounter;
        for (let mut index: i32 =  0; index <= unitCounter2; index += 1)
        {
          if (this.game.Data.UnitObj[index].PreDef == -1 && this.game.Data.UnitObj[index].HQ > -1)
          {
            let mut hq: i32 =  this.game.Data.UnitObj[index].HQ;
            if (hq > this.game.Data.UnitCounter)
              this.game.Data.UnitObj[index].HQ = -1;
            else if (this.game.Data.UnitObj[index].Regime != this.game.Data.UnitObj[hq].Regime)
              this.game.Data.UnitObj[index].HQ = -1;
            else if (!this.game.Data.UnitObj[hq].IsHQ)
              this.game.Data.UnitObj[index].HQ = -1;
          }
        }
        if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 81 &  Math.Round(Conversion.Val(this.game.Data.Designer)) <= 83)
        {
          try
          {
            let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 278, 0, 0));
            let mut length: i32 =  this.game.Data.StringListObj[stringListById].Length;
            for (let mut index: i32 =  2; index <= length; index += 1)
            {
              if (Operators.CompareString(Strings.Trim(this.game.Data.StringListObj[stringListById].Data[index - 2, 0]), "[102]", false) == 0 && Operators.CompareString(Strings.Trim(this.game.Data.StringListObj[stringListById].Data[index - 1, 0]), "[103]", false) == 0 && Operators.CompareString(Strings.Trim(this.game.Data.StringListObj[stringListById].Data[index, 0]), "[102]", false) == 0)
              {
                this.game.Data.StringListObj[stringListById].Data[index, 0] = "[103]";
                this.game.Data.StringListObj[stringListById].Data[index + 1, 0] = "[103]";
                break;
              }
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            ProjectData.ClearProjectError();
          }
        }
        try
        {
          let mut unitCounter3: i32 =  this.game.Data.UnitCounter;
          for (let mut unr: i32 =  0; unr <= unitCounter3; unr += 1)
          {
            if (this.game.Data.UnitObj[unr].HQ > -1)
            {
              if (!this.game.Data.UnitObj[this.game.Data.UnitObj[unr].HQ].IsHQ | this.game.Data.UnitObj[this.game.Data.UnitObj[unr].HQ].PreDef > -1)
                this.game.Data.UnitObj[unr].HQ = -1;
              else if (this.game.HandyFunctionsObj.HasUnitAirSF(unr))
              {
                let mut hq: i32 =  this.game.Data.UnitObj[unr].HQ;
                if (hq > -1)
                {
                  let mut historical: i32 =  this.game.Data.UnitObj[hq].Historical;
                  if (historical > -1)
                  {
                    if (this.game.Data.HistoricalUnitObj[historical].Type != 8 && this.game.Data.HistoricalUnitObj[historical].TempVar1 != 1)
                      this.game.Data.UnitObj[unr].HQ = -1;
                  }
                  else
                    this.game.Data.UnitObj[unr].HQ = -1;
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
      }
      if (this.game.Data.Product == 7)
      {
        if ( Math.Round(Conversion.Val(this.game.Data.Designer)) <= 96)
        {
          let mut locCounter: i32 =  this.game.Data.LocCounter;
          for (let mut index: i32 =  0; index <= locCounter; index += 1)
          {
            let mut hq: i32 =  this.game.Data.LocObj[index].HQ;
            let mut regime: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[index].X, this.game.Data.LocObj[index].Y].Regime;
            if (hq > -1)
            {
              if (hq > this.game.Data.UnitCounter)
                this.game.Data.LocObj[index].HQ = -1;
              else if (this.game.Data.UnitObj[hq].PreDef > -1 | this.game.Data.UnitObj[hq].Regime != regime)
                this.game.Data.LocObj[index].HQ = -1;
              else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[hq].Historical].Type < 8)
                this.game.Data.LocObj[index].HQ = -1;
            }
          }
        }
        if (this.game.Data.Round > 1 &  Math.Round(Conversion.Val(this.game.Data.Designer)) < 11)
        {
          let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 278, 0, 0));
          Right: String = "REGKEY.fist=REGKEY.fist+exe(346,REGKEY.fist,TEMP3,100)";
          str7: String = "REGKEY.fist=REGKEY.fist+inv(REGKEY.fist,TEMP3)";
          let mut length: i32 =  this.game.Data.StringListObj[stringListById].Length;
          for (let mut index: i32 =  0; index <= length; index += 1)
          {
            str1 = this.game.Data.StringListObj[stringListById].Data[index, 2];
            if (Operators.CompareString(str1, Right, false) == 0)
              this.game.Data.StringListObj[stringListById].Data[index, 2] = str7;
          }
        }
        let mut stringListById1: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 232, 0, 0));
        if (stringListById1 > -1)
        {
          let mut length1: i32 =  this.game.Data.StringListObj[stringListById1].Length;
          for (let mut index: i32 =  0; index <= length1; index += 1)
          {
            str1 = this.game.Data.StringListObj[stringListById1].Data[index, 3];
            if (Strings.InStr(str1.ToLower(), "engine") > 0)
            {
              String1: String = this.game.Data.StringListObj[stringListById1].Data[index, 4];
              if (Strings.InStr(String1, ".340") > 0)
              {
                str8: String = String1.Replace(".340", ".314");
                this.game.Data.StringListObj[stringListById1].Data[index, 4] = str8;
              }
            }
          }
          let mut stringListById2: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
          for (let mut length2: i32 =  this.game.Data.StringListObj[stringListById2].Length; length2 >= 1; length2 += -1)
          {
            if ( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].Data[length2, 0])) ==  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].Data[length2 - 1, 0])))
              this.game.Data.StringListObj[stringListById2].RemoveRow(length2);
          }
          try
          {
            let mut stringListById3: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 148, 0, 0));
            for (let mut length3: i32 =  this.game.Data.StringListObj[stringListById3].Length; length3 >= 0; length3 += -1)
            {
              let mut num1: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[length3, 0]));
              let mut num2: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[length3, 1]));
              let mut num3: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[length3, 8]));
              if (num2 >= 3241 & num2 <= 3247 & num3 < 1)
              {
                for (let mut length4: i32 =  this.game.Data.StringListObj[stringListById3].Length; length4 >= 0; length4 += -1)
                {
                  let mut num4: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[length4, 0]));
                  let mut num5: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[length4, 1]));
                  if (length3 != length4 & num4 == num1 && num5 >= 3241 & num5 <= 3247)
                  {
                    let mut num6: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[length4, 8]));
                    if (num2 > num5 & num6 < 1)
                      this.game.Data.StringListObj[stringListById3].RemoveRow(length4);
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
          for (let mut unitCounter4: i32 =  this.game.Data.UnitCounter; unitCounter4 >= 0; unitCounter4 += -1)
          {
            if (this.game.Data.UnitObj[unitCounter4].PreDef == -1 && this.game.Data.UnitObj[unitCounter4].X > -1 && this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unitCounter4].X, this.game.Data.UnitObj[unitCounter4].Y].Regime != this.game.Data.UnitObj[unitCounter4].Regime && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unitCounter4].X, this.game.Data.UnitObj[unitCounter4].Y].Regime, this.game.Data.UnitObj[unitCounter4].Regime) && !this.game.HandyFunctionsObj.HasUnitNavySF(unitCounter4) & this.game.Data.RegimeObj[this.game.Data.UnitObj[unitCounter4].Regime].AI && this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unitCounter4].X, this.game.Data.UnitObj[unitCounter4].Y].Regime != -1)
            {
              this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unitCounter4].X, this.game.Data.UnitObj[unitCounter4].Y].RemoveUnitFromList(unitCounter4);
              data: DataClass = this.game.Data;
              let mut nr: i32 =  unitCounter4;
              let mut gameClass: GameClass = (GameClass) null;
               let mut local: GameClass =  gameClass;
              data.RemoveUnit(nr,  local);
            }
          }
        }
      }
      if (this.game.Data.Round > 0)
      {
        this.game.EditObj.LoadString = "Reload System Gfx";
        BitmapStore.ReloadSystemGraphics(this.game.Data.SystemGfx);
        this.game.EditObj.LoadString = "Load Scenario Gfx";
        this.game.Data.LoadGraphics((Form1) null);
        this.game.EditObj.LoadString = "Make Mini/Strategy Map";
        this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 200, 150, false);
        this.game.EditObj.StratMap = new Bitmap(this.game.ScreenWidth, this.game.ScreenHeight - 305);
        this.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        if (this.game.Data.Product == 7)
          this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.StratMap, this.game.ScreenWidth, this.game.ScreenHeight - 305, false, alsoshading: false);
        else
          this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.StratMap, this.game.ScreenWidth, this.game.ScreenHeight - 305, false, true, false);
        SoundMod.StopWave();
        if (this.game.Data.Product >= 6)
        {
          if (this.game.Data.UseAI == 1)
          {
            if (Information.IsNothing( this.game.NewAIObj))
              this.game.NewAIObj = new NewAIClass(this.game);
            this.game.DC2AIObj = (DC2AIClass) null;
            this.game.AIObj = (AIClass) null;
          }
          else if (this.game.Data.UseAI == 2)
          {
            if (Information.IsNothing( this.game.DC2AIObj))
              this.game.DC2AIObj = new DC2AIClass(this.game);
            this.game.NewAIObj = (NewAIClass) null;
            this.game.AIObj = (AIClass) null;
          }
        }
        if (!this.game.Data.InTurn)
        {
          this.game.EditObj.LoadingResult = LoadType.GameLoop;
          this.game.EditObj.Phase = -1;
        }
        else
        {
          this.game.HandyFunctionsObj.SetInitialXY(this.game.Data.Turn);
          this.game.EventRelatedObj.DoCheckEvents(4);
          this.game.ProcessingObj.LocationProductionPrognosis();
          this.game.EditObj.LoadingResult = LoadType.PlayScreen;
        }
        this.game.EditObj.LoadingFinished = true;
        this.game.EditObj.ButtonLoadMode = false;
      }
      else
      {
        if (Strings.Len(this.game.Data.LoadPass) > 0)
        {
          this.game.FormRef.Cursor = Cursors.Default;
          str1 = Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest");
          if (Operators.CompareString(Strings.LCase(str1), Strings.LCase(this.game.Data.LoadPass), false) == 0)
          {
            let mut num7: i32 =   Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
          }
          else
          {
            let mut num8: i32 =   Interaction.MsgBox( "Wrong Password. You cannot Load this file", Title: ( "Shadow Empire : Planetary Conquest"));
            this.game.Data = DataClass::new();
            this.game.EditObj.LoadingFinished = true;
            return;
          }
        }
        this.game.EditObj.LoadString = "Reload System Gfx";
        BitmapStore.ReloadSystemGraphics(this.game.Data.SystemGfx);
        this.game.EditObj.LoadString = "Load Scenario Gfx";
        this.game.Data.LoadGraphics((Form1) null);
        DrawMod.TGame = this.game;
        this.game.EditObj.LoadingResult = LoadType.FirstScreen;
        this.game.EditObj.LoadFileName = str1;
        this.game.EditObj.LoadingFinished = true;
        if (specialSaveMode == 1)
        {
          this.game.EditObj.LoadingResult = LoadType.GameLoop;
          this.game.EditObj.SetViewMode2 = 0;
          this.game.EditObj.inRandomScreen = false;
          this.game.EditObj.interfaceCue = 0;
          this.game.EditObj.ButtonLoadMode = false;
          this.game.EventRelatedObj.DoCheckEvents(10);
        }
        else
        {
          if (this.game.EditObj.ButtonLoadMode)
            this.game.EventRelatedObj.DoCheckEvents(9);
          this.game.EventRelatedObj.DoCheckEvents(10);
          if (this.game.EditObj.ButtonLoadMode &  this.game.Data.RuleVar[442] > 0.0)
            this.game.EditObj.LoadingResult = LoadType.RandomScreen;
          this.game.EditObj.ButtonLoadMode = false;
        }
      }
    }
  }
}
