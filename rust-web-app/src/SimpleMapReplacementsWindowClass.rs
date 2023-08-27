// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleMapReplacementsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleMapReplacementsWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     detailnr: i32;
     tabnr: i32;
     e1id: i32;
     e2id: i32;
     e3id: i32;
     e4id: i32;
     t1id: i32;
     t2id: i32;
     t3id: i32;
     t4id: i32;
     t5id: i32;
     e1idb: i32;
     e2idb: i32;
     e3idb: i32;
     e4idb: i32;
     t1idb: i32;
     t2idb: i32;
     t3idb: i32;
     t4idb: i32;
     t5idb: i32;
     text1id: i32;
     text2id: i32;
     text3id: i32;
     int[] land;
     int[] road;
     int[] river;
     int[] loctype;

    pub SimpleMapReplacementsWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Replacements")
    {
      self.detailnr = -1;
      self.tabnr = 1;
      self.Counters();
      self.DoStuff();
    }

    pub fn PopUpRefresh()
    {
    }

    pub fn DoRefresh()
    {
      self.Counters();
      self.detailnr = -1;
      self.DoStuff();
    }

    pub fn Counters()
    {
      self.land = new int[self.game.Data.LandscapeTypeCounter + 1];
      self.road = new int[self.game.Data.RoadTypeCounter + 1];
      self.river = new int[self.game.Data.RiverTypeCounter + 1];
      self.loctype = new int[self.game.Data.LocTypeCounter + 1];
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType > -1)
          {
            int[] land = self.land;
            int[] numArray = land;
            let mut landscapeType: i32 = self.game.Data.MapObj[0].HexObj[index1, index2].LandscapeType;
            let mut index3: i32 = landscapeType;
            let mut num: i32 = land[landscapeType] + 1;
            numArray[index3] = num;
          }
          let mut index4: i32 = 0;
          do
          {
            if (self.game.Data.MapObj[0].HexObj[index1, index2].RoadType[index4] > -1)
            {
              int[] road = self.road;
              int[] numArray1 = road;
              int[] roadType = self.game.Data.MapObj[0].HexObj[index1, index2].RoadType;
              int[] numArray2 = roadType;
              let mut index5: i32 = index4;
              let mut index6: i32 = index5;
              let mut index7: i32 = numArray2[index6];
              let mut num: i32 = road[roadType[index5]] + 1;
              numArray1[index7] = num;
            }
            if (self.game.Data.MapObj[0].HexObj[index1, index2].RiverType[index4] > -1)
            {
              int[] river = self.river;
              int[] numArray3 = river;
              int[] riverType = self.game.Data.MapObj[0].HexObj[index1, index2].RiverType;
              int[] numArray4 = riverType;
              let mut index8: i32 = index4;
              let mut index9: i32 = index8;
              let mut index10: i32 = numArray4[index9];
              let mut num: i32 = river[riverType[index8]] + 1;
              numArray3[index10] = num;
            }
            index4 += 1;
          }
          while (index4 <= 5);
          if (self.game.Data.MapObj[0].HexObj[index1, index2].Location > -1 && self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[index1, index2].Location].Type > -1)
          {
            int[] loctype = self.loctype;
            int[] numArray = loctype;
            let mut type: i32 = self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[index1, index2].Location].Type;
            let mut index11: i32 = type;
            let mut num: i32 = loctype[type] + 1;
            numArray[index11] = num;
          }
        }
      }
    }

    pub fn DoStuff()
    {
      let mut val1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.listId > 0)
        self.RemoveSubPart(self.listId);
      if (self.e1id > 0)
        self.RemoveSubPart(self.e1id);
      if (self.e2id > 0)
        self.RemoveSubPart(self.e2id);
      if (self.e3id > 0)
        self.RemoveSubPart(self.e3id);
      if (self.e4id > 0)
        self.RemoveSubPart(self.e4id);
      if (self.t1id > 0)
        self.RemoveSubPart(self.t1id);
      if (self.t2id > 0)
        self.RemoveSubPart(self.t2id);
      if (self.t3id > 0)
        self.RemoveSubPart(self.t3id);
      if (self.t4id > 0)
        self.RemoveSubPart(self.t4id);
      if (self.t5id > 0)
        self.RemoveSubPart(self.t5id);
      if (self.e1idb > 0)
        self.RemoveSubPart(self.e1idb);
      if (self.e2idb > 0)
        self.RemoveSubPart(self.e2idb);
      if (self.e3idb > 0)
        self.RemoveSubPart(self.e3idb);
      if (self.e4idb > 0)
        self.RemoveSubPart(self.e4idb);
      if (self.t1idb > 0)
        self.RemoveSubPart(self.t1idb);
      if (self.t2idb > 0)
        self.RemoveSubPart(self.t2idb);
      if (self.t3idb > 0)
        self.RemoveSubPart(self.t3idb);
      if (self.t4idb > 0)
        self.RemoveSubPart(self.t4idb);
      if (self.t5idb > 0)
        self.RemoveSubPart(self.t5idb);
      if (self.text1id > 0)
        self.RemoveSubPart(self.text1id);
      if (self.text2id > 0)
        self.RemoveSubPart(self.text2id);
      if (self.text3id > 0)
        self.RemoveSubPart(self.text3id);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      let mut num1: i32 = 60;
      if (self.tabnr == 1)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Landscapes", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25), bby: num1, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t1idb = self.AddSubPart( tsubpart, val1 + 25, num1 + 70, 140, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Landscapes", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25), bby: num1, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t1id = self.AddSubPart( tsubpart, val1 + 25, num1 + 70, 140, 35, 1);
      }
      if (self.tabnr == 2)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Road Types", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 320), bby: num1, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t2idb = self.AddSubPart( tsubpart, val1 + 25 + 320, num1 + 70, 140, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Road Types", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 320), bby: num1, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t2id = self.AddSubPart( tsubpart, val1 + 25 + 320, num1 + 70, 140, 35, 1);
      }
      if (self.tabnr == 3)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("River Types", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 160), bby: num1, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t3idb = self.AddSubPart( tsubpart, val1 + 25 + 160, num1 + 70, 140, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("River Types", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 160), bby: num1, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t3id = self.AddSubPart( tsubpart, val1 + 25 + 160, num1 + 70, 140, 35, 1);
      }
      if (self.tabnr == 5)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Loc. Types", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 480), bby: num1, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t5idb = self.AddSubPart( tsubpart, val1 + 25 + 480, num1 + 70, 140, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Loc. Types", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 480), bby: num1, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t5id = self.AddSubPart( tsubpart, val1 + 25 + 480, num1 + 70, 140, 35, 1);
      }
      if (self.tabnr == 4)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Operations", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 640), bby: num1, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t4idb = self.AddSubPart( tsubpart, val1 + 25 + 640, num1 + 70, 140, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Operations", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 25 + 640), bby: num1, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.t4id = self.AddSubPart( tsubpart, val1 + 25 + 640, num1 + 70, 140, 35, 1);
      }
      if (self.tabnr == 1)
        DrawMod.DrawTextColouredMarc( objgraphics, "List of landscape types", self.game.MarcFont1, val1 + 25, num1, Color.White);
      if (self.tabnr == 2)
        DrawMod.DrawTextColouredMarc( objgraphics, "List of road types", self.game.MarcFont1, val1 + 25, num1, Color.White);
      if (self.tabnr == 3)
        DrawMod.DrawTextColouredMarc( objgraphics, "List of river types", self.game.MarcFont1, val1 + 25, num1, Color.White);
      if (self.tabnr == 4)
        DrawMod.DrawTextColouredMarc( objgraphics, "Operations", self.game.MarcFont1, val1 + 25, num1, Color.White);
      if (self.tabnr == 5)
        DrawMod.DrawTextColouredMarc( objgraphics, "List of location types", self.game.MarcFont1, val1 + 25, num1, Color.White);
      let mut num2: i32 = num1 + 120;
      if (self.tabnr <= 3 | self.tabnr == 5)
      {
        self.listObj = ListClass::new();
        let mut num3: i32 = -1;
        let mut tlistselect: i32 = -1;
        if (self.tabnr == 1)
        {
          let mut landscapeTypeCounter: i32 = self.game.Data.LandscapeTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= landscapeTypeCounter; tdata += 1)
          {
            if (self.land[tdata] > 0)
            {
              num3 += 1;
              self.listObj.add(self.game.Data.LandscapeTypeObj[tdata].Name, tdata, self.land[tdata].ToString());
              if (self.detailnr == -1)
                self.detailnr = tdata;
              if (self.detailnr == tdata)
                tlistselect = num3;
            }
          }
        }
        if (self.tabnr == 2)
        {
          let mut roadTypeCounter: i32 = self.game.Data.RoadTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= roadTypeCounter; tdata += 1)
          {
            if (self.road[tdata] > 0)
            {
              num3 += 1;
              self.listObj.add(self.game.Data.RoadTypeObj[tdata].Name, tdata, self.road[tdata].ToString());
              if (self.detailnr == -1)
                self.detailnr = tdata;
              if (self.detailnr == tdata)
                tlistselect = num3;
            }
          }
        }
        if (self.tabnr == 3)
        {
          let mut riverTypeCounter: i32 = self.game.Data.RiverTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= riverTypeCounter; tdata += 1)
          {
            if (self.river[tdata] > 0)
            {
              num3 += 1;
              self.listObj.add(self.game.Data.RiverTypeObj[tdata].Name, tdata, self.river[tdata].ToString());
              if (self.detailnr == -1)
                self.detailnr = tdata;
              if (self.detailnr == tdata)
                tlistselect = num3;
            }
          }
        }
        if (self.tabnr == 5)
        {
          let mut locTypeCounter: i32 = self.game.Data.LocTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= locTypeCounter; tdata += 1)
          {
            if (self.loctype[tdata] > 0)
            {
              num3 += 1;
              self.listObj.add(self.game.Data.LocTypeObj[tdata].Name, tdata, self.loctype[tdata].ToString());
              if (self.detailnr == -1)
                self.detailnr = tdata;
              if (self.detailnr == tdata)
                tlistselect = num3;
            }
          }
        }
        let mut tsubpart1: SubPartClass =  new ListSubPartClass(self.listObj, 18, 500 + Math.Max(0, val1 - 50), tlistselect, self.game, true, "Checklist", false, tShowPair: true, tValueWidth: ( Math.Round(260.0 +  val1 * 0.4)), tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (10 + Math.Min(val1, 50)), bby: num2, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
        self.listId = self.AddSubPart( tsubpart1, 10 + Math.Min(val1, 50), num2, 540 + Math.Max(0, val1 - 50), 504, 0);
        if (self.detailnr > -1)
        {
          let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Replace", 140, "Click to select what other type to replace it with",  self.OwnBitmap, val1 + 25 + 550, num2, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.e1id = self.AddSubPart( tsubpart2, val1 + 25 + 550, num2 + 50, 140, 35, 1);
        }
        else
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Replace", 140, "Please select an item in the list",  self.OwnBitmap, val1 + 25 + 550, num2, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.e1idb = self.AddSubPart( tsubpart3, val1 + 25 + 550, num2 + 50, 140, 35, 1);
        }
      }
      if (self.tabnr != 4)
        return;
      let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Reset Location Type Gfx", 340, "Reset the Landscape Type and Sprite of each Hex with Location to its default",  self.OwnBitmap, val1 + 25, num2, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e4id = self.AddSubPart( tsubpart4, val1 + 25, num2 + 50, 340, 35, 1);
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
            if (num1 == self.listId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (num2 > -1 & self.detailnr != num2)
              {
                self.detailnr = num2;
                self.DoStuff();
              }
              self.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.t1id)
            {
              self.tabnr = 1;
              self.detailnr = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.t2id)
            {
              self.tabnr = 2;
              self.detailnr = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.t3id)
            {
              self.tabnr = 3;
              self.detailnr = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.t4id)
            {
              self.tabnr = 4;
              self.detailnr = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.t5id)
            {
              self.tabnr = 5;
              self.detailnr = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e1id)
            {
              Form3 form3 = Form3::new( self.formref);
              if (self.tabnr == 1)
                form3.Initialize(self.game.Data, 147, self.detailnr, tGame: self.game);
              if (self.tabnr == 2)
                form3.Initialize(self.game.Data, 148, self.detailnr, tGame: self.game);
              if (self.tabnr == 3)
                form3.Initialize(self.game.Data, 149, self.detailnr, tGame: self.game);
              if (self.tabnr == 5)
                form3.Initialize(self.game.Data, 151, self.detailnr, tGame: self.game);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e4id)
            {
              let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
              for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
              {
                let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                {
                  let mut location: i32 = self.game.Data.MapObj[0].HexObj[index2, index3].Location;
                  if (location > -1)
                  {
                    let mut type: i32 = self.game.Data.LocObj[location].Type;
                    if (self.game.Data.LocTypeObj[type].OverdrawLTNr > -1)
                    {
                      self.game.Data.MapObj[0].HexObj[index2, index3].LandscapeType = self.game.Data.LocTypeObj[type].OverdrawLTNr;
                      self.game.Data.MapObj[0].HexObj[index2, index3].SpriteNr = self.game.Data.LocTypeObj[type].OverdrawSpriteNr;
                    }
                  }
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
