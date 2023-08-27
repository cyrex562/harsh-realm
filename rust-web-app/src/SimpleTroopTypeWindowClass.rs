// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleTroopTypeWindowClass
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
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleTroopTypeWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     tableId: i32;
     loadId: i32;
     versionid: i32;
     text1id: i32;
     detailnr: i32;
     addId: i32;
     loadMasterId: i32;
     addReinf: i32;
     removeReinf: i32;
     ratioid: i32;
     ratioidb: i32;
     RenameReinf: i32;
     RenameReinfb: i32;
     RemoveReinfb: i32;
     a1id: i32;
     a2id: i32;
     a3id: i32;
     removeId: i32;
     changeId: i32;
     exitId: i32;
     saveId: i32;
     editId: i32;
     removeIdb: i32;
     saveIdb: i32;
     editIdb: i32;
     strId: i32;
     detailx: i32;
     detaily: i32;
     StringListClass stringy;
     VarsStartOn: i32;
     bool AddNew;
     bool Change;
     currentSfTypeNr: i32;
     cellinfoid: i32;
     int[] ColIsSFTypeVar;
     exportCsv: i32;
     importCsv: i32;
     lastsound: String;
     masterfileStart: String;

    pub SimpleTroopTypeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight, 9, tDoBorders: 1, tHeaderString: "Intermediate TroopType Editor")
    {
      self.ColIsSFTypeVar = new int[100];
      self.detailx = -1;
      self.detaily = -1;
      self.detailnr = -1;
      self.currentSfTypeNr = -1;
      self.AddNew = false;
      self.Change = false;
      self.lastsound = "";
      self.game.EditObj.TempSFType = -1;
      self.masterfileStart = self.game.Data.MasterFile;
      self.DoStuff();
    }

    pub fn DoRefresh()
    {
      if (!((self.AddNew | self.Change) & self.currentSfTypeNr > -1))
        return;
      if (self.tableId > 0)
        self.RemoveSubPart(self.tableId);
      self.tableId = 0;
      self.AddNew = false;
      self.Change = false;
      self.currentSfTypeNr = -1;
      self.DoStuff();
    }

    pub fn PopUpRefresh()
    {
    }

    pub fn RefreshCellInfo()
    {
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      txt: String;
      if (self.detaily == -1 | self.detailx == -1)
        txt = "No cell selected";
      else
        txt = "(row" + self.detailx.ToString() + ",col" + self.detaily.ToString() + ")             " + self.stringy.ColumnName[self.detaily].ToUpper() + "                " + self.stringy.Data[self.detailx, self.detaily];
      let mut tsubpart: SubPartClass =  TextPartClass::new(txt, self.game.MarcFont4, self.game.ScreenWidth - 323, 20, false, tMarc: true);
      self.cellinfoid = self.AddSubPart( tsubpart, 312, 152, self.game.ScreenWidth - 323, 20, 0);
      if (!(self.detailx > -1 & self.detaily >= 8 & self.detaily <= 9) || Operators.CompareString(self.lastsound, self.stringy.Data[self.detailx, self.detaily], false) == 0)
        return;
      self.lastsound = self.stringy.Data[self.detailx, self.detaily];
      SoundMod.PlayAWave(self.game.AppPath + "sound/" + self.stringy.Data[self.detailx, self.detaily],  self.game.EditObj);
    }

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      str: String = "";
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight, -1);
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - 322) / 24.0)) - 1;
      let mut num4: i32 = 172 + num3 * 24 + 46;
      if (self.versionid > 0)
        self.RemoveSubPart(self.versionid);
      if (self.addId > 0)
        self.RemoveSubPart(self.addId);
      if (self.removeId > 0)
        self.RemoveSubPart(self.removeId);
      if (self.editId > 0)
        self.RemoveSubPart(self.editId);
      if (self.saveId > 0)
        self.RemoveSubPart(self.saveId);
      if (self.removeIdb > 0)
        self.RemoveSubPart(self.removeIdb);
      if (self.editIdb > 0)
        self.RemoveSubPart(self.editIdb);
      if (self.saveIdb > 0)
        self.RemoveSubPart(self.saveIdb);
      if (self.exitId > 0)
        self.RemoveSubPart(self.exitId);
      if (self.a1id > 0)
        self.RemoveSubPart(self.a1id);
      if (self.a2id > 0)
        self.RemoveSubPart(self.a2id);
      if (self.a3id > 0)
        self.RemoveSubPart(self.a3id);
      if (self.loadMasterId > 0)
        self.RemoveSubPart(self.loadMasterId);
      if (self.loadId > 0)
        self.RemoveSubPart(self.loadId);
      if (self.exportCsv > 0)
        self.RemoveSubPart(self.exportCsv);
      if (self.importCsv > 0)
        self.RemoveSubPart(self.importCsv);
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      if (self.addReinf > 0)
        self.RemoveSubPart(self.addReinf);
      if (self.removeReinf > 0)
        self.RemoveSubPart(self.removeReinf);
      if (self.RenameReinf > 0)
        self.RemoveSubPart(self.RenameReinf);
      if (self.RemoveReinfb > 0)
        self.RemoveSubPart(self.RemoveReinfb);
      if (self.RenameReinfb > 0)
        self.RemoveSubPart(self.RenameReinfb);
      if (self.ratioid > 0)
        self.RemoveSubPart(self.ratioid);
      if (self.ratioidb > 0)
        self.RemoveSubPart(self.ratioidb);
      if (self.listId > 0)
        self.RemoveSubPart(self.listId);
      self.listObj = ListClass::new();
      let mut reinfCounter1: i32 = self.game.Data.ReinfCounter;
      for (let mut tdata: i32 = 0; tdata <= reinfCounter1; tdata += 1)
        self.listObj.add(self.game.Data.ReinfName[tdata] + " (ratio:" + self.game.Data.ReinfRatio[tdata].ToString() + ")", tdata);
      ListClass listObj = self.listObj;
      let mut detailnr: i32 = self.detailnr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(listObj, 24, 250, detailnr, game, true, "ReinforcementTypes", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 10, bby: 192, tMarcStyle: true, overruleFont: ( local2));
      self.listId = self.AddSubPart( tsubpart1, 10, 192, 250, 432, 0);
      DrawMod.DrawTextColouredMarc( objgraphics, "REINFORCEMENT TYPES", self.game.MarcFont4, 15, 172, Color.White);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Add ReinfType", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 620, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addReinf = self.AddSubPart( tsubpart2, 10, 620, 190, 35, 1);
      if (self.detailnr > -1)
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove ReinfType", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 660, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeReinf = self.AddSubPart( tsubpart3, 10, 660, 190, 35, 1);
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Rename ReinfType", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 700, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RenameReinf = self.AddSubPart( tsubpart4, 10, 700, 190, 35, 1);
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Set Ratio", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 740, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.ratioid = self.AddSubPart( tsubpart5, 10, 740, 190, 35, 1);
      }
      else
      {
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Remove ReinfType", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 660, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveReinfb = self.AddSubPart( tsubpart6, 10, 660, 190, 35, 1);
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Rename ReinfType", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 700, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RenameReinfb = self.AddSubPart( tsubpart7, 10, 700, 190, 35, 1);
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("Set Ratio", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 740, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.ratioidb = self.AddSubPart( tsubpart8, 10, 740, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1 & !Information.IsNothing( self.stringy))
        self.RefreshCellInfo();
      let mut smallPicCounter: i32 = self.game.Data.SmallPicCounter;
      for (let mut index: i32 = 0; index <= smallPicCounter; index += 1)
      {
        self.game.Data.SmallLibId[index].libSlot = -1;
        self.game.Data.SmallLibId[index].id = -1;
      }
      if (self.tableId == 0)
      {
        self.stringy = new StringListClass(-1);
        let mut col1: i32 = 1 - 1;
        self.stringy.AddCol(col1, "Slot#");
        let mut col2: i32 = col1 + 1;
        self.stringy.AddCol(col2, "BasedOn");
        let mut col3: i32 = col2 + 1;
        self.stringy.AddCol(col3, "Name");
        let mut col4: i32 = col3 + 1;
        self.stringy.AddCol(col4, "ReinfType1");
        let mut col5: i32 = col4 + 1;
        self.stringy.AddCol(col5, "ReinfType2");
        let mut col6: i32 = col5 + 1;
        self.stringy.AddCol(col6, "Description");
        let mut col7: i32 = col6 + 1;
        self.stringy.AddCol(col7, "Symbol Gfx");
        let mut col8: i32 = col7 + 1;
        self.stringy.AddCol(col8, "Sideways Gfx");
        let mut col9: i32 = col8 + 1;
        self.stringy.AddCol(col9, "Move Sound");
        let mut col10: i32 = col9 + 1;
        self.stringy.AddCol(col10, "Attack Sound");
        let mut col11: i32 = col10 + 1;
        self.stringy.AddCol(col11, "Ratio");
        let mut col12: i32 = col11 + 1;
        self.stringy.AddCol(col12, "Weight");
        let mut col13: i32 = col12 + 1;
        self.stringy.AddCol(col13, "Carry");
        let mut col14: i32 = col13 + 1;
        self.stringy.AddCol(col14, "Manpower");
        let mut col15: i32 = col14 + 1;
        self.stringy.AddCol(col15, "Man.Carry");
        self.VarsStartOn = col15 + 1;
        self.ColIsSFTypeVar = new int[100];
        let mut num5: i32 = 0;
        do
        {
          if (self.game.Data.TempString[num5 + 600].Length > 0)
          {
            if (Operators.CompareString(Strings.Trim(self.game.Data.TempString[num5 + 600]), "", false) == 0)
              self.game.Data.TempString[num5 + 600] = "";
            if (self.game.Data.TempString[num5 + 600].Length > 0)
            {
              col15 += 1;
              self.ColIsSFTypeVar[col15] = num5;
              self.stringy.AddCol(col15, self.game.Data.TempString[num5 + 600]);
              if (self.game.Data.TempString[num5 + 1000].Length <= 0)
                ;
            }
          }
          num5 += 1;
        }
        while (num5 <= 99);
        let mut index1: i32 = -1;
        let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
        for (let mut index2: i32 = 0; index2 <= sfTypeCounter; index2 += 1)
        {
          if (!self.game.Data.SFTypeObj[index2].DontShowInList & self.game.Data.SFTypeObj[index2].CopyDataFrom == -1)
            str = str + self.game.Data.SFTypeObj[index2].Name + " (SFTypeSlot#" + index2.ToString() + ")\r\n";
          else if (!self.game.Data.SFTypeObj[index2].DontShowInList & self.game.Data.SFTypeObj[index2].CopyDataFrom > -1)
          {
            index1 += 1;
            self.stringy.AddRow(index1 - 1);
            let mut index3: i32 = 1 - 1;
            self.stringy.Data[index1, index3] = index2.ToString();
            let mut index4: i32 = index3 + 1;
            self.stringy.Data[index1, index4] = "(" + self.game.Data.SFTypeObj[index2].Ratio.ToString() + "x)" + self.game.Data.SFTypeObj[self.game.Data.SFTypeObj[index2].CopyDataFrom].Name;
            let mut index5: i32 = index4 + 1;
            self.stringy.Data[index1, index5] = self.game.Data.SFTypeObj[index2].Name;
            let mut index6: i32 = index5 + 1;
            self.stringy.Data[index1, index6] = self.game.Data.SFTypeObj[index2].ReinforcementType <= -1 ? "None" : self.game.Data.ReinfName[self.game.Data.SFTypeObj[index2].ReinforcementType];
            let mut index7: i32 = index6 + 1;
            self.stringy.Data[index1, index7] = self.game.Data.SFTypeObj[index2].ReinforcementType2 <= -1 ? "None" : self.game.Data.ReinfName[self.game.Data.SFTypeObj[index2].ReinforcementType2];
            let mut index8: i32 = index7 + 1;
            self.stringy.Data[index1, index8] = "descript";
            let mut index9: i32 = index8 + 1;
            self.stringy.Data[index1, index9] = self.game.Data.SFTypeObj[index2].SymbolFileName;
            if (self.game.Data.SFTypeObj[index2].SymbolSpriteID > -1)
              self.stringy.TempBmp[index1, index9] = self.game.Data.SFTypeObj[index2].SymbolSpriteID;
            let mut index10: i32 = index9 + 1;
            self.stringy.Data[index1, index10] = self.game.Data.SFTypeObj[index2].SidewaysFileName;
            if (self.game.Data.SFTypeObj[index2].SidewaysSpriteID > -1)
              self.stringy.TempBmp[index1, index10] = self.game.Data.SFTypeObj[index2].SidewaysSpriteID;
            let mut index11: i32 = index10 + 1;
            self.stringy.Data[index1, index11] = self.game.Data.SFTypeObj[index2].MoveWAV;
            let mut index12: i32 = index11 + 1;
            self.stringy.Data[index1, index12] = self.game.Data.SFTypeObj[index2].BattleWAV;
            let mut index13: i32 = index12 + 1;
            self.stringy.Data[index1, index13] = self.game.Data.SFTypeObj[index2].Ratio.ToString();
            let mut index14: i32 = index13 + 1;
            self.stringy.Data[index1, index14] = self.game.Data.SFTypeObj[index2].Weight.ToString();
            let mut index15: i32 = index14 + 1;
            self.stringy.Data[index1, index15] = self.game.Data.SFTypeObj[index2].CarryCap.ToString();
            let mut index16: i32 = index15 + 1;
            self.stringy.Data[index1, index16] = self.game.Data.SFTypeObj[index2].manpower.ToString();
            let mut index17: i32 = index16 + 1;
            self.stringy.Data[index1, index17] = self.game.Data.SFTypeObj[index2].manpowerCarry.ToString();
            let mut index18: i32 = 0;
            do
            {
              if (self.game.Data.TempString[index18 + 600].Length > 0)
              {
                index17 += 1;
                self.stringy.Data[index1, index17] = self.game.Data.SFTypeObj[index2].SFTypeVar[index18].ToString();
              }
              index18 += 1;
            }
            while (index18 <= 99);
          }
        }
        let mut tsubpart9: SubPartClass =  new MatrixSubPartClass(self.stringy, num3 - 1, self.game.ScreenWidth - 323, self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 312, bby: 172, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true);
        self.tableId = self.AddSubPart( tsubpart9, 312, 172, self.game.ScreenWidth - 323, num3 * 25, 0);
      }
      if (str.Length > 0)
      {
        let mut num6: i32 =  Interaction.MsgBox( ("There is a problem with your masterfile. All the SFTypes in the master should be SFType Models (dontShowInList=True). The following are not: " + str), Title: ( "Shadow Empire : Planetary Conquest"));
      }
      let mut tsubpart10: SubPartClass =  new TextButtonPartClass("Add TroopType", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart10, num1 + 10, num4, 190, 35, 1);
      if (self.detailx > -1)
      {
        let mut tsubpart11: SubPartClass =  new TextButtonPartClass("Remove TroopType", 190, "Click and remove selected row",  self.OwnBitmap, num1 + 210, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeId = self.AddSubPart( tsubpart11, num1 + 210, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart12: SubPartClass =  new TextButtonPartClass("Remove TroopType", 190, "No row selected",  self.OwnBitmap, num1 + 210, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeIdb = self.AddSubPart( tsubpart12, num1 + 210, num4, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart13: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "Click to edit selected value or text",  self.OwnBitmap, num1 + 410, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart13, num1 + 410, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart14: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "No cell selected",  self.OwnBitmap, num1 + 410, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editIdb = self.AddSubPart( tsubpart14, num1 + 410, num4, 190, 35, 1);
      }
      if ( self.game.Data.RuleVar[946] < 1.0)
      {
        let mut tsubpart15: SubPartClass =  new TextButtonPartClass("Save Troop Library", 190, "This masterfile does not support creating TroopType libraries",  self.OwnBitmap, num1 + 610, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveIdb = self.AddSubPart( tsubpart15, num1 + 610, num4, 190, 35, 1);
      }
      else if (self.stringy.Length > -1)
      {
        let mut tsubpart16: SubPartClass =  new TextButtonPartClass("Save Troop Library", 190, "Save a TroopType Library",  self.OwnBitmap, num1 + 610, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveId = self.AddSubPart( tsubpart16, num1 + 610, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart17: SubPartClass =  new TextButtonPartClass("Save Troop Library", 190, "Nothing to save",  self.OwnBitmap, num1 + 610, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveIdb = self.AddSubPart( tsubpart17, num1 + 610, num4, 190, 35, 1);
      }
      let mut tsubpart18: SubPartClass =  new TextButtonPartClass("Exit", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 810), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exitId = self.AddSubPart( tsubpart18, num1 + 810, num4, 190, 35, 1);
      let mut tsubpart19: SubPartClass =  new TextButtonPartClass("Load TroopType Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: (num4 + 50), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadId = self.AddSubPart( tsubpart19, num1 + 10, num4 + 50, 190, 35, 1);
      let mut tsubpart20: SubPartClass =  new TextButtonPartClass("Export CSV", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 210), bby: (num4 + 50), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exportCsv = self.AddSubPart( tsubpart20, num1 + 210, num4 + 50, 190, 35, 1);
      let mut tsubpart21: SubPartClass =  new TextButtonPartClass("Import CSV", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 410), bby: (num4 + 50), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.importCsv = self.AddSubPart( tsubpart21, num1 + 410, num4 + 50, 190, 35, 1);
      tsubpart21 =  new TextButtonPartClass("Reload Master", 190, "For if you want to update the model units. It overwrites all but will keep your troops, reinftypes and library settings.",  self.OwnBitmap, num1 + 610, num4 + 50, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadMasterId = self.AddSubPart( tsubpart21, num1 + 610, num4 + 50, 190, 35, 1);
      if (self.game.Data.LibraryCounter == -1)
      {
        self.game.Data.AddLibrary();
        self.game.Data.LibraryObj[0].name = "New TroopType Library";
        self.game.Data.LibraryObj[0].information = "no info specified";
        self.game.Data.LibraryObj[0].creator = "no creator specified";
        self.game.Data.LibraryObj[0].version = 1;
      }
      let mut sfTypeCounter1: i32 = self.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter1; index += 1)
      {
        if (!self.game.Data.SFTypeObj[index].DontShowInList & self.game.Data.SFTypeObj[index].CopyDataFrom > -1)
        {
          self.game.Data.SFTypeObj[index].LibId.libSlot = 0;
          self.game.Data.SFTypeObj[index].LibId.id = -1;
        }
      }
      let mut reinfCounter2: i32 = self.game.Data.ReinfCounter;
      for (let mut index: i32 = 0; index <= reinfCounter2; index += 1)
      {
        self.game.Data.ReinfLibId[index] = LibIdClass::new();
        self.game.Data.ReinfLibId[index].libSlot = 0;
        self.game.Data.ReinfLibId[index].id = -1;
      }
      DrawMod.DrawTextColouredMarc( objgraphics, "Name:", self.game.MarcFont4, num1 + 10, 60, Color.White);
      DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.LibraryObj[0].name, self.game.MarcFont3, num1 + 10, 75, Color.White);
      tsubpart21 =  new TextButtonPartClass("Change Library Name", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a1id = self.AddSubPart( tsubpart21, num1 + 10, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( objgraphics, "Author:", self.game.MarcFont4, num1 + 310, 60, Color.White);
      DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.LibraryObj[0].creator, self.game.MarcFont3, num1 + 310, 75, Color.White);
      tsubpart21 =  new TextButtonPartClass("Change Author", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 310), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a2id = self.AddSubPart( tsubpart21, num1 + 310, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( objgraphics, "Information:", self.game.MarcFont4, num1 + 610, 60, Color.White);
      DrawMod.DrawTextColouredMarc( objgraphics, Strings.Left(self.game.Data.LibraryObj[0].information, 15) + "...", self.game.MarcFont3, num1 + 610, 75, Color.White);
      tsubpart21 =  new TextButtonPartClass("Change information", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 610), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a3id = self.AddSubPart( tsubpart21, num1 + 610, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( objgraphics, "Version:", self.game.MarcFont4, num1 + 910, 60, Color.White);
      DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.LibraryObj[0].version.ToString(), self.game.MarcFont3, num1 + 910, 75, Color.White);
      tsubpart21 =  new TextButtonPartClass("Change version", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 910), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.versionid = self.AddSubPart( tsubpart21, num1 + 910, 100, 190, 35, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.strId == -1 || Information.IsNothing( self.stringy))
        return windowReturnClass1;
      if (nr == 32 & self.detailx > -1 & self.editId > 0)
      {
        windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.editId)] + 1, self.SubPartY[self.SubpartNr(self.editId)] + 1, 1);
        windowReturnClass2.SetFlag(true);
        return windowReturnClass2;
      }
      if (nr == 38 & self.detailx > 0)
      {
        --self.detailx;
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftUp();
        self.RefreshCellInfo();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 40 & self.detailx < self.stringy.Length)
      {
        this += 1.detailx;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 37 & self.detaily > 0)
      {
        --self.detaily;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftLeft();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (!(nr == 39 & self.detaily < self.stringy.Width))
        return windowReturnClass1;
      this += 1.detaily;
      self.RefreshCellInfo();
      self.SubPartList[self.SubpartNr(self.tableId)].ShiftRight();
      self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
      windowReturnClass1.SetFlag(true);
      return windowReturnClass1;
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.addReinf)
            {
              self.game.Data.AddReinf(Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest"));
              self.game.Data.ReinfLibId[self.game.Data.ReinfCounter].libSlot = 0;
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.removeReinf)
            {
              self.game.Data.RemoveReinf(self.detailnr);
              if (self.detailnr > self.game.Data.ReinfCounter)
                --self.detailnr;
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.RenameReinf)
            {
              self.game.Data.ReinfName[self.detailnr] = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest");
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.ratioid)
            {
              self.game.Data.ReinfRatio[self.detailnr] = Conversions.ToInteger(Interaction.InputBox("Give troops ratio.", "Shadow Empire : Planetary Conquest"));
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.listId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.detailnr = num2;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.loadMasterId)
            {
              str1: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  str1 = str1.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  str1 = str1.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                str1 = str1.Replace("scenarios", self.game.ModScenarioDir);
              str2: String = str1 + self.masterfileStart;
              if (File.Exists(str2))
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.EditObj.TutMode = false;
                if (self.game.Data.UseAI == 1)
                  self.game.NewAIObj.LastRegime = -1;
                self.game.SelectX = -1;
                self.game.SelectY = -1;
                dataClass: DataClass = self.game.Data.Clone();
                self.game.Data = DataClass::new();
                GC.Collect();
                Application.DoEvents();
                self.game.HandyFunctionsObj.Unzip(str2);
                self.game.Data = DataClass.deserialize(str2);
                self.game.HandyFunctionsObj.ZipFile(str2);
                for (let mut libraryCounter: i32 = self.game.Data.LibraryCounter; libraryCounter >= 0; libraryCounter += -1)
                  self.game.Data.RemoveLibrary(libraryCounter);
                self.game.Data.AddLibrary();
                self.game.Data.LibraryObj[0] = dataClass.LibraryObj[0].Clone();
                for (let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter; sfTypeCounter >= 0; sfTypeCounter += -1)
                {
                  if (!self.game.Data.SFTypeObj[sfTypeCounter].DontShowInList & self.game.Data.SFTypeObj[sfTypeCounter].CopyDataFrom > -1)
                    self.game.Data.RemoveSFType(sfTypeCounter);
                }
                let mut sfTypeCounter1: i32 = dataClass.SFTypeCounter;
                for (let mut index2: i32 = 0; index2 <= sfTypeCounter1; index2 += 1)
                {
                  if (!dataClass.SFTypeObj[index2].DontShowInList & dataClass.SFTypeObj[index2].CopyDataFrom > -1)
                  {
                    self.game.Data.AddSFType();
                    self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter] = dataClass.SFTypeObj[index2].Clone();
                  }
                }
                let mut sfTypeCounter2: i32 = self.game.Data.SFTypeCounter;
                for (let mut index3: i32 = 0; index3 <= sfTypeCounter2; index3 += 1)
                {
                  self.game.Data.SFTypeObj[index3].CopyDataFromBackup = -1;
                  let mut num3: i32 = 0;
                  let mut sfTypeCounter3: i32 = self.game.Data.SFTypeCounter;
                  for (let mut index4: i32 = 0; index4 <= sfTypeCounter3; index4 += 1)
                  {
                    if (self.game.Data.SFTypeObj[index4].Id == self.game.Data.SFTypeObj[index3].Id & index3 != index4)
                    {
                      num3 = 1;
                      break;
                    }
                  }
                  if (num3 == 1 & self.game.Data.SFTypeObj[index3].LibId.libSlot == -1)
                  {
                    let mut id: i32 = self.game.Data.SFTypeObj[index3].Id;
                    bool flag1 = false;
                    while (!flag1)
                    {
                      bool flag2 = false;
                      let mut sfTypeCounter4: i32 = self.game.Data.SFTypeCounter;
                      for (let mut index5: i32 = 0; index5 <= sfTypeCounter4; index5 += 1)
                      {
                        if (self.game.Data.SFTypeObj[index5].Id == id)
                          flag2 = true;
                      }
                      if (!flag2)
                      {
                        self.game.Data.SFTypeObj[index3].Id = id;
                        if (id > self.game.Data.SFTypeIdCounter)
                        {
                          self.game.Data.SFTypeIdCounter = id;
                          break;
                        }
                        break;
                      }
                      id += 1;
                    }
                    num3 = 0;
                  }
                  if (num3 == 1)
                  {
                    let mut sfTypeCounter5: i32 = self.game.Data.SFTypeCounter;
                    for (let mut index6: i32 = 0; index6 <= sfTypeCounter5; index6 += 1)
                    {
                      if (self.game.Data.SFTypeObj[index6].Id > self.game.Data.SFTypeIdCounter)
                        self.game.Data.SFTypeIdCounter = self.game.Data.SFTypeObj[index6].Id;
                    }
                    this += 1.game.Data.SFTypeIdCounter;
                    self.game.Data.SFTypeObj[index3].Id = self.game.Data.SFTypeIdCounter;
                  }
                }
                while (self.game.Data.ReinfCounter > -1)
                  self.game.Data.RemoveReinf(0);
                let mut reinfCounter: i32 = dataClass.ReinfCounter;
                for (let mut index7: i32 = 0; index7 <= reinfCounter; index7 += 1)
                {
                  self.game.Data.AddReinf(dataClass.ReinfName[index7]);
                  self.game.Data.ReinfId[self.game.Data.ReinfCounter] = dataClass.ReinfId[index7];
                  self.game.Data.ReinfLibId[self.game.Data.ReinfCounter] = dataClass.ReinfLibId[index7].Clone();
                  self.game.Data.ReinfRatio[self.game.Data.ReinfCounter] = dataClass.ReinfRatio[index7];
                }
                self.game.Data.reinfIdCounter = dataClass.reinfIdCounter;
                self.game.Data.Round = 0;
                self.game.Data.Turn = 0;
                if ( self.game.Data.RuleVar[344] == 1.0 & self.game.EditObj.HideUnit == 0)
                  self.game.EditObj.HideUnit = 2;
                self.game.EditObj.TempValue = new MapMatrix2[self.game.Data.MapCounter + 1];
                self.game.EditObj.TempValue2 = new MapMatrix2[self.game.Data.MapCounter + 1];
                let mut mapCounter: i32 = self.game.Data.MapCounter;
                for (let mut index8: i32 = 0; index8 <= mapCounter; index8 += 1)
                {
                  self.game.EditObj.TempValue[index8] = new MapMatrix2(self.game.Data.MapObj[index8].MapWidth, self.game.Data.MapObj[index8].MapHeight);
                  self.game.EditObj.TempValue2[index8] = new MapMatrix2(self.game.Data.MapObj[index8].MapWidth, self.game.Data.MapObj[index8].MapHeight);
                }
                if (Strings.Len(self.game.Data.LoadPass) > 0)
                {
                  self.game.FormRef.Cursor = Cursors.Default;
                  if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(self.game.Data.LoadPass), false) == 0)
                  {
                    let mut num4: i32 =  Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    let mut num5: i32 =  Interaction.MsgBox( "Wrong Password. You cannot Load this file. Exiting whole app.", Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.EndApp();
                  }
                }
                BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                self.game.Data.LoadGraphics((Form1) null);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                self.game.EditObj.StratMap = new Bitmap(self.game.ScreenWidth, self.game.ScreenHeight - 265);
                self.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.StratMap, self.game.ScreenWidth, self.game.ScreenHeight - 265, false, true, false);
                self.game.FormRef.Cursor = Cursors.Default;
                let mut num6: i32 =  Interaction.MsgBox( "Loaded Masterfile", Title: ( "Shadow Empire : Planetary Conquest"));
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.loadId)
            {
              tinitdir: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Troops&Equipment library(*.se1troops)|*.se1troops", "Pick a trooptype library...", tinitdir, false);
              if (File.Exists(str))
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.EditObj.TutMode = false;
                if (self.game.Data.UseAI == 1)
                  self.game.NewAIObj.LastRegime = -1;
                self.game.SelectX = -1;
                self.game.SelectY = -1;
                self.game.Data = DataClass::new();
                GC.Collect();
                Application.DoEvents();
                self.game.HandyFunctionsObj.Unzip(str);
                self.game.Data = DataClass.deserialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                if (Operators.CompareString(self.game.Data.MasterFile, "", false) == 0)
                {
                  if (Strings.Len(self.masterfileStart) > 0 && Interaction.MsgBox( ("Update data with masterfile '" + self.masterfileStart + "' data"), MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                  {
                    self.game.Data.MasterfileReadPeople = false;
                    masterfileStart: String = self.masterfileStart;
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.HandyFunctionsObj.ReturnLongMaster(str, masterfileStart));
                    self.game.Data.MasterFile = self.masterfileStart;
                  }
                }
                else
                {
                  self.game.Data.MasterfileReadPeople = false;
                  masterFile: String = self.game.Data.MasterFile;
                  self.game.HandyFunctionsObj.LoadMasterFile(self.game.HandyFunctionsObj.ReturnLongMaster(str, masterFile));
                }
                self.game.Data.Round = 0;
                self.game.Data.Turn = 0;
                if ( self.game.Data.RuleVar[344] == 1.0 & self.game.EditObj.HideUnit == 0)
                  self.game.EditObj.HideUnit = 2;
                self.game.EditObj.TempValue = new MapMatrix2[self.game.Data.MapCounter + 1];
                self.game.EditObj.TempValue2 = new MapMatrix2[self.game.Data.MapCounter + 1];
                let mut mapCounter: i32 = self.game.Data.MapCounter;
                for (let mut index9: i32 = 0; index9 <= mapCounter; index9 += 1)
                {
                  self.game.EditObj.TempValue[index9] = new MapMatrix2(self.game.Data.MapObj[index9].MapWidth, self.game.Data.MapObj[index9].MapHeight);
                  self.game.EditObj.TempValue2[index9] = new MapMatrix2(self.game.Data.MapObj[index9].MapWidth, self.game.Data.MapObj[index9].MapHeight);
                }
                if (Strings.Len(self.game.Data.LoadPass) > 0)
                {
                  self.game.FormRef.Cursor = Cursors.Default;
                  if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(self.game.Data.LoadPass), false) == 0)
                  {
                    let mut num7: i32 =  Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    let mut num8: i32 =  Interaction.MsgBox( "Wrong Password. You cannot Load this file. Exiting whole app.", Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.EndApp();
                  }
                }
                BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                self.game.Data.LoadGraphics((Form1) null);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                self.game.EditObj.StratMap = new Bitmap(self.game.ScreenWidth, self.game.ScreenHeight - 265);
                self.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.StratMap, self.game.ScreenWidth, self.game.ScreenHeight - 265, false, true, false);
                self.game.FormRef.Cursor = Cursors.Default;
                let mut sfTypeCounter6: i32 = self.game.Data.SFTypeCounter;
                for (let mut index10: i32 = 0; index10 <= sfTypeCounter6; index10 += 1)
                {
                  self.game.Data.SFTypeObj[index10].CopyDataFromBackup = -1;
                  let mut num9: i32 = 0;
                  let mut sfTypeCounter7: i32 = self.game.Data.SFTypeCounter;
                  for (let mut index11: i32 = 0; index11 <= sfTypeCounter7; index11 += 1)
                  {
                    if (self.game.Data.SFTypeObj[index11].Id == self.game.Data.SFTypeObj[index10].Id & index10 != index11)
                    {
                      num9 = 1;
                      break;
                    }
                  }
                  if (num9 == 1 & self.game.Data.SFTypeObj[index10].LibId.libSlot == -1)
                  {
                    let mut id: i32 = self.game.Data.SFTypeObj[index10].Id;
                    bool flag3 = false;
                    while (!flag3)
                    {
                      bool flag4 = false;
                      let mut sfTypeCounter8: i32 = self.game.Data.SFTypeCounter;
                      for (let mut index12: i32 = 0; index12 <= sfTypeCounter8; index12 += 1)
                      {
                        if (self.game.Data.SFTypeObj[index12].Id == id)
                          flag4 = true;
                      }
                      if (!flag4)
                      {
                        self.game.Data.SFTypeObj[index10].Id = id;
                        if (id > self.game.Data.SFTypeIdCounter)
                        {
                          self.game.Data.SFTypeIdCounter = id;
                          break;
                        }
                        break;
                      }
                      id += 1;
                    }
                    num9 = 0;
                  }
                  if (num9 == 1)
                  {
                    let mut sfTypeCounter9: i32 = self.game.Data.SFTypeCounter;
                    for (let mut index13: i32 = 0; index13 <= sfTypeCounter9; index13 += 1)
                    {
                      if (self.game.Data.SFTypeObj[index13].Id > self.game.Data.SFTypeIdCounter)
                        self.game.Data.SFTypeIdCounter = self.game.Data.SFTypeObj[index13].Id;
                    }
                    this += 1.game.Data.SFTypeIdCounter;
                    self.game.Data.SFTypeObj[index10].Id = self.game.Data.SFTypeIdCounter;
                  }
                }
                let mut num10: i32 = 0;
                let mut sfTypeCounter10: i32 = self.game.Data.SFTypeCounter;
                for (let mut index14: i32 = 0; index14 <= sfTypeCounter10; index14 += 1)
                {
                  if (self.game.Data.SFTypeObj[index14].Id >= num10)
                    num10 = self.game.Data.SFTypeObj[index14].Id;
                }
                if (num10 + 100 < self.game.Data.SFTypeIdCounter)
                  self.game.Data.SFTypeIdCounter = num10 + 100;
                let mut num11: i32 =  Interaction.MsgBox( "Loaded TroopType Library", Title: ( "Shadow Empire : Planetary Conquest"));
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.importCsv)
            {
              str3: String = self.game.HandyFunctionsObj.LoadSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Select file to import...", self.game.AppPath + "csv/", false);
              if (Strings.Len(str3) < 2)
              {
                let mut num12: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                StreamReader streamReader;
                windowReturnClass2: WindowReturnClass;
                try
                {
                  streamReader = File.OpenText(str3);
                  let mut num13: i32 = 0;
                  str4: String = ",";
                  try
                  {
                    while (!streamReader.EndOfStream)
                    {
                      str5: String = streamReader.ReadLine();
                      if ((uint) Strings.InStr(str5, "sep=") > 0U)
                      {
                        str4 = Strings.Mid(str5, 5, 1);
                        num13 = 1;
                      }
                      else
                      {
                        switch (num13)
                        {
                          case 0:
                            if (Strings.InStr(str5, "\t") > 0)
                              str4 = "\t";
                            else if (Strings.InStr(str5, ",") > 0)
                              str4 = ",";
                            else if (Strings.InStr(str5, ";") > 0)
                              str4 = ";";
                            num13 = 2;
                            continue;
                          case 1:
                            num13 = 2;
                            continue;
                          case 2:
                            strArray: Vec<String> = str5.Split(Conversions.ToChar(str4));
                            let mut num14: i32 = 0;
                            num15: i32;
                            num15 += 1;
                            let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                            index15: i32;
                            for (index15 = 0; index15 <= sfTypeCounter; index15 += 1)
                            {
                              if (!self.game.Data.SFTypeObj[index15].DontShowInList & self.game.Data.SFTypeObj[index15].CopyDataFrom > -1)
                              {
                                num14 += 1;
                                if (num14 == num15)
                                  break;
                              }
                            }
                            if (num14 != num15)
                            {
                              self.game.Data.AddSFType();
                              index15 = self.game.Data.SFTypeCounter;
                              num14 = num15;
                            }
                            if (num14 == num15)
                            {
                              let mut index16: i32 = index15;
                              self.game.Data.SFTypeObj[index16].CopyDataFrom = Conversions.ToInteger(strArray[0]);
                              self.game.Data.SFTypeObj[index16].Name = strArray[1];
                              self.game.Data.SFTypeObj[index16].ReinforcementType = Conversions.ToInteger(strArray[2]);
                              self.game.Data.SFTypeObj[index16].ReinforcementType2 = Conversions.ToInteger(strArray[3]);
                              while (self.game.Data.SFTypeObj[index16].ReinforcementType > self.game.Data.ReinfCounter)
                              {
                                self.game.Data.AddReinf("Unknown ReinfType");
                                self.game.Data.ReinfLibId[self.game.Data.ReinfCounter].libSlot = 0;
                              }
                              while (self.game.Data.SFTypeObj[index16].ReinforcementType2 > self.game.Data.ReinfCounter)
                              {
                                self.game.Data.AddReinf("Unknown ReinfType");
                                self.game.Data.ReinfLibId[self.game.Data.ReinfCounter].libSlot = 0;
                              }
                              self.game.Data.SFTypeObj[index16].SymbolFileName = strArray[4];
                              self.game.Data.SFTypeObj[index16].SidewaysFileName = strArray[5];
                              self.game.Data.SFTypeObj[index16].MoveWAV = strArray[6];
                              self.game.Data.SFTypeObj[index16].BattleWAV = strArray[7];
                              self.game.Data.SFTypeObj[index16].Ratio = Conversions.ToInteger(strArray[8]);
                              self.game.Data.SFTypeObj[index16].Weight = Conversions.ToInteger(strArray[9]);
                              self.game.Data.SFTypeObj[index16].CarryCap = Conversions.ToInteger(strArray[10]);
                              self.game.Data.SFTypeObj[index16].manpower = Conversions.ToInteger(strArray[11]);
                              self.game.Data.SFTypeObj[index16].manpowerCarry = Conversions.ToInteger(strArray[12]);
                              let mut num16: i32 = 0;
                              let mut index17: i32 = 0;
                              do
                              {
                                if (self.game.Data.TempString[index17 + 600].Length > 0)
                                {
                                  num16 += 1;
                                  self.game.Data.SFTypeObj[index16].SFTypeVar[index17] = Conversions.ToInteger(strArray[12 + num16]);
                                }
                                index17 += 1;
                              }
                              while (index17 <= 99);
                              continue;
                            }
                            continue;
                          default:
                            continue;
                        }
                      }
                    }
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    Exception exception = ex;
                    streamReader.Close();
                    self.game.Data.LoadGraphics(self.formref);
                    let mut num17: i32 =  Interaction.MsgBox( ("Import problem encountered: '" + exception.Message + "'. Your current data might be corrupted. "), Title: ( "Shadow Empire : Planetary Conquest"));
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                    windowReturnClass1.SetFlag(true);
                    windowReturnClass2 = windowReturnClass1;
                    ProjectData.ClearProjectError();
                    goto label_298;
                  }
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  let mut num18: i32 =  Interaction.MsgBox( "Error opening file", Title: ( "Shadow Empire : Planetary Conquest"));
                  windowReturnClass2 = windowReturnClass1;
                  ProjectData.ClearProjectError();
                  goto label_298;
                }
                streamReader.Close();
                self.game.Data.LoadGraphics(self.formref);
                let mut num19: i32 =  Interaction.MsgBox( "Import finished", Title: ( "Shadow Empire : Planetary Conquest"));
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
label_298:
                return windowReturnClass2;
              }
            }
            else if (num1 == self.exportCsv)
            {
              str6: String = self.game.HandyFunctionsObj.SaveSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Give save name...", self.game.AppPath + "csv/", false);
              if (Strings.Len(str6) < 2)
              {
                let mut num20: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                try
                {
                  StreamWriter text = File.CreateText(str6);
                  try
                  {
                    text.WriteLine("sep=\t");
                    str7: String = "" + "BasedOn" + "\t" + "Name" + "\t" + "ReinfType1" + "\t" + "ReinfType2" + "\t" + "Symbol Gfx" + "\t" + "Sideways Gfx" + "\t" + "Move Sound" + "\t" + "Attack Sound" + "\t" + "Ratio" + "\t" + "Weight" + "\t" + "Carry" + "\t" + "Manpower" + "\t" + "ManpCarry";
                    let mut num21: i32 = 0;
                    do
                    {
                      if (self.game.Data.TempString[num21 + 600].Length > 0)
                        str7 = str7 + "\t" + self.game.Data.TempString[num21 + 600];
                      num21 += 1;
                    }
                    while (num21 <= 99);
                    text.WriteLine(str7);
                    let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                    for (let mut index18: i32 = 0; index18 <= sfTypeCounter; index18 += 1)
                    {
                      if (!self.game.Data.SFTypeObj[index18].DontShowInList & self.game.Data.SFTypeObj[index18].CopyDataFrom > -1)
                      {
                        str8: String = "" + self.game.Data.SFTypeObj[index18].CopyDataFrom.ToString() + "\t" + self.game.Data.SFTypeObj[index18].Name + "\t";
                        str9: String = (self.game.Data.SFTypeObj[index18].ReinforcementType <= -1 ? str8 + "-1" : str8 + self.game.Data.SFTypeObj[index18].ReinforcementType.ToString()) + "\t";
                        str10: String = (self.game.Data.SFTypeObj[index18].ReinforcementType2 <= -1 ? str9 + "-1" : str9 + self.game.Data.SFTypeObj[index18].ReinforcementType2.ToString()) + "\t" + self.game.Data.SFTypeObj[index18].SymbolFileName + "\t" + self.game.Data.SFTypeObj[index18].SidewaysFileName + "\t" + self.game.Data.SFTypeObj[index18].MoveWAV + "\t" + self.game.Data.SFTypeObj[index18].BattleWAV + "\t" + self.game.Data.SFTypeObj[index18].Ratio.ToString() + "\t" + self.game.Data.SFTypeObj[index18].Weight.ToString() + "\t" + self.game.Data.SFTypeObj[index18].CarryCap.ToString() + "\t" + self.game.Data.SFTypeObj[index18].manpower.ToString() + "\t" + self.game.Data.SFTypeObj[index18].manpowerCarry.ToString();
                        let mut index19: i32 = 0;
                        do
                        {
                          if (self.game.Data.TempString[index19 + 600].Length > 0)
                            str10 = str10 + "\t" + self.game.Data.SFTypeObj[index18].SFTypeVar[index19].ToString();
                          index19 += 1;
                        }
                        while (index19 <= 99);
                        text.WriteLine(str10);
                      }
                    }
                    text.Close();
                    let mut num22: i32 =  Interaction.MsgBox( "Export has been written to the csv/ directory", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    Exception exception = ex;
                    text.Close();
                    let mut num23: i32 =  Interaction.MsgBox( ("Problem writing: " + exception.Message), Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.ClearProjectError();
                  }
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  let mut num24: i32 =  Interaction.MsgBox( "Problem writing. Check if the file is not opened in other application please.", Title: ( "Shadow Empire : Planetary Conquest"));
                  ProjectData.ClearProjectError();
                }
              }
            }
            else if (num1 == self.saveId)
            {
              tinitdir: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Troops&Equipment library(*.se1troops)|*.se1troops", "Give save name...", tinitdir, false);
              if (Strings.Len(str) < 2)
              {
                let mut num25: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.Interpret();
                self.game.Data.serialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                windowReturnClass1.SetFlag(true);
                self.game.FormRef.Cursor = Cursors.Default;
                self.game.Data.LoadGraphics(self.formref);
                let mut num26: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else
            {
              if (num1 == self.a1id)
              {
                str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].name);
                if (str.Length > 0)
                  self.game.Data.LibraryObj[0].name = str;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.a2id)
              {
                str: String = Interaction.InputBox("Give author name.", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].creator);
                if (str.Length > 0)
                  self.game.Data.LibraryObj[0].creator = str;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.versionid)
              {
                InputStr: String = Interaction.InputBox("Give version.", "Shadow Empire : Planetary Conquest", Conversions.ToString(self.game.Data.LibraryObj[0].version));
                if (InputStr.Length > 0)
                  self.game.Data.LibraryObj[0].version =  Math.Round(Conversion.Val(InputStr));
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.a3id)
              {
                self.Change = true;
                Form2::new( self.formref).Initialize(self.game.Data, 13, 0);
                return windowReturnClass1;
              }
              if (num1 == self.exitId)
              {
                self.game.EditObj.InEditor = false;
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.Interpret();
                self.game.Data.LoadGraphics(self.formref);
                self.game.FormRef.Cursor = Cursors.Default;
                if (self.game.EditorBlock)
                  self.game.EditObj.ShowInitialMenu = true;
                if (self.game.ModIntroType == 0)
                  windowReturnClass1.AddCommand(3, 1);
                else
                  windowReturnClass1.AddCommand(3, 12);
              }
              else
              {
                if (num1 == self.editId)
                {
                  if (self.detaily == 0)
                  {
                    let mut num27: i32 =  Interaction.MsgBox( "Cannot change", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    if (self.detaily == 1)
                    {
                      self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 130, self.currentSfTypeNr, tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 2)
                    {
                      self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.SFTypeObj[self.currentSfTypeNr].Name = str;
                      self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, 0);
                      self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 5)
                    {
                      self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      Form2::new( self.formref).Initialize(self.game.Data, 1, self.currentSfTypeNr);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 3)
                    {
                      self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 51, self.currentSfTypeNr, tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 4)
                    {
                      self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 89, self.currentSfTypeNr, tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 6)
                    {
                      str: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of symbol Sprite:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + str))
                      {
                        if (Strings.InStr(str, "BIG") > 0 | Strings.InStr(str, "SMALL") > 0)
                        {
                          let mut num28: i32 =  Interaction.MsgBox( "You cannot load a file that in the path include 'BIG' or 'SMALL'.", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                        else
                        {
                          self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                          self.stringy.Data[self.detailx, self.detaily] = str;
                          self.game.Data.SFTypeObj[self.currentSfTypeNr].SymbolFileName = str;
                          self.game.Data.SFTypeObj[self.currentSfTypeNr].ReplaceSymbolSprite(str);
                          self.RemoveSubPart(self.tableId);
                          self.tableId = 0;
                          self.DoStuff();
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                      }
                      else
                      {
                        let mut num29: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                    }
                    else if (self.detaily == 7)
                    {
                      s: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of sideways Sprite:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + s))
                      {
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.stringy.Data[self.detailx, self.detaily] = s;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].SidewaysFileName = s;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].ReplaceSidewaysSprite(s);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      let mut num30: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      if (self.detaily == 8)
                      {
                        Left: String = self.game.Data.SoundDir;
                        if (Operators.CompareString(Left, "", false) == 0)
                          Left = self.game.ModSoundDir;
                        if (Operators.CompareString(Left, "", false) == 0)
                          Left = "sound";
                        str11: String = Left + "/";
                        str12: String = self.game.HandyFunctionsObj.LoadSomething("WAVs (*.wav)|*.wav", "Select File For Move Sound", self.game.AppPath + str11, true);
                        if (!File.Exists(self.game.AppPath + str11 + str12))
                        {
                          let mut num31: i32 =  Interaction.MsgBox( "File does not exist. wav set to no sound.", Title: ( "Shadow Empire : Planetary Conquest"));
                          str12 = "";
                        }
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.stringy.Data[self.detailx, self.detaily] = str12;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].MoveWAV = str12;
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 9)
                      {
                        Left: String = self.game.Data.SoundDir;
                        if (Operators.CompareString(Left, "", false) == 0)
                          Left = self.game.ModSoundDir;
                        if (Operators.CompareString(Left, "", false) == 0)
                          Left = "sound";
                        str13: String = Left + "/";
                        str14: String = self.game.HandyFunctionsObj.LoadSomething("WAVs (*.wav)|*.wav", "Select File For Battle Sound", self.game.AppPath + str13, true);
                        if (!File.Exists(self.game.AppPath + str13 + str14))
                        {
                          let mut num32: i32 =  Interaction.MsgBox( "File does not exist. wav set to no sound.", Title: ( "Shadow Empire : Planetary Conquest"));
                          str14 = "";
                        }
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.stringy.Data[self.detailx, self.detaily] = str14;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].BattleWAV = str14;
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 10)
                      {
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give ratio.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].Ratio =  Math.Round(Conversion.Val(InputStr));
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 11)
                      {
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give weight points.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].Weight =  Math.Round(Conversion.Val(InputStr));
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 12)
                      {
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give carry points.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].CarryCap =  Math.Round(Conversion.Val(InputStr));
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 13)
                      {
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give manpower points.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].manpower =  Math.Round(Conversion.Val(InputStr));
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 14)
                      {
                        self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give manpower carry points.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        self.game.Data.SFTypeObj[self.currentSfTypeNr].manpowerCarry =  Math.Round(Conversion.Val(InputStr));
                        self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Conversion.Str( Conversion.Val(Interaction.InputBox("Give new value of cell, please.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.SFTypeObj[self.currentSfTypeNr].SFTypeVar[self.ColIsSFTypeVar[self.detaily]] = Conversions.ToInteger(str);
                      self.SubPartList[self.SubpartNr(self.tableId)].Refresh(self.stringy, self.detailx, self.detaily);
                      self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                  }
                  return windowReturnClass1;
                }
                if (num1 == self.removeId)
                {
                  self.currentSfTypeNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                  --self.detailx;
                  self.game.Data.RemoveSFType(self.currentSfTypeNr);
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.addId)
                {
                  self.game.Data.AddSFType();
                  self.currentSfTypeNr = self.game.Data.SFTypeCounter;
                  self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter].LibId.libSlot = 0;
                  self.AddNew = true;
                  Form3::new( self.formref).Initialize(self.game.Data, 130, self.currentSfTypeNr, 1, self.game);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.tableId)
                {
                  Coordinate coordinate = self.SubPartList[index1].Click2(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (coordinate.x > -1)
                  {
                    self.detailx = coordinate.x;
                    self.detaily = coordinate.y;
                    if (self.detaily > self.stringy.Width)
                      self.detaily = self.stringy.Width;
                    if (self.detailx > self.stringy.Length)
                      self.detailx = self.stringy.Length;
                  }
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
              }
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn Interpret()
    {
      let mut sfTypeCounter1: i32 = self.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter1; index += 1)
      {
        if (!self.game.Data.SFTypeObj[index].DontShowInList & self.game.Data.SFTypeObj[index].CopyDataFrom > -1)
        {
          name: String = self.game.Data.SFTypeObj[index].Name;
          description: String = self.game.Data.SFTypeObj[index].Description;
          let mut reinforcementType: i32 = self.game.Data.SFTypeObj[index].ReinforcementType;
          let mut reinforcementType2: i32 = self.game.Data.SFTypeObj[index].ReinforcementType2;
          symbolFileName: String = self.game.Data.SFTypeObj[index].SymbolFileName;
          sidewaysFileName: String = self.game.Data.SFTypeObj[index].SidewaysFileName;
          moveWav: String = self.game.Data.SFTypeObj[index].MoveWAV;
          battleWav: String = self.game.Data.SFTypeObj[index].BattleWAV;
          let mut copyDataFrom: i32 = self.game.Data.SFTypeObj[index].CopyDataFrom;
          let mut id: i32 = self.game.Data.SFTypeObj[index].Id;
          let mut weight: i32 = self.game.Data.SFTypeObj[index].Weight;
          let mut carryCap: i32 = self.game.Data.SFTypeObj[index].CarryCap;
          let mut manpower: i32 = self.game.Data.SFTypeObj[index].manpower;
          let mut manpowerCarry: i32 = self.game.Data.SFTypeObj[index].manpowerCarry;
          int[] numArray1 = new int[100];
          int[] numArray2 = (int[]) self.game.Data.SFTypeObj[index].SFTypeVar.Clone();
          self.game.Data.SFTypeObj[index] = self.game.Data.SFTypeObj[copyDataFrom].Clone();
          self.game.Data.SFTypeObj[index].Name = name;
          self.game.Data.SFTypeObj[index].Description = description;
          self.game.Data.SFTypeObj[index].SFTypeVar = (int[]) numArray2.Clone();
          self.game.Data.SFTypeObj[index].DontShowInList = false;
          self.game.Data.SFTypeObj[index].ReinforcementType = reinforcementType;
          self.game.Data.SFTypeObj[index].ReinforcementType2 = reinforcementType2;
          self.game.Data.SFTypeObj[index].PicFileName = "systemgraphics/trans.bmp";
          self.game.Data.SFTypeObj[index].SymbolFileName = symbolFileName;
          self.game.Data.SFTypeObj[index].SymbolFileName2 = symbolFileName;
          self.game.Data.SFTypeObj[index].SidewaysFileName = sidewaysFileName;
          self.game.Data.SFTypeObj[index].MoveWAV = moveWav;
          self.game.Data.SFTypeObj[index].BattleWAV = battleWav;
          self.game.Data.SFTypeObj[index].CopyDataFrom = copyDataFrom;
          self.game.Data.SFTypeObj[index].Id = id;
          self.game.Data.SFTypeObj[index].Weight = weight;
          self.game.Data.SFTypeObj[index].CarryCap = carryCap;
          self.game.Data.SFTypeObj[index].manpower = manpower;
          self.game.Data.SFTypeObj[index].manpowerCarry = manpowerCarry;
        }
      }
      let mut sfTypeCounter2: i32 = self.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter2; index += 1)
      {
        if (!self.game.Data.SFTypeObj[index].DontShowInList & self.game.Data.SFTypeObj[index].CopyDataFrom > -1)
        {
          self.game.Data.SFTypeObj[index].LibId.libSlot = 0;
          self.game.Data.SFTypeObj[index].LibId.id = -1;
        }
      }
      let mut reinfCounter: i32 = self.game.Data.ReinfCounter;
      for (let mut index: i32 = 0; index <= reinfCounter; index += 1)
      {
        self.game.Data.ReinfLibId[index].libSlot = 0;
        self.game.Data.ReinfLibId[index].id = -1;
      }
      self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.HandyFunctionsObj.GetEventByID( Math.Round( self.game.Data.RuleVar[946])));
    }
  }
}
