// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFTypeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SFTypeWindowClass : WindowClass
  {
     SFtypeListId: i32;
     ListClass SFtypeListObj;
     LibListId: i32;
     LibNr: i32;
     ListClass LibListObj;
     BAddSFtypeId: i32;
     BAddSFtypeTextId: i32;
     BAddSFtype2Id: i32;
     BAddSFtypeText2Id: i32;
     BNameId: i32;
     BNameTextId: i32;
     BRemoveSFtypeId: i32;
     BRemoveSFtypeTextId: i32;
     BRemoveSFtypeId2: i32;
     BRemoveSFtypeTextId2: i32;
     ListClass TabListObj;
     TabListId: i32;
     BSymbolId: i32;
     BChangeSymbolId: i32;
     BSymbol2Id: i32;
     BChangeSymbol2Id: i32;
     BPicId: i32;
     bChangePicId: i32;
     BSymbolGroupId: i32;
     BSymbolGroupTextId: i32;
     BSymbolWeightId: i32;
     BSymbolWeightTextId: i32;
     BSymbolOverRuleId: i32;
     BSymbolOverRuleTextId: i32;
     BMoveTypeId: i32;
     BMoveTypeTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     B7Id: i32;
     B7TextId: i32;
     B8Id: i32;
     B8TextId: i32;
     b9id: i32;
     b9textid: i32;
     b18id: i32;
     b18textid: i32;
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
     b19id: i32;
     b19textid: i32;
     b20id: i32;
     b20textid: i32;
     b21id: i32;
     b21textid: i32;
     b22id: i32;
     b22textid: i32;
     b23id: i32;
     b23textid: i32;
     b24id: i32;
     b24textid: i32;
     b25id: i32;
     b25textid: i32;
     b26id: i32;
     b26textid: i32;
     b27id: i32;
     b27textid: i32;
     b28id: i32;
     b28textid: i32;
     b29id: i32;
     b29textid: i32;
     b30id: i32;
     b30textid: i32;
     b31id: i32;
     b31textid: i32;
     b32id: i32;
     b32textid: i32;
     b33id: i32;
     b33textid: i32;
     b34id: i32;
     b34textid: i32;
     b35id: i32;
     b35textid: i32;
     b36id: i32;
     b36textid: i32;
     b37id: i32;
     b37textid: i32;
     b38id: i32;
     b38textid: i32;
     b39id: i32;
     b39textid: i32;
     a1id: i32;
     a1textid: i32;
     a2id: i32;
     a2textid: i32;
     a3id: i32;
     a3textid: i32;
     a4id: i32;
     a4textid: i32;
     a5id: i32;
     a5textid: i32;
     a6id: i32;
     a6textid: i32;
     c1id: i32;
     c1textid: i32;
     c2id: i32;
     c2textid: i32;
     c3id: i32;
     c3textid: i32;
     c4id: i32;
     c4textid: i32;
     c5id: i32;
     c5textid: i32;
     c6id: i32;
     c6textid: i32;
     c7id: i32;
     c7textid: i32;
     c8id: i32;
     c8textid: i32;
     c11id: i32;
     c11textid: i32;
     c12id: i32;
     c12textid: i32;
     c13id: i32;
     c13textid: i32;
     c14id: i32;
     c14textid: i32;
     c15id: i32;
     c15textid: i32;
     c16id: i32;
     c16textid: i32;
     c17id: i32;
     c17textid: i32;
     c18id: i32;
     c18textid: i32;
     c19id: i32;
     c19textid: i32;
     c20id: i32;
     c20textid: i32;
     c21id: i32;
     c21textid: i32;
     d1id: i32;
     d1textid: i32;
     e1id: i32;
     e1textid: i32;
     e2id: i32;
     e2textid: i32;
     e3id: i32;
     e3textid: i32;
     e4id: i32;
     e4textid: i32;
     e5id: i32;
     e5textid: i32;
     e6id: i32;
     e6textid: i32;
     e7id: i32;
     e7textid: i32;
     f1id: i32;
     f1textid: i32;
     f2id: i32;
     f2textid: i32;
     f3id: i32;
     f3textid: i32;
     g1id: i32;
     g1textid: i32;
     g2id: i32;
     g2textid: i32;
     g3id: i32;
     g3textid: i32;
     g4id: i32;
     g4textid: i32;
     g5id: i32;
     g5textid: i32;
     g6id: i32;
     g6textid: i32;
     g7id: i32;
     g7textid: i32;
     g8id: i32;
     g8textid: i32;
     g9id: i32;
     g9textid: i32;
     g10id: i32;
     g10textid: i32;
     g11id: i32;
     g11textid: i32;
     g12id: i32;
     g12textid: i32;
     g13id: i32;
     g13textid: i32;
     g14id: i32;
     g14textid: i32;
     g15id: i32;
     g15textid: i32;
     g16id: i32;
     g16textid: i32;
     g17id: i32;
     g17textid: i32;
     g18id: i32;
     g18textid: i32;
     g19id: i32;
     g19textid: i32;
     g20id: i32;
     g20textid: i32;
     g21id: i32;
     g21textid: i32;
     g22id: i32;
     g22textid: i32;
     g23id: i32;
     g23textid: i32;
     g24id: i32;
     g24textid: i32;
     h1id: i32;
     h1textid: i32;
     h2id: i32;
     h2textid: i32;
     h3id: i32;
     h3textid: i32;
     h4id: i32;
     h4textid: i32;
     h5id: i32;
     h5textid: i32;
     h6id: i32;
     h6textid: i32;
     p1id: i32;
     p1textid: i32;
     p2id: i32;
     p2textid: i32;
     p3id: i32;
     p3textid: i32;
     p4id: i32;
     p4textid: i32;
     p5id: i32;
     p5textid: i32;
     p6id: i32;
     p6textid: i32;
     p7id: i32;
     p7textid: i32;
     p8id: i32;
     p8textid: i32;
     p9id: i32;
     p9textid: i32;
     vp1id: i32;
     vp1textid: i32;
     vp2id: i32;
     vp2textid: i32;
     vp3id: i32;
     vp3textid: i32;
     vp4id: i32;
     vp4textid: i32;
     vp5id: i32;
     vp5textid: i32;
     vp6id: i32;
     vp6textid: i32;
     t1id: i32;
     t1textid: i32;
     w1id: i32;
     w1textid: i32;
     w1bid: i32;
     w1btextid: i32;
     w2id: i32;
     w2textid: i32;
     w2bid: i32;
     w2btextid: i32;
     w3id: i32;
     w3textid: i32;
     w133id: i32;
     w133textid: i32;
     w4id: i32;
     w4textid: i32;
     w5id: i32;
     w5textid: i32;
     w6id: i32;
     w6textid: i32;
     x1id: i32;
     x1textid: i32;
     x2id: i32;
     x2textid: i32;
     x3id: i32;
     x3textid: i32;
     x4id: i32;
     x4textid: i32;
     x5id: i32;
     x5textid: i32;
     x6id: i32;
     x6textid: i32;
     w7id: i32;
     w7textid: i32;
     w8id: i32;
     w8textid: i32;
     w9id: i32;
     w9textid: i32;
     w9bid: i32;
     w9btextid: i32;
     v1id: i32;
     v1textid: i32;
     v2id: i32;
     v2textid: i32;
     j1id: i32;
     j1textid: i32;
     j2id: i32;
     j2textid: i32;
     v3id: i32;
     v3textid: i32;
     v4id: i32;
     v4textid: i32;
     v5id: i32;
     v5textid: i32;
     v6id: i32;
     v6textid: i32;
     v7id: i32;
     v7textid: i32;
     v8id: i32;
     v8textid: i32;
     v9id: i32;
     v9textid: i32;
     copyid: i32;
     copytextid: i32;
     v10id: i32;
     v10textid: i32;
     v11id: i32;
     v11textid: i32;
     v12id: i32;
     v12textid: i32;
     v13id: i32;
     v13textid: i32;
     v14id: i32;
     v14textid: i32;
     v15id: i32;
     v15textid: i32;
     v16id: i32;
     v16textid: i32;
     v17id: i32;
     v17textid: i32;
     v18id: i32;
     v18textid: i32;
     v19id: i32;
     v19textid: i32;
     v20id: i32;
     v20textid: i32;
     v21id: i32;
     v21textid: i32;
     v22id: i32;
     v22textid: i32;
     v23id: i32;
     v23textid: i32;
     w10id: i32;
     w10textid: i32;
     w11id: i32;
     w11textid: i32;
     w12id: i32;
     w12textid: i32;
     w13id: i32;
     w13textid: i32;
     w14id: i32;
     w14textid: i32;
     w15id: i32;
     w15textid: i32;
     w16id: i32;
     w16textid: i32;
     w17id: i32;
     w17textid: i32;
     y1id: i32;
     y1textid: i32;
     y2id: i32;
     y2textid: i32;
     y3id: i32;
     y3textid: i32;
     y4id: i32;
     y5id: i32;
     clibid: i32;
     clibtextid: i32;
     y6id: i32;
     y6textid: i32;
     y7id: i32;
     y7btextid: i32;
     y7textid: i32;
     y8id: i32;
     y8btextid: i32;
     y8textid: i32;
     PGListId: i32;
     ExtraListId: i32;
     LogoListId: i32;
     PreventListId: i32;
     VariantListId: i32;
     ListClass PGListObj;
     ListClass ExtraListObj;
     ListClass LogoListObj;
     ListClass PreventListObj;
     ListClass VariantListObj;
     CombatListId: i32;
     combatlist3id: i32;
     combatlist4id: i32;
     ListClass CombatListObj;
     CombatList2Id: i32;
     ResListId: i32;
     ListClass CombatList2Obj;
     ListClass CombatList3Obj;
     ListClass CombatList4Obj;
     ListClass ResListObj;
     SFtypeNr: i32;
     TabSheetNr: i32;
     detailnr2: i32;
     detailnr: i32;
     ss: String;

    pub SFTypeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Subformation Types")
    {
      self.SFtypeNr = -1;
      self.LibNr = -1;
      self.MakeSFtypeListGUI(-1);
      self.TabSheetNr = -1;
      self.detailnr = -1;
      self.detailnr2 = -1;
      tGame.EditObj.TempSelected = -1;
      tGame.EditObj.TempCopy = -1;
    }

    pub fn DoRefresh()
    {
      if (self.game.EditObj.TempSelected > -1)
      {
        self.SFtypeNr = self.game.EditObj.TempSelected;
        self.detailnr = self.SFtypeNr;
        self.game.EditObj.TempSelected = -1;
      }
      if (self.game.EditObj.TempCopy > -1)
      {
        SFTypeClass sfTypeClass = self.game.Data.SFTypeObj[self.SFtypeNr].Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr] = self.game.Data.SFTypeObj[self.game.EditObj.TempCopy].Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].Name = sfTypeClass.Name;
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraCounter = sfTypeClass.ExtraCounter;
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraCode = (int[]) sfTypeClass.ExtraCode.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraName = (string[]) sfTypeClass.ExtraName.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraPicFileName = (string[]) sfTypeClass.ExtraPicFileName.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSidewaysFileName = (string[]) sfTypeClass.ExtraSidewaysFileName.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolColBigFileName = (string[]) sfTypeClass.ExtraSymbolColBigFileName.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolColBigFileName2 = (string[]) sfTypeClass.ExtraSymbolColBigFileName2.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolFileName = (string[]) sfTypeClass.ExtraSymbolFileName.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolFileName2 = (string[]) sfTypeClass.ExtraSymbolFileName2.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraPicSpriteID = (int[]) sfTypeClass.ExtraPicSpriteID.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSidewaysSpriteID = (int[]) sfTypeClass.ExtraSidewaysSpriteID.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolColBigSprite2ID = (int[]) sfTypeClass.ExtraSymbolColBigSprite2ID.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolColBigSpriteID = (int[]) sfTypeClass.ExtraSymbolColBigSpriteID.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolSprite2ID = (int[]) sfTypeClass.ExtraSymbolSprite2ID.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].ExtraSymbolSpriteID = (int[]) sfTypeClass.ExtraSymbolSpriteID.Clone();
        self.game.Data.SFTypeObj[self.SFtypeNr].PicFileName = sfTypeClass.PicFileName;
        self.game.Data.SFTypeObj[self.SFtypeNr].SidewaysFileName = sfTypeClass.SidewaysFileName;
        self.game.Data.SFTypeObj[self.SFtypeNr].SymbolColBigFileName = sfTypeClass.SymbolColBigFileName;
        self.game.Data.SFTypeObj[self.SFtypeNr].SymbolColBigFileName2 = sfTypeClass.SymbolColBigFileName2;
        self.game.Data.SFTypeObj[self.SFtypeNr].SymbolFileName = sfTypeClass.SymbolFileName;
        self.game.Data.SFTypeObj[self.SFtypeNr].SymbolFileName2 = sfTypeClass.SymbolFileName2;
        self.game.Data.SFTypeObj[self.SFtypeNr].Id = sfTypeClass.Id;
        self.game.Data.SFTypeObj[self.SFtypeNr].LoadSprites();
        self.game.EditObj.TempCopy = -1;
      }
      self.MakeSFtypeListGUI(self.SFtypeNr);
      self.MakeSFtypeTypeItemGUI();
    }

     void MakeSFtypeListGUI(tSFtypenr: i32)
    {
      if (self.SFtypeListId > 0)
        self.RemoveSubPart(self.SFtypeListId);
      self.LibListObj = ListClass::new();
      self.LibListObj.add("All", -2);
      let mut num1: i32 = -1;
      let mut num2: i32 = 0;
      let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
      for (let mut index: i32 = 0; index <= libraryCounter; index += 1)
      {
        num2 += 1;
        if (self.LibNr == index)
          num1 = num2;
        self.LibListObj.add(Conversion.Str( index) + ") " + self.game.Data.LibraryObj[index].name, index);
      }
      if (self.LibNr == -1)
        num1 = 0;
      ListClass libListObj = self.LibListObj;
      let mut tlistselect1: i32 = num1;
      let mut game1: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 9, 200, tlistselect1, game1, tHeader: "Libraries", tbackbitmap: ( local1), bbx: 10, bby: 38, overruleFont: ( local2));
      self.LibListId = self.AddSubPart( tsubpart1, 10, 38, 200, 192, 0);
      self.MakeSFtypeTypeItemGUI();
      let mut num3: i32 = -1;
      let mut num4: i32 = -1;
      if (self.game.Data.SFTypeCounter > -1)
      {
        self.SFtypeListObj = ListClass::new();
        let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
        for (let mut index: i32 = 0; index <= sfTypeCounter; index += 1)
        {
          if (self.LibNr == -1 | self.LibNr == self.game.Data.SFTypeObj[index].LibId.libSlot)
          {
            num4 += 1;
            if (index == tSFtypenr)
              num3 = num4;
            self.SFtypeListObj.add(Conversion.Str( index) + ") " + self.game.Data.SFTypeObj[index].Name + "(id=" + self.game.Data.SFTypeObj[index].Id.ToString() + ")", index);
          }
        }
        ListClass sftypeListObj = self.SFtypeListObj;
        let mut tlistselect2: i32 = num3;
        let mut game2: GameClass = self.game;
         local3: Bitmap =  self.OwnBitmap;
        font =  null;
         local4: Font =  font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(sftypeListObj, 9, 200, tlistselect2, game2, tHeader: "SFTypes", tbackbitmap: ( local3), bbx: 220, bby: 38, overruleFont: ( local4));
        self.SFtypeListId = self.AddSubPart( tsubpart2, 220, 38, 200, 192, 0);
        self.SFtypeNr = tSFtypenr;
        self.MakeSFtypeTypeItemGUI();
      }
      else
      {
        self.SFtypeNr = tSFtypenr;
        self.MakeSFtypeTypeItemGUI();
      }
      if (self.BAddSFtypeId > 0)
        self.RemoveSubPart(self.BAddSFtypeId);
      if (self.BAddSFtypeTextId > 0)
        self.RemoveSubPart(self.BAddSFtypeTextId);
      if (self.BAddSFtype2Id > 0)
        self.RemoveSubPart(self.BAddSFtype2Id);
      if (self.BAddSFtypeText2Id > 0)
        self.RemoveSubPart(self.BAddSFtypeText2Id);
      self.ss = "Add a new SFType. Will be added at the end of the list.";
      SubPartClass tsubpart3;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.BAddSFtypeId = self.AddSubPart( tsubpart3, 10, 270, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  TextPartClass::new("Add SFtype Type", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 170, 20, false, tDescript: self.ss);
        self.BAddSFtypeTextId = self.AddSubPart( tsubpart3, 50, 269, 170, 20, 0);
      }
      if (self.SFtypeNr <= -1)
        return;
      self.ss = "Add a copy of the currently selected SFType.. will be added at the end of the list.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.BAddSFtype2Id = self.AddSubPart( tsubpart3, 10, 250, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      tsubpart3 =  TextPartClass::new("Add SFtype Copy", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 170, 20, false, tDescript: self.ss);
      self.BAddSFtypeText2Id = self.AddSubPart( tsubpart3, 50, 249, 170, 20, 0);
    }

     void MakeSFtypeTypeItemGUI()
    {
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
      if (self.BRemoveSFtypeId > 0)
        self.RemoveSubPart(self.BRemoveSFtypeId);
      if (self.BRemoveSFtypeTextId > 0)
        self.RemoveSubPart(self.BRemoveSFtypeTextId);
      if (self.BRemoveSFtypeId2 > 0)
        self.RemoveSubPart(self.BRemoveSFtypeId2);
      if (self.BRemoveSFtypeTextId2 > 0)
        self.RemoveSubPart(self.BRemoveSFtypeTextId2);
      if (self.TabListId > 0)
        self.RemoveSubPart(self.TabListId);
      if (self.g22id > 0)
        self.RemoveSubPart(self.g22id);
      if (self.g22textid > 0)
        self.RemoveSubPart(self.g22textid);
      if (self.x1id > 0)
        self.RemoveSubPart(self.x1id);
      if (self.x1textid > 0)
        self.RemoveSubPart(self.x1textid);
      if (self.x2id > 0)
        self.RemoveSubPart(self.x2id);
      if (self.x2textid > 0)
        self.RemoveSubPart(self.x2textid);
      if (self.x3id > 0)
        self.RemoveSubPart(self.x3id);
      if (self.x3textid > 0)
        self.RemoveSubPart(self.x3textid);
      if (self.x4id > 0)
        self.RemoveSubPart(self.x4id);
      if (self.x4textid > 0)
        self.RemoveSubPart(self.x4textid);
      if (self.x5id > 0)
        self.RemoveSubPart(self.x5id);
      if (self.x5textid > 0)
        self.RemoveSubPart(self.x5textid);
      if (self.x6id > 0)
        self.RemoveSubPart(self.x6id);
      if (self.x6textid > 0)
        self.RemoveSubPart(self.x6textid);
      if (self.clibid > 0)
        self.RemoveSubPart(self.clibid);
      if (self.clibtextid > 0)
        self.RemoveSubPart(self.clibtextid);
      if (self.SFtypeNr > -1)
      {
        if (self.SFtypeNr < self.game.Data.SFTypeCounter)
        {
          self.ss = "Move up";
          if (Strings.Len(self.game.Data.MasterFile) == 0)
          {
            let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONDOWN, tDescript: self.ss);
            self.x4id = self.AddSubPart( tsubpart, 250, 290, 32, 16, 1);
          }
        }
        if (self.SFtypeNr > 0)
        {
          self.ss = "Move down";
          if (Strings.Len(self.game.Data.MasterFile) == 0)
          {
            let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONUP, tDescript: self.ss);
            self.x5id = self.AddSubPart( tsubpart, 300, 290, 32, 16, 1);
          }
        }
        self.ss = "Replace all SFs with this Type with another SFType";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
          self.x6id = self.AddSubPart( tsubpart, 250, 230, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  TextPartClass::new("Replace all instances", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
          self.x6textid = self.AddSubPart( tsubpart, 290, 229, 200, 20, 0);
        }
        self.ss = "Set Library for this SfType";
        txt: String = "Set Lib (.LibSlot=" + self.game.Data.SFTypeObj[self.SFtypeNr].LibId.libSlot.ToString() + ".LibId=" + self.game.Data.SFTypeObj[self.SFtypeNr].LibId.id.ToString() + ")";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
          self.clibid = self.AddSubPart( tsubpart, 500, 230, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  TextPartClass::new(txt, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
          self.clibtextid = self.AddSubPart( tsubpart, 550, 229, 300, 20, 0);
        }
        self.ss = "Add a new SFType";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
          self.x1id = self.AddSubPart( tsubpart, 250, 270, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  TextPartClass::new("Select SFType to View", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
          self.x1textid = self.AddSubPart( tsubpart, 290, 269, 200, 20, 0);
        }
        self.ss = "Copy the stats from a selected other SFType";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
          self.x2id = self.AddSubPart( tsubpart, 250, 250, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  TextPartClass::new("Copy Stats from other SFType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
          self.x2textid = self.AddSubPart( tsubpart, 290, 249, 200, 20, 0);
        }
        self.ss = "Click to change the name of this SFType";
        if (!Information.IsNothing( self.game.Data.SFTypeObj[self.SFtypeNr].LibId))
          self.ss = self.ss + " lib: LibSlot: " + self.game.Data.SFTypeObj[self.SFtypeNr].LibId.libSlot.ToString() + ", id: " + self.game.Data.SFTypeObj[self.SFtypeNr].LibId.id.ToString();
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.BNameId = self.AddSubPart( tsubpart, 10, 230, 32, 16, 1);
        }
        let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.SFTypeObj[self.SFtypeNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 180, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart1, 50, 230, 400, 20, 0);
        self.ss = "Click to remove this SFType from the list";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
          self.BRemoveSFtypeId = self.AddSubPart( tsubpart2, 10, 290, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart3: SubPartClass =  TextPartClass::new("Remove this SFType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
          self.BRemoveSFtypeTextId = self.AddSubPart( tsubpart3, 50, 289, 200, 20, 0);
        }
        self.ss = "Click All SFTypes from the list";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
          self.BRemoveSFtypeId2 = self.AddSubPart( tsubpart4, 10, 310, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart5: SubPartClass =  TextPartClass::new("Remove ALL", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
          self.BRemoveSFtypeTextId2 = self.AddSubPart( tsubpart5, 50, 309, 200, 20, 0);
        }
        self.TabListObj = ListClass::new();
        let mut num1: i32 = -1;
        self.TabListObj.add("Graphics", 0);
        let mut num2: i32 = num1 + 1;
        num3: i32;
        if (self.TabSheetNr == 0)
          num3 = num2;
        self.TabListObj.add("Statistics 1", 1);
        let mut num4: i32 = num2 + 1;
        if (self.TabSheetNr == 1)
          num3 = num4;
        self.TabListObj.add("Statistics 2", 2);
        let mut num5: i32 = num4 + 1;
        if (self.TabSheetNr == 2)
          num3 = num5;
        self.TabListObj.add("Combat Detail Stats", 3);
        let mut num6: i32 = num5 + 1;
        if (self.TabSheetNr == 3)
          num3 = num6;
        self.TabListObj.add("Combat Landscape Mods", 4);
        let mut num7: i32 = num6 + 1;
        if (self.TabSheetNr == 4)
          num3 = num7;
        self.TabListObj.add("AI Role Scores", 6);
        let mut num8: i32 = num7 + 1;
        if (self.TabSheetNr == 6)
          num3 = num8;
        self.TabListObj.add("Logo Values", 7);
        let mut num9: i32 = num8 + 1;
        if (self.TabSheetNr == 7)
          num3 = num9;
        self.TabListObj.add("Prevent List", 8);
        let mut num10: i32 = num9 + 1;
        if (self.TabSheetNr == 8)
          num3 = num10;
        self.TabListObj.add("Fuel + Stockpile + Adv.Supply", 9);
        let mut num11: i32 = num10 + 1;
        if (self.TabSheetNr == 9)
          num3 = num11;
        ListClass tabListObj = self.TabListObj;
        let mut tlistselect: i32 = num3;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart6: SubPartClass =  new ListSubPartClass(tabListObj, 9, 200, tlistselect, game, tHeader: "Propery Sheets", tbackbitmap: ( local1), bbx: 430, bby: 38, overruleFont: ( local2));
        self.TabListId = self.AddSubPart( tsubpart6, 430, 38, 200, 192, 0);
      }
      self.Tabsheet();
    }

     void Tabsheet()
    {
      if (self.BSymbolId > 0)
        self.RemoveSubPart(self.BSymbolId);
      if (self.BChangeSymbolId > 0)
        self.RemoveSubPart(self.BChangeSymbolId);
      if (self.BSymbol2Id > 0)
        self.RemoveSubPart(self.BSymbol2Id);
      if (self.BChangeSymbol2Id > 0)
        self.RemoveSubPart(self.BChangeSymbol2Id);
      if (self.BPicId > 0)
        self.RemoveSubPart(self.BPicId);
      if (self.bChangePicId > 0)
        self.RemoveSubPart(self.bChangePicId);
      if (self.BSymbolGroupId > 0)
        self.RemoveSubPart(self.BSymbolGroupId);
      if (self.BSymbolGroupTextId > 0)
        self.RemoveSubPart(self.BSymbolGroupTextId);
      if (self.BSymbolWeightId > 0)
        self.RemoveSubPart(self.BSymbolWeightId);
      if (self.BSymbolWeightTextId > 0)
        self.RemoveSubPart(self.BSymbolWeightTextId);
      if (self.BSymbolOverRuleId > 0)
        self.RemoveSubPart(self.BSymbolOverRuleId);
      if ((uint) self.BSymbolOverRuleTextId > 0U)
        self.RemoveSubPart(self.BSymbolOverRuleTextId);
      if (self.ResListId > 0)
        self.RemoveSubPart(self.ResListId);
      if (self.ExtraListId > 0)
        self.RemoveSubPart(self.ExtraListId);
      if (self.PGListId > 0)
        self.RemoveSubPart(self.PGListId);
      if (self.CombatListId > 0)
        self.RemoveSubPart(self.CombatListId);
      if (self.CombatList2Id > 0)
        self.RemoveSubPart(self.CombatList2Id);
      if (self.combatlist3id > 0)
        self.RemoveSubPart(self.combatlist3id);
      if (self.combatlist4id > 0)
        self.RemoveSubPart(self.combatlist4id);
      if (self.x3id > 0)
        self.RemoveSubPart(self.x3id);
      if (self.x3textid > 0)
        self.RemoveSubPart(self.x3textid);
      if (self.y1id > 0)
        self.RemoveSubPart(self.y1id);
      if (self.y1textid > 0)
        self.RemoveSubPart(self.y1textid);
      if (self.y3id > 0)
        self.RemoveSubPart(self.y3id);
      if (self.y4id > 0)
        self.RemoveSubPart(self.y4id);
      if (self.y5id > 0)
        self.RemoveSubPart(self.y5id);
      if (self.y6id > 0)
        self.RemoveSubPart(self.y6id);
      if (self.y6textid > 0)
        self.RemoveSubPart(self.y6textid);
      if (self.y7id > 0)
        self.RemoveSubPart(self.y7id);
      if (self.y7textid > 0)
        self.RemoveSubPart(self.y7textid);
      if (self.y7btextid > 0)
        self.RemoveSubPart(self.y7btextid);
      if (self.y8btextid > 0)
        self.RemoveSubPart(self.y8btextid);
      if (self.y8id > 0)
        self.RemoveSubPart(self.y8id);
      if (self.y8textid > 0)
        self.RemoveSubPart(self.y8textid);
      if (self.y3textid > 0)
        self.RemoveSubPart(self.y3textid);
      if (self.j1id > 0)
        self.RemoveSubPart(self.j1id);
      if (self.j1textid > 0)
        self.RemoveSubPart(self.j1textid);
      if (self.LogoListId > 0)
        self.RemoveSubPart(self.LogoListId);
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      if (self.B1TextId > 0)
        self.RemoveSubPart(self.B1TextId);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B6TextId > 0)
        self.RemoveSubPart(self.B6TextId);
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      if (self.B8Id > 0)
        self.RemoveSubPart(self.B8Id);
      if (self.B8TextId > 0)
        self.RemoveSubPart(self.B8TextId);
      if (self.b9id > 0)
        self.RemoveSubPart(self.b9id);
      if (self.b9textid > 0)
        self.RemoveSubPart(self.b9textid);
      if (self.BMoveTypeId > 0)
        self.RemoveSubPart(self.BMoveTypeId);
      if (self.BMoveTypeTextId > 0)
        self.RemoveSubPart(self.BMoveTypeTextId);
      if (self.b10id > 0)
        self.RemoveSubPart(self.b10id);
      if (self.b10textid > 0)
        self.RemoveSubPart(self.b10textid);
      if (self.b11id > 0)
        self.RemoveSubPart(self.b11id);
      if (self.b11textid > 0)
        self.RemoveSubPart(self.b11textid);
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
      if (self.b15id > 0)
        self.RemoveSubPart(self.b15id);
      if (self.b15textid > 0)
        self.RemoveSubPart(self.b15textid);
      if (self.b16id > 0)
        self.RemoveSubPart(self.b16id);
      if (self.b16textid > 0)
        self.RemoveSubPart(self.b16textid);
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
      if (self.b23id > 0)
        self.RemoveSubPart(self.b23id);
      if (self.b23textid > 0)
        self.RemoveSubPart(self.b23textid);
      if (self.b24id > 0)
        self.RemoveSubPart(self.b24id);
      if (self.b24textid > 0)
        self.RemoveSubPart(self.b24textid);
      if (self.b25id > 0)
        self.RemoveSubPart(self.b25id);
      if (self.b25textid > 0)
        self.RemoveSubPart(self.b25textid);
      if (self.b26id > 0)
        self.RemoveSubPart(self.b26id);
      if (self.b26textid > 0)
        self.RemoveSubPart(self.b26textid);
      if (self.b27id > 0)
        self.RemoveSubPart(self.b27id);
      if (self.b27textid > 0)
        self.RemoveSubPart(self.b27textid);
      if (self.b28id > 0)
        self.RemoveSubPart(self.b28id);
      if (self.b29id > 0)
        self.RemoveSubPart(self.b29id);
      if (self.b29textid > 0)
        self.RemoveSubPart(self.b29textid);
      if (self.b30id > 0)
        self.RemoveSubPart(self.b30id);
      if (self.b30textid > 0)
        self.RemoveSubPart(self.b30textid);
      if (self.b31id > 0)
        self.RemoveSubPart(self.b31id);
      if (self.b31textid > 0)
        self.RemoveSubPart(self.b31textid);
      if (self.b32id > 0)
        self.RemoveSubPart(self.b32id);
      if (self.b32textid > 0)
        self.RemoveSubPart(self.b32textid);
      if (self.b33id > 0)
        self.RemoveSubPart(self.b33id);
      if (self.b33textid > 0)
        self.RemoveSubPart(self.b33textid);
      if (self.b34id > 0)
        self.RemoveSubPart(self.b34id);
      if (self.b34textid > 0)
        self.RemoveSubPart(self.b34textid);
      if (self.b35id > 0)
        self.RemoveSubPart(self.b35id);
      if (self.b35textid > 0)
        self.RemoveSubPart(self.b35textid);
      if (self.b36id > 0)
        self.RemoveSubPart(self.b36id);
      if (self.b36textid > 0)
        self.RemoveSubPart(self.b36textid);
      if (self.b37id > 0)
        self.RemoveSubPart(self.b37id);
      if (self.b37textid > 0)
        self.RemoveSubPart(self.b37textid);
      if (self.b38id > 0)
        self.RemoveSubPart(self.b38id);
      if (self.b38textid > 0)
        self.RemoveSubPart(self.b38textid);
      if (self.b39id > 0)
        self.RemoveSubPart(self.b39id);
      if (self.b39textid > 0)
        self.RemoveSubPart(self.b39textid);
      if (self.a1id > 0)
        self.RemoveSubPart(self.a1id);
      if (self.a1textid > 0)
        self.RemoveSubPart(self.a1textid);
      if (self.a2id > 0)
        self.RemoveSubPart(self.a2id);
      if (self.a2textid > 0)
        self.RemoveSubPart(self.a2textid);
      if (self.a3id > 0)
        self.RemoveSubPart(self.a3id);
      if (self.a3textid > 0)
        self.RemoveSubPart(self.a3textid);
      if (self.a4id > 0)
        self.RemoveSubPart(self.a4id);
      if (self.a4textid > 0)
        self.RemoveSubPart(self.a4textid);
      if (self.a5id > 0)
        self.RemoveSubPart(self.a5id);
      if (self.a5textid > 0)
        self.RemoveSubPart(self.a5textid);
      if (self.a6id > 0)
        self.RemoveSubPart(self.a6id);
      if (self.a6textid > 0)
        self.RemoveSubPart(self.a6textid);
      if (self.t1id > 0)
        self.RemoveSubPart(self.t1id);
      if (self.t1textid > 0)
        self.RemoveSubPart(self.t1textid);
      if (self.c11id > 0)
        self.RemoveSubPart(self.c11id);
      if (self.c11textid > 0)
        self.RemoveSubPart(self.c11textid);
      if (self.c12id > 0)
        self.RemoveSubPart(self.c12id);
      if (self.c12textid > 0)
        self.RemoveSubPart(self.c12textid);
      if (self.c13id > 0)
        self.RemoveSubPart(self.c13id);
      if (self.c13textid > 0)
        self.RemoveSubPart(self.c13textid);
      if (self.c14id > 0)
        self.RemoveSubPart(self.c14id);
      if (self.c14textid > 0)
        self.RemoveSubPart(self.c14textid);
      if (self.c15id > 0)
        self.RemoveSubPart(self.c15id);
      if (self.c15textid > 0)
        self.RemoveSubPart(self.c15textid);
      if (self.c16id > 0)
        self.RemoveSubPart(self.c16id);
      if (self.c16textid > 0)
        self.RemoveSubPart(self.c16textid);
      if (self.c17id > 0)
        self.RemoveSubPart(self.c17id);
      if (self.c17textid > 0)
        self.RemoveSubPart(self.c17textid);
      if (self.c18id > 0)
        self.RemoveSubPart(self.c18id);
      if (self.c18textid > 0)
        self.RemoveSubPart(self.c18textid);
      if (self.c19id > 0)
        self.RemoveSubPart(self.c19id);
      if (self.c19textid > 0)
        self.RemoveSubPart(self.c19textid);
      if (self.c20id > 0)
        self.RemoveSubPart(self.c20id);
      if (self.c20textid > 0)
        self.RemoveSubPart(self.c20textid);
      if (self.c21id > 0)
        self.RemoveSubPart(self.c21id);
      if (self.c21textid > 0)
        self.RemoveSubPart(self.c21textid);
      if (self.c1id > 0)
        self.RemoveSubPart(self.c1id);
      if (self.c1textid > 0)
        self.RemoveSubPart(self.c1textid);
      if (self.c2id > 0)
        self.RemoveSubPart(self.c2id);
      if (self.c2textid > 0)
        self.RemoveSubPart(self.c2textid);
      if (self.c3id > 0)
        self.RemoveSubPart(self.c3id);
      if (self.c3textid > 0)
        self.RemoveSubPart(self.c3textid);
      if (self.c4id > 0)
        self.RemoveSubPart(self.c4id);
      if (self.c4textid > 0)
        self.RemoveSubPart(self.c4textid);
      if (self.c5id > 0)
        self.RemoveSubPart(self.c5id);
      if (self.c5textid > 0)
        self.RemoveSubPart(self.c5textid);
      if (self.c6id > 0)
        self.RemoveSubPart(self.c6id);
      if (self.c6textid > 0)
        self.RemoveSubPart(self.c6textid);
      if (self.c7id > 0)
        self.RemoveSubPart(self.c7id);
      if (self.c7textid > 0)
        self.RemoveSubPart(self.c7textid);
      if (self.c8id > 0)
        self.RemoveSubPart(self.c8id);
      if (self.c8textid > 0)
        self.RemoveSubPart(self.c8textid);
      if (self.d1id > 0)
        self.RemoveSubPart(self.d1id);
      if (self.d1textid > 0)
        self.RemoveSubPart(self.d1textid);
      if (self.e1id > 0)
        self.RemoveSubPart(self.e1id);
      if (self.e1textid > 0)
        self.RemoveSubPart(self.e1textid);
      if (self.e2id > 0)
        self.RemoveSubPart(self.e2id);
      if (self.e2textid > 0)
        self.RemoveSubPart(self.e2textid);
      if (self.e3id > 0)
        self.RemoveSubPart(self.e3id);
      if (self.e3textid > 0)
        self.RemoveSubPart(self.e3textid);
      if (self.e4id > 0)
        self.RemoveSubPart(self.e4id);
      if (self.e4textid > 0)
        self.RemoveSubPart(self.e4textid);
      if (self.e5id > 0)
        self.RemoveSubPart(self.e5id);
      if (self.e5textid > 0)
        self.RemoveSubPart(self.e5textid);
      if (self.e6id > 0)
        self.RemoveSubPart(self.e6id);
      if (self.e6textid > 0)
        self.RemoveSubPart(self.e6textid);
      if (self.e7id > 0)
        self.RemoveSubPart(self.e7id);
      if (self.e7textid > 0)
        self.RemoveSubPart(self.e7textid);
      if (self.f1id > 0)
        self.RemoveSubPart(self.f1id);
      if (self.f1textid > 0)
        self.RemoveSubPart(self.f1textid);
      if (self.f2id > 0)
        self.RemoveSubPart(self.f2id);
      if (self.f2textid > 0)
        self.RemoveSubPart(self.f2textid);
      if (self.f3id > 0)
        self.RemoveSubPart(self.f3id);
      if (self.f3textid > 0)
        self.RemoveSubPart(self.f3textid);
      if (self.v1id > 0)
        self.RemoveSubPart(self.v1id);
      if (self.v1textid > 0)
        self.RemoveSubPart(self.v1textid);
      if (self.v2id > 0)
        self.RemoveSubPart(self.v2id);
      if (self.v2textid > 0)
        self.RemoveSubPart(self.v2textid);
      if (self.v3id > 0)
        self.RemoveSubPart(self.v3id);
      if (self.v3textid > 0)
        self.RemoveSubPart(self.v3textid);
      if (self.v4id > 0)
        self.RemoveSubPart(self.v4id);
      if (self.v4textid > 0)
        self.RemoveSubPart(self.v4textid);
      if (self.v5id > 0)
        self.RemoveSubPart(self.v5id);
      if (self.v5textid > 0)
        self.RemoveSubPart(self.v5textid);
      if (self.v6id > 0)
        self.RemoveSubPart(self.v6id);
      if (self.v6textid > 0)
        self.RemoveSubPart(self.v6textid);
      if (self.v7id > 0)
        self.RemoveSubPart(self.v7id);
      if (self.v7textid > 0)
        self.RemoveSubPart(self.v7textid);
      if (self.v8id > 0)
        self.RemoveSubPart(self.v8id);
      if (self.v8textid > 0)
        self.RemoveSubPart(self.v8textid);
      if (self.v9id > 0)
        self.RemoveSubPart(self.v9id);
      if (self.v9textid > 0)
        self.RemoveSubPart(self.v9textid);
      if (self.v10id > 0)
        self.RemoveSubPart(self.v10id);
      if (self.v10textid > 0)
        self.RemoveSubPart(self.v10textid);
      if (self.v11id > 0)
        self.RemoveSubPart(self.v11id);
      if (self.v11textid > 0)
        self.RemoveSubPart(self.v11textid);
      if (self.v12id > 0)
        self.RemoveSubPart(self.v12id);
      if (self.v12textid > 0)
        self.RemoveSubPart(self.v12textid);
      if (self.v13id > 0)
        self.RemoveSubPart(self.v13id);
      if (self.v13textid > 0)
        self.RemoveSubPart(self.v13textid);
      if (self.v14id > 0)
        self.RemoveSubPart(self.v14id);
      if (self.v14textid > 0)
        self.RemoveSubPart(self.v14textid);
      if (self.v15id > 0)
        self.RemoveSubPart(self.v15id);
      if (self.v15textid > 0)
        self.RemoveSubPart(self.v15textid);
      if (self.v16id > 0)
        self.RemoveSubPart(self.v16id);
      if (self.v16textid > 0)
        self.RemoveSubPart(self.v16textid);
      if (self.v17id > 0)
        self.RemoveSubPart(self.v17id);
      if (self.v17textid > 0)
        self.RemoveSubPart(self.v17textid);
      if (self.v18id > 0)
        self.RemoveSubPart(self.v18id);
      if (self.v18textid > 0)
        self.RemoveSubPart(self.v18textid);
      if (self.v19id > 0)
        self.RemoveSubPart(self.v19id);
      if (self.v19textid > 0)
        self.RemoveSubPart(self.v19textid);
      if (self.v20id > 0)
        self.RemoveSubPart(self.v20id);
      if (self.v20textid > 0)
        self.RemoveSubPart(self.v20textid);
      if (self.v21id > 0)
        self.RemoveSubPart(self.v21id);
      if (self.v21textid > 0)
        self.RemoveSubPart(self.v21textid);
      if (self.v22id > 0)
        self.RemoveSubPart(self.v22id);
      if (self.v22textid > 0)
        self.RemoveSubPart(self.v22textid);
      if (self.v23id > 0)
        self.RemoveSubPart(self.v23id);
      if (self.v23textid > 0)
        self.RemoveSubPart(self.v23textid);
      if (self.y2id > 0)
        self.RemoveSubPart(self.y2id);
      if (self.g1id > 0)
        self.RemoveSubPart(self.g1id);
      if (self.g1textid > 0)
        self.RemoveSubPart(self.g1textid);
      if (self.g2id > 0)
        self.RemoveSubPart(self.g2id);
      if (self.g2textid > 0)
        self.RemoveSubPart(self.g2textid);
      if (self.g3id > 0)
        self.RemoveSubPart(self.g3id);
      if (self.g3textid > 0)
        self.RemoveSubPart(self.g3textid);
      if (self.g4id > 0)
        self.RemoveSubPart(self.g4id);
      if (self.g4textid > 0)
        self.RemoveSubPart(self.g4textid);
      if (self.g5id > 0)
        self.RemoveSubPart(self.g5id);
      if (self.g5textid > 0)
        self.RemoveSubPart(self.g5textid);
      if (self.g6id > 0)
        self.RemoveSubPart(self.g6id);
      if (self.g6textid > 0)
        self.RemoveSubPart(self.g6textid);
      if (self.g7id > 0)
        self.RemoveSubPart(self.g7id);
      if (self.g7textid > 0)
        self.RemoveSubPart(self.g7textid);
      if (self.g8id > 0)
        self.RemoveSubPart(self.g8id);
      if (self.g8textid > 0)
        self.RemoveSubPart(self.g8textid);
      if (self.g9id > 0)
        self.RemoveSubPart(self.g9id);
      if (self.g9textid > 0)
        self.RemoveSubPart(self.g9textid);
      if (self.g10id > 0)
        self.RemoveSubPart(self.g10id);
      if (self.g10textid > 0)
        self.RemoveSubPart(self.g10textid);
      if (self.g11id > 0)
        self.RemoveSubPart(self.g11id);
      if (self.g11textid > 0)
        self.RemoveSubPart(self.g11textid);
      if (self.g12id > 0)
        self.RemoveSubPart(self.g12id);
      if (self.g12textid > 0)
        self.RemoveSubPart(self.g12textid);
      if (self.g13id > 0)
        self.RemoveSubPart(self.g13id);
      if (self.g13textid > 0)
        self.RemoveSubPart(self.g13textid);
      if (self.g14id > 0)
        self.RemoveSubPart(self.g14id);
      if (self.g14textid > 0)
        self.RemoveSubPart(self.g14textid);
      if (self.g15id > 0)
        self.RemoveSubPart(self.g15id);
      if (self.g15textid > 0)
        self.RemoveSubPart(self.g15textid);
      if (self.g16id > 0)
        self.RemoveSubPart(self.g16id);
      if (self.g16textid > 0)
        self.RemoveSubPart(self.g16textid);
      if (self.g17id > 0)
        self.RemoveSubPart(self.g17id);
      if (self.g17textid > 0)
        self.RemoveSubPart(self.g17textid);
      if (self.g18id > 0)
        self.RemoveSubPart(self.g18id);
      if (self.g18textid > 0)
        self.RemoveSubPart(self.g18textid);
      if (self.g19id > 0)
        self.RemoveSubPart(self.g19id);
      if (self.g19textid > 0)
        self.RemoveSubPart(self.g19textid);
      if (self.g20id > 0)
        self.RemoveSubPart(self.g20id);
      if (self.g20textid > 0)
        self.RemoveSubPart(self.g20textid);
      if (self.g21id > 0)
        self.RemoveSubPart(self.g21id);
      if (self.g21textid > 0)
        self.RemoveSubPart(self.g21textid);
      if (self.g23id > 0)
        self.RemoveSubPart(self.g23id);
      if (self.g23textid > 0)
        self.RemoveSubPart(self.g23textid);
      if (self.g24id > 0)
        self.RemoveSubPart(self.g24id);
      if (self.g24textid > 0)
        self.RemoveSubPart(self.g24textid);
      if (self.h1id > 0)
        self.RemoveSubPart(self.h1id);
      if (self.h1textid > 0)
        self.RemoveSubPart(self.h1textid);
      if (self.h2id > 0)
        self.RemoveSubPart(self.h2id);
      if (self.h2textid > 0)
        self.RemoveSubPart(self.h2textid);
      if (self.h3id > 0)
        self.RemoveSubPart(self.h3id);
      if (self.h3textid > 0)
        self.RemoveSubPart(self.h3textid);
      if (self.h4id > 0)
        self.RemoveSubPart(self.h4id);
      if (self.h4textid > 0)
        self.RemoveSubPart(self.h4textid);
      if (self.h5id > 0)
        self.RemoveSubPart(self.h5id);
      if (self.h5textid > 0)
        self.RemoveSubPart(self.h5textid);
      if (self.h6id > 0)
        self.RemoveSubPart(self.h6id);
      if (self.h6textid > 0)
        self.RemoveSubPart(self.h6textid);
      if (self.copyid > 0)
        self.RemoveSubPart(self.copyid);
      if (self.copytextid > 0)
        self.RemoveSubPart(self.copytextid);
      if (self.p1id > 0)
        self.RemoveSubPart(self.p1id);
      if (self.p1textid > 0)
        self.RemoveSubPart(self.p1textid);
      if (self.p2id > 0)
        self.RemoveSubPart(self.p2id);
      if (self.p2textid > 0)
        self.RemoveSubPart(self.p2textid);
      if (self.p3id > 0)
        self.RemoveSubPart(self.p3id);
      if (self.p3textid > 0)
        self.RemoveSubPart(self.p3textid);
      if (self.p4id > 0)
        self.RemoveSubPart(self.p4id);
      if (self.p4textid > 0)
        self.RemoveSubPart(self.p4textid);
      if (self.p5id > 0)
        self.RemoveSubPart(self.p5id);
      if (self.p5textid > 0)
        self.RemoveSubPart(self.p5textid);
      if (self.p6id > 0)
        self.RemoveSubPart(self.p6id);
      if (self.p6textid > 0)
        self.RemoveSubPart(self.p6textid);
      if (self.p7id > 0)
        self.RemoveSubPart(self.p7id);
      if (self.p7textid > 0)
        self.RemoveSubPart(self.p7textid);
      if (self.p8id > 0)
        self.RemoveSubPart(self.p8id);
      if (self.p8textid > 0)
        self.RemoveSubPart(self.p8textid);
      if (self.p9id > 0)
        self.RemoveSubPart(self.p9id);
      if (self.p9textid > 0)
        self.RemoveSubPart(self.p9textid);
      if (self.vp1id > 0)
        self.RemoveSubPart(self.vp1id);
      if (self.vp1textid > 0)
        self.RemoveSubPart(self.vp1textid);
      if (self.vp2id > 0)
        self.RemoveSubPart(self.vp2id);
      if (self.vp2textid > 0)
        self.RemoveSubPart(self.vp2textid);
      if (self.vp3id > 0)
        self.RemoveSubPart(self.vp3id);
      if (self.vp3textid > 0)
        self.RemoveSubPart(self.vp3textid);
      if (self.vp4id > 0)
        self.RemoveSubPart(self.vp4id);
      if (self.vp4textid > 0)
        self.RemoveSubPart(self.vp4textid);
      if (self.vp5id > 0)
        self.RemoveSubPart(self.vp5id);
      if (self.vp5textid > 0)
        self.RemoveSubPart(self.vp5textid);
      if (self.vp6id > 0)
        self.RemoveSubPart(self.vp6id);
      if (self.vp6textid > 0)
        self.RemoveSubPart(self.vp6textid);
      if (self.w1id > 0)
        self.RemoveSubPart(self.w1id);
      if (self.w1textid > 0)
        self.RemoveSubPart(self.w1textid);
      if (self.w1bid > 0)
        self.RemoveSubPart(self.w1bid);
      if (self.w1btextid > 0)
        self.RemoveSubPart(self.w1btextid);
      if (self.w2id > 0)
        self.RemoveSubPart(self.w2id);
      if (self.w2textid > 0)
        self.RemoveSubPart(self.w2textid);
      if (self.w2bid > 0)
        self.RemoveSubPart(self.w2bid);
      if (self.w2btextid > 0)
        self.RemoveSubPart(self.w2btextid);
      if (self.w3id > 0)
        self.RemoveSubPart(self.w3id);
      if (self.w3textid > 0)
        self.RemoveSubPart(self.w3textid);
      if (self.w133id > 0)
        self.RemoveSubPart(self.w133id);
      if (self.w133textid > 0)
        self.RemoveSubPart(self.w133textid);
      if (self.w4id > 0)
        self.RemoveSubPart(self.w4id);
      if (self.w4textid > 0)
        self.RemoveSubPart(self.w4textid);
      if (self.w5id > 0)
        self.RemoveSubPart(self.w5id);
      if (self.w5textid > 0)
        self.RemoveSubPart(self.w5textid);
      if (self.w6id > 0)
        self.RemoveSubPart(self.w6id);
      if (self.w6textid > 0)
        self.RemoveSubPart(self.w6textid);
      if (self.w7id > 0)
        self.RemoveSubPart(self.w7id);
      if (self.w7textid > 0)
        self.RemoveSubPart(self.w7textid);
      if (self.w8id > 0)
        self.RemoveSubPart(self.w8id);
      if (self.w8textid > 0)
        self.RemoveSubPart(self.w8textid);
      if (self.w9id > 0)
        self.RemoveSubPart(self.w9id);
      if (self.w9textid > 0)
        self.RemoveSubPart(self.w9textid);
      if (self.w9bid > 0)
        self.RemoveSubPart(self.w9bid);
      if (self.w9btextid > 0)
        self.RemoveSubPart(self.w9btextid);
      if (self.w10id > 0)
        self.RemoveSubPart(self.w10id);
      if (self.w10textid > 0)
        self.RemoveSubPart(self.w10textid);
      if (self.w11id > 0)
        self.RemoveSubPart(self.w11id);
      if (self.w11textid > 0)
        self.RemoveSubPart(self.w11textid);
      if (self.w12id > 0)
        self.RemoveSubPart(self.w12id);
      if (self.w12textid > 0)
        self.RemoveSubPart(self.w12textid);
      if (self.w13id > 0)
        self.RemoveSubPart(self.w13id);
      if (self.w13textid > 0)
        self.RemoveSubPart(self.w13textid);
      if (self.w14id > 0)
        self.RemoveSubPart(self.w14id);
      if (self.w14textid > 0)
        self.RemoveSubPart(self.w14textid);
      if (self.w16id > 0)
        self.RemoveSubPart(self.w16id);
      if (self.w15textid > 0)
        self.RemoveSubPart(self.w15textid);
      if (self.w15id > 0)
        self.RemoveSubPart(self.w15id);
      if (self.w16textid > 0)
        self.RemoveSubPart(self.w16textid);
      if (self.w17id > 0)
        self.RemoveSubPart(self.w17id);
      if (self.w17textid > 0)
        self.RemoveSubPart(self.w17textid);
      if (self.VariantListId > 0)
        self.RemoveSubPart(self.VariantListId);
      if (self.PreventListId > 0)
        self.RemoveSubPart(self.PreventListId);
      if (self.SFtypeNr <= -1)
        return;
      if (self.TabSheetNr == 0)
        self.tabsheet0();
      if (self.TabSheetNr == 1)
        self.tabsheet1();
      if (self.TabSheetNr == 2)
        self.tabsheet2();
      if (self.TabSheetNr == 3)
        self.tabsheet3();
      if (self.TabSheetNr == 4)
        self.tabsheet4();
      if (self.TabSheetNr == 5)
        self.tabsheet5();
      if (self.TabSheetNr == 6)
        self.tabsheet6();
      if (self.TabSheetNr == 7)
        self.tabsheet7();
      if (self.TabSheetNr == 8)
        self.tabsheet8();
      if (self.TabSheetNr == 9)
        self.tabsheet9();
      if (self.TabSheetNr != 10)
        return;
      self.tabsheet10();
    }

     void tabsheet0()
    {
      self.ss = "COUNTER SYMBOL - Click to change the graphics used to symoblize this sftype on a counter of a unit";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.Data.SFTypeObj[self.SFtypeNr].SymbolSpriteID, tDescript: self.ss);
      self.BSymbolId = self.AddSubPart( tsubpart1, 10, 360, 31, 31, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
        self.BChangeSymbolId = self.AddSubPart( tsubpart2, 50, 360, 32, 16, 1);
      }
      self.ss = "MOVE+COMBAT SYMOBL - Click to change the graphics used to symoblize this sftype on a counter of a unit";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.Data.SFTypeObj[self.SFtypeNr].SymbolSprite2ID, tDescript: self.ss);
      self.BSymbol2Id = self.AddSubPart( tsubpart3, 110, 360, 31, 31, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
        self.BChangeSymbol2Id = self.AddSubPart( tsubpart4, 150, 360, 32, 16, 1);
      }
      self.ss = "Let People overdraw a gfx over this sftype. 0=dont. 1=yes in front of eqp. 2=behind eqp";
      let mut tsubpart5: SubPartClass =  TextPartClass::new("UsePplGfx=" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].UsePeopleGraphics), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: self.ss);
      self.y3textid = self.AddSubPart( tsubpart5, 250, 360, 100, 20, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart6: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.y3id = self.AddSubPart( tsubpart6, 210, 360, 32, 16, 1);
      }
      self.ss = "Illustration Graphic - Sideways Sprite";
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.Data.SFTypeObj[self.SFtypeNr].SidewaysSpriteID, tDescript: self.ss, tResizeX: 70, tresizeY: 40);
      self.y4id = self.AddSubPart( tsubpart7, 180, 400, 70, 40, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart8: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
        self.y5id = self.AddSubPart( tsubpart8, 260, 400, 32, 16, 1);
      }
      self.ss = "Click to change the artistic graphic for this sftype";
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(self.game.Data.SFTypeObj[self.SFtypeNr].PicSpriteID, tDescript: self.ss, tResizeX: 96, tresizeY: 72);
      self.BPicId = self.AddSubPart( tsubpart9, 10, 400, 96, 72, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart10: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
        self.bChangePicId = self.AddSubPart( tsubpart10, 140, 400, 32, 16, 1);
      }
      self.ss = "Click to assign the sftype a symbolgroup number, used for pre-calculation which symbolgroup is shown in mixed unit";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart11: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BSymbolGroupId = self.AddSubPart( tsubpart11, 10, 540, 32, 16, 1);
      }
      let mut tsubpart12: SubPartClass =  TextPartClass::new("Symbol Group: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SymbolGroup), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.BSymbolGroupTextId = self.AddSubPart( tsubpart12, 50, 539, 400, 20, 0);
      self.ss = "Click to assign the sftype as symbolweight, the more weight the earlier it prevails as symbol shown in mixed unit";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart13: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BSymbolWeightId = self.AddSubPart( tsubpart13, 10, 570, 32, 16, 1);
      }
      let mut tsubpart14: SubPartClass =  TextPartClass::new("Symbol Weight: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SymbolWeight), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.BSymbolWeightTextId = self.AddSubPart( tsubpart14, 50, 569, 400, 20, 0);
      self.ss = "Click to toggle symbol overrule on or off. A symboloverrule means that this symbol will not be cloured as regime pen colour.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart15: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BSymbolOverRuleId = self.AddSubPart( tsubpart15, 10, 600, 32, 16, 1);
      }
      let mut tsubpart16: SubPartClass =  TextPartClass::new("OverRule Symbol: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SymbolOverrule), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.BSymbolOverRuleTextId = self.AddSubPart( tsubpart16, 50, 599, 400, 20, 0);
    }

     void tabsheet1()
    {
      self.ss = "Click to set the MoveType of this SubformationType";
      str1: String = self.game.Data.TempString[self.game.Data.SFTypeObj[self.SFtypeNr].MoveType];
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BMoveTypeId = self.AddSubPart( tsubpart, 10, 340, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Move Type: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.BMoveTypeTextId = self.AddSubPart( tsubpart1, 50, 339, 400, 20, 0);
      self.ss = "Click to set how much supply sftype can maximally hold with it without using carrycap";
      str2: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SupplyCarry);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B1Id = self.AddSubPart( tsubpart2, 10, 380, 32, 16, 1);
      }
      let mut tsubpart3: SubPartClass =  TextPartClass::new("Supply Carry: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B1TextId = self.AddSubPart( tsubpart3, 50, 379, 400, 20, 0);
      self.ss = "Click to set howmuch supply the sftype can maximally consume per round";
      str3: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].BasicSupplyNeed);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B6Id = self.AddSubPart( tsubpart4, 10, 460, 32, 16, 1);
      }
      let mut tsubpart5: SubPartClass =  TextPartClass::new("Basic Supply Need: " + str3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B6TextId = self.AddSubPart( tsubpart5, 50, 459, 200, 20, 0);
      self.ss = "Click to set the UnitGroup of this sftype. Is used for combatdetail stats and landscape entrench stats.";
      str4: String = self.game.Data.TempString[400 + self.game.Data.SFTypeObj[self.SFtypeNr].UnitGroup] + "(" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].UnitGroup) + ")";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart6: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B7Id = self.AddSubPart( tsubpart6, 10, 480, 32, 16, 1);
      }
      let mut tsubpart7: SubPartClass =  TextPartClass::new("SFType Group: " + str4, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B7TextId = self.AddSubPart( tsubpart7, 50, 479, 200, 20, 0);
      self.ss = "Click to set ammount of reconpoints";
      str5: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ReconPts);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart8: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.f1id = self.AddSubPart( tsubpart8, 10, 500, 32, 16, 1);
      }
      let mut tsubpart9: SubPartClass =  TextPartClass::new("ReconPts: " + str5, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.f1textid = self.AddSubPart( tsubpart9, 50, 500, 200, 20, 0);
      self.ss = "Click to set ammount of hidepoints. Specifying the minimal number of reconpoints needed to see this sftype.";
      str6: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].HidePts);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart10: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.f2id = self.AddSubPart( tsubpart10, 10, 520, 32, 16, 1);
      }
      let mut tsubpart11: SubPartClass =  TextPartClass::new("HidePts: " + str6, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.f2textid = self.AddSubPart( tsubpart11, 50, 520, 200, 20, 0);
      self.ss = "Click to set the number of Zone of Controll points";
      str7: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ZOCPts);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart12: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.f3id = self.AddSubPart( tsubpart12, 10, 540, 32, 16, 1);
      }
      let mut tsubpart13: SubPartClass =  TextPartClass::new("ZOCPts: " + str7, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.f3textid = self.AddSubPart( tsubpart13, 50, 540, 200, 20, 0);
      self.ss = "Click to toggle on/off if the sftype can be used for paradropping. Without paradrop airlift is always still possible";
      str8: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].CanDoParadrop);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart14: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g1id = self.AddSubPart( tsubpart14, 10, 560, 32, 16, 1);
      }
      let mut tsubpart15: SubPartClass =  TextPartClass::new("CanDoParadrop: " + str8, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g1textid = self.AddSubPart( tsubpart15, 50, 560, 200, 20, 0);
      self.ss = "Click to set the number of anti-struc points per combatround this sftype can maximally do";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart16: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g2id = self.AddSubPart( tsubpart16, 10, 580, 32, 16, 1);
      }
      let mut tsubpart17: SubPartClass =  TextPartClass::new("AntiStrucPts: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AntiStrucPts), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g2textid = self.AddSubPart( tsubpart17, 50, 580, 200, 20, 0);
      self.ss = "Click to set the Theater type of this sftype. 0=land, 1=navy and 2=air";
      str9: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].Theater);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart18: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B8Id = self.AddSubPart( tsubpart18, 310, 360, 32, 16, 1);
      }
      let mut tsubpart19: SubPartClass =  TextPartClass::new("Theater: " + str9, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B8TextId = self.AddSubPart( tsubpart19, 350, 359, 200, 20, 0);
      self.ss = "Click to set the weight of this sftype. Is used in mobility determination calcs and transfers/str.transfers";
      str10: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].Weight);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart20: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b9id = self.AddSubPart( tsubpart20, 310, 380, 32, 16, 1);
      }
      let mut tsubpart21: SubPartClass =  TextPartClass::new("Weight: " + str10, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b9textid = self.AddSubPart( tsubpart21, 350, 379, 200, 20, 0);
      self.ss = "Click to set how much weight points this sftype can carry/mobilize";
      str11: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].CarryCap);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart22: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.d1id = self.AddSubPart( tsubpart22, 310, 400, 32, 16, 1);
      }
      let mut tsubpart23: SubPartClass =  TextPartClass::new("CarryCap: " + str11, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.d1textid = self.AddSubPart( tsubpart23, 350, 399, 200, 20, 0);
      self.ss = "Click to set howmany entrench points this sftype generates at start of every turn";
      str12: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].EntrenchPower);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart24: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.e1id = self.AddSubPart( tsubpart24, 310, 420, 32, 16, 1);
      }
      let mut tsubpart25: SubPartClass =  TextPartClass::new("EntrenchPower: " + str12, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.e1textid = self.AddSubPart( tsubpart25, 350, 419, 200, 20, 0);
      self.ss = "Click to set the powerpoints of this sftype. Very important for experience calculations! Used to display counter strenght.";
      str13: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].PowerPts);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart26: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.e3id = self.AddSubPart( tsubpart26, 310, 460, 32, 16, 1);
      }
      let mut tsubpart27: SubPartClass =  TextPartClass::new("PowerPts: " + str13, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.e3textid = self.AddSubPart( tsubpart27, 350, 459, 200, 20, 0);
      self.ss = "Click to set the percentage of movement cost reduction this sftype will get on its movetype costs. Example: 40 is 40% less AP cost";
      str14: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].MoveRedux);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart28: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b24id = self.AddSubPart( tsubpart28, 310, 640, 32, 16, 1);
      }
      let mut tsubpart29: SubPartClass =  TextPartClass::new("MoveRedux: " + str14, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b24textid = self.AddSubPart( tsubpart29, 350, 639, 200, 20, 0);
      self.ss = "Click to set a possible actionpomod: i32. Making it possible to give this sftype more or less than 100ap if fully ready.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart30: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g4id = self.AddSubPart( tsubpart30, 610, 240, 32, 16, 1);
      }
      let mut tsubpart31: SubPartClass =  TextPartClass::new("ActionPoMod: i32: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ApMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g4textid = self.AddSubPart( tsubpart31, 650, 239, 200, 20, 0);
      self.ss = "Click to set howmuch absolute readiness points this sfype loses with each attack";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart32: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g6id = self.AddSubPart( tsubpart32, 610, 260, 32, 16, 1);
      }
      let mut tsubpart33: SubPartClass =  TextPartClass::new("RdnLossPerAttack: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].RdnLossPerAttack), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g6textid = self.AddSubPart( tsubpart33, 650, 259, 200, 20, 0);
      self.ss = "Click to toggle on/off if this sftype should autodestroy after having finished one full combatround.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart34: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g7id = self.AddSubPart( tsubpart34, 610, 280, 32, 16, 1);
      }
      let mut tsubpart35: SubPartClass =  TextPartClass::new("AutoDestroy: Att=" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AutoDestroy) + ", Def=" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AutoDestroy2), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g7textid = self.AddSubPart( tsubpart35, 650, 279, 200, 20, 0);
      self.ss = "Click to set the ammount of engineer points this sftype will get every round";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart36: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g8id = self.AddSubPart( tsubpart36, 610, 300, 32, 16, 1);
      }
      let mut tsubpart37: SubPartClass =  TextPartClass::new("EP: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].EP), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g8textid = self.AddSubPart( tsubpart37, 650, 299, 200, 20, 0);
      self.ss = "Click to choose the sound that has to be played when the sftype moves";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart38: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g10id = self.AddSubPart( tsubpart38, 610, 340, 32, 16, 1);
      }
      let mut tsubpart39: SubPartClass =  TextPartClass::new("MoveWAV: " + self.game.Data.SFTypeObj[self.SFtypeNr].MoveWAV, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g10textid = self.AddSubPart( tsubpart39, 650, 339, 200, 20, 0);
      self.ss = "Click to choose the sound that has to be played when the sftype fights";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart40: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g11id = self.AddSubPart( tsubpart40, 610, 360, 32, 16, 1);
      }
      let mut tsubpart41: SubPartClass =  TextPartClass::new("BattleWAV: " + self.game.Data.SFTypeObj[self.SFtypeNr].BattleWAV, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g11textid = self.AddSubPart( tsubpart41, 650, 359, 200, 20, 0);
      self.ss = "Click to set the number of staff points this sftype has. 1 Staffpoints is needed for each Powerpounder: i32 command.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart42: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g15id = self.AddSubPart( tsubpart42, 610, 400, 32, 16, 1);
      }
      let mut tsubpart43: SubPartClass =  TextPartClass::new("StaffPts: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StaffPts), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g15textid = self.AddSubPart( tsubpart43, 650, 399, 200, 20, 0);
      self.ss = "Click to set the anti-struc points generated by this sftype when attempting to blow a bridge";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart44: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g17id = self.AddSubPart( tsubpart44, 610, 420, 32, 16, 1);
      }
      let mut tsubpart45: SubPartClass =  TextPartClass::new("BlowBridgePts: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].BlowBridgePts), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g17textid = self.AddSubPart( tsubpart45, 650, 419, 200, 20, 0);
      self.ss = "Click to set the percentage chance a kill against this sftype is mutated into a retreat.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart46: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g18id = self.AddSubPart( tsubpart46, 610, 440, 32, 16, 1);
      }
      let mut tsubpart47: SubPartClass =  TextPartClass::new("KillToRetr%(in def): " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].KilltoRetreatChance), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g18textid = self.AddSubPart( tsubpart47, 650, 439, 200, 20, 0);
      self.ss = "Click if the sftype has staff points to set the max combat modifier for units under a hq with this sftype.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart48: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b29id = self.AddSubPart( tsubpart48, 610, 460, 32, 16, 1);
      }
      let mut tsubpart49: SubPartClass =  TextPartClass::new("StaffCombatMod: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StaffCombatMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b29textid = self.AddSubPart( tsubpart49, 650, 459, 200, 20, 0);
      self.ss = "Click if the sftype has staff points to set the max morale modifier for units under a hq with this sftype.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart50: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b30id = self.AddSubPart( tsubpart50, 610, 480, 32, 16, 1);
      }
      let mut tsubpart51: SubPartClass =  TextPartClass::new("StaffMoraleMod: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StaffMoraleMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b30textid = self.AddSubPart( tsubpart51, 650, 479, 200, 20, 0);
      self.ss = "Click to set the antisupply points this sftype has versus land hexes.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart52: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g19id = self.AddSubPart( tsubpart52, 610, 500, 32, 16, 1);
      }
      let mut tsubpart53: SubPartClass =  TextPartClass::new("AntiSupplyPts: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AntiSupply), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g19textid = self.AddSubPart( tsubpart53, 650, 499, 200, 20, 0);
      self.ss = "Click to set how far in Action Points these anti supply points are in effect";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart54: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g20id = self.AddSubPart( tsubpart54, 610, 520, 32, 16, 1);
      }
      let mut tsubpart55: SubPartClass =  TextPartClass::new("AntiSupplyRange: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AntiSupplyRange), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g20textid = self.AddSubPart( tsubpart55, 650, 519, 200, 20, 0);
      self.ss = "Click the antisupply points this sftype has versus sea hexes";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart56: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g21id = self.AddSubPart( tsubpart56, 610, 540, 32, 16, 1);
      }
      let mut tsubpart57: SubPartClass =  TextPartClass::new("AntiSupplySea: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AntiSupplySea), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g21textid = self.AddSubPart( tsubpart57, 650, 539, 200, 20, 0);
      self.ss = "Click to set an absolute readiness loss points for every 100ap spent. (50 ap spent is half specified loss)";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart58: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b32id = self.AddSubPart( tsubpart58, 610, 560, 32, 16, 1);
      }
      let mut tsubpart59: SubPartClass =  TextPartClass::new("Abs.Rdnloss100ap: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ReadinessLoss), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: self.ss);
      self.b32textid = self.AddSubPart( tsubpart59, 650, 559, 250, 20, 0);
      self.ss = "Click to set railcap pts..";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart60: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b33id = self.AddSubPart( tsubpart60, 610, 580, 32, 16, 1);
      }
      let mut tsubpart61: SubPartClass =  TextPartClass::new("Railcap: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].RailCap), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b33textid = self.AddSubPart( tsubpart61, 650, 579, 200, 20, 0);
      self.ss = "Click to set regimevar of regime that kills 1 of this sftype to be raised by 1. -1=no regvar raise.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart62: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.h5id = self.AddSubPart( tsubpart62, 610, 600, 32, 16, 1);
      }
      let mut tsubpart63: SubPartClass =  TextPartClass::new("KillIsRegVar: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].KillIsRegVar), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.h5textid = self.AddSubPart( tsubpart63, 650, 599, 200, 20, 0);
      self.ss = "Click to set which Slot Number of the hex attacked by this SFType should be increased by 1 for each attack in each combatround";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart64: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b34id = self.AddSubPart( tsubpart64, 610, 620, 32, 16, 1);
      }
      let mut tsubpart65: SubPartClass =  TextPartClass::new("OnAttackSetSlot: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SlotNumber), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b34textid = self.AddSubPart( tsubpart65, 650, 619, 200, 20, 0);
      self.ss = "Click to set the ratio. 0=no ratio. But for example 2 shows player 2 times as many as their are individuals. Use for historicity purposes.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart66: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w6id = self.AddSubPart( tsubpart66, 310, 560, 32, 16, 1);
      }
      let mut tsubpart67: SubPartClass =  TextPartClass::new("Ratio: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].Ratio), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.w6textid = self.AddSubPart( tsubpart67, 350, 559, 200, 20, 0);
      self.ss = "Click to set Air AP Overrule cost. Leave -1 to keep standard functionality. >-1 means thats the ap cost to move into any hex.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart68: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.x3id = self.AddSubPart( tsubpart68, 310, 580, 32, 16, 1);
      }
      let mut tsubpart69: SubPartClass =  TextPartClass::new("AirOverrule: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AirAPRule), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.x3textid = self.AddSubPart( tsubpart69, 350, 579, 200, 20, 0);
      self.ss = "Click to set CopyFromSFType stat. This is only used by some scripts like those who interprent in the Trooptype Editor.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart70: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w7id = self.AddSubPart( tsubpart70, 910, 120, 32, 16, 1);
      }
      if (self.game.Data.SFTypeObj[self.SFtypeNr].CopyDataFrom > -1)
      {
        let mut tsubpart71: SubPartClass =  TextPartClass::new("CopyFromSFType: " + self.game.Data.SFTypeObj[self.game.Data.SFTypeObj[self.SFtypeNr].CopyDataFrom].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w7textid = self.AddSubPart( tsubpart71, 950, 119, 200, 20, 0);
      }
      else
      {
        let mut tsubpart72: SubPartClass =  TextPartClass::new("CopyFromSFType: NONE", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w7textid = self.AddSubPart( tsubpart72, 950, 119, 200, 20, 0);
      }
      self.ss = "Click to select reinforcement type. Current Type#: " + self.game.Data.SFTypeObj[self.SFtypeNr].ReinforcementType.ToString();
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart73: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w9id = self.AddSubPart( tsubpart73, 310, 440, 32, 16, 1);
      }
      if (self.game.Data.SFTypeObj[self.SFtypeNr].ReinforcementType > -1)
      {
        let mut tsubpart74: SubPartClass =  TextPartClass::new("ReinforcementType: " + self.game.Data.ReinfName[self.game.Data.SFTypeObj[self.SFtypeNr].ReinforcementType], Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w9textid = self.AddSubPart( tsubpart74, 350, 439, 200, 20, 0);
      }
      else
      {
        let mut tsubpart75: SubPartClass =  TextPartClass::new("ReinforcementType: NONE", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w9textid = self.AddSubPart( tsubpart75, 350, 439, 200, 20, 0);
      }
      self.ss = "Click to set if in auto-reinforce phase this unit type should never be returned from a HQ (do for trucks and trains)";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart76: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w10id = self.AddSubPart( tsubpart76, 310, 600, 32, 16, 1);
      }
      let mut tsubpart77: SubPartClass =  TextPartClass::new("DontReturnFromHQ: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].DontReturnFromHQ), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.w10textid = self.AddSubPart( tsubpart77, 350, 599, 200, 20, 0);
      self.ss = "Click to set ConsiderCarry true/false. If false then this sftype weight is added to the prognose weight statistic of a unit it is part of.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart78: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w12id = self.AddSubPart( tsubpart78, 310, 540, 32, 16, 1);
      }
      let mut tsubpart79: SubPartClass =  TextPartClass::new("ConsiderCarry: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ConsiderCarry), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.w12textid = self.AddSubPart( tsubpart79, 350, 539, 200, 20, 0);
      self.ss = "Click to reduce the penalty this SFType gets in the first 2 rounds of combat. 1 =full rulevar penalty. 0.5=half, 0=none.";
      if (Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople)
      {
        let mut tsubpart80: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b36id = self.AddSubPart( tsubpart80, 310, 500, 32, 16, 1);
      }
      let mut tsubpart81: SubPartClass =  TextPartClass::new("FirstRoundPenaltyMod: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FirstRoundPenaltyMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b36textid = self.AddSubPart( tsubpart81, 350, 499, 200, 20, 0);
      self.ss = "Click to set show/hide in info window lists";
      if (Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople)
      {
        let mut tsubpart82: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g24id = self.AddSubPart( tsubpart82, 910, 180, 32, 16, 1);
      }
      let mut tsubpart83: SubPartClass =  TextPartClass::new("DontShowInList: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].DontShowInList), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g24textid = self.AddSubPart( tsubpart83, 950, 179, 200, 20, 0);
      if (self.game.Data.Product >= 6)
      {
        self.ss = "Click to change Start and End Combat Round";
        if (Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople)
        {
          let mut tsubpart84: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.w14id = self.AddSubPart( tsubpart84, 910, 200, 32, 16, 1);
        }
        let mut tsubpart85: SubPartClass =  TextPartClass::new("Start+End Combat Round: " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StartCombatRound) + "," + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].EndCombatRound), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w14textid = self.AddSubPart( tsubpart85, 950, 199, 200, 20, 0);
      }
      self.ss = "Click to select secondary reinforcement type. Be careful with this and read docs since its functionality is very limited.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart86: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w9bid = self.AddSubPart( tsubpart86, 310, 520, 32, 16, 1);
      }
      if (self.game.Data.SFTypeObj[self.SFtypeNr].ReinforcementType2 > -1)
      {
        let mut tsubpart87: SubPartClass =  TextPartClass::new("2nd ReinforcementType: " + self.game.Data.ReinfName[self.game.Data.SFTypeObj[self.SFtypeNr].ReinforcementType2], Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w9btextid = self.AddSubPart( tsubpart87, 350, 519, 200, 20, 0);
      }
      else
      {
        let mut tsubpart88: SubPartClass =  TextPartClass::new("2nd ReinforcementType: NONE", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w9btextid = self.AddSubPart( tsubpart88, 350, 519, 200, 20, 0);
      }
      self.tabsheet1b();
    }

    pub fn tabsheet1b()
    {
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.detailnr <= -1)
        return;
      self.ss = "Click to toggle on/off if this sftype can be recruited from selected peoplegroup";
      if (Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B4Id = self.AddSubPart( tsubpart, 10, 620, 32, 16, 1);
      }
      if (!(Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople))
        return;
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Change Value", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B4TextId = self.AddSubPart( tsubpart1, 50, 619, 400, 20, 0);
    }

    pub fn tabsheet2()
    {
      self.ss = "Click to set the initiative of this sftype if attacking and if defending";
      str1: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].Initiative) + " / " + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].InitiativeDef);
      SubPartClass tsubpart;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b11id = self.AddSubPart( tsubpart, 10, 380, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Initiative: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b11textid = self.AddSubPart( tsubpart, 50, 379, 200, 20, 0);
      self.ss = "Click to set the number of attacks this sftype can do every combatround (10 ap per combatround)";
      str2: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].Attacks);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b12id = self.AddSubPart( tsubpart, 10, 400, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Attacks: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b12textid = self.AddSubPart( tsubpart, 50, 399, 200, 20, 0);
      self.ss = "Click to set the max number of times this sftype can be attacked before these attacks get penalties";
      str3: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].MaxAttacked);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b13id = self.AddSubPart( tsubpart, 10, 420, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("MaxAttacked: " + str3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b13textid = self.AddSubPart( tsubpart, 50, 419, 200, 20, 0);
      self.ss = "Click to set the stackpoints this sftype consumes";
      str4: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].Frontage);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b14id = self.AddSubPart( tsubpart, 10, 440, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Stack Pts: " + str4, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b14textid = self.AddSubPart( tsubpart, 50, 439, 200, 20, 0);
      self.ss = "Click to toggle on/off if this sftype is a rear area sftype (instead of frontline)";
      str5: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].BackBench);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b15id = self.AddSubPart( tsubpart, 10, 460, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Rear Area: " + str5, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b15textid = self.AddSubPart( tsubpart, 50, 459, 200, 20, 0);
      self.ss = "Click to set artillery range. Range of 0 means it has no artillery capability.";
      str6: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ArtRange);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b16id = self.AddSubPart( tsubpart, 10, 480, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Art.Range: " + str6, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b16textid = self.AddSubPart( tsubpart, 50, 479, 200, 20, 0);
      self.ss = "Click to set the number of random enemy individuals the sftype can browse through to select a best opponent.";
      str7: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FavTargetTries);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b17id = self.AddSubPart( tsubpart, 10, 500, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("FavTarget Tries: " + str7, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b17textid = self.AddSubPart( tsubpart, 50, 499, 200, 20, 0);
      self.ss = "Click to set the range of the Anti-Air power of this sftype.";
      str8: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AARange);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g16id = self.AddSubPart( tsubpart, 10, 520, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("AARange: " + str8, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.g16textid = self.AddSubPart( tsubpart, 50, 519, 400, 20, 0);
      self.ss = "Click to set the percentchance that a hit by this sftype is a kill";
      str9: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].KillPercent)) + "% on target";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b20id = self.AddSubPart( tsubpart, 10, 540, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Kill%: " + str9, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b20textid = self.AddSubPart( tsubpart, 50, 539, 200, 20, 0);
      self.ss = "Click to set the percentchance that a hit by this sftype is a retreat for the attacked individual";
      str10: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].RetreatPercent)) + "% on target";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b22id = self.AddSubPart( tsubpart, 10, 560, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Retreat%: " + str10, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.b22textid = self.AddSubPart( tsubpart, 50, 559, 400, 20, 0);
      self.ss = "Click to change the description of the sftype.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b27id = self.AddSubPart( tsubpart, 310, 360, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  TextPartClass::new("Change Description", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b27textid = self.AddSubPart( tsubpart, 350, 359, 200, 20, 0);
      }
      self.ss = "Click to let this SFType use the LandscapeMod table of another SFType for artillery attacks.";
      str11: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ArtSFType);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w13id = self.AddSubPart( tsubpart, 10, 580, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Artillery Mod Sftyp: " + str11, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.w13textid = self.AddSubPart( tsubpart, 50, 579, 200, 20, 0);
      self.ss = "If ind. scores a RETREAT or KILL hit on enemy (that consumed supply last turn) it has a 0.x chance to get killed. 0.05=5% chance. Only done for att side! ";
      str12: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ChanceOnDeathIfMakeHit) + "%";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w133id = self.AddSubPart( tsubpart, 10, 600, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("ChanceOnDeathIfMakeHit: " + str12, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: self.ss);
      self.w133textid = self.AddSubPart( tsubpart, 50, 599, 250, 20, 0);
      self.ss = "";
      str13: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].directRange);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w15id = self.AddSubPart( tsubpart, 10, 620, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("DirectRange: " + str13, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: self.ss);
      self.w15textid = self.AddSubPart( tsubpart, 50, 619, 250, 20, 0);
      self.ss = "";
      str14: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].directModFirstHex);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w16id = self.AddSubPart( tsubpart, 10, 640, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("DirectModFirstHex: " + str14, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: self.ss);
      self.w16textid = self.AddSubPart( tsubpart, 50, 639, 250, 20, 0);
      self.ss = "";
      str15: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].directModPerHex);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.w17id = self.AddSubPart( tsubpart, 10, 660, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("DirectModPerHex: " + str15, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: self.ss);
      self.w17textid = self.AddSubPart( tsubpart, 50, 659, 250, 20, 0);
      tText: String = self.game.Data.SFTypeObj[self.SFtypeNr].Description;
      if (self.game.Data.Product >= 7)
      {
        let mut index: i32 = 0;
        do
        {
          if (self.game.Data.SFTypeObj[self.SFtypeNr].SFTypeVar[index] > 0)
            tText = tText + "\r\n" + "SFTYPEVAR_" + index.ToString() + "=" + self.game.Data.SFTypeObj[self.SFtypeNr].SFTypeVar[index].ToString();
          index += 1;
        }
        while (index <= 99);
      }
      tsubpart =  new TextAreaClass(self.game, 650, 10, Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), "Description", false, tText, Color.White, tbackbitmap: ( self.OwnBitmap), bbx: 310, bby: 390);
      self.b28id = self.AddSubPart( tsubpart, 310, 390, 650, 208, 0);
    }

    pub fn tabsheet9()
    {
      self.ss = "Which regimevar # is used as fuel resource";
      str1: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FuelRegimeVar);
      SubPartClass tsubpart;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c2id = self.AddSubPart( tsubpart, 10, 380, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("FuelRegimeVar: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c2textid = self.AddSubPart( tsubpart, 50, 379, 200, 20, 0);
      self.ss = "For every 10AP the SFType moves it needs this QTY of fuel.";
      str2: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FuelForMove);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c3id = self.AddSubPart( tsubpart, 10, 400, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("FuelForMove(10ap): " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c3textid = self.AddSubPart( tsubpart, 50, 399, 200, 20, 0);
      self.ss = "If the fuel is not available movement cost will be multiplied.. 2=double movement cost. 3=3x movement cost ";
      str3: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelMove);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c4id = self.AddSubPart( tsubpart, 10, 420, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OutOfFuelMove: " + str3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c4textid = self.AddSubPart( tsubpart, 50, 419, 200, 20, 0);
      self.ss = "For every combatround (10AP) the SFType needs this QTY of fuel/";
      str4: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FuelForAttack) + "/" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FuelForAttackDef);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c5id = self.AddSubPart( tsubpart, 10, 440, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("FuelForAttack(10ap): " + str4, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c5textid = self.AddSubPart( tsubpart, 50, 439, 200, 20, 0);
      self.ss = "If fuel is not available in a given combatround and the SFType is attacking. Its strength will be modified by X. 0.5=halved.";
      str5: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelAttack);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c6id = self.AddSubPart( tsubpart, 10, 460, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OutOfFuelAttack: " + str5, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c6textid = self.AddSubPart( tsubpart, 50, 459, 200, 20, 0);
      self.ss = "If fuel is not available in a given combatround and the SFType is defending. Its strength will be modified by X. 0.5=halved.";
      str6: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelDefense);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c7id = self.AddSubPart( tsubpart, 10, 480, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OutOfFuelDefense: " + str6, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c7textid = self.AddSubPart( tsubpart, 50, 479, 200, 20, 0);
      self.ss = "Copy fuel stats from specified SFType number";
      Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelDefense);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.y2id = self.AddSubPart( tsubpart, 10, 520, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Copy fuel stats from..", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.y1textid = self.AddSubPart( tsubpart, 50, 519, 200, 20, 0);
      self.ss = "Howmuch supply is taken out of the stockpile per round of attack. 0=no stockpile rule.";
      str7: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StockpileUsedPerRound);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c12id = self.AddSubPart( tsubpart, 410, 380, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("StockUsePerRound: " + str7, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c12textid = self.AddSubPart( tsubpart, 450, 379, 200, 20, 0);
      self.ss = "Maximum size of the stockpile";
      str8: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StockpileMax);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c13id = self.AddSubPart( tsubpart, 410, 400, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("StockPileMax: " + str8, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c13textid = self.AddSubPart( tsubpart, 450, 399, 200, 20, 0);
      self.ss = "0=no maximum/rule not used. Otherwise maximum stockpile supply request in per round.";
      str9: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StockpileMaxIn);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c14id = self.AddSubPart( tsubpart, 410, 420, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("StockPileMaxIn: " + str9, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c14textid = self.AddSubPart( tsubpart, 450, 419, 200, 20, 0);
      self.ss = "Any attack made by this sftype, artillery or otherwise is modified with out of stockmod when no stockpile left.";
      str10: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].StockpileDepletedMod);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c15id = self.AddSubPart( tsubpart, 410, 440, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OutofStockMod): " + str10, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c15textid = self.AddSubPart( tsubpart, 450, 439, 200, 20, 0);
      self.ss = "0=no maximum/rule not used. Otherwise its the maximum supply request in per round.";
      str11: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SupplyMaxIn);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c16id = self.AddSubPart( tsubpart, 410, 480, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("SupplyMaxIn " + str11, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c16textid = self.AddSubPart( tsubpart, 450, 479, 200, 20, 0);
      self.ss = ".";
      str12: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SupplyForAttack);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c17id = self.AddSubPart( tsubpart, 710, 380, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("SupplyForAttack: " + str12, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c17textid = self.AddSubPart( tsubpart, 750, 379, 200, 20, 0);
      self.ss = ".";
      str13: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].SupplyForAttackDef);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c18id = self.AddSubPart( tsubpart, 710, 400, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("SupplyForAttackDef: " + str13, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c18textid = self.AddSubPart( tsubpart, 750, 399, 200, 20, 0);
      self.ss = ".";
      str14: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].OutOfSupplyAttack);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c19id = self.AddSubPart( tsubpart, 710, 420, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OutOfSupplyAttack: " + str14, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c19textid = self.AddSubPart( tsubpart, 750, 419, 200, 20, 0);
      self.ss = ".";
      str15: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].OutOfSupplyDefense);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c20id = self.AddSubPart( tsubpart, 710, 440, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OutOfSupplyDefense: " + str15, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c20textid = self.AddSubPart( tsubpart, 750, 439, 200, 20, 0);
      self.ss = ".";
      str16: String = Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FuelCarry);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.c21id = self.AddSubPart( tsubpart, 710, 480, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("FuelCarry: " + str16, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.c21textid = self.AddSubPart( tsubpart, 750, 479, 200, 20, 0);
    }

    pub fn tabsheet3()
    {
      self.CombatListObj = ListClass::new();
      if (self.detailnr < -1 | self.detailnr > 99)
        self.detailnr = -1;
      let mut index: i32 = 0;
      do
      {
        str1: String = "";
        str2: String = Conversion.Str( index) + ") " + self.game.Data.TempString[index + 400];
        if (Strings.Len(str2) > 15)
          str2 = Strings.Left(str2, 15);
        str3: String = str1 + str2 + Strings.Space(25 - Strings.Len(str2));
        Expression1: String = "Fav=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FavTarget[index]));
        str4: String = str3 + Expression1 + Strings.Space(12 - Strings.Len(Expression1));
        Expression2: String = "Pow=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AttackPower[index]));
        str5: String = str4 + Expression2 + Strings.Space(12 - Strings.Len(Expression2));
        Expression3: String = "PowDef=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AttackPowerDef[index]));
        str6: String = str5 + Expression3 + Strings.Space(12 - Strings.Len(Expression3));
        Expression4: String = "ArtPow=" + Strings.Trim(Conversion.Str(RuntimeHelpers.GetObjectValue(self.game.Data.SFTypeObj[self.SFtypeNr].AttackArt[index])));
        str7: String = str6 + Expression4 + Strings.Space(12 - Strings.Len(Expression4));
        Expression5: String = "ArtFav=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FavArtTarget[index]));
        str8: String = str7 + Expression5 + Strings.Space(12 - Strings.Len(Expression5));
        Expression6: String = "HitPts=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].HitPoints[index]));
        self.CombatListObj.add(str8 + Expression6 + Strings.Space(12 - Strings.Len(Expression6)) + ("HitPtsDef=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].HitPointsDef[index]))), index);
        index += 1;
      }
      while (index <= 99);
      if (self.game.ScreenHeight >= 800)
      {
        ListClass combatListObj = self.CombatListObj;
        let mut detailnr: i32 = self.detailnr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(combatListObj, 16, 880, detailnr, game, true, tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
        self.CombatListId = self.AddSubPart( tsubpart, 10, 340, 880, 304, 0);
      }
      else
      {
        ListClass combatListObj = self.CombatListObj;
        let mut detailnr: i32 = self.detailnr;
        let mut game: GameClass = self.game;
         local3: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local4: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(combatListObj, 12, 880, detailnr, game, true, tbackbitmap: ( local3), bbx: 10, bby: 340, overruleFont: ( local4));
        self.CombatListId = self.AddSubPart( tsubpart, 10, 340, 880, 240, 0);
      }
      if (self.detailnr <= -1)
        return;
      self.tabsheet3b();
    }

    pub fn tabsheet3b()
    {
      self.ss = "Click to set how favourite this unitgroup is as a target for this sftype. the higher the more favourite.";
      str1: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FavTarget[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b18id = self.AddSubPart( tsubpart, 910, 340, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Fav: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b18textid = self.AddSubPart( tsubpart, 950, 339, 400, 20, 0);
      }
      self.ss = "Click to set the attackpower of this sftype in offense versus this unitgroup.";
      str2: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AttackPower[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b19id = self.AddSubPart( tsubpart, 910, 360, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Pow: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b19textid = self.AddSubPart( tsubpart, 950, 359, 400, 20, 0);
      }
      self.ss = "Click to set the attackpower of this sftype in defense versus this unitgroup.";
      str3: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AttackPowerDef[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b23id = self.AddSubPart( tsubpart, 910, 380, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("PowDef: " + str3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b23textid = self.AddSubPart( tsubpart, 950, 379, 400, 20, 0);
      }
      self.ss = "Click to set the attackpower of this sftype versus this unitgroup if it does an artillery attack";
      str4: String = Strings.Trim(Conversion.Str(RuntimeHelpers.GetObjectValue(self.game.Data.SFTypeObj[self.SFtypeNr].AttackArt[self.detailnr])));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b25id = self.AddSubPart( tsubpart, 910, 400, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("ArtPow: " + str4, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b25textid = self.AddSubPart( tsubpart, 950, 399, 400, 20, 0);
      }
      self.ss = "Click to set how favourite this unitgroup is as a target for an artillery attack of this sftype. the higher the more favourite.";
      str5: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].FavArtTarget[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b26id = self.AddSubPart( tsubpart, 910, 420, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("ArtFav: " + str5, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b26textid = self.AddSubPart( tsubpart, 950, 419, 400, 20, 0);
      }
      self.ss = "Click to set hitpoints when attacking a hex.";
      str6: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].HitPoints[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b37id = self.AddSubPart( tsubpart, 910, 440, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("HitPoints: " + str6, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b37textid = self.AddSubPart( tsubpart, 950, 439, 400, 20, 0);
      }
      self.ss = "Click to set hitpoints when defending a hex.";
      str7: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].HitPointsDef[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.b38id = self.AddSubPart( tsubpart, 910, 460, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("HitPointsDef: " + str7, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b38textid = self.AddSubPart( tsubpart, 950, 459, 400, 20, 0);
      }
      self.ss = "Click to copy the stats in this table from another SFtype.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.copyid = self.AddSubPart( tsubpart, 910, 480, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Copy combattable from ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.copytextid = self.AddSubPart( tsubpart, 950, 479, 400, 20, 0);
      }
      self.ss = "Click to set all att/def hitpoints in 1 go.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.y1id = self.AddSubPart( tsubpart, 910, 500, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Set ALL hitpoints ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.y1textid = self.AddSubPart( tsubpart1, 950, 499, 400, 20, 0);
    }

    pub fn tabsheet4()
    {
      self.CombatList2Obj = ListClass::new();
      if (self.detailnr < -1 | self.detailnr > self.game.Data.LandscapeTypeCounter)
        self.detailnr = -1;
      let mut landscapeTypeCounter: i32 = self.game.Data.LandscapeTypeCounter;
      for (let mut index: i32 = 0; index <= landscapeTypeCounter; index += 1)
      {
        str1: String = "";
        Expression1: String = Conversion.Str( index) + ") " + self.game.Data.LandscapeTypeObj[index].Name;
        if (Strings.Len(Expression1) > 30)
          Expression1 = Strings.Left(str1, 15);
        str2: String = str1 + Expression1 + Strings.Space(29 - Math.Min(28, Strings.Len(Expression1)));
        Expression2: String = "Att=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].CombatModAtt[index]));
        str3: String = str2 + Expression2 + Strings.Space(13 - Strings.Len(Expression2));
        Expression3: String = "Def=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].CombatModDef[index]));
        tname: String = str3 + Expression3 + Strings.Space(13 - Strings.Len(Expression3));
        if ( self.game.Data.RuleVar[900] > 0.0)
        {
          str4: String = "ExtraRecon=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ExtraRecon[index]));
          tname += str4;
        }
        self.CombatList2Obj.add(tname, index);
      }
      ListClass combatList2Obj = self.CombatList2Obj;
      let mut detailnr: i32 = self.detailnr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(combatList2Obj, 12, 580, detailnr, game, true, tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      self.CombatList2Id = self.AddSubPart( tsubpart, 10, 340, 580, 240, 0);
      if (self.detailnr <= -1)
        return;
      self.tabsheet4b();
    }

    pub fn tabsheet4b()
    {
      self.ss = "Click to set the modifier for this sftype if it attacks this landscape. 1=no mod, 0.5=half power, 1.5=+50% power";
      str1: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].CombatModAtt[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g13id = self.AddSubPart( tsubpart, 610, 340, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Att: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.g13textid = self.AddSubPart( tsubpart, 650, 339, 400, 20, 0);
      }
      self.ss = "Click to set the modifier for this sftype if it defends in this landscape. 1=no mod, 0.5=half power, 1.5=+50% power";
      str2: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].CombatModDef[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.g14id = self.AddSubPart( tsubpart, 610, 360, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Def: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.g14textid = self.AddSubPart( tsubpart, 650, 359, 400, 20, 0);
      }
      if ( self.game.Data.RuleVar[900] > 0.0)
      {
        self.ss = "Click to set the recon value this SFType has if it looks through a special connection. Only for its main hex to direct connections.";
        str3: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ExtraRecon[self.detailnr]));
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.g23id = self.AddSubPart( tsubpart, 610, 380, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  TextPartClass::new("ExtRec: " + str3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
          self.g23textid = self.AddSubPart( tsubpart, 650, 379, 400, 20, 0);
        }
      }
      self.ss = "Click to set these att and def modifiers for all sftypes with the same unitgroup.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.w8id = self.AddSubPart( tsubpart, 610, 400, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Set for all (unitgroup)", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.w8textid = self.AddSubPart( tsubpart, 650, 399, 400, 20, 0);
      }
      self.ss = "Click to set these att and def modifiers for all sftypes with the same unitgroup.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.b39id = self.AddSubPart( tsubpart, 610, 420, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Set for all (reinfgroup)", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.b39textid = self.AddSubPart( tsubpart, 650, 419, 400, 20, 0);
      }
      self.ss = "Click to copy from a specific SFType #";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.t1id = self.AddSubPart( tsubpart, 610, 450, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Copy from SFType #", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.t1textid = self.AddSubPart( tsubpart1, 650, 449, 400, 20, 0);
    }

    pub fn tabsheet7()
    {
      if (self.detailnr2 > 99)
        self.detailnr2 = -1;
      self.LogoListObj = ListClass::new();
      let mut index: i32 = 0;
      do
      {
        self.LogoListObj.add(Conversion.Str( index) + ") " + self.game.Data.TempString[1100 + index] + " = '" + self.game.Data.SFTypeObj[self.SFtypeNr].LogoString[index] + "'" + " , nato=" + self.game.Data.TempString[1000 + index], index);
        index += 1;
      }
      while (index <= 99);
      ListClass logoListObj = self.LogoListObj;
      let mut detailnr2: i32 = self.detailnr2;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(logoListObj, 10, 350, detailnr2, game, true, "Logostrings", tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      self.LogoListId = self.AddSubPart( tsubpart1, 10, 340, 350, 208, 0);
      if (self.detailnr2 <= -1)
        return;
      self.ss = "Set string , no is: String no stat and it will not be shown.";
      SubPartClass tsubpart2;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.j1id = self.AddSubPart( tsubpart2, 10, 570, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Set string ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.j1textid = self.AddSubPart( tsubpart2, 50, 569, 400, 20, 0);
    }

    pub fn tabsheet8()
    {
      if (self.detailnr2 > self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter)
        self.detailnr2 = -1;
      self.PreventListObj = ListClass::new();
      let mut preventCounter: i32 = self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter;
      for (let mut index: i32 = 0; index <= preventCounter; index += 1)
      {
        str1: String = Conversion.Str( index) + ") ";
        str2: String = self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn[index] <= -1 ? "ALL" : self.game.Data.TempString[400 + self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn[index]];
        if (Strings.Len(str2) > 12)
          str2 = Strings.Left(str2, 12);
        str3: String = str1 + str2 + Strings.Space(15 - Strings.Len(str2));
        str4: String = self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom[index] <= -1 ? "ALL" : self.game.Data.TempString[400 + self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom[index]];
        if (Strings.Len(str4) > 12)
          str4 = Strings.Left(str4, 12);
        str5: String = str3 + str4 + Strings.Space(15 - Strings.Len(str4));
        Expression1: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority[index]));
        str6: String = str5 + Expression1 + Strings.Space(10 - Strings.Len(Expression1));
        Expression2: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance[index])) + "%";
        self.PreventListObj.add(str6 + Expression2 + Strings.Space(10 - Strings.Len(Expression2)) + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints[index])), index);
      }
      ListClass preventListObj = self.PreventListObj;
      let mut detailnr2: i32 = self.detailnr2;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(preventListObj, 9, 450, detailnr2, game, true, "#  ON               FROM            PRIORITY    CHANCE   POINTS", tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      self.PreventListId = self.AddSubPart( tsubpart1, 10, 340, 450, 192, 0);
      SubPartClass tsubpart2;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.p1id = self.AddSubPart( tsubpart2, 10, 550, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("add a prevent ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.p1textid = self.AddSubPart( tsubpart2, 50, 549, 400, 20, 0);
      if (self.detailnr2 > -1)
      {
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
          self.p2id = self.AddSubPart( tsubpart2, 10, 570, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("remove this prevent ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.p2textid = self.AddSubPart( tsubpart2, 50, 569, 400, 20, 0);
        self.ss = "";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.p3id = self.AddSubPart( tsubpart2, 510, 340, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Prevent Hit On", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.p3textid = self.AddSubPart( tsubpart2, 550, 339, 400, 20, 0);
        self.ss = "";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.p4id = self.AddSubPart( tsubpart2, 510, 360, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Prevent Hit From", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.p4textid = self.AddSubPart( tsubpart2, 550, 359, 400, 20, 0);
        self.ss = "";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.p5id = self.AddSubPart( tsubpart2, 510, 380, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Prevent Priority", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.p5textid = self.AddSubPart( tsubpart2, 550, 379, 400, 20, 0);
        self.ss = "";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.p6id = self.AddSubPart( tsubpart2, 510, 400, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Prevent Chance", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.p6textid = self.AddSubPart( tsubpart2, 550, 399, 400, 20, 0);
        self.ss = "";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.p7id = self.AddSubPart( tsubpart2, 510, 420, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Prevent Points", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.p7textid = self.AddSubPart( tsubpart2, 550, 419, 400, 20, 0);
      }
      self.ss = "How many prevent points can this sftype provide to sheltering other sftypes";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.p8id = self.AddSubPart( tsubpart2, 810, 340, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("MaxPrvPointsUsed=" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].MaxPreventPointsUsed), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.p8textid = self.AddSubPart( tsubpart2, 850, 339, 400, 20, 0);
      self.ss = "How many preventers pts can this sftype use to be prevented it self by another sftype";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.p9id = self.AddSubPart( tsubpart2, 810, 360, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("MaxPrvPointsGiven" + Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].MaxPreventPointsGiven), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.p9textid = self.AddSubPart( tsubpart2, 850, 359, 400, 20, 0);
    }

    pub fn tabsheet10()
    {
      if (self.detailnr2 > self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter)
        self.detailnr2 = -1;
      self.VariantListObj = ListClass::new();
      let mut modelVariantCounter: i32 = self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter;
      for (let mut index: i32 = 0; index <= modelVariantCounter; index += 1)
      {
        str1: String = Conversion.Str( index) + ") ";
        str2: String = self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName[index];
        if (Strings.Len(str2) > 28)
          str2 = Strings.Left(str2, 28);
        str3: String = str1 + str2 + Strings.Space(30 - Strings.Len(str2));
        Expression: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck[index]));
        self.VariantListObj.add(str3 + Expression + Strings.Space(10 - Strings.Len(Expression)) + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec[index])), index);
      }
      ListClass variantListObj = self.VariantListObj;
      let mut detailnr2: i32 = self.detailnr2;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(variantListObj, 9, 450, detailnr2, game, true, "#  ALTERATION NAME              CHECK    EXEC", tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      self.VariantListId = self.AddSubPart( tsubpart1, 10, 340, 450, 192, 0);
      SubPartClass tsubpart2;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.vp1id = self.AddSubPart( tsubpart2, 10, 550, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("add an alteration ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.vp1textid = self.AddSubPart( tsubpart2, 50, 549, 400, 20, 0);
      if (self.detailnr2 <= -1)
        return;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
        self.vp2id = self.AddSubPart( tsubpart2, 10, 570, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("remove this alteration ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.vp2textid = self.AddSubPart( tsubpart2, 50, 569, 400, 20, 0);
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.vp3id = self.AddSubPart( tsubpart2, 510, 340, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Set Name", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.vp3textid = self.AddSubPart( tsubpart2, 550, 339, 400, 20, 0);
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.vp4id = self.AddSubPart( tsubpart2, 510, 360, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Set Check", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.vp4textid = self.AddSubPart( tsubpart2, 550, 359, 400, 20, 0);
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.vp5id = self.AddSubPart( tsubpart2, 510, 380, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Set Exec", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.vp5textid = self.AddSubPart( tsubpart2, 550, 379, 400, 20, 0);
    }

    pub fn tabsheet5()
    {
      if (self.detailnr2 > self.game.Data.ResearchCounter)
        self.detailnr2 = -1;
      self.ResListObj = ListClass::new();
      let mut researchCounter: i32 = self.game.Data.ResearchCounter;
      for (let mut index1: i32 = 0; index1 <= researchCounter; index1 += 1)
      {
        str1: String = "";
        str2: String = Conversion.Str( index1) + ") " + self.game.Data.ResearchObj[index1].Name;
        if (Strings.Len(str2) > 17)
          str2 = Strings.Left(str2, 17);
        str3: String = str1 + str2 + Strings.Space(19 - Strings.Len(str2));
        Expression1: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelLastState[index1]));
        str4: String = str3 + Expression1 + Strings.Space(5 - Strings.Len(Expression1));
        Expression2: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelPossibleImp[index1]));
        str5: String = str4 + Expression2 + Strings.Space(5 - Strings.Len(Expression2));
        Expression3: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelImproveEvent[index1]));
        str6: String = str5 + Expression3 + Strings.Space(5 - Strings.Len(Expression3));
        let mut Number: i32 = 0;
        let mut index2: i32 = 1;
        while (self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[index2] != index1)
        {
          index2 += 1;
          if (index2 > 9)
            goto label_9;
        }
        Number = index2;
label_9:
        Expression4: String = Strings.Trim(Conversion.Str( Number));
        self.ResListObj.add(str6 + Expression4 + Strings.Space(3 - Strings.Len(Expression4)) + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelAutoImprovement[index1])), index1);
      }
      ListClass resListObj = self.ResListObj;
      let mut detailnr2: i32 = self.detailnr2;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(resListObj, 10, 350, detailnr2, game, true, "NAME           ST   POS   EV   RES  AUTO", tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      self.ResListId = self.AddSubPart( tsubpart1, 10, 340, 350, 208, 0);
      SubPartClass tsubpart2;
      if (self.detailnr2 > -1)
      {
        self.ss = "Set ModelLastState";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.v1id = self.AddSubPart( tsubpart3, 10, 570, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Modellaststate ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.v1textid = self.AddSubPart( tsubpart2, 50, 569, 400, 20, 0);
        self.ss = "Set Possible Improvement";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.v2id = self.AddSubPart( tsubpart2, 10, 590, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Possible Improvement ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.v2textid = self.AddSubPart( tsubpart2, 50, 589, 400, 20, 0);
        self.ss = "Set Improve Event";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.v3id = self.AddSubPart( tsubpart2, 10, 610, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Improve Event ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.v3textid = self.AddSubPart( tsubpart2, 50, 609, 400, 20, 0);
        self.ss = "Set Research for Level";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.v4id = self.AddSubPart( tsubpart2, 10, 630, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("Set Research for Level ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.v4textid = self.AddSubPart( tsubpart2, 50, 629, 400, 20, 0);
        self.ss = "Change if it is auto-improvement field";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.v16id = self.AddSubPart( tsubpart2, 10, 650, 32, 16, 1);
        }
        tsubpart2 =  TextPartClass::new("ModelAutoimprovement ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
        self.v16textid = self.AddSubPart( tsubpart2, 50, 649, 400, 20, 0);
      }
      self.ss = "";
      str7: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelIsBase));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v5id = self.AddSubPart( tsubpart2, 410, 340, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelisBase = " + str7, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v5textid = self.AddSubPart( tsubpart2, 450, 339, 400, 20, 0);
      self.ss = "";
      str8: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostType));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v6id = self.AddSubPart( tsubpart2, 410, 360, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelCostType = " + str8, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v6textid = self.AddSubPart( tsubpart2, 450, 359, 400, 20, 0);
      self.ss = "";
      str9: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelCost));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v7id = self.AddSubPart( tsubpart2, 410, 380, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelCost = " + str9, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v7textid = self.AddSubPart( tsubpart2, 450, 379, 400, 20, 0);
      self.ss = "";
      str10: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerLevel));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v8id = self.AddSubPart( tsubpart2, 410, 400, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelCostPerLevel = " + str10, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v8textid = self.AddSubPart( tsubpart2, 450, 399, 400, 20, 0);
      self.ss = "";
      str11: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerSameModel));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v9id = self.AddSubPart( tsubpart2, 410, 420, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelCostPerSameModel= " + str11, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v9textid = self.AddSubPart( tsubpart2, 450, 419, 400, 20, 0);
      self.ss = "";
      str12: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelNewEvent));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v10id = self.AddSubPart( tsubpart2, 410, 440, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelNewEvent = " + str12, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v10textid = self.AddSubPart( tsubpart2, 450, 439, 400, 20, 0);
      self.ss = "";
      str13: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelNameList));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v11id = self.AddSubPart( tsubpart2, 410, 460, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelNameList strlID= " + str13, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v11textid = self.AddSubPart( tsubpart2, 450, 459, 400, 20, 0);
      self.ss = "";
      str14: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowUpgrade));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v12id = self.AddSubPart( tsubpart2, 410, 480, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelAllowUpgrade = " + str14, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.v12textid = self.AddSubPart( tsubpart2, 450, 479, 400, 20, 0);
      self.ss = "";
      str15: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowImprovements));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v13id = self.AddSubPart( tsubpart2, 710, 340, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelAllowImprovements= " + str15, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v13textid = self.AddSubPart( tsubpart2, 750, 339, 400, 20, 0);
      self.ss = "";
      str16: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelImproveCostMod));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v14id = self.AddSubPart( tsubpart2, 710, 360, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelImproveCost= " + str16, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v14textid = self.AddSubPart( tsubpart2, 750, 359, 400, 20, 0);
      self.ss = "";
      str17: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelItemType));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v15id = self.AddSubPart( tsubpart2, 710, 380, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Modelitemtype= " + str17, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v15textid = self.AddSubPart( tsubpart2, 750, 379, 400, 20, 0);
      self.ss = "";
      str18: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelRegime));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v17id = self.AddSubPart( tsubpart2, 710, 420, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Modelregime= " + str18, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v17textid = self.AddSubPart( tsubpart2, 750, 419, 400, 20, 0);
      self.ss = "You need to have this research before you can make a NEW of this basemodel.";
      str19: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[0]));
      if (self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[0] > -1)
        str19 = self.game.Data.ResearchObj[self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[0]].Name;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v18id = self.AddSubPart( tsubpart2, 710, 440, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Modelresearch(0)= " + str19, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v18textid = self.AddSubPart( tsubpart2, 750, 439, 400, 20, 0);
      self.ss = "";
      str20: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelInitialForAll));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v19id = self.AddSubPart( tsubpart2, 710, 460, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelInitialForAll= " + str20, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v19textid = self.AddSubPart( tsubpart2, 750, 459, 400, 20, 0);
      str21: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelInitialEvent));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v20id = self.AddSubPart( tsubpart2, 710, 480, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelInitialevent= " + str21, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v20textid = self.AddSubPart( tsubpart2, 750, 479, 400, 20, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.v21id = self.AddSubPart( tsubpart2, 710, 500, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Copy all Model settings from #", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v21textid = self.AddSubPart( tsubpart2, 750, 499, 400, 20, 0);
      str22: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelExtraResearch));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v22id = self.AddSubPart( tsubpart2, 710, 520, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelExtraResearch=" + str22, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v22textid = self.AddSubPart( tsubpart2, 750, 519, 400, 20, 0);
      self.ss = "Modifies the setting of upgrade cost for upgrading an SFType in the field for an old model. 1=normal";
      str23: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].ModelSFTypeUpgrade));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.v23id = self.AddSubPart( tsubpart2, 710, 540, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("ModelSFTypeUpgrade=" + str23, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.v23textid = self.AddSubPart( tsubpart2, 750, 539, 400, 20, 0);
    }

    pub fn tabsheet5b()
    {
    }

    pub fn tabsheet6()
    {
      self.CombatList4Obj = ListClass::new();
      if (self.detailnr < -1 | self.detailnr > 99)
        self.detailnr = -1;
      let mut num1: i32 = -1;
      let mut num2: i32 = -1;
      let mut index: i32 = 1;
      do
      {
        str: String = "";
        Expression: String = Conversion.Str( index) + ") ";
        if (index == 1)
          Expression += "Staff";
        if (index == 2)
          Expression += "";
        if (index == 3)
          Expression += "";
        if (index == 4)
          Expression += "";
        if (index == 5)
          Expression += "Engineer";
        if (index == 6)
          Expression += "Infantry";
        if (index == 7)
          Expression += "Inf-Support";
        if (index == 8)
          Expression += "Artillery";
        if (index == 9)
          Expression += "Mobilizer";
        if (index == 10)
          Expression += "Armour";
        if (index == 11)
          Expression += "";
        if (index == 12)
          Expression += "AA";
        if (index == 13)
          Expression += "Fighter";
        if (index == 14)
          Expression += "Bomber Tactical";
        if (index == 15)
          Expression += "";
        if (index == 16)
          Expression += "";
        if (index == 17)
          Expression += "Cargoship";
        if (index == 18)
          Expression += "Naval Supriority";
        if (index == 19)
          Expression += "Raider";
        if (index == 20)
          Expression += "";
        if (index == 21)
          Expression += "";
        if (index == 22)
          Expression += "";
        if (Strings.Len(Expression) > 30)
          Expression = Strings.Left(str, 15);
        tname: String = str + Expression + Strings.Space(30 - Strings.Len(Expression)) + ("Score=" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AIRoleScore[index])));
        num2 += 1;
        if (self.detailnr == index)
          num1 = num2;
        self.CombatList4Obj.add(tname, index);
        index += 1;
      }
      while (index <= 49);
      ListClass combatList4Obj = self.CombatList4Obj;
      let mut tlistselect: i32 = num1;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(combatList4Obj, 12, 580, tlistselect, game, true, tbackbitmap: ( local1), bbx: 10, bby: 340, overruleFont: ( local2));
      self.combatlist4id = self.AddSubPart( tsubpart, 10, 340, 580, 240, 0);
      if (self.detailnr <= -1)
        return;
      self.tabsheet6b();
    }

    pub fn tabsheet6b()
    {
      self.ss = "Set the AIRolescore for this sftype. Basicly you set 100 at the role it is supposed to be used at.";
      str: String = Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AIRoleScore[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.h3id = self.AddSubPart( tsubpart, 610, 340, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("AIRoleScore: " + str, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.h3textid = self.AddSubPart( tsubpart, 650, 339, 400, 20, 0);
      }
      self.ss = "Set the AIRolescore for this sftype. And all with the same Unitgroup";
      Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.SFtypeNr].AIRoleScore[self.detailnr]));
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.w11id = self.AddSubPart( tsubpart, 610, 380, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Set for all", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.w11textid = self.AddSubPart( tsubpart1, 650, 379, 400, 20, 0);
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
            if (num1 == self.SFtypeListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.SFtypeNr = num2;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                self.MakeSFtypeTypeItemGUI();
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
                self.LibNr = num3;
                self.SFtypeNr = -1;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                self.MakeSFtypeTypeItemGUI();
              }
              else if (num3 == -2)
              {
                self.LibNr = -1;
                self.SFtypeNr = -1;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                self.MakeSFtypeTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.TabListId)
            {
              let mut num4: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num4 > -1)
              {
                self.TabSheetNr = num4;
                self.MakeSFtypeTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ExtraListId)
            {
              let mut num5: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num5 > -1)
              {
                self.detailnr = num5;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ResListId)
            {
              let mut num6: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num6 > -1)
              {
                self.detailnr2 = num6;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.LogoListId)
            {
              let mut num7: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num7 > -1)
              {
                self.detailnr2 = num7;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.PreventListId)
            {
              let mut num8: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num8 > -1)
              {
                self.detailnr2 = num8;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.VariantListId)
            {
              let mut num9: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num9 > -1)
              {
                self.detailnr2 = num9;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p1id)
            {
              SFTypeClass[] sfTypeObj = self.game.Data.SFTypeObj;
              SFTypeClass[] sfTypeClassArray = sfTypeObj;
              let mut sftypeNr: i32 = self.SFtypeNr;
              let mut index2: i32 = sftypeNr;
              sfTypeClassArray[index2].PreventCounter = sfTypeObj[sftypeNr].PreventCounter + 1;
              self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
              self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
              self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
              self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
              self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.vp1id)
            {
              SFTypeClass[] sfTypeObj = self.game.Data.SFTypeObj;
              SFTypeClass[] sfTypeClassArray = sfTypeObj;
              let mut sftypeNr: i32 = self.SFtypeNr;
              let mut index3: i32 = sftypeNr;
              sfTypeClassArray[index3].ModelVariantCounter = sfTypeObj[sftypeNr].ModelVariantCounter + 1;
              self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName = (string[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName, (Array) new string[self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter + 1]);
              self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter + 1]);
              self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter + 1]);
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p2id)
            {
              if (self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter > 0)
              {
                let mut detailnr2: i32 = self.detailnr2;
                let mut num10: i32 = self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter - 1;
                for (let mut index4: i32 = detailnr2; index4 <= num10; index4 += 1)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn[index4] = self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn[index4 + 1];
                  self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom[index4] = self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom[index4 + 1];
                  self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority[index4] = self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority[index4 + 1];
                  self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance[index4] = self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance[index4 + 1];
                  self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints[index4] = self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints[index4 + 1];
                }
              }
              --self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter;
              if (self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter > -1)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitOn, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventHitFrom, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].PreventCounter + 1]);
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.vp2id)
            {
              if (self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter > 0)
              {
                let mut detailnr2: i32 = self.detailnr2;
                let mut num11: i32 = self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter - 1;
                for (let mut index5: i32 = detailnr2; index5 <= num11; index5 += 1)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName[index5] = self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName[index5 + 1];
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck[index5] = self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck[index5 + 1];
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec[index5] = self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec[index5 + 1];
                }
              }
              --self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter;
              if (self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter > -1)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName = (string[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName, (Array) new string[self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter + 1]);
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCheck, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter + 1]);
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec = (int[]) Utils.CopyArray((Array) self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantExec, (Array) new int[self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantCounter + 1]);
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.y3id)
            {
              if (self.game.Data.SFTypeObj[self.SFtypeNr].UsePeopleGraphics == 0)
                self.game.Data.SFTypeObj[self.SFtypeNr].UsePeopleGraphics = 1;
              else if (self.game.Data.SFTypeObj[self.SFtypeNr].UsePeopleGraphics == 1)
                self.game.Data.SFTypeObj[self.SFtypeNr].UsePeopleGraphics = 2;
              else
                self.game.Data.SFTypeObj[self.SFtypeNr].UsePeopleGraphics = 0;
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.copyid)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 70, self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p3id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 68, self.SFtypeNr, self.detailnr2);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.vp3id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].ModelVariantName[self.detailnr2] = Interaction.InputBox("Give name", "Shadow Empire : Planetary Conquest");
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p4id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 69, self.SFtypeNr, self.detailnr2);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.vp4id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 77, self.SFtypeNr, self.detailnr2);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.vp5id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 78, self.SFtypeNr, self.detailnr2);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p5id)
            {
              let mut num12: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Prevent Priority.", "Shadow Empire : Planetary Conquest")));
              if (num12 >= -1 & num12 < 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventPriority[self.detailnr2] = num12;
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num13: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p6id)
            {
              let mut num14: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Prevent Chance in %.", "Shadow Empire : Planetary Conquest")));
              if (num14 >= 0 & num14 <= 100)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventChance[self.detailnr2] = num14;
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num15: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p7id)
            {
              let mut num16: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Prevent Points.", "Shadow Empire : Planetary Conquest")));
              if (num16 >= 0 & num16 <= 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].PreventPoints[self.detailnr2] = num16;
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num17: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p8id)
            {
              let mut num18: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Max Prevent Points Used. (0=cannot use any)", "Shadow Empire : Planetary Conquest")));
              if (num18 >= 0 & num18 <= 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].MaxPreventPointsUsed = num18;
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num19: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.p9id)
            {
              let mut num20: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Max Prevent Points Given. (0=cannot use any)", "Shadow Empire : Planetary Conquest")));
              if (num20 >= 0 & num20 <= 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].MaxPreventPointsGiven = num20;
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num21: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.x1id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 42, self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.x2id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 43, self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.x3id)
            {
              let mut num22: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Air Overrule AP Cost (-1=default).", "Shadow Empire : Planetary Conquest")));
              if (num22 >= -1 & num22 < 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].AirAPRule = num22;
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num23: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.x4id)
            {
              SFTypeClass sfTypeClass = self.game.Data.SFTypeObj[self.SFtypeNr].Clone();
              self.game.Data.SFTypeObj[self.SFtypeNr] = self.game.Data.SFTypeObj[self.SFtypeNr + 1].Clone();
              self.game.Data.SFTypeObj[self.SFtypeNr + 1] = sfTypeClass;
              self.game.Data.ChangeSFTypeNr(self.SFtypeNr, 9999);
              self.game.Data.ChangeSFTypeNr(self.SFtypeNr + 1, self.SFtypeNr);
              self.game.Data.ChangeSFTypeNr(9999, self.SFtypeNr + 1);
              self.game.Data.SFTypeObj[self.SFtypeNr].LoadSprites();
              self.game.Data.SFTypeObj[self.SFtypeNr + 1].LoadSprites();
              this += 1.SFtypeNr;
              self.SubPartFlag[self.SubpartNr(self.SFtypeListId)] = true;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              self.MakeSFtypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.x5id)
            {
              SFTypeClass sfTypeClass = self.game.Data.SFTypeObj[self.SFtypeNr].Clone();
              self.game.Data.SFTypeObj[self.SFtypeNr] = self.game.Data.SFTypeObj[self.SFtypeNr - 1].Clone();
              self.game.Data.SFTypeObj[self.SFtypeNr - 1] = sfTypeClass;
              self.game.Data.ChangeSFTypeNr(self.SFtypeNr, 9999);
              self.game.Data.ChangeSFTypeNr(self.SFtypeNr - 1, self.SFtypeNr);
              self.game.Data.ChangeSFTypeNr(9999, self.SFtypeNr - 1);
              self.game.Data.SFTypeObj[self.SFtypeNr].LoadSprites();
              self.game.Data.SFTypeObj[self.SFtypeNr - 1].LoadSprites();
              --self.SFtypeNr;
              self.SubPartFlag[self.SubpartNr(self.SFtypeListId)] = true;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              self.MakeSFtypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.clibid)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 93, self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.x6id)
            {
              let mut num24: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give SFType to replace with.", "Shadow Empire : Planetary Conquest")));
              if (num24 >= 0 & num24 <= self.game.Data.SFTypeCounter)
              {
                let mut sfCounter: i32 = self.game.Data.SFCounter;
                Number: i32;
                for (let mut index6: i32 = 0; index6 <= sfCounter; index6 += 1)
                {
                  if (self.game.Data.SFObj[index6].Type == self.SFtypeNr)
                  {
                    self.game.Data.SFObj[index6].Type = num24;
                    Number += 1;
                  }
                }
                let mut num25: i32 =  Interaction.MsgBox( ("Made " + Conversion.Str( Number) + " conversions throughout all the subformations in the units."));
              }
              else
              {
                let mut num26: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and SFTypeCounter", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.y5id)
            {
              s: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For Replacement of sideways sprite", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + s))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceSidewaysSprite(s);
              }
              else
              {
                let mut num27: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w1id)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For Replacement of extra Pic Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceExtraPic(self.detailnr, filename);
              }
              else
              {
                let mut num28: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w1bid)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For Replacement of extra Pic Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceExtraSideways(self.detailnr, filename);
              }
              else
              {
                let mut num29: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w10id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].DontReturnFromHQ = !self.game.Data.SFTypeObj[self.SFtypeNr].DontReturnFromHQ;
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w12id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].ConsiderCarry = !self.game.Data.SFTypeObj[self.SFtypeNr].ConsiderCarry;
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w8id)
            {
              let mut num30: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Set for unitgroup# ", "Shadow Empire : Planetary Conquest")));
              let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
              for (let mut index7: i32 = 0; index7 <= sfTypeCounter; index7 += 1)
              {
                if (self.game.Data.SFTypeObj[index7].UnitGroup == num30)
                {
                  let mut upperBound: i32 = self.game.Data.SFTypeObj[index7].CombatModAtt.GetUpperBound(0);
                  for (let mut index8: i32 = 0; index8 <= upperBound; index8 += 1)
                  {
                    self.game.Data.SFTypeObj[index7].CombatModAtt[index8] = self.game.Data.SFTypeObj[self.SFtypeNr].CombatModAtt[index8];
                    self.game.Data.SFTypeObj[index7].CombatModDef[index8] = self.game.Data.SFTypeObj[self.SFtypeNr].CombatModDef[index8];
                    self.game.Data.SFTypeObj[index7].ExtraRecon[index8] = self.game.Data.SFTypeObj[self.SFtypeNr].ExtraRecon[index8];
                  }
                }
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b39id)
            {
              let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
              for (let mut index9: i32 = 0; index9 <= sfTypeCounter; index9 += 1)
              {
                if (self.game.Data.SFTypeObj[index9].ReinforcementType == self.game.Data.SFTypeObj[self.SFtypeNr].ReinforcementType)
                {
                  let mut upperBound: i32 = self.game.Data.SFTypeObj[index9].CombatModAtt.GetUpperBound(0);
                  for (let mut index10: i32 = 0; index10 <= upperBound; index10 += 1)
                  {
                    self.game.Data.SFTypeObj[index9].CombatModAtt[index10] = self.game.Data.SFTypeObj[self.SFtypeNr].CombatModAtt[index10];
                    self.game.Data.SFTypeObj[index9].CombatModDef[index10] = self.game.Data.SFTypeObj[self.SFtypeNr].CombatModDef[index10];
                    self.game.Data.SFTypeObj[index9].ExtraRecon[index10] = self.game.Data.SFTypeObj[self.SFtypeNr].ExtraRecon[index10];
                  }
                }
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.t1id)
            {
              let mut index11: i32 =  Math.Round(Conversion.Int(Conversion.Val(Interaction.InputBox("Give SFType# to copy from", "Shadow Empire : Planetary Conquest"))));
              if (index11 > -1 & index11 <= self.game.Data.SFTypeCounter)
              {
                let mut sftypeNr: i32 = self.SFtypeNr;
                let mut upperBound: i32 = self.game.Data.SFTypeObj[sftypeNr].CombatModAtt.GetUpperBound(0);
                for (let mut index12: i32 = 0; index12 <= upperBound; index12 += 1)
                {
                  self.game.Data.SFTypeObj[sftypeNr].CombatModAtt[index12] = self.game.Data.SFTypeObj[index11].CombatModAtt[index12];
                  self.game.Data.SFTypeObj[sftypeNr].CombatModDef[index12] = self.game.Data.SFTypeObj[index11].CombatModDef[index12];
                  self.game.Data.SFTypeObj[sftypeNr].ExtraRecon[index12] = self.game.Data.SFTypeObj[index11].ExtraRecon[index12];
                }
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w2id)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of extra Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceExtraSymbol(self.detailnr, filename);
              }
              else
              {
                let mut num31: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.y8id)
            {
              filename1: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of extra Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename1))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceExtraColBigSymbol(self.detailnr, filename1);
              }
              else
              {
                let mut num32: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              filename2: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give SECOND File Name For Replacement of extra Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename2))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceExtraColBigSymbol2(self.detailnr, filename2);
              }
              else
              {
                let mut num33: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w2bid)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of extra Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceExtraSymbol2(self.detailnr, filename);
              }
              else
              {
                let mut num34: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.y2id)
            {
              let mut index13: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give SFType to Copy From .", "Shadow Empire : Planetary Conquest")));
              if (index13 >= 0 & index13 <= self.game.Data.SFTypeCounter)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].FuelForAttack = self.game.Data.SFTypeObj[index13].FuelForAttack;
                self.game.Data.SFTypeObj[self.SFtypeNr].FuelForAttackDef = self.game.Data.SFTypeObj[index13].FuelForAttackDef;
                self.game.Data.SFTypeObj[self.SFtypeNr].FuelForMove = self.game.Data.SFTypeObj[index13].FuelForMove;
                self.game.Data.SFTypeObj[self.SFtypeNr].FuelRegimeVar = self.game.Data.SFTypeObj[index13].FuelRegimeVar;
                self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelAttack = self.game.Data.SFTypeObj[index13].OutOfFuelAttack;
                self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelDefense = self.game.Data.SFTypeObj[index13].OutOfFuelDefense;
                self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelMove = self.game.Data.SFTypeObj[index13].OutOfFuelMove;
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num35: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.w3id)
            {
              let mut num36: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Code for selected extra graphics.", "Shadow Empire : Planetary Conquest")));
              if (num36 > 0 & num36 < 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ExtraCode[self.detailnr] = num36;
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num37: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.h6id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].ExtraName[self.detailnr] = Interaction.InputBox("Give Name.", "Shadow Empire : Planetary Conquest");
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w4id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].RemoveExtraSprite(self.detailnr);
              self.detailnr = -1;
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.w5id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].AddExtraSprite();
              self.detailnr = self.game.Data.SFTypeObj[self.SFtypeNr].ExtraCounter;
              self.Tabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b28id)
            {
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddSFtypeId)
            {
              self.game.Data.AddSFType();
              self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter].LibId.libSlot = self.LibNr;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddSFtype2Id)
            {
              let mut sftypeNr: i32 = self.SFtypeNr;
              self.game.Data.AddSFType();
              let mut id: i32 = self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter].Id;
              self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter] = self.game.Data.SFTypeObj[sftypeNr].Clone();
              self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter].Id = id;
              self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter].LoadSprites();
              self.game.Data.SFTypeObj[self.game.Data.SFTypeCounter].LibId.libSlot = self.LibNr;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BNameId)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.j1id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].LogoString[self.detailnr2] = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BRemoveSFtypeId)
            {
              self.game.Data.RemoveSFType(self.SFtypeNr);
              if (self.game.Data.SFTypeCounter < self.SFtypeNr)
                self.SFtypeNr = self.game.Data.SFTypeCounter;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BRemoveSFtypeId2)
            {
              for (let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter; sfTypeCounter >= 0; sfTypeCounter += -1)
                self.game.Data.RemoveSFType(sfTypeCounter);
              self.SFtypeNr = -1;
              self.MakeSFtypeListGUI(-1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BChangeSymbolId)
            {
              s: String = self.game.HandyFunctionsObj.LoadSomething("Pings (*.Png)|*.png|Bmp|*.bmp", "Give File Name For Replacement of Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + s))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceSymbolSprite(s);
              }
              else
              {
                let mut num38: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.y7id)
            {
              s: String = self.game.HandyFunctionsObj.LoadSomething("Pings (*.Png)|*.png|Bmp|*.bmp", "Give File Name For Replacement of Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + s))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceColBigSymbolSprite(s);
              }
              else
              {
                let mut num39: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BChangeSymbol2Id)
            {
              s: String = self.game.HandyFunctionsObj.LoadSomething("Pings (*.Png)|*.png|Bmp|*.bmp", "Give File Name For Replacement of Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + s))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplaceSymbolSprite2(s);
              }
              else
              {
                let mut num40: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.bChangePicId)
            {
              s: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Give File Name For Replacement of Picture Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + s))
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReplacePicSprite(s);
              }
              else
              {
                let mut num41: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BSymbolGroupId)
            {
              let mut num42: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new Symbol Group Number, please.", "Shadow Empire : Planetary Conquest")));
              if (num42 > -2 & num42 < 100)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].SymbolGroup = num42;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num43: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and 99", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.BSymbolOverRuleId)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].SymbolOverrule = !self.game.Data.SFTypeObj[self.SFtypeNr].SymbolOverrule;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b35id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].FreeAir = !self.game.Data.SFTypeObj[self.SFtypeNr].FreeAir;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BMoveTypeId)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 4, self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BSymbolWeightId)
            {
              let mut num44: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new Symbol Weight, please.", "Shadow Empire : Planetary Conquest")));
              if (num44 > -1 & num44 < 10000)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].SymbolWeight = num44;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num45: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.B1Id)
            {
              let mut num46: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Supply Carry, please.", "Shadow Empire : Planetary Conquest")));
              if (num46 > -2 & num46 < 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].SupplyCarry = num46;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num47: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b33id)
            {
              let mut num48: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give RailCap pts", "Shadow Empire : Planetary Conquest")));
              if (num48 > -1 & num48 <= 10000)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].RailCap = num48;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num49: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 10000", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b31id)
            {
              let mut num50: i32 =  Interaction.MsgBox( "For all, or only selected peoplegroup", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest");
              let mut num51: i32 =  Interaction.MsgBox( "Set true? yes=true.. no=false", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest");
              let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
              for (let mut index14: i32 = 0; index14 <= sfTypeCounter; index14 += 1)
              {
                let mut index15: i32 = 0;
                do
                {
                  if (num50 == 6)
                  {
                    if (self.detailnr > -1)
                    {
                      if (num51 == 6)
                        self.game.Data.SFTypeObj[index14].PeopleGroup[self.detailnr] = true;
                      else
                        self.game.Data.SFTypeObj[index14].PeopleGroup[self.detailnr] = false;
                    }
                  }
                  else if (num51 == 6)
                    self.game.Data.SFTypeObj[index14].PeopleGroup[index15] = true;
                  else
                    self.game.Data.SFTypeObj[index14].PeopleGroup[index15] = false;
                  index15 += 1;
                }
                while (index15 <= 99);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.h3id)
            {
              let mut num52: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New RoleScore, please.", "Shadow Empire : Planetary Conquest")));
              if (num52 > -1 & num52 < 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].AIRoleScore[self.detailnr] = num52;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num53: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.w11id)
            {
              let mut num54: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New RoleScore (will set for all with same unitgroup), please.", "Shadow Empire : Planetary Conquest")));
              if (num54 > -1 & num54 < 9999)
              {
                let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                for (let mut index16: i32 = 0; index16 <= sfTypeCounter; index16 += 1)
                {
                  if (self.game.Data.SFTypeObj[index16].UnitGroup == self.game.Data.SFTypeObj[self.SFtypeNr].UnitGroup)
                    self.game.Data.SFTypeObj[index16].AIRoleScore[self.detailnr] = num54;
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num55: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.B5Id)
            {
              let mut num56: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New LandCap, please.", "Shadow Empire : Planetary Conquest")));
              if (num56 > -1 & num56 < 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].Cap = num56;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num57: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.y6id)
            {
              let mut num58: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Morph: Color value.", "Shadow Empire : Planetary Conquest")));
              if (num58 > -1 & num58 < 7)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].BaseColor = num58;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num59: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 4", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.B6Id)
            {
              let mut num60: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Basic Supply Need, please.", "Shadow Empire : Planetary Conquest")));
              if (num60 > -1 & num60 < 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].BasicSupplyNeed = num60;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num61: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b32id)
            {
              let mut num62: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New absolute readiness loss per 100ap please.", "Shadow Empire : Planetary Conquest")));
              if (num62 > -1 & num62 <= 100)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ReadinessLoss = num62;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num63: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.B7Id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 5, self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B8Id)
            {
              let mut num64: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Theater #, please.", "Shadow Empire : Planetary Conquest")));
              if (num64 > -1 & num64 < 3)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].Theater = num64;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num65: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 2", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b9id)
            {
              let mut num66: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Weight, please.", "Shadow Empire : Planetary Conquest")));
              if (num66 > 0 & num66 < 999999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].Weight = num66;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num67: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 999999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b10id)
            {
              let mut num68: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New DefPower for ATTACK HEX, please.", "Shadow Empire : Planetary Conquest")));
              if (num68 > 0 & num68 < 999999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].DefPower = num68;
              }
              else
              {
                let mut num69: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 999999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b11id)
            {
              let mut num70: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Initiative for ATTACK HEX, please.", "Shadow Empire : Planetary Conquest")));
              if (num70 > 0 & num70 < 999999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].Initiative = num70;
              }
              else
              {
                let mut num71: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 999999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              let mut num72: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Initiative for DEFEND HEX, please.", "Shadow Empire : Planetary Conquest")));
              if (num72 > 0 & num72 < 999999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].InitiativeDef = num72;
              }
              else
              {
                let mut num73: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 999999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b12id)
            {
              let mut num74: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Attacks, please.", "Shadow Empire : Planetary Conquest")));
              if (num74 > -1 & num74 <= 9999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].Attacks = num74;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num75: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 99", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b13id)
            {
              let mut num76: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New MaxAttacked, please.", "Shadow Empire : Planetary Conquest")));
              if (num76 > 0 & num76 < 999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].MaxAttacked = num76;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num77: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b14id)
            {
              let mut num78: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Frontage, please.", "Shadow Empire : Planetary Conquest")));
              if (num78 > -1 & num78 < 999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].Frontage = num78;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num79: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b15id)
            {
              self.game.Data.SFTypeObj[self.SFtypeNr].BackBench = !self.game.Data.SFTypeObj[self.SFtypeNr].BackBench;
              self.MakeSFtypeListGUI(self.SFtypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b16id)
            {
              let mut num80: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New ArtRange, please.", "Shadow Empire : Planetary Conquest")));
              if (num80 > -1 & num80 < 99)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ArtRange = num80;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num81: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.w13id)
            {
              let mut num82: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New SFType # used to look up landscape mods for artillery attack.", "Shadow Empire : Planetary Conquest")));
              if (num82 >= -1 & num82 < self.game.Data.SFTypeCounter)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ArtSFType = num82;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num83: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and maxLT", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b17id)
            {
              let mut num84: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New FavTarget Tries, please.", "Shadow Empire : Planetary Conquest")));
              if (num84 > 0 & num84 < 999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].FavTargetTries = num84;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num85: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 1 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b18id)
            {
              let mut num86: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Fav Target score, please.", "Shadow Empire : Planetary Conquest")));
              if (num86 > -1 & num86 < 999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].FavTarget[self.detailnr] = num86;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num87: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b19id)
            {
              let mut num88: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Attack Power score, please.", "Shadow Empire : Planetary Conquest")));
              if (num88 > -1 & num88 < 9990)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].AttackPower[self.detailnr] = num88;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num89: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9990", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b23id)
            {
              let mut num90: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Attack Def Power score, please.", "Shadow Empire : Planetary Conquest")));
              if (num90 > -1 & num90 < 9990)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].AttackPowerDef[self.detailnr] = num90;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num91: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9990", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b25id)
            {
              let mut num92: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Art Attack score, please.", "Shadow Empire : Planetary Conquest")));
              if (num92 > -1 & num92 < 9990)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].AttackArt[self.detailnr] =  num92;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num93: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9990", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b26id)
            {
              let mut num94: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Artillery Fav score, please.", "Shadow Empire : Planetary Conquest")));
              if (num94 > -1 & num94 < 9990)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].FavArtTarget[self.detailnr] = num94;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num95: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9990", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b37id)
            {
              let mut num96: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give hitpoints", "Shadow Empire : Planetary Conquest")));
              if (num96 > -1 & num96 < 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].HitPoints[self.detailnr] = num96;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num97: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b38id)
            {
              let mut num98: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give def hitpoints", "Shadow Empire : Planetary Conquest")));
              if (num98 > -1 & num98 < 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].HitPointsDef[self.detailnr] = num98;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num99: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.w15id)
            {
              let mut num100: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("DirectRange", "Shadow Empire : Planetary Conquest")));
              if (num100 > -1 & num100 < 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].directRange = num100;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num101: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.w16id)
            {
              let mut num102: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Direct Mod 1st Hex", "Shadow Empire : Planetary Conquest")));
              if (num102 > -1 & num102 < 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].directModFirstHex = num102;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num103: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.w17id)
            {
              let mut num104: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Direct Mod 2nd+ Hex", "Shadow Empire : Planetary Conquest")));
              if (num104 > -1 & num104 < 99999)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].directModPerHex = num104;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num105: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.y1id)
            {
              let mut num106: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give def hitpoints", "Shadow Empire : Planetary Conquest")));
              if (num106 > -1 & num106 < 99999)
              {
                let mut index17: i32 = 0;
                do
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].HitPointsDef[index17] = num106;
                  self.game.Data.SFTypeObj[self.SFtypeNr].HitPoints[index17] = num106;
                  index17 += 1;
                }
                while (index17 <= 99);
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num107: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b20id)
            {
              let mut num108: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Kill % chance please.", "Shadow Empire : Planetary Conquest")));
              if (num108 > -1 & num108 < 101)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].KillPercent = num108;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num109: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100%", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b21id)
            {
              let mut num110: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give  Equipment Kill % chance, please.", "Shadow Empire : Planetary Conquest")));
              if (num110 > -1 & num110 < 101)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].EquipPercent = num110;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num111: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100%", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b22id)
            {
              let mut num112: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Retreat % chance, please.", "Shadow Empire : Planetary Conquest")));
              if (num112 > -1 & num112 < 101)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].RetreatPercent = num112;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num113: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100%", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b34id)
            {
              let mut num114: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give slot# (-1 = dont set any slot).", "Shadow Empire : Planetary Conquest")));
              if (num114 >= -1 & num114 <= 9)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].SlotNumber = num114;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num115: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and 9", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.b24id)
            {
              let mut num116: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Move Redux in %, please.", "Shadow Empire : Planetary Conquest")));
              if (num116 > -101 & num116 < 101)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].MoveRedux = num116;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num117: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100%", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              return windowReturnClass;
            }
            if (num1 == self.PGListId)
            {
              let mut num118: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num118 > -1)
              {
                self.detailnr = num118;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.CombatListId)
            {
              let mut num119: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num119 > -1)
              {
                self.detailnr = num119;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.CombatList2Id)
            {
              let mut num120: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num120 > -1)
              {
                self.detailnr = num120;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.combatlist3id)
            {
              let mut num121: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num121 > -1)
              {
                self.detailnr = num121;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.combatlist4id)
            {
              let mut num122: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num122 > -1)
              {
                self.detailnr = num122;
                self.Tabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B4Id)
            {
              if (self.detailnr > -1)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].PeopleGroup[self.detailnr] = !self.game.Data.SFTypeObj[self.SFtypeNr].PeopleGroup[self.detailnr];
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.d1id)
              {
                let mut num123: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New CarryCap, please.", "Shadow Empire : Planetary Conquest")));
                if (num123 > -1 & num123 < 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].CarryCap = num123;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num124: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.e1id)
              {
                let mut num125: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New EntrenchPower, please.", "Shadow Empire : Planetary Conquest")));
                if (num125 > -1 & num125 < 901)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].EntrenchPower = num125;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num126: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 900", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.e5id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 26, self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w9id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 51, self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.w9bid)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 89, self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.e6id)
              {
                let mut num127: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Upgrade cost in ProdPts, please. (rulevar77 how much prodpts = 1 supply)", "Shadow Empire : Planetary Conquest")));
                if (num127 > -1 & num127 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].UpgradeCost = num127;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num128: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 - 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.e7id)
              {
                let mut num129: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Upgrade XP needed, please.", "Shadow Empire : Planetary Conquest")));
                if (num129 > -1 & num129 <= 999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].UpgradeXP = num129;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num130: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 - 999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.e3id)
              {
                let mut num131: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New PowerPts, please.", "Shadow Empire : Planetary Conquest")));
                if (num131 > -1 & num131 < 999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].PowerPts = num131;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num132: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.w6id)
              {
                let mut num133: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New ratio, please.", "Shadow Empire : Planetary Conquest")));
                if (num133 > -1 & num133 < 999999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].Ratio = num133;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num134: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.w7id)
              {
                let mut num135: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new copyFrom sftype slot.", "Shadow Empire : Planetary Conquest")));
                if (num135 >= -1 & num135 <= self.game.Data.SFTypeCounter)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].CopyDataFrom = num135;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num136: i32 =  Interaction.MsgBox( "Cancelled. Value must be valid sftype slot", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v19id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelInitialForAll = !self.game.Data.SFTypeObj[self.SFtypeNr].ModelInitialForAll;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.f1id)
              {
                let mut num137: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Recon Pts, please.", "Shadow Empire : Planetary Conquest")));
                if (num137 > -1 & num137 < 999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ReconPts = num137;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num138: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.f2id)
              {
                let mut num139: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Hide Pts, please.", "Shadow Empire : Planetary Conquest")));
                if (num139 > -1 & num139 < 999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].HidePts = num139;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num140: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v1id)
              {
                let mut num141: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Last State. 1=got it. 0=not", "Shadow Empire : Planetary Conquest")));
                if (num141 >= 0 & num141 <= 1)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelLastState[self.detailnr2] = num141;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num142: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 1", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v2id)
              {
                let mut num143: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Possible Improvement. 1=gives impr. 0=not", "Shadow Empire : Planetary Conquest")));
                if (num143 >= 0 & num143 <= 1)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelPossibleImp[self.detailnr2] = num143;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num144: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 1", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c2id)
              {
                let mut num145: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give regimevar used for fuel (-1=none)", "Shadow Empire : Planetary Conquest")));
                if (num145 >= -1 & num145 <= 400)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].FuelRegimeVar = num145;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num146: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 1", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c3id)
              {
                let mut num147: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give fuel needed for move (10 ap or minimum cost)", "Shadow Empire : Planetary Conquest")));
                if (num147 >= 0 & num147 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].FuelForMove = num147;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num148: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c12id)
              {
                let mut num149: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give stockpile used per round", "Shadow Empire : Planetary Conquest")));
                if (num149 >= 0 & num149 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StockpileUsedPerRound = num149;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num150: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c13id)
              {
                let mut num151: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give stockpile max size", "Shadow Empire : Planetary Conquest")));
                if (num151 >= 0 & num151 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StockpileMax = num151;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num152: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c14id)
              {
                let mut num153: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give stockpile max in", "Shadow Empire : Planetary Conquest")));
                if (num153 >= 0 & num153 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StockpileMaxIn = num153;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num154: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c15id)
              {
                float num155 =  Conversion.Val(Interaction.InputBox("Give attack value modifier if out of stockpile", "Shadow Empire : Planetary Conquest"));
                if ( num155 >= 0.0 &  num155 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StockpileDepletedMod = num155;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num156: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0.0 and 99999.0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c16id)
              {
                let mut num157: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give max (regular) supply in", "Shadow Empire : Planetary Conquest")));
                if (num157 >= 0 & num157 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].SupplyMaxIn = num157;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num158: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c17id)
              {
                let mut num159: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Supply for 100AP of offensive combat", "Shadow Empire : Planetary Conquest")));
                if (num159 >= 0 & num159 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].SupplyForAttack = num159;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num160: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c18id)
              {
                let mut num161: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Supply for 100AP of defensive combat", "Shadow Empire : Planetary Conquest")));
                if (num161 >= 0 & num161 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].SupplyForAttackDef = num161;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num162: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c19id)
              {
                float num163 =  Conversion.Val(Interaction.InputBox("Give attack value modifier if out of supply", "Shadow Empire : Planetary Conquest"));
                if ( num163 >= 0.0 &  num163 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].OutOfSupplyAttack = num163;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num164: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0.0 and 99999.0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c20id)
              {
                float num165 =  Conversion.Val(Interaction.InputBox("Give attack value modifier if out of supply", "Shadow Empire : Planetary Conquest"));
                if ( num165 >= 0.0 &  num165 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].OutOfSupplyDefense = num165;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num166: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0.0 and 99999.0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c21id)
              {
                let mut num167: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Fuel Carry Pts", "Shadow Empire : Planetary Conquest")));
                if (num167 >= 0 & num167 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].FuelCarry = num167;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num168: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c5id)
              {
                let mut num169: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give fuel needed for 1 combatround in OFFENSE", "Shadow Empire : Planetary Conquest")));
                let mut num170: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give fuel needed for 1 combatround in DEFENSE", "Shadow Empire : Planetary Conquest")));
                if (num169 >= 0 & num169 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].FuelForAttack = num169;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                if (num170 >= 0 & num170 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].FuelForAttackDef = num170;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                return windowReturnClass;
              }
              if (num1 == self.v3id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 63, self.SFtypeNr, self.detailnr2);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v4id)
              {
                let mut index18: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("For which level is this research necc. 0-9 (0=for none)", "Shadow Empire : Planetary Conquest")));
                if (index18 >= 0 & index18 <= 9)
                {
                  let mut index19: i32 = 1;
                  do
                  {
                    if (self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[index19] == self.detailnr2)
                      self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[index19] = -1;
                    index19 += 1;
                  }
                  while (index19 <= 9);
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[index18] = self.detailnr2;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num171: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v5id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelIsBase = !self.game.Data.SFTypeObj[self.SFtypeNr].ModelIsBase;
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v6id)
              {
                let mut num172: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New ModelCostType. -1=PP. 0-499=regvar ", "Shadow Empire : Planetary Conquest")));
                if (num172 >= -1 & num172 <= 499)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostType = num172;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num173: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and 499", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v17id)
              {
                let mut num174: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Model Regime. -1=all ", "Shadow Empire : Planetary Conquest")));
                if (num174 >= -2 & num174 <= self.game.Data.RegimeCounter)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelRegime = num174;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num175: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and regimecounter", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v18id)
              {
                let mut num176: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Research(0). (-1 no need) ", "Shadow Empire : Planetary Conquest")));
                if (num176 >= -1 & num176 <= self.game.Data.ResearchCounter)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[0] = num176;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num177: i32 =  Interaction.MsgBox( "Cancelled. Value must be between -1 and researchcounter", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v7id)
              {
                let mut num178: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New Cost 0-99999 ", "Shadow Empire : Planetary Conquest")));
                if (num178 >= 0 & num178 <= 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCost = num178;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num179: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c4id)
              {
                float num180 =  Conversion.Val(Interaction.InputBox("Give Out of Fuel Move Modifier. 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num180 >= 0.0 &  num180 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelMove = num180;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num181: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c6id)
              {
                float num182 =  Conversion.Val(Interaction.InputBox("Give Out of Fuel Attack Modifier 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num182 >= 0.0 &  num182 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelAttack = num182;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num183: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.w133id)
              {
                float num184 =  Conversion.Val(Interaction.InputBox("Give new 0.0-1.0 score", "Shadow Empire : Planetary Conquest"));
                if ( num184 >= 0.0 &  num184 <= 1.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ChanceOnDeathIfMakeHit = num184;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num185: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0.0 and 1.0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.c7id)
              {
                float num186 =  Conversion.Val(Interaction.InputBox("Give Out of Fuel Defend Modifier 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num186 >= 0.0 &  num186 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].OutOfFuelDefense = num186;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num187: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v8id)
              {
                float num188 =  Conversion.Val(Interaction.InputBox("Give New Cost modifier per level. 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num188 >= 0.0 &  num188 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerLevel = num188;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num189: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v9id)
              {
                float num190 =  Conversion.Val(Interaction.InputBox("Give New Cost modifier per same model. 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num190 >= 0.0 &  num190 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerSameModel = num190;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num191: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v10id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 64, self.SFtypeNr, self.detailnr2);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v11id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 65, self.SFtypeNr, self.detailnr2);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v20id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 66, self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v15id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 67, self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v22id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 71, self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v12id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowUpgrade = !self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowUpgrade;
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v13id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowImprovements = !self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowImprovements;
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.v21id)
              {
                let mut index20: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give SFType # to copy from. (-1=cancel)", "Shadow Empire : Planetary Conquest")));
                if (index20 > -1 & index20 <= self.game.Data.SFTypeCounter)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelImproveCostMod = self.game.Data.SFTypeObj[index20].ModelImproveCostMod;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelInitialEvent = self.game.Data.SFTypeObj[index20].ModelInitialEvent;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelInitialForAll = self.game.Data.SFTypeObj[index20].ModelInitialForAll;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelIsBase = self.game.Data.SFTypeObj[index20].ModelIsBase;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelNewEvent = self.game.Data.SFTypeObj[index20].ModelNewEvent;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowUpgrade = self.game.Data.SFTypeObj[index20].ModelAllowUpgrade;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelAllowImprovements = self.game.Data.SFTypeObj[index20].ModelAllowImprovements;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerLevel = self.game.Data.SFTypeObj[index20].ModelCostPerLevel;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerSameImp = self.game.Data.SFTypeObj[index20].ModelCostPerSameImp;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCostPerSameModel = self.game.Data.SFTypeObj[index20].ModelCostPerSameModel;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelCost = self.game.Data.SFTypeObj[index20].ModelCost;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelExtraResearch = self.game.Data.SFTypeObj[index20].ModelExtraResearch;
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelSFTypeUpgrade = self.game.Data.SFTypeObj[index20].ModelSFTypeUpgrade;
                  let mut researchCounter: i32 = self.game.Data.ResearchCounter;
                  for (let mut index21: i32 = 0; index21 <= researchCounter; index21 += 1)
                  {
                    self.game.Data.SFTypeObj[self.SFtypeNr].ModelAutoImprovement[index21] = self.game.Data.SFTypeObj[index20].ModelAutoImprovement[index21];
                    self.game.Data.SFTypeObj[self.SFtypeNr].ModelLastState[index21] = self.game.Data.SFTypeObj[index20].ModelLastState[index21];
                    self.game.Data.SFTypeObj[self.SFtypeNr].ModelPossibleImp[index21] = self.game.Data.SFTypeObj[index20].ModelPossibleImp[index21];
                    self.game.Data.SFTypeObj[self.SFtypeNr].ModelImproveEvent[index21] = self.game.Data.SFTypeObj[index20].ModelImproveEvent[index21];
                  }
                  let mut index22: i32 = 0;
                  do
                  {
                    self.game.Data.SFTypeObj[self.SFtypeNr].ModelResearch[index22] = self.game.Data.SFTypeObj[index20].ModelResearch[index22];
                    index22 += 1;
                  }
                  while (index22 <= 9);
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num192: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v14id)
              {
                float num193 =  Conversion.Val(Interaction.InputBox("Give New Cost modifier for improvement. 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num193 >= 0.0 &  num193 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelImproveCostMod = num193;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num194: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v23id)
              {
                float num195 =  Conversion.Val(Interaction.InputBox("Give New modifier 0.x-x.x ", "Shadow Empire : Planetary Conquest"));
                if ( num195 >= 0.0 &  num195 <= 99999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ModelSFTypeUpgrade = num195;
                  self.Tabsheet();
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num196: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.v16id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].ModelAutoImprovement[self.detailnr2] = !self.game.Data.SFTypeObj[self.SFtypeNr].ModelAutoImprovement[self.detailnr2];
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.f3id)
              {
                let mut num197: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New ZOC Pts, please.", "Shadow Empire : Planetary Conquest")));
                if (num197 > -1 & num197 < 999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ZOCPts = num197;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num198: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g1id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].CanDoParadrop = !self.game.Data.SFTypeObj[self.SFtypeNr].CanDoParadrop;
                self.Tabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g2id)
              {
                let mut num199: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New AntiStruc Pts, please.", "Shadow Empire : Planetary Conquest")));
                if (num199 > -1 & num199 < 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].AntiStrucPts = num199;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num200: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.w14id)
              {
                let mut num201: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new Start Round (0=normal), please.", "Shadow Empire : Planetary Conquest")));
                if (num201 > -1 & num201 < 9)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StartCombatRound = num201;
                }
                else
                {
                  let mut num202: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                let mut num203: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new End Round (0=normal), please.", "Shadow Empire : Planetary Conquest")));
                if (num203 > -1 & num203 < 9)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].EndCombatRound = num203;
                }
                else
                {
                  let mut num204: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g3id)
              {
                let mut num205: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New AirCarrierCap, please.", "Shadow Empire : Planetary Conquest")));
                if (num205 > -1 & num205 < 99999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].AirCarrierCap = num205;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num206: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 99999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g24id)
              {
                self.game.Data.SFTypeObj[self.SFtypeNr].DontShowInList = !self.game.Data.SFTypeObj[self.SFtypeNr].DontShowInList;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              num207: i32;
              if (num1 == self.b36id)
              {
                float num208 =  Conversion.Val(Interaction.InputBox("Give First Rounds Penalty Mod, please (0.0(gone)-1.0(normal)).", "Shadow Empire : Planetary Conquest"));
                if ( num208 >= 0.0 & num207 <= 1)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].FirstRoundPenaltyMod = num208;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num209: i32 =  Interaction.MsgBox( "Cancelled. Value must be equal/between 0.0 and 1.0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g4id)
              {
                float num210 =  Conversion.Val(Interaction.InputBox("Give New Ap Mod, please.", "Shadow Empire : Planetary Conquest"));
                if ( num210 > 0.0 & num207 < 10)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ApMod = num210;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num211: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0.0 and 10.0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g6id)
              {
                let mut num212: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New RdnLossPerAttack, please.", "Shadow Empire : Planetary Conquest")));
                if (num212 >= 0 & num212 <= 100)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].RdnLossPerAttack = num212;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num213: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g7id)
              {
                if (MsgBoxResult.Yes == Interaction.MsgBox( "AutoDestroy in Attack?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest"))
                  self.game.Data.SFTypeObj[self.SFtypeNr].AutoDestroy = true;
                else
                  self.game.Data.SFTypeObj[self.SFtypeNr].AutoDestroy = false;
                if (MsgBoxResult.Yes == Interaction.MsgBox( "AutoDestroy in Defense?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest"))
                  self.game.Data.SFTypeObj[self.SFtypeNr].AutoDestroy2 = true;
                else
                  self.game.Data.SFTypeObj[self.SFtypeNr].AutoDestroy2 = false;
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g8id)
              {
                let mut num214: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give EP pts, please.", "Shadow Empire : Planetary Conquest")));
                if (num214 >= 0 & num214 <= 9999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].EP = num214;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num215: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 9999", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g9id)
              {
                let mut num216: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give ACap Service percentage, please.", "Shadow Empire : Planetary Conquest")));
                if (num216 >= 0 & num216 <= 100)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].EP = num216;
                  self.MakeSFtypeListGUI(self.SFtypeNr);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num217: i32 =  Interaction.MsgBox( "Cancelled. Value must be between 0 and 100", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass;
              }
              if (num1 == self.g10id)
              {
                Left: String = "";
                if (!Information.IsNothing( DrawMod.TGame) && !Information.IsNothing( DrawMod.TGame.Data.SoundDir))
                  Left = DrawMod.TGame.Data.SoundDir;
                if (Operators.CompareString(Left, "", false) == 0)
                  Left = DrawMod.TGame.ModSoundDir;
                str: String = self.game.HandyFunctionsObj.LoadSomething("WAVs (*.wav)|*.wav", "Select File For Move Sound", self.game.AppPath + Left + "\\", true);
                if (File.Exists(self.game.AppPath + Left + "\\" + str))
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].MoveWAV = str;
                  SoundMod.PlayAWave(self.game.AppPath + Left + "\\" + self.game.Data.SFTypeObj[self.SFtypeNr].MoveWAV,  self.game.EditObj);
                }
                else
                {
                  let mut num218: i32 =  Interaction.MsgBox( "File does not exist. wav set to no sound.", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.game.Data.SFTypeObj[self.SFtypeNr].MoveWAV = "";
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g11id)
              {
                Left: String = "";
                if (!Information.IsNothing( DrawMod.TGame) && !Information.IsNothing( DrawMod.TGame.Data.SoundDir))
                  Left = DrawMod.TGame.Data.SoundDir;
                if (Operators.CompareString(Left, "", false) == 0)
                  Left = DrawMod.TGame.ModSoundDir;
                str: String = self.game.HandyFunctionsObj.LoadSomething("WAVs (*.wav)|*.wav", "Select File For Battle Sound", self.game.AppPath + Left + "\\", true);
                if (File.Exists(self.game.AppPath + Left + "\\" + str))
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].BattleWAV = str;
                  SoundMod.PlayAWave(self.game.AppPath + Left + "\\" + self.game.Data.SFTypeObj[self.SFtypeNr].BattleWAV,  self.game.EditObj);
                }
                else
                {
                  let mut num219: i32 =  Interaction.MsgBox( "File does not exist. wav set to no sound.", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.game.Data.SFTypeObj[self.SFtypeNr].BattleWAV = "";
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g13id)
              {
                float num220 =  Conversion.Val(Interaction.InputBox("Give att mod.", "Shadow Empire : Planetary Conquest"));
                if ( num220 >= 0.0 &  num220 <= 999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].CombatModAtt[self.detailnr] = num220;
                }
                else
                {
                  let mut num221: i32 =  Interaction.MsgBox( "Value between 0 - 999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g14id)
              {
                float num222 =  Conversion.Val(Interaction.InputBox("Give def mod.", "Shadow Empire : Planetary Conquest"));
                if ( num222 >= 0.0 &  num222 <= 999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].CombatModDef[self.detailnr] = num222;
                }
                else
                {
                  let mut num223: i32 =  Interaction.MsgBox( "Value between 0 - 999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g23id)
              {
                let mut num224: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give extra recon", "Shadow Empire : Planetary Conquest")));
                if (num224 >= 0 & num224 <= 9999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].ExtraRecon[self.detailnr] = num224;
                }
                else
                {
                  let mut num225: i32 =  Interaction.MsgBox( "Value between 0 - 9999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b29id)
              {
                float num226 =  Conversion.Val(Interaction.InputBox("Give staffcombatmod... 0.0=none, 1.0=100%", "Shadow Empire : Planetary Conquest"));
                if ( num226 >= 0.0 &  num226 <= 999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StaffCombatMod = num226;
                }
                else
                {
                  let mut num227: i32 =  Interaction.MsgBox( "Value between 0 - 999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b30id)
              {
                float num228 =  Conversion.Val(Interaction.InputBox("Give staffmoralemod... 0.0=none, 1.0=100%", "Shadow Empire : Planetary Conquest"));
                if ( num228 >= 0.0 &  num228 <= 999.0)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StaffMoraleMod = num228;
                }
                else
                {
                  let mut num229: i32 =  Interaction.MsgBox( "Value between 0 - 999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g15id)
              {
                let mut num230: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give # of staff pts.", "Shadow Empire : Planetary Conquest")));
                if (num230 > -1 & num230 <= 9999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].StaffPts = num230;
                }
                else
                {
                  let mut num231: i32 =  Interaction.MsgBox( "btween 0-9999 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g16id)
              {
                let mut num232: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give AA Range. 1=distance 1, 2=distance 2", "Shadow Empire : Planetary Conquest")));
                if (num232 > -1 & num232 <= 99)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].AARange = num232;
                }
                else
                {
                  let mut num233: i32 =  Interaction.MsgBox( "btween 0-99 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g17id)
              {
                let mut num234: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give # of blowbridge pts.", "Shadow Empire : Planetary Conquest")));
                if (num234 > -1 & num234 <= 9999)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].BlowBridgePts = num234;
                }
                else
                {
                  let mut num235: i32 =  Interaction.MsgBox( "btween 0-9999 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g18id)
              {
                let mut num236: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Kill to Retreat chance (0-100).", "Shadow Empire : Planetary Conquest")));
                if (num236 > -1 & num236 <= 100)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].KilltoRetreatChance = num236;
                }
                else
                {
                  let mut num237: i32 =  Interaction.MsgBox( "btween 0-100 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g19id)
              {
                let mut num238: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Anti Supply Pts versus LAnd.", "Shadow Empire : Planetary Conquest")));
                if (num238 > -1 & num238 <= 10009)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].AntiSupply = num238;
                }
                else
                {
                  let mut num239: i32 =  Interaction.MsgBox( "btween 0-10009 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g21id)
              {
                let mut num240: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Anti Supply Pts versus Sea.", "Shadow Empire : Planetary Conquest")));
                if (num240 > -1 & num240 <= 10009)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].AntiSupplySea = num240;
                }
                else
                {
                  let mut num241: i32 =  Interaction.MsgBox( "btween 0-19000 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.g20id)
              {
                let mut num242: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Anti Supply Range in AP", "Shadow Empire : Planetary Conquest")));
                if (num242 > -1 & num242 <= 1090)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].AntiSupplyRange = num242;
                }
                else
                {
                  let mut num243: i32 =  Interaction.MsgBox( "btween 0-1000 pls", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.h5id)
              {
                let mut num244: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Kill is RegVar #", "Shadow Empire : Planetary Conquest")));
                if (num244 >= -1 & num244 <= 499)
                {
                  self.game.Data.SFTypeObj[self.SFtypeNr].KillIsRegVar = num244;
                }
                else
                {
                  let mut num245: i32 =  Interaction.MsgBox( "btween 0-499 pls.. or -1 for none", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeSFtypeListGUI(self.SFtypeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b27id)
              {
                Form2::new( self.formref).Initialize(self.game.Data, 1, self.SFtypeNr);
                self.MakeSFtypeListGUI(self.SFtypeNr);
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
