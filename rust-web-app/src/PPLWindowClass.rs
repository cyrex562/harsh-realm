// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PPLWindowClass
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
  pub class PPLWindowClass : WindowClass
  {
     TempText1: i32;
     temptext2: i32;
     temptext3: i32;
     temptext4: i32;
     temptext5: i32;
     temptext6: i32;
     temptext7: i32;
     temptext8: i32;
     temptext9: i32;
     temptext10: i32;
     TempText11: i32;
     temptext12: i32;
     temptext13: i32;
     temptext14: i32;
     temptext15: i32;
     temptext16: i32;
     temptext17: i32;
     temptext18: i32;
     temptext19: i32;
     temptext20: i32;
     TempText21: i32;
     temptext22: i32;
     temptext23: i32;
     temptext24: i32;
     temptext25: i32;
     temptext26: i32;
     temptext27: i32;
     temptext28: i32;
     temptext29: i32;
     temptext30: i32;
     TempText31: i32;
     temptext32: i32;
     temptext33: i32;
     temptext34: i32;
     temptext35: i32;
     temptext36: i32;
     temptext37: i32;
     temptext38: i32;
     temptext39: i32;
     temptext40: i32;
     temptext41: i32;
     temptext42: i32;
     temptext43: i32;
     temptext44: i32;
     temptext45: i32;
     temptext46: i32;
     LogoListId: i32;
     but1id: i32;
     tab1id: i32;
     tab2id: i32;
     tab3id: i32;
     tab4id: i32;
     tab5id: i32;
     but1textid: i32;
     but1bid: i32;
     hqbut0: i32;
     hqbut1: i32;
     hqbut2: i32;
     but2id: i32;
     but2textid: i32;
     but3id: i32;
     but3textid: i32;
     but4id: i32;
     but4textid: i32;
     but5id: i32;
     but5textid: i32;
     but6id: i32;
     but6textid: i32;
     but7id: i32;
     quitid: i32;
     but7textid: i32;
     descid: i32;
     comparenr: i32;
     sliderid: i32;
     logolist2id: i32;
     logolist3id: i32;
     float tempBlink;
     unr: i32;
     sfnr: i32;
     sftyp: i32;
     detailnr: i32;
     detailnr2: i32;
     detailtype: i32;
     ammount: i32;
     bool hqreach;
     passenger: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ATListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ATListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ATListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ATListClass OptionsList6Obj;
     combatListId: i32;
     ATListClass combatListObj;
     combatList2Id: i32;
     ATListClass combatList2Obj;
     StatTyp: i32;
     StatMode: i32;
     int[] ChainHq;
     HQselect: i32;
     infoid: i32;
     ltnr: i32;
     locnr: i32;
     ppl: i32;
     spnr: i32;

    pub handleTimer: WindowReturnClass() => WindowReturnClass::new();

    pub fn DoRefresh()
    {
      this.comparenr = this.game.EditObj.SFCompare;
      if (this.descid > 0)
      {
        this.RemoveSubPart(this.descid);
        this.descid = 0;
      }
      if (this.OptionsListId > 0)
      {
        this.RemoveSubPart(this.OptionsListId);
        this.OptionsListId = 0;
      }
      if (this.OptionsList2Id > 0)
      {
        this.RemoveSubPart(this.OptionsList2Id);
        this.OptionsList2Id = 0;
      }
      if (this.OptionsList3Id > 0)
      {
        this.RemoveSubPart(this.OptionsList3Id);
        this.OptionsList3Id = 0;
      }
      if (this.OptionsList4Id > 0)
      {
        this.RemoveSubPart(this.OptionsList4Id);
        this.OptionsList4Id = 0;
      }
      if (this.OptionsList5Id > 0)
      {
        this.RemoveSubPart(this.OptionsList5Id);
        this.OptionsList5Id = 0;
      }
      if (this.OptionsList6Id > 0)
      {
        this.RemoveSubPart(this.OptionsList6Id);
        this.OptionsList6Id = 0;
      }
      if (this.combatListId > 0)
      {
        this.RemoveSubPart(this.combatListId);
        this.combatListId = 0;
      }
      if (this.combatList2Id > 0)
      {
        this.RemoveSubPart(this.combatList2Id);
        this.combatList2Id = 0;
      }
      this.DoStuff();
    }

    pub PPLWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.ChainHq = new int[3];
      this.tempBlink = 0.0f;
      this.game.EditObj.CurrentDescript = "";
      this.unr = -1;
      this.ppl = this.game.EditObj.PeopleSelected;
      this.StatMode = 0;
      this.detailnr = -1;
      this.detailnr2 = 0;
      this.DoStuff();
    }

    pub fn DoStuff()
    {
      this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND2MARC);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.TempText1 > 0)
        this.RemoveSubPart(this.TempText1);
      if (this.temptext2 > 0)
        this.RemoveSubPart(this.temptext2);
      if (this.temptext3 > 0)
        this.RemoveSubPart(this.temptext3);
      if (this.temptext4 > 0)
        this.RemoveSubPart(this.temptext4);
      if (this.temptext5 > 0)
        this.RemoveSubPart(this.temptext5);
      if (this.temptext6 > 0)
        this.RemoveSubPart(this.temptext6);
      if (this.temptext7 > 0)
        this.RemoveSubPart(this.temptext7);
      if (this.temptext8 > 0)
        this.RemoveSubPart(this.temptext8);
      if (this.temptext9 > 0)
        this.RemoveSubPart(this.temptext9);
      if (this.temptext10 > 0)
        this.RemoveSubPart(this.temptext10);
      if (this.TempText11 > 0)
        this.RemoveSubPart(this.TempText11);
      if (this.temptext12 > 0)
        this.RemoveSubPart(this.temptext12);
      if (this.temptext13 > 0)
        this.RemoveSubPart(this.temptext13);
      if (this.temptext14 > 0)
        this.RemoveSubPart(this.temptext14);
      if (this.temptext15 > 0)
        this.RemoveSubPart(this.temptext15);
      if (this.temptext16 > 0)
        this.RemoveSubPart(this.temptext16);
      if (this.temptext17 > 0)
        this.RemoveSubPart(this.temptext17);
      if (this.temptext18 > 0)
        this.RemoveSubPart(this.temptext18);
      if (this.temptext19 > 0)
        this.RemoveSubPart(this.temptext19);
      if (this.temptext20 > 0)
        this.RemoveSubPart(this.temptext20);
      if (this.TempText21 > 0)
        this.RemoveSubPart(this.TempText21);
      if (this.temptext22 > 0)
        this.RemoveSubPart(this.temptext22);
      if (this.temptext23 > 0)
        this.RemoveSubPart(this.temptext23);
      if (this.temptext24 > 0)
        this.RemoveSubPart(this.temptext24);
      if (this.temptext25 > 0)
        this.RemoveSubPart(this.temptext25);
      if (this.temptext26 > 0)
        this.RemoveSubPart(this.temptext26);
      if (this.temptext27 > 0)
        this.RemoveSubPart(this.temptext27);
      if (this.temptext28 > 0)
        this.RemoveSubPart(this.temptext28);
      if (this.temptext29 > 0)
        this.RemoveSubPart(this.temptext29);
      if (this.temptext30 > 0)
        this.RemoveSubPart(this.temptext30);
      if (this.TempText31 > 0)
        this.RemoveSubPart(this.TempText31);
      if (this.temptext32 > 0)
        this.RemoveSubPart(this.temptext32);
      if (this.temptext33 > 0)
        this.RemoveSubPart(this.temptext33);
      if (this.temptext34 > 0)
        this.RemoveSubPart(this.temptext34);
      if (this.temptext35 > 0)
        this.RemoveSubPart(this.temptext35);
      if (this.temptext36 > 0)
        this.RemoveSubPart(this.temptext36);
      if (this.temptext37 > 0)
        this.RemoveSubPart(this.temptext37);
      if (this.temptext38 > 0)
        this.RemoveSubPart(this.temptext38);
      if (this.temptext39 > 0)
        this.RemoveSubPart(this.temptext39);
      if (this.temptext40 > 0)
        this.RemoveSubPart(this.temptext40);
      if (this.temptext41 > 0)
        this.RemoveSubPart(this.temptext41);
      if (this.temptext42 > 0)
        this.RemoveSubPart(this.temptext42);
      if (this.temptext43 > 0)
        this.RemoveSubPart(this.temptext43);
      if (this.temptext44 > 0)
        this.RemoveSubPart(this.temptext44);
      if (this.temptext45 > 0)
        this.RemoveSubPart(this.temptext45);
      if (this.temptext46 > 0)
        this.RemoveSubPart(this.temptext46);
      if (this.hqbut0 > 0)
        this.RemoveSubPart(this.hqbut0);
      if (this.hqbut1 > 0)
        this.RemoveSubPart(this.hqbut1);
      if (this.hqbut2 > 0)
        this.RemoveSubPart(this.hqbut2);
      if (this.LogoListId > 0)
        this.RemoveSubPart(this.LogoListId);
      if (this.logolist2id > 0)
        this.RemoveSubPart(this.logolist2id);
      if (this.logolist3id > 0)
        this.RemoveSubPart(this.logolist3id);
      if (this.but1id > 0)
        this.RemoveSubPart(this.but1id);
      if (this.but1bid > 0)
        this.RemoveSubPart(this.but1bid);
      if (this.but1textid > 0)
        this.RemoveSubPart(this.but1textid);
      if (this.but2id > 0)
        this.RemoveSubPart(this.but2id);
      if (this.but2textid > 0)
        this.RemoveSubPart(this.but2textid);
      if (this.but3id > 0)
        this.RemoveSubPart(this.but3id);
      if (this.but3textid > 0)
        this.RemoveSubPart(this.but3textid);
      if (this.but4id > 0)
        this.RemoveSubPart(this.but4id);
      if (this.but4textid > 0)
        this.RemoveSubPart(this.but4textid);
      if (this.but5id > 0)
        this.RemoveSubPart(this.but5id);
      if (this.but5textid > 0)
        this.RemoveSubPart(this.but5textid);
      if (this.but6id > 0)
        this.RemoveSubPart(this.but6id);
      if (this.but6textid > 0)
        this.RemoveSubPart(this.but6textid);
      if (this.but7id > 0)
        this.RemoveSubPart(this.but7id);
      if (this.but7textid > 0)
        this.RemoveSubPart(this.but7textid);
      if (this.sliderid > 0)
        this.RemoveSubPart(this.sliderid);
      if (this.descid > 0)
        this.RemoveSubPart(this.descid);
      if (this.tab1id > 0)
        this.RemoveSubPart(this.tab1id);
      if (this.tab2id > 0)
        this.RemoveSubPart(this.tab2id);
      if (this.tab3id > 0)
        this.RemoveSubPart(this.tab3id);
      if (this.tab4id > 0)
        this.RemoveSubPart(this.tab4id);
      if (this.tab5id > 0)
        this.RemoveSubPart(this.tab5id);
      if (this.quitid > 0)
        this.RemoveSubPart(this.quitid);
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONQUIT);
      this.but1id = this.AddSubPart( tsubpart1, 952, 22, 32, 32, 1);
      let mut num1: i32 = 270;
      let mut y: i32 = 50;
      name: String = this.game.Data.PeopleObj[this.ppl].Name;
       let mut local1: &Graphics = &Expression;
      Rectangle rect1_1 = Rectangle::new(num1, y, 192, 14);
      Rectangle rectangle1 = Rectangle::new(num1, y + 14, 192, 23);
      let mut rect2_1: &Rectangle = &rectangle1
      txt2: String = name;
      DrawMod.MakeFullBoxVic2( local1, rect1_1, "SELECTED PEOPLE", rect2_1, txt2);
      SubPartClass tsubpart2;
      if (this.game.Data.PeopleObj[this.ppl].SFIll > -1)
      {
        tsubpart2 =  new SFButtonPartClass(this.game.Data.PeopleObj[this.ppl].SFIll, this.game.Data.PeopleObj[this.ppl].SFExtra, 192, 144);
        this.temptext39 = this.AddSubPart( tsubpart2, num1, y + 47, 192, 144, 0);
      }
      else
        DrawMod.DrawBlock( Expression, num1, y + 47, 192, 144,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.DrawRectangle( Expression, num1 - 1, y + 46, 193, 145,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      let mut num2: i32 = 522;
      let mut num3: i32 = 60;
      this.combatListObj = ATListClass::new();
      let mut peopleCounter1: i32 = this.game.Data.PeopleCounter;
      for (let mut tdata: i32 = 0; tdata <= peopleCounter1; tdata += 1)
        this.combatListObj.add(this.game.Data.PeopleObj[tdata].Name, tdata);
      if (this.combatListId > 0)
      {
        this.SubPartList[this.SubpartNr(this.combatListId)].Refresh(this.combatListObj, this.ppl);
        this.SubPartFlag[this.SubpartNr(this.combatListId)] = true;
      }
      else
      {
        tsubpart2 =  new ATListSubPartClass(this.combatListObj, 11, 225, this.ppl, this.game, true, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num2, bby: num3);
        this.combatListId = this.AddSubPart( tsubpart2, num2, num3, 225, 192, 0);
      }
      let mut num4: i32 = 265;
      let mut num5: i32 = 96;
      tsubpart2 =  new TextButtonPartClass("General Stats", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num5 + 170), bby: num4, tred: (this.StatMode == 0));
      this.tab1id = this.AddSubPart( tsubpart2, num5 + 170, num4, 150, 35, 1);
      tsubpart2 =  new TextButtonPartClass("Production Mods", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num5 + 340), bby: num4, tred: (this.StatMode == 1));
      this.tab2id = this.AddSubPart( tsubpart2, num5 + 340, num4, 150, 35, 1);
      tsubpart2 =  new TextButtonPartClass("Item Types", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num5 + 510), bby: num4, tred: (this.StatMode == 2));
      this.tab3id = this.AddSubPart( tsubpart2, num5 + 510, num4, 150, 35, 1);
      DrawMod.DrawBlock( Expression, 65, num4 + 55, 890, 355,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  Expression, 65, num4 + 55, 890, 355, -1, -1);
      Rectangle rectangle2;
      if (this.StatMode == 0)
      {
        let mut num6: i32 = 115;
        let mut num7: i32 = 375;
        this.OptionsList3Obj = ATListClass::new();
         let mut local2: &Graphics = &Expression;
        rectangle1 = Rectangle::new(num6, num7 - 14, 330, 13);
        let mut rect1_2: &Rectangle = &rectangle1
        txt1: String = Strings.UCase("Under People      Morale            Bat.For Mod    Ba.Vs Mod");
        let mut rect2_2: &Rectangle = &rectangle2
        DrawMod.MakeFullBoxVic2( local2, rect1_2, txt1, rect2_2, "");
        let mut peopleCounter2: i32 = this.game.Data.PeopleCounter;
        for (let mut index: i32 = 0; index <= peopleCounter2; index += 1)
        {
          let mut peopleGroup: i32 = this.game.Data.PeopleObj[index].PeopleGroup;
          this.OptionsList3Obj.add(this.game.Data.PeopleObj[index].Name, -1, Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].BaseMorale[peopleGroup])), Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].BattleForMod[peopleGroup])), Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].BattleVSMod[peopleGroup])));
        }
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart2 =  new ATListSubPartClass(this.OptionsList3Obj, 15, 330, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 230, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num6, bby: num7);
          this.OptionsList3Id = this.AddSubPart( tsubpart2, num6, num7, 330, 256, 0);
        }
        let mut num8: i32 = 530;
        let mut num9: i32 = 390;
        tText: String = this.game.Data.PeopleObj[this.ppl].Name + "\r\n\r\n" + this.game.Data.PeopleObj[this.ppl].Description;
        DrawMod.DrawPaperSheet( Expression, num8 - 20, num9 - 10, 390, 220);
        tsubpart2 =  new PaperAreaClass(this.game, 360, 10,  null, "Description", false, tText, this.game.VicColor8, tItemSize: 20, tbackbitmap: ( this.OwnBitmap), bbx: num8, bby: num9);
        this.descid = this.AddSubPart( tsubpart2, num8, num9, 360, 220, 0);
      }
      if (this.StatMode == 1)
      {
        let mut num10: i32 = 200;
        let mut num11: i32 = 375;
        this.OptionsList4Obj = ATListClass::new();
         let mut local3: &Graphics = &Expression;
        rectangle1 = Rectangle::new(num10, num11 - 14, 730, 13);
        let mut rect1_3: &Rectangle = &rectangle1
        txt1: String = Strings.UCase("Under Regime People                          ProdMod 1                 ProdMod 2                ProdMod 3               ProdMod4");
        let mut rect2_3: &Rectangle = &rectangle2
        DrawMod.MakeFullBoxVic2( local3, rect1_3, txt1, rect2_3, "");
        let mut peopleCounter3: i32 = this.game.Data.PeopleCounter;
        for (let mut index: i32 = 0; index <= peopleCounter3; index += 1)
        {
          let mut peopleGroup: i32 = this.game.Data.PeopleObj[index].PeopleGroup;
          this.OptionsList4Obj.add(this.game.Data.PeopleObj[index].Name, -1, Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].ProdMod[peopleGroup])), Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].ProdMod2[peopleGroup])), Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].ProdMod3[peopleGroup])), Strings.Trim(Conversion.Str( this.game.Data.PeopleObj[this.ppl].ProdMod4[peopleGroup])));
        }
        if (this.OptionsList4Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
        }
        else
        {
          tsubpart2 =  new ATListSubPartClass(this.OptionsList4Obj, 15, 630, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 430, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num10, bby: num11);
          this.OptionsList4Id = this.AddSubPart( tsubpart2, num10, num11, 630, 256, 0);
        }
      }
      if (this.StatMode == 2)
      {
        let mut num12: i32 = 200;
        let mut num13: i32 = 375;
        this.OptionsList4Obj = ATListClass::new();
         let mut local4: &Graphics = &Expression;
        rectangle1 = Rectangle::new(num12, num13 - 14, 730, 13);
        let mut rect1_4: &Rectangle = &rectangle1
        txt1: String = Strings.UCase("ItemType                                              ProdMod                                                      People Overrule");
        let mut rect2_4: &Rectangle = &rectangle2
        DrawMod.MakeFullBoxVic2( local4, rect1_4, txt1, rect2_4, "");
        let mut itemTypeCounter: i32 = this.game.Data.ItemTypeCounter;
        for (let mut index: i32 = 0; index <= itemTypeCounter; index += 1)
          this.OptionsList4Obj.add(this.game.Data.ItemTypeObj[index].Name, -1, this.game.Data.ItemTypeObj[index].UseProdMod > 1 ? Strings.Trim(Conversion.Str( this.game.Data.ItemTypeObj[index].UseProdMod)) : "1", this.game.Data.ItemTypeObj[index].PeopleMod <= 0 ? "none" : this.game.Data.PeopleObj[this.game.Data.ItemTypeObj[index].PeopleMod].Name);
        if (this.OptionsList4Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
        }
        else
        {
          tsubpart2 =  new ATListSubPartClass(this.OptionsList4Obj, 15, 630, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 430, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num12, bby: num13);
          this.OptionsList4Id = this.AddSubPart( tsubpart2, num12, num13, 630, 256, 0);
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub object ReturnSFSpriteNr(typ: i32, regnr: i32, pplnr: i32)
    {
      let mut symbolSpriteId: i32 = this.game.Data.SFTypeObj[typ].SymbolSpriteID;
      if (regnr > -1)
      {
        if (this.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = this.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (this.game.Data.SFTypeObj[typ].ExtraCode[index] == this.game.Data.RegimeObj[regnr].ExtraGraphicUse)
              symbolSpriteId = this.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
        else if (pplnr > -1 && this.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = this.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (this.game.Data.SFTypeObj[typ].ExtraCode[index] == this.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              symbolSpriteId = this.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
      }
      else if (this.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
      {
        let mut extraCounter: i32 = this.game.Data.SFTypeObj[typ].ExtraCounter;
        for (let mut index: i32 = 0; index <= extraCounter; index += 1)
        {
          if (this.game.Data.SFTypeObj[typ].ExtraCode[index] == this.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
            symbolSpriteId = this.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
        }
      }
      return  symbolSpriteId;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = this.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 = this.SubPartID[index];
            if (num1 == this.tab1id)
            {
              this.StatMode = 0;
              this.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.tab2id)
            {
              this.StatMode = 1;
              this.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.tab3id)
            {
              this.StatMode = 2;
              this.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.tab4id)
            {
              this.StatMode = 3;
              this.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.tab5id)
            {
              this.StatMode = 4;
              this.DoRefresh();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.quitid)
            {
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            num2: i32;
            if (num1 == this.OptionsListId)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.LogoListId)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.logolist2id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.logolist3id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList2Id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList3Id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.descid)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList4Id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList5Id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList6Id)
            {
              num2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.combatListId)
            {
              let mut num3: i32 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              if (num3 > -1)
              {
                this.ppl = num3;
                this.DoRefresh();
              }
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.combatList2Id)
            {
              let mut num4: i32 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              if (num4 > -1)
              {
                this.locnr = num4;
                if (this.locnr == 9999)
                  this.locnr = -1;
                this.DoRefresh();
              }
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.but1id || num1 == this.but1bid)
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.hqbut0)
            {
              this.HQselect = this.ChainHq[0];
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.hqbut1)
            {
              this.HQselect = this.ChainHq[1];
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.but7id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 74, -1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.hqbut2)
            {
              this.HQselect = this.ChainHq[2];
              this.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.but2id)
            {
              if (Interaction.MsgBox( "Are you sure you want to disband this subformation?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                if (this.game.EditObj.SoundOn)
                  SoundMod.PlayAWave(this.game.AppPath + "sound/disband.wav",  this.game.EditObj);
                OrderResult orderResult = this.game.ProcessingObj.DoDisband(this.game.EditObj.UnitSelected, this.sfnr);
                if (orderResult.OK)
                {
                  if (orderResult.ErrorString.Length > 1)
                  {
                    let mut num5: i32 =  Interaction.MsgBox( orderResult.ErrorString, Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            else
            {
              if (num1 == this.sliderid)
              {
                this.detailnr2 = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index], b);
                this.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.but3id)
              {
                if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount > 6 && this.game.Data.SFObj[this.sfnr].Qty != this.detailnr2)
                {
                  let mut num6: i32 =  Interaction.MsgBox( "You can only upgrade ALL because there is already 8 subformations in unit.");
                  return windowReturnClass;
                }
                OrderResult orderResult = this.game.ProcessingObj.DoUpgrade(this.game.EditObj.UnitSelected, this.sfnr, this.detailnr2, this.HQselect);
                if (this.game.EditObj.SoundOn)
                  SoundMod.PlayAWave(this.game.AppPath + "sound/building.wav",  this.game.EditObj);
                if (orderResult.OK)
                {
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
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
