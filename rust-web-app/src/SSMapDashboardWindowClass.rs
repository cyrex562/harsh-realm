// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SSMapDashboardWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SSMapDashboardWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     loadMapId: i32;
     setdateid: i32;
     exportid: i32;
     exportid2: i32;
     setdescriptid: i32;
     setnameid: i32;
     setdesignid: i32;
     loadMapIdB: i32;
     saveId: i32;
     newId: i32;
     saveid2: i32;
     textId: i32;
     text2id: i32;
     text3id: i32;
     text4id: i32;
     e4id: i32;
     detailnr: i32;
     currentStep: i32;
     loadLayer: i32;
     removeLayer: i32;
     removeLayerB: i32;
     rleft: i32;
     rtop: i32;
     rbottom: i32;
     rright: i32;
     aleft: i32;
     atop: i32;
     abottom: i32;
     aright: i32;
     e1id: i32;
     e2id: i32;
     e3id: i32;

    pub SSMapDashboardWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Map Options")
    {
      self.detailnr = -1;
      self.DoStuff();
    }

    pub fn PopUpRefresh() => self.DoStuff();

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.loadMapId > 0)
        self.RemoveSubPart(self.loadMapId);
      if (self.saveId > 0)
        self.RemoveSubPart(self.saveId);
      if (self.saveid2 > 0)
        self.RemoveSubPart(self.saveid2);
      if (self.newId > 0)
        self.RemoveSubPart(self.newId);
      if (self.textId > 0)
        self.RemoveSubPart(self.textId);
      if (self.text2id > 0)
        self.RemoveSubPart(self.text2id);
      if (self.text3id > 0)
        self.RemoveSubPart(self.text3id);
      if (self.text4id > 0)
        self.RemoveSubPart(self.text4id);
      if (self.e1id > 0)
        self.RemoveSubPart(self.e1id);
      if (self.e2id > 0)
        self.RemoveSubPart(self.e2id);
      if (self.e3id > 0)
        self.RemoveSubPart(self.e3id);
      if (self.e4id > 0)
        self.RemoveSubPart(self.e4id);
      if (self.exportid > 0)
        self.RemoveSubPart(self.exportid);
      if (self.exportid2 > 0)
        self.RemoveSubPart(self.exportid2);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      let mut y1: i32 = 80;
      tText1: String = "This is the Story & Stratagem (S&S) editor. Welcome! Check the PDF S&S Guide on how to make a so called 'library' with this editor.";
      DrawMod.DrawTextColouredMarc( objgraphics, "S&S Dashboard", self.game.MarcFont1, num1 + 25, y1, Color.White);
      let mut num2: i32 = y1 + 0;
      let mut tsubpart: SubPartClass =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText1, 27,  self.OwnBitmap, num1 + 10, num2, true, true);
      self.textId = self.AddSubPart( tsubpart, num1 + 10, num2, 450, 108, 0);
      let mut y2: i32 = num2 + 120;
      tText2: String = "Click to load or save an S&S library file.";
      DrawMod.DrawTextColouredMarc( objgraphics, "File Ops", self.game.MarcFont1, num1 + 25, y2, Color.White);
      let mut num3: i32 = y2 + 0;
      tsubpart =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText2, 27,  self.OwnBitmap, num1 + 10, num3, true, true);
      self.text2id = self.AddSubPart( tsubpart, num1 + 10, num3, 450, 108, 0);
      let mut num4: i32 = num3 + 70;
      tsubpart =  new TextButtonPartClass("Load Lib", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadMapId = self.AddSubPart( tsubpart, num1 + 25, num4, 140, 35, 1);
      if (Strings.InStr(self.game.Data.LibraryObj[0].name.ToLower(), "default") > 0)
      {
        tsubpart =  new TextButtonPartClass("Save Lib", 140, "You cannot save if the word 'default' is present in the library name.",  self.OwnBitmap, num1 + 25 + 160, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveid2 = self.AddSubPart( tsubpart, num1 + 25 + 160, num4, 140, 35, 0);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Save Lib", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 160), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveId = self.AddSubPart( tsubpart, num1 + 25 + 160, num4, 140, 35, 1);
      }
      tsubpart =  new TextButtonPartClass("Clear Lib", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 320), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.newId = self.AddSubPart( tsubpart, num1 + 25 + 320, num4, 140, 35, 1);
      let mut y3: i32 = num4 + 70;
      creator: String = self.game.Data.LibraryObj[0].creator;
      DrawMod.DrawTextColouredMarc( objgraphics, "Designer", self.game.MarcFont1, num1 + 25, y3, Color.White);
      tsubpart =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, creator, 27,  self.OwnBitmap, num1 + 10, y3 - 5, true, true);
      self.textId = self.AddSubPart( tsubpart, num1 + 10, y3 - 5, 250, 108, 0);
      tsubpart =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: (y3 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e1id = self.AddSubPart( tsubpart, num1 + 25, y3 + 70, 140, 35, 1);
      tText3: String = self.game.Data.LibraryObj[0].version.ToString();
      DrawMod.DrawTextColouredMarc( objgraphics, "Library Version", self.game.MarcFont1, num1 + 25 + 250, y3, Color.White);
      tsubpart =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText3, 27,  self.OwnBitmap, num1 + 10 + 250, y3 - 5, true, true);
      self.text2id = self.AddSubPart( tsubpart, num1 + 10 + 250, y3 - 5, 250, 108, 0);
      tsubpart =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 250), bby: (y3 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e3id = self.AddSubPart( tsubpart, num1 + 25 + 250, y3 + 70, 140, 35, 1);
      name: String = self.game.Data.LibraryObj[0].name;
      DrawMod.DrawTextColouredMarc( objgraphics, "Library Name", self.game.MarcFont1, num1 + 25 + 500, y3, Color.White);
      tsubpart =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, name, 27,  self.OwnBitmap, num1 + 10 + 500, y3 - 5, true, true);
      self.text3id = self.AddSubPart( tsubpart, num1 + 10 + 500, y3 - 5, 550, 108, 0);
      tsubpart =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 500), bby: (y3 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e2id = self.AddSubPart( tsubpart, num1 + 25 + 500, y3 + 70, 140, 35, 1);
      tText4: String = Strings.Left(self.game.Data.LibraryObj[0].information, 15);
      if (self.game.Data.LibraryObj[0].information.Length > 15)
        tText4 += "...";
      if (tText4.Length < 1)
        tText4 = "-Not yet given-";
      DrawMod.DrawTextColouredMarc( objgraphics, "Library Description", self.game.MarcFont1, num1 + 25 + 750, y3, Color.White);
      tsubpart =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText4, 27,  self.OwnBitmap, num1 + 10 + 750, y3 - 5, true, true);
      self.text4id = self.AddSubPart( tsubpart, num1 + 10 + 750, y3 - 5, 550, 108, 0);
      tsubpart =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 750), bby: (y3 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e4id = self.AddSubPart( tsubpart, num1 + 25 + 750, y3 + 70, 140, 35, 1);
      let mut y4: i32 = y3 + 160;
      tText5: String = "By using the export function you can create a se1zip file that can be easily imported by the end user of your Mod Library.";
      DrawMod.DrawTextColouredMarc( objgraphics, "Exporting", self.game.MarcFont1, num1 + 25, y4, Color.White);
      let mut num5: i32 = y4 + 0;
      tsubpart =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText5, 27,  self.OwnBitmap, num1 + 10, num5, true, true);
      self.textId = self.AddSubPart( tsubpart, num1 + 10, num5, 450, 108, 0);
      let mut num6: i32 = num5 + 100;
      if (Strings.InStr(self.game.Data.LibraryObj[0].name.ToLower(), "default") > 0)
      {
        tsubpart =  new TextButtonPartClass("Export Se1zip", 240, "You still need to rename your Mod Library",  self.OwnBitmap, num1 + 25, num6, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.exportid2 = self.AddSubPart( tsubpart, num1 + 25, num6, 240, 35, 0);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Export Se1zip", 240, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.exportid = self.AddSubPart( tsubpart, num1 + 25, num6, 240, 35, 1);
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
            if (num1 == self.e4id)
            {
              Form2::new( self.formref).Initialize(self.game.Data, 13, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e1id)
            {
              self.game.Data.LibraryObj[0].creator = Interaction.InputBox("Give designer name", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].creator);
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e2id)
            {
              str: String = Interaction.InputBox("Give library name", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].name);
              let mut num2: i32 = 0;
              if (Strings.InStr(str.ToLower(), "default") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "random") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "system") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "shadow") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "big") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "small") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "se_") > 0)
                num2 = 1;
              if (Strings.InStr(str.ToLower(), "_v") > 0)
                num2 = 1;
              if (num2 == 0)
              {
                self.game.Data.LibraryObj[0].name = str;
              }
              else
              {
                let mut num3: i32 =  Interaction.MsgBox( "You are not allowed to use the following reserved words: default, _v, random, system, shadow, big, small, se_", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e3id)
            {
              let mut num4: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give library version number", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].version.ToString())));
              if (num4 > 0)
              {
                self.game.Data.LibraryObj[0].version = num4;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.loadMapId)
              {
                path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Event library(*.se1evlib)|*.se1evlib", "Pick Mod Library to load...", self.game.AppPath + self.game.ModScenarioDir, false);
                if (File.Exists(path))
                {
                  self.game.EditObj.LoadFileName = path;
                  self.game.EditObj.PopupValue = 19;
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num5: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.newId)
              {
                path: String = self.game.AppPath + self.game.ModScenarioDir + "/default.se1evlib";
                if (File.Exists(path))
                {
                  self.game.EditObj.LoadFileName = path;
                  self.game.EditObj.PopupValue = 19;
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num6: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.exportid)
              {
                self.game.HandyFunctionsObj.ExportModLibraryEditor(self.game.Data.LibraryObj[0].name);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.saveId)
              {
                self.game.Data.MasterFile = "";
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
                str1: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Event library(*.se1evlib)|*.se1evlib", "Give save name...", tinitdir, false);
                if (Strings.Len(str1) < 2)
                {
                  let mut num7: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  self.game.FormRef.Cursor = Cursors.WaitCursor;
                  self.game.Data.serialize(str1);
                  self.game.HandyFunctionsObj.ZipFile(str1);
                  windowReturnClass.SetFlag(true);
                  self.game.FormRef.Cursor = Cursors.Default;
                  self.game.Data.LoadGraphics(self.formref);
                  self.game.Data.PermanentOverlaySpriteID = -1;
                  self.game.Data.PermanentOverlayUse = false;
                  path1: String = self.game.AppPath + "graphics\\" + self.game.Data.LibraryObj[0].name.ToLower();
                  if (!Directory.Exists(path1))
                    Directory.CreateDirectory(path1);
                  path2: String = self.game.AppPath + "graphics\\" + self.game.Data.LibraryObj[0].name.ToLower() + "BIG";
                  if (!Directory.Exists(path2))
                    Directory.CreateDirectory(path2);
                  path3: String = self.game.AppPath + "graphics\\" + self.game.Data.LibraryObj[0].name.ToLower() + "SMALL";
                  if (!Directory.Exists(path3))
                    Directory.CreateDirectory(path3);
                  str2: String = self.game.AppPath + "graphics\\";
                  let mut num8: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
                }
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
