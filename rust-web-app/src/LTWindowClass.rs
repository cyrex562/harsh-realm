// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LTWindowClass
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
  pub class LTWindowClass : WindowClass
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

    pub LTWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.ChainHq = new int[3];
      this.tempBlink = 0.0f;
      this.game.EditObj.CurrentDescript = "";
      this.unr = -1;
      this.ppl = -1;
      if (this.game.SelectX > -1)
      {
        this.ltnr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType;
        this.spnr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr;
        this.locnr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
        if (this.locnr > -1)
          this.ppl = this.game.Data.LocObj[this.locnr].People;
        if (this.locnr > -1)
        {
          this.locnr = this.game.Data.LocObj[this.locnr].Type;
          this.ppl = this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].People;
        }
      }
      if (this.game.EditObj.LocTypeSelected > -1)
      {
        this.locnr = this.game.EditObj.LocTypeSelected;
        this.game.EditObj.LocTypeSelected = -1;
      }
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
      let mut x1: i32 =  296;
      let mut y1: i32 =  50;
      name1: String = this.game.Data.LandscapeTypeObj[this.ltnr].Name;
       let mut local1: &Graphics = &Expression;
      Rectangle rectangle1 = Rectangle::new(x1, y1, 192, 14);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(x1, y1 + 14, 192, 23);
      let mut rect2_1: &Rectangle = &rectangle2
      txt2_1: String = name1;
      DrawMod.MakeFullBoxVic2( local1, rect1_1, "SELECTED LANDSCAPE TYPE", rect2_1, txt2_1);
      let mut x2: i32 =  532;
      let mut y2: i32 =  50;
      str1: String = this.locnr <= -1 ? "No Location Type selected" : this.game.Data.LocTypeObj[this.locnr].Name;
       let mut local2: &Graphics = &Expression;
      rectangle2 = Rectangle::new(x2, y2, 192, 14);
      let mut rect1_2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(x2, y2 + 14, 192, 23);
      let mut rect2_2: &Rectangle = &rectangle1
      txt2_2: String = str1;
      DrawMod.MakeFullBoxVic2( local2, rect1_2, "SELECTED LOCATION TYPE", rect2_2, txt2_2);
      let mut num1: i32 =  384;
      DrawMod.DrawRectangle( Expression, num1 - 1, y2 + 66, 251, 104,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      this.spnr = 0;
      let mut num2: i32 =  num1;
      let mut num3: i32 =  y2 + 67;
      if (this.ltnr > -1 & this.spnr > -1)
      {
        if ( this.game.Data.RuleVar[869] == 0.0 |  this.game.Data.RuleVar[869] == 3.0)
        {
          let mut nr: i32 =  this.game.Data.LandscapeTypeObj[this.ltnr].BasicPicID[this.spnr];
           let mut local3: &Graphics = &Expression;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local4: &Bitmap = &bitmap;
          let mut x3: i32 =  num2;
          let mut y3: i32 =  num3;
          DrawMod.DrawScaled( local3,  local4, x3, y3, 250, 103);
        }
        else
        {
          if ( this.game.Data.RuleVar[869] == 1.0)
          {
            let mut nr: i32 =  this.game.Data.LandscapeTypeObj[this.ltnr].SidewaysSPriteID1[this.spnr];
             let mut local5: &Graphics = &Expression;
            bitmap: Bitmap = BitmapStore.GetBitmap(nr);
             let mut local6: &Bitmap = &bitmap;
            let mut x4: i32 =  num2;
            let mut y4: i32 =  num3;
            DrawMod.DrawScaled( local5,  local6, x4, y4, 250, 103);
          }
          let mut nr1: i32 =  this.game.Data.LandscapeTypeObj[this.ltnr].SidewaysSPriteID2[this.spnr];
           let mut local7: &Graphics = &Expression;
          bitmap1: Bitmap = BitmapStore.GetBitmap(nr1);
           let mut local8: &Bitmap = &bitmap1;
          let mut x5: i32 =  num2;
          let mut y5: i32 =  num3;
          DrawMod.DrawScaled( local7,  local8, x5, y5, 250, 103);
          let mut nr2: i32 =  this.game.Data.LandscapeTypeObj[this.ltnr].SidewaysSPriteID3[this.spnr];
           let mut local9: &Graphics = &Expression;
          bitmap2: Bitmap = BitmapStore.GetBitmap(nr2);
           let mut local10: &Bitmap = &bitmap2;
          let mut x6: i32 =  num2;
          let mut y6: i32 =  num3;
          DrawMod.DrawScaled( local9,  local10, x6, y6, 250, 103);
        }
        if (this.locnr > -1 && this.game.Data.LocTypeObj[this.locnr].PictureLT > -1)
        {
          if ( this.game.Data.RuleVar[869] == 0.0 |  this.game.Data.RuleVar[869] == 3.0)
          {
            let mut nr: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.locnr].PictureLT].BasicPicID[this.game.Data.LocTypeObj[this.locnr].PictureSprite];
             let mut local11: &Graphics = &Expression;
            bitmap: Bitmap = BitmapStore.GetBitmap(nr);
             let mut local12: &Bitmap = &bitmap;
            let mut x7: i32 =  num2;
            let mut y7: i32 =  num3;
            DrawMod.DrawScaled( local11,  local12, x7, y7, 250, 103);
          }
          else
          {
            if ( this.game.Data.RuleVar[869] == 1.0)
            {
              let mut nr: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.locnr].PictureLT].SidewaysSPriteID1[this.game.Data.LocTypeObj[this.locnr].PictureSprite];
               let mut local13: &Graphics = &Expression;
              bitmap: Bitmap = BitmapStore.GetBitmap(nr);
               let mut local14: &Bitmap = &bitmap;
              let mut x8: i32 =  num2;
              let mut y8: i32 =  num3;
              DrawMod.DrawScaled( local13,  local14, x8, y8, 250, 103);
            }
            let mut nr3: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.locnr].PictureLT].SidewaysSPriteID2[this.game.Data.LocTypeObj[this.locnr].PictureSprite];
             let mut local15: &Graphics = &Expression;
            bitmap3: Bitmap = BitmapStore.GetBitmap(nr3);
             let mut local16: &Bitmap = &bitmap3;
            let mut x9: i32 =  num2;
            let mut y9: i32 =  num3;
            DrawMod.DrawScaled( local15,  local16, x9, y9, 250, 103);
            let mut nr4: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.locnr].PictureLT].SidewaysSPriteID3[this.game.Data.LocTypeObj[this.locnr].PictureSprite];
             let mut local17: &Graphics = &Expression;
            bitmap4: Bitmap = BitmapStore.GetBitmap(nr4);
             let mut local18: &Bitmap = &bitmap4;
            let mut x10: i32 =  num2;
            let mut y10: i32 =  num3;
            DrawMod.DrawScaled( local17,  local18, x10, y10, 250, 103);
          }
        }
      }
      let mut num4: i32 =  60;
      let mut num5: i32 =  60;
      this.combatListObj = ATListClass::new();
      let mut num6: i32 =  -1;
      let mut tlistselect: i32 =  -1;
      let mut landscapeTypeCounter1: i32 =  this.game.Data.LandscapeTypeCounter;
      str2: String;
      str3: String;
      for (let mut tdata: i32 =  0; tdata <= landscapeTypeCounter1; tdata += 1)
      {
        if (!this.game.Data.LandscapeTypeObj[tdata].DontShowInList)
        {
          str2 = "";
          str3 = "";
          num6 += 1;
          if (this.ltnr == tdata)
            tlistselect = num6;
          this.combatListObj.add(this.game.Data.LandscapeTypeObj[tdata].Name, tdata);
        }
      }
      if (this.combatListId > 0)
      {
        this.SubPartList[this.SubpartNr(this.combatListId)].Refresh(this.combatListObj, tlistselect);
        this.SubPartFlag[this.SubpartNr(this.combatListId)] = true;
      }
      else
      {
        let mut tsubpart2: SubPartClass =  new ATListSubPartClass(this.combatListObj, 11, 225, tlistselect, this.game, true, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num4, bby: num5);
        this.combatListId = this.AddSubPart( tsubpart2, num4, num5, 225, 192, 0);
      }
      let mut num7: i32 =  745;
      let mut num8: i32 =  60;
      this.combatList2Obj = ATListClass::new();
      this.combatList2Obj.add("None", 9999);
      let mut locTypeCounter1: i32 =  this.game.Data.LocTypeCounter;
      for (let mut tdata: i32 =  0; tdata <= locTypeCounter1; tdata += 1)
      {
        str2 = "";
        str3 = "";
        this.combatList2Obj.add(this.game.Data.LocTypeObj[tdata].Name, tdata);
      }
      SubPartClass tsubpart3;
      if (this.combatList2Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.combatList2Id)].Refresh(this.combatList2Obj, this.locnr + 1);
        this.SubPartFlag[this.SubpartNr(this.combatList2Id)] = true;
      }
      else
      {
        tsubpart3 =  new ATListSubPartClass(this.combatList2Obj, 11, 225, this.locnr + 1, this.game, true, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num7, bby: num8);
        this.combatList2Id = this.AddSubPart( tsubpart3, num7, num8, 225, 192, 0);
      }
      let mut num9: i32 =  300;
      let mut num10: i32 =  15;
      if (this.locnr > -1)
        num10 -= 85;
      tsubpart3 =  new TextButtonPartClass("General Stats", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num10 + 170), bby: num9, tred: (this.StatMode == 0));
      this.tab1id = this.AddSubPart( tsubpart3, num10 + 170, num9, 150, 35, 1);
      tsubpart3 =  new TextButtonPartClass("Entrench Stats", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num10 + 340), bby: num9, tred: (this.StatMode == 1));
      this.tab2id = this.AddSubPart( tsubpart3, num10 + 340, num9, 150, 35, 1);
      tsubpart3 =  new TextButtonPartClass("Combat Mods", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num10 + 510), bby: num9, tred: (this.StatMode == 2));
      this.tab3id = this.AddSubPart( tsubpart3, num10 + 510, num9, 150, 35, 1);
      tsubpart3 =  new TextButtonPartClass("Movement Stats", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num10 + 680), bby: num9, tred: (this.StatMode == 3));
      this.tab4id = this.AddSubPart( tsubpart3, num10 + 680, num9, 150, 35, 1);
      if (this.locnr > -1)
      {
        tsubpart3 =  new TextButtonPartClass("Location Details", 150, tBackbitmap: ( this.OwnBitmap), bbx: (num10 + 850), bby: num9, tred: (this.StatMode == 4));
        this.tab5id = this.AddSubPart( tsubpart3, num10 + 850, num9, 150, 35, 1);
        num10 += 85;
      }
      else if (this.StatMode == 4)
        this.StatMode = 0;
      DrawMod.DrawBlock( Expression, num10 + 50, num9 + 55, 890, 355,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  Expression, num10 + 50, num9 + 55, 890, 355, -1, -1);
      num11: i32;
      num12: i32;
      num13: i32;
      Rectangle rectangle3;
      if (this.StatMode == 0)
      {
        num11 = -1;
        num12 = -1;
        num13 = 0;
        let mut num14: i32 =  150;
        let mut num15: i32 =  401;
        if (this.locnr == -1)
          num14 = 347;
        this.OptionsList3Obj = ATListClass::new();
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Can Build Road", -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].CanBuildRoad)));
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Can Amphibious", -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].CanAmph)));
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Can Paradrop", -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].CanParadrop)));
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Road Cost Mod", -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].RoadCostModifier)));
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("LandscapeType Group", -1, this.game.Data.TempString[this.game.Data.LandscapeTypeObj[this.ltnr].BuildGround + 100]);
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Is Sea", -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].IsSea)));
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Hide Points", -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].HidePts)));
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList3Obj, 6, 330, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 165, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num14, bby: num15);
          this.OptionsList3Id = this.AddSubPart( tsubpart3, num14, num15, 330, 112, 0);
        }
         let mut local19: &Graphics = &Expression;
        rectangle2 = Rectangle::new(num14, num15 - 14, 330, 13);
        let mut rect1_3: &Rectangle = &rectangle2
        let mut rect2_3: &Rectangle = &rectangle3
        DrawMod.MakeFullBoxVic2( local19, rect1_3, "LANDSCAPE GENERAL STATS", rect2_3, "");
        if (this.locnr > -1)
        {
          let mut num16: i32 =  150;
          let mut num17: i32 =  546;
          this.OptionsList4Obj = ATListClass::new();
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Loc on Destruct", -1, !(this.game.Data.LocTypeObj[this.locnr].OnDestructLT > -1 & !this.game.Data.LocTypeObj[this.locnr].Invincible) ? "Indestructable" : this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.locnr].OnDestructLT].Name);
          str2 = "";
          str3 = "";
          tvalue: String = Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].IsAirfield));
          if (this.game.Data.LocTypeObj[this.locnr].IsAirfield)
            tvalue = this.game.Data.LocTypeObj[this.locnr].TopAirStack <= 0 ? tvalue + ", unlimited stacking" : tvalue + ", max stack " + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].TopAirStack));
          this.OptionsList4Obj.add("Is Airfield", -1, tvalue);
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Is Port", -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].IsPort)));
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Buildable", -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].Buildable)));
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Effect on LT", -1, this.game.Data.LocTypeObj[this.locnr].PictureLT <= -1 ? "No effect on LT" : "Adds to LT mods");
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Structural Points", -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].StructuralPts)));
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Auto Repair", -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].AutoRecoverPts)));
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Is Upgrade", -1, this.game.Data.LocTypeObj[this.locnr].UpgradeNr <= -1 ? "No" : "Yes, from " + this.game.Data.LocTypeObj[this.game.Data.LocTypeObj[this.locnr].UpgradeNr].Name);
          str2 = "";
          str3 = "";
          this.OptionsList4Obj.add("Production Points", -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].MaxProd)));
           let mut local20: &Graphics = &Expression;
          rectangle2 = Rectangle::new(num16, num17 - 14, 330, 13);
          let mut rect1_4: &Rectangle = &rectangle2
          let mut rect2_4: &Rectangle = &rectangle3
          DrawMod.MakeFullBoxVic2( local20, rect1_4, "LOCATION TYPE GENERAL STATS", rect2_4, "");
          if (this.OptionsList4Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, -1);
            this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
          }
          else
          {
            tsubpart3 =  new ATListSubPartClass(this.OptionsList4Obj, 8, 330, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 165, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num16, bby: num17);
            this.OptionsList4Id = this.AddSubPart( tsubpart3, num16, num17, 330, 144, 0);
          }
        }
        str4: String = "";
        if (this.locnr > -1)
          str4 = str4 + this.game.Data.LocTypeObj[this.locnr].Name + "\r\n\r\n" + this.game.Data.LocTypeObj[this.locnr].Description + "\r\n\r\n";
        tText: String = str4 + this.game.Data.LandscapeTypeObj[this.ltnr].Name + "\r\n\r\n" + this.game.Data.LandscapeTypeObj[this.ltnr].Description;
        if (this.locnr > -1)
        {
          let mut num18: i32 =  530;
          let mut num19: i32 =  411;
          DrawMod.DrawPaperSheet( Expression, num18 - 20, num19 - 10, 390, 280);
          tsubpart3 =  new PaperAreaClass(this.game, 360, 13,  null, "Description", false, tText, this.game.VicColor8, tItemSize: 20, tbackbitmap: ( this.OwnBitmap), bbx: num18, bby: num19);
          this.descid = this.AddSubPart( tsubpart3, num18, num19, 360, 280, 0);
        }
        else
        {
          let mut num20: i32 =  230;
          let mut num21: i32 =  545;
          DrawMod.DrawPaperSheet( Expression, num20 - 20, num21 - 10, 590, 160);
          tsubpart3 =  new PaperAreaClass(this.game, 560, 7,  null, "Description", false, tText, this.game.VicColor8, tItemSize: 20, tbackbitmap: ( this.OwnBitmap), bbx: num20, bby: num21);
          this.descid = this.AddSubPart( tsubpart3, num20, num21, 560, 160, 0);
        }
      }
      tvalue3: String;
      tvalue4: String;
      if (this.StatMode == 1)
      {
        let mut index1: i32 =  -1;
        if (this.locnr > -1 && this.game.Data.LocTypeObj[this.locnr].PictureLT > -1)
          index1 = this.game.Data.LocTypeObj[this.locnr].PictureLT;
        let mut num22: i32 =  200;
        let mut num23: i32 =  405;
        this.OptionsList3Obj = ATListClass::new();
        if (index1 > -1)
        {
           let mut local21: &Graphics = &Expression;
          rectangle2 = Rectangle::new(num22, num23 - 14, 730, 13);
          let mut rect1_5: &Rectangle = &rectangle2
          let mut rect2_5: &Rectangle = &rectangle3
          DrawMod.MakeFullBoxVic2( local21, rect1_5, "                                                                               LANDSCAPE TYPE STATS                              LOCATION TYPE STATS", rect2_5, "");
          this.OptionsList3Obj.add("SUBFORMATION TYPE GROUP", -1, "AUTO ENTR", "MAX ENTR", "AUTO ENTR", "MAX ENTR");
        }
        else
          this.OptionsList3Obj.add("SUBFORMATION TYPE GROUP", -1, "AUTO ENTR", "MAX ENTR");
        let mut index2: i32 =  0;
        do
        {
          if (!Information.IsNothing( this.game.Data.TempString[400 + index2]) && this.game.Data.TempString[400 + index2].Length > 1)
          {
            tname: String = this.game.Data.TempString[400 + index2];
            tvalue: String = Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].DefBonus[index2]));
            tvalue2: String = Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].DefBonusMax[index2]));
            if (index1 > -1)
            {
              tvalue3 = "+" + Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[index1].DefBonus[index2]));
              tvalue4 = "+" + Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[index1].DefBonusMax[index2]));
            }
            if (index1 > -1)
              this.OptionsList3Obj.add(tname, -1, tvalue, tvalue2, tvalue3, tvalue4);
            else
              this.OptionsList3Obj.add(tname, -1, tvalue, tvalue2);
          }
          index2 += 1;
        }
        while (index2 <= 99);
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList3Obj, 15, 630, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 430, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num22, bby: num23);
          this.OptionsList3Id = this.AddSubPart( tsubpart3, num22, num23, 630, 256, 0);
        }
      }
      if (this.StatMode == 2)
      {
        let mut index3: i32 =  -1;
        if (this.locnr > -1 && this.game.Data.LocTypeObj[this.locnr].PictureLT > -1)
          index3 = this.game.Data.LocTypeObj[this.locnr].PictureLT;
        let mut num24: i32 =  200;
        let mut num25: i32 =  405;
        this.OptionsList3Obj = ATListClass::new();
        if (index3 > -1)
        {
           let mut local22: &Graphics = &Expression;
          rectangle2 = Rectangle::new(num24, num25 - 14, 730, 13);
          let mut rect1_6: &Rectangle = &rectangle2
          let mut rect2_6: &Rectangle = &rectangle3
          DrawMod.MakeFullBoxVic2( local22, rect1_6, "                                                                               LANDSCAPE TYPE STATS                              LOCATION TYPE STATS", rect2_6, "");
          this.OptionsList3Obj.add("SUBFORMATION TYPE", -1, "OFF MOD", "DEF MOD", "OFF MOD", "DEF MOD");
        }
        else
          this.OptionsList3Obj.add("SUBFORMATION TYPE", -1, "OFF MOD", "DEF MOD");
        let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
        for (let mut index4: i32 =  0; index4 <= sfTypeCounter; index4 += 1)
        {
          if (!this.game.Data.SFTypeObj[index4].DontShowInList)
          {
            let mut num26: i32 =  0;
            if ( this.game.Data.SFTypeObj[index4].CombatModAtt[this.ltnr] != 1.0)
              num26 = 1;
            if ( this.game.Data.SFTypeObj[index4].CombatModDef[this.ltnr] != 1.0)
              num26 = 1;
            if (index3 > -1)
            {
              if ( this.game.Data.SFTypeObj[index4].CombatModAtt[index3] != 1.0)
                num26 = 1;
              if ( this.game.Data.SFTypeObj[index4].CombatModDef[index3] != 1.0)
                num26 = 1;
            }
            if (num26 == 1)
            {
              name2: String = this.game.Data.SFTypeObj[index4].Name;
              tvalue: String = "* " + Strings.Trim(Conversion.Str( this.game.Data.SFTypeObj[index4].CombatModAtt[this.ltnr]));
              tvalue2: String = "* " + Strings.Trim(Conversion.Str( this.game.Data.SFTypeObj[index4].CombatModDef[this.ltnr]));
              if (index3 > -1)
              {
                tvalue3 = "* " + Strings.Trim(Conversion.Str( this.game.Data.SFTypeObj[index4].CombatModAtt[index3]));
                tvalue4 = "* " + Strings.Trim(Conversion.Str( this.game.Data.SFTypeObj[index4].CombatModDef[index3]));
              }
              if (index3 > -1)
                this.OptionsList3Obj.add(name2, -1, tvalue, tvalue2, tvalue3, tvalue4);
              else
                this.OptionsList3Obj.add(name2, -1, tvalue, tvalue2);
            }
          }
        }
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList3Obj, 15, 630, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 430, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num24, bby: num25);
          this.OptionsList3Id = this.AddSubPart( tsubpart3, num24, num25, 630, 256, 0);
        }
      }
      if (this.StatMode == 3)
      {
        let mut num27: i32 =  -1;
        if (this.locnr > -1 && this.game.Data.LocTypeObj[this.locnr].PictureLT > -1)
          num27 = this.game.Data.LocTypeObj[this.locnr].PictureLT;
        let mut num28: i32 =  300;
        let mut num29: i32 =  405;
        this.OptionsList3Obj = ATListClass::new();
        this.OptionsList3Obj.add("MOVEMENT TYPE", -1, "AP COST");
        let mut index: i32 =  0;
        do
        {
          if (this.game.Data.TempString[index].Length > 1)
            this.OptionsList3Obj.add(this.game.Data.TempString[index], -1, Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.ltnr].MoveCost[index])));
          index += 1;
        }
        while (index <= 99);
         let mut local23: &Graphics = &Expression;
        rectangle2 = Rectangle::new(num28, num29 - 14, 730, 13);
        let mut rect1_7: &Rectangle = &rectangle2
        let mut rect2_7: &Rectangle = &rectangle3
        DrawMod.MakeFullBoxVic2( local23, rect1_7, "LANDSCAPE TYPE MOVEMENT COSTS", rect2_7, "");
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList3Obj, 15, 430, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 230, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num28, bby: num29);
          this.OptionsList3Id = this.AddSubPart( tsubpart3, num28, num29, 430, 256, 0);
        }
      }
      if (this.StatMode == 4 & this.locnr > -1)
      {
        num11 = -1;
        num12 = -1;
        num13 = 0;
        let mut num30: i32 =  150;
        let mut num31: i32 =  401;
        this.OptionsList3Obj = ATListClass::new();
        str2 = "";
        str3 = "";
        this.OptionsList3Obj.add("Buildable", -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].Buildable)));
        str2 = "";
        str3 = "";
        tvalue: String;
        if ( this.game.Data.RuleVar[902] == 1.0)
        {
          tvalue = "No".to_owned();
        }
        else
        {
          tvalue = "No".to_owned();
          if (this.game.Data.LocTypeObj[this.locnr].EPCost > 0)
            tvalue = "Yes, EP cost " + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].EPCost));
        }
        this.OptionsList3Obj.add("Repairable", -1, tvalue);
        if (this.game.Data.LocTypeObj[this.locnr].Buildable)
        {
          str2 = "";
          str3 = "";
          this.OptionsList3Obj.add("Is Upgrade", -1, this.game.Data.LocTypeObj[this.locnr].UpgradeNr <= -1 ? "No" : "Yes, from " + this.game.Data.LocTypeObj[this.game.Data.LocTypeObj[this.locnr].UpgradeNr].Name);
          let mut index5: i32 =  0;
          do
          {
            if (this.game.Data.LocTypeObj[this.locnr].Research[index5] > -1)
            {
              str2 = "";
              str3 = "";
              this.OptionsList3Obj.add("Research Req", -1, this.game.Data.ResearchObj[this.game.Data.LocTypeObj[this.locnr].Research[index5]].Name);
            }
            index5 += 1;
          }
          while (index5 <= 4);
          if (this.game.Data.LocTypeObj[this.locnr].EPCost > 0)
            this.OptionsList3Obj.add("EP Cost", -1, Conversions.ToString(this.game.Data.LocTypeObj[this.locnr].EPCost));
          if (this.game.Data.LocTypeObj[this.locnr].PPCost > 0)
            this.OptionsList3Obj.add("PP Cost", -1, Conversions.ToString(this.game.Data.LocTypeObj[this.locnr].PPCost));
          if (this.game.Data.LocTypeObj[this.locnr].SupplyCost > 0)
            this.OptionsList3Obj.add("Supply Cost", -1, Conversions.ToString( Math.Round( ( this.game.Data.LocTypeObj[this.locnr].SupplyCost / this.game.Data.RuleVar[77]))));
          let mut index6: i32 =  0;
          do
          {
            if (this.game.Data.LocTypeObj[this.locnr].VarQty[index6] > 0 | this.game.Data.LocTypeObj[this.locnr].VarType[index6] > -1)
              this.OptionsList3Obj.add(this.game.Data.RegimeSlotName[this.game.Data.LocTypeObj[this.locnr].VarType[index6]], -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].VarQty[index6])));
            index6 += 1;
          }
          while (index6 <= 4);
        }
         let mut local24: &Graphics = &Expression;
        rectangle2 = Rectangle::new(num30, num31 - 14, 330, 13);
        let mut rect1_8: &Rectangle = &rectangle2
        let mut rect2_8: &Rectangle = &rectangle3
        DrawMod.MakeFullBoxVic2( local24, rect1_8, "BUILDING REQUIREMENTS", rect2_8, "");
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList3Obj, 6, 330, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 240, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num30, bby: num31);
          this.OptionsList3Id = this.AddSubPart( tsubpart3, num30, num31, 330, 112, 0);
        }
        let mut num32: i32 =  150;
        let mut num33: i32 =  546;
        this.OptionsList4Obj = ATListClass::new();
        let mut index7: i32 =  0;
        do
        {
          if (!Information.IsNothing( this.game.Data.TempString[300 + index7]) && this.game.Data.TempString[300 + index7].Length > 1 & this.game.Data.LocTypeObj[this.locnr].ItemGroup[index7])
          {
            str2 = "";
            str3 = "";
            this.OptionsList4Obj.add(this.game.Data.TempString[300 + index7], -1);
          }
          index7 += 1;
        }
        while (index7 <= 99);
         let mut local25: &Graphics = &Expression;
        rectangle2 = Rectangle::new(num32, num33 - 14, 330, 13);
        let mut rect1_9: &Rectangle = &rectangle2
        let mut rect2_9: &Rectangle = &rectangle3
        DrawMod.MakeFullBoxVic2( local25, rect1_9, "PRODUCABLE ITEM TYPE GROUPS", rect2_9, "");
        if (this.OptionsList4Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList4Obj, 7, 330, -1, this.game, true, tHighlight: false, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num32, bby: num33);
          this.OptionsList4Id = this.AddSubPart( tsubpart3, num32, num33, 330, 128, 0);
        }
        let mut num34: i32 =  550;
        let mut num35: i32 =  401;
        this.OptionsList5Obj = ATListClass::new();
        let mut index8: i32 =  0;
        do
        {
          if (!Information.IsNothing( this.game.Data.TempString[200 + index8]) && this.game.Data.TempString[200 + index8].Length > 1 & this.game.Data.LocTypeObj[this.locnr].PeopleGroup[index8])
          {
            str2 = "";
            str3 = "";
            this.OptionsList5Obj.add(this.game.Data.TempString[200 + index8], -1);
          }
          index8 += 1;
        }
        while (index8 <= 99);
         let mut local26: &Graphics = &Expression;
        rectangle2 = Rectangle::new(num34, num35 - 14, 330, 13);
        let mut rect1_10: &Rectangle = &rectangle2
        let mut rect2_10: &Rectangle = &rectangle3
        DrawMod.MakeFullBoxVic2( local26, rect1_10, "REGIME PEOPLES ALLOWED", rect2_10, "");
        if (this.OptionsList5Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList5Id)].Refresh(this.OptionsList5Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList5Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList5Obj, 6, 160, -1, this.game, true, tHighlight: false, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num34, bby: num35);
          this.OptionsList5Id = this.AddSubPart( tsubpart3, num34, num35, 160, 112, 0);
        }
        let mut num36: i32 =  720;
        let mut num37: i32 =  401;
        let mut num38: i32 =  0;
        this.OptionsList6Obj = ATListClass::new();
        let mut index9: i32 =  0;
        do
        {
          if (this.game.Data.LocTypeObj[this.locnr].MinDistance[index9] > 0)
          {
            let mut locTypeCounter2: i32 =  this.game.Data.LocTypeCounter;
            for (let mut index10: i32 =  0; index10 <= locTypeCounter2; index10 += 1)
            {
              if (this.game.Data.LocTypeObj[this.locnr].MinDistance[this.game.Data.LocTypeObj[index10].LocTypeGroup] > 1)
              {
                num38 += 1;
                this.OptionsList6Obj.add(this.game.Data.LocTypeObj[index10].Name, -1, Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.locnr].MinDistance[this.game.Data.LocTypeObj[index10].LocTypeGroup])));
              }
            }
          }
          index9 += 1;
        }
        while (index9 <= 99);
        if (num38 == 0)
          this.OptionsList6Obj.add("No min. distances", -1);
         let mut local27: &Graphics = &Expression;
        rectangle2 = Rectangle::new(num36, num37 - 14, 330, 13);
        let mut rect1_11: &Rectangle = &rectangle2
        let mut rect2_11: &Rectangle = &rectangle3
        DrawMod.MakeFullBoxVic2( local27, rect1_11, "MIN.DISTANCE NEED", rect2_11, "");
        if (this.OptionsList6Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList6Id)].Refresh(this.OptionsList6Obj, -1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList6Id)] = true;
        }
        else
        {
          tsubpart3 =  new ATListSubPartClass(this.OptionsList6Obj, 6, 160, -1, this.game, true, tHighlight: false, tShowPair: true, tValueWidth: 40, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num36, bby: num37);
          this.OptionsList6Id = this.AddSubPart( tsubpart3, num36, num37, 160, 112, 0);
        }
        let mut num39: i32 =  550;
        let mut num40: i32 =  546;
        if (this.game.Data.LocTypeObj[this.locnr].Buildable)
        {
          this.OptionsListObj = ATListClass::new();
          let mut index11: i32 =  0;
          do
          {
            if (this.game.Data.LocTypeObj[this.locnr].BuildgroundType[index11])
            {
              let mut landscapeTypeCounter2: i32 =  this.game.Data.LandscapeTypeCounter;
              for (let mut index12: i32 =  0; index12 <= landscapeTypeCounter2; index12 += 1)
              {
                if (this.game.Data.LandscapeTypeObj[index12].BuildGround == index11 & !this.game.Data.LandscapeTypeObj[index12].DontShowInList)
                {
                  num38 += 1;
                  this.OptionsListObj.add(this.game.Data.LandscapeTypeObj[index12].Name, -1);
                }
              }
            }
            index11 += 1;
          }
          while (index11 <= 99);
           let mut local28: &Graphics = &Expression;
          rectangle2 = Rectangle::new(num39, num40 - 14, 330, 13);
          let mut rect1_12: &Rectangle = &rectangle2
          let mut rect2_12: &Rectangle = &rectangle3
          DrawMod.MakeFullBoxVic2( local28, rect1_12, "BUILDABLE LANDSC.TYPES", rect2_12, "");
          if (this.OptionsListId > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, -1);
            this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
          }
          else
          {
            tsubpart3 =  new ATListSubPartClass(this.OptionsListObj, 6, 160, -1, this.game, true, tHighlight: false, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: num39, bby: num40);
            this.OptionsListId = this.AddSubPart( tsubpart3, num39, num40, 160, 112, 0);
          }
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub object ReturnSFSpriteNr(typ: i32, regnr: i32, pplnr: i32)
    {
      let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[typ].SymbolSpriteID;
      if (regnr > -1)
      {
        if (this.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 =  0; index <= extraCounter; index += 1)
          {
            if (this.game.Data.SFTypeObj[typ].ExtraCode[index] == this.game.Data.RegimeObj[regnr].ExtraGraphicUse)
              symbolSpriteId = this.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
        else if (pplnr > -1 && this.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 =  0; index <= extraCounter; index += 1)
          {
            if (this.game.Data.SFTypeObj[typ].ExtraCode[index] == this.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              symbolSpriteId = this.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
      }
      else if (this.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
      {
        let mut extraCounter: i32 =  this.game.Data.SFTypeObj[typ].ExtraCounter;
        for (let mut index: i32 =  0; index <= extraCounter; index += 1)
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
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
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
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              if (num3 > -1)
              {
                this.ltnr = num3;
                this.DoRefresh();
              }
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.combatList2Id)
            {
              let mut num4: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
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
                    let mut num5: i32 =   Interaction.MsgBox( orderResult.ErrorString, Title: ( "Shadow Empire : Planetary Conquest"));
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
                  let mut num6: i32 =   Interaction.MsgBox( "You can only upgrade ALL because there is already 8 subformations in unit.");
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
