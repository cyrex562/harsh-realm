// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.StringListWindowClass
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
  pub class StringListWindowClass : WindowClass
  {
     ListId: i32;
     ListClass ListObj;
     LibListId: i32;
     ListClass LibListObj;
     BAddId: i32;
     BAddTextId: i32;
     b1id: i32;
     b1textid: i32;
     b2id: i32;
     b2textid: i32;
     b3id: i32;
     b3textid: i32;
     b4id: i32;
     b4textid: i32;
     b5id: i32;
     b5textid: i32;
     b6id: i32;
     b6textid: i32;
     b7id: i32;
     b7textid: i32;
     b8id: i32;
     b8textid: i32;
     b9id: i32;
     b9textid: i32;
     b10id: i32;
     b10textid: i32;
     b11id: i32;
     b11textid: i32;
     b12id: i32;
     b12textid: i32;
     b13id: i32;
     b13textid: i32;
     b14id: i32;
     b14textid: i32;
     b15id: i32;
     b15textid: i32;
     b16id: i32;
     b16textid: i32;
     b17id: i32;
     b17textid: i32;
     b18id: i32;
     b18textid: i32;
     b19id: i32;
     b19textid: i32;
     b20id: i32;
     b20textid: i32;
     b21id: i32;
     b21textid: i32;
     b22id: i32;
     b22textid: i32;
     BNameId: i32;
     BNameTextId: i32;
     OptionsListId: i32;
     BRemoveId: i32;
     BRemoveTextId: i32;
     detailnr: i32;
     libnr: i32;
     detailx: i32;
     detaily: i32;
     ss: String;

    pub StringListWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "String Lists")
    {
      self.detailnr = -1;
      self.libnr = -1;
      self.detailx = -1;
      self.detaily = -1;
      self.MakeList(-1);
    }

    pub fn DoRefresh() => self.MakeList(self.detailnr);

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.detailnr == -1)
        return windowReturnClass;
      if (nr == 32 & self.detailx > -1 & self.b1id > 0)
      {
        windowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.b1id)] + 1, self.SubPartY[self.SubpartNr(self.b1id)] + 1, 1);
        windowReturnClass.SetFlag(true);
      }
      if (nr == 38 & self.detailx > 0)
      {
        --self.detailx;
        self.MakeItem();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (nr == 40 & self.detailx < self.game.Data.StringListObj[self.detailnr].Length)
      {
        this += 1.detailx;
        self.MakeItem();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (nr == 37 & self.detaily > 0)
      {
        --self.detaily;
        self.MakeItem();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (!(nr == 39 & self.detaily < self.game.Data.StringListObj[self.detailnr].Width))
        return windowReturnClass;
      this += 1.detaily;
      self.MakeItem();
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

     void MakeList(tDetailnr: i32)
    {
      let mut num1: i32 = -1;
      if (self.LibListId > 0)
        self.RemoveSubPart(self.LibListId);
      if (self.game.Data.LibraryCounter > -1)
      {
        self.LibListObj = ListClass::new();
        self.LibListObj.add("All stringlists", -2);
        self.LibListObj.add("Without libraries", -3);
        if (self.libnr == -1)
          num1 = 0;
        if (self.libnr == -2)
          num1 = 1;
        let mut num2: i32 = 1;
        let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
        for (let mut index: i32 = 0; index <= libraryCounter; index += 1)
        {
          num2 += 1;
          if (self.libnr == index)
            num1 = num2;
          self.LibListObj.add(Conversion.Str( index) + ") " + self.game.Data.LibraryObj[index].name, index);
        }
        ListClass libListObj = self.LibListObj;
        let mut tlistselect: i32 = num1;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(libListObj, 10, 200, tlistselect, game, tHeader: "Libraries", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
        self.LibListId = self.AddSubPart( tsubpart, 10, 50, 200, 208, 0);
      }
      if (self.ListId > 0)
        self.RemoveSubPart(self.ListId);
      let mut num3: i32 = -1;
      let mut num4: i32 = -1;
      if (self.game.Data.StringListCounter > -1)
      {
        self.ListObj = ListClass::new();
        let mut stringListCounter: i32 = self.game.Data.StringListCounter;
        for (let mut index: i32 = 0; index <= stringListCounter; index += 1)
        {
          if (self.game.Data.StringListObj[index].LibId.libSlot == self.libnr | self.libnr == -1 | self.game.Data.StringListObj[index].LibId.libSlot == -1 & self.libnr == -2)
          {
            num3 += 1;
            if (tDetailnr == index)
              num4 = num3;
            self.ListObj.add(Conversion.Str( index) + ") " + self.game.Data.StringListObj[index].Name + "(ID" + Strings.Trim(Conversion.Str( self.game.Data.StringListObj[index].ID)) + ")", index);
          }
        }
        let mut num5: i32 = 0;
        if (self.game.ScreenHeight > 768)
          num5 = Math.Max(0,  Math.Round( (self.game.ScreenHeight - 768) / 16.0) - 2);
        ListClass listObj = self.ListObj;
        let mut tlistsize: i32 = 20 + num5;
        let mut tlistselect: i32 = num4;
        let mut game: GameClass = self.game;
         local3: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local4: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(listObj, tlistsize, 200, tlistselect, game, tHeader: "Stringlists", tbackbitmap: ( local3), bbx: 10, bby: 274, overruleFont: ( local4));
        self.ListId = self.AddSubPart( tsubpart, 10, 274, 200, (23 + num5) * 16, 0);
        self.detailnr = tDetailnr;
        self.MakeItem();
      }
      else
      {
        self.detailnr = tDetailnr;
        self.MakeItem();
      }
      if (self.BAddId > 0)
        self.RemoveSubPart(self.BAddId);
      if (self.BAddTextId > 0)
        self.RemoveSubPart(self.BAddTextId);
      self.ss = "Click to add a new simplelist";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
      self.BAddId = self.AddSubPart( tsubpart1, 310, 50, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Add Simplelist", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.BAddTextId = self.AddSubPart( tsubpart2, 350, 49, 150, 20, 0);
      if (self.b8id > 0)
        self.RemoveSubPart(self.b8id);
      if (self.b8textid > 0)
        self.RemoveSubPart(self.b8textid);
      if (self.b9id > 0)
        self.RemoveSubPart(self.b9id);
      if (self.b9textid > 0)
        self.RemoveSubPart(self.b9textid);
      if (self.b10id > 0)
        self.RemoveSubPart(self.b10id);
      if (self.b10textid > 0)
        self.RemoveSubPart(self.b10textid);
      if (self.b12id > 0)
        self.RemoveSubPart(self.b12id);
      if (self.b12textid > 0)
        self.RemoveSubPart(self.b12textid);
      if (self.b13id > 0)
        self.RemoveSubPart(self.b13id);
      if (self.b13textid > 0)
        self.RemoveSubPart(self.b13textid);
      if (self.b14id > 0)
        self.RemoveSubPart(self.b14id);
      if (self.b14textid > 0)
        self.RemoveSubPart(self.b14textid);
      if (self.b17id > 0)
        self.RemoveSubPart(self.b17id);
      if (self.b17textid > 0)
        self.RemoveSubPart(self.b17textid);
      if (self.b18id > 0)
        self.RemoveSubPart(self.b18id);
      if (self.b18textid > 0)
        self.RemoveSubPart(self.b18textid);
      if (self.b19id > 0)
        self.RemoveSubPart(self.b19id);
      if (self.b19textid > 0)
        self.RemoveSubPart(self.b19textid);
      self.ss = "Import 1 list from another scenario or masterfile";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.b8id = self.AddSubPart( tsubpart2, 510, 50, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Import Spec.List", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b8textid = self.AddSubPart( tsubpart2, 550, 49, 150, 20, 0);
      self.ss = "Import all lists from another scenario or masterfile";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.b9id = self.AddSubPart( tsubpart2, 710, 50, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Import All.List", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b9textid = self.AddSubPart( tsubpart2, 750, 49, 150, 20, 0);
      self.ss = "Get all stringlists where a certain expression occurs";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.b19id = self.AddSubPart( tsubpart2, 1110, 50, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Search", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b19textid = self.AddSubPart( tsubpart2, 1150, 49, 150, 20, 0);
      self.ss = "Add records from textfile to list. can use comma-seperated-values. will auto add columns if neccessary.";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.b10id = self.AddSubPart( tsubpart2, 910, 50, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Imp Txt Names", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b10textid = self.AddSubPart( tsubpart2, 950, 49, 150, 20, 0);
      self.ss = "Writes this stringlist to a file of choice in CSV format.";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.b12id = self.AddSubPart( tsubpart2, 910, 70, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Export Txt Names", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b12textid = self.AddSubPart( tsubpart2, 950, 69, 150, 20, 0);
      self.ss = "Clears this Stringlist";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
      self.b13id = self.AddSubPart( tsubpart2, 710, 70, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("CLEAR", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b13textid = self.AddSubPart( tsubpart2, 750, 69, 150, 20, 0);
    }

     void MakeItem()
    {
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
      if (self.BRemoveId > 0)
        self.RemoveSubPart(self.BRemoveId);
      if (self.BRemoveTextId > 0)
        self.RemoveSubPart(self.BRemoveTextId);
      if (self.b2id > 0)
        self.RemoveSubPart(self.b2id);
      if (self.b2textid > 0)
        self.RemoveSubPart(self.b2textid);
      if (self.b3id > 0)
        self.RemoveSubPart(self.b3id);
      if (self.b3textid > 0)
        self.RemoveSubPart(self.b3textid);
      if (self.b1id > 0)
        self.RemoveSubPart(self.b1id);
      if (self.b1textid > 0)
        self.RemoveSubPart(self.b1textid);
      if (self.b6id > 0)
        self.RemoveSubPart(self.b6id);
      if (self.b6textid > 0)
        self.RemoveSubPart(self.b6textid);
      if (self.b4id > 0)
        self.RemoveSubPart(self.b4id);
      if (self.b4textid > 0)
        self.RemoveSubPart(self.b4textid);
      if (self.b5id > 0)
        self.RemoveSubPart(self.b5id);
      if (self.b5textid > 0)
        self.RemoveSubPart(self.b5textid);
      if (self.b7id > 0)
        self.RemoveSubPart(self.b7id);
      if (self.b7textid > 0)
        self.RemoveSubPart(self.b7textid);
      if (self.b16id > 0)
        self.RemoveSubPart(self.b16id);
      if (self.b16textid > 0)
        self.RemoveSubPart(self.b16textid);
      if (self.b14id > 0)
        self.RemoveSubPart(self.b14id);
      if (self.b14textid > 0)
        self.RemoveSubPart(self.b14textid);
      if (self.b17id > 0)
        self.RemoveSubPart(self.b17id);
      if (self.b17textid > 0)
        self.RemoveSubPart(self.b17textid);
      if (self.b18id > 0)
        self.RemoveSubPart(self.b18id);
      if (self.b18textid > 0)
        self.RemoveSubPart(self.b18textid);
      if (self.b20id > 0)
        self.RemoveSubPart(self.b20id);
      if (self.b20textid > 0)
        self.RemoveSubPart(self.b20textid);
      if (self.b21id > 0)
        self.RemoveSubPart(self.b21id);
      if (self.b21textid > 0)
        self.RemoveSubPart(self.b21textid);
      if (self.b22id > 0)
        self.RemoveSubPart(self.b22id);
      if (self.b22textid > 0)
        self.RemoveSubPart(self.b22textid);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 100, -1);
      self.FlagAll();
      if (self.detailnr <= -1)
        return;
      self.ss = "Click to set library";
      str: String = "(No Lib set)";
      if (self.game.Data.StringListObj[self.detailnr].LibId.libSlot > -1)
        str = "(.LibSlot=" + self.game.Data.LibraryObj[self.game.Data.StringListObj[self.detailnr].LibId.libSlot].name + ". LibId=" + self.game.Data.StringListObj[self.detailnr].LibId.id.ToString() + ")";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.b14id = self.AddSubPart( tsubpart1, 910, 90, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Set Lib " + str, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 350, 20, false, tDescript: self.ss);
      self.b14textid = self.AddSubPart( tsubpart2, 950, 89, 350, 20, 0);
      if (self.b11id > 0)
        self.RemoveSubPart(self.b11id);
      if (self.b11textid > 0)
        self.RemoveSubPart(self.b11textid);
      self.ss = "Add a list, thats copied from this one";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
      self.b11id = self.AddSubPart( tsubpart2, 510, 70, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Copy", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b11textid = self.AddSubPart( tsubpart2, 550, 69, 150, 20, 0);
      self.ss = "Replace foranother: String in currently selected stringlist";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.b20id = self.AddSubPart( tsubpart2, 1310, 50, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Replace", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b20textid = self.AddSubPart( tsubpart2, 1350, 49, 150, 20, 0);
      self.ss = "Click to change the name of this StringList";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.BNameId = self.AddSubPart( tsubpart2, 310, 110, 32, 16, 1);
      if (self.detailx > -1 & self.detaily > -1)
      {
        tsubpart2 =  TextPartClass::new("Name: " + self.game.Data.StringListObj[self.detailnr].Name + ", Value: " + self.game.Data.StringListObj[self.detailnr].Data[self.detailx, self.detaily] + " (row:" + self.detaily.ToString() + ",col:" + self.detailx.ToString() + ")", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart2, 350, 109, 400, 20, 0);
      }
      else
      {
        tsubpart2 =  TextPartClass::new("Name: " + self.game.Data.StringListObj[self.detailnr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart2, 350, 109, 400, 20, 0);
      }
      self.ss = "Click to remove this stringlist";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
      self.BRemoveId = self.AddSubPart( tsubpart2, 310, 70, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Remove this Stringlist", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.BRemoveTextId = self.AddSubPart( tsubpart2, 350, 69, 200, 20, 0);
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONPLUS);
      self.b4id = self.AddSubPart( tsubpart2, 510, 160, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Add Row", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
      self.b4textid = self.AddSubPart( tsubpart2, 550, 159, 140, 20, 0);
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONPLUS);
      self.b5id = self.AddSubPart( tsubpart2, 710, 160, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Add Col", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
      self.b5textid = self.AddSubPart( tsubpart2, 750, 159, 140, 20, 0);
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK);
      self.b16id = self.AddSubPart( tsubpart2, 910, 130, 32, 16, 1);
      self.ss = "Click to set description for stringlist. And if it can be edited in simple editor.";
      tsubpart2 =  TextPartClass::new("Descr./Editable =" + self.game.Data.StringListObj[self.detailnr].Editable.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
      self.b16textid = self.AddSubPart( tsubpart2, 950, 129, 140, 20, 0);
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK);
      self.b7id = self.AddSubPart( tsubpart2, 810, 110, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("ID=" + Conversion.Str( self.game.Data.StringListObj[self.detailnr].ID), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
      self.b7textid = self.AddSubPart( tsubpart2, 850, 109, 140, 20, 0);
      self.ss = "Set the column selected to a lookUp Column of a certain stringlist ID";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.b17id = self.AddSubPart( tsubpart2, 1310, 70, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Set Col to LookupCol", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b17textid = self.AddSubPart( tsubpart2, 1350, 69, 150, 20, 0);
      if (self.game.Data.Product == 7)
      {
        self.ss = "n/a";
        if (self.detaily > -1)
          self.ss = self.game.Data.StringListObj[self.detailnr].SSID[self.detaily].ToString();
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b22id = self.AddSubPart( tsubpart2, 1610, 70, 32, 16, 1);
        tsubpart2 =  TextPartClass::new("SSID=" + self.ss, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false);
        self.b22textid = self.AddSubPart( tsubpart2, 1650, 69, 150, 20, 0);
      }
      self.ss = "When referred to by other stringlist use the following columns for ID + Label. ";
      tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.b18id = self.AddSubPart( tsubpart2, 1110, 70, 32, 16, 1);
      tsubpart2 =  TextPartClass::new("Lookup ID/Label", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: self.ss);
      self.b18textid = self.AddSubPart( tsubpart2, 1150, 69, 150, 20, 0);
      let mut num: i32 = 0;
      if (self.game.ScreenHeight > 768)
        num = Math.Max(0,  Math.Round( (self.game.ScreenHeight - 768) / 16.0) - 2);
      tsubpart2 =  new MatrixSubPartClass(self.game.Data.StringListObj[self.detailnr], 20 + num, Math.Min(1600, self.game.ScreenWidth - 324), self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 310, bby: 200);
      self.OptionsListId = self.AddSubPart( tsubpart2, 310, 200, Math.Min(1600, self.game.ScreenWidth - 324), (23 + num) * 16, 0);
      self.MakeTab();
    }

    pub fn MakeTab()
    {
      if (self.b2id > 0)
        self.RemoveSubPart(self.b2id);
      if (self.b2textid > 0)
        self.RemoveSubPart(self.b2textid);
      if (self.b3id > 0)
        self.RemoveSubPart(self.b3id);
      if (self.b3textid > 0)
        self.RemoveSubPart(self.b3textid);
      if (self.b1id > 0)
        self.RemoveSubPart(self.b1id);
      if (self.b1textid > 0)
        self.RemoveSubPart(self.b1textid);
      if (self.b6id > 0)
        self.RemoveSubPart(self.b6id);
      if (self.b6textid > 0)
        self.RemoveSubPart(self.b6textid);
      if (self.b15id > 0)
        self.RemoveSubPart(self.b15id);
      if (self.b15textid > 0)
        self.RemoveSubPart(self.b15textid);
      if (self.b21id > 0)
        self.RemoveSubPart(self.b21id);
      if (self.b21textid > 0)
        self.RemoveSubPart(self.b21textid);
      if (self.detailx > -1)
      {
        let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK);
        self.b1id = self.AddSubPart( tsubpart1, 310, 130, 32, 16, 1);
        let mut tsubpart2: SubPartClass =  TextPartClass::new("Set Value", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
        self.b1textid = self.AddSubPart( tsubpart2, 350, 129, 140, 20, 0);
      }
      if (self.detailx > -1)
      {
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK);
        self.b6id = self.AddSubPart( tsubpart3, 310, 160, 32, 16, 1);
        let mut tsubpart4: SubPartClass =  TextPartClass::new("Name Col", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
        self.b6textid = self.AddSubPart( tsubpart4, 350, 159, 140, 20, 0);
      }
      if (self.detailx > -1)
      {
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(self.game.BUTTONKILL);
        self.b2id = self.AddSubPart( tsubpart5, 510, 130, 32, 16, 1);
        let mut tsubpart6: SubPartClass =  TextPartClass::new("Remove Row", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
        self.b2textid = self.AddSubPart( tsubpart6, 550, 129, 140, 20, 0);
      }
      if (self.detailx > -1 & self.game.Data.StringListObj[self.detailnr].Width > 0 & self.detaily > -1)
      {
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.BUTTONKILL);
        self.b3id = self.AddSubPart( tsubpart7, 710, 130, 32, 16, 1);
        let mut tsubpart8: SubPartClass =  TextPartClass::new("Remove Col", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
        self.b3textid = self.AddSubPart( tsubpart8, 750, 129, 140, 20, 0);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart9: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK);
        self.b15id = self.AddSubPart( tsubpart9, 910, 160, 32, 16, 1);
        let mut tsubpart10: SubPartClass =  TextPartClass::new("Set Col Value Type", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
        self.b15textid = self.AddSubPart( tsubpart10, 950, 159, 140, 20, 0);
      }
      if (self.game.Data.Product != 7 || !(self.detailx > -1 & self.detaily > -1))
        return;
      let mut tsubpart11: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK);
      self.b21id = self.AddSubPart( tsubpart11, 1110, 160, 32, 16, 1);
      let mut tsubpart12: SubPartClass =  TextPartClass::new("LogEnabled=" + self.game.Data.StringListObj[self.detailnr].logEnabled[self.detaily].ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 140, 20, false, tDescript: self.ss);
      self.b21textid = self.AddSubPart( tsubpart12, 1150, 159, 140, 20, 0);
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
            if (num1 == self.ListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.detailnr = num2;
                self.detailx = -1;
                self.detaily = -1;
                self.MakeItem();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.LibListId)
            {
              let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                self.libnr = num3;
                self.detailnr = -1;
                self.MakeList(self.detailnr);
              }
              else
              {
                switch (num3)
                {
                  case -3:
                    self.libnr = -2;
                    self.detailnr = -1;
                    self.MakeList(self.detailnr);
                    break;
                  case -2:
                    self.libnr = -1;
                    self.detailnr = -1;
                    self.MakeList(self.detailnr);
                    break;
                }
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddId)
            {
              self.game.Data.AddStringList();
              self.detailnr = self.game.Data.StringListCounter;
              Form3::new( self.formref).Initialize(self.game.Data, 56, self.game.Data.StringListCounter);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b11id)
            {
              self.game.Data.AddStringList();
              let mut id: i32 = self.game.Data.StringListObj[self.game.Data.StringListCounter].ID;
              self.game.Data.StringListObj[self.game.Data.StringListCounter] = self.game.Data.StringListObj[self.detailnr].Clone();
              self.game.Data.StringListObj[self.game.Data.StringListCounter].ID = id;
              self.detailnr = self.game.Data.StringListCounter;
              self.MakeList(self.detailnr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b10id)
            {
              path: String = self.game.HandyFunctionsObj.LoadSomething("Text file (*.txt)|*.txt", "Pick a textfile. all lines will be added as rows. use , to seperate column values.", self.game.AppPath + "logs\\", false);
              if (File.Exists(path))
              {
                try
                {
                  StreamReader streamReader = File.OpenText(path);
                  while (!streamReader.EndOfStream)
                  {
                    Expression: String = streamReader.ReadLine();
                    if (Strings.Len(Expression) > 0)
                    {
                      self.game.Data.StringListObj[self.detailnr].AddRow(self.game.Data.StringListObj[self.detailnr].Length);
                      strArray: Vec<String> = Expression.Split('\t');
                      let mut index2: i32 = -1;
                      foreach (str: String in strArray)
                      {
                        index2 += 1;
                        if (index2 > self.game.Data.StringListObj[self.detailnr].Width)
                          self.game.Data.StringListObj[self.detailnr].AddCol(self.game.Data.StringListObj[self.detailnr].Width);
                        self.game.Data.StringListObj[self.detailnr].Data[self.game.Data.StringListObj[self.detailnr].Length, index2] = str;
                      }
                    }
                  }
                  streamReader.Close();
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  ProjectData.ClearProjectError();
                }
              }
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BNameId)
            {
              self.game.Data.StringListObj[self.detailnr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeList(self.detailnr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              Coordinate coordinate = self.SubPartList[index1].Click2(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (coordinate.x > -1)
              {
                self.detailx = coordinate.x;
                self.detaily = coordinate.y;
                if (self.detaily > self.game.Data.StringListObj[self.detailnr].Width)
                  self.detaily = self.game.Data.StringListObj[self.detailnr].Width;
                if (self.detailx > self.game.Data.StringListObj[self.detailnr].Length)
                  self.detailx = self.game.Data.StringListObj[self.detailnr].Length;
                self.MakeTab();
                self.RemoveSubPart(self.BNameTextId);
                if (self.detailx > -1 & self.detaily > -1)
                {
                  let mut tsubpart: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.StringListObj[self.detailnr].Name + ", Value: " + self.game.Data.StringListObj[self.detailnr].Data[self.detailx, self.detaily] + " (row:" + self.detailx.ToString() + ",col:" + self.detaily.ToString() + ")", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
                  self.BNameTextId = self.AddSubPart( tsubpart, 350, 109, 400, 20, 0);
                }
                else
                {
                  let mut tsubpart: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.StringListObj[self.detailnr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
                  self.BNameTextId = self.AddSubPart( tsubpart, 350, 109, 400, 20, 0);
                }
                if (self.game.Data.Product == 7)
                {
                  self.RemoveSubPart(self.b22textid);
                  self.ss = "n/a";
                  if (self.detaily > -1)
                    self.ss = self.game.Data.StringListObj[self.detailnr].SSID[self.detaily].ToString();
                  let mut tsubpart: SubPartClass =  TextPartClass::new("SSID=" + self.ss, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false);
                  self.b22textid = self.AddSubPart( tsubpart, 1650, 69, 150, 20, 0);
                }
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b1id)
            {
              self.game.Data.StringListObj[self.detailnr].Data[self.detailx, self.detaily] = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest", self.game.Data.StringListObj[self.detailnr].Data[self.detailx, self.detaily]);
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b2id)
            {
              self.game.Data.StringListObj[self.detailnr].RemoveRow(self.detailx);
              --self.detailx;
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b3id)
            {
              self.game.Data.StringListObj[self.detailnr].RemoveCol(self.detaily);
              --self.detaily;
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b4id)
            {
              self.game.Data.StringListObj[self.detailnr].AddRow(self.detailx);
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b5id)
            {
              self.game.Data.StringListObj[self.detailnr].AddCol(self.detaily);
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b6id)
            {
              self.game.Data.StringListObj[self.detailnr].ColumnName[self.detaily] = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b22id)
            {
              if (self.detaily > -1)
              {
                self.game.Data.StringListObj[self.detailnr].SSID[self.detaily] =  Math.Round(Conversion.Val(Interaction.InputBox("Give new SS-ID value please (0=none)", "Shadow Empire : Planetary Conquest")));
                self.MakeItem();
              }
              else
              {
                let mut num4: i32 =  Interaction.MsgBox( "No column selected");
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b19id)
            {
              lower: String = Interaction.InputBox("Search expression (case insensitive)", "Shadow Empire : Planetary Conquest").ToLower();
              bool[] flagArray = new bool[self.game.Data.StringListCounter + 1];
              let mut stringListCounter1: i32 = self.game.Data.StringListCounter;
              for (let mut index3: i32 = 0; index3 <= stringListCounter1; index3 += 1)
              {
                if (Strings.InStr(self.game.Data.StringListObj[index3].Name.ToLower(), lower) > 0)
                  flagArray[index3] = true;
                let mut length: i32 = self.game.Data.StringListObj[index3].Length;
                for (let mut index4: i32 = 0; index4 <= length; index4 += 1)
                {
                  let mut width: i32 = self.game.Data.StringListObj[index3].Width;
                  for (let mut index5: i32 = 0; index5 <= width; index5 += 1)
                  {
                    if (Strings.InStr(self.game.Data.StringListObj[index3].Data[index4, index5].ToLower(), lower) > 0)
                      flagArray[index3] = true;
                  }
                }
              }
              str: String = "";
              let mut stringListCounter2: i32 = self.game.Data.StringListCounter;
              for (let mut index6: i32 = 0; index6 <= stringListCounter2; index6 += 1)
              {
                if (flagArray[index6])
                {
                  if (str.Length > 0)
                    str += ", ";
                  str = str + self.game.Data.StringListObj[index6].Name + "(" + self.game.Data.StringListObj[index6].ID.ToString() + ")";
                }
              }
              if (Operators.CompareString(str, "", false) == 0)
                str = "No results found";
              let mut num5: i32 =  Interaction.MsgBox( str, Title: ( "Search results"));
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b20id)
            {
              bool flag = false;
              if (Interaction.MsgBox( "Replace in ALL stringlists?", MsgBoxStyle.YesNo,  "Are you sure?") == MsgBoxResult.Yes)
                flag = true;
              str1: String = Interaction.InputBox("Search expression (case sensitive)", "Shadow Empire : Planetary Conquest");
              newValue: String = Interaction.InputBox("Replace expression (case sensitive)", "Shadow Empire : Planetary Conquest");
              let mut num6: i32 = 0;
              bool[] flagArray = new bool[self.game.Data.StringListCounter + 1];
              let mut stringListCounter: i32 = self.game.Data.StringListCounter;
              for (let mut index7: i32 = 0; index7 <= stringListCounter; index7 += 1)
              {
                if (index7 == self.detailnr | flag)
                {
                  let mut length: i32 = self.game.Data.StringListObj[index7].Length;
                  for (let mut index8: i32 = 0; index8 <= length; index8 += 1)
                  {
                    let mut width: i32 = self.game.Data.StringListObj[index7].Width;
                    for (let mut index9: i32 = 0; index9 <= width; index9 += 1)
                    {
                      String1: String = self.game.Data.StringListObj[index7].Data[index8, index9];
                      if (Strings.InStr(String1, str1) > 0)
                      {
                        str2: String = String1.Replace(str1, newValue);
                        self.game.Data.StringListObj[index7].Data[index8, index9] = str2;
                        num6 += 1;
                      }
                    }
                  }
                }
              }
              let mut num7: i32 =  Interaction.MsgBox( (num6.ToString() + " replacements made."), Title: ( "Replace results"));
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b17id)
            {
              let mut num8: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new lookup stringlist ID, please. -1=none. (that stringlist must have lookupId,lookupLabel set) ", "Shadow Empire : Planetary Conquest")));
              if (num8 >= -1 & self.detaily > -1)
                self.game.Data.StringListObj[self.detailnr].LookUpCol[self.detaily] = num8;
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b21id)
            {
              self.game.Data.StringListObj[self.detailnr].logEnabled[self.detaily] = !self.game.Data.StringListObj[self.detailnr].logEnabled[self.detaily];
              return windowReturnClass;
            }
            if (num1 == self.b18id)
            {
              let mut num9: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new column for Lookup ID, -1=none ", "Shadow Empire : Planetary Conquest")));
              if (num9 >= -1)
                self.game.Data.StringListObj[self.detailnr].LookUpId = num9;
              let mut num10: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new column for Lookup Label, -1=none ", "Shadow Empire : Planetary Conquest")));
              if (num10 >= -1)
                self.game.Data.StringListObj[self.detailnr].LookUpLabel = num10;
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b16id)
            {
              if (Interaction.MsgBox( "Is editable in Simple Editor?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                self.game.Data.StringListObj[self.detailnr].Editable = true;
                Form2::new( self.formref).Initialize(self.game.Data, 12, self.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.Data.StringListObj[self.detailnr].Editable = false;
              self.game.Data.StringListObj[self.detailnr].Description = "";
              self.MakeItem();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b15id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 105, self.detailnr, self.detaily);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b14id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 104, self.detailnr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b7id)
            {
              let mut num11: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest")));
              self.game.Data.StringListObj[self.detailnr].ID = num11;
              if (num11 > self.game.Data.StringIDCounter)
                self.game.Data.StringIDCounter = num11;
              self.MakeList(self.detailnr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BRemoveId)
            {
              self.game.Data.RemoveStringList(self.detailnr);
              --self.detailnr;
              self.MakeList(self.detailnr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b12id)
            {
              StreamWriter text = File.CreateText(self.game.AppPath + "logs/" + Interaction.InputBox("Give name of file (writes in log directory, adds .txt automaticly)", "Shadow Empire : Planetary Conquest") + ".txt");
              let mut length: i32 = self.game.Data.StringListObj[self.detailnr].Length;
              for (let mut index10: i32 = 0; index10 <= length; index10 += 1)
              {
                str: String = "";
                let mut width: i32 = self.game.Data.StringListObj[self.detailnr].Width;
                for (let mut index11: i32 = 0; index11 <= width; index11 += 1)
                {
                  if (index11 > 0)
                    str += "\t";
                  str += self.game.Data.StringListObj[self.detailnr].Data[index10, index11];
                }
                text.WriteLine(str);
              }
              text.Close();
              let mut num12: i32 =  Interaction.MsgBox( "finished");
            }
            else
            {
              if (num1 == self.b13id)
              {
                for (self.detailx = self.game.Data.StringListObj[self.detailnr].Length; self.game.Data.StringListObj[self.detailnr].Length > -1 & self.detailx > -1; --self.detailx)
                  self.game.Data.StringListObj[self.detailnr].RemoveRow(self.detailx);
                self.detailx = -1;
                self.detaily = -1;
                self.MakeItem();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b8id)
              {
                str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to a specific stringlist from...", self.game.AppPath + "scenarios\\", false);
                if (File.Exists(str))
                {
                  self.game.HandyFunctionsObj.Unzip(str);
                  dataClass: DataClass = DataClass.deserialize(str);
                  self.game.HandyFunctionsObj.ZipFile(str);
                  InputStr: String = Interaction.InputBox("Give StringList# to import, -1=all");
                  let mut stringListCounter: i32 = dataClass.StringListCounter;
                  for (let mut index12: i32 = 0; index12 <= stringListCounter; index12 += 1)
                  {
                    if ( index12 == Conversion.Val(InputStr) | Conversion.Val(InputStr) == -1.0)
                    {
                      self.game.Data.AddStringList();
                      self.game.Data.StringListObj[self.game.Data.StringListCounter] = dataClass.StringListObj[index12].Clone();
                      if (self.game.Data.StringListObj[self.game.Data.StringListCounter].ID > self.game.Data.StringIDCounter)
                        self.game.Data.StringIDCounter = self.game.Data.StringListObj[self.game.Data.StringListCounter].ID;
                    }
                  }
                  self.detailnr = self.game.Data.StringListCounter;
                  self.MakeList(self.detailnr);
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b9id)
              {
                str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load all stringlists from...", self.game.AppPath + "scenarios\\", false);
                if (File.Exists(str))
                {
                  self.game.HandyFunctionsObj.Unzip(str);
                  dataClass: DataClass = DataClass.deserialize(str);
                  self.game.HandyFunctionsObj.ZipFile(str);
                  let mut stringListCounter: i32 = dataClass.StringListCounter;
                  for (let mut index13: i32 = 0; index13 <= stringListCounter; index13 += 1)
                  {
                    self.game.Data.AddStringList();
                    self.game.Data.StringListObj[self.game.Data.StringListCounter] = dataClass.StringListObj[index13].Clone();
                    if (self.game.Data.StringListObj[self.game.Data.StringListCounter].ID > self.game.Data.StringIDCounter)
                      self.game.Data.StringIDCounter = self.game.Data.StringListObj[self.game.Data.StringListCounter].ID;
                  }
                  self.detailnr = self.game.Data.StringListCounter;
                  self.MakeList(self.detailnr);
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
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
